mod status;
mod frame;
mod acmd;

static mut DOWNB_JUMP : [bool; 8] = [false; 8];
static mut UPB_ANGLE : [i32; 8] = [1; 8];
//0 - Inwards
//1 - Middle
//2 - Outwards
static mut IS_FINAL : [bool; 8] = [false; 8];
static NONE :  smash::phx::Vector3f =  smash::phx::Vector3f { x: 0.0, y: 5.0, z: 0.0 };
static mut variance : [f32; 8] = [0.0; 8];
static mut N1 :  smash::phx::Vector3f =  smash::phx::Vector3f { x: 0.0, y: 3.0, z: -15.0 };
static mut N2 :  smash::phx::Vector3f =  smash::phx::Vector3f { x: 0.0, y: 8.0, z: -24.0 };
static mut F3 : [u32; 8] = [0; 8];
static mut F4 : [u32; 8] = [0; 8];			


pub fn install() {
	frame::install();
	status::install();
	acmd::install();
}
