mod status;
mod frame;
mod acmd;
use crate::util::*;
use smash::lib::lua_const::*;
use smash::hash40;

static mut UPB_FALL : [bool; 8] = [false; 8];
static mut HAS_BUFFER_B : [bool; 8] = [false; 8];



pub fn install() {
	frame::install();
	status::install();
	acmd::install();

	param_config::update_float_2(*FIGHTER_KIND_MARIOD, get_marked_costumes("mariod","mariod"), (smash::hash40("param_special_s"), smash::hash40("special_s_start_mul_spd_x"), 1.3));
	param_config::update_float_2(*FIGHTER_KIND_MARIOD, get_marked_costumes("mariod","mariod"), (smash::hash40("param_reflector"), smash::hash40("limit"), 100.0));
	param_config::update_float_2(*FIGHTER_KIND_MARIOD, get_marked_costumes("mariod","mariod"), (smash::hash40("param_reflector"), smash::hash40("attack_mul"), 0.65));
	param_config::update_float_2(*FIGHTER_KIND_MARIOD, get_marked_costumes("mariod","mariod"), (smash::hash40("param_reflector"), smash::hash40("speed_mul"), 1.8));
	param_config::update_float_2(*FIGHTER_KIND_MARIOD, get_marked_costumes("mariod","mariod"), (smash::hash40("param_reflector"), smash::hash40("life_mul"), 1.0));
	param_config::update_float_2(*FIGHTER_KIND_MARIOD, get_marked_costumes("mariod","mariod"), (smash::hash40("param_special_hi"), smash::hash40("special_air_hi_start_x_mul"), 0.67));
	param_config::update_float_2(*FIGHTER_KIND_MARIOD, get_marked_costumes("mariod","mariod"), (smash::hash40("param_special_hi"), smash::hash40("special_air_hi_accel_y"), 0.5));
	param_config::update_float_2(*FIGHTER_KIND_MARIOD, get_marked_costumes("mariod","mariod"), (smash::hash40("param_special_hi"), smash::hash40("special_hi_dir_stick_x"), 0.5));
	param_config::update_float_2(*FIGHTER_KIND_MARIOD, get_marked_costumes("mariod","mariod"), (smash::hash40("param_special_hi"), smash::hash40("special_hi_dir_mul"), 18.0));
	param_config::update_float_2(*FIGHTER_KIND_MARIOD, get_marked_costumes("mariod","mariod"), (smash::hash40("param_special_hi"), smash::hash40("special_hi_pass_mul"), 0.85));
	param_config::update_float_2(*FIGHTER_KIND_MARIOD, get_marked_costumes("mariod","mariod"), (smash::hash40("param_special_hi"), smash::hash40("special_air_hi_pass_mul"), 0.85));
	param_config::update_float_2(*FIGHTER_KIND_MARIOD, get_marked_costumes("mariod","mariod"), (smash::hash40("param_special_hi"), smash::hash40("special_hi_fall_x_mul"), 0.6));
	param_config::update_int_2(*FIGHTER_KIND_MARIOD, get_marked_costumes("mariod","mariod"), (smash::hash40("param_special_hi"), smash::hash40("special_hi_landing_frame"), 30));
	param_config::update_float_2(*FIGHTER_KIND_MARIOD, get_marked_costumes("mariod","mariod"), (smash::hash40("param_special_hi"), smash::hash40("special_hi_lr_stick_x"), 0.3));
	param_config::update_float_2(*FIGHTER_KIND_MARIOD, get_marked_costumes("mariod","mariod"), (smash::hash40("param_special_hi"), smash::hash40("special_hi_trans_move_end_speed_x_mul"), 1.0));
	param_config::update_float_2(*FIGHTER_KIND_MARIOD, get_marked_costumes("mariod","mariod"), (smash::hash40("param_special_hi"), smash::hash40("special_hi_trans_move_end_speed_y_mul"), 1.0));
	param_config::update_float_2(-*FIGHTER_KIND_MARIOD, get_marked_costumes("mariod","mariod"), (smash::hash40("param_adjust"), smash::hash40("attack_point"), 0.0));
	param_config::update_float_2(*FIGHTER_KIND_MARIOD, get_marked_costumes("mariod","mariod"), (smash::hash40("param_adjust"), smash::hash40("move_point"), 0.0));
	param_config::update_float_2(*FIGHTER_KIND_MARIOD, get_marked_costumes("mariod","mariod"), (smash::hash40("dash_speed"), 0, 1.92));
	param_config::update_float_2(*FIGHTER_KIND_MARIOD, get_marked_costumes("mariod","mariod"), (smash::hash40("run_speed_max"), 0, 1.785));
	param_config::update_float_2(*FIGHTER_KIND_MARIOD, get_marked_costumes("mariod","mariod"), (smash::hash40("jump_speed_x_mul"), 0, 1.0));
	param_config::update_float_2(*FIGHTER_KIND_MARIOD, get_marked_costumes("mariod","mariod"), (smash::hash40("mini_jump_y"), 0, 16.0));
	param_config::update_float_2(*FIGHTER_KIND_MARIOD, get_marked_costumes("mariod","mariod"), (smash::hash40("air_speed_x_stable"), 0, 1.12));
	param_config::update_float_2(*FIGHTER_KIND_MARIOD, get_marked_costumes("mariod","mariod"), (smash::hash40("air_accel_y"), 0, 0.12));
	param_config::update_float_2(*FIGHTER_KIND_MARIOD, get_marked_costumes("mariod","mariod"), (smash::hash40("air_speed_y_stable"), 0, 1.7));
	param_config::update_float_2(*FIGHTER_KIND_MARIOD, get_marked_costumes("mariod","mariod"), (smash::hash40("damage_fly_top_air_accel_y"), 0, 0.0756));
	param_config::update_float_2(*FIGHTER_KIND_MARIOD, get_marked_costumes("mariod","mariod"), (smash::hash40("dive_speed_y"), 0, 2.72));
	param_config::update_float_2(*FIGHTER_KIND_MARIOD, get_marked_costumes("mariod","mariod"), (smash::hash40("weight"), 0, 100.0));
	param_config::update_float_2(*FIGHTER_KIND_MARIOD, get_marked_costumes("mariod","mariod"), (smash::hash40("landing_attack_air_frame_n"), 0, 6.0));
	param_config::update_float_2(*FIGHTER_KIND_MARIOD, get_marked_costumes("mariod","mariod"), (smash::hash40("landing_attack_air_frame_f"), 0, 10.0));
	param_config::update_float_2(*FIGHTER_KIND_MARIOD, get_marked_costumes("mariod","mariod"), (smash::hash40("landing_attack_air_frame_b"), 0, 13.0));
	param_config::update_float_2(*FIGHTER_KIND_MARIOD, get_marked_costumes("mariod","mariod"), (smash::hash40("scale"), 0, 1.15));

}
