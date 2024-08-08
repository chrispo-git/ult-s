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
	Agent::new("daisy")
    .acmd("game_attacklw3", daisy_dtilt, Priority::Low)    
    .acmd("game_attacks3", daisy_ftilt, Priority::Low)    
    .acmd("effect_attacks3", daisy_ftilt_eff, Priority::Low)    
    .acmd("sound_attacks3", daisy_ftilt_snd, Priority::Low)    
    .acmd("expression_attacks3", daisy_ftilt_expr, Priority::Low)    
    .install();
}

unsafe extern "C" fn daisy_dtilt(agent: &mut L2CAgentBase) {
		frame(agent.lua_state_agent, 1.0);
		if macros::is_excute(agent) {
			FighterAreaModuleImpl::enable_fix_jostle_area(agent.module_accessor, 1.0, 4.0);
		}
		frame(agent.lua_state_agent, 6.0);
		if macros::is_excute(agent) {
			FighterAreaModuleImpl::enable_fix_jostle_area(agent.module_accessor, 3.0, 6.0);
		}
		frame(agent.lua_state_agent, 8.0);
		if macros::is_excute(agent) {
			macros::ATTACK(agent, 0, 0, Hash40::new("top"), 9.0, 30, 40, 0, 50, 3.8, 0.0, 3.2, 6.5, Some(0.0), Some(2.8), Some(10.2), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_KICK);
			macros::ATTACK(agent, 1, 0, Hash40::new("top"), 9.0, 30, 40, 0, 50, 3.8, 0.0, 3.2, 6.5, Some(0.0), Some(2.8), Some(10.2), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_KICK);
			AttackModule::set_attack_height_all(agent.module_accessor, smash::app::AttackHeight(*ATTACK_HEIGHT_LOW), false);
		}
		frame(agent.lua_state_agent, 10.0);
		if macros::is_excute(agent) {
			AttackModule::clear_all(agent.module_accessor);
			FighterAreaModuleImpl::enable_fix_jostle_area(agent.module_accessor, 3.0, 3.0);
		}
}
unsafe extern "C" fn daisy_ftilt(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 9.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("havel"), 11.0, 361, 85, 0, 55, 6.0, 0.0, 3.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DAISY_FRYINGPAN, *ATTACK_REGION_OBJECT);
        macros::ATTACK(agent, 1, 0, Hash40::new("havel"), 11.0, 361, 85, 0, 55, 3.5, 0.0, -2.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DAISY_FRYINGPAN, *ATTACK_REGION_OBJECT);
    }
    frame(agent.lua_state_agent, 14.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}
unsafe extern "C" fn daisy_ftilt_eff(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
		frame(fighter.lua_state_agent, 8.0);
		if macros::is_excute(fighter) {
			macros::EFFECT_FLIP_ALPHA(fighter, Hash40::new("sys_attack_arc_b"), Hash40::new("sys_attack_arc_b"), Hash40::new("top"), 0, 10, 5.5, 10, -35, 8, 1.0, 0, 0, 0, 0, 0, 0, true, *EF_FLIP_YZ, 0.3);
        	macros::LAST_EFFECT_SET_RATE(fighter, 1.4);
		}
}
unsafe extern "C" fn daisy_ftilt_snd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 7.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("vc_daisy_attack05"));
    }
    wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_daisy_smash_s02"));
    }
}
unsafe extern "C" fn daisy_ftilt_expr(agent: &mut L2CAgentBase) {
	if macros::is_excute(agent) {
		ItemModule::set_have_item_visibility(agent.module_accessor, false, 0);
		VisibilityModule::set_int64(agent.module_accessor, hash40("smash_item") as i64, hash40("smash_item_pan") as i64);
	}
	frame(agent.lua_state_agent, 19.0);
	if macros::is_excute(agent) {
		ItemModule::set_have_item_visibility(agent.module_accessor, false, 0);
		VisibilityModule::set_int64(agent.module_accessor, hash40("smash_item") as i64, hash40("smash_item_none") as i64);
	}
}
