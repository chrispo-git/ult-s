mod status;
mod frame;
mod acmd;
use crate::util::*;			
use smash::lib::lua_const::*;
use smash::hash40;


static mut SET_UPB_FREEFALL: [bool; 8] = [false; 8];

pub fn install() {
	frame::install();
	status::install();
	acmd::install();

	param_config::update_float_2(*FIGHTER_KIND_TOONLINK, get_marked_costumes("toonlink","toonlink").into_iter().map(|x| x as i32).collect(), (smash::hash40("param_special_hi"), smash::hash40("rslash_air_start_spd_y"), 1.953));
	param_config::update_float_2(*FIGHTER_KIND_TOONLINK, get_marked_costumes("toonlink","toonlink").into_iter().map(|x| x as i32).collect(), (smash::hash40("param_special_hi"), smash::hash40("rslash_air_gravity_y_mul"), 0.45));
	param_config::update_float_2(*FIGHTER_KIND_TOONLINK, get_marked_costumes("toonlink","toonlink").into_iter().map(|x| x as i32).collect(), (smash::hash40("param_special_hi"), smash::hash40("rslash_air_accel_x_mul"), 0.08));
	param_config::update_float_2(*FIGHTER_KIND_TOONLINK, get_marked_costumes("toonlink","toonlink").into_iter().map(|x| x as i32).collect(), (smash::hash40("param_special_hi"), smash::hash40("rslash_air_max_x_mul"), 0.75));
	param_config::update_int_2(*FIGHTER_KIND_TOONLINK, get_marked_costumes("toonlink","toonlink").into_iter().map(|x| x as i32).collect(), (smash::hash40("param_special_hi"), smash::hash40("rslash_landing_frame"), 22));
	param_config::update_float_2(*FIGHTER_KIND_TOONLINK, get_marked_costumes("toonlink","toonlink").into_iter().map(|x| x as i32).collect(), (smash::hash40("param_special_hi"), smash::hash40("rslash_end_air_accel_x_mul"), 0.08));
	param_config::update_float_2(*FIGHTER_KIND_TOONLINK, get_marked_costumes("toonlink","toonlink").into_iter().map(|x| x as i32).collect(), (smash::hash40("param_special_hi"), smash::hash40("rslash_end_air_max_x"), 0.75));
	param_config::update_float_2(*FIGHTER_KIND_TOONLINK, get_marked_costumes("toonlink","toonlink").into_iter().map(|x| x as i32).collect(), (smash::hash40("param_toonlinkbomb"), smash::hash40("toonlinkbomb_throw_speed_mul"), 0.9));
	param_config::update_float_2(*FIGHTER_KIND_TOONLINK, get_marked_costumes("toonlink","toonlink").into_iter().map(|x| x as i32).collect(), (smash::hash40("param_toonlinkbomb"), smash::hash40("toonlinkbomb_scale_mul"), 1.0));
	param_config::update_float_2(*FIGHTER_KIND_TOONLINK, get_marked_costumes("toonlink","toonlink").into_iter().map(|x| x as i32).collect(), (smash::hash40("param_toonlinkbomb"), smash::hash40("toonlinkbomb_bomb_speed"), 1.5));
	param_config::update_float_2(*FIGHTER_KIND_TOONLINK, get_marked_costumes("toonlink","toonlink").into_iter().map(|x| x as i32).collect(), (smash::hash40("ground_brake"), 0, 0.07));
	param_config::update_float_2(*FIGHTER_KIND_TOONLINK, get_marked_costumes("toonlink","toonlink").into_iter().map(|x| x as i32).collect(), (smash::hash40("dash_speed"), 0, 2.09));
	param_config::update_float_2(*FIGHTER_KIND_TOONLINK, get_marked_costumes("toonlink","toonlink").into_iter().map(|x| x as i32).collect(), (smash::hash40("jump_speed_x_mul"), 0, 1.5));
	param_config::update_float_2(*FIGHTER_KIND_TOONLINK, get_marked_costumes("toonlink","toonlink").into_iter().map(|x| x as i32).collect(), (smash::hash40("dive_speed_y"), 0, 2.622));
	param_config::update_float_2(*FIGHTER_KIND_TOONLINK, get_marked_costumes("toonlink","toonlink").into_iter().map(|x| x as i32).collect(), (smash::hash40("weight"), 0, 87.0));

}