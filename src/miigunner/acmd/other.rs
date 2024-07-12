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
    Agent::new("miigunner")
    .acmd("game_catch", gunner_grab, Priority::Low)    
    .install();

}
unsafe extern "C" fn gunner_grab(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
        macros::FT_MOTION_RATE(fighter, 2.0);
        wait(fighter.lua_state_agent, 4.0);
        macros::FT_MOTION_RATE(fighter, 1.0);
		frame(fighter.lua_state_agent, 5.0);
		if macros::is_excute(fighter) {
			GrabModule::set_rebound(fighter.module_accessor, /*CanCatchRebound*/ true);
		}
		frame(fighter.lua_state_agent, 6.0);
        if macros::is_excute(fighter) {
            macros::CATCH(fighter, 0, Hash40::new("top"), 3.2, 0.0, 6.6, 7.3, Some(0.0), Some(6.6), Some(11.6), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_G);
            macros::CATCH(fighter, 1, Hash40::new("top"), 1.6, 0.0, 6.6, 5.7, Some(0.0), Some(6.6), Some(13.2), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_A);
        }
		if true{
			macros::game_CaptureCutCommon(fighter);
		}
		wait(fighter.lua_state_agent, 2.0);
		if macros::is_excute(fighter) {
			grab!(fighter, *MA_MSC_CMD_GRAB_CLEAR_ALL);
			WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT);
			GrabModule::set_rebound(fighter.module_accessor, /*CanCatchRebound*/ false);
		}
}	