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

	 crate::param_cache::update_float_2(*FIGHTER_KIND_KAMUI, get_marked_costumes("kamui","kamui").into_iter().map(|x| x as i32).collect(), (smash::hash40("walk_speed_max"), 0, 1.4075));
	 crate::param_cache::update_float_2(*FIGHTER_KIND_KAMUI, get_marked_costumes("kamui","kamui").into_iter().map(|x| x as i32).collect(), (smash::hash40("dash_speed"), 0, 1.95));
	 crate::param_cache::update_float_2(*FIGHTER_KIND_KAMUI, get_marked_costumes("kamui","kamui").into_iter().map(|x| x as i32).collect(), (smash::hash40("run_speed_max"), 0, 1.8));
	 crate::param_cache::update_float_2(*FIGHTER_KIND_KAMUI, get_marked_costumes("kamui","kamui").into_iter().map(|x| x as i32).collect(), (smash::hash40("jump_speed_x_mul"), 0, 1.15));
	 crate::param_cache::update_float_2(*FIGHTER_KIND_KAMUI, get_marked_costumes("kamui","kamui").into_iter().map(|x| x as i32).collect(), (smash::hash40("jump_y"), 0, 30.0));
	 crate::param_cache::update_float_2(*FIGHTER_KIND_KAMUI, get_marked_costumes("kamui","kamui").into_iter().map(|x| x as i32).collect(), (smash::hash40("air_accel_x_mul"), 0, 0.06));
	 crate::param_cache::update_float_2(*FIGHTER_KIND_KAMUI, get_marked_costumes("kamui","kamui").into_iter().map(|x| x as i32).collect(), (smash::hash40("air_speed_x_stable"), 0, 1.1));
	 crate::param_cache::update_float_2(*FIGHTER_KIND_KAMUI, get_marked_costumes("kamui","kamui").into_iter().map(|x| x as i32).collect(), (smash::hash40("landing_attack_air_frame_f"), 0, 10.0));
	 crate::param_cache::update_float_2(*FIGHTER_KIND_KAMUI, get_marked_costumes("kamui","kamui").into_iter().map(|x| x as i32).collect(), (smash::hash40("landing_attack_air_frame_b"), 0, 12.0));
	 crate::param_cache::update_float_2(*FIGHTER_KIND_KAMUI, get_marked_costumes("kamui","kamui").into_iter().map(|x| x as i32).collect(), (smash::hash40("landing_attack_air_frame_hi"), 0, 11.0));
	 crate::param_cache::update_float_2(*FIGHTER_KIND_KAMUI, get_marked_costumes("kamui","kamui").into_iter().map(|x| x as i32).collect(), (smash::hash40("scale"), 0, 1.0395));
	 crate::param_cache::update_float_2(*FIGHTER_KIND_KAMUI, get_marked_costumes("kamui","kamui").into_iter().map(|x| x as i32).collect(), (smash::hash40("combo_attack_12_end"), 0, 0.0));
	 crate::param_cache::update_float_2(*FIGHTER_KIND_KAMUI, get_marked_costumes("kamui","kamui").into_iter().map(|x| x as i32).collect(), (smash::hash40("combo_attack_13_end"), 0, 0.0));
	crate::param_cache::update_int_2(*FIGHTER_KIND_KAMUI, get_marked_costumes("kamui","kamui").into_iter().map(|x| x as i32).collect(), (smash::hash40("attack_combo_max"), 0, 1));

}