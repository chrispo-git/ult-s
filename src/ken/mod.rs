mod status;
mod frame;
mod acmd;
use crate::util::*;
use smash::lib::lua_const::*;
use smash::hash40;


static mut KEN_MAX_METER : i32 = 44;
static mut PPP: i32 = 3;
static FIGHTER_KEN_INSTANCE_WORK_ID_INT_KEN_SUPER : i32 = 0;
static FIGHTER_KEN_INSTANCE_WORK_ID_FLAG_KEN_IS_EX : i32 = 1;
static FIGHTER_KEN_INSTANCE_WORK_ID_FLAG_HAS_ADDED : i32 = 2;
static FIGHTER_KEN_INSTANCE_WORK_ID_INT_KEN_FX_TIMER : i32 = 3;
static FIGHTER_KEN_INSTANCE_WORK_ID_FLAG_IS_SUPER : i32 = 4;
static FIGHTER_KEN_INSTANCE_WORK_ID_FLAG_EX_DOWNB : i32 = 5;
static mut HANDS :  smash::phx::Vector3f =  smash::phx::Vector3f { x: 0.0, y: 0.0, z: 0.0 };
static mut FEET :  smash::phx::Vector3f =  smash::phx::Vector3f { x: 1.0, y: 0.0, z: 0.0 };


pub fn install() {
	frame::install();
	status::install();
	acmd::install();

	 crate::param_cache::update_float_2(*FIGHTER_KIND_KEN, get_marked_costumes("ken","ken").into_iter().map(|x| x as i32).collect(), (smash::hash40("param_special_lw"), smash::hash40("step_cancel_invalid_frame"), 999.0));
	 crate::param_cache::update_float_2(*FIGHTER_KIND_KEN, get_marked_costumes("ken","ken").into_iter().map(|x| x as i32).collect(), (smash::hash40("dash_speed"), 0, 1.85));
	 crate::param_cache::update_float_2(*FIGHTER_KIND_KEN, get_marked_costumes("ken","ken").into_iter().map(|x| x as i32).collect(), (smash::hash40("run_speed_max"), 0, 1.68));
	 crate::param_cache::update_float_2(*FIGHTER_KIND_KEN, get_marked_costumes("ken","ken").into_iter().map(|x| x as i32).collect(), (smash::hash40("air_speed_x_stable"), 0, 1.1));
	 crate::param_cache::update_float_2(*FIGHTER_KIND_KEN, get_marked_costumes("ken","ken").into_iter().map(|x| x as i32).collect(), (smash::hash40("landing_attack_air_frame_n"), 0, 6.0));

}
