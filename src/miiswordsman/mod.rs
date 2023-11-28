mod status;
mod frame;
mod acmd;


static mut COUNTER_STORE: [bool; 8] = [false; 8];
static mut CUSTOM_BOMB: [bool; 8] = [false; 8];
static mut BOMB_TIME: [i32; 8] = [0; 8];
static mut NADO_COOLDOWN: [i32; 8] = [0; 8];
static mut NADO_MAX: i32 = 70;



pub fn install() {
	frame::install();
	status::install();
	acmd::install();
}
