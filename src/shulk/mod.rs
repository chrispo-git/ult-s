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

	param_config::update_float_2(*FIGHTER_KIND_SHULK, get_marked_costumes("shulk","shulk"), (smash::hash40("jump_speed_x_mul"), 0, 1.2));
	param_config::update_float_2(*FIGHTER_KIND_SHULK, get_marked_costumes("shulk","shulk"), (smash::hash40("landing_attack_air_frame_n"), 0, 10.0));

}