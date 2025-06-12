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
use crate::jack::*;

pub fn install() {
	Agent::new("jack")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
	.on_line(Main, joker_frame)
	.install();
}

unsafe extern "C" fn joker_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent); 
		let status_kind = smash::app::lua_bind::StatusModule::status_kind(boma);
		let ENTRY_ID = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
		let cat1 = ControlModule::get_command_flag_cat(boma, 0);
		let motion_kind = MotionModule::motion_kind(boma);
		
		if is_default(boma) {
			if [*FIGHTER_STATUS_KIND_DEAD, *FIGHTER_STATUS_KIND_LOSE, *FIGHTER_STATUS_KIND_WIN, *FIGHTER_STATUS_KIND_ENTRY].contains(&status_kind) || smash::app::sv_information::is_ready_go() == false{
				BATON_TYPE[ENTRY_ID] = 2;
			};
			if ![*FIGHTER_STATUS_KIND_APPEAL, *FIGHTER_STATUS_KIND_SPECIAL_S, *FIGHTER_STATUS_KIND_WIN].contains(&status_kind) && smash::app::sv_information::is_ready_go(){
				ArticleModule::remove_exist(boma, *FIGHTER_JACK_GENERATE_ARTICLE_MONA,smash::app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
			};
		}
    }
}