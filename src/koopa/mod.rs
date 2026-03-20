mod status;
mod frame;
mod acmd;
use crate::util::*;
use smash::lib::lua_const::*;
use smash::hash40;

static mut FIREBALL : [i32; 8] = [0; 8];
static mut SPECIAL_ZOOM_GFX : [i32; 8] = [0; 8];
static mut KOOPA_EXCELLENT_SMASH : [bool; 8] = [false; 8];
static NONE :  smash::phx::Vector3f =  smash::phx::Vector3f { x: 0.0, y: 0.0, z: 0.0 };
		
pub fn install() {
	frame::install();
	status::install();
	acmd::install();

	param_config::update_int_2(*FIGHTER_KIND_KOOPA, get_marked_costumes("koopa","koopa"), (smash::hash40("param_special_n"), smash::hash40("fireframe"), 999));
	param_config::update_int_2(*FIGHTER_KIND_KOOPA, get_marked_costumes("koopa","koopa"), (smash::hash40("param_special_n"), smash::hash40("gene_interval"), 70));
	param_config::update_float_2(*FIGHTER_KIND_KOOPA, get_marked_costumes("koopa","koopa"), (smash::hash40("param_special_n"), smash::hash40("ptc_wide"), 0.0));
	param_config::update_float_2(*FIGHTER_KIND_KOOPA, get_marked_costumes("koopa","koopa"), (smash::hash40("param_special_n"), smash::hash40("gene_angle"), 0.0));
	param_config::update_float_2(*FIGHTER_KIND_KOOPA, get_marked_costumes("koopa","koopa"), (smash::hash40("param_special_n"), smash::hash40("limit_up"), 0.0));
	param_config::update_float_2(*FIGHTER_KIND_KOOPA, get_marked_costumes("koopa","koopa"), (smash::hash40("param_special_n"), smash::hash40("limit_down"), 0.0));
	param_config::update_float_2(*FIGHTER_KIND_KOOPA, get_marked_costumes("koopa","koopa"), (smash::hash40("param_special_n"), smash::hash40("f_ang"), 0.0));
	param_config::update_float_2(*FIGHTER_KIND_KOOPA, get_marked_costumes("koopa","koopa"), (smash::hash40("param_special_n"), smash::hash40("neck_rate"), 0.0));
	param_config::update_float_2(*FIGHTER_KIND_KOOPA, get_marked_costumes("koopa","koopa"), (smash::hash40("param_special_n"), smash::hash40("fire_speed_mul_max"), 1.5));
	param_config::update_float_2(*FIGHTER_KIND_KOOPA, get_marked_costumes("koopa","koopa"), (smash::hash40("param_special_n"), smash::hash40("fire_speed_mul_min"), 1.5));
	param_config::update_float_2(*FIGHTER_KIND_KOOPA, get_marked_costumes("koopa","koopa"), (smash::hash40("param_special_n"), smash::hash40("fire_scale_min"), 0.57));
	param_config::update_int_2(*FIGHTER_KIND_KOOPA, get_marked_costumes("koopa","koopa"), (smash::hash40("param_special_n"), smash::hash40("quake_interval"), 100));
	param_config::update_float_2(*FIGHTER_KIND_KOOPA, get_marked_costumes("koopa","koopa"), (smash::hash40("param_special_n"), smash::hash40("fire_scale_min_frame"), 96.0));
	param_config::update_float_2(*FIGHTER_KIND_KOOPA, get_marked_costumes("koopa","koopa"), (smash::hash40("param_special_n"), smash::hash40("fire_scale_max_frame"), 1080.0));
	param_config::update_float_2(-*FIGHTER_KIND_KOOPA, get_marked_costumes("koopa","koopa"), (smash::hash40("param_breath"), smash::hash40("life"), 70.0));
	param_config::update_float_2(-*FIGHTER_KIND_KOOPA, get_marked_costumes("koopa","koopa"), (smash::hash40("param_breath"), smash::hash40("hit_frames"), 70.0));
	param_config::update_float_2(-*FIGHTER_KIND_KOOPA, get_marked_costumes("koopa","koopa"), (smash::hash40("param_breath"), smash::hash40("min_speed"), 0.8));
	param_config::update_float_2(-*FIGHTER_KIND_KOOPA, get_marked_costumes("koopa","koopa"), (smash::hash40("param_breath"), smash::hash40("max_speed"), 0.8));
	param_config::update_float_2(*FIGHTER_KIND_KOOPA, get_marked_costumes("koopa","koopa"), (smash::hash40("landing_attack_air_frame_n"), 0, 12.0));
	param_config::update_float_2(*FIGHTER_KIND_KOOPA, get_marked_costumes("koopa","koopa"), (smash::hash40("landing_attack_air_frame_lw"), 0, 18.0));
	param_config::update_float_2(*FIGHTER_KIND_KOOPA, get_marked_costumes("koopa","koopa"), (smash::hash40("landing_frame"), 0, 4.0));

}
