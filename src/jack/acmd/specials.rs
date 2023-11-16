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
use crate::jack::*;
use super::*;
pub fn install() {
    smashline::install_acmd_scripts!(
		joker_eiagon,
		joker_eiagon_air,
		joker_gun,
		joker_gun_air,
		joker_gun_b,
		joker_gun_b_air,
		s_joker_gun,
		s_joker_gun_b,
		e_joker_gun,
		e_joker_gun_b,
		
		//Baton Pass
		joker_sideb,
		joker_sideb_eff,
		joker_sideb_snd
    );
}
	
#[acmd_script(
    agent = "jack",
    scripts =  ["game_specialn1_ex", "game_specialn1"],
    category = ACMD_GAME,
	low_priority)]
unsafe fn joker_gun(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent); 
	let status_kind = smash::app::lua_bind::StatusModule::status_kind(boma);
	let ENTRY_ID = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
		if macros::is_excute(fighter) {
			WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_JACK_STATUS_SPECIAL_N_FLAG_UNABLE_SPECIAL_N);
		}
		frame(fighter.lua_state_agent, 12.0);
		if macros::is_excute(fighter) {
			if GUN_C[ENTRY_ID] == ATTACK_N{
				macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 5.0, /*Angle*/ 361, /*KBG*/ 40, /*FKB*/ 0, /*BKB*/ 75, /*Size*/ 4.5, /*X*/ 0.0, /*Y*/ 11.0, /*Z*/ 9.0, /*X2*/ Some(0.0), /*Y2*/ Some(11.0), /*Z2*/ Some(30.0), /*Hitlag*/ 0.5, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ -2, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_jack_bullet"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_JACK_SHOT, /*Type*/ *ATTACK_REGION_OBJECT);
			} else if GUN_C[ENTRY_ID] == ATTACK_S4 {
				macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 5.0, /*Angle*/ 120, /*KBG*/ 30, /*FKB*/ 0, /*BKB*/ 100, /*Size*/ 4.5, /*X*/ 0.0, /*Y*/ 11.0, /*Z*/ 9.0, /*X2*/ Some(0.0), /*Y2*/ Some(11.0), /*Z2*/ Some(44.0), /*Hitlag*/ 0.5, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ -2, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_jack_bullet"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_JACK_SHOT, /*Type*/ *ATTACK_REGION_OBJECT);
			} else {
				macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 1.0, /*Angle*/ 361, /*KBG*/ 30, /*FKB*/ 0, /*BKB*/ 5, /*Size*/ 2.5, /*X*/ 0.0, /*Y*/ 11.0, /*Z*/ 9.0, /*X2*/ Some(0.0), /*Y2*/ Some(11.0), /*Z2*/ Some(50.0), /*Hitlag*/ 0.5, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ -2, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_jack_bullet"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_JACK_SHOT, /*Type*/ *ATTACK_REGION_OBJECT);
			};
			GUN_C[ENTRY_ID] = NONE;
			if WorkModule::is_flag(boma, *FIGHTER_JACK_INSTANCE_WORK_ID_FLAG_DOYLE) {
				AttackModule::set_add_reaction_frame(fighter.module_accessor, /*ID*/ 0, /*Frames*/ 6.0, /*Unk*/ false);
			};
			macros::FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(fighter, 0, 4, 4);
			macros::FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(fighter, 1, 4, 4);
			macros::FT_SHOOTING_ATTACK_GROUND_CHECK_NEW_arg5(fighter, 2, 4, 4, Hash40::new("jack_gun_hit2"), Hash40::new("se_jack_special_n02"));
		}
		wait(fighter.lua_state_agent, 1.0);
		if macros::is_excute(fighter) {
			AttackModule::clear_all(fighter.module_accessor);
		}
		frame(fighter.lua_state_agent, 26.0);
		if macros::is_excute(fighter) {
			WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_JACK_STATUS_SPECIAL_N_FLAG_CHECK_BUTTON_RAPID);
		}
		frame(fighter.lua_state_agent, 27.0);
		if macros::is_excute(fighter) {
			WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_JACK_STATUS_SPECIAL_N_FLAG_UNABLE_JUMP);
		}
		frame(fighter.lua_state_agent, 31.0);
		if macros::is_excute(fighter) {
			WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_JACK_STATUS_SPECIAL_N_FLAG_CHECK_NEXT_STATUS);
		}
		frame(fighter.lua_state_agent, 34.0);
		if macros::is_excute(fighter) {
			WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_JACK_STATUS_SPECIAL_N_FLAG_UNABLE_SPECIAL_N);
		}
}		
#[acmd_script(
    agent = "jack",
    scripts =  ["game_specialairn1_ex", "game_specialairn1"],
    category = ACMD_GAME,
	low_priority)]
