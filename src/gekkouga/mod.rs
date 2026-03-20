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

	param_config::update_float_2(*FIGHTER_KIND_GEKKOUGA, get_marked_costumes("gekkouga","gekkouga"), (smash::hash40("param_special_s"), smash::hash40("warp_distance_max"), 90.0));
	param_config::update_float_2(*FIGHTER_KIND_GEKKOUGA, get_marked_costumes("gekkouga","gekkouga"), (smash::hash40("param_special_s"), smash::hash40("life"), 130.0));
	param_config::update_float_2(*FIGHTER_KIND_GEKKOUGA, get_marked_costumes("gekkouga","gekkouga"), (smash::hash40("param_special_hi"), smash::hash40("landing_frame"), 13.0));
	param_config::update_float_2(*FIGHTER_KIND_GEKKOUGA, get_marked_costumes("gekkouga","gekkouga"), (smash::hash40("ground_brake"), 0, 0.0725));
	param_config::update_float_2(*FIGHTER_KIND_GEKKOUGA, get_marked_costumes("gekkouga","gekkouga"), (smash::hash40("run_speed_max"), 0, 2.225));
	param_config::update_float_2(*FIGHTER_KIND_GEKKOUGA, get_marked_costumes("gekkouga","gekkouga"), (smash::hash40("mini_jump_y"), 0, 15.0));
	param_config::update_float_2(*FIGHTER_KIND_GEKKOUGA, get_marked_costumes("gekkouga","gekkouga"), (smash::hash40("weight"), 0, 84.0));
	param_config::update_float_2(*FIGHTER_KIND_GEKKOUGA, get_marked_costumes("gekkouga","gekkouga"), (smash::hash40("landing_attack_air_frame_n"), 0, 9.0));

}