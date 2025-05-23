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
use crate::mario::*;
use super::*;

pub fn install() {
    Agent::new("mario")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .acmd("sound_appealhir", mario_utaunt_snd, Priority::Low)    
    .acmd("sound_appealhil", mario_utaunt_snd, Priority::Low)    
    .acmd("effect_appealhir", mario_utaunt_eff, Priority::Low)    
    .acmd("effect_appealhil", mario_utaunt_eff, Priority::Low)    
    .acmd("game_run", mario_run_game, Priority::Low)    
    .install();
}

unsafe extern "C" fn mario_run_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
	let ENTRY_ID = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
	frame(fighter.lua_state_agent, 1.0);
		for _ in 0..i32::MAX {
			if macros::is_excute(fighter) {
				RUNLOOPCOUNT[ENTRY_ID] += 1;
			}
			wait(fighter.lua_state_agent, 1.0);
		}
}	
unsafe extern "C" fn mario_utaunt_snd(fighter: &mut L2CAgentBase) {
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
unsafe extern "C" fn mario_utaunt_eff(fighter: &mut L2CAgentBase) {
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