unsafe fn joker_gun_air(fighter: &mut L2CAgentBase) {
    	let lua_state = fighter.lua_state_agent;
    	let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent); 
		let status_kind = smash::app::lua_bind::StatusModule::status_kind(boma);
		let ENTRY_ID = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
		if macros::is_excute(fighter) {
			WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_JACK_STATUS_SPECIAL_N_FLAG_UNABLE_SPECIAL_N);
		}
		frame(fighter.lua_state_agent, 12.0);
		if macros::is_excute(fighter) {
			if GUN_C[ENTRY_ID] == ATTACK_AIR_HI {
				macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 5.0, /*Angle*/ 90, /*KBG*/ 60, /*FKB*/ 0, /*BKB*/ 60, /*Size*/ 14.0, /*X*/ 0.0, /*Y*/ 11.0, /*Z*/ 9.0, /*X2*/ Some(0.0), /*Y2*/ Some(11.0), /*Z2*/ Some(9.0), /*Hitlag*/ 0.5, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ -2, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_jack_bullet"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_JACK_SHOT, /*Type*/ *ATTACK_REGION_OBJECT);
			} else if GUN_C[ENTRY_ID] == ATTACK_AIR_F {
				macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 5.0, /*Angle*/ 50, /*KBG*/ 30, /*FKB*/ 0, /*BKB*/ 5, /*Size*/ 5.5, /*X*/ 0.0, /*Y*/ 11.0, /*Z*/ 9.0, /*X2*/ Some(0.0), /*Y2*/ Some(11.0), /*Z2*/ Some(30.0), /*Hitlag*/ 0.5, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ -2, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ true, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_jack_bullet"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_JACK_SHOT, /*Type*/ *ATTACK_REGION_OBJECT);
			} else {
				macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 1.0, /*Angle*/ 361, /*KBG*/ 30, /*FKB*/ 0, /*BKB*/ 5, /*Size*/ 2.5, /*X*/ 0.0, /*Y*/ 11.0, /*Z*/ 9.0, /*X2*/ Some(0.0), /*Y2*/ Some(11.0), /*Z2*/ Some(50.0), /*Hitlag*/ 0.5, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ -2, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_jack_bullet"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_JACK_SHOT, /*Type*/ *ATTACK_REGION_OBJECT);
			};
			GUN_C[ENTRY_ID] = NONE;
			if WorkModule::is_flag(boma, *FIGHTER_JACK_INSTANCE_WORK_ID_FLAG_DOYLE) {
				AttackModule::set_add_reaction_frame(fighter.module_accessor, /*ID*/ 0, /*Frames*/ 6.0, /*Unk*/ false);
			};
			macros::FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(fighter, 0, 4, 4);
			macros::FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(fighter, 1, 4, 4);
			macros::FT_SHOOTING_ATTACK_GROUND_CHECK_NEW_arg5(fighter, 2, 4, 4, Hash40::new("jack_gun_hit2"), Hash40::new("se_jack_special_n02"));
		}
		wait(fighter.lua_state_agent, 1.0);
		if macros::is_excute(fighter) {
			AttackModule::clear_all(fighter.module_accessor);
		}
		frame(fighter.lua_state_agent, 14.0);
		if macros::is_excute(fighter) {
			WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_JACK_STATUS_SPECIAL_N_FLAG_CHECK_BARRAGE_TRIGGER);
		}
		frame(fighter.lua_state_agent, 17.0);
		if macros::is_excute(fighter) {
			WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_JACK_STATUS_SPECIAL_N_FLAG_CHECK_BARRAGE_BUTTON_ON);
		}
		frame(fighter.lua_state_agent, 21.0);
		if macros::is_excute(fighter) {
			WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_JACK_STATUS_SPECIAL_N_FLAG_ENABLE_CONTROL_ENERGY);
		}
		frame(fighter.lua_state_agent, 26.0);
		if macros::is_excute(fighter) {
			WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_JACK_STATUS_SPECIAL_N_FLAG_CHECK_BUTTON_RAPID);
		}
		frame(fighter.lua_state_agent, 31.0);
		if macros::is_excute(fighter) {
			WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_JACK_STATUS_SPECIAL_N_FLAG_CHECK_NEXT_STATUS);
		}
		frame(fighter.lua_state_agent, 34.0);
		if macros::is_excute(fighter) {
			WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_JACK_STATUS_SPECIAL_N_FLAG_UNABLE_SPECIAL_N);
		}
}	
#[acmd_script(
    agent = "jack",
    scripts =  ["sound_specialn1_ex", "sound_specialairn1_ex"],
    category = ACMD_SOUND,
	low_priority)]
