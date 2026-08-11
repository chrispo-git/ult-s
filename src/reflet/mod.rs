mod status;
mod frame;
mod acmd;
use crate::util::*;
use smash::lib::lua_const::*;
use smash::hash40;

//Grima Install
static FIGHTER_REFLET_INSTANCE_WORK_ID_FLAG_IS_GRIMA : i32 = 0;
static FIGHTER_REFLET_INSTANCE_WORK_ID_INT_DMG_COUNTER : i32 = 1;
static mut DMG_COUNTER_MAX : i32 = 60;
static mut DMG_ADD : f32 = 0.7;
static FIGHTER_REFLET_INSTANCE_WORK_ID_INT_SPECIAL_HI_CURR : i32 = 2;
static FIGHTER_REFLET_INSTANCE_WORK_ID_INT_SPECIAL_N_CURR : i32 = 3;
static FIGHTER_REFLET_INSTANCE_WORK_ID_FLOAT_SPECIAL_S_CURR : i32 = 4;

//Float Stuff
static mut FLOAT : [i32; 8] = [0; 8]; //Logs Float Time
static FIGHTER_REFLET_INSTANCE_WORK_ID_FLAG_START_FLOAT : i32 = 5;
static FIGHTER_REFLET_INSTANCE_WORK_ID_FLAG_JUMPSQUAT_FLOAT : i32 = 6;
static FIGHTER_REFLET_INSTANCE_WORK_ID_INT_CHECK_FLOAT : i32 = 7;
static mut CHECK_FLOAT_MAX : i32 = 15; //Frames where jump needs to be held to start floating
static mut X : [f32; 8] = [0.0; 8]; //Logs speed
static mut Y : [f32; 8] = [0.0; 8]; //Logs speed
static mut FLOAT_MAX : i32 = 95; //Frames this bitch can float (In frames, 60 frames = 1 second)
static mut X_MAX : f32 = 1.208; //Max Horizontal movespeed
static mut X_ACCEL_ADD : f32 = 0.02; //Air Accel Add
static mut X_ACCEL_MUL : f32 = 0.09; //Air Accel Mul
static mut Y_MAX : f32 = 0.0; //Max Vertical movespeed
static mut Y_ACCEL_ADD : f32 = 0.06;
static mut Y_ACCEL_MUL : f32 = 0.06;

static mut HANDS :  smash::phx::Vector3f =  smash::phx::Vector3f { x: 0.0, y: 0.0, z: 0.0 };
			


pub fn install() {
	frame::install();
	status::install();
	acmd::install();

	 crate::param_cache::update_float_2(*FIGHTER_KIND_REFLET, get_marked_costumes("reflet","reflet").into_iter().map(|x| x as i32).collect(), (smash::hash40("run_speed_max"), 0, 1.4));

}