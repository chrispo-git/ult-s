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

	param_config::update_int_2(*FIGHTER_KIND_DAISY, get_marked_costumes("daisy","daisy").into_iter().map(|x| x as i32).collect(), (smash::hash40("param_special_n"), smash::hash40("special_n_hit_stop_frame"), 2));
	param_config::update_int_2(*FIGHTER_KIND_DAISY, get_marked_costumes("daisy","daisy").into_iter().map(|x| x as i32).collect(), (smash::hash40("param_private"), smash::hash40("uniq_float_float_frame"), 80));
	param_config::update_float_2(*FIGHTER_KIND_DAISY, get_marked_costumes("daisy","daisy").into_iter().map(|x| x as i32).collect(), (smash::hash40("run_speed_max"), 0, 1.625));
	param_config::update_float_2(*FIGHTER_KIND_DAISY, get_marked_costumes("daisy","daisy").into_iter().map(|x| x as i32).collect(), (smash::hash40("jump_y"), 0, 28.0));
	param_config::update_float_2(*FIGHTER_KIND_DAISY, get_marked_costumes("daisy","daisy").into_iter().map(|x| x as i32).collect(), (smash::hash40("mini_jump_y"), 0, 13.5));
	param_config::update_float_2(*FIGHTER_KIND_DAISY, get_marked_costumes("daisy","daisy").into_iter().map(|x| x as i32).collect(), (smash::hash40("jump_aerial_y"), 0, 28.0));
	param_config::update_float_2(*FIGHTER_KIND_DAISY, get_marked_costumes("daisy","daisy").into_iter().map(|x| x as i32).collect(), (smash::hash40("air_accel_x_mul"), 0, 0.15));
	param_config::update_float_2(*FIGHTER_KIND_DAISY, get_marked_costumes("daisy","daisy").into_iter().map(|x| x as i32).collect(), (smash::hash40("air_speed_y_stable"), 0, 1.73));
	param_config::update_float_2(*FIGHTER_KIND_DAISY, get_marked_costumes("daisy","daisy").into_iter().map(|x| x as i32).collect(), (smash::hash40("dive_speed_y"), 0, 2.752));
	param_config::update_float_2(*FIGHTER_KIND_DAISY, get_marked_costumes("daisy","daisy").into_iter().map(|x| x as i32).collect(), (smash::hash40("weight"), 0, 83.0));
	param_config::update_float_2(*FIGHTER_KIND_DAISY, get_marked_costumes("daisy","daisy").into_iter().map(|x| x as i32).collect(), (smash::hash40("landing_attack_air_frame_n"), 0, 11.0));
	param_config::update_float_2(*FIGHTER_KIND_DAISY, get_marked_costumes("daisy","daisy").into_iter().map(|x| x as i32).collect(), (smash::hash40("landing_attack_air_frame_b"), 0, 9.0));
	
	param_config::update_float_2(-*FIGHTER_KIND_DAISY, get_marked_costumes("daisy","daisy").into_iter().map(|x| x as i32).collect(), (smash::hash40("param_daisydaikon"), smash::hash40("daikon_throw_speed_mul"), 1.0));
	param_config::update_float_2(-*FIGHTER_KIND_DAISY, get_marked_costumes("daisy","daisy").into_iter().map(|x| x as i32).collect(), (smash::hash40("param_daisydaikon"), smash::hash40("daikon_life_frame"), 200.0));
	param_config::update_float_2(-*FIGHTER_KIND_DAISY, get_marked_costumes("daisy","daisy").into_iter().map(|x| x as i32).collect(), (smash::hash40("param_daisydaikon"), smash::hash40("daikon_1_power"), 3.0));
	param_config::update_float_2(-*FIGHTER_KIND_DAISY, get_marked_costumes("daisy","daisy").into_iter().map(|x| x as i32).collect(), (smash::hash40("param_daisydaikon"), smash::hash40("daikon_2_power"), 3.0));
	param_config::update_float_2(-*FIGHTER_KIND_DAISY, get_marked_costumes("daisy","daisy").into_iter().map(|x| x as i32).collect(), (smash::hash40("param_daisydaikon"), smash::hash40("daikon_3_power"), 3.0));
	param_config::update_float_2(-*FIGHTER_KIND_DAISY, get_marked_costumes("daisy","daisy").into_iter().map(|x| x as i32).collect(), (smash::hash40("param_daisydaikon"), smash::hash40("daikon_4_power"), 3.0));
	param_config::update_float_2(-*FIGHTER_KIND_DAISY, get_marked_costumes("daisy","daisy").into_iter().map(|x| x as i32).collect(), (smash::hash40("param_daisydaikon"), smash::hash40("daikon_5_power"), 3.0));
	param_config::update_float_2(-*FIGHTER_KIND_DAISY, get_marked_costumes("daisy","daisy").into_iter().map(|x| x as i32).collect(), (smash::hash40("param_daisydaikon"), smash::hash40("daikon_6_power"), 3.0));
	param_config::update_float_2(-*FIGHTER_KIND_DAISY, get_marked_costumes("daisy","daisy").into_iter().map(|x| x as i32).collect(), (smash::hash40("param_daisydaikon"), smash::hash40("daikon_7_power"), 3.0));
	param_config::update_float_2(-*FIGHTER_KIND_DAISY, get_marked_costumes("daisy","daisy").into_iter().map(|x| x as i32).collect(), (smash::hash40("param_daisydaikon"), smash::hash40("daikon_8_power"), 3.0));

}