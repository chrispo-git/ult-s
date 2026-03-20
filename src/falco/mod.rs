mod status;
mod frame;
mod acmd;
use crate::util::*;

static mut HAS_DOWNB : [bool; 8] = [false; 8];
static mut DO_STALL : [bool; 8] = [false; 8];

pub fn install() {
	frame::install();
	status::install();
	acmd::install();

	param_config::update_float_2(*FIGHTER_KIND_FALCO, get_marked_costumes("falco","falco"), (smash::hash40("dash_speed"), 0, 2.35));
	param_config::update_float_2(*FIGHTER_KIND_FALCO, get_marked_costumes("falco","falco"), (smash::hash40("jump_speed_x"), 0, 0.8));
	param_config::update_float_2(*FIGHTER_KIND_FALCO, get_marked_costumes("falco","falco"), (smash::hash40("jump_speed_x_mul"), 0, 1.2));
	param_config::update_float_2(*FIGHTER_KIND_FALCO, get_marked_costumes("falco","falco"), (smash::hash40("jump_speed_x_max"), 0, 2.5));
	param_config::update_float_2(*FIGHTER_KIND_FALCO, get_marked_costumes("falco","falco"), (smash::hash40("jump_y"), 0, 40.2));
	param_config::update_float_2(*FIGHTER_KIND_FALCO, get_marked_costumes("falco","falco"), (smash::hash40("mini_jump_y"), 0, 16.4));
	param_config::update_float_2(*FIGHTER_KIND_FALCO, get_marked_costumes("falco","falco"), (smash::hash40("jump_aerial_y"), 0, 40.2));
	param_config::update_float_2(*FIGHTER_KIND_FALCO, get_marked_costumes("falco","falco"), (smash::hash40("air_speed_x_stable"), 0, 1.1));
	param_config::update_float_2(*FIGHTER_KIND_FALCO, get_marked_costumes("falco","falco"), (smash::hash40("air_accel_y"), 0, 0.17));
	param_config::update_float_2(*FIGHTER_KIND_FALCO, get_marked_costumes("falco","falco"), (smash::hash40("air_speed_y_stable"), 0, 1.825));
	param_config::update_float_2(*FIGHTER_KIND_FALCO, get_marked_costumes("falco","falco"), (smash::hash40("dive_speed_y"), 0, 2.92));
	param_config::update_float_2(*FIGHTER_KIND_FALCO, get_marked_costumes("falco","falco"), (smash::hash40("landing_attack_air_frame_n"), 0, 6.0));
	param_config::update_float_2(*FIGHTER_KIND_FALCO, get_marked_costumes("falco","falco"), (smash::hash40("landing_attack_air_frame_b"), 0, 11.0));
}