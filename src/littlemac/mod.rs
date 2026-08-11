	
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

	crate::param_cache::update_int_2(*FIGHTER_KIND_LITTLEMAC, get_marked_costumes("littlemac","littlemac").into_iter().map(|x| x as i32).collect(), (smash::hash40("param_special_n"), smash::hash40("special_n_active_start_frame"), 13));
	crate::param_cache::update_int_2(*FIGHTER_KIND_LITTLEMAC, get_marked_costumes("littlemac","littlemac").into_iter().map(|x| x as i32).collect(), (smash::hash40("param_special_n"), smash::hash40("special_n_cancel_frame"), 20));
	 crate::param_cache::update_float_2(*FIGHTER_KIND_LITTLEMAC, get_marked_costumes("littlemac","littlemac").into_iter().map(|x| x as i32).collect(), (smash::hash40("param_special_hi"), smash::hash40("special_air_hi_pass_mul"), 1.2));
	 crate::param_cache::update_float_2(*FIGHTER_KIND_LITTLEMAC, get_marked_costumes("littlemac","littlemac").into_iter().map(|x| x as i32).collect(), (smash::hash40("jump_speed_x_mul"), 0, 1.0));
	 crate::param_cache::update_float_2(*FIGHTER_KIND_LITTLEMAC, get_marked_costumes("littlemac","littlemac").into_iter().map(|x| x as i32).collect(), (smash::hash40("jump_y"), 0, 30.0));
	 crate::param_cache::update_float_2(*FIGHTER_KIND_LITTLEMAC, get_marked_costumes("littlemac","littlemac").into_iter().map(|x| x as i32).collect(), (smash::hash40("mini_jump_y"), 0, 14.0));
	 crate::param_cache::update_float_2(*FIGHTER_KIND_LITTLEMAC, get_marked_costumes("littlemac","littlemac").into_iter().map(|x| x as i32).collect(), (smash::hash40("jump_aerial_y"), 0, 30.0));
	 crate::param_cache::update_float_2(*FIGHTER_KIND_LITTLEMAC, get_marked_costumes("littlemac","littlemac").into_iter().map(|x| x as i32).collect(), (smash::hash40("landing_attack_air_frame_n"), 0, 5.0));
	 crate::param_cache::update_float_2(*FIGHTER_KIND_LITTLEMAC, get_marked_costumes("littlemac","littlemac").into_iter().map(|x| x as i32).collect(), (smash::hash40("landing_attack_air_frame_f"), 0, 9.0));
	 crate::param_cache::update_float_2(*FIGHTER_KIND_LITTLEMAC, get_marked_costumes("littlemac","littlemac").into_iter().map(|x| x as i32).collect(), (smash::hash40("landing_attack_air_frame_b"), 0, 9.0));

}
