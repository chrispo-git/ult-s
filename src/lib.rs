#![feature(concat_idents)]
#![feature(proc_macro_hygiene)]
#![feature(asm)]
#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(non_upper_case_globals)]
#![allow(warnings, unused)]

#[cfg(feature = "main_nro")]
use skyline_web::dialog_ok::DialogOk;
use std::{fs, path::Path};

#[macro_use]
extern crate modular_bitfield;

#[macro_use]
extern crate lazy_static;

pub static mut FIGHTER_MANAGER: usize = 0;

use skyline::libc::c_char;
use skyline::nro::{self, NroInfo};
use smash::params::add_hook;
use std::sync::atomic::{AtomicBool, Ordering};
use skyline::hooks::InlineCtx;

pub fn is_on_ryujinx() -> bool {
    unsafe {
        // Ryujinx skip based on text addr
        let text_addr = skyline::hooks::getRegionAddress(skyline::hooks::Region::Text) as u64;
        if text_addr == 0x8504000 || text_addr == 0x80004000 {
            println!("we are on Ryujinx");
            return true;
        } else {
            println!("we are not on Ryujinx");
            return false;
        }
    }
}

#[cfg(feature = "main_nro")]
pub fn quick_validate_install() {
    //plugin checks
    let has_param_config = Path::new("sd:/atmosphere/contents/01006a800016e000/romfs/skyline/plugins/libparam_config.nro").is_file();
    let has_css_redirector = Path::new("sd:/atmosphere/contents/01006a800016e000/romfs/skyline/plugins/libcss_slot_redirection.nro").is_file();
    let has_stage_config = Path::new("sd:/atmosphere/contents/01006a800016e000/romfs/skyline/plugins/libstage_config.nro").is_file();
    let has_arcropolis = Path::new("sd:/atmosphere/contents/01006a800016e000/romfs/skyline/plugins/libarcropolis.nro").is_file();
    let has_nro_hook = Path::new("sd:/atmosphere/contents/01006a800016e000/romfs/skyline/plugins/libnro_hook.nro").is_file();
    let has_smashline = Path::new("sd:/atmosphere/contents/01006a800016e000/romfs/skyline/plugins/libsmashline_hook.nro").is_file();
    let has_skyline = Path::new("sd:/atmosphere/contents/01006a800016e000/exefs/").is_dir();

    if has_param_config {
        println!("libparam_config.nro is present");
    } else {
        if is_on_ryujinx() {
            println!("libparam_config.nro not found! This installation is incomplete. Please download all dependencies listed in the README file.");
        } else {
            DialogOk::ok("libparam_config.nro not found! This installation is incomplete. Please download all dependencies listed in the README file.");
        }
    }
    if has_css_redirector {
        println!("libcss_slot_redirection.nro is present");
    } else {
        if is_on_ryujinx() {
            println!("libcss_slot_redirection.nro not found! This installation is incomplete. Please download all dependencies listed in the README file.");
        } else {
            DialogOk::ok("libcss_slot_redirection.nro not found! This installation is incomplete. Please download all dependencies listed in the README file.");
        }
    }
    if has_stage_config {
        println!("libstage_config.nro is present");
    } else {
        if is_on_ryujinx() {
            println!("libstage_config.nro not found! This installation is incomplete. Please download all dependencies listed in the README file.");
        } else {
            DialogOk::ok("libstage_config.nro not found! This installation is incomplete. Please download all dependencies listed in the README file.");
        }
    }
    if has_arcropolis {
        println!("libarcropolis.nro is present");
    } else {
        if is_on_ryujinx() {
            println!("libarcropolis.nro not found! This installation is incomplete. Please download all dependencies listed in the README file.");
        } else {
            DialogOk::ok("libarcropolis.nro not found! This installation is incomplete. Please download all dependencies listed in the README file.");
        }
    }
    if has_nro_hook {
        println!("libnro_hook.nro is present");
    } else {
        if is_on_ryujinx() {
            println!("libnro_hook.nro not found! This installation is incomplete. Please download all dependencies listed in the README file.");
        } else {
            DialogOk::ok("libnro_hook.nro not found! This installation is incomplete. Please download all dependencies listed in the README file.");
        }
    }
    if has_smashline {
        println!("libsmashline_hook.nro is present");
    } else {
        if is_on_ryujinx() {
            println!("libsmashline_hook.nro not found! This installation is incomplete. Please download all dependencies listed in the README file.");
        } else {
            DialogOk::ok("libsmashline_hook.nro not found! This installation is incomplete. Please download all dependencies listed in the README file.");
        }
    }
    if has_skyline {
        println!("Skyline is present");
    } else {
        if is_on_ryujinx() {
            println!("Skyline not found! This installation is incomplete. Please download all dependencies listed in the README file.");
        } else {
            DialogOk::ok("Skyline not found! This installation is incomplete. Please download all dependencies listed in the README file.");
        }
    }
}

