mod status;
mod frame;
mod acmd;

static mut MAKE_NEW_BOMB : [bool; 8] = [false; 8];
static mut NEW_BOMB_X : [f32; 8] = [0.0; 8];
static mut NEW_BOMB_Y : [f32; 8] = [0.0; 8];
static mut EXPLODE_END_TIMER : [i32; 8] = [0; 8];
static mut NEUTRALB_CHARGE : [i32; 8] = [0; 8];
static mut NEUTRALB_MAX : i32 = 120;
static mut MIN_DISTANCE : f32 = 0.75;
static mut MAX_DISTANCE : f32 = 1.75;
static mut NEUTRALB_DIST : [f32; 8] = [1.0; 8];
static mut BOMB_TO_REMOVE : [bool; 8] = [false; 8];
static mut SIDEB_CATCH : [bool; 8] = [false; 8];
static mut FORCE_END : [bool; 8] = [false; 8];
static mut FALL_COUNT : [i32; 8] = [0; 8];


static mut EXPLODE : [bool; 8] = [false; 8];
static mut BOMB :  smash::phx::Vector3f =  smash::phx::Vector3f { x: 0.0, y: 10.5, z: 0.0 };

pub fn install() {
	frame::install();
	status::install();
	acmd::install();
} 