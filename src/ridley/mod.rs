mod status;
mod frame;
mod acmd;

static mut UPB_ANGLE : [f32; 8] = [0.0; 8];
			


pub fn install() {
	frame::install();
	status::install();
	acmd::install();
}
