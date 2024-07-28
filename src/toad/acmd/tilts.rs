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
use super::super::*;

unsafe extern "C" fn toad_utilt(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
		frame(fighter.lua_state_agent, 2.0);
		if macros::is_excute(fighter) {
			ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_MURABITO_GENERATE_ARTICLE_UMBRELLA, false, 0);
			ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_MURABITO_GENERATE_ARTICLE_UMBRELLA,smash::phx::Hash40::new("attack_air_hi"),false,0.0);
		}
		frame(fighter.lua_state_agent, 6.0);
		if macros::is_excute(fighter) {
			macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("head"), /*Damage*/ 10.0, /*Angle*/ 90, /*KBG*/ 100, /*FKB*/ 0, /*BKB*/ 50, /*Size*/ 7.5, /*X*/ 4.4, /*Y*/ -1.0, /*Z*/ 0.0, /*X*/ None, /*Y*/ None, /*Z*/ None, /*Hitlag*/ 1.0, /*SDI*/ 0.8, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_sting"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_PUNCH);
			macros::ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("legc"), /*Damage*/ 6.5, /*Angle*/ 90, /*KBG*/ 100, /*FKB*/ 0, /*BKB*/ 50, /*Size*/ 4.5, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X*/ None, /*Y*/ None, /*Z*/ None, /*Hitlag*/ 1.0, /*SDI*/ 0.8, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_PUNCH, /*Type*/ *ATTACK_REGION_PUNCH);
		}
		frame(fighter.lua_state_agent, 13.0);
		if macros::is_excute(fighter) {
			AttackModule::clear_all(fighter.module_accessor);
		}
		frame(fighter.lua_state_agent, 20.0);
		if macros::is_excute(fighter) {
			ArticleModule::remove_exist(fighter.module_accessor, *FIGHTER_MURABITO_GENERATE_ARTICLE_UMBRELLA,smash::app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
		}
}	
unsafe extern "C" fn toad_utilt_snd(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	frame(fighter.lua_state_agent, 5.0);
	if macros::is_excute(fighter) {
		attack_vc(fighter);
		macros::PLAY_SE(fighter, Hash40::new("se_murabito_swing_s"));
	};
}
unsafe extern "C" fn toad_utilt_eff(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
		frame(fighter.lua_state_agent, 6.0);
		if macros::is_excute(fighter) {
			macros::EFFECT_FOLLOW(fighter, Hash40::new("sys_jump_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, false);
			macros::EFFECT_FOLLOW_FLIP_ALPHA(fighter, Hash40::new("sys_attack_speedline"), Hash40::new("sys_attack_speedline"), Hash40::new("top"), 0, 4, 0, -90, 0, 0, 1.1, true, *EF_FLIP_YZ, 0.3);
		}
		frame(fighter.lua_state_agent, 24.0);
		if macros::is_excute(fighter) {
			macros::LANDING_EFFECT(fighter, Hash40::new("sys_down_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.4, 0, 0, 0, 0, 0, 0, false);
		}
}	

unsafe extern "C" fn toad_ftilt(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
		frame(fighter.lua_state_agent, 2.0);
		macros::FT_MOTION_RATE(fighter, 0.5);
		if macros::is_excute(fighter) {
			ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_MURABITO_GENERATE_ARTICLE_UMBRELLA, false, 0);
			ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_MURABITO_GENERATE_ARTICLE_UMBRELLA,smash::phx::Hash40::new("attack_s3_s"),false,0.0);
		}
		wait(fighter.lua_state_agent, 10.0);
		macros::FT_MOTION_RATE(fighter, 1.0);
		frame(fighter.lua_state_agent, 13.0);
		if macros::is_excute(fighter) {
			macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("haver"), /*Damage*/ 11.0, /*Angle*/ 38, /*KBG*/ 76, /*FKB*/ 0, /*BKB*/ 48, /*Size*/ 4.5, /*X*/ 0.0, /*Y*/ 9.5, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_KICK, /*Type*/ *ATTACK_REGION_OBJECT);
			macros::ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("haver"), /*Damage*/ 11.0, /*Angle*/ 38, /*KBG*/ 76, /*FKB*/ 0, /*BKB*/ 48, /*Size*/ 4.5, /*X*/ 0.0, /*Y*/ 4.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_KICK, /*Type*/ *ATTACK_REGION_OBJECT);
		}
		frame(fighter.lua_state_agent, 16.0);
		macros::FT_MOTION_RATE(fighter, /*FSM*/ 2.111);
		if macros::is_excute(fighter) {
			AttackModule::clear_all(fighter.module_accessor);
		}
		frame(fighter.lua_state_agent, 24.0);
		if macros::is_excute(fighter) {
			ArticleModule::remove_exist(fighter.module_accessor, *FIGHTER_MURABITO_GENERATE_ARTICLE_UMBRELLA,smash::app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
		}
}	
unsafe extern "C" fn toad_ftilt_eff(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
		frame(fighter.lua_state_agent, 12.0);
		if macros::is_excute(fighter) {
			macros::EFFECT_FOLLOW_FLIP(fighter, Hash40::new("sys_attack_arc_c"), Hash40::new("sys_attack_arc_c"), Hash40::new("top"), -1, 10.5, 3, 0, -50, -90, 1.0, true, *EF_FLIP_YZ);
			macros::LAST_EFFECT_SET_RATE(fighter, 1.7);
		}
		frame(fighter.lua_state_agent, 13.0);
		if macros::is_excute(fighter) {
			macros::LANDING_EFFECT(fighter, Hash40::new("sys_soil_landing"), Hash40::new("top"), 11.5, 0, 0, 0, 0, 0, 0.55, 0, 0, 0, 0, 0, 0, false);
		}
		frame(fighter.lua_state_agent, 23.0);
		if macros::is_excute(fighter) {
			macros::EFFECT(fighter, Hash40::new("sys_erace_smoke"), Hash40::new("top"), 10, 2, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false);
		}
}	
unsafe extern "C" fn toad_ftilt_snd(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	frame(fighter.lua_state_agent, 14.0);
	if macros::is_excute(fighter) {
		attack_vc(fighter);
		macros::PLAY_SE(fighter, Hash40::new("se_murabito_swing_l"));
	};
}	

unsafe extern "C" fn toad_dtilt(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
		if macros::is_excute(fighter) {
			ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_MURABITO_GENERATE_ARTICLE_UMBRELLA, false, 0);
			ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_MURABITO_GENERATE_ARTICLE_UMBRELLA,smash::phx::Hash40::new("attack_lw3"),false,0.0);
		}
		frame(fighter.lua_state_agent, 7.0);
		if macros::is_excute(fighter) {
			macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("haver"), /*Damage*/ 8.0, /*Angle*/ 80, /*KBG*/ 50, /*FKB*/ 0, /*BKB*/ 50, /*Size*/ 3.3, /*X*/ 0.0, /*Y*/ 3.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_PUNCH, /*Type*/ *ATTACK_REGION_TAIL);
			macros::ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("haver"), /*Damage*/ 8.0, /*Angle*/ 80, /*KBG*/ 50, /*FKB*/ 0, /*BKB*/ 50, /*Size*/ 4.0, /*X*/ 0.0, /*Y*/ 6.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_PUNCH, /*Type*/ *ATTACK_REGION_TAIL);
			macros::ATK_SET_SHIELD_SETOFF_MUL(fighter, /*ID*/ 0, /*ShieldstunMul*/ 0.4);
			macros::ATK_SET_SHIELD_SETOFF_MUL(fighter, /*ID*/ 1, /*ShieldstunMul*/ 0.4);
			AttackModule::set_attack_height_all(fighter.module_accessor, smash::app::AttackHeight(*ATTACK_HEIGHT_LOW), false);
		}
		wait(fighter.lua_state_agent, 2.0);
		if macros::is_excute(fighter) {
			AttackModule::clear_all(fighter.module_accessor);
		}
		frame(fighter.lua_state_agent, 23.0);
		if macros::is_excute(fighter) {
			ArticleModule::remove_exist(fighter.module_accessor, *FIGHTER_MURABITO_GENERATE_ARTICLE_UMBRELLA,smash::app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
		}
}	
unsafe extern "C" fn toad_dtilt_eff(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
		
}	
unsafe extern "C" fn toad_dtilt_snd(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	frame(fighter.lua_state_agent, 6.0);
	if macros::is_excute(fighter) {
		attack_vc(fighter);
		macros::PLAY_SE(fighter, Hash40::new("se_murabito_smash_h01"));
	};
}	
unsafe extern "C" fn toad_dtilt_expr(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
		if macros::is_excute(fighter) {
			ItemModule::set_have_item_visibility(fighter.module_accessor, false, 0);
		}
		frame(fighter.lua_state_agent, 40.0);
		if macros::is_excute(fighter) {
			ItemModule::set_have_item_visibility(fighter.module_accessor, true, 0);
		}
}	

pub fn install() {
    Agent::new("murabito")
        .game_acmd("game_attackhi3toad", toad_utilt, Priority::Low)
        .sound_acmd("sound_attackhi3toad", toad_utilt_snd, Priority::Low)
        .effect_acmd("effect_attackhi3toad", toad_utilt_eff, Priority::Low)
        .game_acmd("game_attacks3toad", toad_ftilt, Priority::Low)
        .effect_acmd("effect_attacks3toad", toad_ftilt_eff, Priority::Low)
        .sound_acmd("sound_attacks3toad", toad_ftilt_snd, Priority::Low)
        .game_acmd("game_attacklw3toad", toad_dtilt, Priority::Low)
        .effect_acmd("effect_attacklw3toad", toad_dtilt_eff, Priority::Low)
        .sound_acmd("sound_attacklw3toad", toad_dtilt_snd, Priority::Low)
        .expression_acmd("expression_attacklw3toad", toad_dtilt_expr, Priority::Low)
        .install();
}