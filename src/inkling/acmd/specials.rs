use smash::app::sv_animcmd::*;
use smash::phx::Hash40;
use smash::app::lua_bind::*;
use smash::lib::lua_const::*;
use smash::app::utility::get_kind;
use smash::hash40;
use smash::lua2cpp::*;
use smashline::*;
use smash_script::*;
use smash::lib::{L2CValue, L2CAgent};
use std::mem;
use smash::app::*;
use smash::phx::Vector3f;
use crate::util::*;
use super::*;

pub fn install() {
    Agent::new("inkling_splashbomb")
    .acmd("game_explode", ink_bomb, Priority::Low)    
    .install();
    Agent::new("inkling")
    .acmd("game_specialhidown", ink_upb_down, Priority::Low)    
    .install();
    Agent::new("inkling_squid")
    .acmd("effect_specialhi2", ink_upb_down_eff, Priority::Low)    
    .install();
}

unsafe extern "C" fn ink_bomb(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 0.0, 120, 47, 0, 60, 10.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_ink_hit"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_WATER, *ATTACK_REGION_BOMB);
        AttackModule::set_ink_value(agent.module_accessor, 110, 200.0);
        macros::QUAKE(agent, *CAMERA_QUAKE_KIND_L);
    }
    wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        AttackModule::set_size(agent.module_accessor, 0, 12.0);
    }
    wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        AttackModule::set_size(agent.module_accessor, 0, 14.0);
    }
    wait(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}
unsafe extern "C" fn ink_upb_down(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        let boma = agent.module_accessor;
        macros::ATTACK(agent, 0, 0, Hash40::new("rot"), 11.0, 45, 74, 0, 70, 10.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_ink_hit"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BOMB);
        AttackModule::set_ink_value(agent.module_accessor, 0, 80.0);
        macros::SET_SPEED_EX(agent, 4.0/3.0, -4.0, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        macros::EFFECT_FOLLOW(agent, Hash40::new("inkling_superjump_jet"), Hash40::new("hip"), -2, 0, 0, 0, 0, -90, 1, true);
		EffectModule::enable_sync_init_pos_last(agent.module_accessor);
		macros::LAST_PARTICLE_SET_COLOR(agent, WorkModule::get_float(boma, *FIGHTER_INKLING_INSTANCE_WORK_ID_FLOAT_INK_R), WorkModule::get_float(boma, *FIGHTER_INKLING_INSTANCE_WORK_ID_FLOAT_INK_G), WorkModule::get_float(boma, *FIGHTER_INKLING_INSTANCE_WORK_ID_FLOAT_INK_B));
	}
}
unsafe extern "C" fn ink_upb_down_eff(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        let otarget_id = WorkModule::get_int(agent.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER) as u32;
        let boma = smash::app::sv_battle_object::module_accessor(otarget_id);
        macros::EFFECT_FOLLOW(agent, Hash40::new("inkling_superjump_jet"), Hash40::new("head"), -2, 0, 0, 0, 0, -90, 1, true);
        EffectModule::enable_sync_init_pos_last(agent.module_accessor);
    	macros::LAST_PARTICLE_SET_COLOR(agent, WorkModule::get_float(boma, *FIGHTER_INKLING_INSTANCE_WORK_ID_FLOAT_INK_R), WorkModule::get_float(boma, *FIGHTER_INKLING_INSTANCE_WORK_ID_FLOAT_INK_G), WorkModule::get_float(boma, *FIGHTER_INKLING_INSTANCE_WORK_ID_FLOAT_INK_B));
	}
}