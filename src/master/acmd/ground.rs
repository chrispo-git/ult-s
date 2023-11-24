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
		axe_dsmash_eff
    );
}	
#[acmd_script(
    agent = "master_axe",
    scripts =  ["effect_attacklw4"],
    category = ACMD_EFFECT,
	low_priority)]
unsafe fn axe_dsmash_eff(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
		frame(fighter.lua_state_agent, 1.0);
		if macros::is_excute(fighter) {
			macros::EFFECT_FOLLOW_ALPHA(fighter, Hash40::new("master_axeflare1"), Hash40::new("blade1"), 0, 0, 0, 0, 0, 0, 1, true, 0.58);
			macros::EFFECT_FOLLOW_ALPHA(fighter, Hash40::new("master_axeflare2"), Hash40::new("blade2"), 0, 0, 0, 0, 0, 0, 1, true, 0.58);
			macros::EFFECT_FOLLOW_ALPHA(fighter, Hash40::new("master_axeflare3"), Hash40::new("axe"), 0, 0, 0, 0, 0, 0, 1, true, 0.58);
			macros::EFFECT_FOLLOW_ALPHA(fighter, Hash40::new("master_axe_hold"), Hash40::new("blade2"), 0, 0, 0, 0, 0, 0, 0.8, true, 0.75);
		}
		frame(fighter.lua_state_agent, 5.0);
		if macros::is_excute(fighter) {
			macros::EFFECT_OFF_KIND(fighter, Hash40::new("master_axe_hold"), false, true);
		}
		frame(fighter.lua_state_agent, 15.0);
		if macros::is_excute(fighter) {
			macros::EFFECT_FOLLOW_ALPHA(fighter, Hash40::new_raw(0x1067610ee8), Hash40::new("blade1"), 0, 0, 0, 0, 0, 0, 1, true, 0.8);
			macros::EFFECT_FOLLOW_ALPHA(fighter, Hash40::new_raw(0x10fe685f52), Hash40::new("blade2"), 0, 0, 0, 0, 0, 0, 1, true, 0.8);
			macros::EFFECT_FOLLOW_ALPHA(fighter, Hash40::new_raw(0x10896f6fc4), Hash40::new("axe"), 0, 0, 0, 0, 0, 0, 1, true, 0.8);
		}
		frame(fighter.lua_state_agent, 25.0);
		if macros::is_excute(fighter) {
			macros::EFFECT_OFF_KIND(fighter, Hash40::new_raw(0x1067610ee8), true, true);
			macros::EFFECT_OFF_KIND(fighter, Hash40::new_raw(0x10fe685f52), true, true);
			macros::EFFECT_OFF_KIND(fighter, Hash40::new_raw(0x10896f6fc4), true, true);
			macros::EFFECT_FOLLOW_ALPHA(fighter, Hash40::new_raw(0x1067610ee8), Hash40::new("blade1"), 0, 0, 0, 0, 0, 0, 1, true, 0.8);
			macros::EFFECT_FOLLOW_ALPHA(fighter, Hash40::new_raw(0x10fe685f52), Hash40::new("blade2"), 0, 0, 0, 0, 0, 0, 1, true, 0.8);
			macros::EFFECT_FOLLOW_ALPHA(fighter, Hash40::new_raw(0x10896f6fc4), Hash40::new("axe"), 0, 0, 0, 0, 0, 0, 1, true, 0.8);
		}
		frame(fighter.lua_state_agent, 37.0);
		if macros::is_excute(fighter) {
			macros::EFFECT_OFF_KIND(fighter, Hash40::new("master_axeflare1"), false, true);
			macros::EFFECT_OFF_KIND(fighter, Hash40::new("master_axeflare2"), false, true);
			macros::EFFECT_OFF_KIND(fighter, Hash40::new("master_axeflare3"), false, true);
			macros::EFFECT_OFF_KIND(fighter, Hash40::new_raw(0x10fe685f52), false, true);
			macros::EFFECT_OFF_KIND(fighter, Hash40::new_raw(0x10896f6fc4), false, true);
			macros::EFFECT_OFF_KIND(fighter, Hash40::new_raw(0x1067610ee8), false, true);
			macros::EFFECT_OFF_KIND(fighter, Hash40::new_raw(0x10fe685f52), false, true);
			macros::EFFECT_OFF_KIND(fighter, Hash40::new_raw(0x10896f6fc4), false, true);
		}
	}	