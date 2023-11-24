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
use crate::ike::*;
pub fn install() {
    smashline::install_acmd_scripts!(
		ike_upb4,
		ike_sideb_eff,
		ike_upb4_eff, ike_upb4_snd
    );
}
#[acmd_script(
    agent = "ike",
    scripts =  ["game_specialnend", "game_specialairnend"],
    category = ACMD_GAME,
	low_priority)]
unsafe fn ike_neutralb(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
		frame(fighter.lua_state_agent, 11.0);
		if macros::is_excute(fighter) {
			macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 9.0, /*Angle*/ 80, /*KBG*/ 100, /*FKB*/ 0, /*BKB*/ 40, /*Size*/ 15.0, /*X*/ 0.0, /*Y*/ 8.0, /*Z*/ 8.6, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 1, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_fire"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_FIRE, /*Type*/ *ATTACK_REGION_BOMB);
			macros::ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 9.0, /*Angle*/ 80, /*KBG*/ 100, /*FKB*/ 0, /*BKB*/ 40, /*Size*/ 10.0, /*X*/ 0.0, /*Y*/ 25.0, /*Z*/ 8.6, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 1, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_fire"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_FIRE, /*Type*/ *ATTACK_REGION_BOMB);
			macros::ATTACK(fighter, /*ID*/ 2, /*Part*/ 0, /*Bone*/ Hash40::new("sword"), /*Damage*/ 9.0, /*Angle*/ 85, /*KBG*/ 100, /*FKB*/ 0, /*BKB*/ 40, /*Size*/ 4.0, /*X*/ 0.0, /*Y*/ 12.1, /*Z*/ 1.6, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 1, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
			macros::ATK_SET_SHIELD_SETOFF_MUL_arg4(fighter, /*ID1*/ 0, /*ID2*/ 1,/*ID1*/ 2, /*ShieldstunMul*/ 0.05);
		}
		frame(fighter.lua_state_agent, 16.0);
		if macros::is_excute(fighter) {
			AttackModule::clear(fighter.module_accessor, /*ID*/ 0, false);
			AttackModule::clear(fighter.module_accessor, /*ID*/ 1, false);
		}
		frame(fighter.lua_state_agent, 18.0);
		if macros::is_excute(fighter) {
			AttackModule::clear_all(fighter.module_accessor);
		}
}	
#[acmd_script(
    agent = "ike",
    script =  "game_specialhi4",
    category = ACMD_GAME,
	low_priority)]
unsafe fn ike_upb4(fighter: &mut L2CAgentBase) {
		let lua_state = fighter.lua_state_agent;
		let ENTRY_ID = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
		if macros::is_excute(fighter) {
			if IKE_INSTALL[ENTRY_ID] > 0 {
				macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 6.0, /*Angle*/ 45, /*KBG*/ 155, /*FKB*/ 0, /*BKB*/ 70, /*Size*/ 10.0, /*X*/ 0.0, /*Y*/ 6.0, /*Z*/ 11.8, /*X2*/ Some(0.0), /*Y2*/ Some(11.0), /*Z2*/ Some(11.8), /*Hitlag*/ 1.2, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_fire"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_FIRE, /*Type*/ *ATTACK_REGION_SWORD);
				macros::ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 6.0, /*Angle*/ 55, /*KBG*/ 155, /*FKB*/ 0, /*BKB*/ 70, /*Size*/ 15.0, /*X*/ 0.0, /*Y*/ 8.0, /*Z*/ 18.6, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_fire"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_FIRE, /*Type*/ *ATTACK_REGION_BOMB);
				macros::ATTACK(fighter, /*ID*/ 2, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 6.0, /*Angle*/ 55, /*KBG*/ 155, /*FKB*/ 0, /*BKB*/ 70, /*Size*/ 10.0, /*X*/ 0.0, /*Y*/ 25.0, /*Z*/ 18.6, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_fire"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_FIRE, /*Type*/ *ATTACK_REGION_BOMB);
			} else {
				macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 6.0, /*Angle*/ 50, /*KBG*/ 138, /*FKB*/ 0, /*BKB*/ 70, /*Size*/ 10.0, /*X*/ 0.0, /*Y*/ 6.0, /*Z*/ 11.8, /*X2*/ Some(0.0), /*Y2*/ Some(11.0), /*Z2*/ Some(11.8), /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_IKE, /*Type*/ *ATTACK_REGION_SWORD);
			};
		};
		frame(fighter.lua_state_agent, 5.0);
		if macros::is_excute(fighter) {
			AttackModule::clear(fighter.module_accessor, 0, false);
			IKE_INSTALL[ENTRY_ID] = 0;
		};
		frame(fighter.lua_state_agent, 6.0);
		if macros::is_excute(fighter) {
			AttackModule::clear_all(fighter.module_accessor);
		};
}
#[acmd_script(
    agent = "ike",
    script =  "effect_specialhi4",
    category = ACMD_EFFECT,
	low_priority)]
