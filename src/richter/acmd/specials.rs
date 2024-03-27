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

#[acmd_script(
    agent = "richter",
    scripts =  ["game_specialn", "game_specialairn"],
    category = ACMD_GAME,
	low_priority)]
unsafe fn richter_neutralb(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
		macros::FT_MOTION_RATE(fighter, /*FSM*/ 0.3);
		frame(fighter.lua_state_agent, 20.0);
		macros::FT_MOTION_RATE(fighter, /*FSM*/ 0.6);
        frame(fighter.lua_state_agent, 30.0);
        macros::FT_MOTION_RATE(fighter, /*FSM*/ 1);
		frame(fighter.lua_state_agent, 31.0);
		macros::FT_MOTION_RATE(fighter, /*FSM*/ 0.75);
		if macros::is_excute(fighter) {
			ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_SIMON_GENERATE_ARTICLE_AXE, false, -1);
			ArticleModule::shoot(fighter.module_accessor, *FIGHTER_SIMON_GENERATE_ARTICLE_AXE, smash::app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_LAST), false);
		}
}	

#[acmd_script(
    agent = "richter_axe",
    script =  "game_fly",
    category = ACMD_GAME,
	low_priority)]
unsafe fn richter_axe(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
		if macros::is_excute(fighter) {
			macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 7.0, /*Angle*/ 45, /*KBG*/ 100, /*FKB*/ 50, /*BKB*/ 0, /*Size*/ 3.5, /*X*/ 0.0, /*Y*/ 6.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.75, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_SPEED, /*SetWeight*/ false, /*ShieldDamage*/ -6, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ true, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_OBJECT);
			AttackModule::enable_safe_pos(fighter.module_accessor);
		}
}			
#[acmd_script(
    agent = "richter_axe",
    script =  "effect_fly",
    category = ACMD_EFFECT,
	low_priority)]
unsafe fn richter_axe_eff(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
		if macros::is_excute(fighter) {
			macros::EFFECT_FOLLOW(fighter, Hash40::new("sys_greenshell_trace"), Hash40::new("rot"), 0, -1, -4, 0, 0, 0, 1, true);
		}
}
#[acmd_script(
    agent = "richter",
    scripts =  ["game_specials1", "game_specialairs1"],
    category = ACMD_GAME,
	low_priority)]
unsafe fn richter_sideb(agent: &mut L2CAgentBase) {
	macros::FT_MOTION_RATE(agent, 0.5);
	if macros::is_excute(agent) {
		notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
	}
    frame(agent.lua_state_agent, 10.0);
	macros::FT_MOTION_RATE(agent, 0.75);
    for _ in 0..5 {
		if macros::is_excute(agent) {
			macros::ATTACK(agent, 2, 0, Hash40::new("top"), 2.0, 10, 10, 0, 55, 4.0, 0.0, 13.0, 6.0, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_SPEED, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_whip"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_WHIP);
			macros::ATTACK(agent, 1, 0, Hash40::new("top"), 2.0, 367, 10, 0, 70, 4.0, 0.0, 13.0, -6.0, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_SPEED, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_whip"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_WHIP);
			macros::ATTACK(agent, 0, 0, Hash40::new("top"), 2.0, 40, 10, 0, 50, 4.0, 0.0, 4.0, 6.0, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_SPEED, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_whip"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_WHIP);
			macros::ATTACK(agent, 3, 0, Hash40::new("top"), 2.0, 367, 10, 0, 70, 4.0, 0.0, 4.0, -6.0, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_SPEED, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_whip"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_WHIP);
		}
		wait(agent.lua_state_agent, 2.0);
		if macros::is_excute(agent) {
			AttackModule::clear_all(agent.module_accessor);
		}
		wait(agent.lua_state_agent, 1.0);
	}
	if macros::is_excute(agent) {
		macros::ATTACK(agent, 0, 0, Hash40::new("top"), 6.0, 50, 110, 0, 65, 11.5, 0.0, 8.8, 0.5, None, None, None, 1.1, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_whip"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_WHIP);
	}
	wait(agent.lua_state_agent, 2.0);
	macros::FT_MOTION_RATE(agent, 1.0);
	if macros::is_excute(agent) {
		AttackModule::clear_all(agent.module_accessor);
	}
}	
#[acmd_script(
    agent = "richter",
    scripts =  ["effect_specials1", "effect_specialairs1"],
    category = ACMD_EFFECT,
	low_priority)]
unsafe fn richter_sideb_eff(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 7.0);
    if macros::is_excute(agent) {
        macros::FOOT_EFFECT(agent, Hash40::new("sys_dash_smoke"), Hash40::new("top"), -1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FLW_POS(agent, Hash40::new("richter_whip_dash"), Hash40::new("top"), -2.4, 9, 1, 0, 0, 0, 1, true);
		macros::LAST_EFFECT_SET_RATE(agent, 1.333);
        macros::FOOT_EFFECT(agent, Hash40::new("sys_run_smoke"), Hash40::new("top"), 8, 0, 0, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, false);
    }
    frame(agent.lua_state_agent, 13.0);
    if macros::is_excute(agent) {
        macros::FOOT_EFFECT(agent, Hash40::new("sys_run_smoke"), Hash40::new("top"), 8, 0, 0, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, false);
    }
    frame(agent.lua_state_agent, 16.0);
    if macros::is_excute(agent) {
        macros::FOOT_EFFECT(agent, Hash40::new("sys_run_smoke"), Hash40::new("top"), 8, 0, 0, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, false);
    }
    frame(agent.lua_state_agent, 35.0);
    if macros::is_excute(agent) {
        macros::FOOT_EFFECT(agent, Hash40::new("null"), Hash40::new("top"), 2, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
}
#[acmd_script(
    agent = "richter",
    scripts =  ["sound_specials1", "sound_specialairs1"],
    category = ACMD_SOUND,
	low_priority)]
unsafe fn richter_sideb_snd(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_richter_whip_holding"));
    }
    frame(agent.lua_state_agent, 8.0);
    if macros::is_excute(agent) {
        macros::PLAY_SEQUENCE(agent, Hash40::new("seq_richter_rnd_attack"));
        macros::PLAY_SE(agent, Hash40::new("se_richter_attackdash"));
    }
}

pub fn install() {
    smashline::install_acmd_scripts!(
		richter_neutralb,
        richter_axe, richter_axe_eff,
		richter_sideb, richter_sideb_eff, richter_sideb_snd
    );
}