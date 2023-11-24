	
mod status;
mod frame;
mod acmd;

static mut NONE :  smash::phx::Vector3f =  smash::phx::Vector3f { x: 0.0, y: 0.0, z: 0.0 };
static mut SHOOT :  smash::phx::Vector3f =  smash::phx::Vector3f { x: 0.0, y: 10.0, z: 20.0 };



pub fn install() {
	frame::install();
	status::install();
	acmd::install();
}