extern "C" {
	fn change_version_string(arg: u64, string: *const c_char);
}
pub fn nro_hook(info: &skyline::nro::NroInfo) {
    if info.module.isLoaded {
        return;
    }

    if info.name == "common" {
        skyline::install_hooks!(
            cpu::dmg_fly_main,
            cpu::dmg_fly_roll_main,
            cpu::dmg_main,
            cpu::dmg_air_main
        );
    }
}


unsafe fn calc_nnsdk_offset() -> u64 {
    let mut symbol = 0usize;
    skyline::nn::ro::LookupSymbol(&mut symbol, b"_ZN7android7IBinderD1Ev\0".as_ptr());
    (symbol - 0x240) as u64
}

static mut OFFSET1: u64 = 0;
static mut OFFSET2: u64 = 0;

#[skyline::hook(replace = OFFSET1)]
unsafe fn set_interval_1(window: u64, _: i32) {
    call_original!(window, 0);
}

#[skyline::hook(replace = OFFSET2, inline)]
unsafe fn set_interval_2(ctx: &mut InlineCtx) {
    *ctx.registers[8].x.as_mut() = 0;
}

static mut RUN: AtomicBool = AtomicBool::new(false);

#[skyline::hook(offset = 0x380f9e4, inline)]
unsafe fn vsync_count_thread(_: &skyline::hooks::InlineCtx) {
    RUN.store(true, Ordering::SeqCst);
}

static mut DUMMY_BLOCK: [u8; 0x100] = [0; 0x100];

#[skyline::hook(offset = 0x3746afc, inline)]
unsafe fn run_scene_update(_: &skyline::hooks::InlineCtx) {
    while !RUN.swap(false, Ordering::SeqCst) {
        skyline::nn::hid::GetNpadFullKeyState(DUMMY_BLOCK.as_mut_ptr() as _, &0);
    }
}
  
#[skyline::hook(replace = change_version_string)]
fn change_version_string_hook(arg: u64, string: *const c_char) {
	let original_str = unsafe { skyline::from_c_str(string) };
	if original_str.contains("Ver.") {
        let s_ver = match std::fs::read_to_string("sd:/ultimate/mods/Ultimate S Arcropolis/version.txt") {
            Ok(version_value) => version_value.trim().to_string(),
            Err(_) => {
                #[cfg(feature = "main_nro")]
                if !is_on_ryujinx() {
                    skyline_web::dialog_ok::DialogOk::ok(
                        "Ultimate S Version unknown!",
                    );
                }

                String::from("UNKNOWN")
            }
        };
		let version_str = format!("{} / Ultimate S {}\0", original_str, s_ver);
		call_original!(arg, skyline::c_str(&version_str))
	} else {
		call_original!(arg, string)
	}
}
#[skyline::from_offset(0x37a1270)]
unsafe fn set_text_string(pane: u64, string: *const u8);

unsafe fn get_pane_by_name(arg: u64, arg2: *const u8) -> [u64; 4] {
    let func_addr = (skyline::hooks::getRegionAddress(skyline::hooks::Region::Text) as *mut u8).add(0x37752e0);
    let callable: extern "C" fn(u64, *const u8, ...) -> [u64; 4] = std::mem::transmute(func_addr);
    callable(arg, arg2)
}

