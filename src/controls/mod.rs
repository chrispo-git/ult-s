pub mod ext;
pub mod consts;
pub mod offsets;
pub mod util;
pub mod singletons;
pub mod modules;
pub mod controls_2;
pub use modules::*;

use ext::*;

use smash::app::lua_bind::*;
use smash::lua2cpp::*;
use smash::lib::{*, lua_const::*};

macro_rules! lua_gettop {
    ($state:ident) => {{
        let top = *($state as *const u64).add(2);
        let ci = *($state as *const u64).add(4);
        let func = *(ci as *const u64);
        (top - func) / 0x10
    }};
}

macro_rules! lua_settop {
    ($state:ident, $index:expr) => {{
        if $index < 0 {
            let ci = *($state as *const u64).add(4);
            let func = *(ci as *const i64);
            let new_top = (-$index * 0x10) + func;
            let top = ($state as *mut i64).add(2);
            while *top < new_top {
                *((*top) as *mut u32) = 0x0;
                *top = *top + 0x10;
            }
        } else {
            let top = ($state as *mut i64).add(2);
            *top = *top - (($index) * 0x10);
        }
    }};
}

#[skyline::from_offset(0x38f7620)]
unsafe extern "C" fn luaL_tolstring(lua_state: u64, index: i32, size: *mut usize) -> *const u8;

unsafe extern "C" fn lua_print_impl(lua_state: u64) -> i32 {
    let num_args = lua_gettop!(lua_state) - 1;
    for x in 1..=num_args {
        let mut size = 0;
        let c_str = luaL_tolstring(lua_state, x as i32, &mut size);
        let slice = std::slice::from_raw_parts(c_str, size);
        if x > 1 {
            print!("\t");
        }
        print!("{}", std::str::from_utf8(slice).unwrap());
        lua_settop!(lua_state, -2i64);
    }
    println!();
    return 0;
}

#[skyline::hook(offset = 0x38f49b0)]
unsafe fn lua_load(arg: u64, arg2: u64, arg3: u64, arg4: u64, mode: *const u8) -> u32 {
    let result = call_original!(arg, arg2, arg3, arg4, "bt\0".as_ptr());
    if result == 3 {
        let mut size = 0;
        let c_str = luaL_tolstring(arg, 1, &mut size);
        let slice = std::slice::from_raw_parts(c_str, size);
        println!(
            "error reading lua file: {}",
            std::str::from_utf8(slice).unwrap()
        );
    }
    result
}

#[skyline::from_offset(0x37701a0)]
unsafe fn register_button(arg: u64, id: i32, string: *const u8);

#[skyline::hook(offset = 0x1d32960, inline)]
unsafe fn add_buttons_to_subwindow(ctx: &mut skyline::hooks::InlineCtx) {
    let ptr = *ctx.registers[0].x.as_ref();
    register_button(ptr, 4, "set_parts_category_04\0".as_ptr());
    register_button(ptr, 6, "set_parts_category_05\0".as_ptr());
    // Can't use 5 here since that's for the "OK" button and
    // changing that will break the ability to save changes :weary:
    // *ctx.registers[1].x.as_mut() = 6;
    // *ctx.registers[20].x.as_mut() = 6;
    IS_IN_UI = true;
}

#[skyline::from_offset(0x3770730)]
unsafe fn layout_get(arg: u64, arg2: u64, id: u64);

#[skyline::from_offset(0x3770450)]
unsafe fn set_something(arg: u64, val: u64, val2: u64);

#[skyline::hook(offset = 0x1d32b84, inline)]
unsafe fn setup_buttons(ctx: &skyline::hooks::InlineCtx) {
    let ptr = *ctx.registers[0].x.as_ref();
    let ptr2 = *ctx.registers[1].x.as_ref();

    layout_get(ptr, ptr2, 4);

    let p_vtable = **(ptr as *const *const u64).add(1);
    let func: extern "C" fn(u64, u64) =
        std::mem::transmute(*((*(p_vtable as *const u64) + 0x208) as *const u64));

    func(p_vtable, 0);

    let func: extern "C" fn(u64) -> bool =
        std::mem::transmute(*((*(p_vtable as *const u64) + 0x200) as *const u64));

    let var = func(p_vtable);
    let var = if !var {
        true
    } else {
        let func: extern "C" fn(u64) -> bool =
            std::mem::transmute(*((*(p_vtable as *const u64) + 0x210) as *const u64));

        !func(p_vtable)
    };

    let func: extern "C" fn(u64, bool, u64) =
        std::mem::transmute(*((*(p_vtable as *const u64) + 0x388) as *const u64));

    func(p_vtable, var, 0);

    *(ptr as *mut u64).add(1) = 0;

    layout_get(ptr, ptr2, 6);

    let p_vtable = **(ptr as *const *const u64).add(1);
    let func: extern "C" fn(u64, u64) =
        std::mem::transmute(*((*(p_vtable as *const u64) + 0x208) as *const u64));

    func(p_vtable, 0);

    let func: extern "C" fn(u64) -> bool =
        std::mem::transmute(*((*(p_vtable as *const u64) + 0x200) as *const u64));

    let var = func(p_vtable);
    let var = if !var {
        true
    } else {
        let func: extern "C" fn(u64) -> bool =
            std::mem::transmute(*((*(p_vtable as *const u64) + 0x210) as *const u64));

        !func(p_vtable)
    };

    let func: extern "C" fn(u64, bool, u64) =
        std::mem::transmute(*((*(p_vtable as *const u64) + 0x388) as *const u64));

    func(p_vtable, var, 0);

    *(ptr as *mut u64).add(1) = 0;
}

