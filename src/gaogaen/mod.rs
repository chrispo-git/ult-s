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

	param_config::update_float_2(*FIGHTER_KIND_GAOGAEN, get_marked_costumes("gaogaen","gaogaen"), (smash::hash40("param_special_lw"), smash::hash40("special_lw_hit_damage_mul"), 0.2));
	param_config::update_float_2(*FIGHTER_KIND_GAOGAEN, get_marked_costumes("gaogaen","gaogaen"), (smash::hash40("param_special_lw"), smash::hash40("special_lw_revenge_rate_max"), 1.5));
	param_config::update_float_2(*FIGHTER_KIND_GAOGAEN, get_marked_costumes("gaogaen","gaogaen"), (smash::hash40("param_special_lw"), smash::hash40("special_lw_revenge_time"), 120.0));
	param_config::update_float_2(*FIGHTER_KIND_GAOGAEN, get_marked_costumes("gaogaen","gaogaen"), (smash::hash40("dash_speed"), 0, 1.8));
	param_config::update_float_2(*FIGHTER_KIND_GAOGAEN, get_marked_costumes("gaogaen","gaogaen"), (smash::hash40("run_speed_max"), 0, 1.3));
	param_config::update_float_2(*FIGHTER_KIND_GAOGAEN, get_marked_costumes("gaogaen","gaogaen"), (smash::hash40("air_speed_x_stable"), 0, 0.9875));
	param_config::update_float_2(*FIGHTER_KIND_GAOGAEN, get_marked_costumes("gaogaen","gaogaen"), (smash::hash40("landing_attack_air_frame_f"), 0, 11.0));
	param_config::update_float_2(*FIGHTER_KIND_GAOGAEN, get_marked_costumes("gaogaen","gaogaen"), (smash::hash40("landing_attack_air_frame_b"), 0, 8.0));

}