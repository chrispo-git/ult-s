mod status;
mod frame;
mod acmd;
use crate::util::*;
use smash::lib::lua_const::*;
use smash::hash40;


static mut HIT0: [f32; 8] = [0.0; 8];
static mut HIT1: [f32; 8] = [0.0; 8];
static mut HIT2: [f32; 8] = [0.0; 8];
static mut HIT3: [f32; 8] = [0.0; 8];
static mut HIT4: [f32; 8] = [0.0; 8];
static mut HIT5: [f32; 8] = [0.0; 8];
static mut HIT6: [f32; 8] = [0.0; 8];
static mut HIT0S: [f32; 8] = [0.0; 8];
static mut HIT1S: [f32; 8] = [0.0; 8];
static mut HIT2S: [f32; 8] = [0.0; 8];
static mut HIT3S: [f32; 8] = [0.0; 8];
static mut HIT4S: [f32; 8] = [0.0; 8];
static mut HIT5S: [f32; 8] = [0.0; 8];
static mut HIT6S: [f32; 8] = [0.0; 8];
static mut HIT_Y: [f32; 8] = [0.0; 8];
static mut ATK_HEIGHT: [i32; 8] = [0; 8]; //0 Mid, 1 High, 2 Low, 3 Wide
static mut HIGH_ADD: f32 = 10.0;
static mut LOW_ADD: f32 = -10.0;

pub fn install() {
	frame::install();
	status::install();
	acmd::install();

	param_config::update_float_2(*FIGHTER_KIND_DEDEDE, get_marked_costumes("dedede","dedede").into_iter().map(|x| x as i32).collect(), (smash::hash40("run_speed_max"), 0, 1.675));
	param_config::update_float_2(*FIGHTER_KIND_DEDEDE, get_marked_costumes("dedede","dedede").into_iter().map(|x| x as i32).collect(), (smash::hash40("jump_aerial_y"), 0, 35.0));
	param_config::update_float_2(*FIGHTER_KIND_DEDEDE, get_marked_costumes("dedede","dedede").into_iter().map(|x| x as i32).collect(), (smash::hash40("air_speed_x_stable"), 0, 1.01));
	param_config::update_float_2(*FIGHTER_KIND_DEDEDE, get_marked_costumes("dedede","dedede").into_iter().map(|x| x as i32).collect(), (smash::hash40("air_speed_y_stable"), 0, 1.6));
	param_config::update_float_2(*FIGHTER_KIND_DEDEDE, get_marked_costumes("dedede","dedede").into_iter().map(|x| x as i32).collect(), (smash::hash40("dive_speed_y"), 0, 2.56));
	param_config::update_float_2(*FIGHTER_KIND_DEDEDE, get_marked_costumes("dedede","dedede").into_iter().map(|x| x as i32).collect(), (smash::hash40("weight"), 0, 120.0));
	param_config::update_float_2(*FIGHTER_KIND_DEDEDE, get_marked_costumes("dedede","dedede").into_iter().map(|x| x as i32).collect(), (smash::hash40("landing_attack_air_frame_f"), 0, 12.0));
	param_config::update_float_2(*FIGHTER_KIND_DEDEDE, get_marked_costumes("dedede","dedede").into_iter().map(|x| x as i32).collect(), (smash::hash40("landing_attack_air_frame_b"), 0, 9.0));
	param_config::update_float_2(*FIGHTER_KIND_DEDEDE, get_marked_costumes("dedede","dedede").into_iter().map(|x| x as i32).collect(), (smash::hash40("landing_frame"), 0, 4.0));

}