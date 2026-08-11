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


	 crate::param_cache::update_float_2(*FIGHTER_KIND_PURIN, get_marked_costumes("purin","purin").into_iter().map(|x| x as i32).collect(), (smash::hash40("walk_speed_max"), 0, 1.2));
	 crate::param_cache::update_float_2(*FIGHTER_KIND_PURIN, get_marked_costumes("purin","purin").into_iter().map(|x| x as i32).collect(), (smash::hash40("dash_speed"), 0, 1.76));
	 crate::param_cache::update_float_2(*FIGHTER_KIND_PURIN, get_marked_costumes("purin","purin").into_iter().map(|x| x as i32).collect(), (smash::hash40("run_speed_max"), 0, 1.35));
	 crate::param_cache::update_float_2(*FIGHTER_KIND_PURIN, get_marked_costumes("purin","purin").into_iter().map(|x| x as i32).collect(), (smash::hash40("weight"), 0, 72.0));
	 crate::param_cache::update_float_2(*FIGHTER_KIND_PURIN, get_marked_costumes("purin","purin").into_iter().map(|x| x as i32).collect(), (smash::hash40("landing_attack_air_frame_lw"), 0, 9.0));

}