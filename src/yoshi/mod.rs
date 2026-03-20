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

	param_config::update_float_2(*FIGHTER_KIND_YOSHI, get_marked_costumes("yoshi","yoshi"), (smash::hash40("jump_speed_x_mul"), 0, 1.15));
	param_config::update_float_2(*FIGHTER_KIND_YOSHI, get_marked_costumes("yoshi","yoshi"), (smash::hash40("air_speed_x_stable"), 0, 1.285));
	param_config::update_float_2(*FIGHTER_KIND_YOSHI, get_marked_costumes("yoshi","yoshi"), (smash::hash40("air_speed_y_stable"), 0, 1.5));
	param_config::update_float_2(*FIGHTER_KIND_YOSHI, get_marked_costumes("yoshi","yoshi"), (smash::hash40("landing_attack_air_frame_f"), 0, 10.0));
	param_config::update_float_2(*FIGHTER_KIND_YOSHI, get_marked_costumes("yoshi","yoshi"), (smash::hash40("landing_attack_air_frame_b"), 0, 7.0));

}