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

#[acmd_script( agent = "pacman", 
script = "expression_fallspecial", 
category = ACMD_EXPRESSION,
low_priority)]
unsafe fn pac_freefall(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
}

pub fn install() {
    smashline::install_acmd_scripts!(
		pac_freefall
    );
}