mod status;
mod frame;
mod acmd;			


static mut SET_UPB_FREEFALL: [bool; 8] = [false; 8];

pub fn install() {
	frame::install();
	status::install();
	acmd::install();
}