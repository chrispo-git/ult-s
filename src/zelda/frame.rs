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

#[fighter_frame( agent = FIGHTER_KIND_ZELDA, main )]
fn zelda_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
		let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);   
		if is_default(boma) {
			let status_kind = smash::app::lua_bind::StatusModule::status_kind(boma);
			let ENTRY_ID = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize; 
			let frame = MotionModule::frame(boma);
			let end_frame = MotionModule::end_frame(boma);
			let is_exist = ArticleModule::is_exist(boma, *FIGHTER_ZELDA_GENERATE_ARTICLE_DEIN);
			if StatusModule::is_situation_changed(boma) && [*FIGHTER_ZELDA_STATUS_KIND_SPECIAL_S_LOOP, *FIGHTER_ZELDA_STATUS_KIND_SPECIAL_S_END, *FIGHTER_STATUS_KIND_SPECIAL_S].contains(&status_kind) {
				if is_exist {
					let article = ArticleModule::get_article(boma,  *FIGHTER_ZELDA_GENERATE_ARTICLE_DEIN);
					let article_id = smash::app::lua_bind::Article::get_battle_object_id(article) as u32;
					let article_boma = sv_battle_object::module_accessor(article_id);
					WorkModule::set_float(article_boma, 1.0, *WEAPON_ZELDA_DEIN_STATUS_WORK_FLOAT_LIFE);
				}
				WorkModule::set_float(boma, 13.0, *FIGHTER_INSTANCE_WORK_ID_FLOAT_LANDING_FRAME);
				StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL, true);
			};
		}
	}
}	

pub fn install() {
    smashline::install_agent_frames!(zelda_frame);
}