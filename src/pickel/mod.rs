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

	 crate::param_cache::update_float_2(*FIGHTER_KIND_PICKEL, get_marked_costumes("pickel","pickel").into_iter().map(|x| x as i32).collect(), (smash::hash40("param_special_n"), 0x1a89765685u64, 2.5));
	 crate::param_cache::update_float_2(*FIGHTER_KIND_PICKEL, get_marked_costumes("pickel","pickel").into_iter().map(|x| x as i32).collect(), (smash::hash40("param_special_n"), 0x1a0a95e89au64, 0.93));
	 crate::param_cache::update_float_2(*FIGHTER_KIND_PICKEL, get_marked_costumes("pickel","pickel").into_iter().map(|x| x as i32).collect(), (smash::hash40("param_special_n"), 0x1a1bb90606u64, 1.25));
	 crate::param_cache::update_float_2(*FIGHTER_KIND_PICKEL, get_marked_costumes("pickel","pickel").into_iter().map(|x| x as i32).collect(), (smash::hash40("param_special_n"), 0x1b78f1e855u64, 1.1875));
	 crate::param_cache::update_float_2(*FIGHTER_KIND_PICKEL, get_marked_costumes("pickel","pickel").into_iter().map(|x| x as i32).collect(), (smash::hash40("param_special_n"), 0x1aa714f1cau64, 1.125));
	 crate::param_cache::update_float_2(*FIGHTER_KIND_PICKEL, get_marked_costumes("pickel","pickel").into_iter().map(|x| x as i32).collect(), (smash::hash40("param_special_n"), 0x1da2122271u64, 1.0625));
	crate::param_cache::update_int_2(-*WEAPON_KIND_PICKEL_TROLLEY, get_marked_costumes("pickel","pickel").into_iter().map(|x| x as i32).collect(), (smash::hash40("param_trolley"), smash::hash40("lifetime"), 50));
	 crate::param_cache::update_float_2(-*WEAPON_KIND_PICKEL_TROLLEY, get_marked_costumes("pickel","pickel").into_iter().map(|x| x as i32).collect(), (smash::hash40("param_trolley"), smash::hash40("catch_time"), 21.0));
	 crate::param_cache::update_float_2(-*WEAPON_KIND_PICKEL_TROLLEY, get_marked_costumes("pickel","pickel").into_iter().map(|x| x as i32).collect(), (smash::hash40("param_trolley"), smash::hash40("catch_time_add_by_damage"), 0.0));
	 crate::param_cache::update_float_2(-*WEAPON_KIND_PICKEL_TROLLEY, get_marked_costumes("pickel","pickel").into_iter().map(|x| x as i32).collect(), (smash::hash40("param_trolley"), smash::hash40("catch_time_dec_by_clatter"), 0.0));
	crate::param_cache::update_int_2(-*WEAPON_KIND_PICKEL_TROLLEY, get_marked_costumes("pickel","pickel").into_iter().map(|x| x as i32).collect(), (smash::hash40("param_trolley"), smash::hash40("lifetime"), 50));
	 crate::param_cache::update_float_2(-*WEAPON_KIND_PICKEL_TROLLEY, get_marked_costumes("pickel","pickel").into_iter().map(|x| x as i32).collect(), (smash::hash40("param_trolley"), smash::hash40("catch_time"), 21.0));
	 crate::param_cache::update_float_2(-*WEAPON_KIND_PICKEL_TROLLEY, get_marked_costumes("pickel","pickel").into_iter().map(|x| x as i32).collect(), (smash::hash40("param_trolley"), smash::hash40("catch_time_add_by_damage"), 0.0));
	 crate::param_cache::update_float_2(-*WEAPON_KIND_PICKEL_TROLLEY, get_marked_costumes("pickel","pickel").into_iter().map(|x| x as i32).collect(), (smash::hash40("param_trolley"), smash::hash40("catch_time_dec_by_clatter"), 0.0));
	 crate::param_cache::update_float_2(*FIGHTER_KIND_PICKEL, get_marked_costumes("pickel","pickel").into_iter().map(|x| x as i32).collect(), (smash::hash40("param_special_hi"), smash::hash40("start_speed_x_mul_ground"), 0.8));
	 crate::param_cache::update_float_2(*FIGHTER_KIND_PICKEL, get_marked_costumes("pickel","pickel").into_iter().map(|x| x as i32).collect(), (smash::hash40("param_special_hi"), smash::hash40("start_speed_x_mul_air"), 0.8));
	 crate::param_cache::update_float_2(*FIGHTER_KIND_PICKEL, get_marked_costumes("pickel","pickel").into_iter().map(|x| x as i32).collect(), (smash::hash40("param_special_hi"), smash::hash40("start_control_accel_x_mul_air"), 0.8));
	 crate::param_cache::update_float_2(*FIGHTER_KIND_PICKEL, get_marked_costumes("pickel","pickel").into_iter().map(|x| x as i32).collect(), (smash::hash40("param_special_hi"), smash::hash40("start_control_max_speed_x_mul_air"), 0.8));
	crate::param_cache::update_int_2(*FIGHTER_KIND_PICKEL, get_marked_costumes("pickel","pickel").into_iter().map(|x| x as i32).collect(), (smash::hash40("param_private"), smash::hash40("material_gold_max"), 6));
	crate::param_cache::update_int_2(*FIGHTER_KIND_PICKEL, get_marked_costumes("pickel","pickel").into_iter().map(|x| x as i32).collect(), (smash::hash40("param_private"), smash::hash40("material_red_stone_max"), 7));
	crate::param_cache::update_int_2(*FIGHTER_KIND_PICKEL, get_marked_costumes("pickel","pickel").into_iter().map(|x| x as i32).collect(), (smash::hash40("param_private"), smash::hash40("material_diamond_max"), 4));
	crate::param_cache::update_int_2(*FIGHTER_KIND_PICKEL, get_marked_costumes("pickel","pickel").into_iter().map(|x| x as i32).collect(), (smash::hash40("param_private"), smash::hash40("start_material_grade_1_num"), 25));
	crate::param_cache::update_int_2(*FIGHTER_KIND_PICKEL, get_marked_costumes("pickel","pickel").into_iter().map(|x| x as i32).collect(), (smash::hash40("param_private"), smash::hash40("start_material_iron_num"), 2));
	crate::param_cache::update_int_2(*FIGHTER_KIND_PICKEL, get_marked_costumes("pickel","pickel").into_iter().map(|x| x as i32).collect(), (smash::hash40("param_private"), smash::hash40("start_material_red_stone_num"), 0));
	crate::param_cache::update_int_2(*FIGHTER_KIND_PICKEL, get_marked_costumes("pickel","pickel").into_iter().map(|x| x as i32).collect(), (smash::hash40("param_private"), smash::hash40("start_material_grade_1_num_kirby"), 25));
	 crate::param_cache::update_float_2(-*FIGHTER_KIND_PICKEL, get_marked_costumes("pickel","pickel").into_iter().map(|x| x as i32).collect(), (smash::hash40("param_pickelobject"), smash::hash40("auto_damage"), 0.05));
	 crate::param_cache::update_float_2(*FIGHTER_KIND_PICKEL, get_marked_costumes("pickel","pickel").into_iter().map(|x| x as i32).collect(), (smash::hash40("dash_speed"), 0, 1.5));
	 crate::param_cache::update_float_2(*FIGHTER_KIND_PICKEL, get_marked_costumes("pickel","pickel").into_iter().map(|x| x as i32).collect(), (smash::hash40("jump_speed_x_mul"), 0, 1.05));
	 crate::param_cache::update_float_2(*FIGHTER_KIND_PICKEL, get_marked_costumes("pickel","pickel").into_iter().map(|x| x as i32).collect(), (smash::hash40("air_speed_x_stable"), 0, 0.91));

}