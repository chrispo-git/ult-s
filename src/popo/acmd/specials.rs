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
    Agent::new("popo")
    .acmd("game_specialn", ics_shot, Priority::Low)    
    .acmd("game_specialairn", ics_shot, Priority::Low)    
    .acmd("game_specialn_nana", ics_shot2, Priority::Low)    
    .acmd("game_specialairn_nana", ics_shot2, Priority::Low)    
    .install();
}

unsafe extern "C" fn ics_shot(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
		macros::FT_MOTION_RATE(fighter, /*FSM*/ 0.5);
		frame(fighter.lua_state_agent, 6.0);
		if macros::is_excute(fighter) {
			WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_POPO_STATUS_SPECIAL_N_FLAG_GENERATE_ARTICLE);
		}
		frame(fighter.lua_state_agent, 18.0);
		macros::FT_MOTION_RATE(fighter, /*FSM*/ 1.0);
		if macros::is_excute(fighter) {
			ArticleModule::shoot_exist(fighter.module_accessor, *FIGHTER_POPO_GENERATE_ARTICLE_ICESHOT, smash::app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL), false);
		}
}
unsafe extern "C" fn ics_shot2(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
		macros::FT_MOTION_RATE(fighter, /*FSM*/ 0.5);
		frame(fighter.lua_state_agent, 6.0);
		if macros::is_excute(fighter) {
			WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_POPO_STATUS_SPECIAL_N_FLAG_GENERATE_ARTICLE);
		}
		frame(fighter.lua_state_agent, 18.0);
		macros::FT_MOTION_RATE(fighter, /*FSM*/ 1.0);
		if macros::is_excute(fighter) {
			ArticleModule::shoot_exist(fighter.module_accessor, *FIGHTER_POPO_GENERATE_ARTICLE_ICESHOT, smash::app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL), false);
		}
		frame(fighter.lua_state_agent, 50.0);
		if macros::is_excute(fighter) {
			WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_POPO_STATUS_SPECIAL_N_FLAG_ENABLE_COUPLE);
		}
}