mod status;
mod frame;
mod acmd;
use crate::util::*;

static mut HYDRANT_POS_X : [f32; 8] = [0.0; 8];
static mut HYDRANT_POS_Y : [f32; 8] = [0.0; 8];
static mut TRAMPOLINE_POS_X : [f32; 8] = [0.0; 8];
static mut TRAMPOLINE_POS_Y : [f32; 8] = [0.0; 8];
static mut TRAMPOLINE_DELETE_TIMER : [i32; 8] = [0; 8];
static mut HAS_UPB_ENDS : [bool; 8] = [false; 8];
static mut WE_BOUNCE_NOW : [bool; 8] = [false; 8];
			


pub fn install() {
	frame::install();
	status::install();
	acmd::install();

	param_config::update_float_2(-*FIGHTER_KIND_PACMAN, get_marked_costumes("pacman","pacman"), (smash::hash40("param_key"), smash::hash40("speed"), 3.45));
	param_config::update_float_2(*FIGHTER_KIND_PACMAN, get_marked_costumes("pacman","pacman"), (smash::hash40("param_special_hi"), smash::hash40("air_accel_x_mul"), 1.5));
	param_config::update_int_2(-*FIGHTER_KIND_PACMAN, get_marked_costumes("pacman","pacman"), (smash::hash40("param_firehydrant"), smash::hash40("life"), 275));
	param_config::update_int_2(-*FIGHTER_KIND_PACMAN, get_marked_costumes("pacman","pacman"), (smash::hash40("param_firehydrant"), smash::hash40("hp"), 7));
	param_config::update_float_2(*FIGHTER_KIND_PACMAN, get_marked_costumes("pacman","pacman"), (smash::hash40("ground_brake"), 0, 0.085));
	param_config::update_float_2(*FIGHTER_KIND_PACMAN, get_marked_costumes("pacman","pacman"), (smash::hash40("dash_speed"), 0, 1.9));
	param_config::update_float_2(*FIGHTER_KIND_PACMAN, get_marked_costumes("pacman","pacman"), (smash::hash40("run_speed_max"), 0, 1.75));
	param_config::update_float_2(*FIGHTER_KIND_PACMAN, get_marked_costumes("pacman","pacman"), (smash::hash40("jump_speed_x_mul"), 0, 1.0));
	param_config::update_float_2(*FIGHTER_KIND_PACMAN, get_marked_costumes("pacman","pacman"), (smash::hash40("landing_attack_air_frame_hi"), 0, 8.0));
	param_config::update_float_2(*FIGHTER_KIND_PACMAN, get_marked_costumes("pacman","pacman"), (smash::hash40("landing_attack_air_frame_lw"), 0, 9.0));

}