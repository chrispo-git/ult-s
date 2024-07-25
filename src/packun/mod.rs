mod status;
mod frame;
mod acmd;

static mut IS_BAIR : [bool; 8] = [false; 8];
static mut IS_SIDEB_EAT : [bool; 8] = [false; 8];

static mut BREATH_POS_X : [f32; 8] = [0.0; 8];
static mut BREATH_POS_Y : [f32; 8] = [0.0; 8];
			


pub fn install() {
	frame::install();
	status::install();
	acmd::install();
}