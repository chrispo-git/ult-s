	
mod status;
mod frame;
mod acmd;
use crate::util::*;


pub fn install() {
	frame::install();
	status::install();
	acmd::install();

	param_config::update_int_2(*FIGHTER_KIND_LITTLEMAC, get_marked_costumes("littlemac","littlemac"), (smash::hash40("param_special_n"), smash::hash40("special_n_active_start_frame"), 13));
	param_config::update_int_2(*FIGHTER_KIND_LITTLEMAC, get_marked_costumes("littlemac","littlemac"), (smash::hash40("param_special_n"), smash::hash40("special_n_cancel_frame"), 20));
	param_config::update_float_2(*FIGHTER_KIND_LITTLEMAC, get_marked_costumes("littlemac","littlemac"), (smash::hash40("param_special_hi"), smash::hash40("special_air_hi_pass_mul"), 1.2));
	param_config::update_float_2(*FIGHTER_KIND_LITTLEMAC, get_marked_costumes("littlemac","littlemac"), (smash::hash40("jump_speed_x_mul"), 0, 1.0));
	param_config::update_float_2(*FIGHTER_KIND_LITTLEMAC, get_marked_costumes("littlemac","littlemac"), (smash::hash40("jump_y"), 0, 30.0));
	param_config::update_float_2(*FIGHTER_KIND_LITTLEMAC, get_marked_costumes("littlemac","littlemac"), (smash::hash40("mini_jump_y"), 0, 14.0));
	param_config::update_float_2(*FIGHTER_KIND_LITTLEMAC, get_marked_costumes("littlemac","littlemac"), (smash::hash40("jump_aerial_y"), 0, 30.0));
	param_config::update_float_2(*FIGHTER_KIND_LITTLEMAC, get_marked_costumes("littlemac","littlemac"), (smash::hash40("landing_attack_air_frame_n"), 0, 5.0));
	param_config::update_float_2(*FIGHTER_KIND_LITTLEMAC, get_marked_costumes("littlemac","littlemac"), (smash::hash40("landing_attack_air_frame_f"), 0, 9.0));
	param_config::update_float_2(*FIGHTER_KIND_LITTLEMAC, get_marked_costumes("littlemac","littlemac"), (smash::hash40("landing_attack_air_frame_b"), 0, 9.0));

}