#[skyline::hook(offset = 0x1d3339c, inline)]
unsafe fn hijack_animation_get(ctx: &skyline::hooks::InlineCtx) {
    // this memleaks but I DON'T GIVE A FUCK (Askew: doesn't actually memleak you schmuck)
    let ptr = *ctx.registers[0].x.as_ref();
    let our_ptr = SHARED_PTR1[0];
    layout_get(ptr, our_ptr, 0);

    let p_vtable = **(ptr as *const *const u64).add(1);
    let func: extern "C" fn(u64, bool, u64) =
        std::mem::transmute(*((*(p_vtable as *const u64) + 0x2f0) as *const u64));
    func(p_vtable, !CURRENT_UI_PARRY_TOGGLE, 0);

    *(ptr as *mut u64).add(1) = 0;

    layout_get(ptr, our_ptr, 1);

    let p_vtable = **(ptr as *const *const u64).add(1);
    let func: extern "C" fn(u64, bool, u64) =
        std::mem::transmute(*((*(p_vtable as *const u64) + 0x2f0) as *const u64));
    func(p_vtable, CURRENT_UI_PARRY_TOGGLE, 0);

    *(ptr as *mut u64).add(1) = 0;

    set_something(SHARED_PTR1.as_ptr() as _, CURRENT_UI_PARRY_TOGGLE as _, 1);

    let our_ptr = SHARED_PTR2[0];
    layout_get(ptr, our_ptr, 0);

    let p_vtable = **(ptr as *const *const u64).add(1);
    let func: extern "C" fn(u64, bool, u64) =
        std::mem::transmute(*((*(p_vtable as *const u64) + 0x2f0) as *const u64));
    func(p_vtable, !CURRENT_UI_RIVALS_JUMP, 0);

    *(ptr as *mut u64).add(1) = 0;

    layout_get(ptr, our_ptr, 1);

    let p_vtable = **(ptr as *const *const u64).add(1);
    let func: extern "C" fn(u64, bool, u64) =
        std::mem::transmute(*((*(p_vtable as *const u64) + 0x2f0) as *const u64));
    func(p_vtable, CURRENT_UI_RIVALS_JUMP, 0);

    *(ptr as *mut u64).add(1) = 0;

    set_something(SHARED_PTR2.as_ptr() as _, CURRENT_UI_RIVALS_JUMP as _, 1);
}

static mut SHARED_PTR1: [u64; 2] = [0, 0];
static mut SHARED_PTR2: [u64; 2] = [0, 0];

#[skyline::hook(offset = 0x1d32de4, inline)]
unsafe fn frank_talk_think_tankk(ctx: &mut skyline::hooks::InlineCtx) {
    if *ctx.registers[22].x.as_ref() == 4 {
        SHARED_PTR1[0] = 0;
        SHARED_PTR1[1] = 0;
        *ctx.registers[19].x.as_mut() = SHARED_PTR1.as_ptr() as u64;
    }

    if *ctx.registers[22].x.as_ref() == 5 {
        SHARED_PTR2[0] = 0;
        SHARED_PTR2[1] = 0;
        *ctx.registers[19].x.as_mut() = SHARED_PTR2.as_ptr() as u64;
    }
}

static mut CURRENT_UI_PARRY_TOGGLE: bool = false;
static mut CURRENT_UI_RIVALS_JUMP: bool = false;

#[skyline::hook(offset = 0x1d2ff18, inline)]
unsafe fn get_on_value_for_custom(ctx: &mut skyline::hooks::InlineCtx) {
    if *ctx.registers[1].x.as_ref() == 4 {
        *ctx.registers[19].x.as_mut() = CURRENT_UI_PARRY_TOGGLE as u64;
    } else if *ctx.registers[1].x.as_ref() == 6 {
        *ctx.registers[19].x.as_mut() = CURRENT_UI_RIVALS_JUMP as u64;
    }
}

#[skyline::hook(offset = 0x1d2ff34, inline)]
unsafe fn get_shared_ptr_for_custom(ctx: &mut skyline::hooks::InlineCtx) {
    if *ctx.registers[8].x.as_ref() == 4 {
        *ctx.registers[20].x.as_mut() = SHARED_PTR1.as_ptr() as u64;
    } else if *ctx.registers[8].x.as_ref() == 6 {
        *ctx.registers[20].x.as_mut() = SHARED_PTR2.as_ptr() as u64;
    }
}

#[skyline::hook(offset = 0x1d2f2cc, inline)]
unsafe fn init_buttons_in_main_loop(ctx: &skyline::hooks::InlineCtx) {
    let flag = *ctx.registers[1].x.as_ref() != 0;
    let ptr = SHARED_PTR1[0];
    let func: extern "C" fn(u64, bool) =
        std::mem::transmute(*((*(ptr as *const u64) + 0x60) as *const u64));

    func(ptr, flag);

    let ptr = SHARED_PTR2[0];
    let func: extern "C" fn(u64, bool) =
        std::mem::transmute(*((*(ptr as *const u64) + 0x60) as *const u64));

    func(ptr, flag);
}

#[skyline::hook(offset = 0x1d2f358, inline)]
unsafe fn init_buttons_in_main_loop_again(ctx: &skyline::hooks::InlineCtx) {
    init_buttons_in_main_loop(ctx);
}

