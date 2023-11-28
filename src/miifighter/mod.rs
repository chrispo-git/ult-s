mod status;
mod frame;
mod acmd;

static mut COUNTER_IS : [bool; 8] = [false; 8];
static mut BAN_DOWNB : [bool; 8] = [false; 8];
static mut ESK_CHARGE : [i32; 8] = [0; 8];
static mut ESK :  smash::phx::Vector3f =  smash::phx::Vector3f { x: 0.0, y: 0.0, z: 0.0 };




pub fn install() {
	frame::install();
	status::install();
	acmd::install();
}