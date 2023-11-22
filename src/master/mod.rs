mod status;
mod frame;
mod acmd;

static mut IS_THUNDER : [bool; 8] = [false; 8];


pub fn install() {
	frame::install();
	status::install();
	acmd::install();
}

