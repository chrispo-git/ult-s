mod status;
mod frame;
mod acmd;

static mut HAS_DOWNB : [bool; 8] = [false; 8];
static mut DO_STALL : [bool; 8] = [false; 8];

pub fn install() {
	frame::install();
	status::install();
	acmd::install();
}