unsafe fn s_joker_gun(fighter: &mut L2CAgentBase) {
		let lua_state = fighter.lua_state_agent;
		let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent); 
		let status_kind = smash::app::lua_bind::StatusModule::status_kind(boma);
		let ENTRY_ID = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
		frame(fighter.lua_state_agent, 12.0);
		if macros::is_excute(fighter) {
			macros::PLAY_SE(fighter, Hash40::new("se_jack_special_n01"));
		}
}		
#[acmd_script(
    agent = "jack",
    scripts =  ["effect_specialn1_ex", "effect_specialairn1_ex"],
    category = ACMD_EFFECT,
	low_priority)]
unsafe fn e_joker_gun(fighter: &mut L2CAgentBase) {
		let lua_state = fighter.lua_state_agent;
		let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent); 
		let status_kind = smash::app::lua_bind::StatusModule::status_kind(boma);
		let ENTRY_ID = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
		frame(fighter.lua_state_agent, 12.0);
		if macros::is_excute(fighter) {
			macros::EFFECT(fighter, Hash40::new_raw(0x16b6b8d02d), Hash40::new("gunl"), 0, 0.8, 1.9, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
		}
}		
#[acmd_script(
    agent = "jack",
    scripts =  ["effect_specialnescapeb_ex", "effect_specialairnescapeb_ex"],
    category = ACMD_EFFECT,
	low_priority)]
unsafe fn e_joker_gun_b(fighter: &mut L2CAgentBase) {
    	let lua_state = fighter.lua_state_agent;
    	let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent); 
		let status_kind = smash::app::lua_bind::StatusModule::status_kind(boma);
		let ENTRY_ID = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
		frame(fighter.lua_state_agent, 22.0);
		if macros::is_excute(fighter) {
			macros::EFFECT(fighter, Hash40::new_raw(0x16b6b8d02d), Hash40::new("gunl"), 0, 0.8, 1.9, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
		}
}	
#[acmd_script(
    agent = "jack",
    scripts =  ["sound_specialnescapeb", "sound_specialairnescapeb", "sound_specialnescapeb_ex", "sound_specialairnescapeb_ex"],
    category = ACMD_SOUND,
	low_priority)]
unsafe fn s_joker_gun_b(fighter: &mut L2CAgentBase) {
    	let lua_state = fighter.lua_state_agent;
   		let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent); 
		let status_kind = smash::app::lua_bind::StatusModule::status_kind(boma);
		let ENTRY_ID = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
		frame(fighter.lua_state_agent, 5.0);
		if macros::is_excute(fighter) {
			macros::PLAY_SE(fighter, Hash40::new("se_jack_escape"));
		}
		frame(fighter.lua_state_agent, 22.0);
		if macros::is_excute(fighter) {
			macros::PLAY_SE(fighter, Hash40::new("se_jack_special_n01"));
		}
}	
		
#[acmd_script(
    agent = "jack",
    scripts =  ["game_specialnescapeb_ex", "game_specialairnescapeb_ex","game_specialnescapeb", "game_specialairnescapeb"],
    category = ACMD_GAME,
	low_priority)]