unsafe fn set_room_text(arg: u64, string: String) {
    let func_addr = (skyline::hooks::getRegionAddress(skyline::hooks::Region::Text) as *mut u8).add(0x3778c50);
    let callable: extern "C" fn(u64, *const u8, usize, *const u16, ...) = std::mem::transmute(func_addr);
    callable(arg, b"mnu_online_room_inside_room_id\0".as_ptr(), 1, string.encode_utf16().collect::<Vec<u16>>().as_ptr())
}

static mut CURRENT_PANE_HANDLE: usize = 0;
static mut CURRENT_ARENA_ID: String = String::new();
static mut CURRENT_INPUT_BUFFER: isize = 4;
static mut MOST_RECENT_AUTO: isize = -1;

const MAX_INPUT_BUFFER: isize = 25;
const MIN_INPUT_BUFFER: isize = -1;

#[skyline::hook(offset = 0x1887700, inline)]
unsafe fn non_hdr_update_room_hook(_: &skyline::hooks::InlineCtx) {
    static mut CURRENT_COUNTER: usize = 0;
    if ninput::any::is_press(ninput::Buttons::RIGHT) {
        if CURRENT_COUNTER == 0 {
            CURRENT_INPUT_BUFFER += 1;
        }
        CURRENT_COUNTER = (CURRENT_COUNTER + 1) % 10;
    } else if ninput::any::is_press(ninput::Buttons::LEFT) {
        if CURRENT_COUNTER == 0 {
            CURRENT_INPUT_BUFFER -= 1;
        }
        CURRENT_COUNTER = (CURRENT_COUNTER + 1) % 10;
    } else {
        CURRENT_COUNTER = 0;
    }

    CURRENT_INPUT_BUFFER = CURRENT_INPUT_BUFFER.clamp(MIN_INPUT_BUFFER, MAX_INPUT_BUFFER);
    if CURRENT_INPUT_BUFFER == -1 {
        if MOST_RECENT_AUTO == -1 {
            set_text_string(
                CURRENT_PANE_HANDLE as u64,
                format!("ROOM ID: {}\nInput Latency: Auto\0", CURRENT_ARENA_ID).as_ptr(),
            );
        } else {
            set_text_string(
                CURRENT_PANE_HANDLE as u64,
                format!("ROOM ID: {}\nInput Latency: Auto ({})\0", CURRENT_ARENA_ID, MOST_RECENT_AUTO).as_ptr()
            )
        }
    } else {
        set_text_string(CURRENT_PANE_HANDLE as u64, format!("{}\nInput Latency: {}\0", CURRENT_ARENA_ID, CURRENT_INPUT_BUFFER).as_ptr());
    }
}

#[skyline::hook(offset = 0x188702c, inline)]
unsafe fn non_hdr_set_room_id(ctx: &skyline::hooks::InlineCtx) {
    let panel = *((*((*ctx.registers[0].x.as_ref() + 8) as *const u64) + 0x10) as *const u64);
    CURRENT_PANE_HANDLE = panel as usize;
    CURRENT_ARENA_ID = dbg!(String::from_utf16(std::slice::from_raw_parts(*ctx.registers[3].x.as_ref() as *const u16, 5)).unwrap());
}

static mut PANE: u64 = 0;

