mod status;
mod frame;
mod acmd;

//Grima Install
static mut IS_GRIMA : [bool; 8] = [false; 8];
static mut DMG_COUNTER : [i32; 8] = [0; 8];
static mut DMG_COUNTER_MAX : i32 = 60;
static mut DMG_ADD : f32 = 1.5;
static mut SPECIAL_HI_CURR : [i32; 8] = [0; 8]; 
static mut SPECIAL_N_CURR : [i32; 8] = [0; 8]; 
static mut SPECIAL_S_CURR : [f32; 8] = [0.0; 8]; 

//Float Stuff
static mut FLOAT : [i32; 8] = [0; 8]; //Logs Float Time
static mut START_FLOAT : [bool; 8] = [false; 8];
static mut JUMPSQUAT_FLOAT : [bool; 8] = [false; 8];
static mut CHECK_FLOAT : [i32; 8] = [0; 8];
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
}