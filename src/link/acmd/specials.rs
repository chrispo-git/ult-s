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
use crate::link::*;
use super::*;

pub fn install() {
    Agent::new("link")
    .acmd("game_specialairnstart", link_arrow_air, Priority::Low)    
    .acmd("game_specialnstart", link_arrow, Priority::Low)    
    .acmd("game_speciallwblast", link_downb, Priority::Low)    
    .acmd("game_specialairlwblast", link_downb, Priority::Low)    
    .acmd("effect_speciallwblast", link_downb_eff, Priority::Low)    
    .acmd("effect_specialairlwblast", link_downb_eff, Priority::Low)    
    .acmd("sound_speciallwblast", link_downb_snd, Priority::Low)    
    .acmd("sound_specialairlwblast", link_downb_snd, Priority::Low)    
    .install();

	Agent::new("link_boomerang")
    .acmd("game_fly", linkerang, Priority::Low)    
    .install();
}

unsafe extern "C" fn link_arrow_air(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
		if macros::is_excute(fighter) {
			ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_LINK_GENERATE_ARTICLE_BOWARROW, false, 0);
			ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_LINK_GENERATE_ARTICLE_BOWARROW,smash::phx::Hash40::new("n_air_start"),false,0.0);
		}
		macros::FT_MOTION_RATE(fighter, /*FSM*/ 0.722222);
		frame(fighter.lua_state_agent, 18.0);
		macros::FT_MOTION_RATE(fighter, /*FSM*/ 1);
		if macros::is_excute(fighter) {
			WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_LINK_STATUS_BOW_FLAG_CHARGE);
		}
} 
unsafe extern "C" fn link_arrow(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
		if macros::is_excute(fighter) {
			ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_LINK_GENERATE_ARTICLE_BOWARROW, false, 0);
			ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_LINK_GENERATE_ARTICLE_BOWARROW,smash::phx::Hash40::new("n_start"),false,0.0);
		}
		macros::FT_MOTION_RATE(fighter, /*FSM*/ 0.722222);
		frame(fighter.lua_state_agent, 18.0);
		macros::FT_MOTION_RATE(fighter, /*FSM*/ 1);
		if macros::is_excute(fighter) {
			WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_LINK_STATUS_BOW_FLAG_CHARGE);
		}
} 

unsafe extern "C" fn link_downb(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
	if StatusModule::status_kind(fighter.module_accessor) == *FIGHTER_LINK_STATUS_KIND_SPECIAL_LW_BLAST {
			if macros::is_excute(fighter) {
				WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_LINK_STATUS_WORK_ID_FLAG_BOMB_BLAST);
			}
	} else {
			if macros::is_excute(fighter) {
				ArticleModule::change_status_exist(fighter.module_accessor, *FIGHTER_LINK_GENERATE_ARTICLE_BOW, *WN_LINK_BOW_STATUS_KIND_BACK);
			}
			frame(fighter.lua_state_agent, 11.0);
			if macros::is_excute(fighter) {
				macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 6.0, /*Angle*/ 45, /*KBG*/ 30, /*FKB*/ 0, /*BKB*/ 30, /*Size*/ 6.5, /*X*/ 0.0, /*Y*/ 10.0, /*Z*/ 20.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_paralyze"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_ELEC, /*Type*/ *ATTACK_REGION_ENERGY);
				macros::ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 8.0, /*Angle*/ 80, /*KBG*/ 90, /*FKB*/ 0, /*BKB*/ 40, /*Size*/ 11.5, /*X*/ 0.0, /*Y*/ 10.0, /*Z*/ 20.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.8, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_elec"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_ELEC, /*Type*/ *ATTACK_REGION_ENERGY);
				macros::ATK_SET_SHIELD_SETOFF_MUL(fighter, /*ID*/ 0, /*ShieldstunMul*/ 0.1);
				macros::ATK_SET_SHIELD_SETOFF_MUL(fighter, /*ID*/ 1, /*ShieldstunMul*/ 0.1);
			}
			frame(fighter.lua_state_agent, 16.0);
			if macros::is_excute(fighter) {
				AttackModule::clear_all(fighter.module_accessor);
			}
	};
}		
unsafe extern "C" fn link_downb_eff(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
	if StatusModule::status_kind(fighter.module_accessor) != *FIGHTER_LINK_STATUS_KIND_SPECIAL_LW_BLAST {
			if macros::is_excute(fighter) {
				macros::EFFECT_OFF_KIND(fighter, Hash40::new("sys_smash_flash"), false, true);
			}
			frame(fighter.lua_state_agent, 10.0);
			if macros::is_excute(fighter) {
				let eff: u32 = EffectModule::req_follow(fighter.module_accessor, smash::phx::Hash40::new("link_swordbeam_hit"), smash::phx::Hash40::new("top"), &SHOOT, &NONE, 0.85, true, 0, 0, 0, 0, 0, true, true) as u32;
				EffectModule::set_rgb(fighter.module_accessor, eff, 6.95, 1.5, 0.06);
				EffectModule::set_rate(fighter.module_accessor, eff, 0.65);
			};
	};
}	
unsafe extern "C" fn link_downb_snd(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
	if StatusModule::status_kind(fighter.module_accessor) != *FIGHTER_LINK_STATUS_KIND_SPECIAL_LW_BLAST {
					frame(fighter.lua_state_agent, 10.0);
			if macros::is_excute(fighter) {
				macros::PLAY_SEQUENCE(fighter, Hash40::new("seq_link_rnd_attack"));
				macros::PLAY_STATUS(fighter, Hash40::new("se_link_smash_s01"));
				macros::PLAY_STATUS(fighter, Hash40::new("se_link_smash_s01"));
				macros::PLAY_STATUS(fighter, Hash40::new("se_link_smash_s01"));
			}
			};
}	
unsafe extern "C" fn linkerang(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
		if macros::is_excute(fighter) {
			macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 4.0, /*Angle*/ 70, /*KBG*/ 40, /*FKB*/ 0, /*BKB*/ 80, /*Size*/ 4.0, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_SPEED, /*SetWeight*/ false, /*ShieldDamage*/ -4, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ true, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_KICK, /*Type*/ *ATTACK_REGION_OBJECT);
			AttackModule::enable_safe_pos(fighter.module_accessor);
		}
		frame(fighter.lua_state_agent, 10.0);
		if macros::is_excute(fighter) {
			macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 3.0, /*Angle*/ 65, /*KBG*/ 40, /*FKB*/ 0, /*BKB*/ 60, /*Size*/ 3.6, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_SPEED, /*SetWeight*/ false, /*ShieldDamage*/ -3, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ true, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_KICK, /*Type*/ *ATTACK_REGION_OBJECT);
		}
}