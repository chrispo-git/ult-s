use smash::app::sv_animcmd::*;
use smash::phx::Hash40;
use smash::app::lua_bind::*;
use smash::lib::lua_const::*;
use smash::app::utility::get_kind;
use smash::hash40;
use smash::lua2cpp::*;
use smashline::*;
use smash_script::*;
use smash::lib::{L2CValue, L2CAgent};
use std::mem;
use smash::app::*;
use smash::phx::Vector3f;
use crate::util::*;
use super::*;

pub fn install() {
    Agent::new("packun")
    .on_line(Main, plant_frame)
    .install();

	Agent::new("packun_poisonbreath")
    .on_line(Main, poison_frame)
    .install();
}

unsafe extern "C" fn plant_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent); 
		if is_default(boma) {
			let status_kind = smash::app::lua_bind::StatusModule::status_kind(boma);
			let motion_kind = MotionModule::motion_kind(boma);
			let frame = MotionModule::frame(boma);
			let ENTRY_ID = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
			let situation_kind = StatusModule::situation_kind(boma);
			let end_frame = MotionModule::end_frame(boma);
			let cancel_frame = FighterMotionModuleImpl::get_cancel_frame(boma,smash::phx::Hash40::new_raw(MotionModule::motion_kind(boma)),false) as f32;
			if smash::app::sv_information::is_ready_go() == false {
				BREATH_POS_X[ENTRY_ID] = 0.0;
				BREATH_POS_Y[ENTRY_ID] = 0.0;
				IS_BAIR[ENTRY_ID] = false;
				IS_SIDEB_EAT[ENTRY_ID] = false;
			};
			if !ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_PACKUN_GENERATE_ARTICLE_POISONBREATH) {
				BREATH_POS_X[ENTRY_ID] = 0.0;
				BREATH_POS_Y[ENTRY_ID] = 0.0;
			} else {
				if [hash40("special_s_shoot"), hash40("special_air_s_shoot")].contains(&motion_kind) {
					ModelModule::set_mesh_visibility(boma, Hash40::new("heada"), true);
        			ModelModule::set_mesh_visibility(boma, Hash40::new("headb"), false);
				}
			}


			if ![hash40("attack_air_b"), hash40("special_s_shoot"), hash40("special_air_s_shoot")].contains(&motion_kind) {
				IS_BAIR[ENTRY_ID] = false;
				IS_SIDEB_EAT[ENTRY_ID] = false;
			}
		}
	}
}
unsafe extern "C" fn poison_frame(weapon: &mut L2CFighterBase) {
    unsafe {
        let otarget_id = WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER) as u32;
        let boma = smash::app::sv_battle_object::module_accessor(otarget_id);
		let status_kind = smash::app::lua_bind::StatusModule::status_kind(weapon.module_accessor);
		let motion_kind = MotionModule::motion_kind(weapon.module_accessor);
		let parent_motion_kind = MotionModule::motion_kind(&mut *boma);
		let ENTRY_ID = WorkModule::get_int(&mut *boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
        if smash::app::utility::get_kind(&mut *boma) == *FIGHTER_KIND_PACKUN {
			BREATH_POS_X[ENTRY_ID] = PostureModule::pos_x(weapon.module_accessor);
			BREATH_POS_Y[ENTRY_ID] = PostureModule::pos_y(weapon.module_accessor);
			let scale = PostureModule::scale(weapon.module_accessor);
			let lr = PostureModule::lr(&mut *boma);
			let pos_x = PostureModule::pos_x(&mut *boma)+(-11.0*lr);
			let pos_x2 = PostureModule::pos_x(&mut *boma)+(28.0*lr);
			let pos_y = PostureModule::pos_y(&mut *boma)+4.0;
			//println!("Breath Pos [{},{}] Plant Pos [{}, {}]", BREATH_POS_X[ENTRY_ID], BREATH_POS_Y[ENTRY_ID], pos_x, pos_y);
			if ((BREATH_POS_X[ENTRY_ID]  - pos_x).abs() < 9.0*scale) &&
				((BREATH_POS_Y[ENTRY_ID]  - pos_y).abs() < 9.0*scale) &&
				BREATH_POS_Y[ENTRY_ID] != 0.0 && 
				IS_BAIR[ENTRY_ID] &&
				motion_kind != hash40("explode")
				{
					//println!("Woo!");
					MotionModule::change_motion(weapon.module_accessor, Hash40::new("explode"), 0.0, 1.0, false, 0.0, false, false);
			}else if ((BREATH_POS_X[ENTRY_ID]  - pos_x2).abs() < 9.0*scale) &&
				((BREATH_POS_Y[ENTRY_ID]  - pos_y).abs() < 9.0*scale) &&
				BREATH_POS_Y[ENTRY_ID] != 0.0 && 
				IS_SIDEB_EAT[ENTRY_ID] &&
				motion_kind != hash40("explode")
				{
					//println!("Woo!");
					MotionModule::change_motion(weapon.module_accessor, Hash40::new("explode"), 0.0, 1.0, false, 0.0, false, false);
			}
		};
    }
}