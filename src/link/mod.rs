	
mod status;
mod frame;
mod acmd;
use crate::util::*;
use smash::lib::lua_const::*;
use smash::hash40;

static mut NONE :  smash::phx::Vector3f =  smash::phx::Vector3f { x: 0.0, y: 0.0, z: 0.0 };
static mut SHOOT :  smash::phx::Vector3f =  smash::phx::Vector3f { x: 0.0, y: 10.0, z: 20.0 };



pub fn install() {
	frame::install();
	status::install();
	acmd::install();

	param_config::update_float_2(*FIGHTER_KIND_LINK, get_marked_costumes("link","link"), (smash::hash40("ground_brake"), 0, 0.08));
	param_config::update_float_2(*FIGHTER_KIND_LINK, get_marked_costumes("link","link"), (smash::hash40("dash_speed"), 0, 2.01));
	param_config::update_float_2(*FIGHTER_KIND_LINK, get_marked_costumes("link","link"), (smash::hash40("run_speed_max"), 0, 1.76));
	param_config::update_float_2(*FIGHTER_KIND_LINK, get_marked_costumes("link","link"), (smash::hash40("jump_speed_x_mul"), 0, 0.875));
	param_config::update_float_2(*FIGHTER_KIND_LINK, get_marked_costumes("link","link"), (smash::hash40("mini_jump_y"), 0, 12.7));
	param_config::update_float_2(*FIGHTER_KIND_LINK, get_marked_costumes("link","link"), (smash::hash40("jump_aerial_y"), 0, 26.0));
	param_config::update_float_2(*FIGHTER_KIND_LINK, get_marked_costumes("link","link"), (smash::hash40("air_speed_x_stable"), 0, 1.08));
	param_config::update_float_2(*FIGHTER_KIND_LINK, get_marked_costumes("link","link"), (smash::hash40("weight"), 0, 102.0));
	param_config::update_float_2(*FIGHTER_KIND_LINK, get_marked_costumes("link","link"), (smash::hash40("clip_sphere_offset_y"), 0, -2.0));
	param_config::update_float_2(*FIGHTER_KIND_LINK, get_marked_costumes("link","link"), (smash::hash40("landing_attack_air_frame_hi"), 0, 7.0));
	param_config::update_float_2(*FIGHTER_KIND_LINK, get_marked_costumes("link","link"), (smash::hash40("landing_attack_air_frame_f"), 0, 13.0));
	param_config::update_float_2(*FIGHTER_KIND_LINK, get_marked_costumes("link","link"), (smash::hash40("landing_attack_air_frame_n"), 0, 9.0));
	param_config::update_float_2(*FIGHTER_KIND_LINK, get_marked_costumes("link","link"), (smash::hash40("landing_frame"), 0, 3.0));

}
