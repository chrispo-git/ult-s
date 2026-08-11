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

	 crate::param_cache::update_float_2(*FIGHTER_KIND_ELIGHT, get_marked_costumes("elight","elight").into_iter().map(|x| x as i32).collect(), (smash::hash40("param_private"), smash::hash40("foresight_target_slow_distance"), 30.0));
	 crate::param_cache::update_float_2(*FIGHTER_KIND_ELIGHT, get_marked_costumes("elight","elight").into_iter().map(|x| x as i32).collect(), (smash::hash40("ground_brake"), 0, 0.14));
	 crate::param_cache::update_float_2(*FIGHTER_KIND_ELIGHT, get_marked_costumes("elight","elight").into_iter().map(|x| x as i32).collect(), (smash::hash40("dash_speed"), 0, 2.3));
	 crate::param_cache::update_float_2(*FIGHTER_KIND_ELIGHT, get_marked_costumes("elight","elight").into_iter().map(|x| x as i32).collect(), (smash::hash40("run_speed_max"), 0, 2.2));
	 crate::param_cache::update_float_2(*FIGHTER_KIND_ELIGHT, get_marked_costumes("elight","elight").into_iter().map(|x| x as i32).collect(), (smash::hash40("jump_speed_x_mul"), 0, 1.2));
	 crate::param_cache::update_float_2(*FIGHTER_KIND_ELIGHT, get_marked_costumes("elight","elight").into_iter().map(|x| x as i32).collect(), (smash::hash40("jump_speed_x_max"), 0, 1.5));
	 crate::param_cache::update_float_2(*FIGHTER_KIND_ELIGHT, get_marked_costumes("elight","elight").into_iter().map(|x| x as i32).collect(), (smash::hash40("jump_aerial_y"), 0, 27.5));
	 crate::param_cache::update_float_2(*FIGHTER_KIND_ELIGHT, get_marked_costumes("elight","elight").into_iter().map(|x| x as i32).collect(), (smash::hash40("weight"), 0, 85.0));
	 crate::param_cache::update_float_2(*FIGHTER_KIND_ELIGHT, get_marked_costumes("elight","elight").into_iter().map(|x| x as i32).collect(), (smash::hash40("landing_attack_air_frame_lw"), 0, 8.0));

}