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
		daisy_pummel_eff,
		daisy_fthrow,
		daisy_fthrow_eff,
		daisy_dthrow,
		daisy_dthrow_eff,
		daisy_uthrow_eff,
		daisy_bthrow_eff,
		daisy_remove_toad,
		daisy_catch_release_eff
	);
}
#[acmd_script(
    agent = "daisy",
    script =  "game_throwf",
    category = ACMD_GAME, 
	low_priority)]
unsafe fn daisy_fthrow(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
		if macros::is_excute(fighter) {
			macros::ATTACK_ABS(fighter, /*Kind*/ *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, /*ID*/ 0, /*Damage*/ 9.0, /*Angle*/ 45, /*KBG*/ 83, /*FKB*/ 0, /*BKB*/ 48, /*Hitlag*/ 1.2, /*Unk*/ 1.0, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*Unk*/ 0.0, /*Unk*/ true, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_PUNCH, /*Type*/ *ATTACK_REGION_THROW);
			macros::ATTACK_ABS(fighter, /*Kind*/ *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, /*ID*/ 0, /*Damage*/ 3.0, /*Angle*/ 361, /*KBG*/ 100, /*FKB*/ 0, /*BKB*/ 40, /*Hitlag*/ 0.0, /*Unk*/ 1.0, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*Unk*/ 0.0, /*Unk*/ true, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_PUNCH, /*Type*/ *ATTACK_REGION_THROW);
		}
		frame(fighter.lua_state_agent, 10.0);
		if macros::is_excute(fighter) {
			macros::ATK_HIT_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), WorkModule::get_int64(fighter.module_accessor,*FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT), WorkModule::get_int64(fighter.module_accessor,*FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP), WorkModule::get_int64(fighter.module_accessor,*FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO));
		}
}
#[acmd_script(
    agent = "daisy",
    script =  "effect_throwf",
    category = ACMD_EFFECT, 
	low_priority)]
unsafe fn daisy_fthrow_eff(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
		frame(fighter.lua_state_agent, 9.0);
		if macros::is_excute(fighter) {
			macros::FOOT_EFFECT(fighter, Hash40::new("null"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
			macros::EFFECT_FLIP_ALPHA(fighter, Hash40::new("sys_attack_arc_b"), Hash40::new("sys_attack_arc_b"), Hash40::new("top"), 0, 10, 7.5, 10, -35, 8, 0.8, 0, 0, 0, 0, 0, 0, true, *EF_FLIP_YZ, 0.3);
			macros::LAST_EFFECT_SET_RATE(fighter, 1.4);
		}
}
#[acmd_script(
    agent = "daisy",
    script =  "game_throwlw",
    category = ACMD_GAME, 
	low_priority)]
unsafe fn daisy_dthrow(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
		if macros::is_excute(fighter) {
			macros::ATTACK_ABS(fighter, /*Kind*/ *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, /*ID*/ 0, /*Damage*/ 7.0, /*Angle*/ 70, /*KBG*/ 60, /*FKB*/ 0, /*BKB*/ 60, /*Hitlag*/ 0.0, /*Unk*/ 1.0, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*Unk*/ 0.0, /*Unk*/ true, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_NONE, /*Type*/ *ATTACK_REGION_THROW);
			macros::ATTACK_ABS(fighter, /*Kind*/ *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, /*ID*/ 0, /*Damage*/ 3.0, /*Angle*/ 361, /*KBG*/ 100, /*FKB*/ 0, /*BKB*/ 40, /*Hitlag*/ 0.0, /*Unk*/ 1.0, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*Unk*/ 0.0, /*Unk*/ true, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_NONE, /*Type*/ *ATTACK_REGION_THROW);
		}
		frame(fighter.lua_state_agent, 34.0);
		if macros::is_excute(fighter) {
			macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 1.0, /*Angle*/ 270, /*KBG*/ 100, /*FKB*/ 20, /*BKB*/ 0, /*Size*/ 3.8, /*X*/ 0.0, /*Y*/ 4.5, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_KICK, /*Type*/ *ATTACK_REGION_NONE);
			AttackModule::set_catch_only_all(fighter.module_accessor, true, false);
			macros::CHECK_FINISH_CAMERA(fighter, 1, 1);
		}
		frame(fighter.lua_state_agent, 36.0);
		if macros::is_excute(fighter) {
			AttackModule::clear_all(fighter.module_accessor);
		}
		frame(fighter.lua_state_agent, 37.0);
		if macros::is_excute(fighter) {
			macros::ATK_HIT_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), WorkModule::get_int64(fighter.module_accessor,*FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT), WorkModule::get_int64(fighter.module_accessor,*FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP), WorkModule::get_int64(fighter.module_accessor,*FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO));
		}
}
#[acmd_script(
    agent = "daisy",
    script =  "effect_throwlw",
    category = ACMD_EFFECT, 
	low_priority)]
