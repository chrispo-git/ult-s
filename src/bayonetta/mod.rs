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
	param_config::update_int_2(*FIGHTER_KIND_BAYONETTA, get_marked_costumes("bayonetta","bayonetta"), (smash::hash40("param_special_hi"), smash::hash40("landing_frame"), 15));
	param_config::update_int_2(*FIGHTER_KIND_BAYONETTA, get_marked_costumes("bayonetta","bayonetta"), (smash::hash40("param_special_hi"), smash::hash40("act_landing_frame"), 19));
	param_config::update_int_2(*FIGHTER_KIND_BAYONETTA, get_marked_costumes("bayonetta","bayonetta"), (smash::hash40("param_special_hi"), smash::hash40("landing_frame_2nd"), 23));
	param_config::update_int_2(*FIGHTER_KIND_BAYONETTA, get_marked_costumes("bayonetta","bayonetta"), (smash::hash40("param_special_hi"), smash::hash40("act_landing_frame_2nd"), 26));
	param_config::update_float_2(*FIGHTER_KIND_BAYONETTA, get_marked_costumes("bayonetta","bayonetta"), (smash::hash40("param_special_lw"), smash::hash40("base_slow_frame_max"), 120.0));
	param_config::update_float_2(*FIGHTER_KIND_BAYONETTA, get_marked_costumes("bayonetta","bayonetta"), (smash::hash40("jump_speed_x_mul"), 0, 1.5));
}