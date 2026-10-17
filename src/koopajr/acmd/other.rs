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
    Agent::new("koopajr")
    .set_costume(get_marked_costumes("koopajr","koopajr"))
    .acmd("game_jumpbackmini", game_jumpbackmini, Priority::Low)     
    .acmd("game_jumpfrontmini", game_jumpfrontmini, Priority::Low)    
    .install();
}

unsafe extern "C" fn game_jumpbackmini(agent: &mut L2CAgentBase) {
    macros::FT_MOTION_RATE(agent, 0.8);
}
unsafe extern "C" fn game_jumpfrontmini(agent: &mut L2CAgentBase) {
    macros::FT_MOTION_RATE(agent, 0.8);
}
