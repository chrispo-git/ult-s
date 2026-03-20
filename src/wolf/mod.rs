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

	param_config::update_float_2(*FIGHTER_KIND_WOLF, get_marked_costumes("wolf","wolf").into_iter().map(|x| x as i32).collect(), (smash::hash40("jump_speed_x_mul"), 0, 1.0));
	param_config::update_float_2(*FIGHTER_KIND_WOLF, get_marked_costumes("wolf","wolf").into_iter().map(|x| x as i32).collect(), (smash::hash40("weight"), 0, 87.0));
	param_config::update_float_2(*FIGHTER_KIND_WOLF, get_marked_costumes("wolf","wolf").into_iter().map(|x| x as i32).collect(), (smash::hash40("landing_attack_air_frame_n"), 0, 11.0));
	param_config::update_float_2(*FIGHTER_KIND_WOLF, get_marked_costumes("wolf","wolf").into_iter().map(|x| x as i32).collect(), (smash::hash40("scale"), 0, 1.14));

}