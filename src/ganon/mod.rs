mod status;
mod frame;
mod acmd;
use crate::util::*;
use smash::lib::lua_const::*;
use smash::hash40;

static mut FLOAT : [i32; 8] = [0; 8];
static mut X : [f32; 8] = [0.0; 8];
static mut Y : [f32; 8] = [0.0; 8];
static mut FLOAT_MAX : i32 = 90;
static mut X_MAX : f32 = 1.155;
static mut X_ACCEL_ADD : f32 = 0.06;
static mut X_ACCEL_MUL : f32 = 0.12;
static mut Y_MAX : f32 = 0.4;
static mut Y_ACCEL_ADD : f32 = 0.03;
static mut Y_ACCEL_MUL : f32 = 0.03;



pub fn install() {
	frame::install();
	status::install();
	acmd::install();

	param_config::update_float_2(*FIGHTER_KIND_GANON, get_marked_costumes("ganon","ganon"), (smash::hash40("walk_speed_max"), 0, 0.875));
	param_config::update_float_2(*FIGHTER_KIND_GANON, get_marked_costumes("ganon","ganon"), (smash::hash40("run_speed_max"), 0, 1.45));
	param_config::update_float_2(*FIGHTER_KIND_GANON, get_marked_costumes("ganon","ganon"), (smash::hash40("jump_aerial_y"), 0, 29.0));
	param_config::update_float_2(*FIGHTER_KIND_GANON, get_marked_costumes("ganon","ganon"), (smash::hash40("air_accel_x_mul"), 0, 0.05));
	param_config::update_float_2(*FIGHTER_KIND_GANON, get_marked_costumes("ganon","ganon"), (smash::hash40("air_speed_x_stable"), 0, 0.92));
	param_config::update_float_2(*FIGHTER_KIND_GANON, get_marked_costumes("ganon","ganon"), (smash::hash40("weight"), 0, 115.0));
	param_config::update_float_2(*FIGHTER_KIND_GANON, get_marked_costumes("ganon","ganon"), (smash::hash40("landing_attack_air_frame_f"), 0, 13.0));
	param_config::update_float_2(*FIGHTER_KIND_GANON, get_marked_costumes("ganon","ganon"), (smash::hash40("landing_attack_air_frame_b"), 0, 12.0));
	param_config::update_float_2(*FIGHTER_KIND_GANON, get_marked_costumes("ganon","ganon"), (smash::hash40("landing_attack_air_frame_hi"), 0, 13.0));
	param_config::update_float_2(*FIGHTER_KIND_GANON, get_marked_costumes("ganon","ganon"), (smash::hash40("landing_attack_air_frame_lw"), 0, 18.0));

}
