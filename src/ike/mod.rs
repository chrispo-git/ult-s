mod status;
mod frame;
mod acmd;
use crate::util::*;
use smash::lib::lua_const::*;
use smash::hash40;

static mut IKE_INSTALL: [i32; 8] = [0; 8];
static mut IKE_INSTALL_TIME: i32 = 1800;
static mut IKE_INSTALL_EFF: [u32; 8] = [0; 8];
static mut TIMER : [i32; 8] = [0; 8];
static mut S1 :  smash::phx::Vector3f =  smash::phx::Vector3f { x: 0.0, y: 11.0, z: 0.0 };
static mut S2 :  smash::phx::Vector3f =  smash::phx::Vector3f { x: 0.0, y: 6.2, z: 0.0 };
static mut S3 :  smash::phx::Vector3f =  smash::phx::Vector3f { x: 0.0, y: 3.0, z: 0.0 };
	


pub fn install() {
	frame::install();
	status::install();
	acmd::install();

	param_config::update_float_2(*FIGHTER_KIND_IKE, get_marked_costumes("ike","ike").into_iter().map(|x| x as i32).collect(), (smash::hash40("dash_speed"), 0, 1.87));
	param_config::update_float_2(*FIGHTER_KIND_IKE, get_marked_costumes("ike","ike").into_iter().map(|x| x as i32).collect(), (smash::hash40("run_speed_max"), 0, 1.6));
	param_config::update_float_2(*FIGHTER_KIND_IKE, get_marked_costumes("ike","ike").into_iter().map(|x| x as i32).collect(), (smash::hash40("landing_attack_air_frame_n"), 0, 10.0));
	param_config::update_float_2(*FIGHTER_KIND_IKE, get_marked_costumes("ike","ike").into_iter().map(|x| x as i32).collect(), (smash::hash40("landing_attack_air_frame_f"), 0, 11.0));

}