#[skyline::hook(offset = 0x1d2f558, inline)]
unsafe fn get_index_for_a_press(ctx: &mut skyline::hooks::InlineCtx) {
    if *ctx.registers[22].x.as_ref() == 4 {
        *ctx.registers[10].x.as_mut() = CURRENT_UI_PARRY_TOGGLE as u64;
    } else if *ctx.registers[22].x.as_ref() == 6 {
        *ctx.registers[10].x.as_mut() = CURRENT_UI_RIVALS_JUMP as u64;
    }
}

#[skyline::hook(offset = 0x1d2f634, inline)]
unsafe fn update_index_for_a_press(ctx: &mut skyline::hooks::InlineCtx) {
    if *ctx.registers[22].x.as_ref() == 4 {
        CURRENT_UI_PARRY_TOGGLE = *ctx.registers[11].x.as_ref() != 0;
    } else if *ctx.registers[22].x.as_ref() == 6 {
        CURRENT_UI_RIVALS_JUMP = *ctx.registers[11].x.as_ref() != 0;
    }
}

static mut IS_IN_UI: bool = false;

#[skyline::hook(offset = 0x376c360)]
unsafe fn set_next_button(arg: u64, button: i32, other: u64) {
    if !IS_IN_UI {
        call_original!(arg, button, other);
        return;
    }

    let current_button = *(arg as *const u32).add(0x250 / 0x4);

    if current_button == 3 && button == 5 {
        call_original!(arg, 4, other);
    } else if current_button == 6 && button == 0 {
        call_original!(arg, 5, other);
    } else {
        call_original!(arg, button, other);
    }
}

#[skyline::hook(offset = 0x1d2fec4, inline)]
unsafe fn skip_set_setting_for_ok(ctx: &mut skyline::hooks::InlineCtx) {
    if *ctx.registers[1].x.as_ref() == 5 {
        *ctx.registers[1].x.as_mut() = 300;
    }
}

#[skyline::hook(offset = 0x1d30700, inline)]
unsafe fn init_ui_state(ctx: &mut skyline::hooks::InlineCtx) {
    let ptr = (*ctx.registers[8].x.as_ref() as *mut u8).add(1);
    CURRENT_UI_PARRY_TOGGLE = (*ptr >> 1) & 1 != 0;
    CURRENT_UI_RIVALS_JUMP = (*ptr >> 2) & 1 != 0;
    *ptr &= 1;
}

#[skyline::hook(offset = 0x1d2e8e4, inline)]
unsafe fn exit_gc(ctx: &mut skyline::hooks::InlineCtx) {
    let ptr = (*ctx.registers[20].x.as_ref() as *mut u8).add(0xC4);
    *ptr |= (CURRENT_UI_PARRY_TOGGLE as u8) << 1;
    *ptr |= (CURRENT_UI_RIVALS_JUMP as u8) << 2;
    IS_IN_UI = false;
}

#[skyline::hook(offset = 0x1d2e8b0, inline)]
unsafe fn exit_fk(ctx: &mut skyline::hooks::InlineCtx) {
    let ptr = (*ctx.registers[20].x.as_ref() as *mut u8).add(0xE0);
    *ptr |= (CURRENT_UI_PARRY_TOGGLE as u8) << 1;
    *ptr |= (CURRENT_UI_RIVALS_JUMP as u8) << 2;
    IS_IN_UI = false;
}

#[skyline::hook(offset = 0x1d2e864, inline)]
unsafe fn exit_jc(ctx: &mut skyline::hooks::InlineCtx) {
    let ptr = (*ctx.registers[20].x.as_ref() as *mut u8).add(0xD4);
    *ptr |= (CURRENT_UI_PARRY_TOGGLE as u8) << 1;
    *ptr |= (CURRENT_UI_RIVALS_JUMP as u8) << 2;
    IS_IN_UI = false;
}

unsafe fn get_parts(arg: u64, arg2: *const u8) -> [u64; 4] {
    let func_addr =
        (skyline::hooks::getRegionAddress(skyline::hooks::Region::Text) as *mut u8).add(0x3775010);
    let callable: extern "C" fn(u64, *const u8, ...) -> [u64; 4] = std::mem::transmute(func_addr);
    callable(arg, arg2)
}

#[skyline::hook(offset = 0x1d32774, inline)]
unsafe fn set_pane_text_values(ctx: &skyline::hooks::InlineCtx) {
    let layout_view = *ctx.registers[0].x.as_ref();

    let mut parts = get_parts(
        [0, layout_view].as_ptr() as _,
        b"set_parts_category_04\0".as_ptr(),
    );

    let pane = crate::online::get_pane_by_name(parts.as_mut_ptr() as _, b"txt_name_00\0".as_ptr());

    crate::online::set_text_string(*((pane[1] + 0x10) as *const u64), "Parry Input\0".as_ptr());

    let mut parts = get_parts(
        [0, layout_view].as_ptr() as _,
        b"set_parts_category_05\0".as_ptr(),
    );

    let pane = crate::online::get_pane_by_name(parts.as_mut_ptr() as _, b"txt_name_00\0".as_ptr());

    crate::online::set_text_string(
        *((pane[1] + 0x10) as *const u64),
        "To Be Decided\0".as_ptr(),
    );
}

#[skyline::hook(offset = 0x1d33060, inline)]
unsafe fn set_parry_button_shield_text(ctx: &skyline::hooks::InlineCtx) {
    let sp = (ctx as *const _ as *const u8).add(0x100);
    let ptr = *(sp.add(0xa8) as *const u64);

    if *ctx.registers[22].x.as_ref() == 4 {
        crate::online::set_text_string(*((ptr + 0x10) as *const u64), "Special\0".as_ptr());
    }
}

