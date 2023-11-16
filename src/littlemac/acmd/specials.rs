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
		mac_neutralbdash,
		mac_neutralbairdash_eff,
		mac_neutralbdashturn,
		mac_neutralbairdashturn_eff,
		mac_neutralbairdash
    );
}
#[acmd_script(
    agent = "littlemac",
    script =  "game_specialndash",
    category = ACMD_GAME,
	low_priority)]
unsafe fn mac_neutralbdash(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
	
		frame(fighter.lua_state_agent, 4.0);
		if macros::is_excute(fighter) {
			macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("armr"), /*Damage*/ 12.0, /*Angle*/ 0, /*KBG*/ 100, /*FKB*/ 40, /*BKB*/ 0, /*Size*/ 3.5, /*X*/ 4.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ Some(-11.0), /*Y2*/ Some(0.0), /*Z2*/ Some(0.0), /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_THRU, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ true, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_G, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_PUNCH, /*Type*/ *ATTACK_REGION_PUNCH);
			macros::ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("armr"), /*Damage*/ 12.0, /*Angle*/ 367, /*KBG*/ 100, /*FKB*/ 50, /*BKB*/ 0, /*Size*/ 3.5, /*X*/ 4.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ Some(-11.0), /*Y2*/ Some(0.0), /*Z2*/ Some(0.0), /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_THRU, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ true, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_PUNCH, /*Type*/ *ATTACK_REGION_PUNCH);
			WorkModule::set_float(fighter.module_accessor, 12.0, *FIGHTER_LITTLEMAC_STATUS_SPECIAL_N_FLOAT_POWER_MIN);
			WorkModule::set_float(fighter.module_accessor, 29.0, *FIGHTER_LITTLEMAC_STATUS_SPECIAL_N_FLOAT_POWER_MAX);
			AttackModule::set_add_reaction_frame(fighter.module_accessor, /*ID*/ 0, /*Frames*/ 13.0, /*Unk*/ false);
			AttackModule::set_add_reaction_frame(fighter.module_accessor, /*ID*/ 1, /*Frames*/ 13.0, /*Unk*/ false);
		}
		frame(fighter.lua_state_agent, 18.0);
		if macros::is_excute(fighter) {
			AttackModule::clear_all(fighter.module_accessor);
		}
}	
#[acmd_script(
    agent = "littlemac",
    script =  "game_specialairndash",
    category = ACMD_GAME,
	low_priority)]
unsafe fn mac_neutralbairdash(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
	
		frame(fighter.lua_state_agent, 4.0);
		if macros::is_excute(fighter) {
			macros::SET_SPEED_EX(fighter, 2.5, -2.5, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
			macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("armr"), /*Damage*/ 10.0, /*Angle*/ 300, /*KBG*/ 100, /*FKB*/ 0, /*BKB*/ 50, /*Size*/ 3.5, /*X*/ 4.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ Some(-11.0), /*Y2*/ Some(0.0), /*Z2*/ Some(0.0), /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_THRU, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ true, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_PUNCH, /*Type*/ *ATTACK_REGION_PUNCH);
			WorkModule::set_float(fighter.module_accessor, 10.0, *FIGHTER_LITTLEMAC_STATUS_SPECIAL_N_FLOAT_POWER_MIN);
			WorkModule::set_float(fighter.module_accessor, 29.0, *FIGHTER_LITTLEMAC_STATUS_SPECIAL_N_FLOAT_POWER_MAX);
		}
		frame(fighter.lua_state_agent, 8.0);
		if macros::is_excute(fighter) {
			macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("armr"), /*Damage*/ 10.0, /*Angle*/ 361, /*KBG*/ 100, /*FKB*/ 0, /*BKB*/ 50, /*Size*/ 3.5, /*X*/ 4.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ Some(-11.0), /*Y2*/ Some(0.0), /*Z2*/ Some(0.0), /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_THRU, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ true, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_PUNCH, /*Type*/ *ATTACK_REGION_PUNCH);
			WorkModule::set_float(fighter.module_accessor, 10.0, *FIGHTER_LITTLEMAC_STATUS_SPECIAL_N_FLOAT_POWER_MIN);
			WorkModule::set_float(fighter.module_accessor, 29.0, *FIGHTER_LITTLEMAC_STATUS_SPECIAL_N_FLOAT_POWER_MAX);
		}
		frame(fighter.lua_state_agent, 11.0);
		if macros::is_excute(fighter) {
			AttackModule::clear_all(fighter.module_accessor);
		}
}		
#[acmd_script(
    agent = "littlemac",
    script =  "game_specialairndashturn",
    category = ACMD_GAME,
	low_priority)]
