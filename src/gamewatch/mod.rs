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

	param_config::update_float_2(*FIGHTER_KIND_GAMEWATCH, get_marked_costumes("gamewatch","gamewatch").into_iter().map(|x| x as i32).collect(), (smash::hash40("dash_speed"), 0, 2.1));
	param_config::update_float_2(*FIGHTER_KIND_GAMEWATCH, get_marked_costumes("gamewatch","gamewatch").into_iter().map(|x| x as i32).collect(), (smash::hash40("jump_speed_x_mul"), 0, 0.975));
	param_config::update_float_2(*FIGHTER_KIND_GAMEWATCH, get_marked_costumes("gamewatch","gamewatch").into_iter().map(|x| x as i32).collect(), (smash::hash40("landing_attack_air_frame_n"), 0, 9.0));
	param_config::update_float_2(*FIGHTER_KIND_GAMEWATCH, get_marked_costumes("gamewatch","gamewatch").into_iter().map(|x| x as i32).collect(), (smash::hash40("landing_attack_air_frame_f"), 0, 12.0));
	param_config::update_float_2(*FIGHTER_KIND_GAMEWATCH, get_marked_costumes("gamewatch","gamewatch").into_iter().map(|x| x as i32).collect(), (smash::hash40("landing_attack_air_frame_b"), 0, 15.0));
	param_config::update_float_2(*FIGHTER_KIND_GAMEWATCH, get_marked_costumes("gamewatch","gamewatch").into_iter().map(|x| x as i32).collect(), (smash::hash40("landing_attack_air_frame_lw"), 0, 10.0));
	param_config::update_float_2(*FIGHTER_KIND_GAMEWATCH, get_marked_costumes("gamewatch","gamewatch").into_iter().map(|x| x as i32).collect(), (smash::hash40("scale"), 0, 0.89));

}