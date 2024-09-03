mod status;
mod frame;
mod acmd;

use smash::lib::lua_const::*;

static mut BAN_SIDEB : [bool; 8] = [false; 8];
static mut HAS_BOUNCE : [bool; 8] = [false; 8];
static mut IS_JUMP : [bool; 8] = [false; 8];
static mut SHOW_COUNT : [bool; 8] = [false; 8];
static mut COIN_COUNT : [i32; 8] = [0; 8];
static mut ALPHA_COUNTER : [f32; 8] = [0.0; 8];
			



pub fn install() {
	smashline::clone_weapon("koopajr", *WEAPON_KIND_KOOPAJR_CANNONBALL, "wario", "coin", false);
	smashline::clone_weapon("peach", *WEAPON_KIND_PEACH_KINOPIO, "wario", "counter", false);
	
	frame::install();
	status::install();
	acmd::install();
}