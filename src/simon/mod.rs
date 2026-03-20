mod status;
mod frame;
mod acmd;
use crate::util::*;

static mut STATIC_MUT : [i32; 8] = [6; 8];
			


pub fn install() {
	frame::install();
	status::install();
	acmd::install();

	param_config::update_float_2(-*FIGHTER_KIND_SIMON, get_marked_costumes("simon","simon"), (smash::hash40("param_axe"), smash::hash40("throw_angle"), 30.0));
	param_config::update_float_2(-*FIGHTER_KIND_SIMON, get_marked_costumes("simon","simon"), (smash::hash40("param_axe"), smash::hash40("throw_angle_stick_back"), 40.0));
	param_config::update_float_2(-*FIGHTER_KIND_SIMON, get_marked_costumes("simon","simon"), (smash::hash40("param_axe"), smash::hash40("throw_angle_stick_front"), 20.0));
	param_config::update_float_2(*FIGHTER_KIND_SIMON, get_marked_costumes("simon","simon"), (smash::hash40("jump_speed_x_mul"), 0, 1.075));
	param_config::update_float_2(*FIGHTER_KIND_SIMON, get_marked_costumes("simon","simon"), (smash::hash40("air_speed_y_stable"), 0, 1.6));
	param_config::update_float_2(*FIGHTER_KIND_SIMON, get_marked_costumes("simon","simon"), (smash::hash40("shield_radius"), 0, 13.0));

}
