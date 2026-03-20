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

	param_config::update_float_2(-*FIGHTER_KIND_PEACH, get_marked_costumes("peach","peach").into_iter().map(|x| x as i32).collect(), (smash::hash40("param_peachdaikon"), smash::hash40("daikon_life_frame"), 120.0));
	param_config::update_float_2(-*FIGHTER_KIND_PEACH, get_marked_costumes("peach","peach").into_iter().map(|x| x as i32).collect(), (smash::hash40("param_peachdaikon"), smash::hash40("daikon_1_power"), 6.0));
	param_config::update_float_2(-*FIGHTER_KIND_PEACH, get_marked_costumes("peach","peach").into_iter().map(|x| x as i32).collect(), (smash::hash40("param_peachdaikon"), smash::hash40("daikon_2_power"), 6.0));
	param_config::update_float_2(-*FIGHTER_KIND_PEACH, get_marked_costumes("peach","peach").into_iter().map(|x| x as i32).collect(), (smash::hash40("param_peachdaikon"), smash::hash40("daikon_3_power"), 6.0));
	param_config::update_float_2(-*FIGHTER_KIND_PEACH, get_marked_costumes("peach","peach").into_iter().map(|x| x as i32).collect(), (smash::hash40("param_peachdaikon"), smash::hash40("daikon_4_power"), 6.0));
	param_config::update_float_2(-*FIGHTER_KIND_PEACH, get_marked_costumes("peach","peach").into_iter().map(|x| x as i32).collect(), (smash::hash40("param_peachdaikon"), smash::hash40("daikon_5_power"), 6.0));
	param_config::update_int_2(*FIGHTER_KIND_PEACH, get_marked_costumes("peach","peach").into_iter().map(|x| x as i32).collect(), (smash::hash40("param_private"), smash::hash40("uniq_float_float_frame"), 120));
	param_config::update_float_2(*FIGHTER_KIND_PEACH, get_marked_costumes("peach","peach").into_iter().map(|x| x as i32).collect(), (smash::hash40("air_accel_x_mul"), 0, 0.06));
	param_config::update_float_2(*FIGHTER_KIND_PEACH, get_marked_costumes("peach","peach").into_iter().map(|x| x as i32).collect(), (smash::hash40("landing_attack_air_frame_n"), 0, 7.0));

}