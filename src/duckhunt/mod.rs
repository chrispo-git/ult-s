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

	param_config::update_float_2(*FIGHTER_KIND_DUCKHUNT, get_marked_costumes("duckhunt","duckhunt").into_iter().map(|x| x as i32).collect(), (smash::hash40("ground_brake"), 0, 0.06));
	param_config::update_float_2(*FIGHTER_KIND_DUCKHUNT, get_marked_costumes("duckhunt","duckhunt").into_iter().map(|x| x as i32).collect(), (smash::hash40("dash_speed"), 0, 1.87));
	param_config::update_float_2(*FIGHTER_KIND_DUCKHUNT, get_marked_costumes("duckhunt","duckhunt").into_iter().map(|x| x as i32).collect(), (smash::hash40("jump_speed_x_mul"), 0, 1.0));
	param_config::update_float_2(*FIGHTER_KIND_DUCKHUNT, get_marked_costumes("duckhunt","duckhunt").into_iter().map(|x| x as i32).collect(), (smash::hash40("air_speed_x_stable"), 0, 1.2));
	param_config::update_float_2(*FIGHTER_KIND_DUCKHUNT, get_marked_costumes("duckhunt","duckhunt").into_iter().map(|x| x as i32).collect(), (smash::hash40("air_speed_y_stable"), 0, 1.7));
	param_config::update_float_2(*FIGHTER_KIND_DUCKHUNT, get_marked_costumes("duckhunt","duckhunt").into_iter().map(|x| x as i32).collect(), (smash::hash40("dive_speed_y"), 0, 2.72));
	param_config::update_float_2(*FIGHTER_KIND_DUCKHUNT, get_marked_costumes("duckhunt","duckhunt").into_iter().map(|x| x as i32).collect(), (smash::hash40("landing_attack_air_frame_f"), 0, 7.0));
	param_config::update_float_2(*FIGHTER_KIND_DUCKHUNT, get_marked_costumes("duckhunt","duckhunt").into_iter().map(|x| x as i32).collect(), (smash::hash40("landing_attack_air_frame_hi"), 0, 10.0));

}