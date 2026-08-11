mod status;
mod frame;
mod acmd;
use crate::util::*;
use smash::lib::lua_const::*;
use smash::hash40;

static FIGHTER_KOOPA_INSTANCE_WORK_ID_INT_FIREBALL : i32 = 0;
static FIGHTER_KOOPA_INSTANCE_WORK_ID_INT_SPECIAL_ZOOM_GFX : i32 = 1;
static FIGHTER_KOOPA_INSTANCE_WORK_ID_FLAG_KOOPA_EXCELLENT_SMASH : i32 = 2;
static NONE :  smash::phx::Vector3f =  smash::phx::Vector3f { x: 0.0, y: 0.0, z: 0.0 };
		
pub fn install() {
	frame::install();
	status::install();
	acmd::install();

	crate::param_cache::update_int_2(*FIGHTER_KIND_KOOPA, get_marked_costumes("koopa","koopa").into_iter().map(|x| x as i32).collect(), (smash::hash40("param_special_n"), smash::hash40("fireframe"), 999));
	crate::param_cache::update_int_2(*FIGHTER_KIND_KOOPA, get_marked_costumes("koopa","koopa").into_iter().map(|x| x as i32).collect(), (smash::hash40("param_special_n"), smash::hash40("gene_interval"), 70));
	 crate::param_cache::update_float_2(*FIGHTER_KIND_KOOPA, get_marked_costumes("koopa","koopa").into_iter().map(|x| x as i32).collect(), (smash::hash40("param_special_n"), smash::hash40("ptc_wide"), 0.0));
	 crate::param_cache::update_float_2(*FIGHTER_KIND_KOOPA, get_marked_costumes("koopa","koopa").into_iter().map(|x| x as i32).collect(), (smash::hash40("param_special_n"), smash::hash40("gene_angle"), 0.0));
	 crate::param_cache::update_float_2(*FIGHTER_KIND_KOOPA, get_marked_costumes("koopa","koopa").into_iter().map(|x| x as i32).collect(), (smash::hash40("param_special_n"), smash::hash40("limit_up"), 0.0));
	 crate::param_cache::update_float_2(*FIGHTER_KIND_KOOPA, get_marked_costumes("koopa","koopa").into_iter().map(|x| x as i32).collect(), (smash::hash40("param_special_n"), smash::hash40("limit_down"), 0.0));
	 crate::param_cache::update_float_2(*FIGHTER_KIND_KOOPA, get_marked_costumes("koopa","koopa").into_iter().map(|x| x as i32).collect(), (smash::hash40("param_special_n"), smash::hash40("f_ang"), 0.0));
	 crate::param_cache::update_float_2(*FIGHTER_KIND_KOOPA, get_marked_costumes("koopa","koopa").into_iter().map(|x| x as i32).collect(), (smash::hash40("param_special_n"), smash::hash40("neck_rate"), 0.0));
	 crate::param_cache::update_float_2(*FIGHTER_KIND_KOOPA, get_marked_costumes("koopa","koopa").into_iter().map(|x| x as i32).collect(), (smash::hash40("param_special_n"), smash::hash40("fire_speed_mul_max"), 1.5));
	 crate::param_cache::update_float_2(*FIGHTER_KIND_KOOPA, get_marked_costumes("koopa","koopa").into_iter().map(|x| x as i32).collect(), (smash::hash40("param_special_n"), smash::hash40("fire_speed_mul_min"), 1.5));
	 crate::param_cache::update_float_2(*FIGHTER_KIND_KOOPA, get_marked_costumes("koopa","koopa").into_iter().map(|x| x as i32).collect(), (smash::hash40("param_special_n"), smash::hash40("fire_scale_min"), 0.57));
	crate::param_cache::update_int_2(*FIGHTER_KIND_KOOPA, get_marked_costumes("koopa","koopa").into_iter().map(|x| x as i32).collect(), (smash::hash40("param_special_n"), smash::hash40("quake_interval"), 100));
	 crate::param_cache::update_float_2(*FIGHTER_KIND_KOOPA, get_marked_costumes("koopa","koopa").into_iter().map(|x| x as i32).collect(), (smash::hash40("param_special_n"), smash::hash40("fire_scale_min_frame"), 96.0));
	 crate::param_cache::update_float_2(*FIGHTER_KIND_KOOPA, get_marked_costumes("koopa","koopa").into_iter().map(|x| x as i32).collect(), (smash::hash40("param_special_n"), smash::hash40("fire_scale_max_frame"), 1080.0));
	 crate::param_cache::update_float_2(-*WEAPON_KIND_KOOPA_BREATH, get_marked_costumes("koopa","koopa").into_iter().map(|x| x as i32).collect(), (smash::hash40("param_breath"), smash::hash40("life"), 70.0));
	 crate::param_cache::update_float_2(-*WEAPON_KIND_KOOPA_BREATH, get_marked_costumes("koopa","koopa").into_iter().map(|x| x as i32).collect(), (smash::hash40("param_breath"), smash::hash40("hit_frames"), 70.0));
	 crate::param_cache::update_float_2(-*WEAPON_KIND_KOOPA_BREATH, get_marked_costumes("koopa","koopa").into_iter().map(|x| x as i32).collect(), (smash::hash40("param_breath"), smash::hash40("min_speed"), 0.8));
	 crate::param_cache::update_float_2(-*WEAPON_KIND_KOOPA_BREATH, get_marked_costumes("koopa","koopa").into_iter().map(|x| x as i32).collect(), (smash::hash40("param_breath"), smash::hash40("max_speed"), 0.8));
	 crate::param_cache::update_float_2(*FIGHTER_KIND_KOOPA, get_marked_costumes("koopa","koopa").into_iter().map(|x| x as i32).collect(), (smash::hash40("landing_attack_air_frame_n"), 0, 12.0));
	 crate::param_cache::update_float_2(*FIGHTER_KIND_KOOPA, get_marked_costumes("koopa","koopa").into_iter().map(|x| x as i32).collect(), (smash::hash40("landing_attack_air_frame_lw"), 0, 18.0));
	 crate::param_cache::update_float_2(*FIGHTER_KIND_KOOPA, get_marked_costumes("koopa","koopa").into_iter().map(|x| x as i32).collect(), (smash::hash40("landing_frame"), 0, 4.0));

}
