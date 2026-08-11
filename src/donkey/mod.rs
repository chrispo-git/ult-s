mod status;
mod frame;
mod acmd;
use crate::util::*;
use smash::lib::lua_const::*;
use smash::hash40;

static FIGHTER_DONKEY_INSTANCE_WORK_ID_FLAG_IS_DK_START_ITEM_CHUCK : i32 = 0;
static FIGHTER_DONKEY_INSTANCE_WORK_ID_FLAG_IS_DK_UPB_BARREL : i32 = 1;
static FIGHTER_DONKEY_INSTANCE_WORK_ID_INT_UPB_TIMER : i32 = 2;
static FIGHTER_DONKEY_INSTANCE_WORK_ID_FLOAT_UPB_ANGLE_X : i32 = 3;
static FIGHTER_DONKEY_INSTANCE_WORK_ID_FLOAT_UPB_ANGLE_Y : i32 = 4;

const UPB_SPEED: f32 = 2.5;
static mut UPB_30_X: f32 = 0.0;
static mut UPB_30_Y: f32 = 0.0;


pub fn install() {
	frame::install();
	status::install();
	acmd::install();

	param_config::update_float_2(*FIGHTER_KIND_DONKEY, get_marked_costumes("donkey","donkey").into_iter().map(|x| x as i32).collect(), (smash::hash40("landing_attack_air_frame_n"), 0, 9.0));
	param_config::update_float_2(*FIGHTER_KIND_DONKEY, get_marked_costumes("donkey","donkey").into_iter().map(|x| x as i32).collect(), (smash::hash40("landing_attack_air_frame_f"), 0, 13.0));
	param_config::update_float_2(*FIGHTER_KIND_DONKEY, get_marked_costumes("donkey","donkey").into_iter().map(|x| x as i32).collect(), (smash::hash40("landing_attack_air_frame_hi"), 0, 13.0));
	param_config::update_float_2(*FIGHTER_KIND_DONKEY, get_marked_costumes("donkey","donkey").into_iter().map(|x| x as i32).collect(), (smash::hash40("landing_attack_air_frame_lw"), 0, 14.0));
	param_config::update_float_2(*FIGHTER_KIND_DONKEY, get_marked_costumes("donkey","donkey").into_iter().map(|x| x as i32).collect(), (smash::hash40("jump_speed_x_mul"), 0, 1.0));
	param_config::update_float_2(*FIGHTER_KIND_DONKEY, get_marked_costumes("donkey","donkey").into_iter().map(|x| x as i32).collect(), (smash::hash40("clip_sphere_offset_y"), 0, 3.0));
	param_config::update_int_2(*FIGHTER_KIND_DONKEY, get_marked_costumes("donkey","donkey").into_iter().map(|x| x as i32).collect(), (smash::hash40("wall_jump_type"), 0, 1));

}