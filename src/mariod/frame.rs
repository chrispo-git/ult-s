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
use crate::mariod::*;
use super::*;
pub fn install() {
	smashline::install_agent_frames!(mariod_frame);
}
#[fighter_frame(agent = FIGHTER_KIND_MARIOD)]
pub fn mariod_frame(fighter : &mut L2CFighterCommon) {
    unsafe {
			let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent); 
			if is_default(boma) {
				let status_kind = smash::app::lua_bind::StatusModule::status_kind(boma);
				let ENTRY_ID = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
				let fighter_kind = smash::app::utility::get_kind(boma);

				if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_S && StatusModule::is_situation_changed(boma) && AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_SHIELD) == false {
					StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_LANDING, true);
				};
				if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_S && MotionModule::frame(boma) == 8.0 {
						macros::SET_SPEED_EX(fighter, 1.1, -0.25, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
				};
				if [hash40("special_hi"),hash40("special_air_hi")].contains(&MotionModule::motion_kind(boma)) {
					if MotionModule::frame(boma) >= 42.0 {
						StatusModule::change_status_request_from_script(boma, *FIGHTER_MARIOD_STATUS_SPECIAL_S_FLAG_SPECIAL_FALL, true);
					};
					if MotionModule::frame(boma) >= 19.0 && ControlModule::check_button_on_trriger(boma, *CONTROL_PAD_BUTTON_SPECIAL) {
						HAS_BUFFER_B[ENTRY_ID] = true;
					};
					if MotionModule::frame(boma) >= 26.0 && ControlModule::check_button_on_trriger(boma, *CONTROL_PAD_BUTTON_SPECIAL) {
						MotionModule::change_motion(boma, smash::phx::Hash40::new("special_hi_2"), 0.0, 1.0, false, 0.0, false, false);
						HAS_BUFFER_B[ENTRY_ID] = false;
					};
					if MotionModule::frame(boma) >= 28.0 && HAS_BUFFER_B[ENTRY_ID] {
						MotionModule::change_motion(boma, smash::phx::Hash40::new("special_hi_2"), 0.0, 1.0, false, 0.0, false, false);
						HAS_BUFFER_B[ENTRY_ID] = false;
					};
				};
				if [hash40("special_input")].contains(&MotionModule::motion_kind(boma)) {
					macros::SET_SPEED_EX(fighter, 0.0, 0.5, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
					if MotionModule::frame(boma) >= 17.0 {
						MotionModule::change_motion(boma, smash::phx::Hash40::new("special_hi_2"), 0.0, 1.0, false, 0.0, false, false);
					};
				};
				if [hash40("special_hi_2")].contains(&MotionModule::motion_kind(boma)) {
					if MotionModule::frame(boma) >= 0.0 {
						macros::SET_SPEED_EX(fighter,0.0, -5.25, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
					};
					UPB_FALL[ENTRY_ID] = true;
					let stick_y = ControlModule::get_stick_y(boma);
					if stick_y <= -0.5 {
						GroundModule::pass_floor(boma);
						if ray_check_pos(boma, 0.0, -3.0, false) == 1 {
							MotionModule::change_motion(boma, smash::phx::Hash40::new("special_hi_landing"), 0.0, 1.0, false, 0.0, false, false);
							StatusModule::set_situation_kind(boma, smash::app::SituationKind(*SITUATION_KIND_GROUND), true);
						};
					}else {
						GroundModule::clear_pass_floor(boma);
						if ray_check_pos(boma, 0.0, -3.0, true) == 1 {
							MotionModule::change_motion(boma, smash::phx::Hash40::new("special_hi_landing"), 0.0, 1.0, false, 0.0, false, false);
							StatusModule::set_situation_kind(boma, smash::app::SituationKind(*SITUATION_KIND_GROUND), true);
						};
					};
				};
				if ![*FIGHTER_STATUS_KIND_SPECIAL_HI, *FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL].contains(&status_kind) {
					UPB_FALL[ENTRY_ID] = false;
					HAS_BUFFER_B[ENTRY_ID] = false;
				};
				if UPB_FALL[ENTRY_ID] == true && (status_kind == *FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL || [hash40("special_hi"),hash40("special_air_hi")].contains(&MotionModule::motion_kind(boma))) {
					MotionModule::change_motion(boma, smash::phx::Hash40::new("special_hi_landing"), 0.0, 1.0, false, 0.0, false, false);
					StatusModule::set_situation_kind(boma, smash::app::SituationKind(*SITUATION_KIND_GROUND), true);
					UPB_FALL[ENTRY_ID] = false;
				};
				if [hash40("special_hi_landing")].contains(&MotionModule::motion_kind(boma)) {
					if MotionModule::frame(boma) >= 40.0 {
						StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_WAIT, true);
					};
				};
			}	
	};
}