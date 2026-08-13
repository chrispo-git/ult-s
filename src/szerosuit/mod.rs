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

static FIGHTER_SZEROSUIT_INSTANCE_WORK_ID_FLAG_CAN_CANCEL : i32 = 0;
static FIGHTER_SZEROSUIT_INSTANCE_WORK_ID_INT_CAN_CANCEL_TIMER : i32 = 1;
static FIGHTER_SZEROSUIT_INSTANCE_WORK_ID_INT_NO_WAVEDASH_TIMER : i32 = 2;
static FIGHTER_SZEROSUIT_INSTANCE_WORK_ID_FLAG_IS_AIR_SIDEB : i32 = 3;
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

	 crate::param_cache::update_float_2(*FIGHTER_KIND_SZEROSUIT, get_marked_costumes("szerosuit","szerosuit").into_iter().map(|x| x as i32).collect(), (smash::hash40("param_special_lw"), smash::hash40("speed_mul"), 0.90));
	 crate::param_cache::update_float_2(*FIGHTER_KIND_SZEROSUIT, get_marked_costumes("szerosuit","szerosuit").into_iter().map(|x| x as i32).collect(), (smash::hash40("param_special_lw"), smash::hash40("control_speed_x_mul"), 0.70));
	 crate::param_cache::update_float_2(*FIGHTER_KIND_SZEROSUIT, get_marked_costumes("szerosuit","szerosuit").into_iter().map(|x| x as i32).collect(), (smash::hash40("dash_speed"), 0, 2.3));
	 crate::param_cache::update_float_2(*FIGHTER_KIND_SZEROSUIT, get_marked_costumes("szerosuit","szerosuit").into_iter().map(|x| x as i32).collect(), (smash::hash40("run_speed_max"), 0, 2.15));
	 crate::param_cache::update_float_2(*FIGHTER_KIND_SZEROSUIT, get_marked_costumes("szerosuit","szerosuit").into_iter().map(|x| x as i32).collect(), (smash::hash40("mini_jump_y"), 0, 14.0));
	 crate::param_cache::update_float_2(*FIGHTER_KIND_SZEROSUIT, get_marked_costumes("szerosuit","szerosuit").into_iter().map(|x| x as i32).collect(), (smash::hash40("air_accel_x_mul"), 0, 0.07));
	 crate::param_cache::update_float_2(*FIGHTER_KIND_SZEROSUIT, get_marked_costumes("szerosuit","szerosuit").into_iter().map(|x| x as i32).collect(), (smash::hash40("weight"), 0, 83.0));
	 crate::param_cache::update_float_2(*FIGHTER_KIND_SZEROSUIT, get_marked_costumes("szerosuit","szerosuit").into_iter().map(|x| x as i32).collect(), (smash::hash40("landing_attack_air_frame_n"), 0, 8.0));
	 crate::param_cache::update_float_2(*FIGHTER_KIND_SZEROSUIT, get_marked_costumes("szerosuit","szerosuit").into_iter().map(|x| x as i32).collect(), (smash::hash40("landing_attack_air_frame_b"), 0, 11.0));
	 crate::param_cache::update_float_2(*FIGHTER_KIND_SZEROSUIT, get_marked_costumes("szerosuit","szerosuit").into_iter().map(|x| x as i32).collect(), (smash::hash40("landing_attack_air_frame_lw"), 0, 10.0));
	 crate::param_cache::update_float_2(*FIGHTER_KIND_SZEROSUIT, get_marked_costumes("szerosuit","szerosuit").into_iter().map(|x| x as i32).collect(), (smash::hash40("combo_attack_12_end"), 0, 0.0));
	 crate::param_cache::update_float_2(*FIGHTER_KIND_SZEROSUIT, get_marked_costumes("szerosuit","szerosuit").into_iter().map(|x| x as i32).collect(), (smash::hash40("combo_attack_13_end"), 0, 0.0));
	crate::param_cache::update_int_2(*FIGHTER_KIND_SZEROSUIT, get_marked_costumes("szerosuit","szerosuit").into_iter().map(|x| x as i32).collect(), (smash::hash40("air_lasso_type"), 0, 1));

}