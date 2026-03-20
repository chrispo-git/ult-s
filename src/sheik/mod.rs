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

	param_config::update_int_2(*FIGHTER_KIND_SHEIK, get_marked_costumes("sheik","sheik").into_iter().map(|x| x as i32).collect(), (smash::hash40("param_special_n"), smash::hash40("landing_frame"), 0));
	param_config::update_float_2(*FIGHTER_KIND_SHEIK, get_marked_costumes("sheik","sheik").into_iter().map(|x| x as i32).collect(), (smash::hash40("param_special_n"), smash::hash40("charge_speed_mul"), 1.3));
	param_config::update_int_2(-*FIGHTER_KIND_SHEIK, get_marked_costumes("sheik","sheik").into_iter().map(|x| x as i32).collect(), (smash::hash40("param_needle"), smash::hash40("life"), 11));
	param_config::update_float_2(*FIGHTER_KIND_SHEIK, get_marked_costumes("sheik","sheik").into_iter().map(|x| x as i32).collect(), (smash::hash40("air_speed_x_stable"), 0, 1.2));
	param_config::update_float_2(*FIGHTER_KIND_SHEIK, get_marked_costumes("sheik","sheik").into_iter().map(|x| x as i32).collect(), (smash::hash40("weight"), 0, 80.0));
	param_config::update_float_2(*FIGHTER_KIND_SHEIK, get_marked_costumes("sheik","sheik").into_iter().map(|x| x as i32).collect(), (smash::hash40("landing_attack_air_frame_b"), 0, 7.0));
	param_config::update_float_2(*FIGHTER_KIND_SHEIK, get_marked_costumes("sheik","sheik").into_iter().map(|x| x as i32).collect(), (smash::hash40("landing_attack_air_frame_lw"), 0, 10.0));

}