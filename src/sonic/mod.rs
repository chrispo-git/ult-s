mod status;
mod frame;
mod acmd;
use crate::util::*;
use smash::lib::lua_const::*;
use smash::hash40;

static mut LIGHTSPEED :  smash::phx::Vector3f =  smash::phx::Vector3f { x: 0.0, y: 6.5, z: 0.0 };
static mut LIGHTSPEED_ROT :  smash::phx::Vector3f =  smash::phx::Vector3f { x: 0.0, y: 180.0, z: 0.0 };
use smash::phx::Vector2f;
static mut BAN_SIDEB : [bool; 8] = [false; 8];
static mut HAS_MADE_SIDEB_EFF : [bool; 8] = [false; 8];
			


pub fn install() {
	frame::install();
	status::install();
	acmd::install();

	param_config::update_float_2(-*FIGHTER_KIND_SONIC, get_marked_costumes("sonic","sonic").into_iter().map(|x| x as i32).collect(), (smash::hash40("cliff_hang_data"), smash::hash40("p2_y"), 5.0));
	param_config::update_int_2(*FIGHTER_KIND_SONIC, get_marked_costumes("sonic","sonic").into_iter().map(|x| x as i32).collect(), (smash::hash40("param_special_n"), smash::hash40("special_n_start_enable_attack_frame"), 5));
	param_config::update_int_2(*FIGHTER_KIND_SONIC, get_marked_costumes("sonic","sonic").into_iter().map(|x| x as i32).collect(), (smash::hash40("param_special_n"), smash::hash40("special_n_start_auto_attack_frame"), 30));
	param_config::update_float_2(*FIGHTER_KIND_SONIC, get_marked_costumes("sonic","sonic").into_iter().map(|x| x as i32).collect(), (smash::hash40("param_special_n"), smash::hash40("special_n_add_attack_power"), 5.0));
	param_config::update_float_2(*FIGHTER_KIND_SONIC, get_marked_costumes("sonic","sonic").into_iter().map(|x| x as i32).collect(), (smash::hash40("param_special_n"), smash::hash40("special_n_homing_speed"), 4.0));
	param_config::update_float_2(*FIGHTER_KIND_SONIC, get_marked_costumes("sonic","sonic").into_iter().map(|x| x as i32).collect(), (smash::hash40("param_special_n"), smash::hash40("special_n_hit_accel_y_mul"), 3.0));
	param_config::update_float_2(*FIGHTER_KIND_SONIC, get_marked_costumes("sonic","sonic").into_iter().map(|x| x as i32).collect(), (smash::hash40("param_special_hi"), smash::hash40("special_hi_jump_speed_y"), 8.975));
	param_config::update_float_2(*FIGHTER_KIND_SONIC, get_marked_costumes("sonic","sonic").into_iter().map(|x| x as i32).collect(), (smash::hash40("walk_speed_max"), 0, 1.395));
	param_config::update_float_2(*FIGHTER_KIND_SONIC, get_marked_costumes("sonic","sonic").into_iter().map(|x| x as i32).collect(), (smash::hash40("ground_brake"), 0, 0.095));
	param_config::update_float_2(*FIGHTER_KIND_SONIC, get_marked_costumes("sonic","sonic").into_iter().map(|x| x as i32).collect(), (smash::hash40("dash_speed"), 0, 2.425));
	param_config::update_float_2(*FIGHTER_KIND_SONIC, get_marked_costumes("sonic","sonic").into_iter().map(|x| x as i32).collect(), (smash::hash40("run_speed_max"), 0, 2.35));
	param_config::update_float_2(*FIGHTER_KIND_SONIC, get_marked_costumes("sonic","sonic").into_iter().map(|x| x as i32).collect(), (smash::hash40("air_accel_x_mul"), 0, 0.025));
	param_config::update_float_2(*FIGHTER_KIND_SONIC, get_marked_costumes("sonic","sonic").into_iter().map(|x| x as i32).collect(), (smash::hash40("air_accel_y"), 0, 0.135));
	param_config::update_float_2(*FIGHTER_KIND_SONIC, get_marked_costumes("sonic","sonic").into_iter().map(|x| x as i32).collect(), (smash::hash40("air_speed_y_stable"), 0, 1.7625));
	param_config::update_float_2(*FIGHTER_KIND_SONIC, get_marked_costumes("sonic","sonic").into_iter().map(|x| x as i32).collect(), (smash::hash40("dive_speed_y"), 0, 2.82));
	param_config::update_float_2(*FIGHTER_KIND_SONIC, get_marked_costumes("sonic","sonic").into_iter().map(|x| x as i32).collect(), (smash::hash40("weight"), 0, 85.0));
	param_config::update_float_2(*FIGHTER_KIND_SONIC, get_marked_costumes("sonic","sonic").into_iter().map(|x| x as i32).collect(), (smash::hash40("landing_attack_air_frame_n"), 0, 7.0));
	param_config::update_float_2(*FIGHTER_KIND_SONIC, get_marked_costumes("sonic","sonic").into_iter().map(|x| x as i32).collect(), (smash::hash40("landing_attack_air_frame_f"), 0, 7.0));
	param_config::update_float_2(*FIGHTER_KIND_SONIC, get_marked_costumes("sonic","sonic").into_iter().map(|x| x as i32).collect(), (smash::hash40("landing_attack_air_frame_b"), 0, 9.0));
	param_config::update_float_2(*FIGHTER_KIND_SONIC, get_marked_costumes("sonic","sonic").into_iter().map(|x| x as i32).collect(), (smash::hash40("landing_attack_air_frame_hi"), 0, 8.0));
	param_config::update_float_2(*FIGHTER_KIND_SONIC, get_marked_costumes("sonic","sonic").into_iter().map(|x| x as i32).collect(), (smash::hash40("landing_attack_air_frame_lw"), 0, 21.0));
	param_config::update_int_2(*FIGHTER_KIND_SONIC, get_marked_costumes("sonic","sonic").into_iter().map(|x| x as i32).collect(), (smash::hash40("attack_combo_max"), 0, 2));
	param_config::update_int_2(*FIGHTER_KIND_SONIC, get_marked_costumes("sonic","sonic").into_iter().map(|x| x as i32).collect(), (smash::hash40("attack_100_enable_cnt"), 0, 6));
	param_config::update_int_2(*FIGHTER_KIND_SONIC, get_marked_costumes("sonic","sonic").into_iter().map(|x| x as i32).collect(), (smash::hash40("attack_100_rebound_count"), 0, 10));
	param_config::update_int_2(*FIGHTER_KIND_SONIC, get_marked_costumes("sonic","sonic").into_iter().map(|x| x as i32).collect(), (smash::hash40("attack100_type"), 0, 1));

}