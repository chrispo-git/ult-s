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

	 crate::param_cache::update_float_2(*FIGHTER_KIND_ZELDA, get_marked_costumes("zelda","zelda").into_iter().map(|x| x as i32).collect(), (smash::hash40("param_special_s"), smash::hash40("accel_y"), 0.0005));
	 crate::param_cache::update_float_2(*FIGHTER_KIND_ZELDA, get_marked_costumes("zelda","zelda").into_iter().map(|x| x as i32).collect(), (smash::hash40("param_special_s"), smash::hash40("speed_mul_x"), 1.3));
	 crate::param_cache::update_float_2(-*WEAPON_KIND_ZELDA_DEIN, get_marked_costumes("zelda","zelda").into_iter().map(|x| x as i32).collect(), (smash::hash40("param_dein"), smash::hash40("size_max"), 4.2));
	 crate::param_cache::update_float_2(-*WEAPON_KIND_ZELDA_DEIN, get_marked_costumes("zelda","zelda").into_iter().map(|x| x as i32).collect(), (smash::hash40("param_dein"), smash::hash40("bang_time"), 2.0));
	 crate::param_cache::update_float_2(*FIGHTER_KIND_ZELDA, get_marked_costumes("zelda","zelda").into_iter().map(|x| x as i32).collect(), (smash::hash40("jump_speed_x_mul"), 0, 0.8));
	 crate::param_cache::update_float_2(*FIGHTER_KIND_ZELDA, get_marked_costumes("zelda","zelda").into_iter().map(|x| x as i32).collect(), (smash::hash40("air_accel_x_mul"), 0, 0.09));
	 crate::param_cache::update_float_2(*FIGHTER_KIND_ZELDA, get_marked_costumes("zelda","zelda").into_iter().map(|x| x as i32).collect(), (smash::hash40("landing_attack_air_frame_f"), 0, 11.0));

}