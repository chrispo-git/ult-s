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

	param_config::update_float_2(*FIGHTER_KIND_YOUNGLINK, get_marked_costumes("younglink","younglink").into_iter().map(|x| x as i32).collect(), (smash::hash40("param_special_s"), smash::hash40("boomerang_stick_y"), 0.4));
	param_config::update_float_2(*FIGHTER_KIND_YOUNGLINK, get_marked_costumes("younglink","younglink").into_iter().map(|x| x as i32).collect(), (smash::hash40("param_special_s"), smash::hash40("boomerang_angle_max"), 66.0));
	param_config::update_float_2(-*FIGHTER_KIND_YOUNGLINK, get_marked_costumes("younglink","younglink").into_iter().map(|x| x as i32).collect(), (smash::hash40("param_linkbomb"), smash::hash40("linkbomb_limit_frame"), 220.0));
	param_config::update_float_2(-*FIGHTER_KIND_YOUNGLINK, get_marked_costumes("younglink","younglink").into_iter().map(|x| x as i32).collect(), (smash::hash40("param_linkbomb"), smash::hash40("linkbomb_flash_frame"), 154.0));
	param_config::update_float_2(*FIGHTER_KIND_YOUNGLINK, get_marked_costumes("younglink","younglink").into_iter().map(|x| x as i32).collect(), (smash::hash40("jump_speed_x_mul"), 0, 0.85));
	param_config::update_float_2(*FIGHTER_KIND_YOUNGLINK, get_marked_costumes("younglink","younglink").into_iter().map(|x| x as i32).collect(), (smash::hash40("air_speed_x_stable"), 0, 1.1));
	param_config::update_float_2(*FIGHTER_KIND_YOUNGLINK, get_marked_costumes("younglink","younglink").into_iter().map(|x| x as i32).collect(), (smash::hash40("air_speed_y_stable"), 0, 1.92));
	param_config::update_float_2(*FIGHTER_KIND_YOUNGLINK, get_marked_costumes("younglink","younglink").into_iter().map(|x| x as i32).collect(), (smash::hash40("dive_speed_y"), 0, 3.072));
	param_config::update_float_2(*FIGHTER_KIND_YOUNGLINK, get_marked_costumes("younglink","younglink").into_iter().map(|x| x as i32).collect(), (smash::hash40("landing_attack_air_frame_f"), 0, 10.0));
	param_config::update_float_2(*FIGHTER_KIND_YOUNGLINK, get_marked_costumes("younglink","younglink").into_iter().map(|x| x as i32).collect(), (smash::hash40("landing_attack_air_frame_b"), 0, 8.0));
	param_config::update_float_2(*FIGHTER_KIND_YOUNGLINK, get_marked_costumes("younglink","younglink").into_iter().map(|x| x as i32).collect(), (smash::hash40("landing_attack_air_frame_hi"), 0, 9.0));
	param_config::update_float_2(*FIGHTER_KIND_YOUNGLINK, get_marked_costumes("younglink","younglink").into_iter().map(|x| x as i32).collect(), (smash::hash40("scale"), 0, 1.05));
	param_config::update_float_2(*FIGHTER_KIND_YOUNGLINK, get_marked_costumes("younglink","younglink").into_iter().map(|x| x as i32).collect(), (smash::hash40("shield_radius"), 0, 10.5));

}