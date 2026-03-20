mod status;
mod frame;
mod acmd;
use crate::util::*;
use smash::lib::lua_const::*;
use smash::hash40;
		
static mut WAS_AIR : [bool; 8] = [false; 8];	


pub fn install() {
	frame::install();
	status::install();
	acmd::install();

	param_config::update_float_2(-*FIGHTER_KIND_RICHTER, get_marked_costumes("richter","richter").into_iter().map(|x| x as i32).collect(), (smash::hash40("param_axe"), smash::hash40("gravity"), 0.0));
	param_config::update_float_2(-*FIGHTER_KIND_RICHTER, get_marked_costumes("richter","richter").into_iter().map(|x| x as i32).collect(), (smash::hash40("param_axe"), smash::hash40("throw_angle"), 0.0));
	param_config::update_float_2(-*FIGHTER_KIND_RICHTER, get_marked_costumes("richter","richter").into_iter().map(|x| x as i32).collect(), (smash::hash40("param_axe"), smash::hash40("throw_speed"), 4.0));
	param_config::update_float_2(-*FIGHTER_KIND_RICHTER, get_marked_costumes("richter","richter").into_iter().map(|x| x as i32).collect(), (smash::hash40("param_axe"), smash::hash40("brake_x"), 0.0));
	param_config::update_float_2(-*FIGHTER_KIND_RICHTER, get_marked_costumes("richter","richter").into_iter().map(|x| x as i32).collect(), (smash::hash40("param_axe"), smash::hash40("rot_speed"), 0.0));
	param_config::update_int_2(-*FIGHTER_KIND_RICHTER, get_marked_costumes("richter","richter").into_iter().map(|x| x as i32).collect(), (smash::hash40("param_axe"), smash::hash40("life"), 45));
	param_config::update_float_2(-*FIGHTER_KIND_RICHTER, get_marked_costumes("richter","richter").into_iter().map(|x| x as i32).collect(), (smash::hash40("param_axe"), smash::hash40("throw_angle_stick_back"), 0.0));
	param_config::update_float_2(-*FIGHTER_KIND_RICHTER, get_marked_costumes("richter","richter").into_iter().map(|x| x as i32).collect(), (smash::hash40("param_axe"), smash::hash40("throw_angle_stick_front"), 0.0));
	param_config::update_float_2(-*FIGHTER_KIND_RICHTER, get_marked_costumes("richter","richter").into_iter().map(|x| x as i32).collect(), (smash::hash40("param_axe"), smash::hash40("throw_speed_stick_back"), 4.0));
	param_config::update_float_2(*FIGHTER_KIND_RICHTER, get_marked_costumes("richter","richter").into_iter().map(|x| x as i32).collect(), (smash::hash40("param_special_hi"), smash::hash40("start_y_mul_air"), 1.1));
	param_config::update_float_2(*FIGHTER_KIND_RICHTER, get_marked_costumes("richter","richter").into_iter().map(|x| x as i32).collect(), (smash::hash40("param_special_hi"), smash::hash40("limit_speed_y_air"), 2.1));
	param_config::update_float_2(*FIGHTER_KIND_RICHTER, get_marked_costumes("richter","richter").into_iter().map(|x| x as i32).collect(), (smash::hash40("param_special_lw"), smash::hash40("throw_speed_ground"), 1.25));
	param_config::update_float_2(*FIGHTER_KIND_RICHTER, get_marked_costumes("richter","richter").into_iter().map(|x| x as i32).collect(), (smash::hash40("param_special_lw"), smash::hash40("throw_speed_air"), 1.25));
	param_config::update_int_2(-*FIGHTER_KIND_RICHTER, get_marked_costumes("richter","richter").into_iter().map(|x| x as i32).collect(), (smash::hash40("param_holywater"), smash::hash40("fire_pillar_life_frame"), 45));
	param_config::update_float_2(*FIGHTER_KIND_RICHTER, get_marked_costumes("richter","richter").into_iter().map(|x| x as i32).collect(), (smash::hash40("dash_speed"), 0, 1.98));
	param_config::update_float_2(*FIGHTER_KIND_RICHTER, get_marked_costumes("richter","richter").into_iter().map(|x| x as i32).collect(), (smash::hash40("run_speed_max"), 0, 1.87));
	param_config::update_float_2(*FIGHTER_KIND_RICHTER, get_marked_costumes("richter","richter").into_iter().map(|x| x as i32).collect(), (smash::hash40("air_accel_x_mul"), 0, 0.05));
	param_config::update_float_2(*FIGHTER_KIND_RICHTER, get_marked_costumes("richter","richter").into_iter().map(|x| x as i32).collect(), (smash::hash40("air_speed_x_stable"), 0, 1.08));
	param_config::update_float_2(*FIGHTER_KIND_RICHTER, get_marked_costumes("richter","richter").into_iter().map(|x| x as i32).collect(), (smash::hash40("landing_attack_air_frame_n"), 0, 7.0));
	param_config::update_float_2(*FIGHTER_KIND_RICHTER, get_marked_costumes("richter","richter").into_iter().map(|x| x as i32).collect(), (smash::hash40("landing_attack_air_frame_f"), 0, 11.0));
	param_config::update_float_2(*FIGHTER_KIND_RICHTER, get_marked_costumes("richter","richter").into_iter().map(|x| x as i32).collect(), (smash::hash40("landing_attack_air_frame_b"), 0, 13.0));
	param_config::update_float_2(*FIGHTER_KIND_RICHTER, get_marked_costumes("richter","richter").into_iter().map(|x| x as i32).collect(), (smash::hash40("landing_attack_air_frame_hi"), 0, 6.0));
	param_config::update_float_2(*FIGHTER_KIND_RICHTER, get_marked_costumes("richter","richter").into_iter().map(|x| x as i32).collect(), (smash::hash40("shield_radius"), 0, 13.0));
	param_config::update_float_2(*FIGHTER_KIND_RICHTER, get_marked_costumes("richter","richter").into_iter().map(|x| x as i32).collect(), (smash::hash40("mini_jump_y"), 0, 14.0));

}