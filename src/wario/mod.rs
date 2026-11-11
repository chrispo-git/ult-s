mod status;
mod frame;
mod acmd;
use crate::util::*;
use smash::lib::lua_const::*;
use smash::hash40;


static FIGHTER_WARIO_INSTANCE_WORK_ID_FLAG_BAN_SIDEB : i32 = 0;
static FIGHTER_WARIO_INSTANCE_WORK_ID_FLAG_HAS_BOUNCE : i32 = 1;
static FIGHTER_WARIO_INSTANCE_WORK_ID_FLAG_IS_JUMP : i32 = 2;
static FIGHTER_WARIO_INSTANCE_WORK_ID_FLAG_SHOW_COUNT : i32 = 3;
static FIGHTER_WARIO_INSTANCE_WORK_ID_INT_COIN_COUNT : i32 = 4;
static FIGHTER_WARIO_INSTANCE_WORK_ID_FLOAT_ALPHA_COUNTER : i32 = 5;
			
pub static mut FIGHTER_WARIO_GENERATE_ARTICLE_COIN: i32 = 0;
pub static mut WEAPON_KIND_WARIO_COIN: i32 = 0;
pub const WEAPON_WARIO_COIN_STATUS_KIND_SHOOT: i32 = 0x0;

pub static mut FIGHTER_WARIO_GENERATE_ARTICLE_COUNTER: i32 = 0;
pub static mut WEAPON_KIND_WARIO_COUNTER: i32 = 0;
pub const WEAPON_WARIO_COUNTER_STATUS_KIND_APPEAR: i32 = 0x0;



pub fn install() {
	unsafe {
		let coin = smashline::clone_weapon(
			"wario", "coin", 
			*WEAPON_KIND_KOOPAJR_CANNONBALL, 
			false
		);
		let counter = smashline::clone_weapon(
			"wario", "counter", 
			*WEAPON_KIND_PEACH_KINOPIO, 
			false
		);
		FIGHTER_WARIO_GENERATE_ARTICLE_COIN = coin.generate_id;
		WEAPON_KIND_WARIO_COIN = coin.weapon_id;
		FIGHTER_WARIO_GENERATE_ARTICLE_COUNTER = counter.generate_id;
		WEAPON_KIND_WARIO_COUNTER = counter.weapon_id;
	}
	
	frame::install();
	status::install();
	acmd::install();

	 crate::param_cache::update_float_2(*FIGHTER_KIND_WARIO, get_marked_costumes("wario","wario").into_iter().map(|x| x as i32).collect(), (smash::hash40("air_accel_x_mul"), 0, 0.11));
	 crate::param_cache::update_float_2(*FIGHTER_KIND_WARIO, get_marked_costumes("wario","wario").into_iter().map(|x| x as i32).collect(), (smash::hash40("weight"), 0, 104.0));

}