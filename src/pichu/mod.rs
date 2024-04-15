mod status;
mod frame;
mod acmd;

static mut LAG_INCREASE : [bool; 8] = [false; 8];
static mut RECHARGE_TIMER : [i32; 8] = [0; 8];
static mut HAS_DOWNB : [bool; 8] = [false; 8];
static mut DO_STALL : [bool; 8] = [false; 8];
static mut ONE_DAIR : [bool; 8] = [false; 8];
static RECHARGE_MAX : i32 = 90;
			


pub fn install() {
	frame::install();
	status::install();
	acmd::install();
}