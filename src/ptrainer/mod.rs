mod status;
mod frame;
mod acmd;
use crate::util::*;
			


pub fn install() {
	frame::install();
	status::install();
	acmd::install();
	
	param_config::update_float_2(*FIGHTER_KIND_PFUSHIGISOU, get_marked_costumes("pfushigisou","pfushigisou"), (smash::hash40("param_special_s"), smash::hash40("shoot_speed"), 1.25));
	param_config::update_int_2(-*FIGHTER_KIND_PFUSHIGISOU, get_marked_costumes("pfushigisou","pfushigisou"), (smash::hash40("param_leafcutter"), smash::hash40("life"), 75));
	param_config::update_float_2(*FIGHTER_KIND_PFUSHIGISOU, get_marked_costumes("pfushigisou","pfushigisou"), (smash::hash40("landing_attack_air_frame_f"), 0, 11.0));
	param_config::update_float_2(*FIGHTER_KIND_PFUSHIGISOU, get_marked_costumes("pfushigisou","pfushigisou"), (smash::hash40("landing_attack_air_frame_hi"), 0, 16.0));
	param_config::update_float_2(*FIGHTER_KIND_PFUSHIGISOU, get_marked_costumes("pfushigisou","pfushigisou"), (smash::hash40("landing_attack_air_frame_lw"), 0, 16.0));

}