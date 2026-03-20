mod status;
mod frame;
mod acmd;
use crate::util::*;
			
pub fn install() {
	frame::install();
	status::install();
	acmd::install();

	param_config::update_int_2(*FIGHTER_KIND_PIT, get_marked_costumes("pit","pit"), (smash::hash40("param_special_hi"), smash::hash40("landing_frame"), 20));
	param_config::update_float_2(*FIGHTER_KIND_PIT, get_marked_costumes("pit","pit"), (smash::hash40("param_special_hi"), smash::hash40("rush_angle"), 340.0));
	param_config::update_float_2(*FIGHTER_KIND_PIT, get_marked_costumes("pit","pit"), (smash::hash40("walk_speed_max"), 0, 1.5));
	param_config::update_float_2(*FIGHTER_KIND_PIT, get_marked_costumes("pit","pit"), (smash::hash40("air_accel_x_mul"), 0, 0.1));
	param_config::update_float_2(*FIGHTER_KIND_PIT, get_marked_costumes("pit","pit"), (smash::hash40("air_speed_x_stable"), 0, 1.105));
	param_config::update_float_2(*FIGHTER_KIND_PIT, get_marked_costumes("pit","pit"), (smash::hash40("landing_attack_air_frame_n"), 0, 9.0));

}