mod status;
mod frame;
mod acmd;
use crate::util::*;
use smash::lib::lua_const::*;
use smash::hash40;

//Float Stuff
static mut FLOAT : [i32; 8] = [0; 8]; //Logs Float Time
static FIGHTER_KOOPAJR_INSTANCE_WORK_ID_FLAG_START_FLOAT : i32 = 0;
static FIGHTER_KOOPAJR_INSTANCE_WORK_ID_FLAG_JUMPSQUAT_FLOAT : i32 = 1;
static FIGHTER_KOOPAJR_INSTANCE_WORK_ID_INT_CHECK_FLOAT : i32 = 2;
static mut CHECK_FLOAT_MAX : i32 = 15; //Frames where jump needs to be held to start floating
static mut X : [f32; 8] = [0.0; 8]; //Logs speed
static mut Y : [f32; 8] = [0.0; 8]; //Logs speed
static mut FLOAT_MAX : i32 = 70; //Frames this bitch can float (In frames, 60 frames = 1 second)
static mut X_MAX : f32 = 1.208; //Max Horizontal movespeed
static mut X_ACCEL_ADD : f32 = 0.02; //Air Accel Add
static mut X_ACCEL_MUL : f32 = 0.09; //Air Accel Mul
static mut Y_MAX : f32 = 0.0; //Max Vertical movespeed
static mut Y_ACCEL_ADD : f32 = 0.06;
static mut Y_ACCEL_MUL : f32 = 0.06;


pub fn install() {
	frame::install();
	status::install();
	acmd::install();

	 crate::param_cache::update_float_2(*FIGHTER_KIND_KOOPAJR, get_marked_costumes("koopajr","koopajr").into_iter().map(|x| x as i32).collect(), (smash::hash40("dash_speed"), 0, 1.86));
	 crate::param_cache::update_float_2(*FIGHTER_KIND_KOOPAJR, get_marked_costumes("koopajr","koopajr").into_iter().map(|x| x as i32).collect(), (smash::hash40("run_speed_max"), 0, 1.7));
	 crate::param_cache::update_float_2(*FIGHTER_KIND_KOOPAJR, get_marked_costumes("koopajr","koopajr").into_iter().map(|x| x as i32).collect(), (smash::hash40("jump_speed_x_mul"), 0, 1.1));
	 crate::param_cache::update_float_2(*FIGHTER_KIND_KOOPAJR, get_marked_costumes("koopajr","koopajr").into_iter().map(|x| x as i32).collect(), (smash::hash40("landing_attack_air_frame_f"), 0, 11.0));
	 crate::param_cache::update_float_2(*FIGHTER_KIND_KOOPAJR, get_marked_costumes("koopajr","koopajr").into_iter().map(|x| x as i32).collect(), (smash::hash40("clip_sphere_offset_x"), 0, 0.5));

}
