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
	smashline::install_agent_frame_callbacks!(luigi);
}
#[fighter_frame_callback]
pub fn luigi(fighter : &mut L2CFighterCommon) {
    unsafe {
        let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent); 
		let status_kind = smash::app::lua_bind::StatusModule::status_kind(boma);
		let mut stick_x = ControlModule::get_stick_x(boma) ;
		let fighter_kind = smash::app::utility::get_kind(boma);
		let frame = MotionModule::frame(boma);
		if fighter_kind == *FIGHTER_KIND_LUIGI && is_default(boma) {
			if (ControlModule::get_command_flag_cat(boma, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_LW3) != 0 && frame >= 13.0 && status_kind == *FIGHTER_STATUS_KIND_ATTACK_LW3{
				MotionModule::change_motion(boma, smash::phx::Hash40::new("attack_lw3"), 0.0, 1.0, false, 0.0, false, false);
			};
		};
	};
}