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

	 crate::param_cache::update_float_2(*FIGHTER_KIND_METAKNIGHT, get_marked_costumes("metaknight","metaknight").into_iter().map(|x| x as i32).collect(), (smash::hash40("jump_speed_x_mul"), 0, 1.15));
	 crate::param_cache::update_float_2(*FIGHTER_KIND_METAKNIGHT, get_marked_costumes("metaknight","metaknight").into_iter().map(|x| x as i32).collect(), (smash::hash40("jump_speed_x_max"), 0, 1.8));
	 crate::param_cache::update_float_2(*FIGHTER_KIND_METAKNIGHT, get_marked_costumes("metaknight","metaknight").into_iter().map(|x| x as i32).collect(), (smash::hash40("jump_y"), 0, 35.0));
	 crate::param_cache::update_float_2(*FIGHTER_KIND_METAKNIGHT, get_marked_costumes("metaknight","metaknight").into_iter().map(|x| x as i32).collect(), (smash::hash40("mini_jump_y"), 0, 11.5));
	 crate::param_cache::update_float_2(*FIGHTER_KIND_METAKNIGHT, get_marked_costumes("metaknight","metaknight").into_iter().map(|x| x as i32).collect(), (smash::hash40("air_accel_x_add"), 0, 0.02));
	 crate::param_cache::update_float_2(*FIGHTER_KIND_METAKNIGHT, get_marked_costumes("metaknight","metaknight").into_iter().map(|x| x as i32).collect(), (smash::hash40("air_speed_x_stable"), 0, 1.15));
	 crate::param_cache::update_float_2(*FIGHTER_KIND_METAKNIGHT, get_marked_costumes("metaknight","metaknight").into_iter().map(|x| x as i32).collect(), (smash::hash40("weight"), 0, 78.0));
	 crate::param_cache::update_float_2(*FIGHTER_KIND_METAKNIGHT, get_marked_costumes("metaknight","metaknight").into_iter().map(|x| x as i32).collect(), (smash::hash40("landing_attack_air_frame_f"), 0, 8.0));
	 crate::param_cache::update_float_2(*FIGHTER_KIND_METAKNIGHT, get_marked_costumes("metaknight","metaknight").into_iter().map(|x| x as i32).collect(), (smash::hash40("landing_attack_air_frame_lw"), 0, 18.0));

}