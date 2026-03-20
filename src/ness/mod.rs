mod status;
mod frame;
mod acmd;
use crate::util::*;
use smash::lib::lua_const::*;
use smash::hash40;

static mut STATIC_MUT : [i32; 8] = [6; 8];



pub fn install() {
	frame::install();
	status::install();
	acmd::install();

	param_config::update_int_2(-*FIGHTER_KIND_NESS, get_marked_costumes("ness","ness").into_iter().map(|x| x as i32).collect(), (smash::hash40("param_pkfire"), smash::hash40("pillar_life"), 30));
	param_config::update_float_2(*FIGHTER_KIND_NESS, get_marked_costumes("ness","ness").into_iter().map(|x| x as i32).collect(), (smash::hash40("ground_brake"), 0, 0.12));
	param_config::update_float_2(*FIGHTER_KIND_NESS, get_marked_costumes("ness","ness").into_iter().map(|x| x as i32).collect(), (smash::hash40("jump_speed_x_mul"), 0, 0.875));
	param_config::update_float_2(*FIGHTER_KIND_NESS, get_marked_costumes("ness","ness").into_iter().map(|x| x as i32).collect(), (smash::hash40("dive_speed_y"), 0, 2.47));
	param_config::update_float_2(*FIGHTER_KIND_NESS, get_marked_costumes("ness","ness").into_iter().map(|x| x as i32).collect(), (smash::hash40("landing_attack_air_frame_n"), 0, 7.0));
	param_config::update_float_2(*FIGHTER_KIND_NESS, get_marked_costumes("ness","ness").into_iter().map(|x| x as i32).collect(), (smash::hash40("landing_attack_air_frame_f"), 0, 8.0));
	param_config::update_int_2(*FIGHTER_KIND_NESS, get_marked_costumes("ness","ness").into_iter().map(|x| x as i32).collect(), (smash::hash40("wall_jump_type"), 0, 1));

}