unsafe fn joker_gun_b(fighter: &mut L2CAgentBase) {
		let lua_state = fighter.lua_state_agent;
		let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent); 
		let status_kind = smash::app::lua_bind::StatusModule::status_kind(boma);
		let ENTRY_ID = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
		frame(fighter.lua_state_agent, 4.0);
		if macros::is_excute(fighter) {
			JostleModule::set_status(fighter.module_accessor, false);
		}
		frame(fighter.lua_state_agent, 17.0);
		if macros::is_excute(fighter) {
			JostleModule::set_status(fighter.module_accessor, true);
			WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_JACK_STATUS_SPECIAL_N_FLAG_UNABLE_SPECIAL_N);
		}
		frame(fighter.lua_state_agent, 22.0);
		if macros::is_excute(fighter) {
			if GUN_C[ENTRY_ID] == ATTACK_S3 {
				macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 5.0, /*Angle*/ 68, /*KBG*/ 70, /*FKB*/ 0, /*BKB*/ 80, /*Size*/ 4.5, /*X*/ 0.0, /*Y*/ 11.0, /*Z*/ 9.0, /*X2*/ Some(0.0), /*Y2*/ Some(11.0), /*Z2*/ Some(30.0), /*Hitlag*/ 0.5, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ -2, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_jack_bullet"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_JACK_SHOT, /*Type*/ *ATTACK_REGION_OBJECT);
			} else {
				macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 1.0, /*Angle*/ 50, /*KBG*/ 30, /*FKB*/ 0, /*BKB*/ 5, /*Size*/ 2.5, /*X*/ 0.0, /*Y*/ 11.0, /*Z*/ 9.0, /*X2*/ Some(0.0), /*Y2*/ Some(11.0), /*Z2*/ Some(50.0), /*Hitlag*/ 0.5, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ -2, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ true, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_jack_bullet"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_JACK_SHOT, /*Type*/ *ATTACK_REGION_OBJECT);
			};
			if WorkModule::is_flag(boma, *FIGHTER_JACK_INSTANCE_WORK_ID_FLAG_DOYLE) {
				AttackModule::set_add_reaction_frame(fighter.module_accessor, /*ID*/ 0, /*Frames*/ 6.0, /*Unk*/ false);
			};
			GUN_C[ENTRY_ID] = NONE;
			macros::FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(fighter, 0, 4, 4);
			macros::FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(fighter, 1, 4, 4);
			macros::FT_SHOOTING_ATTACK_GROUND_CHECK_NEW_arg5(fighter, 2, 4, 4, Hash40::new("jack_gun_hit2"), Hash40::new("se_jack_special_n02"));
		}
		wait(fighter.lua_state_agent, 1.0);
		if macros::is_excute(fighter) {
			AttackModule::clear_all(fighter.module_accessor);
		}
		frame(fighter.lua_state_agent, 39.0);
		if macros::is_excute(fighter) {
			WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_JACK_STATUS_SPECIAL_N_FLAG_CHECK_NEXT_STATUS);
			WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_JACK_STATUS_SPECIAL_N_FLAG_CHECK_BARRAGE_BUTTON_ON);
		}
		frame(fighter.lua_state_agent, 42.0);
		if macros::is_excute(fighter) {
			WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_JACK_STATUS_SPECIAL_N_FLAG_UNABLE_SPECIAL_N);
		}
}			
#[acmd_script(
    agent = "jack",
    scripts =  ["game_specialairnescapeb_ex", "game_specialairnescapeb"],
    category = ACMD_GAME,
	low_priority)]
