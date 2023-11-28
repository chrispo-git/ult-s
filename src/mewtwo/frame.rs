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
use crate::mewtwo::*;
use super::*;
pub fn install() {
    smashline::install_agent_frames!(
        mew2_frame
    );
}
#[fighter_frame( agent = FIGHTER_KIND_MEWTWO )]
fn mew2_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent); 
		if is_default(boma) {
			let status_kind = smash::app::lua_bind::StatusModule::status_kind(boma);
			let frame = MotionModule::frame(boma);
			let ENTRY_ID = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
			let cancel_frame = FighterMotionModuleImpl::get_cancel_frame(boma,smash::phx::Hash40::new_raw(MotionModule::motion_kind(boma)),false) as f32;
			let motion_kind = MotionModule::motion_kind(boma);
			if (status_kind == *FIGHTER_STATUS_KIND_FALL_SPECIAL || status_kind == *FIGHTER_MEWTWO_STATUS_KIND_SPECIAL_HI_3 )&& !HAS_ALREADY_TELECANCEL[ENTRY_ID] {
				ATTACK_AIR_WINDOW[ENTRY_ID] += 1;
			} else {
				ATTACK_AIR_WINDOW[ENTRY_ID] = 0;
			};
			if ATTACK_AIR_WINDOW[ENTRY_ID] > 3 && ATTACK_AIR_WINDOW[ENTRY_ID] < MAX_ATTACK_AIR_WINDOW && StatusModule::situation_kind(boma) == *SITUATION_KIND_AIR{
				if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_ATTACK) {
					HAS_ATTACK_AIR[ENTRY_ID] = true;
					HAS_ALREADY_TELECANCEL[ENTRY_ID] = true;
					WorkModule::set_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT_MAX, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT);
					WorkModule::set_flag(boma, true, *FIGHTER_INSTANCE_WORK_ID_FLAG_DISABLE_ESCAPE_AIR);
					StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_AIR, true);
				};
			};
			if StatusModule::situation_kind(boma) != *SITUATION_KIND_AIR {
					HAS_ALREADY_TELECANCEL[ENTRY_ID] = false;
					HAS_ATTACK_AIR[ENTRY_ID] = false;
			};
			if [*FIGHTER_STATUS_KIND_DEAD, *FIGHTER_STATUS_KIND_LOSE, *FIGHTER_STATUS_KIND_WIN].contains(&status_kind) || smash::app::sv_information::is_ready_go() == false{
				HAS_ATTACK_AIR[ENTRY_ID] = false;
				ATTACK_AIR_WINDOW[ENTRY_ID] = 0;
			};
		}
    }
}