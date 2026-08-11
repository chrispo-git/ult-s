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
		
	 crate::param_cache::update_float_2(*FIGHTER_KIND_POPO, get_marked_costumes("popo","popo").into_iter().map(|x| x as i32).collect(), (smash::hash40("ground_brake"), 0, 0.0525));
	 crate::param_cache::update_float_2(*FIGHTER_KIND_POPO, get_marked_costumes("popo","popo").into_iter().map(|x| x as i32).collect(), (smash::hash40("air_speed_x_stable"), 0, 0.88));
	 crate::param_cache::update_float_2(*FIGHTER_KIND_POPO, get_marked_costumes("popo","popo").into_iter().map(|x| x as i32).collect(), (smash::hash40("weight"), 0, 86.0));
	 crate::param_cache::update_float_2(*FIGHTER_KIND_POPO, get_marked_costumes("popo","popo").into_iter().map(|x| x as i32).collect(), (smash::hash40("scale"), 0, 1.15));
	 crate::param_cache::update_float_2(*FIGHTER_KIND_POPO, get_marked_costumes("popo","popo").into_iter().map(|x| x as i32).collect(), (smash::hash40("shield_radius"), 0, 11.6));
	 crate::param_cache::update_float_2(*FIGHTER_KIND_POPO, get_marked_costumes("popo","popo").into_iter().map(|x| x as i32).collect(), (smash::hash40("ground_brake"), 0, 0.0602));
	 crate::param_cache::update_float_2(*FIGHTER_KIND_POPO, get_marked_costumes("popo","popo").into_iter().map(|x| x as i32).collect(), (smash::hash40("air_speed_x_stable"), 0, 0.9));
	 crate::param_cache::update_float_2(*FIGHTER_KIND_POPO, get_marked_costumes("popo","popo").into_iter().map(|x| x as i32).collect(), (smash::hash40("weight"), 0, 86.0));
	 crate::param_cache::update_float_2(*FIGHTER_KIND_POPO, get_marked_costumes("popo","popo").into_iter().map(|x| x as i32).collect(), (smash::hash40("scale"), 0, 1.15));
	 crate::param_cache::update_float_2(*FIGHTER_KIND_POPO, get_marked_costumes("popo","popo").into_iter().map(|x| x as i32).collect(), (smash::hash40("shield_radius"), 0, 11.6));

	 crate::param_cache::update_float_2(*FIGHTER_KIND_NANA, get_marked_costumes("popo","popo").into_iter().map(|x| x as i32).collect(), (smash::hash40("ground_brake"), 0, 0.0525));
	 crate::param_cache::update_float_2(*FIGHTER_KIND_NANA, get_marked_costumes("popo","popo").into_iter().map(|x| x as i32).collect(), (smash::hash40("air_speed_x_stable"), 0, 0.88));
	 crate::param_cache::update_float_2(*FIGHTER_KIND_NANA, get_marked_costumes("popo","popo").into_iter().map(|x| x as i32).collect(), (smash::hash40("weight"), 0, 86.0));
	 crate::param_cache::update_float_2(*FIGHTER_KIND_NANA, get_marked_costumes("popo","popo").into_iter().map(|x| x as i32).collect(), (smash::hash40("scale"), 0, 1.15));
	 crate::param_cache::update_float_2(*FIGHTER_KIND_NANA, get_marked_costumes("popo","popo").into_iter().map(|x| x as i32).collect(), (smash::hash40("shield_radius"), 0, 11.6));
	 crate::param_cache::update_float_2(*FIGHTER_KIND_NANA, get_marked_costumes("popo","popo").into_iter().map(|x| x as i32).collect(), (smash::hash40("ground_brake"), 0, 0.0602));
	 crate::param_cache::update_float_2(*FIGHTER_KIND_NANA, get_marked_costumes("popo","popo").into_iter().map(|x| x as i32).collect(), (smash::hash40("air_speed_x_stable"), 0, 0.9));
	 crate::param_cache::update_float_2(*FIGHTER_KIND_NANA, get_marked_costumes("popo","popo").into_iter().map(|x| x as i32).collect(), (smash::hash40("weight"), 0, 86.0));
	 crate::param_cache::update_float_2(*FIGHTER_KIND_NANA, get_marked_costumes("popo","popo").into_iter().map(|x| x as i32).collect(), (smash::hash40("scale"), 0, 1.15));
	 crate::param_cache::update_float_2(*FIGHTER_KIND_NANA, get_marked_costumes("popo","popo").into_iter().map(|x| x as i32).collect(), (smash::hash40("shield_radius"), 0, 11.6));
}