unsafe fn joker_gun_b_air(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent); 
	let status_kind = smash::app::lua_bind::StatusModule::status_kind(boma);
	let ENTRY_ID = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
		frame(fighter.lua_state_agent, 4.0);
		if macros::is_excute(fighter) {
			JostleModule::set_status(fighter.module_accessor, false);
		}
		frame(fighter.lua_state_agent, 17.0);
		if macros::is_excute(fighter) {
			JostleModule::set_status(fighter.module_accessor, true);
			WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_JACK_STATUS_SPECIAL_N_FLAG_UNABLE_SPECIAL_N);
		}
		frame(fighter.lua_state_agent, 22.0);
		if macros::is_excute(fighter) {
			if GUN_C[ENTRY_ID] == ATTACK_AIR_B {
				StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_FALL, true);
			} else {
				macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 1.0, /*Angle*/ 50, /*KBG*/ 30, /*FKB*/ 0, /*BKB*/ 5, /*Size*/ 2.5, /*X*/ 0.0, /*Y*/ 11.0, /*Z*/ 9.0, /*X2*/ Some(0.0), /*Y2*/ Some(11.0), /*Z2*/ Some(50.0), /*Hitlag*/ 0.5, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ -2, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ true, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_jack_bullet"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_JACK_SHOT, /*Type*/ *ATTACK_REGION_OBJECT);
			};
			if WorkModule::is_flag(boma, *FIGHTER_JACK_INSTANCE_WORK_ID_FLAG_DOYLE) {
				AttackModule::set_add_reaction_frame(fighter.module_accessor, /*ID*/ 0, /*Frames*/ 6.0, /*Unk*/ false);
			};
			GUN_C[ENTRY_ID] = NONE;
			macros::FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(fighter, 0, 4, 4);
			macros::FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(fighter, 1, 4, 4);
			macros::FT_SHOOTING_ATTACK_GROUND_CHECK_NEW_arg5(fighter, 2, 4, 4, Hash40::new("jack_gun_hit2"), Hash40::new("se_jack_special_n02"));
		}
		wait(fighter.lua_state_agent, 1.0);
		if macros::is_excute(fighter) {
			AttackModule::clear_all(fighter.module_accessor);
		}
		frame(fighter.lua_state_agent, 32.0);
		if macros::is_excute(fighter) {
			WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_JACK_STATUS_SPECIAL_N_FLAG_ENABLE_CONTROL_ENERGY);
		}
		frame(fighter.lua_state_agent, 36.0);
		if macros::is_excute(fighter) {
			WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_JACK_STATUS_SPECIAL_N_FLAG_CHECK_BARRAGE_TRIGGER);
		}
		frame(fighter.lua_state_agent, 37.0);
		frame(fighter.lua_state_agent, 39.0);
		if macros::is_excute(fighter) {
			WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_JACK_STATUS_SPECIAL_N_FLAG_CHECK_NEXT_STATUS);
			WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_JACK_STATUS_SPECIAL_N_FLAG_CHECK_BARRAGE_BUTTON_ON);
		}
		frame(fighter.lua_state_agent, 42.0);
		if macros::is_excute(fighter) {
			WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_JACK_STATUS_SPECIAL_N_FLAG_UNABLE_SPECIAL_N);
		}
}		
#[acmd_script(
    agent = "jack",
    scripts =  ["game_specials2"],
    category = ACMD_GAME,
	low_priority)]
unsafe fn joker_eiagon(fighter: &mut L2CAgentBase) {
    	let lua_state = fighter.lua_state_agent;
		frame(fighter.lua_state_agent, 1.0);
		macros::FT_MOTION_RATE(fighter, /*FSM*/ 1.4375);
		frame(fighter.lua_state_agent, 18.0);
		macros::FT_MOTION_RATE(fighter, /*FSM*/ 1);
}	
			
#[acmd_script(
    agent = "jack",
    scripts =  ["game_specialairs2"],
    category = ACMD_GAME,
	low_priority)]
unsafe fn joker_eiagon_air(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
		frame(fighter.lua_state_agent, 1.0);
		macros::FT_MOTION_RATE(fighter, /*FSM*/ 1.4375);
		frame(fighter.lua_state_agent, 18.0);
		macros::FT_MOTION_RATE(fighter, /*FSM*/ 1);
		if macros::is_excute(fighter) {
			WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_JACK_STATUS_SPECIAL_S_FLAG_ENABLE_CONTROL_ENERGY);
		}
		frame(fighter.lua_state_agent, 32.0);
		if macros::is_excute(fighter) {
			WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_JACK_STATUS_SPECIAL_S_FLAG_SET_FALL_NORMAL);
		}
}
	
#[acmd_script(
    agent = "jack",
    scripts =  ["game_specials1", "game_specialairs1"],
    category = ACMD_GAME,
	low_priority)]