unsafe fn mac_neutralbairdashturn(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
	
		if macros::is_excute(fighter) {
			WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_LITTLEMAC_STATUS_SPECIAL_N_FLAG_TURN_DASH_START);
		}
		frame(fighter.lua_state_agent, 4.0);
		if macros::is_excute(fighter) {
			macros::SET_SPEED_EX(fighter, 2.5, -2.5, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
			macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("armr"), /*Damage*/ 10.0, /*Angle*/ 300, /*KBG*/ 100, /*FKB*/ 0, /*BKB*/ 50, /*Size*/ 3.5, /*X*/ 4.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ Some(-11.0), /*Y2*/ Some(0.0), /*Z2*/ Some(0.0), /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_THRU, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ true, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_PUNCH, /*Type*/ *ATTACK_REGION_PUNCH);
			WorkModule::set_float(fighter.module_accessor, 10.0, *FIGHTER_LITTLEMAC_STATUS_SPECIAL_N_FLOAT_POWER_MIN);
			WorkModule::set_float(fighter.module_accessor, 29.0, *FIGHTER_LITTLEMAC_STATUS_SPECIAL_N_FLOAT_POWER_MAX);
		}
		frame(fighter.lua_state_agent, 8.0);
		if macros::is_excute(fighter) {
			macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("armr"), /*Damage*/ 10.0, /*Angle*/ 361, /*KBG*/ 100, /*FKB*/ 0, /*BKB*/ 50, /*Size*/ 3.5, /*X*/ 4.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ Some(-11.0), /*Y2*/ Some(0.0), /*Z2*/ Some(0.0), /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_THRU, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ true, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_PUNCH, /*Type*/ *ATTACK_REGION_PUNCH);
			WorkModule::set_float(fighter.module_accessor, 10.0, *FIGHTER_LITTLEMAC_STATUS_SPECIAL_N_FLOAT_POWER_MIN);
			WorkModule::set_float(fighter.module_accessor, 29.0, *FIGHTER_LITTLEMAC_STATUS_SPECIAL_N_FLOAT_POWER_MAX);
		}
		frame(fighter.lua_state_agent, 11.0);
		if macros::is_excute(fighter) {
			AttackModule::clear_all(fighter.module_accessor);
		}
}	
#[acmd_script(
    agent = "littlemac",
    script =  "effect_specialairndash",
    category = ACMD_EFFECT,
	low_priority)]
unsafe fn mac_neutralbairdash_eff(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW(fighter, Hash40::new("littlemac_straight_line"), Hash40::new("top"), 0, 6, 0, 0, 0, 0, 1, true);
        EffectModule::enable_sync_init_pos_last(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 4.0);
	if WorkModule::get_float(fighter.module_accessor, *FIGHTER_LITTLEMAC_STATUS_WORK_ID_FLOAT_SPECIAL_N_CHARGE_RATE) >= 0.625{
        if macros::is_excute(fighter) {
            macros::EFFECT_FOLLOW(fighter, Hash40::new("littlemac_straight3"), Hash40::new("top"), 1.6, 0.2, 13.5, 0, -45, 45, 1, true);
        }
    } else {
		if WorkModule::get_float(fighter.module_accessor, *FIGHTER_LITTLEMAC_STATUS_WORK_ID_FLOAT_SPECIAL_N_CHARGE_RATE) >= 0.37{
			if macros::is_excute(fighter) {
					macros::EFFECT_FOLLOW(fighter, Hash40::new("littlemac_straight2"), Hash40::new("top"), 1.6, 0.2, 13.5, 0, -45, 45, 1, true);
			}
		} else {
			if macros::is_excute(fighter) {
				macros::EFFECT_FOLLOW(fighter, Hash40::new("littlemac_straight"), Hash40::new("top"), 1.6, 0.2, 13.5, 0, -45, 45, 1, true);
				EffectModule::enable_sync_init_pos_last(fighter.module_accessor);
			}
		}
	}
	frame(fighter.lua_state_agent, 15.0);
	if macros::is_excute(fighter) {
		macros::EFFECT_OFF_KIND(fighter, Hash40::new("littlemac_straight_line"), false, false);
		EffectModule::enable_sync_init_pos_last(fighter.module_accessor);
	}
}	
#[acmd_script(
    agent = "littlemac",
    script =  "effect_specialairndashturn",
    category = ACMD_EFFECT,
	low_priority)]
