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

#[fighter_frame_callback]
pub fn wolf(fighter : &mut L2CFighterCommon) {
    unsafe {
        let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent); 
		if is_default(boma) {
			let status_kind = smash::app::lua_bind::StatusModule::status_kind(boma);
			let fighter_kind = smash::app::utility::get_kind(boma);
			if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_N && fighter_kind == *FIGHTER_KIND_WOLF {
				if MotionModule::frame(boma) < 3.0 && MotionModule::frame(boma) > 1.0 {
					MotionModule::set_rate(boma, 2.0);
				} else {
					MotionModule::set_rate(boma, 1.0);
				};
			};
		}
    }
}

pub fn install() {
    smashline::install_agent_frame_callbacks!(wolf);
}