mod status;
mod frame;
mod acmd;

static mut HAS_DOWNB : [bool; 8] = [false; 8];
static mut DOWNB_X : [f32; 8] = [0.0; 8];
static mut DOWNB_Y : [f32; 8] = [0.0; 8];
			

pub fn install() {
	frame::install();
	status::install();
	acmd::install();
}

