mod status;
mod frame;
mod acmd;
use crate::util::*;
use smash::lib::lua_const::*;
use smash::hash40;

static mut IS_UPB_DOWN: [bool; 8] = [false; 8];

pub fn install() {
	frame::install();
	status::install();
	acmd::install();

	param_config::update_float_2(*FIGHTER_KIND_INKLING, get_marked_costumes("inkling","inkling"), (smash::hash40("param_special_hi"), smash::hash40("height_mul_on_air"), 1.0));
	param_config::update_float_2(*FIGHTER_KIND_INKLING, get_marked_costumes("inkling","inkling"), (smash::hash40("param_special_hi"), smash::hash40("height_mul_on_ground"), 1.0));
	param_config::update_float_2(*FIGHTER_KIND_INKLING, get_marked_costumes("inkling","inkling"), (smash::hash40("run_speed_max"), 0, 2.0));
	param_config::update_float_2(*FIGHTER_KIND_INKLING, get_marked_costumes("inkling","inkling"), (smash::hash40("jump_speed_x"), 0, 1.4));
	param_config::update_float_2(*FIGHTER_KIND_INKLING, get_marked_costumes("inkling","inkling"), (smash::hash40("jump_speed_x_mul"), 0, 1.6));
	param_config::update_float_2(*FIGHTER_KIND_INKLING, get_marked_costumes("inkling","inkling"), (smash::hash40("jump_speed_x_max"), 0, 1.7));
	param_config::update_float_2(*FIGHTER_KIND_INKLING, get_marked_costumes("inkling","inkling"), (smash::hash40("jump_y"), 0, 40.0));
	param_config::update_float_2(*FIGHTER_KIND_INKLING, get_marked_costumes("inkling","inkling"), (smash::hash40("air_speed_y_stable"), 0, 1.7));
	param_config::update_float_2(*FIGHTER_KIND_INKLING, get_marked_costumes("inkling","inkling"), (smash::hash40("dive_speed_y"), 0, 2.72));
	param_config::update_float_2(*FIGHTER_KIND_INKLING, get_marked_costumes("inkling","inkling"), (smash::hash40("landing_attack_air_frame_n"), 0, 5.0));
	param_config::update_float_2(*FIGHTER_KIND_INKLING, get_marked_costumes("inkling","inkling"), (smash::hash40("landing_attack_air_frame_f"), 0, 11.0));
	param_config::update_float_2(*FIGHTER_KIND_INKLING, get_marked_costumes("inkling","inkling"), (smash::hash40("landing_attack_air_frame_b"), 0, 7.0));

}