unsafe fn mac_neutralbairdashturn_eff(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW(fighter, Hash40::new("littlemac_straight_line"), Hash40::new("top"), 0, 6, 0, 0, 180, 45, 1, true);
        EffectModule::enable_sync_init_pos_last(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 4.0);
	if WorkModule::get_float(fighter.module_accessor, *FIGHTER_LITTLEMAC_STATUS_WORK_ID_FLOAT_SPECIAL_N_CHARGE_RATE) >= 0.625{
        if macros::is_excute(fighter) {
            macros::EFFECT_FOLLOW(fighter, Hash40::new("littlemac_straight3"), Hash40::new("top"),  -1.6, -1.2, -13.5, 0, 135, -45, 1, true);
        }
    } else {
		if WorkModule::get_float(fighter.module_accessor, *FIGHTER_LITTLEMAC_STATUS_WORK_ID_FLOAT_SPECIAL_N_CHARGE_RATE) >= 0.37{
			if macros::is_excute(fighter) {
					macros::EFFECT_FOLLOW(fighter, Hash40::new("littlemac_straight2"), Hash40::new("top"), -1.6, -1.2, -13.5, 0, 135, -45, 1, true);
			}
		} else {
			if macros::is_excute(fighter) {
				macros::EFFECT_FOLLOW(fighter, Hash40::new("littlemac_straight"), Hash40::new("top"), -1.6, -1.2, -13.5, 0, 135, -45, 1, true);
				EffectModule::enable_sync_init_pos_last(fighter.module_accessor);
			}
		}
	}
	frame(fighter.lua_state_agent, 15.0);
	if macros::is_excute(fighter) {
		macros::EFFECT_OFF_KIND(fighter, Hash40::new("littlemac_straight_line"), false, false);
		EffectModule::enable_sync_init_pos_last(fighter.module_accessor);
	}
}	
#[acmd_script(
    agent = "littlemac",
    script =  "game_specialndashturn",
    category = ACMD_GAME,
	low_priority)]
unsafe fn mac_neutralbdashturn(fighter: &mut L2CAgentBase) {
    	let lua_state = fighter.lua_state_agent;
		frame(fighter.lua_state_agent, 4.0);
		if macros::is_excute(fighter) {
			WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_LITTLEMAC_STATUS_SPECIAL_N_FLAG_TURN_DASH_START);
		}
		frame(fighter.lua_state_agent, 8.0);
		if macros::is_excute(fighter) {
			macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("armr"), /*Damage*/ 10.0, /*Angle*/ 0, /*KBG*/ 100, /*FKB*/ 40, /*BKB*/ 0, /*Size*/ 3.5, /*X*/ 4.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ Some(-11.0), /*Y2*/ Some(0.0), /*Z2*/ Some(0.0), /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_THRU, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ true, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_G, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_PUNCH, /*Type*/ *ATTACK_REGION_PUNCH);
			macros::ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("armr"), /*Damage*/ 10.0, /*Angle*/ 367, /*KBG*/ 100, /*FKB*/ 50, /*BKB*/ 0, /*Size*/ 3.5, /*X*/ 4.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ Some(-11.0), /*Y2*/ Some(0.0), /*Z2*/ Some(0.0), /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_THRU, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ true, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_A, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_PUNCH, /*Type*/ *ATTACK_REGION_PUNCH);
			AttackModule::set_add_reaction_frame(fighter.module_accessor, /*ID*/ 0, /*Frames*/ 13.0, /*Unk*/ false);
			AttackModule::set_add_reaction_frame(fighter.module_accessor, /*ID*/ 1, /*Frames*/ 13.0, /*Unk*/ false);
			WorkModule::set_float(fighter.module_accessor, 10.0, *FIGHTER_LITTLEMAC_STATUS_SPECIAL_N_FLOAT_POWER_MIN);
			WorkModule::set_float(fighter.module_accessor, 27.0, *FIGHTER_LITTLEMAC_STATUS_SPECIAL_N_FLOAT_POWER_MAX);
		}
		frame(fighter.lua_state_agent, 22.0);
		if macros::is_excute(fighter) {
			AttackModule::clear_all(fighter.module_accessor);
		}
}	