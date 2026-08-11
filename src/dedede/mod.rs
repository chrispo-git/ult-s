mod status;
mod frame;
mod acmd;
use crate::util::*;
use smash::lib::lua_const::*;
use smash::hash40;


static FIGHTER_DEDEDE_INSTANCE_WORK_ID_FLOAT_HIT0 : i32 = 0;
static FIGHTER_DEDEDE_INSTANCE_WORK_ID_FLOAT_HIT1 : i32 = 1;
static FIGHTER_DEDEDE_INSTANCE_WORK_ID_FLOAT_HIT2 : i32 = 2;
static FIGHTER_DEDEDE_INSTANCE_WORK_ID_FLOAT_HIT3 : i32 = 3;
static FIGHTER_DEDEDE_INSTANCE_WORK_ID_FLOAT_HIT4 : i32 = 4;
static FIGHTER_DEDEDE_INSTANCE_WORK_ID_FLOAT_HIT5 : i32 = 5;
static FIGHTER_DEDEDE_INSTANCE_WORK_ID_FLOAT_HIT6 : i32 = 6;
static FIGHTER_DEDEDE_INSTANCE_WORK_ID_FLOAT_HIT0S : i32 = 7;
static FIGHTER_DEDEDE_INSTANCE_WORK_ID_FLOAT_HIT1S : i32 = 8;
static FIGHTER_DEDEDE_INSTANCE_WORK_ID_FLOAT_HIT2S : i32 = 9;
static FIGHTER_DEDEDE_INSTANCE_WORK_ID_FLOAT_HIT3S : i32 = 10;
static FIGHTER_DEDEDE_INSTANCE_WORK_ID_FLOAT_HIT4S : i32 = 11;
static FIGHTER_DEDEDE_INSTANCE_WORK_ID_FLOAT_HIT5S : i32 = 12;
static FIGHTER_DEDEDE_INSTANCE_WORK_ID_FLOAT_HIT6S : i32 = 13;
static FIGHTER_DEDEDE_INSTANCE_WORK_ID_FLOAT_HIT_Y : i32 = 14;
static mut ATK_HEIGHT: [i32; 8] = [0; 8]; //0 Mid, 1 High, 2 Low, 3 Wide
static mut HIGH_ADD: f32 = 10.0;
static mut LOW_ADD: f32 = -10.0;

pub fn install() {
	frame::install();
	status::install();
	acmd::install();

	 crate::param_cache::update_float_2(*FIGHTER_KIND_DEDEDE, get_marked_costumes("dedede","dedede").into_iter().map(|x| x as i32).collect(), (smash::hash40("run_speed_max"), 0, 1.675));
	 crate::param_cache::update_float_2(*FIGHTER_KIND_DEDEDE, get_marked_costumes("dedede","dedede").into_iter().map(|x| x as i32).collect(), (smash::hash40("jump_aerial_y"), 0, 35.0));
	 crate::param_cache::update_float_2(*FIGHTER_KIND_DEDEDE, get_marked_costumes("dedede","dedede").into_iter().map(|x| x as i32).collect(), (smash::hash40("air_speed_x_stable"), 0, 1.01));
	 crate::param_cache::update_float_2(*FIGHTER_KIND_DEDEDE, get_marked_costumes("dedede","dedede").into_iter().map(|x| x as i32).collect(), (smash::hash40("air_speed_y_stable"), 0, 1.6));
	 crate::param_cache::update_float_2(*FIGHTER_KIND_DEDEDE, get_marked_costumes("dedede","dedede").into_iter().map(|x| x as i32).collect(), (smash::hash40("dive_speed_y"), 0, 2.56));
	 crate::param_cache::update_float_2(*FIGHTER_KIND_DEDEDE, get_marked_costumes("dedede","dedede").into_iter().map(|x| x as i32).collect(), (smash::hash40("weight"), 0, 120.0));
	 crate::param_cache::update_float_2(*FIGHTER_KIND_DEDEDE, get_marked_costumes("dedede","dedede").into_iter().map(|x| x as i32).collect(), (smash::hash40("landing_attack_air_frame_f"), 0, 12.0));
	 crate::param_cache::update_float_2(*FIGHTER_KIND_DEDEDE, get_marked_costumes("dedede","dedede").into_iter().map(|x| x as i32).collect(), (smash::hash40("landing_attack_air_frame_b"), 0, 9.0));
	 crate::param_cache::update_float_2(*FIGHTER_KIND_DEDEDE, get_marked_costumes("dedede","dedede").into_iter().map(|x| x as i32).collect(), (smash::hash40("landing_frame"), 0, 4.0));

}