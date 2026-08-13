mod status;
mod frame;
mod acmd;
use crate::util::*;			
use smash::lib::lua_const::*;
use smash::hash40;


static FIGHTER_TOONLINK_INSTANCE_WORK_ID_FLAG_SET_UPB_FREEFALL : i32 = 0;

pub fn install() {
	frame::install();
	status::install();
	acmd::install();

	 crate::param_cache::update_float_2(*FIGHTER_KIND_TOONLINK, get_marked_costumes("toonlink","toonlink").into_iter().map(|x| x as i32).collect(), (smash::hash40("param_special_hi"), smash::hash40("rslash_air_start_spd_y"), 1.953));
	 crate::param_cache::update_float_2(*FIGHTER_KIND_TOONLINK, get_marked_costumes("toonlink","toonlink").into_iter().map(|x| x as i32).collect(), (smash::hash40("param_special_hi"), smash::hash40("rslash_air_gravity_y_mul"), 0.45));
	 crate::param_cache::update_float_2(*FIGHTER_KIND_TOONLINK, get_marked_costumes("toonlink","toonlink").into_iter().map(|x| x as i32).collect(), (smash::hash40("param_special_hi"), smash::hash40("rslash_air_accel_x_mul"), 0.08));
	 crate::param_cache::update_float_2(*FIGHTER_KIND_TOONLINK, get_marked_costumes("toonlink","toonlink").into_iter().map(|x| x as i32).collect(), (smash::hash40("param_special_hi"), smash::hash40("rslash_air_max_x_mul"), 0.75));
	crate::param_cache::update_int_2(*FIGHTER_KIND_TOONLINK, get_marked_costumes("toonlink","toonlink").into_iter().map(|x| x as i32).collect(), (smash::hash40("param_special_hi"), smash::hash40("rslash_landing_frame"), 22));
	 crate::param_cache::update_float_2(*FIGHTER_KIND_TOONLINK, get_marked_costumes("toonlink","toonlink").into_iter().map(|x| x as i32).collect(), (smash::hash40("param_special_hi"), smash::hash40("rslash_end_air_accel_x_mul"), 0.08));
	 crate::param_cache::update_float_2(*FIGHTER_KIND_TOONLINK, get_marked_costumes("toonlink","toonlink").into_iter().map(|x| x as i32).collect(), (smash::hash40("param_special_hi"), smash::hash40("rslash_end_air_max_x"), 0.75));
	 crate::param_cache::update_float_2(*FIGHTER_KIND_TOONLINK, get_marked_costumes("toonlink","toonlink").into_iter().map(|x| x as i32).collect(), (smash::hash40("param_toonlinkbomb"), smash::hash40("toonlinkbomb_throw_speed_mul"), 0.9));
	 crate::param_cache::update_float_2(*FIGHTER_KIND_TOONLINK, get_marked_costumes("toonlink","toonlink").into_iter().map(|x| x as i32).collect(), (smash::hash40("param_toonlinkbomb"), smash::hash40("toonlinkbomb_scale_mul"), 1.0));
	 crate::param_cache::update_float_2(*FIGHTER_KIND_TOONLINK, get_marked_costumes("toonlink","toonlink").into_iter().map(|x| x as i32).collect(), (smash::hash40("param_toonlinkbomb"), smash::hash40("toonlinkbomb_bomb_speed"), 1.5));
	 crate::param_cache::update_float_2(*FIGHTER_KIND_TOONLINK, get_marked_costumes("toonlink","toonlink").into_iter().map(|x| x as i32).collect(), (smash::hash40("ground_brake"), 0, 0.07));
	 crate::param_cache::update_float_2(*FIGHTER_KIND_TOONLINK, get_marked_costumes("toonlink","toonlink").into_iter().map(|x| x as i32).collect(), (smash::hash40("dash_speed"), 0, 2.09));
	 crate::param_cache::update_float_2(*FIGHTER_KIND_TOONLINK, get_marked_costumes("toonlink","toonlink").into_iter().map(|x| x as i32).collect(), (smash::hash40("jump_speed_x_mul"), 0, 1.5));
	 crate::param_cache::update_float_2(*FIGHTER_KIND_TOONLINK, get_marked_costumes("toonlink","toonlink").into_iter().map(|x| x as i32).collect(), (smash::hash40("dive_speed_y"), 0, 2.622));
	 crate::param_cache::update_float_2(*FIGHTER_KIND_TOONLINK, get_marked_costumes("toonlink","toonlink").into_iter().map(|x| x as i32).collect(), (smash::hash40("weight"), 0, 87.0));

}