mod status;
mod frame;
mod acmd;
use crate::util::*;
use smash::lib::lua_const::*;
use smash::hash40;

static mut LIGHTSPEED :  smash::phx::Vector3f =  smash::phx::Vector3f { x: 0.0, y: 6.5, z: 0.0 };
static mut LIGHTSPEED_ROT :  smash::phx::Vector3f =  smash::phx::Vector3f { x: 0.0, y: 180.0, z: 0.0 };
use smash::phx::Vector2f;
static FIGHTER_SONIC_INSTANCE_WORK_ID_FLAG_BAN_SIDEB : i32 = 0;
static FIGHTER_SONIC_INSTANCE_WORK_ID_FLAG_HAS_MADE_SIDEB_EFF : i32 = 1;
			


pub fn install() {
	frame::install();
	status::install();
	acmd::install();

	 crate::param_cache::update_float_2(*FIGHTER_KIND_SONIC, get_marked_costumes("sonic","sonic").into_iter().map(|x| x as i32).collect(), (smash::hash40("cliff_hang_data"), smash::hash40("p2_y"), 5.0));
	crate::param_cache::update_int_2(*FIGHTER_KIND_SONIC, get_marked_costumes("sonic","sonic").into_iter().map(|x| x as i32).collect(), (smash::hash40("param_special_n"), smash::hash40("special_n_start_enable_attack_frame"), 5));
	crate::param_cache::update_int_2(*FIGHTER_KIND_SONIC, get_marked_costumes("sonic","sonic").into_iter().map(|x| x as i32).collect(), (smash::hash40("param_special_n"), smash::hash40("special_n_start_auto_attack_frame"), 30));
	 crate::param_cache::update_float_2(*FIGHTER_KIND_SONIC, get_marked_costumes("sonic","sonic").into_iter().map(|x| x as i32).collect(), (smash::hash40("param_special_n"), smash::hash40("special_n_add_attack_power"), 5.0));
	 crate::param_cache::update_float_2(*FIGHTER_KIND_SONIC, get_marked_costumes("sonic","sonic").into_iter().map(|x| x as i32).collect(), (smash::hash40("param_special_n"), smash::hash40("special_n_homing_speed"), 4.0));
	 crate::param_cache::update_float_2(*FIGHTER_KIND_SONIC, get_marked_costumes("sonic","sonic").into_iter().map(|x| x as i32).collect(), (smash::hash40("param_special_n"), smash::hash40("special_n_hit_accel_y_mul"), 3.0));
	 crate::param_cache::update_float_2(*FIGHTER_KIND_SONIC, get_marked_costumes("sonic","sonic").into_iter().map(|x| x as i32).collect(), (smash::hash40("param_special_hi"), smash::hash40("special_hi_jump_speed_y"), 8.975));
	 crate::param_cache::update_float_2(*FIGHTER_KIND_SONIC, get_marked_costumes("sonic","sonic").into_iter().map(|x| x as i32).collect(), (smash::hash40("walk_speed_max"), 0, 1.395));
	 crate::param_cache::update_float_2(*FIGHTER_KIND_SONIC, get_marked_costumes("sonic","sonic").into_iter().map(|x| x as i32).collect(), (smash::hash40("ground_brake"), 0, 0.095));
	 crate::param_cache::update_float_2(*FIGHTER_KIND_SONIC, get_marked_costumes("sonic","sonic").into_iter().map(|x| x as i32).collect(), (smash::hash40("dash_speed"), 0, 2.425));
	 crate::param_cache::update_float_2(*FIGHTER_KIND_SONIC, get_marked_costumes("sonic","sonic").into_iter().map(|x| x as i32).collect(), (smash::hash40("run_speed_max"), 0, 2.35));
	 crate::param_cache::update_float_2(*FIGHTER_KIND_SONIC, get_marked_costumes("sonic","sonic").into_iter().map(|x| x as i32).collect(), (smash::hash40("air_accel_x_mul"), 0, 0.025));
	 crate::param_cache::update_float_2(*FIGHTER_KIND_SONIC, get_marked_costumes("sonic","sonic").into_iter().map(|x| x as i32).collect(), (smash::hash40("air_accel_y"), 0, 0.135));
	 crate::param_cache::update_float_2(*FIGHTER_KIND_SONIC, get_marked_costumes("sonic","sonic").into_iter().map(|x| x as i32).collect(), (smash::hash40("air_speed_y_stable"), 0, 1.7625));
	 crate::param_cache::update_float_2(*FIGHTER_KIND_SONIC, get_marked_costumes("sonic","sonic").into_iter().map(|x| x as i32).collect(), (smash::hash40("dive_speed_y"), 0, 2.82));
	 crate::param_cache::update_float_2(*FIGHTER_KIND_SONIC, get_marked_costumes("sonic","sonic").into_iter().map(|x| x as i32).collect(), (smash::hash40("weight"), 0, 85.0));
	 crate::param_cache::update_float_2(*FIGHTER_KIND_SONIC, get_marked_costumes("sonic","sonic").into_iter().map(|x| x as i32).collect(), (smash::hash40("landing_attack_air_frame_n"), 0, 7.0));
	 crate::param_cache::update_float_2(*FIGHTER_KIND_SONIC, get_marked_costumes("sonic","sonic").into_iter().map(|x| x as i32).collect(), (smash::hash40("landing_attack_air_frame_f"), 0, 7.0));
	 crate::param_cache::update_float_2(*FIGHTER_KIND_SONIC, get_marked_costumes("sonic","sonic").into_iter().map(|x| x as i32).collect(), (smash::hash40("landing_attack_air_frame_b"), 0, 9.0));
	 crate::param_cache::update_float_2(*FIGHTER_KIND_SONIC, get_marked_costumes("sonic","sonic").into_iter().map(|x| x as i32).collect(), (smash::hash40("landing_attack_air_frame_hi"), 0, 8.0));
	 crate::param_cache::update_float_2(*FIGHTER_KIND_SONIC, get_marked_costumes("sonic","sonic").into_iter().map(|x| x as i32).collect(), (smash::hash40("landing_attack_air_frame_lw"), 0, 21.0));
	crate::param_cache::update_int_2(*FIGHTER_KIND_SONIC, get_marked_costumes("sonic","sonic").into_iter().map(|x| x as i32).collect(), (smash::hash40("attack_combo_max"), 0, 2));
	crate::param_cache::update_int_2(*FIGHTER_KIND_SONIC, get_marked_costumes("sonic","sonic").into_iter().map(|x| x as i32).collect(), (smash::hash40("attack_100_enable_cnt"), 0, 6));
	crate::param_cache::update_int_2(*FIGHTER_KIND_SONIC, get_marked_costumes("sonic","sonic").into_iter().map(|x| x as i32).collect(), (smash::hash40("attack_100_rebound_count"), 0, 10));
	crate::param_cache::update_int_2(*FIGHTER_KIND_SONIC, get_marked_costumes("sonic","sonic").into_iter().map(|x| x as i32).collect(), (smash::hash40("attack100_type"), 0, 1));

}