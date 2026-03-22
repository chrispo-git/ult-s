mod status;
mod frame;
mod acmd;
use crate::util::*;
use smash::lib::lua_const::*;
use smash::hash40;

static mut UPB_ANGLE : [f32; 8] = [0.0; 8];
			


pub fn install() {
	frame::install();
	status::install();
	acmd::install();
}
