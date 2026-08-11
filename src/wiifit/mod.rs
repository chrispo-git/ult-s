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

	 crate::param_cache::update_float_2(*FIGHTER_KIND_WIIFIT, get_marked_costumes("wiifit","wiifit").into_iter().map(|x| x as i32).collect(), (smash::hash40("param_special_n"), smash::hash40("0x1b44a04660"), 3.0));
	 crate::param_cache::update_float_2(-*WEAPON_KIND_WIIFIT_SUNBULLET, get_marked_costumes("wiifit","wiifit").into_iter().map(|x| x as i32).collect(), (smash::hash40("param_sunbullet"), smash::hash40("speed_min"), 4.0));
	 crate::param_cache::update_float_2(-*WEAPON_KIND_WIIFIT_SUNBULLET, get_marked_costumes("wiifit","wiifit").into_iter().map(|x| x as i32).collect(), (smash::hash40("param_sunbullet"), smash::hash40("speed_max"), 3.5));
	 crate::param_cache::update_float_2(-*WEAPON_KIND_WIIFIT_SUNBULLET, get_marked_costumes("wiifit","wiifit").into_iter().map(|x| x as i32).collect(), (smash::hash40("param_sunbullet"), smash::hash40("scale_min"), 0.1));
	 crate::param_cache::update_float_2(-*WEAPON_KIND_WIIFIT_SUNBULLET, get_marked_costumes("wiifit","wiifit").into_iter().map(|x| x as i32).collect(), (smash::hash40("param_sunbullet"), smash::hash40("scale_max"), 0.5));
	crate::param_cache::update_int_2(-*WEAPON_KIND_WIIFIT_SUNBULLET, get_marked_costumes("wiifit","wiifit").into_iter().map(|x| x as i32).collect(), (smash::hash40("param_sunbullet"), smash::hash40("life"), 30));
	crate::param_cache::update_int_2(*FIGHTER_KIND_WIIFIT, get_marked_costumes("wiifit","wiifit").into_iter().map(|x| x as i32).collect(), (smash::hash40("param_special_lw"), smash::hash40("0x202e02971b"), 480));
	 crate::param_cache::update_float_2(*FIGHTER_KIND_WIIFIT, get_marked_costumes("wiifit","wiifit").into_iter().map(|x| x as i32).collect(), (smash::hash40("param_special_lw"), smash::hash40("0x1fa3e7a2c8"), 1.65));
	 crate::param_cache::update_float_2(*FIGHTER_KIND_WIIFIT, get_marked_costumes("wiifit","wiifit").into_iter().map(|x| x as i32).collect(), (smash::hash40("param_special_lw"), smash::hash40("0x2206378e6d"), 1.5));
	 crate::param_cache::update_float_2(*FIGHTER_KIND_WIIFIT, get_marked_costumes("wiifit","wiifit").into_iter().map(|x| x as i32).collect(), (smash::hash40("param_special_lw"), smash::hash40("0x213dc6f493"), 1.13));
	crate::param_cache::update_int_2(*FIGHTER_KIND_WIIFIT, get_marked_costumes("wiifit","wiifit").into_iter().map(|x| x as i32).collect(), (smash::hash40("param_special_lw"), smash::hash40("breathing_waza_restore_frame"), 1800));
	 crate::param_cache::update_float_2(*FIGHTER_KIND_WIIFIT, get_marked_costumes("wiifit","wiifit").into_iter().map(|x| x as i32).collect(), (smash::hash40("jump_speed_x_mul"), 0, 1.2));
	 crate::param_cache::update_float_2(*FIGHTER_KIND_WIIFIT, get_marked_costumes("wiifit","wiifit").into_iter().map(|x| x as i32).collect(), (smash::hash40("air_speed_y_stable"), 0, 1.75));
	 crate::param_cache::update_float_2(*FIGHTER_KIND_WIIFIT, get_marked_costumes("wiifit","wiifit").into_iter().map(|x| x as i32).collect(), (smash::hash40("dive_speed_y"), 0, 2.72));

}