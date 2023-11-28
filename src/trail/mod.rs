mod status;
mod frame;
mod acmd;
		
static mut HANDS :  smash::phx::Vector3f =  smash::phx::Vector3f { x: 0.0, y: 0.0, z: 0.0 };
static mut HAS_WALLJUMP: [bool; 8] = [false; 8];
static mut TO_FAIR: [bool; 8] = [false; 8];
			



pub fn install() {
	frame::install();
	status::install();
	acmd::install();
}