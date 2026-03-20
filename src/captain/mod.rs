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

	param_config::update_float_2(*FIGHTER_KIND_CAPTAIN, get_marked_costumes("captain","captain"), (smash::hash40("dash_speed"), 0, 2.35));
	param_config::update_float_2(*FIGHTER_KIND_CAPTAIN, get_marked_costumes("captain","captain"), (smash::hash40("jump_speed_x"), 0, 1.2));
	param_config::update_float_2(*FIGHTER_KIND_CAPTAIN, get_marked_costumes("captain","captain"), (smash::hash40("jump_speed_x_max"), 0, 2.8));
	param_config::update_float_2(*FIGHTER_KIND_CAPTAIN, get_marked_costumes("captain","captain"), (smash::hash40("mini_jump_y"), 0, 15.25));
	param_config::update_float_2(*FIGHTER_KIND_CAPTAIN, get_marked_costumes("captain","captain"), (smash::hash40("air_speed_x_stable"), 0, 1.28));
	param_config::update_float_2(*FIGHTER_KIND_CAPTAIN, get_marked_costumes("captain","captain"), (smash::hash40("air_speed_y_stable"), 0, 1.92));
	param_config::update_float_2(*FIGHTER_KIND_CAPTAIN, get_marked_costumes("captain","captain"), (smash::hash40("dive_speed_y"), 0, 3.072));
	param_config::update_float_2(*FIGHTER_KIND_CAPTAIN, get_marked_costumes("captain","captain"), (smash::hash40("landing_attack_air_frame_f"), 0, 14.0));
	param_config::update_float_2(*FIGHTER_KIND_CAPTAIN, get_marked_costumes("captain","captain"), (smash::hash40("landing_attack_air_frame_b"), 0, 8.0));
	param_config::update_float_2(*FIGHTER_KIND_CAPTAIN, get_marked_costumes("captain","captain"), (smash::hash40("landing_attack_air_frame_hi"), 0, 6.0));

}