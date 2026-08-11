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

	crate::param_cache::update_int_2(*FIGHTER_KIND_DOLLY, get_marked_costumes("dolly","dolly").into_iter().map(|x| x as i32).collect(), (smash::hash40("param_special_s"), smash::hash40("turn_to_b_frame"), 99999999));
	 crate::param_cache::update_float_2(*FIGHTER_KIND_DOLLY, get_marked_costumes("dolly","dolly").into_iter().map(|x| x as i32).collect(), (smash::hash40("param_private"), smash::hash40("super_special_damage"), 110.0));
	 crate::param_cache::update_float_2(*FIGHTER_KIND_DOLLY, get_marked_costumes("dolly","dolly").into_iter().map(|x| x as i32).collect(), (smash::hash40("clip_sphere_offset_x"), 0, 2.0));
	 crate::param_cache::update_float_2(*FIGHTER_KIND_DOLLY, get_marked_costumes("dolly","dolly").into_iter().map(|x| x as i32).collect(), (smash::hash40("clip_sphere_offset_y"), 0, 2.0));
	 crate::param_cache::update_float_2(*FIGHTER_KIND_DOLLY, get_marked_costumes("dolly","dolly").into_iter().map(|x| x as i32).collect(), (smash::hash40("landing_attack_air_frame_f"), 0, 10.0));

}