mod status;
mod frame;
mod acmd;

static mut STATIC_MUT : [i32; 8] = [6; 8];
			


pub fn install() {
	frame::install();
	status::install();
	acmd::install();
}