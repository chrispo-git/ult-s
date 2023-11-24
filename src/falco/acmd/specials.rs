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
		falco_shine,
		falco_shine_eff,
		falco_laser
    );
}	
#[acmd_script(
    agent = "falco_blaster_bullet",
    script =  "game_fly",
    category = ACMD_GAME,
	low_priority)]
unsafe fn falco_laser(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
		if macros::is_excute(fighter) {
			macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 3.0, /*Angle*/ 361, /*KBG*/ 100, /*FKB*/ 30, /*BKB*/ 0, /*Size*/ 1.44, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 0.8, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.1, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_SPEED, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ true, /*Absorbable*/ true, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_elec"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_ELEC, /*Type*/ *ATTACK_REGION_ENERGY);
			AttackModule::enable_safe_pos(fighter.module_accessor);
		}
		frame(fighter.lua_state_agent, 8.0);
		if macros::is_excute(fighter) {
			macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 2.0, /*Angle*/ 361, /*KBG*/ 100, /*FKB*/ 7, /*BKB*/ 0, /*Size*/ 1.44, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 0.8, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.1, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_SPEED, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ true, /*Absorbable*/ true, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_elec"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_ELEC, /*Type*/ *ATTACK_REGION_ENERGY);
		}
		frame(fighter.lua_state_agent, 15.0);
		if macros::is_excute(fighter) {
			macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 0.8, /*Angle*/ 361, /*KBG*/ 100, /*FKB*/ 1, /*BKB*/ 0, /*Size*/ 1.44, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 0.8, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.1, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_SPEED, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ true, /*Absorbable*/ true, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_elec"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_ELEC, /*Type*/ *ATTACK_REGION_ENERGY);
		}
}	
#[acmd_script(
    agent = "falco",
    scripts =  ["game_speciallw", "game_specialairlw"],
    category = ACMD_GAME,
	low_priority)]
unsafe fn falco_shine(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
		frame(fighter.lua_state_agent, 4.0);
		if macros::is_excute(fighter) {
			macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 8.0, /*Angle*/ 84, /*KBG*/ 50, /*FKB*/ 0, /*BKB*/ 80, /*Size*/ 7.5, /*X*/ 0.0, /*Y*/ 7.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.25, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ -5, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_elec"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_ELEC, /*Type*/ *ATTACK_REGION_ENERGY);
			macros::ATK_SET_SHIELD_SETOFF_MUL(fighter, /*ID*/ 0, /*ShieldstunMul*/ 0.1);
			AttackModule::enable_safe_pos(fighter.module_accessor);
		}
		frame(fighter.lua_state_agent, 6.0);
		if macros::is_excute(fighter) {
			AttackModule::clear_all(fighter.module_accessor);
			shield!(fighter, *MA_MSC_CMD_SHIELD_OFF, /*Type*/ *COLLISION_KIND_REFLECTOR, /*ID*/ *FIGHTER_FALCO_REFLECTOR_KIND_REFLECTOR, *FIGHTER_REFLECTOR_GROUP_EXTEND);
		}
		frame(fighter.lua_state_agent, 15.0);
		macros::FT_MOTION_RATE(fighter, /*FSM*/ 1.75);
}		
#[acmd_script(
    agent = "falco",
    scripts =  ["effect_speciallw", "effect_specialairlw"],
    category = ACMD_EFFECT,
	low_priority)]
unsafe fn falco_shine_eff(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
		frame(fighter.lua_state_agent, 2.0);
		if macros::is_excute(fighter) {
			macros::EFFECT_FLW_POS(fighter, Hash40::new_raw(0x0ecca63d69), Hash40::new("top"), 0.0, 7.0, 0, 0, 0, 0, 1.2, true);
			EffectModule::preset_limit_num(fighter.module_accessor, 2);
			macros::EFFECT_FOLLOW(fighter, Hash40::new_raw(0x0fdc7fb0a0), Hash40::new("top"), 0.0, 7.0, 0, 0, 0, 0, 1.2, true);
			macros::FLASH(fighter, 1, 1, 1, 0.627);
		}
		frame(fighter.lua_state_agent, 3.0);
		if macros::is_excute(fighter) {
			macros::EFFECT_FLW_POS(fighter, Hash40::new_raw(0x0fecc8ba2c), Hash40::new("reflector"), 1.4, -0.6, 0, 0, 0, 0, 1, true);
			macros::FLASH(fighter, 0, 1, 1, 0.431);
			macros::FLASH_FRM(fighter, 20, 0, 0.706, 0.392, 0);
		}
		frame(fighter.lua_state_agent, 9.0);
		if macros::is_excute(fighter) {
			macros::COL_NORMAL(fighter);
		}
		frame(fighter.lua_state_agent, 18.0);
		if macros::is_excute(fighter) {
			macros::EFFECT_OFF_KIND(fighter, Hash40::new_raw(0x0ecca63d69), false, false);
			macros::EFFECT_OFF_KIND(fighter, Hash40::new_raw(0x0d30ab52b6), false, false);
			macros::EFFECT_OFF_KIND(fighter, Hash40::new_raw(0x0fdc7fb0a0), true, false);
		}
}			