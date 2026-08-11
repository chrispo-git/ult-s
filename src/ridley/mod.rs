mod status;
mod frame;
mod acmd;
use crate::util::*;
use smash::lib::lua_const::*;
use smash::hash40;

static FIGHTER_RIDLEY_INSTANCE_WORK_ID_FLOAT_UPB_ANGLE : i32 = 0;
			


pub fn install() {
	frame::install();
	status::install();
	acmd::install();
}
