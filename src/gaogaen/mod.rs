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

	 crate::param_cache::update_float_2(*FIGHTER_KIND_GAOGAEN, get_marked_costumes("gaogaen","gaogaen").into_iter().map(|x| x as i32).collect(), (smash::hash40("param_special_lw"), smash::hash40("special_lw_hit_damage_mul"), 0.2));
	 crate::param_cache::update_float_2(*FIGHTER_KIND_GAOGAEN, get_marked_costumes("gaogaen","gaogaen").into_iter().map(|x| x as i32).collect(), (smash::hash40("param_special_lw"), smash::hash40("special_lw_revenge_rate_max"), 1.5));
	 crate::param_cache::update_float_2(*FIGHTER_KIND_GAOGAEN, get_marked_costumes("gaogaen","gaogaen").into_iter().map(|x| x as i32).collect(), (smash::hash40("param_special_lw"), smash::hash40("special_lw_revenge_time"), 120.0));
	 crate::param_cache::update_float_2(*FIGHTER_KIND_GAOGAEN, get_marked_costumes("gaogaen","gaogaen").into_iter().map(|x| x as i32).collect(), (smash::hash40("dash_speed"), 0, 1.8));
	 crate::param_cache::update_float_2(*FIGHTER_KIND_GAOGAEN, get_marked_costumes("gaogaen","gaogaen").into_iter().map(|x| x as i32).collect(), (smash::hash40("run_speed_max"), 0, 1.3));
	 crate::param_cache::update_float_2(*FIGHTER_KIND_GAOGAEN, get_marked_costumes("gaogaen","gaogaen").into_iter().map(|x| x as i32).collect(), (smash::hash40("air_speed_x_stable"), 0, 0.9875));
	 crate::param_cache::update_float_2(*FIGHTER_KIND_GAOGAEN, get_marked_costumes("gaogaen","gaogaen").into_iter().map(|x| x as i32).collect(), (smash::hash40("landing_attack_air_frame_f"), 0, 11.0));
	 crate::param_cache::update_float_2(*FIGHTER_KIND_GAOGAEN, get_marked_costumes("gaogaen","gaogaen").into_iter().map(|x| x as i32).collect(), (smash::hash40("landing_attack_air_frame_b"), 0, 8.0));

}