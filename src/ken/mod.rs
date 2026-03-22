mod status;
mod frame;
mod acmd;
use crate::util::*;
use smash::lib::lua_const::*;
use smash::hash40;


static mut KEN_MAX_METER : i32 = 44;
static mut PPP: i32 = 3;
static mut KEN_SUPER : [i32; 8] = [0; 8];
static mut KEN_IS_EX : [bool; 8] = [false; 8];
static mut HAS_ADDED : [bool; 8] = [false; 8];
static mut KEN_FX_TIMER : [i32; 8] = [0; 8];
static mut IS_SUPER : [bool; 8] = [false; 8];
static mut EX_DOWNB : [bool; 8] = [false; 8];
static mut HANDS :  smash::phx::Vector3f =  smash::phx::Vector3f { x: 0.0, y: 0.0, z: 0.0 };
static mut FEET :  smash::phx::Vector3f =  smash::phx::Vector3f { x: 1.0, y: 0.0, z: 0.0 };


pub fn install() {
	frame::install();
	status::install();
	acmd::install();

	param_config::update_float_2(*FIGHTER_KIND_KEN, get_marked_costumes("ken","ken").into_iter().map(|x| x as i32).collect(), (smash::hash40("param_special_lw"), smash::hash40("step_cancel_invalid_frame"), 999.0));
	param_config::update_float_2(*FIGHTER_KIND_KEN, get_marked_costumes("ken","ken").into_iter().map(|x| x as i32).collect(), (smash::hash40("dash_speed"), 0, 1.85));
	param_config::update_float_2(*FIGHTER_KIND_KEN, get_marked_costumes("ken","ken").into_iter().map(|x| x as i32).collect(), (smash::hash40("run_speed_max"), 0, 1.68));
	param_config::update_float_2(*FIGHTER_KIND_KEN, get_marked_costumes("ken","ken").into_iter().map(|x| x as i32).collect(), (smash::hash40("air_speed_x_stable"), 0, 1.1));
	param_config::update_float_2(*FIGHTER_KIND_KEN, get_marked_costumes("ken","ken").into_iter().map(|x| x as i32).collect(), (smash::hash40("landing_attack_air_frame_n"), 0, 6.0));

}
