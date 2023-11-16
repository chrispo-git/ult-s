mod status;
mod frame;
mod acmd;

static mut IKE_INSTALL: [i32; 8] = [0; 8];
static mut IKE_INSTALL_TIME: i32 = 1800;
static mut IKE_INSTALL_EFF: [u32; 8] = [0; 8];
static mut TIMER : [i32; 8] = [0; 8];
static mut S1 :  smash::phx::Vector3f =  smash::phx::Vector3f { x: 0.0, y: 11.0, z: 0.0 };
static mut S2 :  smash::phx::Vector3f =  smash::phx::Vector3f { x: 0.0, y: 6.2, z: 0.0 };
static mut S3 :  smash::phx::Vector3f =  smash::phx::Vector3f { x: 0.0, y: 3.0, z: 0.0 };
	


pub fn install() {
	frame::install();
	status::install();
	acmd::install();
}