#[skyline::hook(offset = 0x1d331a0, inline)]
unsafe fn set_parry_button_taunt_text(ctx: &skyline::hooks::InlineCtx) {
    let sp = (ctx as *const _ as *const u8).add(0x100);
    let ptr = *(sp.add(0xa8) as *const u64);

    if *ctx.registers[22].x.as_ref() == 4 {
        crate::online::set_text_string(*((ptr + 0x10) as *const u64), "Taunt\0".as_ptr());
    }
}

#[skyline::hook(offset = 0x16d948c, inline)]
unsafe fn packed_packet_creation(ctx: &mut skyline::hooks::InlineCtx) {
    *ctx.registers[22].x.as_mut() = 0x2;
}

#[skyline::hook(offset = 0x16d94c0, inline)]
unsafe fn write_packet(ctx: &mut skyline::hooks::InlineCtx) {
    let raw = *ctx.registers[19].x.as_ref();

    let mapped_inputs = *((raw + 0x49508) as *const MappedInputs);
    let mut packet = 0u64;

    *(&mut packet as *mut u64 as *mut i8) = mapped_inputs.lstick_x;
    *(&mut packet as *mut u64 as *mut i8).add(1) = mapped_inputs.lstick_y;

    let buttons = (mapped_inputs.buttons.bits() as u64) << 16;
    packet |= buttons;

    *(&mut packet as *mut u64 as *mut i8).add(6) = mapped_inputs.rstick_x;
    *(&mut packet as *mut u64 as *mut i8).add(7) = mapped_inputs.rstick_y;

    *ctx.registers[8].x.as_mut() = packet;
}

#[repr(C)]
struct SomeControllerStruct {
    padding: [u8; 0x10],
    controller: &'static mut Controller,
}

unsafe fn get_player_idx_from_boma(boma: u64) -> i32 {
    let control_module = *((boma + 0x48) as *const u64);
    let next = *((control_module + 0x118) as *const u64);
    let next = *((next + 0x58) as *const u64);
    let next = *((next + 0x8) as *const u64);
    *((next + 0x8) as *const i32)
}

macro_rules! apply_button_mappings {
    ($controller:ident, $mappings:ident, $(($button:ident, $mapped:ident, $kind:ident, $output:expr))*) => {{
        let mut buttons = Buttons::empty();
        $(
                if $controller.current_buttons.$button() && (*$mappings).$mapped == InputKind::$kind {
                    buttons |= $output;
                }
        )*
        buttons
    }}
}

