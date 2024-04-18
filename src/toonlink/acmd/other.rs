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
    Agent::new("toonlink")
    .acmd("game_dash", tink_dash)    
    .acmd("game_turndash", tink_dashback)    
    .acmd("game_nstart", tink_hammer)    
    .acmd("game_n", tink_hammer)    
    .acmd("game_nend", tink_hammer)    
    .acmd("game_nairstart", tink_hammer)    
    .acmd("game_nair", tink_hammer)    
    .acmd("game_nairend", tink_hammer)    
    .install();
}

unsafe extern "C" fn tink_dash(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
		frame(fighter.lua_state_agent, 10.0);
		if macros::is_excute(fighter) {
			WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_DASH_TO_RUN);
		}
}	
unsafe extern "C" fn tink_dashback(fighter: &mut L2CAgentBase) {
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

unsafe extern "C" fn tink_hammer(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        LinkModule::set_model_constraint_pos_ort(fighter.module_accessor, *LINK_NO_ARTICLE, Hash40::new("have"), Hash40::new("havel"), *CONSTRAINT_FLAG_ORIENTATION as u32 | *CONSTRAINT_FLAG_POSITION as u32, false);
    }
}