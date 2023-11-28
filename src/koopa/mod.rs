mod status;
mod frame;
mod acmd;

static mut FIREBALL : [i32; 8] = [0; 8];
static mut SPECIAL_ZOOM_GFX : [i32; 8] = [0; 8];
static mut KOOPA_EXCELLENT_SMASH : [bool; 8] = [false; 8];
static NONE :  smash::phx::Vector3f =  smash::phx::Vector3f { x: 0.0, y: 0.0, z: 0.0 };
		
pub fn install() {
	frame::install();
	status::install();
	acmd::install();
}
