mod status;
mod frame;
mod acmd;
use crate::util::*;

use smash::lib::lua_const::*;

static mut BAN_SIDEB : [bool; 8] = [false; 8];
static mut HAS_BOUNCE : [bool; 8] = [false; 8];
static mut IS_JUMP : [bool; 8] = [false; 8];
static mut SHOW_COUNT : [bool; 8] = [false; 8];
static mut COIN_COUNT : [i32; 8] = [0; 8];
static mut ALPHA_COUNTER : [f32; 8] = [0.0; 8];
			
pub static mut FIGHTER_WARIO_GENERATE_ARTICLE_COIN: i32 = 0x3;
pub const WEAPON_WARIO_COIN_STATUS_KIND_SHOOT: i32 = 0x0;

pub static mut FIGHTER_WARIO_GENERATE_ARTICLE_COUNTER: i32 = 0x3;
pub const WEAPON_WARIO_COUNTER_STATUS_KIND_APPEAR: i32 = 0x0;



pub fn install() {
	unsafe {
		FIGHTER_WARIO_GENERATE_ARTICLE_COIN += smashline::clone_weapon("koopajr", *WEAPON_KIND_KOOPAJR_CANNONBALL, "wario", "coin", false);
		FIGHTER_WARIO_GENERATE_ARTICLE_COUNTER += smashline::clone_weapon("peach", *WEAPON_KIND_PEACH_KINOPIO, "wario", "counter", false);
	}
	
	frame::install();
	status::install();
	acmd::install();
}