#[skyline::hook(offset = offsets::map_controls())]
unsafe fn map_controls_hook(
    mappings: *mut ControllerMapping,
    player_idx: i32,
    out: *mut MappedInputs,
    controller_struct: &mut SomeControllerStruct,
    arg: bool
) {
    let entry_count = (*mappings.add(player_idx as usize))._34[0];
    let ret = original!()(mappings, player_idx, out, controller_struct, arg);
    let controller = &mut controller_struct.controller;

    //println!("entry_count vs. current: {} vs. {}", entry_count, (*mappings.add(player_idx as usize))._34[0]);

    if (*out).buttons.contains(Buttons::CStickOn) && (*mappings.add(player_idx as usize))._34[0] != entry_count {
        (*out).rstick_x = (controller.left_stick_x * (i8::MAX as f32)) as i8;
        (*out).rstick_y = (controller.left_stick_y * (i8::MAX as f32)) as i8;
        (*out).buttons |= Buttons::CStickOverride;
    } else {
        (*out).rstick_x = (controller.right_stick_x * (i8::MAX as f32)) as i8;
        (*out).rstick_y = (controller.right_stick_y * (i8::MAX as f32)) as i8;
    }

    let mappings = mappings.add(player_idx as usize);

    if controller.style == ControllerStyle::GCController {
        (*out).buttons |= apply_button_mappings!(
            controller,
            mappings,
                (l, gc_l, JumpMini, Buttons::JumpMini | Buttons::Jump)
                (r, gc_r, JumpMini, Buttons::JumpMini | Buttons::Jump)
                (zl, gc_z, JumpMini, Buttons::JumpMini | Buttons::Jump)
                (zr, gc_z, JumpMini, Buttons::JumpMini | Buttons::Jump)
                (a, gc_a, JumpMini, Buttons::JumpMini | Buttons::Jump)
                (b, gc_b, JumpMini, Buttons::JumpMini | Buttons::Jump)
                (x, gc_x, JumpMini, Buttons::JumpMini | Buttons::Jump)
                (y, gc_y, JumpMini, Buttons::JumpMini | Buttons::Jump)
        );
        (*out).buttons |= apply_button_mappings!(
            controller,
            mappings,
                (l, gc_l, SmashAttack, Buttons::AttackAll)
                (r, gc_r, SmashAttack, Buttons::AttackAll)
                (zl, gc_z, SmashAttack, Buttons::AttackAll)
                (zr, gc_z, SmashAttack, Buttons::AttackAll)
                (a, gc_a, SmashAttack, Buttons::AttackAll)
                (b, gc_b, SmashAttack, Buttons::AttackAll)
                (x, gc_x, SmashAttack, Buttons::AttackAll)
                (y, gc_y, SmashAttack, Buttons::AttackAll)
        );
        (*out).buttons |= apply_button_mappings!(
            controller,
            mappings,
                (l, gc_l,   AppealHi, Buttons::AppealHi)
                (r, gc_r,   AppealHi, Buttons::AppealHi)
                (zl, gc_z,  AppealHi, Buttons::AppealHi)
                (zr, gc_z,  AppealHi, Buttons::AppealHi)
                (a, gc_a,   AppealHi, Buttons::AppealHi)
                (b, gc_b,   AppealHi, Buttons::AppealHi)
                (x, gc_x,   AppealHi, Buttons::AppealHi)
                (y, gc_y,   AppealHi, Buttons::AppealHi)
        );
        (*out).buttons |= apply_button_mappings!(
            controller,
            mappings,
                (l, gc_l,   FullHop, Buttons::FullHop | Buttons::Jump)
                (r, gc_r,   FullHop, Buttons::FullHop | Buttons::Jump)
                (zl, gc_z,  FullHop, Buttons::FullHop | Buttons::Jump)
                (zr, gc_z,  FullHop, Buttons::FullHop | Buttons::Jump)
                (a, gc_a,   FullHop, Buttons::FullHop | Buttons::Jump)
                (b, gc_b,   FullHop, Buttons::FullHop | Buttons::Jump)
                (x, gc_x,   FullHop, Buttons::FullHop | Buttons::Jump)
                (y, gc_y,   FullHop, Buttons::FullHop | Buttons::Jump)
        );
        /*
        if (*mappings.add(player_idx as usize)).gc_absmash {
            if (*out).buttons.contains(Buttons::Attack | Buttons::Special) {
                (*out).buttons &= !(Buttons::Special | Buttons::FullHop);
                (*out).buttons |= Buttons::Smash;
                (*mappings.add(player_idx as usize)).is_absmash = true;
            } else if !(*out).buttons.intersects(Buttons::Attack | Buttons::Special) {
                (*mappings.add(player_idx as usize)).is_absmash = false;
            } else if (*mappings.add(player_idx as usize)).is_absmash {
                (*out).buttons &= !(Buttons::Special | Buttons::FullHop);
            }
        }
        */
    } else if controller.style == ControllerStyle::LeftJoycon || controller.style == ControllerStyle::RightJoycon {
        (*out).buttons |= apply_button_mappings!(
            controller,
            mappings,
                (l, joy_shoulder, JumpMini, Buttons::JumpMini | Buttons::Jump)
                (r, joy_shoulder, JumpMini, Buttons::JumpMini | Buttons::Jump)
                (zl, joy_zshoulder, JumpMini, Buttons::JumpMini | Buttons::Jump)
                (zr, joy_zshoulder, JumpMini, Buttons::JumpMini | Buttons::Jump)
                (left_sl, joy_sl, JumpMini, Buttons::JumpMini | Buttons::Jump)
                (left_sr, joy_sr, JumpMini, Buttons::JumpMini | Buttons::Jump)
                (right_sl, joy_sl, JumpMini, Buttons::JumpMini | Buttons::Jump)
                (right_sr, joy_sr, JumpMini, Buttons::JumpMini | Buttons::Jump)
        );
        (*out).buttons |= apply_button_mappings!(
            controller,
            mappings,
                (l, joy_shoulder,   SmashAttack, Buttons::AttackAll)
                (r, joy_shoulder,   SmashAttack, Buttons::AttackAll)
                (zl, joy_zshoulder, SmashAttack, Buttons::AttackAll)
                (zr, joy_zshoulder, SmashAttack, Buttons::AttackAll)
                (left_sl, joy_sl,   SmashAttack, Buttons::AttackAll)
                (left_sr, joy_sr,   SmashAttack, Buttons::AttackAll)
                (right_sl, joy_sl,  SmashAttack, Buttons::AttackAll)
                (right_sr, joy_sr,  SmashAttack, Buttons::AttackAll)
        );
        (*out).buttons |= apply_button_mappings!(
            controller,
            mappings,
                (l, joy_shoulder,   AppealHi, Buttons::AppealHi)
                (r, joy_shoulder,   AppealHi, Buttons::AppealHi)
                (zl, joy_zshoulder, AppealHi, Buttons::AppealHi)
                (zr, joy_zshoulder, AppealHi, Buttons::AppealHi)
                (left_sl, joy_sl,   AppealHi, Buttons::AppealHi)
                (left_sr, joy_sr,   AppealHi, Buttons::AppealHi)
                (right_sl, joy_sl,  AppealHi, Buttons::AppealHi)
                (right_sr, joy_sr,  AppealHi, Buttons::AppealHi)
        );
        (*out).buttons |= apply_button_mappings!(
            controller,
            mappings,
                (l, joy_shoulder,   FullHop, Buttons::FullHop | Buttons::Jump)
                (r, joy_shoulder,   FullHop, Buttons::FullHop | Buttons::Jump)
                (zl, joy_zshoulder, FullHop, Buttons::FullHop | Buttons::Jump)
                (zr, joy_zshoulder, FullHop, Buttons::FullHop | Buttons::Jump)
                (left_sl, joy_sl,   FullHop, Buttons::FullHop | Buttons::Jump)
                (left_sr, joy_sr,   FullHop, Buttons::FullHop | Buttons::Jump)
                (right_sl, joy_sl,  FullHop, Buttons::FullHop | Buttons::Jump)
                (right_sr, joy_sr,  FullHop, Buttons::FullHop | Buttons::Jump)
        );

        if controller.style == ControllerStyle::LeftJoycon {
            (*out).buttons |= apply_button_mappings!(
                controller,
                mappings,
                    (dpad_left, joy_down, JumpMini, Buttons::JumpMini | Buttons::Jump)
                    (dpad_right, joy_up, JumpMini, Buttons::JumpMini | Buttons::Jump)
                    (dpad_up, joy_left, JumpMini, Buttons::JumpMini | Buttons::Jump)
                    (dpad_down, joy_right, JumpMini, Buttons::JumpMini | Buttons::Jump)
            );
            (*out).buttons |= apply_button_mappings!(
                controller,
                mappings,
                    (dpad_left, joy_down, SmashAttack, Buttons::AttackAll)
                    (dpad_right, joy_up, SmashAttack, Buttons::AttackAll)
                    (dpad_up, joy_left, SmashAttack, Buttons::AttackAll)
                    (dpad_down, joy_right, SmashAttack, Buttons::AttackAll)
            );
            (*out).buttons |= apply_button_mappings!(
                controller,
                mappings,
                    (dpad_left, joy_down,   AppealHi, Buttons::AppealHi)
                    (dpad_right, joy_up,    AppealHi, Buttons::AppealHi)
                    (dpad_up, joy_left,     AppealHi, Buttons::AppealHi)
                    (dpad_down, joy_right,  AppealHi, Buttons::AppealHi)
            );
            (*out).buttons |= apply_button_mappings!(
                controller,
                mappings,
                    (dpad_left, joy_down,   FullHop, Buttons::FullHop | Buttons::Jump)
                    (dpad_right, joy_up,    FullHop, Buttons::FullHop | Buttons::Jump)
                    (dpad_up, joy_left,     FullHop, Buttons::FullHop | Buttons::Jump)
                    (dpad_down, joy_right,  FullHop, Buttons::FullHop | Buttons::Jump)
            );
        } else {
            (*out).buttons |= apply_button_mappings!(
                controller,
                mappings,
                    (a, joy_down, JumpMini, Buttons::JumpMini | Buttons::Jump)
                    (y, joy_up, JumpMini, Buttons::JumpMini | Buttons::Jump)
                    (b, joy_left, JumpMini, Buttons::JumpMini | Buttons::Jump)
                    (x, joy_right, JumpMini, Buttons::JumpMini | Buttons::Jump)
            );
            (*out).buttons |= apply_button_mappings!(
                controller,
                mappings,
                    (a, joy_down, SmashAttack, Buttons::AttackAll)
                    (y, joy_up, SmashAttack, Buttons::AttackAll)
                    (b, joy_left, SmashAttack, Buttons::AttackAll)
                    (x, joy_right, SmashAttack, Buttons::AttackAll)
            );
            (*out).buttons |= apply_button_mappings!(
                controller,
                mappings,
                    (a, joy_down,   AppealHi, Buttons::AppealHi)
                    (y, joy_up,     AppealHi, Buttons::AppealHi)
                    (b, joy_left,   AppealHi, Buttons::AppealHi)
                    (x, joy_right,  AppealHi, Buttons::AppealHi)
            );
            (*out).buttons |= apply_button_mappings!(
                controller,
                mappings,
                    (a, joy_down,   FullHop, Buttons::FullHop | Buttons::Jump)
                    (y, joy_up,     FullHop, Buttons::FullHop | Buttons::Jump)
                    (b, joy_left,   FullHop, Buttons::FullHop | Buttons::Jump)
                    (x, joy_right,  FullHop, Buttons::FullHop | Buttons::Jump)
            );
        }
        /*
        if (*mappings.add(player_idx as usize)).joy_absmash {
            if (*out).buttons.contains(Buttons::Attack | Buttons::Special) {
                (*out).buttons &= !(Buttons::Special | Buttons::FullHop);
                (*out).buttons |= Buttons::Smash;
                (*mappings.add(player_idx as usize)).is_absmash = true;
            } else if !(*out).buttons.intersects(Buttons::Attack | Buttons::Special) {
                (*mappings.add(player_idx as usize)).is_absmash = false;
            } else if (*mappings.add(player_idx as usize)).is_absmash {
                (*out).buttons &= !(Buttons::Special | Buttons::FullHop);
            }
        }*/
    } else {
        (*out).buttons |= apply_button_mappings!(
            controller,
            mappings,
                (l, pro_l, JumpMini, Buttons::JumpMini | Buttons::Jump)
                (r, pro_r, JumpMini, Buttons::JumpMini | Buttons::Jump)
                (zl, pro_zl, JumpMini, Buttons::JumpMini | Buttons::Jump)
                (zr, pro_zr, JumpMini, Buttons::JumpMini | Buttons::Jump)
                (a, pro_a, JumpMini, Buttons::JumpMini | Buttons::Jump)
                (b, pro_b, JumpMini, Buttons::JumpMini | Buttons::Jump)
                (x, pro_x, JumpMini, Buttons::JumpMini | Buttons::Jump)
                (y, pro_y, JumpMini, Buttons::JumpMini | Buttons::Jump)
        );
        (*out).buttons |= apply_button_mappings!(
            controller,
            mappings,
                (l, pro_l, SmashAttack, Buttons::AttackAll)
                (r, pro_r, SmashAttack, Buttons::AttackAll)
                (zl, pro_zl, SmashAttack, Buttons::AttackAll)
                (zr, pro_zr, SmashAttack, Buttons::AttackAll)
                (a, pro_a, SmashAttack, Buttons::AttackAll)
                (b, pro_b, SmashAttack, Buttons::AttackAll)
                (x, pro_x, SmashAttack, Buttons::AttackAll)
                (y, pro_y, SmashAttack, Buttons::AttackAll)
        );
        (*out).buttons |= apply_button_mappings!(
            controller,
            mappings,
                (l, pro_l,      AppealHi, Buttons::AppealHi)
                (r, pro_r,      AppealHi, Buttons::AppealHi)
                (zl, pro_zl,    AppealHi, Buttons::AppealHi)
                (zr, pro_zr,    AppealHi, Buttons::AppealHi)
                (a, pro_a,      AppealHi, Buttons::AppealHi)
                (b, pro_b,      AppealHi, Buttons::AppealHi)
                (x, pro_x,      AppealHi, Buttons::AppealHi)
                (y, pro_y,      AppealHi, Buttons::AppealHi)
        );
        (*out).buttons |= apply_button_mappings!(
            controller,
            mappings,
                (l, pro_l,      FullHop, Buttons::FullHop | Buttons::Jump)
                (r, pro_r,      FullHop, Buttons::FullHop | Buttons::Jump)
                (zl, pro_zl,    FullHop, Buttons::FullHop | Buttons::Jump)
                (zr, pro_zr,    FullHop, Buttons::FullHop | Buttons::Jump)
                (a, pro_a,      FullHop, Buttons::FullHop | Buttons::Jump)
                (b, pro_b,      FullHop, Buttons::FullHop | Buttons::Jump)
                (x, pro_x,      FullHop, Buttons::FullHop | Buttons::Jump)
                (y, pro_y,      FullHop, Buttons::FullHop | Buttons::Jump)
        );
        /*
        if (*mappings.add(player_idx as usize)).pro_absmash {
            if (*out).buttons.contains(Buttons::Attack | Buttons::Special) {
                (*out).buttons &= !(Buttons::Special | Buttons::FullHop);
                (*out).buttons |= Buttons::Smash;
                (*mappings.add(player_idx as usize)).is_absmash = true;
            } else if !(*out).buttons.intersects(Buttons::Attack | Buttons::Special) {
                (*mappings.add(player_idx as usize)).is_absmash = false;
            } else if (*mappings.add(player_idx as usize)).is_absmash {
                (*out).buttons &= !(Buttons::Special | Buttons::FullHop);
            }
        }*/
    }

    // Check if the button combos are being pressed and then force Stock Share + AttackRaw/SpecialRaw depending on input

    if controller.current_buttons.l()
    && controller.current_buttons.r()
    && controller.current_buttons.a()
    && (controller.current_buttons.minus() || controller.current_buttons.plus())
    {
        controller.current_buttons.set_plus(false);
        controller.current_buttons.set_minus(false);
        controller.just_down.set_plus(false);
        controller.just_down.set_minus(false);

        if controller.current_buttons.y() {
            (*out).buttons = Buttons::StockShare | Buttons::AttackRaw;
        } else if controller.current_buttons.x() {
            (*out).buttons = Buttons::StockShare | Buttons::SpecialRaw;
        } else {
            controller.current_buttons.set_plus(true);
            controller.current_buttons.set_minus(true);
            controller.just_down.set_plus(true);
            controller.just_down.set_minus(true);
        }
    }
}

