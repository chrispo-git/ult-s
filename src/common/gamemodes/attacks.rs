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

#[skyline::hook(replace = smash::app::sv_animcmd::ATTACK)]
unsafe fn attack_replace(lua_state: u64) {
    let mut l2c_agent = L2CAgent::new(lua_state);

    let boma = smash::app::sv_system::battle_object_module_accessor(lua_state);
    let mut effs = 0;
    if is_gamemode("effects".to_string()) {
        effs = sv_math::rand(hash40("fighter"), 13);
    }
    let mut hitbox_params: Vec<L2CValue> = (0..36).map(|i| l2c_agent.pop_lua_stack(i + 1)).collect();
    l2c_agent.clear_lua_stack();
    for (i, x) in hitbox_params.iter_mut().enumerate().take(36) {
        if i == 4 && is_gamemode("angles".to_string()) {
            l2c_agent.push_lua_stack(&mut L2CValue::new_num(sv_math::rand(hash40("fighter"), 361) as f32));
        } else if i == 32 && is_gamemode("effects".to_string()) {
            l2c_agent.push_lua_stack(&mut L2CValue::new_hash(get_effect(effs)));
        } else if i == 34 && is_gamemode("effects".to_string()) {
            l2c_agent.push_lua_stack(&mut L2CValue::new_int(get_sfx(effs) as u64));
        } else {
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
}
