mod status;
mod frame;
mod acmd;
use crate::util::*;

static mut HAS_DOWNB : [bool; 8] = [false; 8];
static mut DOWNB_X : [f32; 8] = [0.0; 8];
static mut DOWNB_Y : [f32; 8] = [0.0; 8];
			

pub fn install() {
	frame::install();
	status::install();
	acmd::install();

	param_config::update_float_2(*FIGHTER_KIND_LUCARIO, get_marked_costumes("lucario","lucario").into_iter().map(|x| x as i32).collect(), (smash::hash40("param_aurapower"), smash::hash40("min_aurapower"), 1.0));
	param_config::update_float_2(*FIGHTER_KIND_LUCARIO, get_marked_costumes("lucario","lucario").into_iter().map(|x| x as i32).collect(), (smash::hash40("param_aurapower"), smash::hash40("max_aurapower"), 1.0));
	param_config::update_int_2(*FIGHTER_KIND_LUCARIO, get_marked_costumes("lucario","lucario").into_iter().map(|x| x as i32).collect(), (smash::hash40("param_special_hi"), smash::hash40("landing_frame"), 27));
	param_config::update_float_2(*FIGHTER_KIND_LUCARIO, get_marked_costumes("lucario","lucario").into_iter().map(|x| x as i32).collect(), (smash::hash40("air_speed_x_stable"), 0, 1.155));
	param_config::update_float_2(*FIGHTER_KIND_LUCARIO, get_marked_costumes("lucario","lucario").into_iter().map(|x| x as i32).collect(), (smash::hash40("mini_jump_y"), 0, 16.0));

}

