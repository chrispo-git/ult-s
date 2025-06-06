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
    Agent::new("kamui")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .acmd("effect_dash", corrin_dash_eff, Priority::Low)    
    .acmd("effect_turndash", corrin_dash_eff, Priority::Low)    
    .acmd("effect_run", corrin_run_eff, Priority::Low)    
    .acmd("effect_turnrun", corrin_turnrun_eff, Priority::Low)    
    .acmd("effect_runbrakel", corrin_runbrake_eff, Priority::Low)    
    .acmd("effect_runbraker", corrin_runbrake_eff, Priority::Low)    
    .install();
}

unsafe extern "C" fn corrin_dash_eff(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
		frame(fighter.lua_state_agent, 3.0);
		if macros::is_excute(fighter) {
			macros::FOOT_EFFECT(fighter, Hash40::new("kamui_water_splash"), Hash40::new("top"), -3, 0, 0, 0, 0, 0, 0.75, 0, 0, 0, 0, 0, 0, false);
		}
		frame(fighter.lua_state_agent, 15.0);
		if macros::is_excute(fighter) {
			macros::FOOT_EFFECT(fighter, Hash40::new("null"), Hash40::new("top"), 2, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
		}
}
unsafe extern "C" fn corrin_run_eff(fighter: &mut L2CAgentBase) {
    for _ in 0..i32::MAX {
        frame(fighter.lua_state_agent, 4.0);
        if macros::is_excute(fighter) {
            macros::FOOT_EFFECT(fighter, Hash40::new("kamui_water_sibuki_s"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.0, 0, 0, 0, 0, 0, 0, false);
        }
        fighter.clear_lua_stack();
        wait_loop_sync_mot(fighter.lua_state_agent);
        fighter.pop_lua_stack(1);
    }
}
unsafe extern "C" fn corrin_turnrun_eff(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
		frame(fighter.lua_state_agent, 10.0);
		if macros::is_excute(fighter) {
			macros::FOOT_EFFECT(fighter, Hash40::new("kamui_water_splash"), Hash40::new("top"), -6, 0, 0, 0, 0, 0, 0.75, 0, 0, 0, 0, 0, 0, false);
		}
		frame(fighter.lua_state_agent, 16.0);
		if macros::is_excute(fighter) {
			macros::FOOT_EFFECT(fighter, Hash40::new("kamui_water_splash"), Hash40::new("top"), -5, 0, 0, 0, 0, 0, 0.5, 0, 0, 0, 0, 0, 0, false);
		}
}
unsafe extern "C" fn corrin_runbrake_eff(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
		frame(fighter.lua_state_agent, 4.0);
		if macros::is_excute(fighter) {
			macros::FOOT_EFFECT(fighter, Hash40::new("kamui_water_splash"), Hash40::new("top"), 7.5, 0, 0, 0, 180, 0, 0.62, 0, 0, 0, 0, 0, 0, false);
		}
		wait(fighter.lua_state_agent, 6.0);
		if macros::is_excute(fighter) {
			macros::FOOT_EFFECT(fighter, Hash40::new("kamui_water_splash"), Hash40::new("top"), 7.5, 0, 0, 0, 180, 0, 0.62, 0, 0, 0, 0, 0, 0, false);
		}
		wait(fighter.lua_state_agent, 6.0);
}