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
    Agent::new("ridley")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .acmd("game_specialnstart", ridley_neutralb_start, Priority::Low)    
    .acmd("game_specialairnstart", ridley_neutralb_start, Priority::Low)    
    .install();
}
unsafe extern "C" fn ridley_neutralb_start(fighter: &mut L2CAgentBase) {
	macros::FT_MOTION_RATE(fighter, /*FSM*/ 0.333);
}	