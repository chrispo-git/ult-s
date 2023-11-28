mod status;
mod frame;
mod acmd;

static mut IS_DK_START_ITEM_CHUCK: [bool; 8] = [false; 8];
static mut IS_DK_UPB_BARREL: [bool; 8] = [false; 8];
static mut UPB_TIMER: [i32; 8] = [0; 8];
static mut UPB_ANGLE_X: [f32; 8] = [0.0; 8];
static mut UPB_ANGLE_Y: [f32; 8] = [0.0; 8];

const UPB_SPEED: f32 = 2.5;
static mut UPB_30_X: f32 = 0.0;
static mut UPB_30_Y: f32 = 0.0;


pub fn install() {
	frame::install();
	status::install();
	acmd::install();
}