unsafe fn joker_sideb(fighter: &mut L2CAgentBase) {
        let lua_state = fighter.lua_state_agent;
		let ENTRY_ID = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
		if macros::is_excute(fighter) {
			BATON_TYPE[ENTRY_ID] += 1;
			if BATON_TYPE[ENTRY_ID] > BATON_MAX {
				BATON_TYPE[ENTRY_ID] = 0;
			}
		};
		frame(fighter.lua_state_agent, 5.0);
		if macros::is_excute(fighter) {
			if BATON_TYPE[ENTRY_ID] == 0 {
				ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_JACK_GENERATE_ARTICLE_MONA, false, 0);
				ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_JACK_GENERATE_ARTICLE_MONA,smash::phx::Hash40::new("special_s1"),false,0.0);
			} else if BATON_TYPE[ENTRY_ID] == 1 {
				ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_JACK_GENERATE_ARTICLE_MONA, false, 0);
				ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_JACK_GENERATE_ARTICLE_MONA,smash::phx::Hash40::new("special_s2"),false,0.0);
			} else {
				ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_JACK_GENERATE_ARTICLE_MONA, false, 0);
				ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_JACK_GENERATE_ARTICLE_MONA,smash::phx::Hash40::new("special_s3"),false,0.0);
			};
		}
		//ACMD for each one of the baton passes
		if BATON_TYPE[ENTRY_ID] == 0 { //Skull (Ryuji)
			frame(fighter.lua_state_agent, 15.0);
			if macros::is_excute(fighter) {
				macros::ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 6.0, /*Angle*/ 270, /*KBG*/ 90, /*FKB*/ 0, /*BKB*/ 40, /*Size*/ 3.6, /*X*/ 0.0, /*Y*/ 55.0, /*Z*/ 28.0, /*X2*/ Some(0.0), /*Y2*/ Some(40.0), /*Z2*/ Some(28.0), /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 3, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ true, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_elec"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_ELEC, /*Type*/ *ATTACK_REGION_ENERGY);
				macros::ATTACK(fighter, /*ID*/ 2, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 6.0, /*Angle*/ 361, /*KBG*/ 90, /*FKB*/ 0, /*BKB*/ 40, /*Size*/ 3.6, /*X*/ 0.0, /*Y*/ 40.0, /*Z*/ 28.0, /*X2*/ Some(0.0), /*Y2*/ Some(4.0), /*Z2*/ Some(28.0), /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 3, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ true, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_elec"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_ELEC, /*Type*/ *ATTACK_REGION_ENERGY);
				macros::ATTACK(fighter, /*ID*/ 	0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 10.0, /*Angle*/ 361, /*KBG*/ 90, /*FKB*/ 0, /*BKB*/ 40, /*Size*/ 6.0, /*X*/ 0.0, /*Y*/ 5.0, /*Z*/ 28.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.5, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 3, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ true, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_fire"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_ELEC, /*Type*/ *ATTACK_REGION_ENERGY);
			};
			wait(fighter.lua_state_agent, 2.0);
			if macros::is_excute(fighter) {
				AttackModule::clear_all(fighter.module_accessor);
			};
		} else if BATON_TYPE[ENTRY_ID] == 1 { // Panther (Ann)
			frame(fighter.lua_state_agent, 20.0);
			if macros::is_excute(fighter) {
				macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 14.0, /*Angle*/ 361, /*KBG*/ 98, /*FKB*/ 0, /*BKB*/ 40, /*Size*/ 15.0, /*X*/ 0.0, /*Y*/ 8.0, /*Z*/ 30.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_fire"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_FIRE, /*Type*/ *ATTACK_REGION_BOMB);
			};
			wait(fighter.lua_state_agent, 2.0);
			if macros::is_excute(fighter) {
				AttackModule::clear_all(fighter.module_accessor);
			};
		} else { //Mona (Morgana)
			frame(fighter.lua_state_agent, 15.0);
			if macros::is_excute(fighter) {
				macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 1.0, /*Angle*/ 361, /*KBG*/ 10, /*FKB*/ 0, /*BKB*/ 11, /*Size*/ 10.0, /*X*/ 0.0, /*Y*/ 8.0, /*Z*/ 30.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.4, /*SDI*/ 0.6, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 3, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_PUNCH);
			};
			frame(fighter.lua_state_agent, 30.0);
			if macros::is_excute(fighter) {
				AttackModule::clear_all(fighter.module_accessor);
				macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 5.0, /*Angle*/ 110, /*KBG*/ 60, /*FKB*/ 0, /*BKB*/ 67, /*Size*/ 10.0, /*X*/ 0.0, /*Y*/ 8.0, /*Z*/ 30.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.4, /*SDI*/ 0.6, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 3, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_PUNCH);
			};
			wait(fighter.lua_state_agent, 2.0);
			if macros::is_excute(fighter) {
				AttackModule::clear_all(fighter.module_accessor);
			};
		}
		frame(fighter.lua_state_agent, 48.0);
		if macros::is_excute(fighter) {
			ArticleModule::remove_exist(fighter.module_accessor, *FIGHTER_JACK_GENERATE_ARTICLE_MONA,smash::app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
		}
}	
#[acmd_script(
    agent = "jack",
    scripts =  ["effect_specials1", "effect_specialairs1"],
    category = ACMD_EFFECT,
	low_priority)]
