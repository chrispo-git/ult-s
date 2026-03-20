mod status;
mod frame;
mod acmd;
use crate::util::*;

static mut COUNTER_IS : [bool; 8] = [false; 8];
static mut BAN_DOWNB : [bool; 8] = [false; 8];
static mut ESK_CHARGE : [i32; 8] = [0; 8];
static mut ESK :  smash::phx::Vector3f =  smash::phx::Vector3f { x: 0.0, y: 0.0, z: 0.0 };




pub fn install() {
	frame::install();
	status::install();
	acmd::install();

	param_config::update_float_2(*FIGHTER_KIND_MIIFIGHTER, [0,1,2,3,4,5,6,7,8,9,10,11,12].to_vec(), (smash::hash40("param_special_lw"), smash::hash40("lw2_landing_brake_x"), 0.2));
	param_config::update_float_2(*FIGHTER_KIND_MIIFIGHTER, [0,1,2,3,4,5,6,7,8,9,10,11,12].to_vec(), (smash::hash40("ground_brake"), 0, 0.125));
	param_config::update_float_2(*FIGHTER_KIND_MIIFIGHTER, [0,1,2,3,4,5,6,7,8,9,10,11,12].to_vec(), (smash::hash40("dash_speed"), 0, 2.12));
	param_config::update_float_2(*FIGHTER_KIND_MIIFIGHTER, [0,1,2,3,4,5,6,7,8,9,10,11,12].to_vec(), (smash::hash40("jump_speed_x_mul"), 0, 1.3));
	param_config::update_float_2(*FIGHTER_KIND_MIIFIGHTER, [0,1,2,3,4,5,6,7,8,9,10,11,12].to_vec(), (smash::hash40("air_accel_x_mul"), 0, 0.045));
	param_config::update_float_2(*FIGHTER_KIND_MIIFIGHTER, [0,1,2,3,4,5,6,7,8,9,10,11,12].to_vec(), (smash::hash40("air_speed_x_stable"), 0, 1.2));
	param_config::update_float_2(*FIGHTER_KIND_MIIFIGHTER, [0,1,2,3,4,5,6,7,8,9,10,11,12].to_vec(), (smash::hash40("weight"), 0, 90.0));
	param_config::update_float_2(*FIGHTER_KIND_MIIFIGHTER, [0,1,2,3,4,5,6,7,8,9,10,11,12].to_vec(), (smash::hash40("landing_attack_air_frame_f"), 0, 10.0));
	param_config::update_float_2(*FIGHTER_KIND_MIIFIGHTER, [0,1,2,3,4,5,6,7,8,9,10,11,12].to_vec(), (smash::hash40("landing_attack_air_frame_b"), 0, 8.0));
	param_config::update_float_2(*FIGHTER_KIND_MIIFIGHTER, [0,1,2,3,4,5,6,7,8,9,10,11,12].to_vec(), (smash::hash40("landing_attack_air_frame_lw"), 0, 26.0));

}