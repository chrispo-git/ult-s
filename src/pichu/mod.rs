mod status;
mod frame;
mod acmd;
use crate::util::*;
use smash::lib::lua_const::*;
use smash::hash40;

static FIGHTER_PICHU_INSTANCE_WORK_ID_FLAG_LAG_INCREASE : i32 = 0;
static FIGHTER_PICHU_INSTANCE_WORK_ID_INT_RECHARGE_TIMER : i32 = 1;
static FIGHTER_PICHU_INSTANCE_WORK_ID_FLAG_HAS_DOWNB : i32 = 2;
static FIGHTER_PICHU_INSTANCE_WORK_ID_FLAG_DO_STALL : i32 = 3;
static FIGHTER_PICHU_INSTANCE_WORK_ID_FLAG_ONE_DAIR : i32 = 4;
static RECHARGE_MAX : i32 = 90;
			


pub fn install() {
	frame::install();
	status::install();
	acmd::install();

	 crate::param_cache::update_float_2(*FIGHTER_KIND_PICHU, get_marked_costumes("pichu","pichu").into_iter().map(|x| x as i32).collect(), (smash::hash40("jump_speed_x_mul"), 0, 1.475));
	 crate::param_cache::update_float_2(*FIGHTER_KIND_PICHU, get_marked_costumes("pichu","pichu").into_iter().map(|x| x as i32).collect(), (smash::hash40("air_speed_x_stable"), 0, 1.1));
	 crate::param_cache::update_float_2(*FIGHTER_KIND_PICHU, get_marked_costumes("pichu","pichu").into_iter().map(|x| x as i32).collect(), (smash::hash40("weight"), 0, 75.0));

}