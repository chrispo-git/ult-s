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

	param_config::update_float_2(*FIGHTER_KIND_CLOUD, get_marked_costumes("cloud","cloud").into_iter().map(|x| x as i32).collect(), (smash::hash40("walk_accel_mul"), 0, 0.15));
	param_config::update_float_2(*FIGHTER_KIND_CLOUD, get_marked_costumes("cloud","cloud").into_iter().map(|x| x as i32).collect(), (smash::hash40("walk_slow_speed_mul"), 0, 0.253));
	param_config::update_float_2(*FIGHTER_KIND_CLOUD, get_marked_costumes("cloud","cloud").into_iter().map(|x| x as i32).collect(), (smash::hash40("walk_middle_ratio"), 0, 0.621));
	param_config::update_float_2(*FIGHTER_KIND_CLOUD, get_marked_costumes("cloud","cloud").into_iter().map(|x| x as i32).collect(), (smash::hash40("walk_fast_ratio"), 0, 0.978));
	param_config::update_float_2(*FIGHTER_KIND_CLOUD, get_marked_costumes("cloud","cloud").into_iter().map(|x| x as i32).collect(), (smash::hash40("jump_speed_x_mul"), 0, 1.175));
	param_config::update_float_2(*FIGHTER_KIND_CLOUD, get_marked_costumes("cloud","cloud").into_iter().map(|x| x as i32).collect(), (smash::hash40("jump_initial_y"), 0, 12.496));
	param_config::update_float_2(*FIGHTER_KIND_CLOUD, get_marked_costumes("cloud","cloud").into_iter().map(|x| x as i32).collect(), (smash::hash40("jump_y"), 0, 32.69));
	param_config::update_float_2(*FIGHTER_KIND_CLOUD, get_marked_costumes("cloud","cloud").into_iter().map(|x| x as i32).collect(), (smash::hash40("jump_aerial_y"), 0, 32.69));
	param_config::update_float_2(*FIGHTER_KIND_CLOUD, get_marked_costumes("cloud","cloud").into_iter().map(|x| x as i32).collect(), (smash::hash40("damage_fly_top_air_accel_y"), 0, 0.076608));
	param_config::update_float_2(*FIGHTER_KIND_CLOUD, get_marked_costumes("cloud","cloud").into_iter().map(|x| x as i32).collect(), (smash::hash40("landing_attack_air_frame_b"), 0, 10.0));
	param_config::update_float_2(*FIGHTER_KIND_CLOUD, get_marked_costumes("cloud","cloud").into_iter().map(|x| x as i32).collect(), (smash::hash40("landing_attack_air_frame_lw"), 0, 21.0));
	param_config::update_float_2(*FIGHTER_KIND_CLOUD, get_marked_costumes("cloud","cloud").into_iter().map(|x| x as i32).collect(), (smash::hash40("tread_mini_jump_speed_y_mul"), 0, 0.35));
}