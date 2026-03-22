mod status;
mod frame;
mod acmd;
use smash::lib::lua_const::*;
use smash::hash40;
use crate::util::*;

pub fn install() {
	frame::install();
	status::install();
	acmd::install();

	param_config::update_float_2(*FIGHTER_KIND_KAMUI, get_marked_costumes("kamui","kamui").into_iter().map(|x| x as i32).collect(), (smash::hash40("walk_speed_max"), 0, 1.4075));
	param_config::update_float_2(*FIGHTER_KIND_KAMUI, get_marked_costumes("kamui","kamui").into_iter().map(|x| x as i32).collect(), (smash::hash40("dash_speed"), 0, 1.95));
	param_config::update_float_2(*FIGHTER_KIND_KAMUI, get_marked_costumes("kamui","kamui").into_iter().map(|x| x as i32).collect(), (smash::hash40("run_speed_max"), 0, 1.8));
	param_config::update_float_2(*FIGHTER_KIND_KAMUI, get_marked_costumes("kamui","kamui").into_iter().map(|x| x as i32).collect(), (smash::hash40("jump_speed_x_mul"), 0, 1.15));
	param_config::update_float_2(*FIGHTER_KIND_KAMUI, get_marked_costumes("kamui","kamui").into_iter().map(|x| x as i32).collect(), (smash::hash40("jump_y"), 0, 30.0));
	param_config::update_float_2(*FIGHTER_KIND_KAMUI, get_marked_costumes("kamui","kamui").into_iter().map(|x| x as i32).collect(), (smash::hash40("air_accel_x_mul"), 0, 0.06));
	param_config::update_float_2(*FIGHTER_KIND_KAMUI, get_marked_costumes("kamui","kamui").into_iter().map(|x| x as i32).collect(), (smash::hash40("air_speed_x_stable"), 0, 1.1));
	param_config::update_float_2(*FIGHTER_KIND_KAMUI, get_marked_costumes("kamui","kamui").into_iter().map(|x| x as i32).collect(), (smash::hash40("landing_attack_air_frame_f"), 0, 10.0));
	param_config::update_float_2(*FIGHTER_KIND_KAMUI, get_marked_costumes("kamui","kamui").into_iter().map(|x| x as i32).collect(), (smash::hash40("landing_attack_air_frame_b"), 0, 12.0));
	param_config::update_float_2(*FIGHTER_KIND_KAMUI, get_marked_costumes("kamui","kamui").into_iter().map(|x| x as i32).collect(), (smash::hash40("landing_attack_air_frame_hi"), 0, 11.0));
	param_config::update_float_2(*FIGHTER_KIND_KAMUI, get_marked_costumes("kamui","kamui").into_iter().map(|x| x as i32).collect(), (smash::hash40("scale"), 0, 1.0395));
	param_config::update_float_2(*FIGHTER_KIND_KAMUI, get_marked_costumes("kamui","kamui").into_iter().map(|x| x as i32).collect(), (smash::hash40("combo_attack_12_end"), 0, 0.0));
	param_config::update_float_2(*FIGHTER_KIND_KAMUI, get_marked_costumes("kamui","kamui").into_iter().map(|x| x as i32).collect(), (smash::hash40("combo_attack_13_end"), 0, 0.0));
	param_config::update_int_2(*FIGHTER_KIND_KAMUI, get_marked_costumes("kamui","kamui").into_iter().map(|x| x as i32).collect(), (smash::hash40("attack_combo_max"), 0, 1));

}