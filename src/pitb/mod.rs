mod status;
mod frame;
mod acmd;
use crate::util::*;
use smash::lib::lua_const::*;
use smash::hash40;

//Float Stuff
static mut CHECK_FLOAT : [i32; 8] = [0; 8];
static mut CHECK_FLOAT_MAX : i32 = 14; //Frames where jump needs to be held to start floating
static mut FLOAT_FALLSPEED : f32 = 0.365;
static mut HITLAG_MULT : f32 = 0.45;
			


pub fn install() {
	frame::install();
	status::install();
	acmd::install();
	param_config::update_float_2(-*WEAPON_KIND_PITB_BOWARROW, get_marked_costumes("pitb","pitb").into_iter().map(|x| x as i32).collect(), (smash::hash40("param_bowarrow"), smash::hash40("control_angle"), 1.2));
	param_config::update_float_2(*FIGHTER_KIND_PITB, get_marked_costumes("pitb","pitb").into_iter().map(|x| x as i32).collect(), (smash::hash40("ground_brake"), 0, 0.17));
	param_config::update_float_2(*FIGHTER_KIND_PITB, get_marked_costumes("pitb","pitb").into_iter().map(|x| x as i32).collect(), (smash::hash40("dash_speed"), 0, 2.12));
	param_config::update_float_2(*FIGHTER_KIND_PITB, get_marked_costumes("pitb","pitb").into_iter().map(|x| x as i32).collect(), (smash::hash40("run_speed_max"), 0, 1.9));
	param_config::update_float_2(*FIGHTER_KIND_PITB, get_marked_costumes("pitb","pitb").into_iter().map(|x| x as i32).collect(), (smash::hash40("jump_y"), 0, 33.0));
	param_config::update_int_2(*FIGHTER_KIND_PITB, get_marked_costumes("pitb","pitb").into_iter().map(|x| x as i32).collect(), (smash::hash40("jump_count_max"), 0, 2));
	param_config::update_float_2(*FIGHTER_KIND_PITB, get_marked_costumes("pitb","pitb").into_iter().map(|x| x as i32).collect(), (smash::hash40("jump_aerial_y"), 0, 33.0));
	param_config::update_float_2(*FIGHTER_KIND_PITB, get_marked_costumes("pitb","pitb").into_iter().map(|x| x as i32).collect(), (smash::hash40("air_speed_x_stable"), 0, 1.1));
	param_config::update_float_2(*FIGHTER_KIND_PITB, get_marked_costumes("pitb","pitb").into_iter().map(|x| x as i32).collect(), (smash::hash40("air_accel_y"), 0, 0.112));
	param_config::update_float_2(*FIGHTER_KIND_PITB, get_marked_costumes("pitb","pitb").into_iter().map(|x| x as i32).collect(), (smash::hash40("air_speed_y_stable"), 0, 1.92));
	param_config::update_float_2(*FIGHTER_KIND_PITB, get_marked_costumes("pitb","pitb").into_iter().map(|x| x as i32).collect(), (smash::hash40("dive_speed_y"), 0, 2.88));
	param_config::update_float_2(*FIGHTER_KIND_PITB, get_marked_costumes("pitb","pitb").into_iter().map(|x| x as i32).collect(), (smash::hash40("weight"), 0, 90.0));
	param_config::update_float_2(*FIGHTER_KIND_PITB, get_marked_costumes("pitb","pitb").into_iter().map(|x| x as i32).collect(), (smash::hash40("landing_attack_air_frame_n"), 0, 7.0));
	param_config::update_float_2(*FIGHTER_KIND_PITB, get_marked_costumes("pitb","pitb").into_iter().map(|x| x as i32).collect(), (smash::hash40("landing_attack_air_frame_f"), 0, 9.0));
	param_config::update_float_2(*FIGHTER_KIND_PITB, get_marked_costumes("pitb","pitb").into_iter().map(|x| x as i32).collect(), (smash::hash40("landing_attack_air_frame_hi"), 0, 8.0));
	param_config::update_float_2(*FIGHTER_KIND_PITB, get_marked_costumes("pitb","pitb").into_iter().map(|x| x as i32).collect(), (smash::hash40("landing_frame"), 0, 3.0));

}