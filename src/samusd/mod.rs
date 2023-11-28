mod status;
mod frame;
mod acmd;

static mut HOLD : [i32; 8] = [0; 8];
static mut IS_HOLD : [bool; 8] = [false; 8];
static mut END : [bool; 8] = [false; 8];
static mut HOLD_MAX : i32 = 240;
static mut COOLDOWN : [i32; 8] = [0; 8];
static mut IS_ALLOWED : [bool; 8] = [true; 8];
static mut HOLD_COOLDOWN : i32 = 120;
static mut HANDS :  smash::phx::Vector3f =  smash::phx::Vector3f { x: 0.0, y: 0.0, z: 0.0 };

			


pub fn install() {
	frame::install();
	status::install();
	acmd::install();
}
