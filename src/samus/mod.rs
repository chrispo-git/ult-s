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

mod status;
mod frame;
mod acmd;
use crate::util::*;

static mut NO_WAVEDASH_TIMER : [i32; 8] = [0; 8];
static NO_WAVEDASH_MAX : i32 = 8;


pub(crate) fn check_jump(boma: &mut smash::app::BattleObjectModuleAccessor) -> bool {
	unsafe {
		if ControlModule::check_button_on_trriger(boma, *CONTROL_PAD_BUTTON_JUMP) {
			return true;
		};
		if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_FLICK_JUMP) {
			if ControlModule::get_flick_y(boma) >= 3 && ControlModule::get_stick_y(boma) >= 0.7 {
				return true;
			};
		};
		if ControlModule::check_button_on_trriger(boma, *CONTROL_PAD_BUTTON_JUMP_MINI) {
			return true;
		};
		return false;
	}
}
			


pub fn install() {
	frame::install();
	status::install();
	acmd::install();

	param_config::update_float_2(-*WEAPON_KIND_SAMUS_CSHOT, get_marked_costumes("samus","samus").into_iter().map(|x| x as i32).collect(), (smash::hash40("param_cshot"), smash::hash40("min_speed"), 2.5));
	param_config::update_float_2(*FIGHTER_KIND_SAMUS, get_marked_costumes("samus","samus").into_iter().map(|x| x as i32).collect(), (smash::hash40("jump_speed_x_mul"), 0, 0.9));

}