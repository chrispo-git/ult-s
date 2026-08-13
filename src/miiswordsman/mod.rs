mod status;
mod frame;
mod acmd;
use crate::util::*;
use smash::lib::lua_const::*;
use smash::hash40;

static FIGHTER_MIISWORDSMAN_INSTANCE_WORK_ID_INT_TORNADO_COOLDOWN : i32 = 0;
static TORNADO_MAX : i32 = 70;




pub fn install() {
	frame::install();
	status::install();
	acmd::install();

	crate::param_cache::update_int_2(-*WEAPON_KIND_MIISWORDSMAN_CHAKRAM, [0,1,2,3,4,5,6,7,8,9,10,11,12].to_vec(), (smash::hash40("param_chakram"), smash::hash40("flick_life"), 15));
	 crate::param_cache::update_float_2(-*WEAPON_KIND_MIISWORDSMAN_CHAKRAM, [0,1,2,3,4,5,6,7,8,9,10,11,12].to_vec(), (smash::hash40("param_chakram"), smash::hash40("flick_speed"), 3.0));
	crate::param_cache::update_int_2(-*WEAPON_KIND_MIISWORDSMAN_CHAKRAM, [0,1,2,3,4,5,6,7,8,9,10,11,12].to_vec(), (smash::hash40("param_chakram"), smash::hash40("stick_frame"), 50));
	crate::param_cache::update_int_2(-*WEAPON_KIND_MIISWORDSMAN_CHAKRAM, [0,1,2,3,4,5,6,7,8,9,10,11,12].to_vec(), (smash::hash40("param_shuriken"), smash::hash40("life"), 20));
	crate::param_cache::update_int_2(-*WEAPON_KIND_MIISWORDSMAN_CHAKRAM, [0,1,2,3,4,5,6,7,8,9,10,11,12].to_vec(), (smash::hash40("param_tornado"), smash::hash40("life"), 50));
	 crate::param_cache::update_float_2(-*WEAPON_KIND_MIISWORDSMAN_CHAKRAM, [0,1,2,3,4,5,6,7,8,9,10,11,12].to_vec(), (smash::hash40("param_tornado"), smash::hash40("speed_x"), 1.6));
	 crate::param_cache::update_float_2(-*WEAPON_KIND_MIISWORDSMAN_CHAKRAM, [0,1,2,3,4,5,6,7,8,9,10,11,12].to_vec(), (smash::hash40("param_tornado"), smash::hash40("limit_speed_x"), 2.0));
	crate::param_cache::update_int_2(-*WEAPON_KIND_MIISWORDSMAN_CHAKRAM, [0,1,2,3,4,5,6,7,8,9,10,11,12].to_vec(), (smash::hash40("param_tornado"), smash::hash40("is_penetration"), 0));
	 crate::param_cache::update_float_2(*FIGHTER_KIND_MIISWORDSMAN, [0,1,2,3,4,5,6,7,8,9,10,11,12].to_vec(), (smash::hash40("dash_speed"), 0, 1.82));
	 crate::param_cache::update_float_2(*FIGHTER_KIND_MIISWORDSMAN, [0,1,2,3,4,5,6,7,8,9,10,11,12].to_vec(), (smash::hash40("run_speed_max"), 0, 1.7));
	 crate::param_cache::update_float_2(*FIGHTER_KIND_MIISWORDSMAN, [0,1,2,3,4,5,6,7,8,9,10,11,12].to_vec(), (smash::hash40("jump_speed_x_mul"), 0, 1.1));
	 crate::param_cache::update_float_2(*FIGHTER_KIND_MIISWORDSMAN, [0,1,2,3,4,5,6,7,8,9,10,11,12].to_vec(), (smash::hash40("landing_attack_air_frame_b"), 0, 11.0));
	 crate::param_cache::update_float_2(*FIGHTER_KIND_MIISWORDSMAN, [0,1,2,3,4,5,6,7,8,9,10,11,12].to_vec(), (smash::hash40("landing_attack_air_frame_n"), 0, 7.0));
	 crate::param_cache::update_float_2(*FIGHTER_KIND_MIISWORDSMAN, [0,1,2,3,4,5,6,7,8,9,10,11,12].to_vec(), (smash::hash40("landing_attack_air_frame_hi"), 0, 8.0));

}
