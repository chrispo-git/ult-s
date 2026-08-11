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
	crate::param_cache::update_int_2(*FIGHTER_KIND_BAYONETTA, get_marked_costumes("bayonetta","bayonetta").into_iter().map(|x| x as i32).collect(), (smash::hash40("param_special_hi"), smash::hash40("landing_frame"), 15));
	crate::param_cache::update_int_2(*FIGHTER_KIND_BAYONETTA, get_marked_costumes("bayonetta","bayonetta").into_iter().map(|x| x as i32).collect(), (smash::hash40("param_special_hi"), smash::hash40("act_landing_frame"), 19));
	crate::param_cache::update_int_2(*FIGHTER_KIND_BAYONETTA, get_marked_costumes("bayonetta","bayonetta").into_iter().map(|x| x as i32).collect(), (smash::hash40("param_special_hi"), smash::hash40("landing_frame_2nd"), 23));
	crate::param_cache::update_int_2(*FIGHTER_KIND_BAYONETTA, get_marked_costumes("bayonetta","bayonetta").into_iter().map(|x| x as i32).collect(), (smash::hash40("param_special_hi"), smash::hash40("act_landing_frame_2nd"), 26));
	 crate::param_cache::update_float_2(*FIGHTER_KIND_BAYONETTA, get_marked_costumes("bayonetta","bayonetta").into_iter().map(|x| x as i32).collect(), (smash::hash40("param_special_lw"), smash::hash40("base_slow_frame_max"), 120.0));
	 crate::param_cache::update_float_2(*FIGHTER_KIND_BAYONETTA, get_marked_costumes("bayonetta","bayonetta").into_iter().map(|x| x as i32).collect(), (smash::hash40("jump_speed_x_mul"), 0, 1.5));
}