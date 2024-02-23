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
		daisy_dtilt, daisy_dtilt_eff,
		daisy_ftilt, daisy_ftilt_eff, daisy_ftilt_expr, daisy_ftilt_snd
	);
}
#[acmd_script(
    agent = "daisy",
    script =  "game_attacklw3",
    category = ACMD_GAME,
	low_priority)]
unsafe fn daisy_dtilt(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
		frame(fighter.lua_state_agent, 8.0);
		if macros::is_excute(fighter) {
			macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 9.0, /*Angle*/ 30, /*KBG*/ 40, /*FKB*/ 0, /*BKB*/ 50, /*Size*/ 3.8, /*X*/ 0.0, /*Y*/ 2.5, /*Z*/ 7.5, /*X2*/ Some(0.0), /*Y2*/ Some(2.5), /*Z2*/ Some(10.0), /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_PUNCH, /*Type*/ *ATTACK_REGION_KICK);
		}
		frame(fighter.lua_state_agent, 9.0);
		if macros::is_excute(fighter) {
			AttackModule::clear_all(fighter.module_accessor);
		}
		frame(fighter.lua_state_agent, 10.0);
		macros::FT_MOTION_RATE(fighter, /*FSM*/ 0.5);
		wait(fighter.lua_state_agent, 6.0);
		macros::FT_MOTION_RATE(fighter, /*FSM*/ 1);
}
#[acmd_script(
    agent = "daisy",
    script =  "effect_attacklw3",
    category = ACMD_EFFECT,
	low_priority)]
unsafe fn daisy_dtilt_eff(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
		frame(fighter.lua_state_agent, 7.0);
		if macros::is_excute(fighter) {
			macros::EFFECT_FLIP_ALPHA(fighter, Hash40::new("sys_attack_arc_b"), Hash40::new("sys_attack_arc_b"), Hash40::new("top"), 0, 4.0, 4.5, 10, -35, 8, 0.8, 0, 0, 0, 0, 0, 0, true, *EF_FLIP_YZ, 0.3);
			macros::LAST_EFFECT_SET_RATE(fighter, 1.4);
		}
}
#[acmd_script(
    agent = "daisy",
    script =  "game_attacks3",
    category = ACMD_GAME,
	low_priority)]
unsafe fn daisy_ftilt(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 9.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("havel"), 11.0, 361, 85, 0, 55, 6.0, 0.0, 3.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DAISY_FRYINGPAN, *ATTACK_REGION_OBJECT);
        macros::ATTACK(agent, 1, 0, Hash40::new("havel"), 11.0, 361, 85, 0, 55, 6.0, 0.0, -2.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DAISY_FRYINGPAN, *ATTACK_REGION_OBJECT);
    }
    frame(agent.lua_state_agent, 14.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}
#[acmd_script(
    agent = "daisy",
    script =  "effect_attacks3",
    category = ACMD_EFFECT,
	low_priority)]
unsafe fn daisy_ftilt_eff(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
		frame(fighter.lua_state_agent, 8.0);
		if macros::is_excute(fighter) {
			macros::EFFECT_FLIP_ALPHA(fighter, Hash40::new("sys_attack_arc_b"), Hash40::new("sys_attack_arc_b"), Hash40::new("top"), 0, 10, 5.5, 10, -35, 8, 1.0, 0, 0, 0, 0, 0, 0, true, *EF_FLIP_YZ, 0.3);
        	macros::LAST_EFFECT_SET_RATE(fighter, 1.4);
		}
}
#[acmd_script(
    agent = "daisy",
    script =  "sound_attacks3",
    category = ACMD_SOUND,
	low_priority)]
unsafe fn daisy_ftilt_snd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 7.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("vc_daisy_attack05"));
    }
    wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_daisy_smash_s02"));
    }
}
#[acmd_script(
    agent = "daisy",
    script =  "expression_attacks3",
    category = ACMD_EXPRESSION,
	low_priority)]
unsafe fn daisy_ftilt_expr(agent: &mut L2CAgentBase) {
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
