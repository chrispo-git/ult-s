mod status;
mod frame;
mod acmd;
use crate::util::*;
use smash::lib::lua_const::*;
use smash::hash40;

static FIGHTER_MEWTWO_INSTANCE_WORK_ID_INT_ATTACK_AIR_WINDOW : i32 = 0;
static mut MAX_ATTACK_AIR_WINDOW : i32 = 15;
static FIGHTER_MEWTWO_INSTANCE_WORK_ID_FLAG_HAS_ATTACK_AIR : i32 = 1;
static FIGHTER_MEWTWO_INSTANCE_WORK_ID_FLAG_HAS_ALREADY_TELECANCEL : i32 = 2;




pub fn install() {
	frame::install();
	status::install();
	acmd::install();

	param_config::update_float_2(*FIGHTER_KIND_MEWTWO, get_marked_costumes("mewtwo","mewtwo").into_iter().map(|x| x as i32).collect(), (smash::hash40("jump_speed_x_mul"), 0, 0.8));
	param_config::update_float_2(*FIGHTER_KIND_MEWTWO, get_marked_costumes("mewtwo","mewtwo").into_iter().map(|x| x as i32).collect(), (smash::hash40("air_speed_x_stable"), 0, 1.2));
	param_config::update_float_2(*FIGHTER_KIND_MEWTWO, get_marked_costumes("mewtwo","mewtwo").into_iter().map(|x| x as i32).collect(), (smash::hash40("weight"), 0, 80.0));
	param_config::update_float_2(*FIGHTER_KIND_MEWTWO, get_marked_costumes("mewtwo","mewtwo").into_iter().map(|x| x as i32).collect(), (smash::hash40("landing_attack_air_frame_f"), 0, 7.0));
	param_config::update_float_2(*FIGHTER_KIND_MEWTWO, get_marked_costumes("mewtwo","mewtwo").into_iter().map(|x| x as i32).collect(), (smash::hash40("landing_attack_air_frame_b"), 0, 8.0));
	param_config::update_float_2(*FIGHTER_KIND_MEWTWO, get_marked_costumes("mewtwo","mewtwo").into_iter().map(|x| x as i32).collect(), (smash::hash40("landing_attack_air_frame_hi"), 0, 9.0));
	param_config::update_float_2(*FIGHTER_KIND_MEWTWO, get_marked_costumes("mewtwo","mewtwo").into_iter().map(|x| x as i32).collect(), (smash::hash40("landing_frame"), 0, 3.0));

}