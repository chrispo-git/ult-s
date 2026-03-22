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

	param_config::update_float_2(*FIGHTER_KIND_FOX, get_marked_costumes("fox","fox").into_iter().map(|x| x as i32).collect(), (smash::hash40("param_special_hi"), smash::hash40("fire_rush_speed"), 3.2));
	param_config::update_float_2(*FIGHTER_KIND_FOX, get_marked_costumes("fox","fox").into_iter().map(|x| x as i32).collect(), (smash::hash40("dash_speed"), 0, 2.37));
	param_config::update_float_2(*FIGHTER_KIND_FOX, get_marked_costumes("fox","fox").into_iter().map(|x| x as i32).collect(), (smash::hash40("run_speed_max"), 0, 2.0));
	param_config::update_float_2(*FIGHTER_KIND_FOX, get_marked_costumes("fox","fox").into_iter().map(|x| x as i32).collect(), (smash::hash40("jump_speed_x"), 0, 0.85));
	param_config::update_float_2(*FIGHTER_KIND_FOX, get_marked_costumes("fox","fox").into_iter().map(|x| x as i32).collect(), (smash::hash40("jump_speed_x_mul"), 0, 1.15));
	param_config::update_float_2(*FIGHTER_KIND_FOX, get_marked_costumes("fox","fox").into_iter().map(|x| x as i32).collect(), (smash::hash40("jump_speed_x_max"), 0, 2.5));
	param_config::update_float_2(*FIGHTER_KIND_FOX, get_marked_costumes("fox","fox").into_iter().map(|x| x as i32).collect(), (smash::hash40("air_accel_y"), 0, 0.175));
	param_config::update_float_2(*FIGHTER_KIND_FOX, get_marked_costumes("fox","fox").into_iter().map(|x| x as i32).collect(), (smash::hash40("air_speed_y_stable"), 0, 2.3));
	param_config::update_float_2(*FIGHTER_KIND_FOX, get_marked_costumes("fox","fox").into_iter().map(|x| x as i32).collect(), (smash::hash40("dive_speed_y"), 0, 4.025));
	param_config::update_float_2(*FIGHTER_KIND_FOX, get_marked_costumes("fox","fox").into_iter().map(|x| x as i32).collect(), (smash::hash40("landing_attack_air_frame_lw"), 0, 12.0));

}