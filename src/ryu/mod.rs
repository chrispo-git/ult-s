mod status;
mod frame;
mod acmd;
use crate::util::*;
use smash::lib::lua_const::*;
use smash::hash40;


static mut max_meter : i32 = 1;
static mut PPP: i32 = 3;
static FIGHTER_RYU_INSTANCE_WORK_ID_INT_RYU_SUPER : i32 = 0;
static FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_HAS_ADDED : i32 = 1;
static FIGHTER_RYU_INSTANCE_WORK_ID_INT_RYU_FX_TIMER : i32 = 2;
static FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_IS_SUPER : i32 = 3;
static mut HANDS :  smash::phx::Vector3f =  smash::phx::Vector3f { x: 0.0, y: 0.0, z: 0.0 };
static mut FEET :  smash::phx::Vector3f =  smash::phx::Vector3f { x: 1.0, y: 0.0, z: 0.0 };
			


pub fn install() {
	frame::install();
	status::install();
	acmd::install();

	 crate::param_cache::update_float_2(-*WEAPON_KIND_RYU_HADOKEN, get_marked_costumes("ryu","ryu").into_iter().map(|x| x as i32).collect(), (smash::hash40("param_hadoken"), smash::hash40("command_power_mul"), 1.25));
	 crate::param_cache::update_float_2(*FIGHTER_KIND_RYU, get_marked_costumes("ryu","ryu").into_iter().map(|x| x as i32).collect(), (smash::hash40("param_special_s"), smash::hash40("air_speed_x_w"), 1.3));
	 crate::param_cache::update_float_2(*FIGHTER_KIND_RYU, get_marked_costumes("ryu","ryu").into_iter().map(|x| x as i32).collect(), (smash::hash40("param_special_s"), smash::hash40("air_speed_x_m"), 0.9));
	 crate::param_cache::update_float_2(*FIGHTER_KIND_RYU, get_marked_costumes("ryu","ryu").into_iter().map(|x| x as i32).collect(), (smash::hash40("param_special_s"), smash::hash40("air_speed_x_s"), 1.1));
	crate::param_cache::update_int_2(*FIGHTER_KIND_RYU, get_marked_costumes("ryu","ryu").into_iter().map(|x| x as i32).collect(), (smash::hash40("param_special_s"), smash::hash40("air_end_weak_frame_w"), 29));
	crate::param_cache::update_int_2(*FIGHTER_KIND_RYU, get_marked_costumes("ryu","ryu").into_iter().map(|x| x as i32).collect(), (smash::hash40("param_special_s"), smash::hash40("air_end_weak_frame_m"), 29));
	crate::param_cache::update_int_2(*FIGHTER_KIND_RYU, get_marked_costumes("ryu","ryu").into_iter().map(|x| x as i32).collect(), (smash::hash40("param_special_s"), smash::hash40("air_end_weak_frame_s"), 29));
	 crate::param_cache::update_float_2(*FIGHTER_KIND_RYU, get_marked_costumes("ryu","ryu").into_iter().map(|x| x as i32).collect(), (smash::hash40("param_private"), smash::hash40("near_opponent_range_x"), 0.0));
	 crate::param_cache::update_float_2(*FIGHTER_KIND_RYU, get_marked_costumes("ryu","ryu").into_iter().map(|x| x as i32).collect(), (smash::hash40("air_speed_x_stable"), 0, 1.1));
	 crate::param_cache::update_float_2(*FIGHTER_KIND_RYU, get_marked_costumes("ryu","ryu").into_iter().map(|x| x as i32).collect(), (smash::hash40("landing_attack_air_frame_n"), 0, 6.0));

}