#[skyline::hook(offset = 0x1a12460)]
unsafe fn non_hdr_update_css2(arg: u64) {
    static mut CURRENT_COUNTER: usize = 0;
    if ninput::any::is_press(ninput::Buttons::X) {
        if CURRENT_COUNTER == 0 {
            CURRENT_INPUT_BUFFER += 1;
        }
        CURRENT_COUNTER = (CURRENT_COUNTER + 1) % 10;
    } else if ninput::any::is_press(ninput::Buttons::Y) {
        if CURRENT_COUNTER == 0 {
            CURRENT_INPUT_BUFFER -= 1;
        }
        CURRENT_COUNTER = (CURRENT_COUNTER + 1) % 10;
    } else {
        CURRENT_COUNTER = 0;
    }

    CURRENT_INPUT_BUFFER = CURRENT_INPUT_BUFFER.clamp(MIN_INPUT_BUFFER, MAX_INPUT_BUFFER);
    set_text_string(*((*((arg + 0xe58) as *const u64) + 0x10) as *const u64), format!("Input Latency: {}\0", CURRENT_INPUT_BUFFER).as_ptr());
    set_text_string(*((*((arg + 0xe68) as *const u64) + 0x10) as *const u64), format!("Input Latency: {}\0", CURRENT_INPUT_BUFFER).as_ptr());
    call_original!(arg)
}

static mut IS_USABLE: bool = false;

#[skyline::hook(offset = 0x16cdb08, inline)]
unsafe fn non_hdr_set_online_latency(ctx: &InlineCtx) {
    let auto = *(*ctx.registers[19].x.as_ref() as *mut u8);
    if IS_USABLE {
        MOST_RECENT_AUTO = auto as isize;
        if CURRENT_INPUT_BUFFER != -1 {
            *(*ctx.registers[19].x.as_ref() as *mut u8) = CURRENT_INPUT_BUFFER as u8;
        }
    }
}

#[skyline::hook(offset = 0x22d91f4, inline)]
unsafe fn online_melee_any_scene_create(_: &InlineCtx) {
    IS_USABLE = false;
}

#[skyline::hook(offset = 0x22d9124, inline)]
unsafe fn bg_matchmaking_seq(_: &InlineCtx) {
    IS_USABLE = false;
}

#[skyline::hook(offset = 0x22d9054, inline)]
unsafe fn arena_seq(_: &InlineCtx) {
    IS_USABLE = true;
}

#[skyline::hook(offset = 0x23599b0, inline)]
unsafe fn main_menu(_: &InlineCtx) {
    IS_USABLE = false;
}

extern "C" {
    fn update_room_hook();
}


mod util;
mod controls;
mod common;
mod cpu;

mod bayonetta;
mod brave;
mod buddy;
mod captain;
mod chrom;
mod cloud;
mod daisy;
mod dedede;
mod demon;
mod diddy;
mod dolly;
mod donkey;
mod duckhunt;
mod edge;
mod element;
mod falco;
mod fox;
mod gamewatch;
mod ganon;
mod gaogaen;
mod gekkouga;
mod ike;
mod inkling;
mod jack;
mod kamui;
mod ken;
mod kirby;
mod koopa;
mod koopajr;
mod krool;
mod link;
mod littlemac;
mod lucario;
mod lucas;
mod lucina;
mod luigi;
mod mario;
mod mariod;
mod marth;
mod master;
mod metaknight;
mod mewtwo;
mod miifighter;
mod miigunner;
mod miiswordsman;
mod murabito;
mod ness;
mod packun;
mod pacman;
mod palutena;
mod peach;
mod pichu;
mod pikachu;
mod pikmin;
mod pit;
mod pitb;
mod popo;
mod ptrainer;
mod purin;
mod rayman;
mod reflet;
mod richter;
mod ridley;
mod robot;
mod rockman;
mod rosetta;
mod roy;
mod ryu;
mod samus;
mod samusd;
mod sheik;
mod shizue;
mod shulk;
mod simon;
mod snake;
mod sonic;
mod szerosuit;
mod tantan;
mod toad;
mod toonlink;
mod trail;
mod wario;
mod wiifit;
mod wolf;
mod younglink;
mod yoshi;
mod zelda;









