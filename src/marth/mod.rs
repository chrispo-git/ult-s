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

	param_config::update_float_2(*FIGHTER_KIND_MARTH, get_marked_costumes("marth","marth").into_iter().map(|x| x as i32).collect(), (smash::hash40("walk_speed_max"), 0, 1.48));
	param_config::update_float_2(*FIGHTER_KIND_MARTH, get_marked_costumes("marth","marth").into_iter().map(|x| x as i32).collect(), (smash::hash40("dash_speed"), 0, 2.175));
	param_config::update_float_2(*FIGHTER_KIND_MARTH, get_marked_costumes("marth","marth").into_iter().map(|x| x as i32).collect(), (smash::hash40("jump_speed_x"), 0, 1.0));
	param_config::update_float_2(*FIGHTER_KIND_MARTH, get_marked_costumes("marth","marth").into_iter().map(|x| x as i32).collect(), (smash::hash40("jump_speed_x_mul"), 0, 1.15));
	param_config::update_float_2(*FIGHTER_KIND_MARTH, get_marked_costumes("marth","marth").into_iter().map(|x| x as i32).collect(), (smash::hash40("jump_speed_x_max"), 0, 2.0));
	param_config::update_float_2(*FIGHTER_KIND_MARTH, get_marked_costumes("marth","marth").into_iter().map(|x| x as i32).collect(), (smash::hash40("air_speed_x_stable"), 0, 0.99));
	param_config::update_float_2(*FIGHTER_KIND_MARTH, get_marked_costumes("marth","marth").into_iter().map(|x| x as i32).collect(), (smash::hash40("air_speed_y_stable"), 0, 1.7));
	param_config::update_float_2(*FIGHTER_KIND_MARTH, get_marked_costumes("marth","marth").into_iter().map(|x| x as i32).collect(), (smash::hash40("weight"), 0, 85.0));
	param_config::update_float_2(*FIGHTER_KIND_MARTH, get_marked_costumes("marth","marth").into_iter().map(|x| x as i32).collect(), (smash::hash40("landing_attack_air_frame_n"), 0, 9.0));
	param_config::update_float_2(*FIGHTER_KIND_MARTH, get_marked_costumes("marth","marth").into_iter().map(|x| x as i32).collect(), (smash::hash40("landing_attack_air_frame_f"), 0, 12.0));
	param_config::update_float_2(*FIGHTER_KIND_MARTH, get_marked_costumes("marth","marth").into_iter().map(|x| x as i32).collect(), (smash::hash40("landing_frame"), 0, 3.0));
	param_config::update_float_2(*FIGHTER_KIND_MARTH, get_marked_costumes("marth","marth").into_iter().map(|x| x as i32).collect(), (smash::hash40("combo_attack_12_end"), 0, 0.0));
	param_config::update_int_2(*FIGHTER_KIND_MARTH, get_marked_costumes("marth","marth").into_iter().map(|x| x as i32).collect(), (smash::hash40("attack_combo_max"), 0, 1));

}
