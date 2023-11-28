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
use smash::phx::Vector2f;
use crate::util::*;
use super::*;

#[fighter_frame_callback]
pub fn sword(fighter : &mut L2CFighterCommon) {
    unsafe {
        let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);  
		let fighter_kind = smash::app::utility::get_kind(boma);
		let status_kind = smash::app::lua_bind::StatusModule::status_kind(boma);
		let ENTRY_ID = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
		let frame = MotionModule::frame(boma);
		let end_frame = MotionModule::end_frame(boma);
		if fighter_kind == *FIGHTER_KIND_MIISWORDSMAN && is_default(boma) {
			if [*FIGHTER_STATUS_KIND_DEAD, *FIGHTER_STATUS_KIND_LOSE, *FIGHTER_STATUS_KIND_WIN].contains(&status_kind) || smash::app::sv_information::is_ready_go() == false{
				COUNTER_STORE[ENTRY_ID] = false;
				BOMB_TIME[ENTRY_ID] = 0;
				CUSTOM_BOMB[ENTRY_ID] = false;
				NADO_COOLDOWN[ENTRY_ID] = 0;
			};
			if NADO_COOLDOWN[ENTRY_ID] > 0 {
				NADO_COOLDOWN[ENTRY_ID] -= 1;
			};

			/*if [*FIGHTER_STATUS_KIND_ATTACK_AIR, *FIGHTER_STATUS_KIND_ATTACK_LW3].contains(&status_kind) && CUSTOM_BOMB[ENTRY_ID] == true {
				MotionModule::change_motion(boma, smash::phx::Hash40::new("catch"), 9.0, 1.0, false, 0.0, false, false);
				MotionModule::set_rate(boma, 0.8);
				CUSTOM_BOMB[ENTRY_ID] = false;
			};
			if [hash40("special_n3_start"), hash40("special_air_n3_start")].contains(&MotionModule::motion_kind(boma)){
				if BOMB_TIME[ENTRY_ID] == 0 {
					ItemModule::have_item(fighter.module_accessor, ItemKind(*ITEM_KIND_BOMBCHU), 0, 0, false, false);
					if StatusModule::situation_kind(boma) == *SITUATION_KIND_AIR {
						StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_AIR, false);
						CUSTOM_BOMB[ENTRY_ID] = true;
					} else {
						StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_LW3, false);
						CUSTOM_BOMB[ENTRY_ID] = true;
					};
					BOMB_TIME[ENTRY_ID] = 120;
				} else {
					if StatusModule::situation_kind(boma) == *SITUATION_KIND_AIR {
						StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_AIR, false);
						CUSTOM_BOMB[ENTRY_ID] = true;
					} else {
						StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_LW3, false);
						CUSTOM_BOMB[ENTRY_ID] = true;
					};
				};
			};
			if [hash40("special_n3_end"), hash40("special_air_n3_end")].contains(&MotionModule::motion_kind(boma)){
				if MotionModule::frame(boma) < 6.0 {
					MotionModule::set_rate(boma, 4.0);
				} else {
					MotionModule::set_rate(boma, 1.0);
				};
			};*/
			if [hash40("special_air_n2")].contains(&MotionModule::motion_kind(boma)){
				if frame < 20.0 {
					KineticModule::clear_speed_all(boma);
				};
			};
			if [hash40("special_air_n3_start")].contains(&MotionModule::motion_kind(boma)){
				if frame >= 36.0 {
					StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_FALL, false);
				};
				if AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT) {
					MotionModule::change_motion(boma, smash::phx::Hash40::new("special_air_n3_loop"), 0.0, 1.0, false, 0.0, false, false);
				};
			};
			if [hash40("special_n3_start")].contains(&MotionModule::motion_kind(boma)){
				if frame >= 40.0 {
					StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_WAIT, false);
				};
				if AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT) {
					StatusModule::set_keep_situation_air(boma, true);
					StatusModule::set_situation_kind(boma, smash::app::SituationKind(*SITUATION_KIND_AIR), true);
					MotionModule::change_motion(boma, smash::phx::Hash40::new("special_air_n3_loop"), 3.0, 1.0, false, 0.0, false, false);
				};
			};
			if [hash40("special_air_n3_loop")].contains(&MotionModule::motion_kind(boma)){
				StatusModule::set_keep_situation_air(boma, true);
				if (GroundModule::ray_check(boma, &Vector2f{ x: PostureModule::pos_x(boma), y: PostureModule::pos_y(boma)}, &Vector2f{ x: 0.0, y: -1.5}, true) == 1 && frame > 10.0) {
					StatusModule::change_status_request_from_script(boma, *FIGHTER_MIISWORDSMAN_STATUS_KIND_SPECIAL_N3_END, false);
				};
				if frame > 30.0 {
					StatusModule::change_status_request_from_script(boma, *FIGHTER_MIISWORDSMAN_STATUS_KIND_SPECIAL_N3_END, false);
				};
			};
			if [hash40("special_air_n3_end")].contains(&MotionModule::motion_kind(boma)){
				StatusModule::set_keep_situation_air(boma, true);
				if KineticModule::get_kinetic_type(boma) != *FIGHTER_KINETIC_TYPE_JUMP {
					KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_JUMP);
				};
				if frame >= 22.0 {
					CancelModule::enable_cancel(boma);
				};
				if end_frame - frame < 3.0 {
					StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_FALL, true);
				};
			};
			if BOMB_TIME[ENTRY_ID] > 0 {
				BOMB_TIME[ENTRY_ID] -= 1;
			};
			if [hash40("special_s1_hit"), hash40("special_air_s1_hit")].contains(&MotionModule::motion_kind(boma)) {
				MotionModule::set_rate(boma, 1.42857142);
			};
			if status_kind == *FIGHTER_MIISWORDSMAN_STATUS_KIND_SPECIAL_LW1_HIT && COUNTER_STORE[ENTRY_ID] == false {
				if MotionModule::frame(boma) < 12.0 && ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_GUARD) {
					if StatusModule::situation_kind(boma) == *SITUATION_KIND_AIR {
						StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_FALL_AERIAL, false);
					} else {
						StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_GUARD_OFF, false);
					};
					COUNTER_STORE[ENTRY_ID] = true;
				};
			};
			if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_LW && COUNTER_STORE[ENTRY_ID] == true{
				StatusModule::change_status_request_from_script(boma, *FIGHTER_MIISWORDSMAN_STATUS_KIND_SPECIAL_LW1_HIT, false);
			};
		};
    };
}	

pub fn install() {
    smashline::install_agent_frame_callbacks!(sword);
}