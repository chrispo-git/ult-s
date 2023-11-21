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
use crate::mario::*;
use super::*;
pub fn install() {
    smashline::install_acmd_scripts!(
		mario_air_sideb,
		mario_air_sideb_eff,
		mario_air_sideb_snd,
		mario_ground_sideb,
		mario_ground_sideb_eff,
		mario_ground_sideb_snd,
		mario_fireball
    );
}
#[acmd_script(
    agent = "mario",
    script =  "game_specials",
    category = ACMD_GAME,
	low_priority)]
unsafe fn mario_ground_sideb(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
		frame(fighter.lua_state_agent, 9.0);
		if macros::is_excute(fighter) {
			JostleModule::set_status(fighter.module_accessor, false);
			macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("hip"), /*Damage*/ 2.0, /*Angle*/ 53, /*KBG*/ 25, /*FKB*/ 0, /*BKB*/ 40, /*Size*/ 5.0, /*X*/ 0.0, /*Y*/ 8.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.75, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ true, /*ShieldDamage*/ -2, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_KAMEHIT, /*Type*/ *ATTACK_REGION_BODY);
		}
		frame(fighter.lua_state_agent, 53.0);
		macros::FT_MOTION_RATE(fighter, /*FSM*/ 0.8);
		if macros::is_excute(fighter) {
			JostleModule::set_status(fighter.module_accessor, true);
			AttackModule::clear_all(fighter.module_accessor);
		}
		frame(fighter.lua_state_agent, 70.0);
		if macros::is_excute(fighter) {
			CancelModule::enable_cancel(fighter.module_accessor);
		}
}			
#[acmd_script(
    agent = "mario",
    script =  "effect_specialairs",
    category = ACMD_EFFECT,
	low_priority)]
unsafe fn mario_air_sideb_eff(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
		frame(fighter.lua_state_agent, 12.0);
		for _ in 0..5 
 {
			if macros::is_excute(fighter) {
				macros::EFFECT_FOLLOW_FLIP(fighter, Hash40::new("sys_spin_wind"), Hash40::new("sys_spin_wind"), Hash40::new("hat"), 0, 0.0, 0, 0, 0, 0, 0.75, true, *EF_FLIP_YZ);
				macros::LAST_EFFECT_SET_COLOR(fighter, 1.25, 1.0875, 0.3375);
				macros::LAST_EFFECT_SET_RATE(fighter, 2.2);
				EffectModule::set_alpha_last(fighter.module_accessor, 0.5);
				macros::EFFECT_FOLLOW_FLIP(fighter, Hash40::new("sys_spin_wind"), Hash40::new("sys_spin_wind"), Hash40::new("hat"), 0, 0.0, 0, 0, 0, 0, 0.4, true, *EF_FLIP_YZ);
				macros::LAST_EFFECT_SET_COLOR(fighter, 1.25, 1.0875, 0.3375);
				macros::LAST_EFFECT_SET_RATE(fighter, 2.2);
			}
			wait(fighter.lua_state_agent, 4.0);
		}
	}
#[acmd_script(
    agent = "mario",
    script =  "effect_specials",
    category = ACMD_EFFECT,
	low_priority)]
unsafe fn mario_ground_sideb_eff(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
		frame(fighter.lua_state_agent, 10.0);
		for _ in 0..7 
 {
			if macros::is_excute(fighter) {
				macros::FOOT_EFFECT(fighter, Hash40::new("sys_dash_smoke"), Hash40::new("top"), -3, 0, 0, 0, 0, 0, 0.75, 0, 0, 0, 0, 0, 0, false);
				macros::EFFECT_FOLLOW_ALPHA(fighter, Hash40::new("sys_spin_wind"), Hash40::new("rot"), 0, 0, 0, 0, 0, 270, 0.6, true, 0.6);
				macros::LAST_EFFECT_SET_RATE(fighter, 2);
			}
			wait(fighter.lua_state_agent, 6.0);
		}
	}		
#[acmd_script(
    agent = "mario",
    script =  "sound_specials",
    category = ACMD_SOUND,
	low_priority)]
unsafe fn mario_ground_sideb_snd(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
		frame(fighter.lua_state_agent, 6.0);
		if macros::is_excute(fighter) {
			macros::PLAY_SEQUENCE(fighter, Hash40::new("seq_mario_rnd_attack"));
		}
		frame(fighter.lua_state_agent, 10.0);
		for _ in 0..4 
 {
			if macros::is_excute(fighter) {
				macros::PLAY_SE(fighter, Hash40::new("se_mario_dash_start"));
			}
			wait(fighter.lua_state_agent, 6.0);
			if macros::is_excute(fighter) {
				macros::PLAY_SE(fighter, Hash40::new("se_mario_dash_start"));
				macros::PLAY_SE(fighter, Hash40::new("se_mario_attackdash"));
			}
			wait(fighter.lua_state_agent, 6.0);
		}
	}		
#[acmd_script(
    agent = "mario",
    script =  "sound_specialairs",
    category = ACMD_SOUND,
	low_priority)]
