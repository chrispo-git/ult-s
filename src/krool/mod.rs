mod status;
mod frame;
mod acmd;
use crate::util::*;


pub fn install() {
	frame::install();
	status::install();
	acmd::install();
}
