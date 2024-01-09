mod status;
mod frame;
mod acmd;

/*static mut SIDEB_COOLDOWN : i32 = 300;*/
static mut CHARGE_FRAMES : [i32; 8] = [0; 8];
static mut NO_FP : [i32; 8] = [0; 8];
static MAX_FRAMES : i32 = 60;
static FP_DELAY : i32 = 100;


pub fn install() {
	frame::install();
	status::install();
	acmd::install();
}
