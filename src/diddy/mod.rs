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

	param_config::update_float_2(*FIGHTER_KIND_DIDDY, get_marked_costumes("diddy","diddy").into_iter().map(|x| x as i32).collect(), (smash::hash40("param_special_n"), smash::hash40("special_n_charge_start_spd_x"), 2.75));
	param_config::update_float_2(*FIGHTER_KIND_DIDDY, get_marked_costumes("diddy","diddy").into_iter().map(|x| x as i32).collect(), (smash::hash40("param_special_n"), smash::hash40("special_n_charge_end_spd_x"), 3.5));
	param_config::update_float_2(*FIGHTER_KIND_DIDDY, get_marked_costumes("diddy","diddy").into_iter().map(|x| x as i32).collect(), (smash::hash40("jump_speed_x_mul"), 0, 1.075));
	param_config::update_float_2(*FIGHTER_KIND_DIDDY, get_marked_costumes("diddy","diddy").into_iter().map(|x| x as i32).collect(), (smash::hash40("clip_sphere_offset_x"), 0, 0.0));
	param_config::update_float_2(*FIGHTER_KIND_DIDDY, get_marked_costumes("diddy","diddy").into_iter().map(|x| x as i32).collect(), (smash::hash40("clip_sphere_offset_y"), 0, 0.0));

}