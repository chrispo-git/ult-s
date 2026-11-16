mod status;
mod frame;
mod acmd;
use crate::util::*;
use smash::lib::lua_const::*;
use smash::hash40;
			


pub fn install() {
	let costumes_sheik_sheik: Vec<i32> = get_marked_costumes("sheik", "sheik").into_iter().map(|x| x as i32).collect();
	frame::install();
	status::install();
	acmd::install();

	crate::param_cache::update_int_2(*FIGHTER_KIND_SHEIK, costumes_sheik_sheik.clone(), (smash::hash40("param_special_n"), smash::hash40("landing_frame"), 0));
	 crate::param_cache::update_float_2(*FIGHTER_KIND_SHEIK, costumes_sheik_sheik.clone(), (smash::hash40("param_special_n"), smash::hash40("charge_speed_mul"), 1.3));
	crate::param_cache::update_int_2(-*WEAPON_KIND_SHEIK_NEEDLE, costumes_sheik_sheik.clone(), (smash::hash40("param_needle"), smash::hash40("life"), 11));
	 crate::param_cache::update_float_2(*FIGHTER_KIND_SHEIK, costumes_sheik_sheik.clone(), (smash::hash40("air_speed_x_stable"), 0, 1.2));
	 crate::param_cache::update_float_2(*FIGHTER_KIND_SHEIK, costumes_sheik_sheik.clone(), (smash::hash40("weight"), 0, 80.0));
	 crate::param_cache::update_float_2(*FIGHTER_KIND_SHEIK, costumes_sheik_sheik.clone(), (smash::hash40("landing_attack_air_frame_b"), 0, 7.0));
	 crate::param_cache::update_float_2(*FIGHTER_KIND_SHEIK, costumes_sheik_sheik.clone(), (smash::hash40("landing_attack_air_frame_lw"), 0, 10.0));

}