unsafe fn joker_sideb_eff(fighter: &mut L2CAgentBase) {
        let lua_state = fighter.lua_state_agent;
		let ENTRY_ID = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
		frame(fighter.lua_state_agent, 2.0);
		if BATON_TYPE[ENTRY_ID] == 0 { //Skull (Ryuji)
			frame(fighter.lua_state_agent, 14.0);
			if macros::is_excute(fighter) {
				macros::EFFECT(fighter, Hash40::new("sys_thunder_flash"), Hash40::new("top"), 0, 0, 28, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
			};
			println!("");
		} else if BATON_TYPE[ENTRY_ID] == 1 { // Panther (Ann)
			frame(fighter.lua_state_agent, 19.0);
			if macros::is_excute(fighter) {
				macros::EFFECT(fighter, Hash40::new("sys_bomb_a"), Hash40::new("top"), 0.0, 8.0, 30.0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, true);
			};
		} else { //Mona (Morgana)
			frame(fighter.lua_state_agent, 14.0);
			for _ in 0..5 {
				if macros::is_excute(fighter) {
					macros::EFFECT_FOLLOW(fighter, Hash40::new("sys_spin_wind"), Hash40::new("top"), 0.0, 12.0, 30.0, 0, 0, 0, 1.2*0.7, true);
					macros::LAST_EFFECT_SET_COLOR(fighter, 0.69, 2.45, 0.66);
					macros::EFFECT_FOLLOW(fighter, Hash40::new("sys_spin_wind"), Hash40::new("top"), 0.0, 7.5, 30.0, 0, 10, 0, 1.0*0.7, true);
					macros::LAST_EFFECT_SET_COLOR(fighter, 0.69, 2.45, 0.66);
					macros::EFFECT_FOLLOW(fighter, Hash40::new("sys_spin_wind"), Hash40::new("top"), 0.0, 5.5, 30.0, 0, -18, 0, 0.9*0.7, true);
					macros::LAST_EFFECT_SET_COLOR(fighter, 0.69, 2.45, 0.66);
				};	
				wait(fighter.lua_state_agent, 1.0);
				if macros::is_excute(fighter) {
					macros::EFFECT_FOLLOW(fighter, Hash40::new("sys_spin_wind"), Hash40::new("top"), 0.0, 9.0, 30.0, 0, 25, 0, 1.1*0.7, true);
					macros::LAST_EFFECT_SET_COLOR(fighter, 0.69, 2.45, 0.66);
					macros::EFFECT_FOLLOW(fighter, Hash40::new("sys_spin_wind"), Hash40::new("top"), 0.0, 10.2, 30.0, 0, -16, 0, 0.8*0.7, true);
					macros::LAST_EFFECT_SET_COLOR(fighter, 0.69, 2.45, 0.66);
					macros::EFFECT_FOLLOW(fighter, Hash40::new("sys_spin_wind"), Hash40::new("top"), 0.0, 5.5, 30.0, 0, 3, 0, 1.3*0.7, true);
					macros::LAST_EFFECT_SET_COLOR(fighter, 0.69, 2.45, 0.66);
				};	
				wait(fighter.lua_state_agent, 1.0);
				if macros::is_excute(fighter) {
					macros::EFFECT_FOLLOW(fighter, Hash40::new("sys_spin_wind"), Hash40::new("top"), 0.0, 7.0, 30.0, 0, 11, 0, 1.0*0.7, true);
					macros::LAST_EFFECT_SET_COLOR(fighter, 0.69, 2.45, 0.66);
					macros::EFFECT_FOLLOW(fighter, Hash40::new("sys_spin_wind"), Hash40::new("top"), 0.0, 10.5, 30.0, 0, 13, 0, 1.325*0.7, true);
					macros::LAST_EFFECT_SET_COLOR(fighter, 0.69, 2.45, 0.66);
					macros::EFFECT_FOLLOW(fighter, Hash40::new("sys_spin_wind"), Hash40::new("top"), 0.0, 3.5, 30.0, 0, -5, 0, 1.15*0.7, true);
					macros::LAST_EFFECT_SET_COLOR(fighter, 0.69, 2.45, 0.66);
				};	
				wait(fighter.lua_state_agent, 1.0);
			};
			frame(fighter.lua_state_agent, 30.0);
			if macros::is_excute(fighter) {
				macros::EFFECT_FOLLOW(fighter, Hash40::new("sys_spin_wind"), Hash40::new("top"), 0.0, 12.0, 30.0, 0, 0, 0, 1.2*1.5*0.7, true);
				macros::LAST_EFFECT_SET_COLOR(fighter, 0.69, 2.45, 0.66);
				macros::EFFECT_FOLLOW(fighter, Hash40::new("sys_spin_wind"), Hash40::new("top"), 0.0, 5.5, 30.0, 0, 10, 0, 1.0*1.5*0.7, true);
				macros::LAST_EFFECT_SET_COLOR(fighter, 0.69, 2.45, 0.66);
				macros::EFFECT_FOLLOW(fighter, Hash40::new("sys_spin_wind"), Hash40::new("top"), 0.0, 7.5, 30.0, 0, -18, 0, 0.9*1.5*0.7, true);
				macros::LAST_EFFECT_SET_COLOR(fighter, 0.69, 2.45, 0.66);
			};
		}
		frame(fighter.lua_state_agent, 47.0);
		if macros::is_excute(fighter) {
			if BATON_TYPE[ENTRY_ID] != 2 { //Not Mona
				macros::EFFECT(fighter, Hash40::new("jack_mona_smoke"), Hash40::new("top"), 0.0, 6.0, 15.0, 0, 0, 0, 3.0, 0, 0, 0, 0, 0, 0, true);
			} else {
				macros::EFFECT(fighter, Hash40::new("jack_mona_smoke"), Hash40::new("top"), 0.0, 2.0, 15.0, 0, 0, 0, 1.0, 0, 0, 0, 0, 0, 0, true);
			};
		}
}	
#[acmd_script(
    agent = "jack",
    scripts =  ["sound_specials1", "sound_specialairs1"],
    category = ACMD_SOUND,
	low_priority)]
