mod status;
mod frame;
mod acmd;

static mut BAN_SIDEB : [bool; 8] = [false; 8];
static mut HAS_BOUNCE : [bool; 8] = [false; 8];
static mut IS_JUMP : [bool; 8] = [false; 8];
			



pub fn install() {
	frame::install();
	status::install();
	acmd::install();
}