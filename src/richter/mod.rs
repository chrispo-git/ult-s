mod status;
mod frame;
mod acmd;
use crate::util::*;
		
static mut WAS_AIR : [bool; 8] = [false; 8];	


pub fn install() {
	frame::install();
	status::install();
	acmd::install();
}