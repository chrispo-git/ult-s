mod status;
mod frame;
mod acmd;
use crate::util::*;
use smash::lib::lua_const::*;
use smash::hash40;
			
static mut ISA_RESHOOT_TIME: [i32; 8] = [0; 8];
static mut ISA_SHOT_KIND: [i32; 8] = [1; 8];
			


pub fn install() {
	frame::install();
	status::install();
	acmd::install();

	param_config::update_float_2(-*FIGHTER_KIND_SHIZUE, get_marked_costumes("shizue","shizue"), (smash::hash40("param_pot"), smash::hash40("speed_x"), 1.5));
	param_config::update_float_2(-*FIGHTER_KIND_SHIZUE, get_marked_costumes("shizue","shizue"), (smash::hash40("param_pot"), smash::hash40("speed_y"), 2.0));
	param_config::update_int_2(-*FIGHTER_KIND_SHIZUE, get_marked_costumes("shizue","shizue"), (smash::hash40("param_bullet"), smash::hash40("life"), 20));
	param_config::update_float_2(-*FIGHTER_KIND_SHIZUE, get_marked_costumes("shizue","shizue"), (smash::hash40("param_bullet"), smash::hash40("limit_gravity"), 0.5));
	param_config::update_float_2(-*FIGHTER_KIND_SHIZUE, get_marked_costumes("shizue","shizue"), (smash::hash40("param_bullet"), smash::hash40("speed"), 4.0));
	param_config::update_float_2(-*FIGHTER_KIND_SHIZUE, get_marked_costumes("shizue","shizue"), (smash::hash40("param_bullet"), smash::hash40("limit_speed"), 1.5));
	param_config::update_float_2(-*FIGHTER_KIND_SHIZUE, get_marked_costumes("shizue","shizue"), (smash::hash40("map_collision_clayrocket"), smash::hash40("half_width"), 0.0));
	param_config::update_float_2(*FIGHTER_KIND_SHIZUE, get_marked_costumes("shizue","shizue"), (smash::hash40("ground_brake"), 0, 0.075));
	param_config::update_float_2(*FIGHTER_KIND_SHIZUE, get_marked_costumes("shizue","shizue"), (smash::hash40("dash_speed"), 0, 2.1583));
	param_config::update_float_2(*FIGHTER_KIND_SHIZUE, get_marked_costumes("shizue","shizue"), (smash::hash40("run_speed_max"), 0, 1.76));
	param_config::update_float_2(*FIGHTER_KIND_SHIZUE, get_marked_costumes("shizue","shizue"), (smash::hash40("jump_speed_x_mul"), 0, 1.2));
	param_config::update_float_2(*FIGHTER_KIND_SHIZUE, get_marked_costumes("shizue","shizue"), (smash::hash40("air_speed_x_stable"), 0, 0.95));
	param_config::update_float_2(*FIGHTER_KIND_SHIZUE, get_marked_costumes("shizue","shizue"), (smash::hash40("air_speed_y_stable"), 0, 1.675));
	param_config::update_float_2(*FIGHTER_KIND_SHIZUE, get_marked_costumes("shizue","shizue"), (smash::hash40("dive_speed_y"), 0, 2.68));
	param_config::update_float_2(*FIGHTER_KIND_SHIZUE, get_marked_costumes("shizue","shizue"), (smash::hash40("weight"), 0, 85.0));
	param_config::update_float_2(*FIGHTER_KIND_SHIZUE, get_marked_costumes("shizue","shizue"), (smash::hash40("landing_attack_air_frame_n"), 0, 5.0));
	param_config::update_float_2(*FIGHTER_KIND_SHIZUE, get_marked_costumes("shizue","shizue"), (smash::hash40("landing_attack_air_frame_f"), 0, 17.0));
	param_config::update_float_2(*FIGHTER_KIND_SHIZUE, get_marked_costumes("shizue","shizue"), (smash::hash40("landing_attack_air_frame_b"), 0, 9.0));
	param_config::update_float_2(*FIGHTER_KIND_SHIZUE, get_marked_costumes("shizue","shizue"), (smash::hash40("landing_attack_air_frame_hi"), 0, 6.0));
	param_config::update_float_2(*FIGHTER_KIND_SHIZUE, get_marked_costumes("shizue","shizue"), (smash::hash40("landing_attack_air_frame_lw"), 0, 6.0));
	param_config::update_float_2(*FIGHTER_KIND_SHIZUE, get_marked_costumes("shizue","shizue"), (smash::hash40("scale"), 0, 1.05));

}