std::arch::global_asm!(
    r#"
    .section .nro_header
    .global __nro_header_start
    .word 0
    .word _mod_header
    .word 0
    .word 0
    
    .section .rodata.module_name
        .word 0
        .word 5
        .ascii "ultimate-s"
    .section .rodata.mod0
    .global _mod_header
    _mod_header:
        .ascii "MOD0"
        .word __dynamic_start - _mod_header
        .word __bss_start - _mod_header
        .word __bss_end - _mod_header
        .word __eh_frame_hdr_start - _mod_header
        .word __eh_frame_hdr_end - _mod_header
        .word __nx_module_runtime - _mod_header // runtime-generated module object offset
    .global IS_NRO
    IS_NRO:
        .word 1
    
    .section .bss.module_runtime
    __nx_module_runtime:
    .space 0xD0
    "#
);

#[no_mangle]
pub extern "C" fn main() {

    //runs the dependencies check function
    #[cfg(feature = "main_nro")]
    {
        quick_validate_install();
    }

    //allows online play with added chars
    unsafe { 
        extern "C" { fn allow_ui_chara_hash_online(ui_chara_hash: u64); }
        allow_ui_chara_hash_online(0xf1062d2e5); //rayman
        allow_ui_chara_hash_online(0xda4cbcb12); //toad
    }
	
	//Common
    if !is_on_ryujinx() {
        println!("We're on switch! Yay!");
        unsafe {
            OFFSET1 = calc_nnsdk_offset() + 0x429d60;
            OFFSET2 = calc_nnsdk_offset() + 0x26e94;
        }
    
        skyline::install_hooks!(
            set_interval_1,
            set_interval_2,
            run_scene_update,
            vsync_count_thread,
        );
        skyline::install_hooks!(non_hdr_set_room_id, non_hdr_update_room_hook, non_hdr_set_online_latency, online_melee_any_scene_create, bg_matchmaking_seq, arena_seq, main_menu);
        skyline::install_hooks!(change_version_string_hook);
    }
	nro::add_hook(nro_hook).unwrap();






	
	
	util::install();
	common::install();
	controls::install();
	cpu::install();
	

	//Fighters
	bayonetta::install();
	brave::install();
	buddy::install();
	
	captain::install();
	chrom::install();
	cloud::install();
	
	daisy::install();
	dedede::install();
	demon::install();
	diddy::install();
	dolly::install();
	donkey::install();
	duckhunt::install();
	
	edge::install();
	element::install();
	
	falco::install();
	fox::install();
	
	gamewatch::install();
	ganon::install();
	gaogaen::install();
	gekkouga::install();
	
	ike::install();
	inkling::install();
	
	jack::install();
	
	kamui::install();
	ken::install();
	kirby::install();
	koopa::install();
	koopajr::install();
	krool::install();
	
	link::install();
	littlemac::install();
	lucario::install();
	lucas::install();
	lucina::install();
	luigi::install();
	
	mario::install();
	mariod::install();
	marth::install();
	master::install();
	metaknight::install();
	mewtwo::install();
	miifighter::install();
	miigunner::install();
	miiswordsman::install();
	murabito::install();

	ness::install();
	
	packun::install();
	pacman::install();
	palutena::install();
	peach::install();
	pichu::install();
	pikachu::install();
	pikmin::install();
	pit::install();
	pitb::install();
	popo::install();
	ptrainer::install();
	purin::install();
	
    rayman::install();
	reflet::install();
	richter::install();
	ridley::install();
	robot::install();
	rockman::install();
	rosetta::install();
	roy::install();
	ryu::install();
	
	samus::install();
	samusd::install();
	sheik::install();
	shizue::install();
    shulk::install();
	simon::install();
	snake::install();
	sonic::install();
	szerosuit::install();
	
	tantan::install();
    toad::install();
	toonlink::install();
	trail::install();
	
	wario::install();
	wiifit::install();
	wolf::install();
	
	younglink::install();
    yoshi::install();
	
	zelda::install();

	//Stage Patching

	//Arena Ferox Screenshake
	skyline::patching::Patch::in_text(0x28444cc).data(0x52800009u32);
    skyline::patching::Patch::in_text(0x28440f4).data(0x52800009u32);
    skyline::patching::Patch::in_text(0x2844500).nop();
    skyline::patching::Patch::in_text(0x2844128).nop();
}
