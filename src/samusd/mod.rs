mod status;
mod frame;
mod acmd;
use smash::lib::lua_const::*;
use smash::hash40;
use crate::util::*;

static FIGHTER_SAMUSD_INSTANCE_WORK_ID_INT_HOLD : i32 = 0;
static FIGHTER_SAMUSD_INSTANCE_WORK_ID_FLAG_IS_HOLD : i32 = 1;
static FIGHTER_SAMUSD_INSTANCE_WORK_ID_FLAG_END : i32 = 2;
static mut HOLD_MAX : i32 = 300;
static FIGHTER_SAMUSD_INSTANCE_WORK_ID_INT_COOLDOWN : i32 = 3;
static FIGHTER_SAMUSD_INSTANCE_WORK_ID_FLAG_IS_ALLOWED : i32 = 4;
static mut HOLD_COOLDOWN : i32 = 120;
static FIGHTER_SAMUSD_INSTANCE_WORK_ID_FLOAT_UPB_ANGLE : i32 = 5;
static mut HANDS :  smash::phx::Vector3f =  smash::phx::Vector3f { x: 0.0, y: 0.0, z: 0.0 };

			


pub fn install() {
	frame::install();
	status::install();
	acmd::install();

	param_config::update_int_2(-*WEAPON_KIND_SAMUSD_MISSILE, get_marked_costumes("samusd","samusd").into_iter().map(|x| x as i32).collect(), (smash::hash40("param_missile"), smash::hash40("h_life"), 290));
	param_config::update_float_2(-*WEAPON_KIND_SAMUSD_MISSILE, get_marked_costumes("samusd","samusd").into_iter().map(|x| x as i32).collect(), (smash::hash40("param_missile"), smash::hash40("h_spd"), 0.0));
	param_config::update_float_2(-*WEAPON_KIND_SAMUSD_MISSILE, get_marked_costumes("samusd","samusd").into_iter().map(|x| x as i32).collect(), (smash::hash40("param_missile"), smash::hash40("h_anglemax"), 0.0));
	param_config::update_float_2(-*WEAPON_KIND_SAMUSD_MISSILE, get_marked_costumes("samusd","samusd").into_iter().map(|x| x as i32).collect(), (smash::hash40("param_missile"), smash::hash40("h_angle"), 0.0));
	param_config::update_float_2(-*WEAPON_KIND_SAMUSD_MISSILE, get_marked_costumes("samusd","samusd").into_iter().map(|x| x as i32).collect(), (smash::hash40("param_missile"), smash::hash40("h_angleplay"), 0.0));
	param_config::update_int_2(-*WEAPON_KIND_SAMUSD_MISSILE, get_marked_costumes("samusd","samusd").into_iter().map(|x| x as i32).collect(), (smash::hash40("param_missile"), smash::hash40("h_homingf"), 0));
	param_config::update_float_2(-*WEAPON_KIND_SAMUSD_MISSILE, get_marked_costumes("samusd","samusd").into_iter().map(|x| x as i32).collect(), (smash::hash40("param_missile"), smash::hash40("h_spdmul"), 0.0));
	param_config::update_float_2(-*WEAPON_KIND_SAMUSD_MISSILE, get_marked_costumes("samusd","samusd").into_iter().map(|x| x as i32).collect(), (smash::hash40("param_missile"), smash::hash40("h_spdmin"), 0.0));
	param_config::update_float_2(*FIGHTER_KIND_SAMUSD, get_marked_costumes("samusd","samusd").into_iter().map(|x| x as i32).collect(), (smash::hash40("param_special_hi"), smash::hash40("sjump_ar_vy0"), 4.0));
	param_config::update_float_2(*FIGHTER_KIND_SAMUSD, get_marked_costumes("samusd","samusd").into_iter().map(|x| x as i32).collect(), (smash::hash40("param_special_hi"), smash::hash40("sjump_fall_x_mul"), 1.3));
	param_config::update_float_2(*FIGHTER_KIND_SAMUSD, get_marked_costumes("samusd","samusd").into_iter().map(|x| x as i32).collect(), (smash::hash40("param_special_lw"), smash::hash40("sp_lw_gr_vy0"), 0.0));
	param_config::update_float_2(*FIGHTER_KIND_SAMUSD, get_marked_costumes("samusd","samusd").into_iter().map(|x| x as i32).collect(), (smash::hash40("param_special_lw"), smash::hash40("sp_lw_ar_vy0"), 0.0));
	param_config::update_float_2(*FIGHTER_KIND_SAMUSD, get_marked_costumes("samusd","samusd").into_iter().map(|x| x as i32).collect(), (smash::hash40("ground_brake"), 0, 0.06));
	param_config::update_float_2(*FIGHTER_KIND_SAMUSD, get_marked_costumes("samusd","samusd").into_iter().map(|x| x as i32).collect(), (smash::hash40("dash_speed"), 0, 1.92));
	param_config::update_float_2(*FIGHTER_KIND_SAMUSD, get_marked_costumes("samusd","samusd").into_iter().map(|x| x as i32).collect(), (smash::hash40("run_speed_max"), 0, 1.8));
	param_config::update_float_2(*FIGHTER_KIND_SAMUSD, get_marked_costumes("samusd","samusd").into_iter().map(|x| x as i32).collect(), (smash::hash40("jump_speed_x_mul"), 0, 1.25));
	param_config::update_float_2(*FIGHTER_KIND_SAMUSD, get_marked_costumes("samusd","samusd").into_iter().map(|x| x as i32).collect(), (smash::hash40("jump_aerial_y"), 0, 40.0));
	param_config::update_float_2(*FIGHTER_KIND_SAMUSD, get_marked_costumes("samusd","samusd").into_iter().map(|x| x as i32).collect(), (smash::hash40("air_accel_y"), 0, 0.1375));
	param_config::update_float_2(*FIGHTER_KIND_SAMUSD, get_marked_costumes("samusd","samusd").into_iter().map(|x| x as i32).collect(), (smash::hash40("air_speed_y_stable"), 0, 1.5));
	param_config::update_float_2(*FIGHTER_KIND_SAMUSD, get_marked_costumes("samusd","samusd").into_iter().map(|x| x as i32).collect(), (smash::hash40("dive_speed_y"), 0, 2.4));
	param_config::update_float_2(*FIGHTER_KIND_SAMUSD, get_marked_costumes("samusd","samusd").into_iter().map(|x| x as i32).collect(), (smash::hash40("weight"), 0, 98.0));
	param_config::update_float_2(*FIGHTER_KIND_SAMUSD, get_marked_costumes("samusd","samusd").into_iter().map(|x| x as i32).collect(), (smash::hash40("landing_attack_air_frame_n"), 0, 6.0));
	param_config::update_float_2(*FIGHTER_KIND_SAMUSD, get_marked_costumes("samusd","samusd").into_iter().map(|x| x as i32).collect(), (smash::hash40("landing_attack_air_frame_f"), 0, 10.0));
	param_config::update_float_2(*FIGHTER_KIND_SAMUSD, get_marked_costumes("samusd","samusd").into_iter().map(|x| x as i32).collect(), (smash::hash40("landing_attack_air_frame_b"), 0, 6.0));
	param_config::update_float_2(*FIGHTER_KIND_SAMUSD, get_marked_costumes("samusd","samusd").into_iter().map(|x| x as i32).collect(), (smash::hash40("landing_attack_air_frame_hi"), 0, 12.0));
	param_config::update_float_2(*FIGHTER_KIND_SAMUSD, get_marked_costumes("samusd","samusd").into_iter().map(|x| x as i32).collect(), (smash::hash40("combo_attack_12_end"), 0, 0.0));
	param_config::update_int_2(*FIGHTER_KIND_SAMUSD, get_marked_costumes("samusd","samusd").into_iter().map(|x| x as i32).collect(), (smash::hash40("attack_combo_max"), 0, 1));

}
