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
		m2_dash,
		m2_turn_dash
    );
}
#[acmd_script(
    agent = "mewtwo",
    script =  "game_dash",
    category = ACMD_GAME,
	low_priority)]
unsafe fn m2_dash(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
		frame(fighter.lua_state_agent, 1.0);
		if macros::is_excute(fighter) {
			macros::HIT_NODE(fighter, Hash40::new("s_tail2"), *HIT_STATUS_XLU);
			macros::HIT_NODE(fighter, Hash40::new("s_tail3"), *HIT_STATUS_XLU);
			macros::HIT_NODE(fighter, Hash40::new("s_tail5"), *HIT_STATUS_XLU);
			macros::HIT_NODE(fighter, Hash40::new("s_tail4"), *HIT_STATUS_XLU);
			macros::HIT_NODE(fighter, Hash40::new("s_tail7"), *HIT_STATUS_XLU);
		}
		frame(fighter.lua_state_agent, 11.0);
		if macros::is_excute(fighter) {
			macros::HIT_NODE(fighter, Hash40::new("s_tail2"), *HIT_STATUS_NORMAL);
			macros::HIT_NODE(fighter, Hash40::new("s_tail5"), *HIT_STATUS_NORMAL);
			macros::HIT_NODE(fighter, Hash40::new("s_tail3"), *HIT_STATUS_NORMAL);
			macros::HIT_NODE(fighter, Hash40::new("s_tail4"), *HIT_STATUS_NORMAL);
			macros::HIT_NODE(fighter, Hash40::new("s_tail7"), *HIT_STATUS_NORMAL);
			WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_DASH_TO_RUN);
		}
}	
#[acmd_script(
    agent = "mewtwo",
    script =  "game_turndash",
    category = ACMD_GAME,
	low_priority)]
unsafe fn m2_turn_dash(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
		frame(fighter.lua_state_agent, 1.0);
		if macros::is_excute(fighter) {
			macros::HIT_NODE(fighter, Hash40::new("s_tail2"), *HIT_STATUS_XLU);
			macros::HIT_NODE(fighter, Hash40::new("s_tail3"), *HIT_STATUS_XLU);
			macros::HIT_NODE(fighter, Hash40::new("s_tail5"), *HIT_STATUS_XLU);
			macros::HIT_NODE(fighter, Hash40::new("s_tail4"), *HIT_STATUS_XLU);
			macros::HIT_NODE(fighter, Hash40::new("s_tail7"), *HIT_STATUS_XLU);
		}
		frame(fighter.lua_state_agent, 3.0);
		if macros::is_excute(fighter) {
			WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_DASH_FLAG_TURN_DASH);
		}
		frame(fighter.lua_state_agent, 13.0);
		if macros::is_excute(fighter) {
			macros::HIT_NODE(fighter, Hash40::new("s_tail2"), *HIT_STATUS_NORMAL);
			macros::HIT_NODE(fighter, Hash40::new("s_tail3"), *HIT_STATUS_NORMAL);
			macros::HIT_NODE(fighter, Hash40::new("s_tail5"), *HIT_STATUS_NORMAL);
			macros::HIT_NODE(fighter, Hash40::new("s_tail4"), *HIT_STATUS_NORMAL);
			macros::HIT_NODE(fighter, Hash40::new("s_tail7"), *HIT_STATUS_NORMAL);
			WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_DASH_TO_RUN);
		}
}	