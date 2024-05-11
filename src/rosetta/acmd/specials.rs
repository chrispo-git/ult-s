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

pub fn install() {
    Agent::new("rosetta")
    .acmd("game_specialhistart", rosa_upb_start, Priority::Low)    
    .acmd("game_specialairhistart", rosa_upb_start, Priority::Low)    
    .acmd("game_specialhi", rosa_upb, Priority::Low)    
    .acmd("game_specialairhi", rosa_upb, Priority::Low)    
    .acmd("game_explode", rosa_boom, Priority::Low)    
    .install();

	Agent::new("rosetta_tico")
    .acmd("game_explode", luma_boom, Priority::Low)    
    .acmd("effect_explode", luma_boom_eff, Priority::Low)    
    .acmd("sound_explode", luma_boom_snd, Priority::Low)    
    .install();
}

unsafe extern "C" fn rosa_upb_start(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
		frame(fighter.lua_state_agent, 6.0);
		if macros::is_excute(fighter) {
			macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("hip"), /*Damage*/ 5.0, /*Angle*/ 367, /*KBG*/ 100, /*FKB*/ 90, /*BKB*/ 0, /*Size*/ 13.5, /*X*/ 0.0, /*Y*/ 11.0, /*Z*/ 10.0, /*X2*/ Some(0.0), /*Y2*/ Some(11.0), /*Z2*/ Some(15.0), /*Hitlag*/ 0.75, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_magic"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_MAGIC, /*Type*/ *ATTACK_REGION_MAGIC);
		}
}	
unsafe extern "C" fn rosa_upb(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
		if macros::is_excute(fighter) {
			JostleModule::set_status(fighter.module_accessor, false);
			macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("hip"), /*Damage*/ 2.5, /*Angle*/ 367, /*KBG*/ 100, /*FKB*/ 90, /*BKB*/ 0, /*Size*/ 13.5, /*X*/ 0.0, /*Y*/ 11.0, /*Z*/ 10.0, /*X2*/ Some(0.0), /*Y2*/ Some(11.0), /*Z2*/ Some(15.0), /*Hitlag*/ 0.75, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 4, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_magic"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_MAGIC, /*Type*/ *ATTACK_REGION_MAGIC);
		}
		frame(fighter.lua_state_agent, 12.0);
		if macros::is_excute(fighter) {
			notify_event_msc_cmd!(fighter, 0x2127e37c07u64, *GROUND_CLIFF_CHECK_KIND_ALWAYS);
		}
		frame(fighter.lua_state_agent, 17.0);
		if macros::is_excute(fighter) {	
			AttackModule::clear_all(fighter.module_accessor);
			macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("hip"), /*Damage*/ 8.5, /*Angle*/ 363, /*KBG*/ 70, /*FKB*/ 0, /*BKB*/ 74, /*Size*/ 17.5, /*X*/ 0.0, /*Y*/ 11.0, /*Z*/ 10.0, /*X2*/ Some(0.0), /*Y2*/ Some(11.0), /*Z2*/ Some(15.0), /*Hitlag*/ 1.2, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_magic"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_MAGIC, /*Type*/ *ATTACK_REGION_MAGIC);
		}
		frame(fighter.lua_state_agent, 19.0);
		if macros::is_excute(fighter) {
			AttackModule::clear_all(fighter.module_accessor);
		}
}		

unsafe extern "C" fn rosa_boom(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
	let ENTRY_ID = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
	if macros::is_excute(fighter) && !IS_TICO_DEAD[ENTRY_ID]{
		ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_ROSETTA_GENERATE_ARTICLE_TICO,smash::phx::Hash40::new("explode"),false,0.0);
	}
}
unsafe extern "C" fn luma_boom(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
	let status_kind = smash::app::lua_bind::StatusModule::status_kind(fighter.module_accessor);
	let otarget_id = WorkModule::get_int(fighter.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER) as u32;
	let boma = smash::app::sv_battle_object::module_accessor(otarget_id);
	let ENTRY_ID = WorkModule::get_int(&mut *boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
	if ![*WEAPON_ROSETTA_TICO_STATUS_KIND_DEAD, *WEAPON_ROSETTA_TICO_STATUS_KIND_NONE].contains(&status_kind) && !IS_TICO_DEAD[ENTRY_ID] {
		frame(fighter.lua_state_agent, 21.0);
		if macros::is_excute(fighter) {
			macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("rot"), /*Damage*/ 13.5, /*Angle*/ 361, /*KBG*/ 115, /*FKB*/ 0, /*BKB*/ 60, /*Size*/ 22.0, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.5, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_fire"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_BOMB, /*Type*/ *ATTACK_REGION_MAGIC);
		}
		frame(fighter.lua_state_agent, 24.0);
		if macros::is_excute(fighter) {
			AttackModule::clear_all(fighter.module_accessor);
			IS_TICO_DEAD[ENTRY_ID] = true;
			StatusModule::change_status_request_from_script(fighter.module_accessor, *WEAPON_ROSETTA_TICO_STATUS_KIND_DEAD, false);
		}
	}
}
unsafe extern "C" fn luma_boom_eff(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
	let status_kind = smash::app::lua_bind::StatusModule::status_kind(fighter.module_accessor);
	let otarget_id = WorkModule::get_int(fighter.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER) as u32;
	let boma = smash::app::sv_battle_object::module_accessor(otarget_id);
	let ENTRY_ID = WorkModule::get_int(&mut *boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
	if ![*WEAPON_ROSETTA_TICO_STATUS_KIND_DEAD, *WEAPON_ROSETTA_TICO_STATUS_KIND_NONE].contains(&status_kind) && !IS_TICO_DEAD[ENTRY_ID] {
		frame(fighter.lua_state_agent, 18.0);
		if macros::is_excute(fighter) {
			macros::EFFECT(fighter, Hash40::new("rosetta_tico_dead"), Hash40::new("hip"), 1, 0, 0, 0, 0, -90, 1.6, 0, 0, 0, 0, 0, 0, true);
		}
		frame(fighter.lua_state_agent, 20.0);
		if macros::is_excute(fighter) {
			macros::QUAKE(fighter, *CAMERA_QUAKE_KIND_M);
			WorkModule::on_flag(fighter.module_accessor, *WEAPON_ROSETTA_TICO_INSTANCE_WORK_ID_FLAG_CAMERA_OFF);
		}
	}
}
unsafe extern "C" fn luma_boom_snd(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
	let status_kind = smash::app::lua_bind::StatusModule::status_kind(fighter.module_accessor);
	let otarget_id = WorkModule::get_int(fighter.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER) as u32;
	let boma = smash::app::sv_battle_object::module_accessor(otarget_id);
	let ENTRY_ID = WorkModule::get_int(&mut *boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
	if ![*WEAPON_ROSETTA_TICO_STATUS_KIND_DEAD, *WEAPON_ROSETTA_TICO_STATUS_KIND_NONE].contains(&status_kind) && !IS_TICO_DEAD[ENTRY_ID] {
		frame(fighter.lua_state_agent, 20.0);
		if macros::is_excute(fighter) {
			macros::PLAY_SE(fighter, Hash40::new("se_common_bomb_ll"));
		}
	}
}