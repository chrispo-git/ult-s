mod status;
mod frame;
mod acmd;
use crate::util::*;
			


pub fn install() {
	frame::install();
	status::install();
	acmd::install();

	param_config::update_float_2(*FIGHTER_KIND_ROBOT, get_marked_costumes("robot","robot").into_iter().map(|x| x as i32).collect(), (smash::hash40("param_special_n"), smash::hash40("miss_shoot_energy"), 75.0));
	param_config::update_int_2(-*FIGHTER_KIND_ROBOT, get_marked_costumes("robot","robot").into_iter().map(|x| x as i32).collect(), (smash::hash40("param_beam"), smash::hash40("life"), 25));
	param_config::update_int_2(-*FIGHTER_KIND_ROBOT, get_marked_costumes("robot","robot").into_iter().map(|x| x as i32).collect(), (smash::hash40("param_beam"), smash::hash40("strong_life"), 105));
	param_config::update_float_2(-*FIGHTER_KIND_ROBOT, get_marked_costumes("robot","robot").into_iter().map(|x| x as i32).collect(), (smash::hash40("param_beam"), smash::hash40("strong_speed"), 1.6));
	param_config::update_float_2(*FIGHTER_KIND_ROBOT, get_marked_costumes("robot","robot").into_iter().map(|x| x as i32).collect(), (smash::hash40("param_special_lw"), smash::hash40("charge_max_frame"), 150.0));
	param_config::update_float_2(*FIGHTER_KIND_ROBOT, get_marked_costumes("robot","robot").into_iter().map(|x| x as i32).collect(), (smash::hash40("air_speed_x_stable"), 0, 1.02));
	param_config::update_float_2(*FIGHTER_KIND_ROBOT, get_marked_costumes("robot","robot").into_iter().map(|x| x as i32).collect(), (smash::hash40("weight"), 0, 110.0));
	param_config::update_float_2(*FIGHTER_KIND_ROBOT, get_marked_costumes("robot","robot").into_iter().map(|x| x as i32).collect(), (smash::hash40("landing_attack_air_frame_n"), 0, 10.0));
	param_config::update_float_2(*FIGHTER_KIND_ROBOT, get_marked_costumes("robot","robot").into_iter().map(|x| x as i32).collect(), (smash::hash40("landing_attack_air_frame_f"), 0, 7.0));

}