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
    smashline::install_acmd_scripts!(
		lucario_auraball,
		lucario_downb,
		lucario_downb_eff,
		lucario_sound_downb
    );
}
#[acmd_script( agent = "lucario_auraball", script = "game_shoot", category = ACMD_GAME, low_priority )]
unsafe fn lucario_auraball(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 10.0, 361, 42, 0, 14, 2.2, 0.0, 0.0, 0.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, -2.3, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_aura"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_NONE);
        macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 25.0, 40, 65, 0, 45, 2.2, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, -4, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_aura"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_NONE);
        attack!(fighter, *MA_MSC_CMD_ATTACK_SET_LERP, 0, 1);
        AttackModule::enable_safe_pos(fighter.module_accessor);
    }
}
#[acmd_script(
    agent = "lucario",
    scripts =  ["game_speciallw", "game_specialairlw"],
    category = ACMD_GAME,
	low_priority)]
unsafe fn lucario_downb(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
	if macros::is_excute(fighter) {
		JostleModule::set_status(fighter.module_accessor, false);
	}
	frame(fighter.lua_state_agent, 32.0);
	if macros::is_excute(fighter) {
		JostleModule::set_status(fighter.module_accessor, true);
	}
}	
#[acmd_script(
    agent = "lucario",
    scripts =  ["sound_speciallw", "sound_specialairlw"],
    category = ACMD_SOUND,
	low_priority)]
unsafe fn lucario_sound_downb(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
		frame(fighter.lua_state_agent, 3.0);
		if macros::is_excute(fighter) {
			macros::PLAY_SE_REMAIN(fighter, Hash40::new("vc_lucario_007"));
		}
		wait(fighter.lua_state_agent, 1.0);
		if macros::is_excute(fighter) {
			macros::PLAY_SE_REMAIN(fighter, Hash40::new("se_lucario_special_l02"));
		}
}		
#[acmd_script(
    agent = "lucario",
    scripts =  ["effect_speciallw", "effect_specialairlw"],
    category = ACMD_EFFECT,
	low_priority)]
unsafe fn lucario_downb_eff(fighter: &mut L2CAgentBase) {
    	let lua_state = fighter.lua_state_agent;
		if macros::is_excute(fighter) {
			macros::EFFECT_FOLLOW(fighter, Hash40::new("lucario_kagebunshin"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, true);
			macros::FLASH(fighter, 1, 1, 1, 0.75);
		}
		frame(fighter.lua_state_agent, 3.0);
		for _ in 0..4 {
			if macros::is_excute(fighter) {
				macros::FLASH(fighter, 0.7, 0.7, 0.7, 0.5);
			}
			wait(fighter.lua_state_agent, 2.0);
			if macros::is_excute(fighter) {
				macros::FLASH(fighter, 0.67, 0, 0.78, 0.31);
			}
			wait(fighter.lua_state_agent, 2.0);
			if macros::is_excute(fighter) {
				macros::COL_NORMAL(fighter);
			}
			wait(fighter.lua_state_agent, 2.0);
		}
		frame(fighter.lua_state_agent, 27.0);
		if macros::is_excute(fighter) {
			macros::EFFECT_OFF_KIND(fighter, Hash40::new("lucario_kagebunshin"), false, false);
		}
}