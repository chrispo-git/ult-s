mod status;
mod frame;
mod acmd;

static mut HAS_DOUBLE_UPB : [bool; 8] = [false; 8];
static mut SPEED_DOUBLE_UPB : [bool; 8] = [false; 8];
			


pub fn install() {
	frame::install();
	status::install();
	acmd::install();
}
