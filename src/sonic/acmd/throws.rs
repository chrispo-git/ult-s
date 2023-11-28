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
    script =  "effect_throwf",
    category = ACMD_EFFECT,
	low_priority)]
unsafe fn sonic_fthrow_eff(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
		frame(fighter.lua_state_agent, 8.0);
		if macros::is_excute(fighter) {
			macros::EFFECT_FOLLOW_NO_STOP_FLIP(fighter, Hash40::new("sys_attack_arc"), Hash40::new("sys_attack_arc_d"), Hash40::new("top"), -2, 13, 4, 6, -70, 80, 0.9, true, *EF_FLIP_YZ);
			macros::LAST_EFFECT_SET_COLOR(fighter, 3, 0.0, 0.0);
			macros::LAST_EFFECT_SET_RATE(fighter, 1.5);
		}
		frame(fighter.lua_state_agent, 12.0);
		if macros::is_excute(fighter) {
			macros::EFFECT_FLIP(fighter, Hash40::new("sys_smash_flash_s"), Hash40::new("sys_smash_flash_s"), Hash40::new("throw"), 3, 2, 3, 0, 0, 0, 1.5, 0, 0, 0, 0, 0, 0, true, *EF_FLIP_YZ);
		}
}	
#[acmd_script(
    agent = "sonic",
    script =  "effect_throwb",
    category = ACMD_EFFECT,
	low_priority)]
unsafe fn sonic_bthrow_eff(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
		frame(fighter.lua_state_agent, 11.0);
		if macros::is_excute(fighter) {
			macros::LANDING_EFFECT(fighter, Hash40::new("sys_landing_smoke_s"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.6, 0, 0, 0, 0, 0, 0, true);
		}
		frame(fighter.lua_state_agent, 17.0);
		if macros::is_excute(fighter) {
			macros::EFFECT_FOLLOW_FLIP_ALPHA(fighter, Hash40::new("sys_spin_wind"), Hash40::new("sys_spin_wind"), Hash40::new("trans"), 0, 16, 0, 0, 0, 90, 0.65, true, *EF_FLIP_YZ, 0.6);
		}
		frame(fighter.lua_state_agent, 21.0);
		if macros::is_excute(fighter) {
			macros::EFFECT_FOLLOW_FLIP_ALPHA(fighter, Hash40::new("sys_spin_wind"), Hash40::new("sys_spin_wind"), Hash40::new("trans"), 0, 16, 0, 0, 180, 90, 1, true, *EF_FLIP_YZ, 0.6);
		}
		frame(fighter.lua_state_agent, 41.0);
		if macros::is_excute(fighter) {
			macros::EFFECT_FOLLOW_NO_STOP_FLIP(fighter, Hash40::new("sys_attack_arc"), Hash40::new("sys_attack_arc_d"), Hash40::new("top"), -2, 5, -1.5, 6, 110, 80, 0.9, true, *EF_FLIP_YZ);
			macros::LAST_EFFECT_SET_COLOR(fighter, 3, 0.0, 0.0);
			macros::LAST_EFFECT_SET_RATE(fighter, 1.6);
			macros::EFFECT_FLIP(fighter, Hash40::new("sys_smash_flash_s"), Hash40::new("sys_smash_flash_s"), Hash40::new("throw"), 3, 3, 10, 0, 0, 0, 1.5, 0, 0, 0, 0, 0, 0, true, *EF_FLIP_YZ);
		}
		frame(fighter.lua_state_agent, 41.0);
		if macros::is_excute(fighter) {
			macros::LANDING_EFFECT(fighter, Hash40::new("sys_landing_smoke"), Hash40::new("top"), 0, 0, -3, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
		}
		frame(fighter.lua_state_agent, 45.0);
		if macros::is_excute(fighter) {
			macros::EFFECT_OFF_KIND(fighter, Hash40::new("sys_attack_arc"), false, true);
		}
}	

pub fn install() {
    smashline::install_acmd_scripts!(
		sonic_fthrow_eff,
        sonic_bthrow_eff
    );
}