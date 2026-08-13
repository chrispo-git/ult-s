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
use crate::master::*;
use super::*;

pub fn install() {
    Agent::new("master")
    .set_costume(get_marked_costumes("master","master"))
    .acmd("game_speciallw", byleth_downb, Priority::Low)    
    .acmd("game_specialairlw", byleth_downb, Priority::Low)    
    .install();


	Agent::new("master_axe")
    .set_costume(get_marked_costumes("master","master"))
    .acmd("game_speciallw", axe_downb, Priority::Low)    
    .acmd("game_specialairlw", axe_downb, Priority::Low)    
    .install();
}
				
unsafe extern "C" fn axe_downb(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
	macros::FT_MOTION_RATE(fighter, /*FSM*/ 0.04761904);
	if macros::is_excute(fighter) {
		WorkModule::set_int(fighter.module_accessor, 6, *WEAPON_MASTER_AXE_INSTANCE_WORK_ID_INT_CRITICAL_ATTACK_ID);
	}
	frame(fighter.lua_state_agent, 42.0);
	macros::FT_MOTION_RATE(fighter, /*FSM*/ 1);
	frame(fighter.lua_state_agent, 61.0);
	if macros::is_excute(fighter) {
		macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("haver"),  /*Damage*/ 16.8, /*Angle*/ 361, /*KBG*/ 80, /*FKB*/ 0, /*BKB*/ 50, /*Size*/ 6.7, /*X*/ 0.0, /*Y*/ 14.0, /*Z*/ 1.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.45, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_THRU, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_MASTER_AXE, /*Type*/ *ATTACK_REGION_OBJECT);
	}
	frame(fighter.lua_state_agent, 67.0);
	if macros::is_excute(fighter) {
		AttackModule::clear_all(fighter.module_accessor);
	}
}			
unsafe extern "C" fn byleth_downb(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	macros::FT_MOTION_RATE(fighter, /*FSM*/ 0.04761904);
	if macros::is_excute(fighter) {
		WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_MASTER_STATUS_SPECIAL_LW_FLAG_INHERIT_LANDING_1);
	}
	frame(fighter.lua_state_agent, 42.0);
	macros::FT_MOTION_RATE(fighter, /*FSM*/ 1);
	frame(fighter.lua_state_agent, 51.0);
	if macros::is_excute(fighter) {
		WorkModule::off_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_MASTER_STATUS_SPECIAL_LW_FLAG_INHERIT_LANDING_1);
	}
	frame(fighter.lua_state_agent, 60.0);
	if macros::is_excute(fighter) {
		WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_MASTER_STATUS_SPECIAL_LW_FLAG_FORBID_LANDING);
	}
	frame(fighter.lua_state_agent, 64.0);
	if macros::is_excute(fighter) {
		ArticleModule::set_flag(fighter.module_accessor, *FIGHTER_MASTER_GENERATE_ARTICLE_AXE, true, *WEAPON_PIERCE_INSTANCE_WORK_ID_FLAG_PIERCE_GROUND);
	}
	frame(fighter.lua_state_agent, 65.0);
	if macros::is_excute(fighter) {
		ArticleModule::set_flag(fighter.module_accessor, *FIGHTER_MASTER_GENERATE_ARTICLE_AXE, false, *WEAPON_PIERCE_INSTANCE_WORK_ID_FLAG_PIERCE_GROUND);
		WorkModule::off_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_MASTER_STATUS_SPECIAL_LW_FLAG_FORBID_LANDING);
		WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_MASTER_STATUS_SPECIAL_LW_FLAG_INHERIT_LANDING_2);
	}
	frame(fighter.lua_state_agent, 96.0);
	if macros::is_excute(fighter) {
		WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_MASTER_STATUS_SPECIAL_LW_FLAG_CONTROL_ENERGY);
		WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_MASTER_STATUS_SPECIAL_LW_FLAG_REVERT_FALL_SPEED);
		CancelModule::enable_cancel(fighter.module_accessor);
	}
	frame(fighter.lua_state_agent, 117.0);
	if macros::is_excute(fighter) {
		WorkModule::off_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_MASTER_STATUS_SPECIAL_LW_FLAG_INHERIT_LANDING_2);
	}
	frame(fighter.lua_state_agent, 118.0);
	if macros::is_excute(fighter) {
		ArticleModule::remove_exist(fighter.module_accessor, *FIGHTER_MASTER_GENERATE_ARTICLE_AXE,smash::app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
	}
}