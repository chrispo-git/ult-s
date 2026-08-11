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

	 crate::param_cache::update_float_2(*FIGHTER_KIND_EFLAME, get_marked_costumes("eflame","eflame").into_iter().map(|x| x as i32).collect(), (smash::hash40("walk_accel_add"), 0, 0.5));
	 crate::param_cache::update_float_2(*FIGHTER_KIND_EFLAME, get_marked_costumes("eflame","eflame").into_iter().map(|x| x as i32).collect(), (smash::hash40("walk_speed_max"), 0, 1.25));
	 crate::param_cache::update_float_2(*FIGHTER_KIND_EFLAME, get_marked_costumes("eflame","eflame").into_iter().map(|x| x as i32).collect(), (smash::hash40("ground_brake"), 0, 0.075));
	 crate::param_cache::update_float_2(*FIGHTER_KIND_EFLAME, get_marked_costumes("eflame","eflame").into_iter().map(|x| x as i32).collect(), (smash::hash40("run_speed_max"), 0, 1.5));
	 crate::param_cache::update_float_2(*FIGHTER_KIND_EFLAME, get_marked_costumes("eflame","eflame").into_iter().map(|x| x as i32).collect(), (smash::hash40("jump_speed_x_mul"), 0, 1.15));
	 crate::param_cache::update_float_2(*FIGHTER_KIND_EFLAME, get_marked_costumes("eflame","eflame").into_iter().map(|x| x as i32).collect(), (smash::hash40("jump_aerial_y"), 0, 32.0));
	 crate::param_cache::update_float_2(*FIGHTER_KIND_EFLAME, get_marked_costumes("eflame","eflame").into_iter().map(|x| x as i32).collect(), (smash::hash40("air_speed_x_stable"), 0, 1.14));
	 crate::param_cache::update_float_2(*FIGHTER_KIND_EFLAME, get_marked_costumes("eflame","eflame").into_iter().map(|x| x as i32).collect(), (smash::hash40("weight"), 0, 97.0));
	 crate::param_cache::update_float_2(*FIGHTER_KIND_EFLAME, get_marked_costumes("eflame","eflame").into_iter().map(|x| x as i32).collect(), (smash::hash40("landing_attack_air_frame_n"), 0, 10.0));
	 crate::param_cache::update_float_2(*FIGHTER_KIND_EFLAME, get_marked_costumes("eflame","eflame").into_iter().map(|x| x as i32).collect(), (smash::hash40("landing_attack_air_frame_f"), 0, 15.0));

}