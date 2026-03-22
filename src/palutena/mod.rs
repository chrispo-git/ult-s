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

	param_config::update_float_2(*FIGHTER_KIND_PALUTENA, get_marked_costumes("palutena","palutena").into_iter().map(|x| x as i32).collect(), (smash::hash40("param_special_n"), smash::hash40("special_n_auto_target_deg_angle"), 0.0));
	param_config::update_float_2(*FIGHTER_KIND_PALUTENA, get_marked_costumes("palutena","palutena").into_iter().map(|x| x as i32).collect(), (smash::hash40("param_special_n"), smash::hash40("special_n_auto_target_distance"), 0.0));
	param_config::update_float_2(-*FIGHTER_KIND_PALUTENA, get_marked_costumes("palutena","palutena").into_iter().map(|x| x as i32).collect(), (smash::hash40("param_autoaimbullet"), smash::hash40("life"), 19.0));
	param_config::update_float_2(*FIGHTER_KIND_PALUTENA, get_marked_costumes("palutena","palutena").into_iter().map(|x| x as i32).collect(), (smash::hash40("air_accel_x_mul"), 0, 0.11));
	param_config::update_float_2(*FIGHTER_KIND_PALUTENA, get_marked_costumes("palutena","palutena").into_iter().map(|x| x as i32).collect(), (smash::hash40("air_speed_x_stable"), 0, 1.05));
	param_config::update_float_2(*FIGHTER_KIND_PALUTENA, get_marked_costumes("palutena","palutena").into_iter().map(|x| x as i32).collect(), (smash::hash40("landing_attack_air_frame_lw"), 0, 10.0));

}