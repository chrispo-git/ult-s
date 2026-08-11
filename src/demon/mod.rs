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

	 crate::param_cache::update_float_2(*FIGHTER_KIND_DEMON, get_marked_costumes("demon","demon").into_iter().map(|x| x as i32).collect(), (smash::hash40("ground_brake"), 0, 0.1));
	 crate::param_cache::update_float_2(*FIGHTER_KIND_DEMON, get_marked_costumes("demon","demon").into_iter().map(|x| x as i32).collect(), (smash::hash40("dash_speed"), 0, 1.825));
	 crate::param_cache::update_float_2(*FIGHTER_KIND_DEMON, get_marked_costumes("demon","demon").into_iter().map(|x| x as i32).collect(), (smash::hash40("run_speed_max"), 0, 1.6));
	 crate::param_cache::update_float_2(*FIGHTER_KIND_DEMON, get_marked_costumes("demon","demon").into_iter().map(|x| x as i32).collect(), (smash::hash40("jump_y"), 0, 22.0));
	 crate::param_cache::update_float_2(*FIGHTER_KIND_DEMON, get_marked_costumes("demon","demon").into_iter().map(|x| x as i32).collect(), (smash::hash40("jump_aerial_y"), 0, 50.0));
	 crate::param_cache::update_float_2(*FIGHTER_KIND_DEMON, get_marked_costumes("demon","demon").into_iter().map(|x| x as i32).collect(), (smash::hash40("air_accel_x_mul"), 0, 0.05));
	 crate::param_cache::update_float_2(*FIGHTER_KIND_DEMON, get_marked_costumes("demon","demon").into_iter().map(|x| x as i32).collect(), (smash::hash40("air_speed_x_stable"), 0, 1.075));
	 crate::param_cache::update_float_2(*FIGHTER_KIND_DEMON, get_marked_costumes("demon","demon").into_iter().map(|x| x as i32).collect(), (smash::hash40("weight"), 0, 105.0));
	 crate::param_cache::update_float_2(*FIGHTER_KIND_DEMON, get_marked_costumes("demon","demon").into_iter().map(|x| x as i32).collect(), (smash::hash40("landing_attack_air_frame_lw"), 0, 25.0));
	 crate::param_cache::update_float_2(*FIGHTER_KIND_DEMON, get_marked_costumes("demon","demon").into_iter().map(|x| x as i32).collect(), (smash::hash40("attack_s4_smash_hold_attack_up"), 0, 1.3));
	 crate::param_cache::update_float_2(*FIGHTER_KIND_DEMON, get_marked_costumes("demon","demon").into_iter().map(|x| x as i32).collect(), (smash::hash40("attack_hi4_smash_hold_attack_up"), 0, 1.3));
	 crate::param_cache::update_float_2(*FIGHTER_KIND_DEMON, get_marked_costumes("demon","demon").into_iter().map(|x| x as i32).collect(), (smash::hash40("attack_lw4_smash_hold_attack_up"), 0, 1.3));
	crate::param_cache::update_int_2(*FIGHTER_KIND_DEMON, get_marked_costumes("demon","demon").into_iter().map(|x| x as i32).collect(), (smash::hash40("jump_squat_frame"), 0, 5));

}