unsafe fn joker_sideb_snd(fighter: &mut L2CAgentBase) {
        let lua_state = fighter.lua_state_agent;
		let ENTRY_ID = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
		frame(fighter.lua_state_agent, 3.0);
		let rand_val = smash::app::sv_math::rand(hash40("fighter"), 3);
		if BATON_TYPE[ENTRY_ID] == 0 { //Skull (Ryuji)
			frame(fighter.lua_state_agent, 6.0);
			if macros::is_excute(fighter) {
				if rand_val == 0 {
					macros::PLAY_SE(fighter, Hash40::new("vc_jack_special_s02")); // vc 2
				} else {
					macros::PLAY_SE(fighter, Hash40::new("vc_jack_special_n03")); // vc 1
				};
			};
			frame(fighter.lua_state_agent, 14.0);
			if macros::is_excute(fighter) {
				macros::PLAY_SE(fighter, Hash40::new("se_common_smashswing_02"));
				macros::PLAY_SE(fighter, Hash40::new("se_common_down_m_02"));
				macros::PLAY_SE(fighter, Hash40::new("se_common_electric_hit_s"));
			};
			println!("");
		} else if BATON_TYPE[ENTRY_ID] == 1 { // Panther (Ann)
			frame(fighter.lua_state_agent, 8.0);
			if macros::is_excute(fighter) {
				if rand_val == 0 {
					macros::PLAY_SE(fighter, Hash40::new("vc_jack_special_n02")); // vc 1
				} else {
					macros::PLAY_SE(fighter, Hash40::new("vc_jack_special_s01")); // vc 2
				};
			};
			frame(fighter.lua_state_agent, 19.0);
			if macros::is_excute(fighter) {
				macros::PLAY_SE(fighter, Hash40::new("se_common_bomb_ll"));
			};
		} else { //Mona (Morgana)
			frame(fighter.lua_state_agent, 6.0);
			if macros::is_excute(fighter) {
				if rand_val == 0 {
					macros::PLAY_SE(fighter, Hash40::new("vc_jack_special_n01")); // vc 1
				} else {
					macros::PLAY_SE(fighter, Hash40::new("vc_jack_special_n04")); // vc 2
				};
			};
			frame(fighter.lua_state_agent, 14.0);
			for _ in 0..5 {
				if macros::is_excute(fighter) {
					macros::PLAY_SE(fighter, Hash40::new("se_common_swing_01"));
				}
				wait(fighter.lua_state_agent, 1.0);
				if macros::is_excute(fighter) {
					macros::PLAY_SE(fighter, Hash40::new("se_common_swing_03"));
				}
				wait(fighter.lua_state_agent, 1.0);
				if macros::is_excute(fighter) {
					macros::PLAY_SE(fighter, Hash40::new("se_common_swing_02"));
				}
				wait(fighter.lua_state_agent, 1.0);
			};
			frame(fighter.lua_state_agent, 30.0);
			if macros::is_excute(fighter) {
				macros::PLAY_SE(fighter, Hash40::new("se_common_swing_08"));
			}
		}
}	