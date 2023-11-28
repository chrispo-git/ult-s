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
pub fn pt(fighter : &mut L2CFighterCommon) {
    unsafe {
        let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent); 
		let status_kind = smash::app::lua_bind::StatusModule::status_kind(boma);
		let lua_state = fighter.lua_state_agent;
		let fighter_kind = smash::app::utility::get_kind(boma);
		if is_default(boma) {
			if fighter_kind == *FIGHTER_KIND_PFUSHIGISOU {
				if [hash40("attack_air_hi")].contains(&MotionModule::motion_kind(boma)) {
					if MotionModule::frame(boma) < 12.0 {
						MotionModule::set_rate(boma, 0.7059);
					} else {
						MotionModule::set_rate(boma, 1.0);
					};
				};
			};
			if fighter_kind == *FIGHTER_KIND_PLIZARDON {
				if [hash40("special_hi"), hash40("special_air_hi")].contains(&MotionModule::motion_kind(boma)) {
					if MotionModule::frame(boma) < 6.0 {
						MotionModule::set_rate(boma, 3.0);
					} else {
						MotionModule::set_rate(boma, 1.0);
					};
				};
			};
		}
	};
}	

pub fn install() {
    smashline::install_agent_frame_callbacks!(pt);
}