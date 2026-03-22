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

	param_config::update_float_2(*FIGHTER_KIND_BRAVE, get_marked_costumes("brave","brave").into_iter().map(|x| x as i32).collect(), (smash::hash40("jump_speed_x_mul"), 0, 1.0));
	param_config::update_float_2(*FIGHTER_KIND_BRAVE, get_marked_costumes("brave","brave").into_iter().map(|x| x as i32).collect(), (smash::hash40("landing_attack_air_frame_f"), 0, 10.0));
	param_config::update_float_2(*FIGHTER_KIND_BRAVE, get_marked_costumes("brave","brave").into_iter().map(|x| x as i32).collect(), (smash::hash40("landing_attack_air_frame_b"), 0, 11.0));
	param_config::update_float_2(*FIGHTER_KIND_BRAVE, get_marked_costumes("brave","brave").into_iter().map(|x| x as i32).collect(), (smash::hash40("landing_attack_air_frame_hi"), 0, 6.0));

}