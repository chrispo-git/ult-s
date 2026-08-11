	
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


	 crate::param_cache::update_float_2(*FIGHTER_KIND_LUIGI, get_marked_costumes("luigi","luigi").into_iter().map(|x| x as i32).collect(), (smash::hash40("param_special_lw"), smash::hash40("buoyancy_max"), 1.4));
	 crate::param_cache::update_float_2(*FIGHTER_KIND_LUIGI, get_marked_costumes("luigi","luigi").into_iter().map(|x| x as i32).collect(), (smash::hash40("ground_brake"), 0, 0.04));
	 crate::param_cache::update_float_2(*FIGHTER_KIND_LUIGI, get_marked_costumes("luigi","luigi").into_iter().map(|x| x as i32).collect(), (smash::hash40("jump_speed_x_mul"), 0, 1.0));
	 crate::param_cache::update_float_2(*FIGHTER_KIND_LUIGI, get_marked_costumes("luigi","luigi").into_iter().map(|x| x as i32).collect(), (smash::hash40("air_speed_x_stable"), 0, 1.08));
	 crate::param_cache::update_float_2(*FIGHTER_KIND_LUIGI, get_marked_costumes("luigi","luigi").into_iter().map(|x| x as i32).collect(), (smash::hash40("dive_speed_y"), 0, 2.508));
	crate::param_cache::update_int_2(*FIGHTER_KIND_LUIGI, get_marked_costumes("luigi","luigi").into_iter().map(|x| x as i32).collect(), (smash::hash40("wall_jump_type"), 0, 1));

}
