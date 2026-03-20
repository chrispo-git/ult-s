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

	param_config::update_float_2(*FIGHTER_KIND_EDGE, get_marked_costumes("edge","edge"), (smash::hash40("param_one_winged"), smash::hash40("dash_speed_mul"), 1.13));
	param_config::update_float_2(*FIGHTER_KIND_EDGE, get_marked_costumes("edge","edge"), (smash::hash40("param_one_winged"), smash::hash40("air_accel_x_mul"), 1.2));
	param_config::update_float_2(*FIGHTER_KIND_EDGE, get_marked_costumes("edge","edge"), (smash::hash40("param_one_winged"), smash::hash40("air_speed_x_stable_mul"), 1.0857));
	param_config::update_float_2(*FIGHTER_KIND_EDGE, get_marked_costumes("edge","edge"), (smash::hash40("air_speed_x_stable"), 0, 1.1));
	param_config::update_float_2(*FIGHTER_KIND_EDGE, get_marked_costumes("edge","edge"), (smash::hash40("landing_attack_air_frame_n"), 0, 8.0));
	param_config::update_float_2(*FIGHTER_KIND_EDGE, get_marked_costumes("edge","edge"), (smash::hash40("landing_attack_air_frame_hi"), 0, 15.0));
	param_config::update_float_2(*FIGHTER_KIND_EDGE, get_marked_costumes("edge","edge"), (smash::hash40("clip_sphere_offset_x"), 0, 0.0));
	param_config::update_float_2(*FIGHTER_KIND_EDGE, get_marked_costumes("edge","edge"), (smash::hash40("clip_sphere_offset_y"), 0, -0.7));

}