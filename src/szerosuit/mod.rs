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

static mut CAN_CANCEL : [bool; 8] = [false; 8];
static mut CAN_CANCEL_TIMER : [i32; 8] = [0; 8];
static mut NO_WAVEDASH_TIMER : [i32; 8] = [0; 8];
static mut IS_AIR_SIDEB : [bool; 8] = [false; 8];
static NO_WAVEDASH_MAX : i32 = 8;
static WINDOW : i32 = 20;

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

	param_config::update_float_2(*FIGHTER_KIND_SZEROSUIT, get_marked_costumes("szerosuit","szerosuit"), (smash::hash40("param_special_lw"), smash::hash40("speed_mul"), 0.90));
	param_config::update_float_2(*FIGHTER_KIND_SZEROSUIT, get_marked_costumes("szerosuit","szerosuit"), (smash::hash40("param_special_lw"), smash::hash40("control_speed_x_mul"), 0.70));
	param_config::update_float_2(*FIGHTER_KIND_SZEROSUIT, get_marked_costumes("szerosuit","szerosuit"), (smash::hash40("dash_speed"), 0, 2.3));
	param_config::update_float_2(*FIGHTER_KIND_SZEROSUIT, get_marked_costumes("szerosuit","szerosuit"), (smash::hash40("run_speed_max"), 0, 2.15));
	param_config::update_float_2(*FIGHTER_KIND_SZEROSUIT, get_marked_costumes("szerosuit","szerosuit"), (smash::hash40("mini_jump_y"), 0, 14.0));
	param_config::update_float_2(*FIGHTER_KIND_SZEROSUIT, get_marked_costumes("szerosuit","szerosuit"), (smash::hash40("air_accel_x_mul"), 0, 0.07));
	param_config::update_float_2(*FIGHTER_KIND_SZEROSUIT, get_marked_costumes("szerosuit","szerosuit"), (smash::hash40("weight"), 0, 83.0));
	param_config::update_float_2(*FIGHTER_KIND_SZEROSUIT, get_marked_costumes("szerosuit","szerosuit"), (smash::hash40("landing_attack_air_frame_n"), 0, 8.0));
	param_config::update_float_2(*FIGHTER_KIND_SZEROSUIT, get_marked_costumes("szerosuit","szerosuit"), (smash::hash40("landing_attack_air_frame_b"), 0, 11.0));
	param_config::update_float_2(*FIGHTER_KIND_SZEROSUIT, get_marked_costumes("szerosuit","szerosuit"), (smash::hash40("landing_attack_air_frame_lw"), 0, 10.0));
	param_config::update_float_2(*FIGHTER_KIND_SZEROSUIT, get_marked_costumes("szerosuit","szerosuit"), (smash::hash40("combo_attack_12_end"), 0, 0.0));
	param_config::update_float_2(*FIGHTER_KIND_SZEROSUIT, get_marked_costumes("szerosuit","szerosuit"), (smash::hash40("combo_attack_13_end"), 0, 0.0));
	param_config::update_int_2(*FIGHTER_KIND_SZEROSUIT, get_marked_costumes("szerosuit","szerosuit"), (smash::hash40("air_lasso_type"), 0, 1));

}