unsafe fn ike_upb4_eff(fighter: &mut L2CAgentBase) {
		let lua_state = fighter.lua_state_agent;
		let ENTRY_ID = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
		if macros::is_excute(fighter) {
			macros::LANDING_EFFECT(fighter, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.1, 0, 0, 0, 0, 0, 0, false);
			macros::EFFECT_FOLLOW(fighter, Hash40::new("ike_sword"), Hash40::new("sword"), 0, 0, 0, 0, 0, 0, 1, true);
			if IKE_INSTALL[ENTRY_ID] > 0 {
				macros::EFFECT(fighter, Hash40::new("ike_volcano_ground"), Hash40::new("top"), 0, 0, 20, 0, 0, 0, 1.3, 0, 0, 0, 0, 0, 0, true);
				macros::EFFECT(fighter, Hash40::new("ike_volcano"), Hash40::new("top"), 0, 0, 20, 0, 0, 0, 1.3, 0, 0, 0, 0, 0, 0, true);
			};
		};
		frame(fighter.lua_state_agent, 5.0);
		if macros::is_excute(fighter) {
			macros::EFFECT_OFF_KIND(fighter, Hash40::new("ike_sword"), true, true);
		};
}
#[acmd_script(
    agent = "ike",
    script =  "sound_specialhi4",
    category = ACMD_SOUND,
	low_priority)]
unsafe fn ike_upb4_snd(fighter: &mut L2CAgentBase) {
		let lua_state = fighter.lua_state_agent;
		let ENTRY_ID = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
		if macros::is_excute(fighter) {
			macros::STOP_SE(fighter, Hash40::new("se_ike_special_h05"));
			macros::PLAY_SE(fighter, Hash40::new("se_ike_special_h06"));
			macros::PLAY_SE(fighter, Hash40::new("se_ike_swordgroundhit"));
			if IKE_INSTALL[ENTRY_ID] > 0 {
				macros::PLAY_SE(fighter, Hash40::new("se_ike_special_n08"));
			};
		};
}
#[acmd_script(
    agent = "ike",
    scripts =  ["effect_specialsattack", "effect_specialairsattack"],
    category = ACMD_EFFECT,
	low_priority)]
unsafe fn ike_sideb_eff(fighter: &mut L2CAgentBase) {
		let lua_state = fighter.lua_state_agent;
		let ENTRY_ID = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
		frame(fighter.lua_state_agent, 1.0);
		if macros::is_excute(fighter) {
			if IKE_INSTALL[ENTRY_ID] > 0 {
				macros::AFTER_IMAGE4_ON_arg29(fighter, Hash40::new("tex_ike_sword5"), Hash40::new("tex_ike_sword2"), 4, Hash40::new("sword"), 0.0, 1.0, 0.0, Hash40::new("sword"), -0.0, 16.2, 0.0, true, Hash40::new("ike_sword2"), Hash40::new("sword"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.1);
			} else {
				macros::AFTER_IMAGE4_ON_arg29(fighter, Hash40::new("tex_ike_sword6"), Hash40::new("tex_ike_sword2"), 4, Hash40::new("sword"), 0.0, 1.0, 0.0, Hash40::new("sword"), -0.0, 16.2, 0.0, true, Hash40::new("ike_sword"), Hash40::new("sword"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.1);
			};
		};
		frame(fighter.lua_state_agent, 12.0);
		if macros::is_excute(fighter) {
			macros::AFTER_IMAGE_OFF(fighter, 3);
		};
}