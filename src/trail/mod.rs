mod status;
mod frame;
mod acmd;
use crate::util::*;
use smash::lib::lua_const::*;
use smash::hash40;
		
static mut HANDS :  smash::phx::Vector3f =  smash::phx::Vector3f { x: 0.0, y: 0.0, z: 0.0 };
static mut HAS_WALLJUMP: [bool; 8] = [false; 8];
static mut TO_FAIR: [bool; 8] = [false; 8];
			



pub fn install() {
	frame::install();
	status::install();
	acmd::install();

	param_config::update_float_2(*FIGHTER_KIND_TRAIL, get_marked_costumes("trail","trail"), (smash::hash40("param_private"), smash::hash40("attack_air_f2_hit_speed_y"), 0.0));
	param_config::update_int_2(*FIGHTER_KIND_TRAIL, get_marked_costumes("trail","trail"), (smash::hash40("param_private"), smash::hash40("attack_air_f_combo_max"), 0));
	param_config::update_float_2(*FIGHTER_KIND_TRAIL, get_marked_costumes("trail","trail"), (smash::hash40("param_private"), smash::hash40("combo_attack_ari_f2_end"), 0.0));
	param_config::update_float_2(*FIGHTER_KIND_TRAIL, get_marked_costumes("trail","trail"), (smash::hash40("param_private"), smash::hash40("combo_attack_ari_f3_end"), 0.0));
	param_config::update_float_2(*FIGHTER_KIND_TRAIL, get_marked_costumes("trail","trail"), (smash::hash40("dash_speed"), 0, 1.825));
	param_config::update_float_2(*FIGHTER_KIND_TRAIL, get_marked_costumes("trail","trail"), (smash::hash40("run_speed_max"), 0, 1.78));
	param_config::update_float_2(*FIGHTER_KIND_TRAIL, get_marked_costumes("trail","trail"), (smash::hash40("jump_speed_x_mul"), 0, 1.5));
	param_config::update_float_2(*FIGHTER_KIND_TRAIL, get_marked_costumes("trail","trail"), (smash::hash40("air_speed_x_stable"), 0, 1.175));
	param_config::update_float_2(*FIGHTER_KIND_TRAIL, get_marked_costumes("trail","trail"), (smash::hash40("air_speed_y_stable"), 0, 1.75));
	param_config::update_float_2(*FIGHTER_KIND_TRAIL, get_marked_costumes("trail","trail"), (smash::hash40("dive_speed_y"), 0, 2.8));
	param_config::update_float_2(*FIGHTER_KIND_TRAIL, get_marked_costumes("trail","trail"), (smash::hash40("landing_attack_air_frame_n"), 0, 9.0));
	param_config::update_float_2(*FIGHTER_KIND_TRAIL, get_marked_costumes("trail","trail"), (smash::hash40("landing_attack_air_frame_f"), 0, 8.0));
	param_config::update_float_2(*FIGHTER_KIND_TRAIL, get_marked_costumes("trail","trail"), (smash::hash40("landing_attack_air_frame_hi"), 0, 9.0));

}