#[skyline::hook(offset = offsets::analog_trigger_l(), inline)]
unsafe fn analog_trigger_l(ctx: &mut skyline::hooks::InlineCtx) {
    if *ctx.registers[9].x.as_ref() & 0x40 != 0 {
        *ctx.registers[11].x.as_mut() = 0;
    } else {
        *ctx.registers[11].w.as_mut() = 0x27FF;
    }
}

#[skyline::hook(offset = offsets::analog_trigger_r(), inline)]
unsafe fn analog_trigger_r(ctx: &mut skyline::hooks::InlineCtx) {
    if *ctx.registers[8].x.as_ref() & 0x80 != 0 {
        *ctx.registers[11].x.as_mut() = 0;
    } else {
        *ctx.registers[11].w.as_mut() = 0x27FF;
    }
}

#[repr(C)]
struct ControlModuleInternal {
    vtable: *mut u8,
    controller_index: i32,
    buttons: Buttons,
    stick_x: f32,
    stick_y: f32,
    padding: [f32; 2],
    unk: [u32; 8],
    clamped_lstick_x: f32,
    clamped_lstick_y: f32,
    padding2: [f32; 2],
    clamped_rstick_x: f32,
    clamped_rstick_y: f32,
}

unsafe fn get_mapped_controller_inputs(player: usize) -> &'static MappedInputs {
    let base = *((skyline::hooks::getRegionAddress(skyline::hooks::Region::Text) as *mut u8).add(0x52c30f0) as *const u64);
    &*((base + 0x2b8 + 0x8 * (player as u64)) as *const MappedInputs)
}

