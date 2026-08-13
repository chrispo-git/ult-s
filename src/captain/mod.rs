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

	 crate::param_cache::update_float_2(*FIGHTER_KIND_CAPTAIN, get_marked_costumes("captain","captain").into_iter().map(|x| x as i32).collect(), (smash::hash40("dash_speed"), 0, 2.35));
	 crate::param_cache::update_float_2(*FIGHTER_KIND_CAPTAIN, get_marked_costumes("captain","captain").into_iter().map(|x| x as i32).collect(), (smash::hash40("jump_speed_x"), 0, 1.2));
	 crate::param_cache::update_float_2(*FIGHTER_KIND_CAPTAIN, get_marked_costumes("captain","captain").into_iter().map(|x| x as i32).collect(), (smash::hash40("jump_speed_x_max"), 0, 2.8));
	 crate::param_cache::update_float_2(*FIGHTER_KIND_CAPTAIN, get_marked_costumes("captain","captain").into_iter().map(|x| x as i32).collect(), (smash::hash40("mini_jump_y"), 0, 15.25));
	 crate::param_cache::update_float_2(*FIGHTER_KIND_CAPTAIN, get_marked_costumes("captain","captain").into_iter().map(|x| x as i32).collect(), (smash::hash40("air_speed_x_stable"), 0, 1.28));
	 crate::param_cache::update_float_2(*FIGHTER_KIND_CAPTAIN, get_marked_costumes("captain","captain").into_iter().map(|x| x as i32).collect(), (smash::hash40("air_speed_y_stable"), 0, 1.92));
	 crate::param_cache::update_float_2(*FIGHTER_KIND_CAPTAIN, get_marked_costumes("captain","captain").into_iter().map(|x| x as i32).collect(), (smash::hash40("dive_speed_y"), 0, 3.072));
	 crate::param_cache::update_float_2(*FIGHTER_KIND_CAPTAIN, get_marked_costumes("captain","captain").into_iter().map(|x| x as i32).collect(), (smash::hash40("landing_attack_air_frame_f"), 0, 14.0));
	 crate::param_cache::update_float_2(*FIGHTER_KIND_CAPTAIN, get_marked_costumes("captain","captain").into_iter().map(|x| x as i32).collect(), (smash::hash40("landing_attack_air_frame_b"), 0, 8.0));
	 crate::param_cache::update_float_2(*FIGHTER_KIND_CAPTAIN, get_marked_costumes("captain","captain").into_iter().map(|x| x as i32).collect(), (smash::hash40("landing_attack_air_frame_hi"), 0, 6.0));

}