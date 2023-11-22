mod status;
mod frame;
mod acmd;

static mut UPB_FALL : [bool; 8] = [false; 8];
static mut HAS_BUFFER_B : [bool; 8] = [false; 8];



pub fn install() {
	frame::install();
	status::install();
	acmd::install();
}