static mut LAST_ALT_STICK: [f32; 2] = [0.0, 0.0];

#[skyline::hook(offset = 0x3f7220)]
unsafe fn parse_inputs(this: &mut ControlModuleInternal) {
    const NEUTRAL: f32 = 0.2;
    const CLAMP_MAX: f32 = 120.0;

    // println!("this: {:#x}", this as *mut ControlModuleInternal as u64);

    if this.controller_index == -1 {
        return call_original!(this);
    }

    //println!("this.controller_index: {}", this.controller_index);
    // assert!(this.controller_index <= 7);

    let inputs = get_mapped_controller_inputs(this.controller_index as usize);

    let clamp_mul = 1.0 / CLAMP_MAX;

    // let raw_lstick_x = ((inputs.lstick_x as f32) * clamp_mul).clamp(-1.0, 1.0);
    // let raw_lstick_y = ((inputs.lstick_y as f32) * clamp_mul).clamp(-1.0, 1.0);

    // let raw_lstick_x = if raw_lstick_x.abs() >= NEUTRAL { raw_lstick_x } else { 0.0 };
    // let raw_lstick_y = if raw_lstick_y.abs() >= NEUTRAL { raw_lstick_y } else { 0.0 };

    let raw_rstick_x = ((inputs.rstick_x as f32) * clamp_mul).clamp(-1.0, 1.0);
    let raw_rstick_y = ((inputs.rstick_y as f32) * clamp_mul).clamp(-1.0, 1.0);

    LAST_ALT_STICK[0] = if raw_rstick_x.abs() >= NEUTRAL { raw_rstick_x } else { 0.0 };
    LAST_ALT_STICK[1] = if raw_rstick_y.abs() >= NEUTRAL { raw_rstick_y } else { 0.0 };

    call_original!(this)
}

