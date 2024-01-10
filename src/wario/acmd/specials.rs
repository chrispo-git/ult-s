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
    agent = "wario",
    scripts =  ["game_specialssearch", "game_specialairssearch"],
    category = ACMD_GAME,
	low_priority)]
unsafe fn wario_sideb(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
	let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent); 
	let ENTRY_ID = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
		frame(fighter.lua_state_agent, 7.0);
		macros::FT_MOTION_RATE(fighter, 0.76);
		if macros::is_excute(fighter) {
			damage!(fighter, /*MSC*/ *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, /*Type*/ *DAMAGE_NO_REACTION_MODE_DAMAGE_POWER, /*DamageThreshold*/ 11);
			macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 11.0, /*Angle*/ 361, /*KBG*/ 97, /*FKB*/ 0, /*BKB*/ 60, /*Size*/ 4.0, /*X*/ 0.0, /*Y*/ 5.0, /*Z*/ 5.8, /*X2*/ Some(0.0), /*Y2*/ Some(9.7), /*Z2*/ Some(5.8), /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_PUNCH, /*Type*/ *ATTACK_REGION_BODY);
			macros::ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 11.0, /*Angle*/ 361, /*KBG*/ 97, /*FKB*/ 0, /*BKB*/ 60, /*Size*/ 4.0, /*X*/ 0.0, /*Y*/ 6.0, /*Z*/ 2.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_PUNCH, /*Type*/ *ATTACK_REGION_BODY);
		}
		frame(fighter.lua_state_agent, 11.0);
		if macros::is_excute(fighter) {
			damage!(fighter, /*MSC*/ *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, /*Type*/ *DAMAGE_NO_REACTION_MODE_NORMAL, /*DamageThreshold*/ 0);
			macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 5.0, /*Angle*/ 90, /*KBG*/ 97, /*FKB*/ 0, /*BKB*/ 60, /*Size*/ 4.0, /*X*/ 0.0, /*Y*/ 5.0, /*Z*/ 5.8, /*X2*/ Some(0.0), /*Y2*/ Some(9.7), /*Z2*/ Some(5.8), /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_PUNCH, /*Type*/ *ATTACK_REGION_BODY);
			macros::ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 5.0, /*Angle*/ 90, /*KBG*/ 97, /*FKB*/ 0, /*BKB*/ 60, /*Size*/ 4.0, /*X*/ 0.0, /*Y*/ 6.0, /*Z*/ 2.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_PUNCH, /*Type*/ *ATTACK_REGION_BODY);
		}
		frame(fighter.lua_state_agent, 25.0);
		macros::FT_MOTION_RATE(fighter, 1.0);
		if macros::is_excute(fighter) {
			AttackModule::clear_all(fighter.module_accessor);
		}
}	
#[acmd_script(
    agent = "wario",
    scripts =  ["effect_specialssearch", "effect_specialairssearch"],
    category = ACMD_EFFECT,
	low_priority)]
unsafe fn wario_sideb_eff(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
	let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent); 
	let ENTRY_ID = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
		frame(fighter.lua_state_agent, 8.0);
		if macros::is_excute(fighter) {
			macros::EFFECT_FOLLOW(fighter, Hash40::new_raw(0x11f62f23d0), Hash40::new("top"), 0, 4, 16, 0, 0, 0, 1, true);
			macros::LANDING_EFFECT(fighter, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 9, 0, 0, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, false);
			macros::LAST_EFFECT_SET_RATE(fighter, 0.6);
		}
		frame(fighter.lua_state_agent, 12.0);
		if macros::is_excute(fighter) {
			macros::FOOT_EFFECT(fighter, Hash40::new("sys_turn_smoke"), Hash40::new("top"), 9, 0, 0, 0, 0, 0, 1.5, 0, 0, 0, 0, 0, 0, false);
		}
		frame(fighter.lua_state_agent, 17.0);
		if macros::is_excute(fighter) {
			macros::FOOT_EFFECT(fighter, Hash40::new("sys_turn_smoke"), Hash40::new("top"), 9, 0, 0, 0, 0, 0, 1.3, 0, 0, 0, 0, 0, 0, false);
		}
}	
#[acmd_script(
    agent = "wario",
    scripts =  ["sound_specialssearch", "sound_specialairssearch"],
    category = ACMD_SOUND,
	low_priority)]
unsafe fn wario_sideb_snd(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
	let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent); 
	let ENTRY_ID = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
}	
#[acmd_script(
    agent = "wario",
    scripts =  ["sound_specialsdrive", "sound_specialsride", "sound_specialairs", "sound_specialairs", "sound_specials", "sound_specialsstart", "sound_specialairsstart"],
    category = ACMD_SOUND,
	low_priority)]
unsafe fn wario_sideb_start_snd(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
	let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent); 
	let ENTRY_ID = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
		if macros::is_excute(fighter) {
			macros::PLAY_SE(fighter, Hash40::new("vc_wario_006"));
			macros::PLAY_SE(fighter, Hash40::new("se_wario_attackdash"));
		}
	}	

pub fn install() {
    smashline::install_acmd_scripts!(
		wario_sideb, wario_sideb_eff, wario_sideb_snd,
        wario_sideb_start_snd
    );
}