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

	param_config::update_float_2(-*WEAPON_KIND_PIKACHU_DENGEKI, get_marked_costumes("pikachu","pikachu").into_iter().map(|x| x as i32).collect(), (smash::hash40("param_dengeki"), smash::hash40("move_rate_"), 0.8));
	param_config::update_float_2(*FIGHTER_KIND_PIKACHU, get_marked_costumes("pikachu","pikachu").into_iter().map(|x| x as i32).collect(), (smash::hash40("param_special_lw"), smash::hash40("special_lw_offset_y_"), 7.0));
	param_config::update_float_2(-*WEAPON_KIND_PIKACHU_KAMINARI, get_marked_costumes("pikachu","pikachu").into_iter().map(|x| x as i32).collect(), (smash::hash40("param_kaminari"), smash::hash40("width_"), 2.4));
	param_config::update_float_2(*FIGHTER_KIND_PIKACHU, get_marked_costumes("pikachu","pikachu").into_iter().map(|x| x as i32).collect(), (smash::hash40("jump_speed_x"), 0, 1.0));
	param_config::update_float_2(*FIGHTER_KIND_PIKACHU, get_marked_costumes("pikachu","pikachu").into_iter().map(|x| x as i32).collect(), (smash::hash40("jump_speed_x_mul"), 0, 1.0));
	param_config::update_float_2(*FIGHTER_KIND_PIKACHU, get_marked_costumes("pikachu","pikachu").into_iter().map(|x| x as i32).collect(), (smash::hash40("jump_aerial_speed_x_mul"), 0, 1.0));
	param_config::update_float_2(*FIGHTER_KIND_PIKACHU, get_marked_costumes("pikachu","pikachu").into_iter().map(|x| x as i32).collect(), (smash::hash40("mini_jump_y"), 0, 17.12));
	param_config::update_float_2(*FIGHTER_KIND_PIKACHU, get_marked_costumes("pikachu","pikachu").into_iter().map(|x| x as i32).collect(), (smash::hash40("air_accel_x_mul"), 0, 0.105));
	param_config::update_float_2(*FIGHTER_KIND_PIKACHU, get_marked_costumes("pikachu","pikachu").into_iter().map(|x| x as i32).collect(), (smash::hash40("air_speed_x_stable"), 0, 1.2));
	param_config::update_float_2(*FIGHTER_KIND_PIKACHU, get_marked_costumes("pikachu","pikachu").into_iter().map(|x| x as i32).collect(), (smash::hash40("air_brake_x"), 0, 0.015));
	param_config::update_float_2(*FIGHTER_KIND_PIKACHU, get_marked_costumes("pikachu","pikachu").into_iter().map(|x| x as i32).collect(), (smash::hash40("air_accel_y"), 0, 0.14));
	param_config::update_float_2(*FIGHTER_KIND_PIKACHU, get_marked_costumes("pikachu","pikachu").into_iter().map(|x| x as i32).collect(), (smash::hash40("air_speed_y_stable"), 0, 1.8));
	param_config::update_float_2(*FIGHTER_KIND_PIKACHU, get_marked_costumes("pikachu","pikachu").into_iter().map(|x| x as i32).collect(), (smash::hash40("dive_speed_y"), 0, 3.2));
	param_config::update_float_2(*FIGHTER_KIND_PIKACHU, get_marked_costumes("pikachu","pikachu").into_iter().map(|x| x as i32).collect(), (smash::hash40("landing_attack_air_frame_n"), 0, 7.0));
	param_config::update_float_2(*FIGHTER_KIND_PIKACHU, get_marked_costumes("pikachu","pikachu").into_iter().map(|x| x as i32).collect(), (smash::hash40("landing_attack_air_frame_b"), 0, 10.0));
	param_config::update_float_2(*FIGHTER_KIND_PIKACHU, get_marked_costumes("pikachu","pikachu").into_iter().map(|x| x as i32).collect(), (smash::hash40("landing_attack_air_frame_hi"), 0, 8.0));

}