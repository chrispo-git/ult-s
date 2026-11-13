mod status;
mod frame;
mod acmd;
use crate::util::*;
use smash::lib::lua_const::*;
use smash::hash40;
			


pub fn install() {
	let costumes_pfushigisou_pfushigisou: Vec<i32> = get_marked_costumes("pfushigisou", "pfushigisou").into_iter().map(|x| x as i32).collect();
	let costumes_plizardon_plizardon: Vec<i32> = get_marked_costumes("plizardon", "plizardon").into_iter().map(|x| x as i32).collect();
	let costumes_pzenigame_pzenigame: Vec<i32> = get_marked_costumes("pzenigame", "pzenigame").into_iter().map(|x| x as i32).collect();
	frame::install();
	status::install();
	acmd::install();

	 crate::param_cache::update_float_2(*FIGHTER_KIND_PFUSHIGISOU, costumes_pfushigisou_pfushigisou.clone(), (smash::hash40("param_special_s"), smash::hash40("shoot_speed"), 1.25));
	crate::param_cache::update_int_2(-*WEAPON_KIND_PFUSHIGISOU_LEAFCUTTER, costumes_pfushigisou_pfushigisou.clone(), (smash::hash40("param_leafcutter"), smash::hash40("life"), 75));
	 crate::param_cache::update_float_2(*FIGHTER_KIND_PFUSHIGISOU, costumes_pfushigisou_pfushigisou.clone(), (smash::hash40("landing_attack_air_frame_f"), 0, 11.0));
	 crate::param_cache::update_float_2(*FIGHTER_KIND_PFUSHIGISOU, costumes_pfushigisou_pfushigisou.clone(), (smash::hash40("landing_attack_air_frame_hi"), 0, 16.0));
	 crate::param_cache::update_float_2(*FIGHTER_KIND_PFUSHIGISOU, costumes_pfushigisou_pfushigisou.clone(), (smash::hash40("landing_attack_air_frame_lw"), 0, 16.0));

	 crate::param_cache::update_float_2(*FIGHTER_KIND_PZENIGAME, costumes_pzenigame_pzenigame.clone(), (smash::hash40("run_speed_max"), 0, 1.82));
	 crate::param_cache::update_float_2(*FIGHTER_KIND_PZENIGAME, costumes_pzenigame_pzenigame.clone(), (smash::hash40("jump_speed_x_mul"), 0, 1.1));
	 crate::param_cache::update_float_2(*FIGHTER_KIND_PZENIGAME, costumes_pzenigame_pzenigame.clone(), (smash::hash40("air_speed_x_stable"), 0, 1.11));
	 crate::param_cache::update_float_2(*FIGHTER_KIND_PZENIGAME, costumes_pzenigame_pzenigame.clone(), (smash::hash40("landing_attack_air_frame_f"), 0, 8.0));
	 crate::param_cache::update_float_2(*FIGHTER_KIND_PZENIGAME, costumes_pzenigame_pzenigame.clone(), (smash::hash40("landing_attack_air_frame_hi"), 0, 8.0));

	 crate::param_cache::update_float_2(*FIGHTER_KIND_PLIZARDON, costumes_plizardon_plizardon.clone(), (smash::hash40("jump_speed_x_mul"), 0, 1.05));
	 crate::param_cache::update_float_2(*FIGHTER_KIND_PLIZARDON, costumes_plizardon_plizardon.clone(), (smash::hash40("landing_attack_air_frame_f"), 0, 12.0));
	 crate::param_cache::update_float_2(*FIGHTER_KIND_PLIZARDON, costumes_plizardon_plizardon.clone(), (smash::hash40("landing_attack_air_frame_b"), 0, 15.0));

}