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
    agent = "toonlink",
    script =  "game_dash",
    category = ACMD_GAME,
	low_priority)]
unsafe fn tink_dash(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
		frame(fighter.lua_state_agent, 10.0);
		if macros::is_excute(fighter) {
			WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_DASH_TO_RUN);
		}
}	
#[acmd_script(
    agent = "toonlink",
    script =  "game_turndash",
    category = ACMD_GAME,
	low_priority)]
unsafe fn tink_dashback(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
		frame(fighter.lua_state_agent, 3.0);
		if macros::is_excute(fighter) {
			WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_DASH_FLAG_TURN_DASH);
		}
		frame(fighter.lua_state_agent, 12.0);
		if macros::is_excute(fighter) {
			WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_DASH_TO_RUN);
		}
}

#[acmd_script( agent = "toonlink_bow", scripts = ["game_nstart", "game_n", "game_nend", "game_nairstart", "game_nair", "game_nairend"], category = ACMD_GAME, low_priority )]
unsafe fn tink_hammer(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        LinkModule::set_model_constraint_pos_ort(fighter.module_accessor, *LINK_NO_ARTICLE, Hash40::new("have"), Hash40::new("havel"), *CONSTRAINT_FLAG_ORIENTATION as u32 | *CONSTRAINT_FLAG_POSITION as u32, false);
    }
}

pub fn install() {
    smashline::install_acmd_scripts!(
		tink_dash,
        tink_dashback,
        tink_hammer
    );
}