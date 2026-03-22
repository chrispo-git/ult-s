mod status;
mod frame;
mod acmd;
use crate::util::*;
use smash::lib::lua_const::*;
use smash::hash40;

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

	param_config::update_float_2(*FIGHTER_KIND_PICHU, get_marked_costumes("pichu","pichu").into_iter().map(|x| x as i32).collect(), (smash::hash40("jump_speed_x_mul"), 0, 1.475));
	param_config::update_float_2(*FIGHTER_KIND_PICHU, get_marked_costumes("pichu","pichu").into_iter().map(|x| x as i32).collect(), (smash::hash40("air_speed_x_stable"), 0, 1.1));
	param_config::update_float_2(*FIGHTER_KIND_PICHU, get_marked_costumes("pichu","pichu").into_iter().map(|x| x as i32).collect(), (smash::hash40("weight"), 0, 75.0));

}