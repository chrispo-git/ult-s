mod status;
mod frame;
mod acmd;
use crate::util::*;
use smash::lib::lua_const::*;
use smash::hash40;

static FIGHTER_ROCKMAN_INSTANCE_WORK_ID_FLAG_MEGA_AERIAL : i32 = 0;
			


pub fn install() {
	let costumes_rockman_rockman: Vec<i32> = get_marked_costumes("rockman", "rockman").into_iter().map(|x| x as i32).collect();
	frame::install();
	status::install();
	acmd::install();

	 crate::param_cache::update_float_2(*FIGHTER_KIND_ROCKMAN, costumes_rockman_rockman.clone(), (smash::hash40("landing_attack_air_frame_n"), 0, 11.0));
	 crate::param_cache::update_float_2(*FIGHTER_KIND_ROCKMAN, costumes_rockman_rockman.clone(), (smash::hash40("landing_attack_air_frame_b"), 0, 10.0));

}