#[skyline::hook(offset = 0x6b9c5c, inline)]
unsafe fn after_exec(ctx: &skyline::hooks::InlineCtx) {
    let module = *ctx.registers[19].x.as_ref();
    let internal_class = *(module as *const u64).add(0x110 / 0x8);
    *(internal_class as *mut f32).add(0x40 / 0x4) = LAST_ALT_STICK[0];
    *(internal_class as *mut f32).add(0x44 / 0x4) = LAST_ALT_STICK[1];
}

#[skyline::hook(offset = 0x16d7ee4, inline)]
unsafe fn handle_incoming_packet(ctx: &mut skyline::hooks::InlineCtx) {
    let packet = *ctx.registers[15].x.as_ref();

    let mut inputs = MappedInputs {
        buttons: Buttons::empty(),
        lstick_x: 0,
        lstick_y: 0,
        rstick_x: 0,
        rstick_y: 0
    };

    let raw_buttons = ((packet >> 16) & 0xFFFF_FFFF) as u32;
    let lstick_x = (packet & 0xFF) as i8;
    let lstick_y = ((packet & 0xFF00) >> 8) as i8;
    let rstick_x = ((packet >> 0x30) & 0xFF) as i8;
    let rstick_y = ((packet >> 0x38) & 0xFF) as i8;

    inputs.buttons = Buttons::from_bits_unchecked(raw_buttons as _);
    inputs.lstick_x = lstick_x;
    inputs.lstick_y = lstick_y;
    inputs.rstick_x = rstick_x;
    inputs.rstick_y = rstick_y;

    *ctx.registers[13].x.as_mut() = std::mem::transmute(inputs);
}

/// fix throws not respecting the cstick, especially dk cargo throw
#[skyline::hook(replace = L2CFighterCommon_IsThrowStick)]
unsafe extern "C" fn is_throw_stick(fighter: &mut L2CFighterCommon) -> L2CValue {
    let mut out = fighter.local_func__fighter_status_catch_1();
    let stick_x = fighter.stick_x() * PostureModule::lr(fighter.boma());
    let stick_y = fighter.stick_y();
    if stick_x > fighter.get_param_float("common", "attack_lw3_stick_x") {
        out["f"] = true.into();
    } else if stick_x < -fighter.get_param_float("common", "attack_lw3_stick_x") {
        out["b"] = true.into();
    }
    if stick_y > fighter.get_param_float("common", "attack_hi4_stick_y") {
        out["hi"] = true.into();
    } else if stick_y < fighter.get_param_float("common", "attack_lw4_stick_y") {
        out["lw"] = true.into();
    }
    out
}

fn nro_hook(info: &skyline::nro::NroInfo) {
    if info.name == "common" {
        skyline::install_hook!(is_throw_stick);
    }
}

pub fn install() {
    unsafe {
        skyline::patching::Patch::in_text(0x5291c70).data((lua_print_impl as *const ()));
        skyline::patching::Patch::in_text(0x372b4b0).data(0xD503201Fu32);
        skyline::patching::Patch::in_text(0x1d3321c).data(0xF1001ADFu32);
        skyline::patching::Patch::in_text(0x1d2fec4).data(0x7100183Fu32);
        // skyline::patching::Patch::in_text(0x1d2fec8).nop();
        skyline::patching::Patch::in_text(0x1d2ff48).data(0xAA1403F8u32);
    }
    skyline::install_hooks!(
        lua_load,
        add_buttons_to_subwindow,
        frank_talk_think_tankk,
        hijack_animation_get,
        setup_buttons,
        get_on_value_for_custom,
        get_shared_ptr_for_custom,
        init_buttons_in_main_loop,
        init_buttons_in_main_loop_again,
        get_index_for_a_press,
        update_index_for_a_press,
        set_next_button,
        skip_set_setting_for_ok,
        init_ui_state,
        exit_gc,
        exit_fk,
        exit_jc,
        set_pane_text_values,
        set_parry_button_shield_text,
        set_parry_button_taunt_text
    );
    skyline::install_hooks!(
        map_controls_hook,
        analog_trigger_l,
        analog_trigger_r,
        packed_packet_creation,
        write_packet,
        parse_inputs,
        handle_incoming_packet,
        after_exec
    );
    skyline::nro::add_hook(nro_hook);
    controls_2::install();
}

