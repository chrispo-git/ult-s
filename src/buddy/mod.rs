mod status;
mod frame;
mod acmd;
use crate::util::*;
use smash::lib::lua_const::*;
use smash::hash40;

pub fn install() {
	let costumes_buddy_buddy: Vec<i32> = get_marked_costumes("buddy", "buddy").into_iter().map(|x| x as i32).collect();
	frame::install();
	status::install();
	acmd::install();

	 crate::param_cache::update_float_2(*FIGHTER_KIND_BUDDY, costumes_buddy_buddy.clone(), (smash::hash40("jump_speed_x_mul"), 0, 1.0));
	 crate::param_cache::update_float_2(*FIGHTER_KIND_BUDDY, costumes_buddy_buddy.clone(), (smash::hash40("landing_attack_air_frame_n"), 0, 12.0));
	 crate::param_cache::update_float_2(*FIGHTER_KIND_BUDDY, costumes_buddy_buddy.clone(), (smash::hash40("landing_attack_air_frame_b"), 0, 11.0));
	 crate::param_cache::update_float_2(*FIGHTER_KIND_BUDDY, costumes_buddy_buddy.clone(), (smash::hash40("landing_attack_air_frame_hi"), 0, 10.0));

}