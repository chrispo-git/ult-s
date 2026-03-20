mod status;
mod frame;
mod acmd;
use crate::util::*;
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


//Joker Gun Cancel 
static mut IS_ARSENE: [bool; 8] = [false; 8];
static mut X: [f32; 8] = [0.0; 8];
static mut Y: [f32; 8] = [0.0; 8];
static mut BATON_TYPE: [i32; 8] = [0; 8];
static BATON_MAX : i32 = 2;


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

	param_config::update_int_2(*FIGHTER_KIND_JACK, get_marked_costumes("jack","jack"), (smash::hash40("escape_num_max"), smash::hash40("reflector_radius_size"), 10));
	param_config::update_float_2(*FIGHTER_KIND_JACK, get_marked_costumes("jack","jack"), (smash::hash40("param_special_lw"), smash::hash40("counter_attack_power_mul"), 1.4));
	param_config::update_float_2(*FIGHTER_KIND_JACK, get_marked_costumes("jack","jack"), (smash::hash40("param_special_lw"), smash::hash40("reflector_attack_power_mul"), 1.4));
	param_config::update_float_2(*FIGHTER_KIND_JACK, get_marked_costumes("jack","jack"), (smash::hash40("param_special_lw"), smash::hash40("guard_rebel_gauge"), 11.943));
	param_config::update_float_2(*FIGHTER_KIND_JACK, get_marked_costumes("jack","jack"), (smash::hash40("param_special_lw"), smash::hash40("rebel_gauge_add"), 0.00925));
	param_config::update_float_2(*FIGHTER_KIND_JACK, get_marked_costumes("jack","jack"), (smash::hash40("param_special_lw"), smash::hash40("shield_radius_size"), 8.0));
	param_config::update_float_2(*FIGHTER_KIND_JACK, get_marked_costumes("jack","jack"), (smash::hash40("param_special_lw"), smash::hash40("reflector_radius_size"), 9.0));
	param_config::update_float_2(*FIGHTER_KIND_JACK, get_marked_costumes("jack","jack"), (smash::hash40("param_private"), smash::hash40("rebel_gauge_damage_add"), 1.1));
	param_config::update_float_2(*FIGHTER_KIND_JACK, get_marked_costumes("jack","jack"), (smash::hash40("param_private"), smash::hash40("rebel_gauge_add"), 0.0));
	param_config::update_float_2(*FIGHTER_KIND_JACK, get_marked_costumes("jack","jack"), (smash::hash40("ground_brake"), 0, 0.09));
	param_config::update_float_2(*FIGHTER_KIND_JACK, get_marked_costumes("jack","jack"), (smash::hash40("dash_speed"), 0, 2.17));
	param_config::update_float_2(*FIGHTER_KIND_JACK, get_marked_costumes("jack","jack"), (smash::hash40("run_speed_max"), 0, 2.0));
	param_config::update_float_2(*FIGHTER_KIND_JACK, get_marked_costumes("jack","jack"), (smash::hash40("jump_speed_x_mul"), 0, 1.0));
	param_config::update_float_2(*FIGHTER_KIND_JACK, get_marked_costumes("jack","jack"), (smash::hash40("air_accel_x_mul"), 0, 0.06));
	param_config::update_float_2(*FIGHTER_KIND_JACK, get_marked_costumes("jack","jack"), (smash::hash40("landing_attack_air_frame_b"), 0, 11.0));
	param_config::update_float_2(*FIGHTER_KIND_JACK, get_marked_costumes("jack","jack"), (smash::hash40("air_speed_x_stable"), 0, 1.175));
	param_config::update_float_2(*FIGHTER_KIND_JACK, get_marked_costumes("jack","jack"), (smash::hash40("air_speed_y_stable"), 0, 1.73));
	param_config::update_float_2(*FIGHTER_KIND_JACK, get_marked_costumes("jack","jack"), (smash::hash40("dive_speed_y"), 0, 3.287));

}