unsafe fn daisy_dthrow_eff(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
		frame(fighter.lua_state_agent, 34.0);
		if macros::is_excute(fighter) {
			macros::LANDING_EFFECT(fighter, Hash40::new("sys_landing_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.6, 0, 0, 0, 0, 0, 0, false);
		}
}
#[acmd_script(
    agent = "daisy",
    script =  "effect_throwhi",
    category = ACMD_EFFECT, 
	low_priority)]
unsafe fn daisy_uthrow_eff(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
		frame(fighter.lua_state_agent, 18.0);
		if macros::is_excute(fighter) {
			macros::EFFECT_FLIP(fighter, Hash40::new("sys_attack_speedline"), Hash40::new("sys_attack_speedline"), Hash40::new("top"), 0, 17, 0, 90, 0, 0, 1, 0, 1, 0, 0, 0, 0, true, *EF_FLIP_YZ);
		}
		frame(fighter.lua_state_agent, 26.0);
		if macros::is_excute(fighter) {
			macros::EFFECT_FLIP(fighter, Hash40::new("sys_smash_flash_s"), Hash40::new("sys_smash_flash_s"), Hash40::new("top"), 0, 27, 0, 0, 0, 0, 1.1, 0, 1, 0, 0, 0, 0, true, *EF_FLIP_YZ);
		}
		frame(fighter.lua_state_agent, 40.0);
		if macros::is_excute(fighter) {
			macros::LANDING_EFFECT_FLIP(fighter, Hash40::new("sys_landing_smoke_s"), Hash40::new("sys_landing_smoke_s"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false, *EF_FLIP_NONE);
		}
}
#[acmd_script(
    agent = "daisy",
    script =  "effect_throwb",
    category = ACMD_EFFECT, 
	low_priority)]
unsafe fn daisy_bthrow_eff(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
		frame(fighter.lua_state_agent, 18.0);
		if macros::is_excute(fighter) {
			macros::EFFECT(fighter, Hash40::new("peach_smash_heart"), Hash40::new("top"), 0, 10, -10, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
		}
		frame(fighter.lua_state_agent, 19.0);
		if macros::is_excute(fighter) {
			macros::LANDING_EFFECT_FLIP(fighter, Hash40::new("sys_atk_smoke"), Hash40::new("sys_atk_smoke"), Hash40::new("top"), -1, 0, -2, 0, 180, 0, 0.7, 0, 0, 0, 0, 0, 0, false, *EF_FLIP_NONE);
		}
		frame(fighter.lua_state_agent, 27.0);
		if macros::is_excute(fighter) {
			macros::EFFECT(fighter, Hash40::new("sys_smash_flash_s"), Hash40::new("throw"), 0, 0, 0, 0, 0, 0, 1.5, 0, 0, 0, 0, 0, 0, true);
		}
}
#[acmd_script(
    agent = "daisy",
    script =  "effect_catchattack",
    category = ACMD_EFFECT, 
	low_priority)]
unsafe fn daisy_pummel_eff(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
		if macros::is_excute(fighter) {
			macros::EFFECT_FLIP(fighter, Hash40::new("sys_run_smoke"), Hash40::new("sys_run_smoke"), Hash40::new("top"), 8, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false, *EF_FLIP_NONE);
		}
		frame(fighter.lua_state_agent, 1.0);
		if macros::is_excute(fighter) {
			macros::LANDING_EFFECT(fighter, Hash40::new("sys_down_smoke"), Hash40::new("top"), 0, 0, 11, 0, 0, 0, 0.4, 0, 0, 0, 0, 0, 0, false);
		}
}
#[acmd_script(
    agent = "daisy",
    scripts =  ["game_catchpull", "game_catchwait"],
    category = ACMD_GAME, 
	low_priority)]
unsafe fn daisy_remove_toad(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
}
#[acmd_script(
    agent = "daisy",
    scripts =  ["effect_catchcut"],
    category = ACMD_EFFECT, 
	low_priority)]
unsafe fn daisy_catch_release_eff(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
}