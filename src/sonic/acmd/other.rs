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
    agent = "sonic",
    script =  "effect_downattacku",
    category = ACMD_EFFECT,
	low_priority)]
unsafe fn sonic_getup_attack_eff(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
		frame(fighter.lua_state_agent, 14.0);
		if macros::is_excute(fighter) {
			macros::LANDING_EFFECT_FLIP(fighter, Hash40::new("sys_whirlwind_r"), Hash40::new("sys_whirlwind_l"), Hash40::new("top"), -2, 0, 0, 0, 0, 0, 1.2, 0, 0, 0, 0, 0, 0, false, *EF_FLIP_NONE);
		}
		frame(fighter.lua_state_agent, 15.0);
		if macros::is_excute(fighter) {
			macros::EFFECT_FOLLOW_FLIP(fighter, Hash40::new("sys_attack_arc_d"), Hash40::new("sys_attack_arc_d"), Hash40::new("top"), -1, 5, 8, 185, 150, 5, 1.1, true, *EF_FLIP_YZ);
			macros::LAST_EFFECT_SET_COLOR(fighter, 3, 0.0, 0.0);
		}
		frame(fighter.lua_state_agent, 19.0);
		if macros::is_excute(fighter) {
			macros::EFFECT_OFF_KIND(fighter, Hash40::new("sys_attack_arc_d"), false, false);
			macros::EFFECT_FOLLOW_FLIP(fighter, Hash40::new("sys_attack_arc_d"), Hash40::new("sys_attack_arc_d"), Hash40::new("top"), 0, 5, -6.5, 185, -20, 8, 1.1, true, *EF_FLIP_YZ);
			macros::LAST_EFFECT_SET_COLOR(fighter, 3, 0.0, 0.0);
		}
		frame(fighter.lua_state_agent, 25.0);
		if macros::is_excute(fighter) {
			macros::EFFECT_OFF_KIND(fighter, Hash40::new("sys_attack_arc_d"), false, false);
		}
}		

#[acmd_script(
    agent = "sonic",
    scripts =  ["effect_appeallwl", "effect_appeallwr"],
    category = ACMD_EFFECT,
	low_priority)]
unsafe fn sonic_dtaunt_eff(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 14.0);
    if macros::is_excute(agent) {
        macros::LANDING_EFFECT_FLIP(agent, Hash40::new("sys_whirlwind_l"), Hash40::new("sys_whirlwind_r"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.65, 0, 0, 0, 0, 0, 0, false, *EF_FLIP_YZ);
    }
    frame(agent.lua_state_agent, 16.0);
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("sys_spin_wind"), Hash40::new("top"), -2.2, 8.5, 0, -20, 15, 11, 0.92, 0, 0, 0, 0, 0, 0, true);
        macros::LAST_EFFECT_SET_ALPHA(agent, 0.55);
    }
    frame(agent.lua_state_agent, 29.0);
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("sys_spin_wind"), Hash40::new("top"), -2, 9.5, 0, -35, 38, 19, 0.92, 0, 0, 0, 0, 0, 0, true);
        macros::LAST_EFFECT_SET_ALPHA(agent, 0.55);
    }
    frame(agent.lua_state_agent, 47.0);
    if macros::is_excute(agent) {
        macros::LANDING_EFFECT(agent, Hash40::new("sys_down_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.5, 0, 0, 0, 0, 0, 0, true);
    }
    frame(agent.lua_state_agent, 130.0);
    if macros::is_excute(agent) {
        macros::LANDING_EFFECT(agent, Hash40::new("sys_down_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.5, 0, 0, 0, 0, 0, 0, true);
    }
    frame(agent.lua_state_agent, 168.0);
    if macros::is_excute(agent) {
        macros::LANDING_EFFECT(agent, Hash40::new("sys_landing_smoke_s"), Hash40::new("top"), 0, 0, -1, 0, 0, 0, 0.5, 0, 0, 0, 0, 0, 0, true);
    }
}

pub fn install() {
    smashline::install_acmd_scripts!(
		sonic_getup_attack_eff,
        sonic_dtaunt_eff
    );
}