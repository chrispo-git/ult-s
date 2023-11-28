mod status;
mod frame;
mod acmd;


static mut max_meter : i32 = 1;
static mut PPP: i32 = 3;
static mut RYU_SUPER : [i32; 8] = [0; 8];
static mut HAS_ADDED : [bool; 8] = [false; 8];
static mut RYU_FX_TIMER : [i32; 8] = [0; 8];
static mut IS_SUPER : [bool; 8] = [false; 8];
static mut HANDS :  smash::phx::Vector3f =  smash::phx::Vector3f { x: 0.0, y: 0.0, z: 0.0 };
static mut FEET :  smash::phx::Vector3f =  smash::phx::Vector3f { x: 1.0, y: 0.0, z: 0.0 };
			


pub fn install() {
	frame::install();
	status::install();
	acmd::install();
}