mod status;
mod frame;
mod acmd;
use crate::util::*;
use smash::lib::lua_const::*;
use smash::hash40;

static mut IS_THUNDER : [bool; 8] = [false; 8];


pub fn install() {
	frame::install();
	status::install();
	acmd::install();

	param_config::update_int_2(*FIGHTER_KIND_MASTER, get_marked_costumes("master","master").into_iter().map(|x| x as i32).collect(), (smash::hash40("param_special_n"), smash::hash40("cancel_start_frame"), 300));
	param_config::update_int_2(*FIGHTER_KIND_MASTER, get_marked_costumes("master","master").into_iter().map(|x| x as i32).collect(), (smash::hash40("param_special_n"), smash::hash40("cancel_end_frame"), 350));
	param_config::update_float_2(*FIGHTER_KIND_MASTER, get_marked_costumes("master","master").into_iter().map(|x| x as i32).collect(), (smash::hash40("param_special_n"), smash::hash40("arrow_offset_x"), 4.0));
	param_config::update_int_2(-*FIGHTER_KIND_MASTER, get_marked_costumes("master","master").into_iter().map(|x| x as i32).collect(), (smash::hash40("param_arrow1"), smash::hash40("life"), 10));
	param_config::update_int_2(-*FIGHTER_KIND_MASTER, get_marked_costumes("master","master").into_iter().map(|x| x as i32).collect(), (smash::hash40("param_arrow1"), smash::hash40("vanish_frame"), 8));
	param_config::update_float_2(*FIGHTER_KIND_MASTER, get_marked_costumes("master","master").into_iter().map(|x| x as i32).collect(), (smash::hash40("param_special_hi"), smash::hash40("overtake_attack_hp_rate"), 75.0));
	param_config::update_float_2(*FIGHTER_KIND_MASTER, get_marked_costumes("master","master").into_iter().map(|x| x as i32).collect(), (smash::hash40("param_special_hi"), smash::hash40("overtake_attack_damage"), 75.0));
	param_config::update_float_2(*FIGHTER_KIND_MASTER, get_marked_costumes("master","master").into_iter().map(|x| x as i32).collect(), (smash::hash40("walk_speed_max"), 0, 1.05));
	param_config::update_float_2(*FIGHTER_KIND_MASTER, get_marked_costumes("master","master").into_iter().map(|x| x as i32).collect(), (smash::hash40("dash_speed"), 0, 2.2));
	param_config::update_float_2(*FIGHTER_KIND_MASTER, get_marked_costumes("master","master").into_iter().map(|x| x as i32).collect(), (smash::hash40("run_speed_max"), 0, 1.9));
	param_config::update_float_2(*FIGHTER_KIND_MASTER, get_marked_costumes("master","master").into_iter().map(|x| x as i32).collect(), (smash::hash40("jump_speed_x_mul"), 0, 1.125));
	param_config::update_float_2(*FIGHTER_KIND_MASTER, get_marked_costumes("master","master").into_iter().map(|x| x as i32).collect(), (smash::hash40("air_speed_x_stable"), 0, 0.99));
	param_config::update_float_2(*FIGHTER_KIND_MASTER, get_marked_costumes("master","master").into_iter().map(|x| x as i32).collect(), (smash::hash40("landing_attack_air_frame_n"), 0, 9.0));
	param_config::update_float_2(*FIGHTER_KIND_MASTER, get_marked_costumes("master","master").into_iter().map(|x| x as i32).collect(), (smash::hash40("landing_attack_air_frame_f"), 0, 9.0));
	param_config::update_float_2(*FIGHTER_KIND_MASTER, get_marked_costumes("master","master").into_iter().map(|x| x as i32).collect(), (smash::hash40("landing_attack_air_frame_b"), 0, 11.0));
	param_config::update_float_2(*FIGHTER_KIND_MASTER, get_marked_costumes("master","master").into_iter().map(|x| x as i32).collect(), (smash::hash40("landing_attack_air_frame_hi"), 0, 11.0));
	param_config::update_float_2(*FIGHTER_KIND_MASTER, get_marked_costumes("master","master").into_iter().map(|x| x as i32).collect(), (smash::hash40("landing_attack_air_frame_lw"), 0, 20.0));

}

