mod status;
mod frame;
mod acmd;

static mut HYDRANT_POS_X : [f32; 8] = [0.0; 8];
static mut HYDRANT_POS_Y : [f32; 8] = [0.0; 8];
static mut TRAMPOLINE_POS_X : [f32; 8] = [0.0; 8];
static mut TRAMPOLINE_POS_Y : [f32; 8] = [0.0; 8];
static mut TRAMPOLINE_DELETE_TIMER : [i32; 8] = [0; 8];
static mut HAS_UPB_ENDS : [bool; 8] = [false; 8];
static mut WE_BOUNCE_NOW : [bool; 8] = [false; 8];
			


pub fn install() {
	frame::install();
	status::install();
	acmd::install();
}