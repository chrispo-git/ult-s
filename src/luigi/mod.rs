	
mod status;
mod frame;
mod acmd;
use crate::util::*;


pub fn install() {
	frame::install();
	status::install();
	acmd::install();


	param_config::update_float_2(*FIGHTER_KIND_LUIGI, get_marked_costumes("luigi","luigi"), (smash::hash40("param_special_lw"), smash::hash40("buoyancy_max"), 1.4));
	param_config::update_float_2(*FIGHTER_KIND_LUIGI, get_marked_costumes("luigi","luigi"), (smash::hash40("ground_brake"), 0, 0.04));
	param_config::update_float_2(*FIGHTER_KIND_LUIGI, get_marked_costumes("luigi","luigi"), (smash::hash40("jump_speed_x_mul"), 0, 1.0));
	param_config::update_float_2(*FIGHTER_KIND_LUIGI, get_marked_costumes("luigi","luigi"), (smash::hash40("air_speed_x_stable"), 0, 1.08));
	param_config::update_float_2(*FIGHTER_KIND_LUIGI, get_marked_costumes("luigi","luigi"), (smash::hash40("dive_speed_y"), 0, 2.508));
	param_config::update_int_2(*FIGHTER_KIND_LUIGI, get_marked_costumes("luigi","luigi"), (smash::hash40("wall_jump_type"), 0, 1));

}
