use smash::app::lua_bind::*;
use smash::lib::lua_const::*;
use smash::app::utility::get_kind;
use smash::hash40;
use smash::lua2cpp::*;
use smashline::*;
use smash_script::*;
use smash::phx::*;
use smash::lib::{L2CValue, L2CAgent};
use smash::phx::Vector2f;
use crate::util::*;
use smash::app::sv_math;

static mut CRITICAL_FRAME : [i32; 8] = [0; 8];
static mut DO_CRITICAL : [bool; 8] = [false; 8];

unsafe extern "C" fn critical(fighter: &mut L2CFighterCommon) {
    if !is_gamemode("critical".to_string()) {
        return;
    }
    let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);  
	let ENTRY_ID = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    if DO_CRITICAL[ENTRY_ID] {
        if AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_HIT) {
            CRITICAL_FRAME[ENTRY_ID] += 1;
            if CRITICAL_FRAME[ENTRY_ID] < 2 {
                SlowModule::set_whole(boma, 8, 80);
                macros::CAM_ZOOM_IN_arg5(fighter, /*frames*/ 2.0,/*no*/ 0.0,/*zoom*/ 1.8,/*yrot*/ 0.0,/*xrot*/ 0.0);
                EffectModule::req_follow(boma, Hash40::new("sys_bg_criticalhit"), Hash40::new("top"), &Vector3f{x: 0.0, y: 0.0, z: 0.0} as *const Vector3f, &Vector3f{x: 0.0, y: 0.0, z: 0.0} as *const Vector3f, 1.0, false, 0, 0, 0, 0, 0, false, false);
                macros::PLAY_SE(fighter, Hash40::new("se_common_criticalhit"));
                macros::QUAKE(fighter, *CAMERA_QUAKE_KIND_XL);
            }
            if CRITICAL_FRAME[ENTRY_ID] >= 6 {
                SlowModule::clear_whole(boma);
                CameraModule::reset_all(boma);
                EffectModule::kill_kind(boma, Hash40::new("sys_bg_criticalhit"), false, false);
                macros::CAM_ZOOM_OUT(fighter);
                DO_CRITICAL[ENTRY_ID] = false;
            }
        }
    } else {
        CRITICAL_FRAME[ENTRY_ID] = 0;
    }
}	
#[skyline::hook(replace = smash::app::sv_animcmd::ATTACK)]
unsafe fn attack_replace(lua_state: u64) {
    let mut l2c_agent = L2CAgent::new(lua_state);

    let boma = smash::app::sv_system::battle_object_module_accessor(lua_state);
    let mut ENTRY_ID = 0;
    if smash::app::utility::get_category(boma) == *BATTLE_OBJECT_CATEGORY_FIGHTER {
	    ENTRY_ID = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    }
    let mut effs = 0;
    let mut critical = 0;
    if is_gamemode("effects".to_string()) {
        effs = sv_math::rand(hash40("fighter"), 13);
    }
    if is_gamemode("critical".to_string()) {
        critical = sv_math::rand(hash40("fighter"), 8);
    }
    let mut hitbox_params: Vec<L2CValue> = (0..36).map(|i| l2c_agent.pop_lua_stack(i + 1)).collect();
    l2c_agent.clear_lua_stack();
    for (i, x) in hitbox_params.iter_mut().enumerate().take(36) {
        if i == 3 && is_gamemode("critical".to_string()) && critical == 7 {
            l2c_agent.push_lua_stack(&mut L2CValue::new_num(x.get_num() * 4.0));
            if smash::app::utility::get_category(boma) == *BATTLE_OBJECT_CATEGORY_FIGHTER {
                DO_CRITICAL[ENTRY_ID] = true;
            }
        } else if i == 4 && is_gamemode("angles".to_string()) {
            l2c_agent.push_lua_stack(&mut L2CValue::new_num(sv_math::rand(hash40("fighter"), 361) as f32));
        } else if i == 32 && is_gamemode("effects".to_string()) {
            l2c_agent.push_lua_stack(&mut L2CValue::new_hash(get_effect(effs)));
        } else if i == 34 && is_gamemode("effects".to_string()) {
            l2c_agent.push_lua_stack(&mut L2CValue::new_int(get_sfx(effs) as u64));
        } else if i == 15 && is_gamemode("critical".to_string()) && critical == 7 {
            l2c_agent.push_lua_stack(&mut L2CValue::new_num(x.get_num() * 4.0));
        }else {
            l2c_agent.push_lua_stack(x);
        }
    }
    original!()(lua_state);
}


unsafe fn get_effect(val: i32) -> u64 {
    match val {
        0 => hash40("collision_attr_fire"),
        1 => hash40("collision_attr_bury"),
        2 => hash40("collision_attr_elec"),
        3 => hash40("collision_attr_coin"),
        4 => hash40("collision_attr_paralyze"),
        5 => hash40("collision_attr_sleep"),
        6 => hash40("collision_attr_turn"),
        7 => hash40("collision_attr_flower"),
        8 => hash40("collision_attr_ice"),
        9 => hash40("collision_attr_aura"),
        10 => hash40("collision_attr_magic"),
        _ => hash40("collision_attr_normal")
    }
}
unsafe fn get_sfx(val: i32) -> i32 {
    match val {
        0 => *COLLISION_SOUND_ATTR_FIRE,
        1 => *COLLISION_SOUND_ATTR_HEAVY,
        2 => *COLLISION_SOUND_ATTR_ELEC,
        3 => *COLLISION_SOUND_ATTR_COIN,
        4 => *COLLISION_SOUND_ATTR_ELEC,
        5 => *COLLISION_SOUND_ATTR_MAGIC,
        6 => *COLLISION_SOUND_ATTR_SLAP,
        7 => *COLLISION_SOUND_ATTR_MAGIC,
        8 => *COLLISION_SOUND_ATTR_FREEZE,
        9 => *COLLISION_SOUND_ATTR_FIRE,
        10 => *COLLISION_SOUND_ATTR_MAGIC,
        _ => *COLLISION_SOUND_ATTR_PUNCH
    }
}
pub fn install() {
    skyline::install_hooks!(attack_replace);
    Agent::new("fighter")
	.on_line(Main, critical)
	.install();
}
