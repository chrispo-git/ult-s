mod status;
mod frame;

mod acmd;

pub fn install() {
	frame::install();
	status::install();
	acmd::install();
}