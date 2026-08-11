mod status;
mod frame;
mod acmd;
use crate::util::*;
use smash::lib::lua_const::*;
use smash::hash40;

static FIGHTER_FALCO_INSTANCE_WORK_ID_FLAG_HAS_DOWNB : i32 = 0;
static FIGHTER_FALCO_INSTANCE_WORK_ID_FLAG_DO_STALL : i32 = 1;

pub fn install() {
	frame::install();
	status::install();
	acmd::install();

	 crate::param_cache::update_float_2(*FIGHTER_KIND_FALCO, get_marked_costumes("falco","falco").into_iter().map(|x| x as i32).collect(), (smash::hash40("dash_speed"), 0, 2.35));
	 crate::param_cache::update_float_2(*FIGHTER_KIND_FALCO, get_marked_costumes("falco","falco").into_iter().map(|x| x as i32).collect(), (smash::hash40("jump_speed_x"), 0, 0.8));
	 crate::param_cache::update_float_2(*FIGHTER_KIND_FALCO, get_marked_costumes("falco","falco").into_iter().map(|x| x as i32).collect(), (smash::hash40("jump_speed_x_mul"), 0, 1.2));
	 crate::param_cache::update_float_2(*FIGHTER_KIND_FALCO, get_marked_costumes("falco","falco").into_iter().map(|x| x as i32).collect(), (smash::hash40("jump_speed_x_max"), 0, 2.5));
	 crate::param_cache::update_float_2(*FIGHTER_KIND_FALCO, get_marked_costumes("falco","falco").into_iter().map(|x| x as i32).collect(), (smash::hash40("jump_y"), 0, 40.2));
	 crate::param_cache::update_float_2(*FIGHTER_KIND_FALCO, get_marked_costumes("falco","falco").into_iter().map(|x| x as i32).collect(), (smash::hash40("mini_jump_y"), 0, 16.4));
	 crate::param_cache::update_float_2(*FIGHTER_KIND_FALCO, get_marked_costumes("falco","falco").into_iter().map(|x| x as i32).collect(), (smash::hash40("jump_aerial_y"), 0, 40.2));
	 crate::param_cache::update_float_2(*FIGHTER_KIND_FALCO, get_marked_costumes("falco","falco").into_iter().map(|x| x as i32).collect(), (smash::hash40("air_speed_x_stable"), 0, 1.1));
	 crate::param_cache::update_float_2(*FIGHTER_KIND_FALCO, get_marked_costumes("falco","falco").into_iter().map(|x| x as i32).collect(), (smash::hash40("air_accel_y"), 0, 0.17));
	 crate::param_cache::update_float_2(*FIGHTER_KIND_FALCO, get_marked_costumes("falco","falco").into_iter().map(|x| x as i32).collect(), (smash::hash40("air_speed_y_stable"), 0, 1.825));
	 crate::param_cache::update_float_2(*FIGHTER_KIND_FALCO, get_marked_costumes("falco","falco").into_iter().map(|x| x as i32).collect(), (smash::hash40("dive_speed_y"), 0, 2.92));
	 crate::param_cache::update_float_2(*FIGHTER_KIND_FALCO, get_marked_costumes("falco","falco").into_iter().map(|x| x as i32).collect(), (smash::hash40("landing_attack_air_frame_n"), 0, 6.0));
	 crate::param_cache::update_float_2(*FIGHTER_KIND_FALCO, get_marked_costumes("falco","falco").into_iter().map(|x| x as i32).collect(), (smash::hash40("landing_attack_air_frame_b"), 0, 11.0));
}