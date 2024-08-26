mod status;
mod frame;
mod acmd;

static mut BAN_SIDEB : [bool; 8] = [false; 8];
static mut HAS_BOUNCE : [bool; 8] = [false; 8];
static mut IS_JUMP : [bool; 8] = [false; 8];
static mut COIN_COUNT : [i32; 8] = [0; 8];
			



pub fn install() {
	smashline::clone_weapon("koopajr", "cannonball", "wario", "coin", false);
	smashline::clone_weapon("peach", "kinopio", "wario", "counter", false);
	
	frame::install();
	status::install();
	acmd::install();
}