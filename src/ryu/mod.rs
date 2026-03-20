mod status;
mod frame;
mod acmd;
use crate::util::*;
use smash::lib::lua_const::*;
use smash::hash40;


static mut max_meter : i32 = 1;
static mut PPP: i32 = 3;
static mut RYU_SUPER : [i32; 8] = [0; 8];
static mut HAS_ADDED : [bool; 8] = [false; 8];
static mut RYU_FX_TIMER : [i32; 8] = [0; 8];
static mut IS_SUPER : [bool; 8] = [false; 8];
static mut HANDS :  smash::phx::Vector3f =  smash::phx::Vector3f { x: 0.0, y: 0.0, z: 0.0 };
static mut FEET :  smash::phx::Vector3f =  smash::phx::Vector3f { x: 1.0, y: 0.0, z: 0.0 };
			


pub fn install() {
	frame::install();
	status::install();
	acmd::install();

	param_config::update_float_2(-*FIGHTER_KIND_RYU, get_marked_costumes("ryu","ryu").into_iter().map(|x| x as i32).collect(), (smash::hash40("param_hadoken"), smash::hash40("command_power_mul"), 1.25));
	param_config::update_float_2(*FIGHTER_KIND_RYU, get_marked_costumes("ryu","ryu").into_iter().map(|x| x as i32).collect(), (smash::hash40("param_special_s"), smash::hash40("air_speed_x_w"), 1.3));
	param_config::update_float_2(*FIGHTER_KIND_RYU, get_marked_costumes("ryu","ryu").into_iter().map(|x| x as i32).collect(), (smash::hash40("param_special_s"), smash::hash40("air_speed_x_m"), 0.9));
	param_config::update_float_2(*FIGHTER_KIND_RYU, get_marked_costumes("ryu","ryu").into_iter().map(|x| x as i32).collect(), (smash::hash40("param_special_s"), smash::hash40("air_speed_x_s"), 1.1));
	param_config::update_int_2(*FIGHTER_KIND_RYU, get_marked_costumes("ryu","ryu").into_iter().map(|x| x as i32).collect(), (smash::hash40("param_special_s"), smash::hash40("air_end_weak_frame_w"), 29));
	param_config::update_int_2(*FIGHTER_KIND_RYU, get_marked_costumes("ryu","ryu").into_iter().map(|x| x as i32).collect(), (smash::hash40("param_special_s"), smash::hash40("air_end_weak_frame_m"), 29));
	param_config::update_int_2(*FIGHTER_KIND_RYU, get_marked_costumes("ryu","ryu").into_iter().map(|x| x as i32).collect(), (smash::hash40("param_special_s"), smash::hash40("air_end_weak_frame_s"), 29));
	param_config::update_float_2(*FIGHTER_KIND_RYU, get_marked_costumes("ryu","ryu").into_iter().map(|x| x as i32).collect(), (smash::hash40("param_private"), smash::hash40("near_opponent_range_x"), 0.0));
	param_config::update_float_2(*FIGHTER_KIND_RYU, get_marked_costumes("ryu","ryu").into_iter().map(|x| x as i32).collect(), (smash::hash40("air_speed_x_stable"), 0, 1.1));
	param_config::update_float_2(*FIGHTER_KIND_RYU, get_marked_costumes("ryu","ryu").into_iter().map(|x| x as i32).collect(), (smash::hash40("landing_attack_air_frame_n"), 0, 6.0));

}