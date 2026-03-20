mod status;
mod frame;
mod acmd;
use crate::util::*;
use smash::lib::lua_const::*;
use smash::hash40;

			

pub fn install() {
	frame::install();
	status::install();
	acmd::install();

	param_config::update_float_2(*FIGHTER_KIND_ROY, get_marked_costumes("roy","roy").into_iter().map(|x| x as i32).collect(), (smash::hash40("dash_speed"), 0, 2.3));
	param_config::update_float_2(*FIGHTER_KIND_ROY, get_marked_costumes("roy","roy").into_iter().map(|x| x as i32).collect(), (smash::hash40("air_accel_x_mul"), 0, 0.05));
	param_config::update_float_2(*FIGHTER_KIND_ROY, get_marked_costumes("roy","roy").into_iter().map(|x| x as i32).collect(), (smash::hash40("air_speed_x_stable"), 0, 1.28));
	param_config::update_float_2(*FIGHTER_KIND_ROY, get_marked_costumes("roy","roy").into_iter().map(|x| x as i32).collect(), (smash::hash40("air_speed_y_stable"), 0, 1.75));
	param_config::update_float_2(*FIGHTER_KIND_ROY, get_marked_costumes("roy","roy").into_iter().map(|x| x as i32).collect(), (smash::hash40("dive_speed_y"), 0, 2.8));
	param_config::update_float_2(*FIGHTER_KIND_ROY, get_marked_costumes("roy","roy").into_iter().map(|x| x as i32).collect(), (smash::hash40("landing_attack_air_frame_f"), 0, 9.0));
	param_config::update_float_2(*FIGHTER_KIND_ROY, get_marked_costumes("roy","roy").into_iter().map(|x| x as i32).collect(), (smash::hash40("landing_attack_air_frame_hi"), 0, 7.0));

}