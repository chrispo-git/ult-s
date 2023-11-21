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
		mario_utaunt_eff,
		mario_utaunt_snd
    );
}
#[acmd_script(
    agent = "mario",
    scripts =  ["sound_appealhir", "sound_appealhil"],
    category = ACMD_SOUND,
	low_priority)]
unsafe fn mario_utaunt_snd(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
		frame(fighter.lua_state_agent, 13.0);
		if macros::is_excute(fighter) {
			macros::PLAY_SE(fighter, Hash40::new("se_item_mushroom"));
		}
		frame(fighter.lua_state_agent, 118.0);
		if macros::is_excute(fighter) {
			macros::PLAY_SE(fighter, Hash40::new("se_item_mushd"));
		}
}	
#[acmd_script(
    agent = "mario",
    scripts =  ["effect_appealhir", "effect_appealhil"],
    category = ACMD_EFFECT,
	low_priority)]
unsafe fn mario_utaunt_eff(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
		frame(fighter.lua_state_agent, 11.0);
		for _ in 0..6 
 {
			if macros::is_excute(fighter) {
				macros::FLASH(fighter, 1.0, 1.0, 0.502, 0.667);
			}
			wait(fighter.lua_state_agent, 1.0);
			if macros::is_excute(fighter) {
				macros::FLASH(fighter, 0.313, 0.313, 0.313, 0.35);
			}
			wait(fighter.lua_state_agent, 3.0);
			if macros::is_excute(fighter) {
				macros::FLASH(fighter, 0.313, 0.313, 0.313, 0.35);
			}
			wait(fighter.lua_state_agent, 3.0);
			if macros::is_excute(fighter) {
				macros::FLASH(fighter, 1.0, 1.0, 0.502, 0.667);
			}
			wait(fighter.lua_state_agent, 3.0);
		}
		wait(fighter.lua_state_agent, 1.0);
		if macros::is_excute(fighter) {
			macros::COL_NORMAL(fighter, );
		}
}	