unsafe fn mario_air_sideb_snd(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
		frame(fighter.lua_state_agent, 10.0);
		if macros::is_excute(fighter) {
			macros::PLAY_SE(fighter, Hash40::new("se_mario_appeal_s01"));
			macros::PLAY_SEQUENCE(fighter, Hash40::new("seq_mario_rnd_attack"));
		}
		wait(fighter.lua_state_agent, 3.0);
		for _ in 0..4 
 {
			if macros::is_excute(fighter) {
				macros::PLAY_SE(fighter, Hash40::new("se_mario_appeal_s02"));
			}
			wait(fighter.lua_state_agent, 3.0);
			if macros::is_excute(fighter) {
				macros::PLAY_SE(fighter, Hash40::new("se_mario_appeal_s03"));
			}
			wait(fighter.lua_state_agent, 3.0);
			if macros::is_excute(fighter) {
				macros::PLAY_SE(fighter, Hash40::new("se_mario_appeal_s04"));
			}
			wait(fighter.lua_state_agent, 3.0);
		}
		frame(fighter.lua_state_agent, 51.0);
		if macros::is_excute(fighter) {
			macros::PLAY_SE(fighter, Hash40::new("se_mario_appeal_s06"));
		}
	}	
#[acmd_script(
    agent = "mario",
    script =  "game_specialairs",
    category = ACMD_GAME,
	low_priority)]
unsafe fn mario_air_sideb(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
		macros::FT_MOTION_RATE(fighter, /*FSM*/ 2.5);
		frame(fighter.lua_state_agent, 4.0);
		macros::FT_MOTION_RATE(fighter, /*FSM*/ 0.8);
		frame(fighter.lua_state_agent, 9.0);
		macros::FT_MOTION_RATE(fighter, /*FSM*/ 1);
		if macros::is_excute(fighter) {
			macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("hat"), /*Damage*/ 8.0, /*Angle*/ 65, /*KBG*/ 100, /*FKB*/ 150, /*BKB*/ 0, /*Size*/ 13.0, /*X*/ 0.0, /*Y*/ 8.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.75, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ true, /*ShieldDamage*/ -7, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_KAMEHIT, /*Type*/ *ATTACK_REGION_BODY);
			AttackModule::set_add_reaction_frame(fighter.module_accessor, /*ID*/ 0, /*Frames*/ 9.0, /*Unk*/ false);
			ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_MARIO_GENERATE_ARTICLE_CAPPY, false, 0);
		}
		frame(fighter.lua_state_agent, 10.0);
		macros::FT_MOTION_RATE(fighter, /*FSM*/ 0.75);
		if macros::is_excute(fighter) {
			macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("hat"), /*Damage*/ 8.0, /*Angle*/ 70, /*KBG*/ 100, /*FKB*/ 150, /*BKB*/ 0, /*Size*/ 9.0, /*X*/ 0.0, /*Y*/ 8.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.75, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ true, /*ShieldDamage*/ -7, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_KAMEHIT, /*Type*/ *ATTACK_REGION_BODY);
			AttackModule::set_add_reaction_frame(fighter.module_accessor, /*ID*/ 0, /*Frames*/ 5.0, /*Unk*/ false);
			ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_MARIO_GENERATE_ARTICLE_CAPPY, false, 0);
		}
		frame(fighter.lua_state_agent, 19.0);
		if macros::is_excute(fighter) {
			macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("hat"), /*Damage*/ 8.0, /*Angle*/ 75, /*KBG*/ 100, /*FKB*/ 150, /*BKB*/ 0, /*Size*/ 5.5, /*X*/ 0.0, /*Y*/ 8.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.75, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ true, /*ShieldDamage*/ -7, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_KAMEHIT, /*Type*/ *ATTACK_REGION_BODY);
			AttackModule::set_add_reaction_frame(fighter.module_accessor, /*ID*/ 0, /*Frames*/ 2.0, /*Unk*/ false);
		}
		frame(fighter.lua_state_agent, 20.0);
		macros::FT_MOTION_RATE(fighter, /*FSM*/ 0.575);
		if macros::is_excute(fighter) {
			notify_event_msc_cmd!(fighter, 0x2127e37c07u64, *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
		}
		frame(fighter.lua_state_agent, 29.0);
		if macros::is_excute(fighter) {
			macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("hat"), /*Damage*/ 8.0, /*Angle*/ 75, /*KBG*/ 100, /*FKB*/ 150, /*BKB*/ 0, /*Size*/ 4.0, /*X*/ 0.0, /*Y*/ 8.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.75, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ true, /*ShieldDamage*/ -7, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_KAMEHIT, /*Type*/ *ATTACK_REGION_BODY);
			AttackModule::set_add_reaction_frame(fighter.module_accessor, /*ID*/ 0, /*Frames*/ -1.0, /*Unk*/ false);
		}
		frame(fighter.lua_state_agent, 35.0);
		macros::FT_MOTION_RATE(fighter, /*FSM*/ 1);
		frame(fighter.lua_state_agent, 40.0);
		if macros::is_excute(fighter) {
			AttackModule::clear_all(fighter.module_accessor);
		}
}	
#[acmd_script(
    agent = "mario",
    scripts =  ["game_specialn", "game_specialairn"],
    category = ACMD_GAME,
	low_priority)]
unsafe fn mario_fireball(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
		frame(fighter.lua_state_agent, 14.0);
		if macros::is_excute(fighter) {
			ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_MARIO_GENERATE_ARTICLE_FIREBALL, false, 0);
		}
}	