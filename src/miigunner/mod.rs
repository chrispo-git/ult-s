mod status;
mod frame;
mod acmd;

/*static mut SIDEB_COOLDOWN : i32 = 300;*/
static mut CHARGE_FRAMES : [i32; 8] = [0; 8];
static MAX_FRAMES : i32 = 60;


pub fn install() {
	frame::install();
	status::install();
	acmd::install();
}
