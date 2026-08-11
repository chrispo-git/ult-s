mod status;
mod frame;
mod acmd;
use crate::util::*;
use smash::lib::lua_const::*;
use smash::hash40;

static FIGHTER_PACKUN_INSTANCE_WORK_ID_FLAG_IS_BAIR : i32 = 0;
static FIGHTER_PACKUN_INSTANCE_WORK_ID_FLAG_IS_SIDEB_EAT : i32 = 1;

static FIGHTER_PACKUN_INSTANCE_WORK_ID_FLOAT_BREATH_POS_X : i32 = 2;
static FIGHTER_PACKUN_INSTANCE_WORK_ID_FLOAT_BREATH_POS_Y : i32 = 3;
			


pub fn install() {
	frame::install();
	status::install();
	acmd::install();

	param_config::update_float_2(*FIGHTER_KIND_PACKUN, get_marked_costumes("packun","packun").into_iter().map(|x| x as i32).collect(), (smash::hash40("dash_speed"), 0, 1.92));
	param_config::update_float_2(*FIGHTER_KIND_PACKUN, get_marked_costumes("packun","packun").into_iter().map(|x| x as i32).collect(), (smash::hash40("run_speed_max"), 0, 1.82));
	param_config::update_float_2(*FIGHTER_KIND_PACKUN, get_marked_costumes("packun","packun").into_iter().map(|x| x as i32).collect(), (smash::hash40("jump_speed_x_mul"), 0, 0.95));
	param_config::update_float_2(*FIGHTER_KIND_PACKUN, get_marked_costumes("packun","packun").into_iter().map(|x| x as i32).collect(), (smash::hash40("air_speed_x_stable"), 0, 1.075));
	param_config::update_float_2(*FIGHTER_KIND_PACKUN, get_marked_costumes("packun","packun").into_iter().map(|x| x as i32).collect(), (smash::hash40("landing_attack_air_frame_f"), 0, 9.0));
	param_config::update_float_2(*FIGHTER_KIND_PACKUN, get_marked_costumes("packun","packun").into_iter().map(|x| x as i32).collect(), (smash::hash40("landing_attack_air_frame_b"), 0, 12.0));
	param_config::update_float_2(*FIGHTER_KIND_PACKUN, get_marked_costumes("packun","packun").into_iter().map(|x| x as i32).collect(), (smash::hash40("landing_attack_air_frame_lw"), 0, 12.0));

}