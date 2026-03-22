	
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

	param_config::update_float_2(*FIGHTER_KIND_LUCAS, get_marked_costumes("lucas","lucas").into_iter().map(|x| x as i32).collect(), (smash::hash40("ground_brake"), 0, 0.101));
	param_config::update_float_2(*FIGHTER_KIND_LUCAS, get_marked_costumes("lucas","lucas").into_iter().map(|x| x as i32).collect(), (smash::hash40("dash_speed"), 0, 2.1));
	param_config::update_float_2(*FIGHTER_KIND_LUCAS, get_marked_costumes("lucas","lucas").into_iter().map(|x| x as i32).collect(), (smash::hash40("run_speed_max"), 0, 1.725));
	param_config::update_float_2(*FIGHTER_KIND_LUCAS, get_marked_costumes("lucas","lucas").into_iter().map(|x| x as i32).collect(), (smash::hash40("jump_speed_x_mul"), 0, 1.125));
	param_config::update_float_2(*FIGHTER_KIND_LUCAS, get_marked_costumes("lucas","lucas").into_iter().map(|x| x as i32).collect(), (smash::hash40("jump_aerial_speed_x_mul"), 0, 1.25));
	param_config::update_float_2(*FIGHTER_KIND_LUCAS, get_marked_costumes("lucas","lucas").into_iter().map(|x| x as i32).collect(), (smash::hash40("jump_aerial_y"), 0, 39.5));
	param_config::update_float_2(*FIGHTER_KIND_LUCAS, get_marked_costumes("lucas","lucas").into_iter().map(|x| x as i32).collect(), (smash::hash40("air_speed_x_stable"), 0, 1.05));
	param_config::update_float_2(*FIGHTER_KIND_LUCAS, get_marked_costumes("lucas","lucas").into_iter().map(|x| x as i32).collect(), (smash::hash40("air_accel_y"), 0, 0.125));
	param_config::update_float_2(*FIGHTER_KIND_LUCAS, get_marked_costumes("lucas","lucas").into_iter().map(|x| x as i32).collect(), (smash::hash40("air_speed_y_stable"), 0, 1.9));
	param_config::update_float_2(*FIGHTER_KIND_LUCAS, get_marked_costumes("lucas","lucas").into_iter().map(|x| x as i32).collect(), (smash::hash40("dive_speed_y"), 0, 3.04));
	param_config::update_float_2(*FIGHTER_KIND_LUCAS, get_marked_costumes("lucas","lucas").into_iter().map(|x| x as i32).collect(), (smash::hash40("landing_attack_air_frame_f"), 0, 9.0));

}
