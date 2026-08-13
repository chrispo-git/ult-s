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

	 crate::param_cache::update_float_2(*FIGHTER_KIND_CHROM, get_marked_costumes("chrom","chrom").into_iter().map(|x| x as i32).collect(), (smash::hash40("param_special_hi"), smash::hash40("turn_after_flip_y_mul"), 0.75));
	 crate::param_cache::update_float_2(*FIGHTER_KIND_CHROM, get_marked_costumes("chrom","chrom").into_iter().map(|x| x as i32).collect(), (smash::hash40("jump_speed_x_mul"), 0, 1.0));
	 crate::param_cache::update_float_2(*FIGHTER_KIND_CHROM, get_marked_costumes("chrom","chrom").into_iter().map(|x| x as i32).collect(), (smash::hash40("jump_aerial_y"), 0, 30.0));
	 crate::param_cache::update_float_2(*FIGHTER_KIND_CHROM, get_marked_costumes("chrom","chrom").into_iter().map(|x| x as i32).collect(), (smash::hash40("air_accel_x_mul"), 0, 0.03));
	 crate::param_cache::update_float_2(*FIGHTER_KIND_CHROM, get_marked_costumes("chrom","chrom").into_iter().map(|x| x as i32).collect(), (smash::hash40("air_speed_x_stable"), 0, 1.1));
	 crate::param_cache::update_float_2(*FIGHTER_KIND_CHROM, get_marked_costumes("chrom","chrom").into_iter().map(|x| x as i32).collect(), (smash::hash40("damage_fly_top_speed_y_stable"), 0, 1.9));
	 crate::param_cache::update_float_2(*FIGHTER_KIND_CHROM, get_marked_costumes("chrom","chrom").into_iter().map(|x| x as i32).collect(), (smash::hash40("dive_speed_y"), 0, 3.04));
	 crate::param_cache::update_float_2(*FIGHTER_KIND_CHROM, get_marked_costumes("chrom","chrom").into_iter().map(|x| x as i32).collect(), (smash::hash40("weight"), 0, 93.0));
	 crate::param_cache::update_float_2(*FIGHTER_KIND_CHROM, get_marked_costumes("chrom","chrom").into_iter().map(|x| x as i32).collect(), (smash::hash40("landing_attack_air_frame_n"), 0, 9.0));
	 crate::param_cache::update_float_2(*FIGHTER_KIND_CHROM, get_marked_costumes("chrom","chrom").into_iter().map(|x| x as i32).collect(), (smash::hash40("landing_attack_air_frame_b"), 0, 10.0));
	 crate::param_cache::update_float_2(*FIGHTER_KIND_CHROM, get_marked_costumes("chrom","chrom").into_iter().map(|x| x as i32).collect(), (smash::hash40("landing_attack_air_frame_hi"), 0, 8.0));
	 crate::param_cache::update_float_2(*FIGHTER_KIND_CHROM, get_marked_costumes("chrom","chrom").into_iter().map(|x| x as i32).collect(), (smash::hash40("combo_attack_12_end"), 0, 32.0));
	crate::param_cache::update_int_2(*FIGHTER_KIND_CHROM, get_marked_costumes("chrom","chrom").into_iter().map(|x| x as i32).collect(), (smash::hash40("attack_combo_max"), 0, 2));

}