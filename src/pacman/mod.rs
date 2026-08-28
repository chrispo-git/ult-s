mod status;
mod frame;
mod acmd;
use crate::util::*;
use smash::lib::lua_const::*;
use smash::hash40;

static FIGHTER_PACMAN_INSTANCE_WORK_ID_FLOAT_HYDRANT_POS_X : i32 = 0;
static FIGHTER_PACMAN_INSTANCE_WORK_ID_FLOAT_HYDRANT_POS_Y : i32 = 1;
static FIGHTER_PACMAN_INSTANCE_WORK_ID_FLOAT_TRAMPOLINE_POS_X : i32 = 2;
static FIGHTER_PACMAN_INSTANCE_WORK_ID_FLOAT_TRAMPOLINE_POS_Y : i32 = 3;
static FIGHTER_PACMAN_INSTANCE_WORK_ID_INT_TRAMPOLINE_DELETE_TIMER : i32 = 4;
static FIGHTER_PACMAN_INSTANCE_WORK_ID_FLAG_HAS_UPB_ENDS : i32 = 5;
static FIGHTER_PACMAN_INSTANCE_WORK_ID_FLAG_WE_BOUNCE_NOW : i32 = 6;
			


pub fn install() {
	frame::install();
	status::install();
	acmd::install();

	 crate::param_cache::update_float_2(*FIGHTER_KIND_PACMAN, get_marked_costumes("pacman","pacman").into_iter().map(|x| x as i32).collect(), (smash::hash40("param_key"), smash::hash40("speed"), 3.45));
	 crate::param_cache::update_float_2(*FIGHTER_KIND_PACMAN, get_marked_costumes("pacman","pacman").into_iter().map(|x| x as i32).collect(), (smash::hash40("param_special_hi"), smash::hash40("air_accel_x_mul"), 1.5));
	crate::param_cache::update_int_2(-*WEAPON_KIND_PACMAN_FIREHYDRANT, get_marked_costumes("pacman","pacman").into_iter().map(|x| x as i32).collect(), (smash::hash40("param_firehydrant"), smash::hash40("life"), 275));
	crate::param_cache::update_int_2(-*WEAPON_KIND_PACMAN_FIREHYDRANT, get_marked_costumes("pacman","pacman").into_iter().map(|x| x as i32).collect(), (smash::hash40("param_firehydrant"), smash::hash40("hp"), 7));
	 crate::param_cache::update_float_2(*FIGHTER_KIND_PACMAN, get_marked_costumes("pacman","pacman").into_iter().map(|x| x as i32).collect(), (smash::hash40("ground_brake"), 0, 0.085));
	 crate::param_cache::update_float_2(*FIGHTER_KIND_PACMAN, get_marked_costumes("pacman","pacman").into_iter().map(|x| x as i32).collect(), (smash::hash40("dash_speed"), 0, 1.9));
	 crate::param_cache::update_float_2(*FIGHTER_KIND_PACMAN, get_marked_costumes("pacman","pacman").into_iter().map(|x| x as i32).collect(), (smash::hash40("run_speed_max"), 0, 1.75));
	 crate::param_cache::update_float_2(*FIGHTER_KIND_PACMAN, get_marked_costumes("pacman","pacman").into_iter().map(|x| x as i32).collect(), (smash::hash40("jump_speed_x_mul"), 0, 1.0));
	 crate::param_cache::update_float_2(*FIGHTER_KIND_PACMAN, get_marked_costumes("pacman","pacman").into_iter().map(|x| x as i32).collect(), (smash::hash40("landing_attack_air_frame_hi"), 0, 8.0));
	 crate::param_cache::update_float_2(*FIGHTER_KIND_PACMAN, get_marked_costumes("pacman","pacman").into_iter().map(|x| x as i32).collect(), (smash::hash40("landing_attack_air_frame_lw"), 0, 23.0));

}