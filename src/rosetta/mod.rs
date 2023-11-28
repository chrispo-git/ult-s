mod status;
mod frame;
mod acmd;

static mut TICO_X : [f32; 8] = [0.0; 8];
static mut TICO_Y : [f32; 8] = [0.0; 8];
static mut ROSA_X : [f32; 8] = [0.0; 8];
static mut ROSA_Y : [f32; 8] = [0.0; 8];
static mut IS_TELEPORT : [bool; 8] = [false; 8];
static mut IS_TICO_DEAD : [bool; 8] = [false; 8];
static mut INVIS_FRAMES : [i32; 8] = [0; 8];
static mut MAX_INVIS_FRAMES : i32 = 20;
static mut COOLDOWN : [i32; 8] = [0; 8];
static mut TELEPORT_COOLDOWN : i32 = 300;
static mut HANDS :  smash::phx::Vector3f =  smash::phx::Vector3f { x: 0.0, y: 0.0, z: 0.0 };
			


pub fn install() {
	frame::install();
	status::install();
	acmd::install();
}