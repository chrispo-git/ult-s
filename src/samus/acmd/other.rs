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
  Agent::new("samus")
  .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
  .acmd("effect_jumpfrontmini", samus_shorthop_eff, Priority::Low)    
  .acmd("effect_jumpbackmini", samus_shorthop_eff, Priority::Low)    
  .install();
}

unsafe extern "C" fn samus_shorthop_eff(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    if macros::is_excute(fighter) {
      macros::EFFECT_FOLLOW(fighter, Hash40::new("sys_jump_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, false);
    }
}