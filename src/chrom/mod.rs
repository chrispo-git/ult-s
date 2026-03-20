mod status;
mod frame;
mod acmd;
use crate::util::*;
use smash::lib::lua_const::*;
use smash::hash40;

pub fn install() {
	frame::install();
	status::install();
	acmd::install();

	param_config::update_float_2(*FIGHTER_KIND_CHROM, get_marked_costumes("chrom","chrom"), (smash::hash40("param_special_hi"), smash::hash40("turn_after_flip_y_mul"), 0.75));
	param_config::update_float_2(*FIGHTER_KIND_CHROM, get_marked_costumes("chrom","chrom"), (smash::hash40("jump_speed_x_mul"), 0, 1.0));
	param_config::update_float_2(*FIGHTER_KIND_CHROM, get_marked_costumes("chrom","chrom"), (smash::hash40("jump_aerial_y"), 0, 30.0));
	param_config::update_float_2(*FIGHTER_KIND_CHROM, get_marked_costumes("chrom","chrom"), (smash::hash40("air_accel_x_mul"), 0, 0.03));
	param_config::update_float_2(*FIGHTER_KIND_CHROM, get_marked_costumes("chrom","chrom"), (smash::hash40("air_speed_x_stable"), 0, 1.1));
	param_config::update_float_2(*FIGHTER_KIND_CHROM, get_marked_costumes("chrom","chrom"), (smash::hash40("damage_fly_top_speed_y_stable"), 0, 1.9));
	param_config::update_float_2(*FIGHTER_KIND_CHROM, get_marked_costumes("chrom","chrom"), (smash::hash40("dive_speed_y"), 0, 3.04));
	param_config::update_float_2(*FIGHTER_KIND_CHROM, get_marked_costumes("chrom","chrom"), (smash::hash40("weight"), 0, 93.0));
	param_config::update_float_2(*FIGHTER_KIND_CHROM, get_marked_costumes("chrom","chrom"), (smash::hash40("landing_attack_air_frame_n"), 0, 9.0));
	param_config::update_float_2(*FIGHTER_KIND_CHROM, get_marked_costumes("chrom","chrom"), (smash::hash40("landing_attack_air_frame_b"), 0, 10.0));
	param_config::update_float_2(*FIGHTER_KIND_CHROM, get_marked_costumes("chrom","chrom"), (smash::hash40("landing_attack_air_frame_hi"), 0, 8.0));
	param_config::update_float_2(*FIGHTER_KIND_CHROM, get_marked_costumes("chrom","chrom"), (smash::hash40("combo_attack_12_end"), 0, 32.0));
	param_config::update_int_2(*FIGHTER_KIND_CHROM, get_marked_costumes("chrom","chrom"), (smash::hash40("attack_combo_max"), 0, 2));

}