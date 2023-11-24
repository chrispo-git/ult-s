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
use smash::phx::Vector2f;
use crate::util::*;
use super::*;
pub fn install() {
    smashline::install_acmd_scripts!(
		byleth_dtilt,
		byleth_ftilt,
		byleth_dtilt_eff
    );
}
#[acmd_script( agent = "master", script = "game_attacks3", category = ACMD_GAME, low_priority )]
unsafe fn byleth_ftilt(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 10.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 1, 0, Hash40::new("armr"), 10.0, 35, 90, 0, 30, 2.5, 0.0, 0.0, -1.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATTACK(fighter, 2, 0, Hash40::new("sword1"), 10.0, 35, 90, 0, 30, 3.5, 3.0, 1.0, -2.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATTACK(fighter, 3, 0, Hash40::new("sword1"), 12.0, 35, 90, 0, 30, 3.5, 7.0, 1.0, -2.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATTACK(fighter, 4, 0, Hash40::new("sword1"), 12.0, 35, 90, 0, 30, 3.5, 11.5, 1.0, -2.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 2, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
	}
    frame(fighter.lua_state_agent, 13.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
        FighterAreaModuleImpl::enable_fix_jostle_area(fighter.module_accessor, 3.0, 4.0);
    }
}		
#[acmd_script(
    agent = "master",
    script =  "game_attacklw3",
    category = ACMD_GAME,
	low_priority)]
unsafe fn byleth_dtilt(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
		if macros::is_excute(fighter) {
			ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_MASTER_GENERATE_ARTICLE_SWORD, false, 0);
			ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_MASTER_GENERATE_ARTICLE_SWORD,smash::phx::Hash40::new("attack_lw3"),false,0.0);
		}
		frame(fighter.lua_state_agent, 13.0);
		macros::FT_MOTION_RATE(fighter, /*FSM*/ 1);
		if macros::is_excute(fighter) {
			macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 8.0, /*Angle*/ 85, /*KBG*/ 50, /*FKB*/ 0, /*BKB*/ 65, /*Size*/ 2.8, /*X*/ 0.0, /*Y*/ 2.8, /*Z*/ 10.0, /*X2*/ Some(0.0), /*Y2*/ Some(2.8), /*Z2*/ Some(12.5), /*Hitlag*/ 0.8, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_THRU, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
			macros::ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 8.0, /*Angle*/ 95, /*KBG*/ 50, /*FKB*/ 0, /*BKB*/ 65, /*Size*/ 2.8, /*X*/ 0.0, /*Y*/ 2.8, /*Z*/ 17.0, /*X2*/ Some(0.0), /*Y2*/ Some(2.8), /*Z2*/ Some(20.5), /*Hitlag*/ 1.0, /*SDI*/ 1.5, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_THRU, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
		}
		frame(fighter.lua_state_agent, 16.0);
		if macros::is_excute(fighter) {
			AttackModule::clear_all(fighter.module_accessor);
		}
		frame(fighter.lua_state_agent, 59.0);
		if macros::is_excute(fighter) {
			ArticleModule::remove_exist(fighter.module_accessor, *FIGHTER_MASTER_GENERATE_ARTICLE_SWORD,smash::app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
		}
}	
#[acmd_script(
    agent = "master",
    scripts =  ["effect_attacklw3"],
    category = ACMD_EFFECT,
	low_priority)]
unsafe fn byleth_dtilt_eff(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
		frame(fighter.lua_state_agent, 11.0);
		if macros::is_excute(fighter) {
			macros::EFFECT_FOLLOW(fighter, Hash40::new_raw(0x130d5298d9), Hash40::new("top"), 0, 6, 0, 15, 0, 0, 0.95, true);
			macros::LAST_EFFECT_SET_RATE(fighter, 0.8);
		}
		frame(fighter.lua_state_agent, 13.0);
		if macros::is_excute(fighter) {
			macros::EFFECT(fighter, Hash40::new_raw(0x14f1b6f422), Hash40::new("top"), 24, 0, 0, 0, 0, 0, 0.5, 0, 0, 0, 0, 0, 0, false);
			let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent); 
			if GroundModule::ray_check(boma, &Vector2f{ x: PostureModule::pos_x(boma)+(24.0*PostureModule::lr(boma)), y: PostureModule::pos_y(boma)}, &Vector2f{ x: 0.0, y: -1.0}, true) == 1 {
				macros::LANDING_EFFECT(fighter, Hash40::new_raw(0x15d7fa6259), Hash40::new("top"), 24, 0, 0, 0, 0, 0, 0.5, 0.5, 0, 0.5, 0, 0, 0, false);
			}
		}
	}	