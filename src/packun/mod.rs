mod status;
mod frame;
mod acmd;
use crate::util::*;
use smash::lib::lua_const::*;
use smash::hash40;

static mut IS_BAIR : [bool; 8] = [false; 8];
static mut IS_SIDEB_EAT : [bool; 8] = [false; 8];

static mut BREATH_POS_X : [f32; 8] = [0.0; 8];
static mut BREATH_POS_Y : [f32; 8] = [0.0; 8];
			


pub fn install() {
	frame::install();
	status::install();
	acmd::install();

	param_config::update_float_2(*FIGHTER_KIND_PACKUN, get_marked_costumes("packun","packun"), (smash::hash40("dash_speed"), 0, 1.92));
	param_config::update_float_2(*FIGHTER_KIND_PACKUN, get_marked_costumes("packun","packun"), (smash::hash40("run_speed_max"), 0, 1.82));
	param_config::update_float_2(*FIGHTER_KIND_PACKUN, get_marked_costumes("packun","packun"), (smash::hash40("jump_speed_x_mul"), 0, 0.95));
	param_config::update_float_2(*FIGHTER_KIND_PACKUN, get_marked_costumes("packun","packun"), (smash::hash40("air_speed_x_stable"), 0, 1.075));
	param_config::update_float_2(*FIGHTER_KIND_PACKUN, get_marked_costumes("packun","packun"), (smash::hash40("landing_attack_air_frame_f"), 0, 9.0));
	param_config::update_float_2(*FIGHTER_KIND_PACKUN, get_marked_costumes("packun","packun"), (smash::hash40("landing_attack_air_frame_b"), 0, 12.0));
	param_config::update_float_2(*FIGHTER_KIND_PACKUN, get_marked_costumes("packun","packun"), (smash::hash40("landing_attack_air_frame_lw"), 0, 12.0));

}