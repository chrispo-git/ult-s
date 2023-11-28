mod status;
mod frame;
mod acmd;

//Float Stuff
static mut CHECK_FLOAT : [i32; 8] = [0; 8];
static mut CHECK_FLOAT_MAX : i32 = 14; //Frames where jump needs to be held to start floating
static mut FLOAT_FALLSPEED : f32 = 0.365;
static mut HITLAG_MULT : f32 = 0.45;
			


pub fn install() {
	frame::install();
	status::install();
	acmd::install();
}