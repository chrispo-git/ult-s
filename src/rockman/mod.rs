mod status;
mod frame;
mod acmd;
use crate::util::*;
use smash::lib::lua_const::*;
use smash::hash40;

static mut MEGA_AERIAL : [bool; 8] = [false; 8];
			


pub fn install() {
	frame::install();
	status::install();
	acmd::install();

	param_config::update_float_2(*FIGHTER_KIND_ROCKMAN, get_marked_costumes("rockman","rockman").into_iter().map(|x| x as i32).collect(), (smash::hash40("landing_attack_air_frame_n"), 0, 11.0));
	param_config::update_float_2(*FIGHTER_KIND_ROCKMAN, get_marked_costumes("rockman","rockman").into_iter().map(|x| x as i32).collect(), (smash::hash40("landing_attack_air_frame_b"), 0, 10.0));

}