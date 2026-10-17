mod status;
mod frame;
mod acmd;
use crate::util::*;
use smash::lib::lua_const::*;
use smash::hash40;

//Float Stuff
static FIGHTER_PITB_INSTANCE_WORK_ID_INT_CHECK_FLOAT : i32 = 0;
static mut CHECK_FLOAT_MAX : i32 = 14; //Frames where jump needs to be held to start floating
static mut FLOAT_FALLSPEED : f32 = 0.365;
static mut HITLAG_MULT : f32 = 0.45;
			


pub fn install() {
	frame::install();
	status::install();
	acmd::install();
	 crate::param_cache::update_float_2(-*WEAPON_KIND_PITB_BOWARROW, get_marked_costumes("pitb","pitb").into_iter().map(|x| x as i32).collect(), (smash::hash40("param_bowarrow"), smash::hash40("control_angle"), 1.2));
	 crate::param_cache::update_float_2(*FIGHTER_KIND_PITB, get_marked_costumes("pitb","pitb").into_iter().map(|x| x as i32).collect(), (smash::hash40("ground_brake"), 0, 0.17));
	 crate::param_cache::update_float_2(*FIGHTER_KIND_PITB, get_marked_costumes("pitb","pitb").into_iter().map(|x| x as i32).collect(), (smash::hash40("dash_speed"), 0, 2.12));
	 crate::param_cache::update_float_2(*FIGHTER_KIND_PITB, get_marked_costumes("pitb","pitb").into_iter().map(|x| x as i32).collect(), (smash::hash40("run_speed_max"), 0, 1.9));
	 crate::param_cache::update_float_2(*FIGHTER_KIND_PITB, get_marked_costumes("pitb","pitb").into_iter().map(|x| x as i32).collect(), (smash::hash40("jump_y"), 0, 33.0));
	crate::param_cache::update_int_2(*FIGHTER_KIND_PITB, get_marked_costumes("pitb","pitb").into_iter().map(|x| x as i32).collect(), (smash::hash40("jump_count_max"), 0, 2));
	 crate::param_cache::update_float_2(*FIGHTER_KIND_PITB, get_marked_costumes("pitb","pitb").into_iter().map(|x| x as i32).collect(), (smash::hash40("jump_aerial_y"), 0, 33.0));
	 crate::param_cache::update_float_2(*FIGHTER_KIND_PITB, get_marked_costumes("pitb","pitb").into_iter().map(|x| x as i32).collect(), (smash::hash40("air_speed_x_stable"), 0, 1.1));
	 crate::param_cache::update_float_2(*FIGHTER_KIND_PITB, get_marked_costumes("pitb","pitb").into_iter().map(|x| x as i32).collect(), (smash::hash40("air_accel_y"), 0, 0.112));
	 crate::param_cache::update_float_2(*FIGHTER_KIND_PITB, get_marked_costumes("pitb","pitb").into_iter().map(|x| x as i32).collect(), (smash::hash40("air_speed_y_stable"), 0, 1.92));
	 crate::param_cache::update_float_2(*FIGHTER_KIND_PITB, get_marked_costumes("pitb","pitb").into_iter().map(|x| x as i32).collect(), (smash::hash40("dive_speed_y"), 0, 2.88));
	 crate::param_cache::update_float_2(*FIGHTER_KIND_PITB, get_marked_costumes("pitb","pitb").into_iter().map(|x| x as i32).collect(), (smash::hash40("weight"), 0, 90.0));
	 crate::param_cache::update_float_2(*FIGHTER_KIND_PITB, get_marked_costumes("pitb","pitb").into_iter().map(|x| x as i32).collect(), (smash::hash40("param_special_hi"), smash::hash40("rush_speed"), 2.5));
	 crate::param_cache::update_float_2(*FIGHTER_KIND_PITB, get_marked_costumes("pitb","pitb").into_iter().map(|x| x as i32).collect(), (smash::hash40("landing_attack_air_frame_n"), 0, 7.0));
	 crate::param_cache::update_float_2(*FIGHTER_KIND_PITB, get_marked_costumes("pitb","pitb").into_iter().map(|x| x as i32).collect(), (smash::hash40("landing_attack_air_frame_f"), 0, 9.0));
	 crate::param_cache::update_float_2(*FIGHTER_KIND_PITB, get_marked_costumes("pitb","pitb").into_iter().map(|x| x as i32).collect(), (smash::hash40("landing_attack_air_frame_hi"), 0, 8.0));
	 crate::param_cache::update_float_2(*FIGHTER_KIND_PITB, get_marked_costumes("pitb","pitb").into_iter().map(|x| x as i32).collect(), (smash::hash40("landing_frame"), 0, 3.0));

}