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

	crate::param_cache::update_float_2(
		*FIGHTER_KIND_KOOPAJR, 
		get_marked_costumes("koopajr","koopajr").into_iter().map(|x| x as i32).collect(), 
		(smash::hash40("dash_speed"), 0, 1.7)
	);
	crate::param_cache::update_float_2(
		*FIGHTER_KIND_KOOPAJR, 
		get_marked_costumes("koopajr","koopajr").into_iter().map(|x| x as i32).collect(), 
		(smash::hash40("run_speed_max"), 0, 1.65)
	);
	crate::param_cache::update_float_2(
		*FIGHTER_KIND_KOOPAJR, 
		get_marked_costumes("koopajr","koopajr").into_iter().map(|x| x as i32).collect(), 
		(smash::hash40("walk_speed_max"), 0, 1.575)
	);
	crate::param_cache::update_float_2(
		*FIGHTER_KIND_KOOPAJR, 
		get_marked_costumes("koopajr","koopajr").into_iter().map(|x| x as i32).collect(), 
		(smash::hash40("air_speed_x_stable"), 0, 1.25)
	);
	crate::param_cache::update_float_2(
		*FIGHTER_KIND_KOOPAJR, 
		get_marked_costumes("koopajr","koopajr").into_iter().map(|x| x as i32).collect(), 
		(smash::hash40("air_accel_x_add"), 0, 0.04)
	);
	crate::param_cache::update_float_2(
		*FIGHTER_KIND_KOOPAJR, 
		get_marked_costumes("koopajr","koopajr").into_iter().map(|x| x as i32).collect(), 
		(smash::hash40("air_accel_x_mul"), 0, 0.09)
	);
	crate::param_cache::update_float_2(
		*FIGHTER_KIND_KOOPAJR, 
		get_marked_costumes("koopajr","koopajr").into_iter().map(|x| x as i32).collect(), 
		(smash::hash40("jump_y"), 0, 33.0)
	);
	crate::param_cache::update_float_2(
		*FIGHTER_KIND_KOOPAJR, 
		get_marked_costumes("koopajr","koopajr").into_iter().map(|x| x as i32).collect(), 
		(smash::hash40("jump_initial_y"), 0, 13.0)
	);
	crate::param_cache::update_float_2(
		*FIGHTER_KIND_KOOPAJR, 
		get_marked_costumes("koopajr","koopajr").into_iter().map(|x| x as i32).collect(), 
		(smash::hash40("mini_jump_y"), 0, 14.0)
	);
	crate::param_cache::update_float_2(
		*FIGHTER_KIND_KOOPAJR, 
		get_marked_costumes("koopajr","koopajr").into_iter().map(|x| x as i32).collect(), 
		(smash::hash40("jump_aerial_y"), 0, 16.0)
	);
	crate::param_cache::update_float_2(
		*FIGHTER_KIND_KOOPAJR, 
		get_marked_costumes("koopajr","koopajr").into_iter().map(|x| x as i32).collect(), 
		(smash::hash40("air_speed_y_stable"), 0, 1.35)
	);
	crate::param_cache::update_float_2(
		*FIGHTER_KIND_KOOPAJR, 
		get_marked_costumes("koopajr","koopajr").into_iter().map(|x| x as i32).collect(), 
		(smash::hash40("dive_speed_y"), 0, 2.16)
	);
	crate::param_cache::update_float_2(
		*FIGHTER_KIND_KOOPAJR, 
		get_marked_costumes("koopajr","koopajr").into_iter().map(|x| x as i32).collect(), 
		(smash::hash40("air_accel_y"), 0, 0.07)
	);
	crate::param_cache::update_float_2(
		*FIGHTER_KIND_KOOPAJR, 
		get_marked_costumes("koopajr","koopajr").into_iter().map(|x| x as i32).collect(), 
		(smash::hash40("ground_brake"), 0, 0.1)
	);
	crate::param_cache::update_float_2(
		*FIGHTER_KIND_KOOPAJR, 
		get_marked_costumes("koopajr","koopajr").into_iter().map(|x| x as i32).collect(), 
		(smash::hash40("weight"), 0, 96.0)
	);
	crate::param_cache::update_float_2(
		*FIGHTER_KIND_KOOPAJR, 
		get_marked_costumes("koopajr","koopajr").into_iter().map(|x| x as i32).collect(), 
		(smash::hash40("landing_attack_air_frame_n"), 0, 9.0)
	);
	crate::param_cache::update_float_2(
		*FIGHTER_KIND_KOOPAJR, 
		get_marked_costumes("koopajr","koopajr").into_iter().map(|x| x as i32).collect(), 
		(smash::hash40("landing_attack_air_frame_f"), 0, 9.0)
	);
	crate::param_cache::update_float_2(
		*FIGHTER_KIND_KOOPAJR, 
		get_marked_costumes("koopajr","koopajr").into_iter().map(|x| x as i32).collect(), 
		(smash::hash40("landing_attack_air_frame_b"), 0, 11.0)
	);
	crate::param_cache::update_float_2(
		*FIGHTER_KIND_KOOPAJR, 
		get_marked_costumes("koopajr","koopajr").into_iter().map(|x| x as i32).collect(), 
		(smash::hash40("landing_attack_air_frame_hi"), 0, 10.0)
	);
	crate::param_cache::update_float_2(
		*FIGHTER_KIND_KOOPAJR, 
		get_marked_costumes("koopajr","koopajr").into_iter().map(|x| x as i32).collect(), 
		(smash::hash40("landing_attack_air_frame_lw"), 0, 12.0)
	);
	crate::param_cache::update_int_2(
		*FIGHTER_KIND_KOOPAJR, 
		get_marked_costumes("koopajr","koopajr").into_iter().map(|x| x as i32).collect(), 
		(smash::hash40("jump_count_max"), 0, 5)
	);
	crate::param_cache::update_float_2(
		*FIGHTER_KIND_KOOPAJR, 
		get_marked_costumes("koopajr","koopajr").into_iter().map(|x| x as i32).collect(), 
		(smash::hash40("param_special_s"), smash::hash40("dash_no_damage_power"), 1.0)
	);
	crate::param_cache::update_int_2(
		*FIGHTER_KIND_KOOPAJR, 
		get_marked_costumes("koopajr","koopajr").into_iter().map(|x| x as i32).collect(), 
		(smash::hash40("param_special_s"), smash::hash40("dash_frame"), 60)
	);
	crate::param_cache::update_float_2(
		*FIGHTER_KIND_KOOPAJR, 
		get_marked_costumes("koopajr","koopajr").into_iter().map(|x| x as i32).collect(), 
		(smash::hash40("param_special_s"), smash::hash40("dash_brake_ground"), 0.01)
	);
	crate::param_cache::update_float_2(
		*FIGHTER_KIND_KOOPAJR, 
		get_marked_costumes("koopajr","koopajr").into_iter().map(|x| x as i32).collect(), 
		(smash::hash40("param_special_s"), smash::hash40("dash_brake_air"), 0.0125)
	);
	crate::param_cache::update_float_2(
		*FIGHTER_KIND_KOOPAJR, 
		get_marked_costumes("koopajr","koopajr").into_iter().map(|x| x as i32).collect(), 
		(smash::hash40("param_special_hi"), smash::hash40("shoot_speed_y"), 1.85)
	);
	crate::param_cache::update_float_2(
		*FIGHTER_KIND_KOOPAJR, 
		get_marked_costumes("koopajr","koopajr").into_iter().map(|x| x as i32).collect(), 
		(smash::hash40("param_special_hi"), smash::hash40("landing_frame"), 33.0)
	);
	crate::param_cache::update_float_2(
		*FIGHTER_KIND_KOOPAJR, 
		get_marked_costumes("koopajr","koopajr").into_iter().map(|x| x as i32).collect(), 
		(smash::hash40("param_special_hi"), smash::hash40("dive_landing_frame"), 33.0)
	);
	crate::param_cache::update_float_2(
		*FIGHTER_KIND_KOOPAJR, 
		get_marked_costumes("koopajr","koopajr").into_iter().map(|x| x as i32).collect(), 
		(smash::hash40("param_special_hi"), smash::hash40("fall_air_accel_y_mul"), 1.0)
	);
	crate::param_cache::update_float_2(
		*FIGHTER_KIND_KOOPAJR, 
		get_marked_costumes("koopajr","koopajr").into_iter().map(|x| x as i32).collect(), 
		(smash::hash40("param_special_hi"), smash::hash40("fall_air_speed_y_stable_mul"), 1.0)
	);
}