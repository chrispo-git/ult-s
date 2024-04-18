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
use crate::ken::*;
use super::*;

pub fn install() {
    Agent::new("ken")
	.on_line(Main, supers)
	.install();
}

pub(crate) unsafe fn is_attack_btn(boma: &mut smash::app::BattleObjectModuleAccessor) -> bool {
	return (ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_SPECIAL_RAW)  && ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_ATTACK_RAW)) &&  
	((ControlModule::check_button_off(boma, *CONTROL_PAD_BUTTON_CATCH) && ControlModule::check_button_off(boma, *CONTROL_PAD_BUTTON_GUARD)) || 
	StatusModule::situation_kind(boma) == *SITUATION_KIND_GROUND )
}


//KEN SUPER
unsafe extern "C" fn supers(fighter : &mut L2CFighterCommon) {
    unsafe {
        let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);  
		let fighter_kind = smash::app::utility::get_kind(boma);
		let status_kind = smash::app::lua_bind::StatusModule::status_kind(boma);
		let lua_state = fighter.lua_state_agent;
		let color_id = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);
		let motion_kind = MotionModule::motion_kind(boma);
		let frame = MotionModule::frame(boma);
		let cancel_frame = FighterMotionModuleImpl::get_cancel_frame(boma,smash::phx::Hash40::new_raw(MotionModule::motion_kind(boma)),false) as f32;
		let mut ENTRY_ID = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
		if is_default(boma) {
			let meter_half = KEN_MAX_METER as f32 * 0.5;
			KEN_FX_TIMER[ENTRY_ID] += 1;
			if smash::app::smashball::is_training_mode() == true {
				if status_kind == *FIGHTER_STATUS_KIND_APPEAL {
					KEN_SUPER[ENTRY_ID] = KEN_MAX_METER;
					CancelModule::enable_cancel(boma);
				};
			};
			if KEN_FX_TIMER[ENTRY_ID] == 6 {
				let meter_1: u32 = EffectModule::req_follow(boma, smash::phx::Hash40::new("sys_damage_elec"), smash::phx::Hash40::new("haver"), &HANDS, &HANDS, 0.2, true, 0, 0, 0, 0, 0, true, true) as u32;
				let meter_2: u32 = EffectModule::req_follow(boma, smash::phx::Hash40::new("sys_damage_elec"), smash::phx::Hash40::new("havel"), &HANDS, &HANDS, 0.2, true, 0, 0, 0, 0, 0, true, true) as u32;
				let meter_3: u32 = EffectModule::req_follow(boma, smash::phx::Hash40::new("sys_damage_elec"), smash::phx::Hash40::new("footr"), &FEET, &FEET, 0.25, true, 0, 0, 0, 0, 0, true, true) as u32;
				let meter_4: u32 = EffectModule::req_follow(boma, smash::phx::Hash40::new("sys_damage_elec"), smash::phx::Hash40::new("footl"), &FEET, &FEET, 0.25, true, 0, 0, 0, 0, 0, true, true) as u32;
				if KEN_SUPER[ENTRY_ID] < KEN_MAX_METER && KEN_SUPER[ENTRY_ID] >= meter_half as i32 {
					EffectModule::set_rgb(boma, meter_1, 5.0, 2.75, 0.0);
                    EffectModule::set_rgb(boma, meter_2, 5.0, 2.75, 0.0);
					EffectModule::set_visible(boma, meter_1, true);
					EffectModule::set_visible(boma, meter_2, true);
					EffectModule::set_rgb(boma, meter_3, 5.0, 2.75, 0.0);
                    EffectModule::set_rgb(boma, meter_4, 5.0, 2.75, 0.0);
					EffectModule::set_visible(boma, meter_3, true);
					EffectModule::set_visible(boma, meter_4, true);
					if color_id == 7 {
						EffectModule::set_rgb(boma, meter_3, 0.0, 7.75, 0.0);
						EffectModule::set_rgb(boma, meter_4, 0.0, 7.75, 0.0);
						EffectModule::set_rgb(boma, meter_1, 0.0, 7.75, 0.0);
						EffectModule::set_rgb(boma, meter_2, 0.0, 7.75, 0.0);	
					};
				} else if KEN_SUPER[ENTRY_ID] >= KEN_MAX_METER {
					EffectModule::set_rgb(boma, meter_1, 0.0, 5.0, 5.0);
                    EffectModule::set_rgb(boma, meter_2, 0.0, 5.0, 5.0);
					EffectModule::set_visible(boma, meter_1, true);
					EffectModule::set_visible(boma, meter_2, true);
					EffectModule::set_rgb(boma, meter_3, 0.0, 5.0, 5.0);
                    EffectModule::set_rgb(boma, meter_4, 0.0, 5.0, 5.0);
					EffectModule::set_visible(boma, meter_3, true);
					EffectModule::set_visible(boma, meter_4, true);
					if color_id == 7 {
						EffectModule::set_rgb(boma, meter_3, 10.0, 0.0, 0.0);
						EffectModule::set_rgb(boma, meter_4, 10.0, 0.0, 0.0);
						EffectModule::set_rgb(boma, meter_1, 10.0, 0.0, 0.0);
						EffectModule::set_rgb(boma, meter_2, 10.0, 0.0, 0.0);	
					};
				} else {
					EffectModule::set_visible(boma, meter_1, false);
					EffectModule::set_visible(boma, meter_2, false);
					EffectModule::set_visible(boma, meter_3, false);
					EffectModule::set_visible(boma, meter_4, false)
				}
				KEN_FX_TIMER[ENTRY_ID] = 0;
			};
			if [*FIGHTER_STATUS_KIND_ENTRY, *FIGHTER_STATUS_KIND_WIN].contains(&status_kind) {
				println!("Reset!");
				KEN_SUPER[ENTRY_ID] = 0;
				KEN_FX_TIMER[ENTRY_ID] = 0;
				KEN_IS_EX[ENTRY_ID] = false;
			};
			if !AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_ALL) {
				HAS_ADDED[ENTRY_ID] = false;
			};
			if AttackModule::is_infliction(boma, *COLLISION_KIND_MASK_ALL) && status_kind != *FIGHTER_STATUS_KIND_CATCH_ATTACK && !HAS_ADDED[ENTRY_ID] {
				KEN_SUPER[ENTRY_ID] += 3;
				println!("attacks! {} ", KEN_SUPER[ENTRY_ID]);
				HAS_ADDED[ENTRY_ID] = true;
			};
			if KEN_SUPER[ENTRY_ID] >= KEN_MAX_METER{
				KEN_SUPER[ENTRY_ID] = KEN_MAX_METER;
			};
			if status_kind == *FIGHTER_RYU_STATUS_KIND_SPECIAL_LW_STEP_F || status_kind == *FIGHTER_RYU_STATUS_KIND_SPECIAL_LW_STEP_B {
				if EX_DOWNB[ENTRY_ID] {
					if MotionModule::frame(boma) < 2.0 {
						HitModule::set_whole(boma, smash::app::HitStatus(*HIT_STATUS_XLU), 0);
					};
					if MotionModule::frame(boma) > 3.0 {
						HitModule::set_whole(boma, smash::app::HitStatus(*HIT_STATUS_NORMAL), 0);
					};
					if MotionModule::frame(boma) > 10.0 {
						CancelModule::enable_cancel(boma);
					};
				};
			} else {
				EX_DOWNB[ENTRY_ID] = false;
			};
			if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_LW {
				let mut stick_x = ControlModule::get_stick_x(boma) ;
				stick_x *= PostureModule::lr(boma);
				let is_eligble_meter = (4..25).contains(&(MotionModule::frame(boma) as i32)) && KEN_SUPER[ENTRY_ID] >= meter_half as i32;
				
				if MotionModule::frame(boma) > 25.0 || is_eligble_meter {
					if stick_x >= 0.665 {
						if is_eligble_meter {
							KEN_SUPER[ENTRY_ID] -= meter_half as i32;
							println!("meter spent! {} ", KEN_SUPER[ENTRY_ID]);
							EX_DOWNB[ENTRY_ID] = true;
						}; 
						StatusModule::change_status_request_from_script(boma, *FIGHTER_RYU_STATUS_KIND_SPECIAL_LW_STEP_F, true);
					} else if stick_x <= -0.665 {
						if is_eligble_meter {
							KEN_SUPER[ENTRY_ID] -= meter_half as i32;
							println!("meter spent! {} ", KEN_SUPER[ENTRY_ID]);
							EX_DOWNB[ENTRY_ID] = true;
						};
						StatusModule::change_status_request_from_script(boma, *FIGHTER_RYU_STATUS_KIND_SPECIAL_LW_STEP_B, true);
					};
				};
			};
			if motion_kind == hash40("attack_near_w") && AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT) && !is_hitlag(boma) {
				if (ControlModule::get_command_flag_cat(boma, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_HI3) != 0 {
					StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_HI3, true);
				} else if (ControlModule::get_command_flag_cat(boma, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_LW3) != 0 {
					StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_LW3, true);
				}else if (ControlModule::get_command_flag_cat(boma, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_S3) != 0 {
					StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_DASH, true);
				};
			};
			if [hash40("attack_s3_s_s"), hash40("attack_s3_s_w"), hash40("attack_lw3_s"), hash40("attack_hi3_s")].contains(&motion_kind) && AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_ALL) && !is_hitlag(boma) && KEN_SUPER[ENTRY_ID] >= meter_half as i32 && cancel_frame-frame > 9.0 && ControlModule::check_button_off(boma, *CONTROL_PAD_BUTTON_SPECIAL){
				let mut stick_x = ControlModule::get_stick_x(boma) ;
				stick_x *= PostureModule::lr(boma);
				if stick_x >= 0.665 && (ControlModule::get_command_flag_cat(boma, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_DASH) != 0 {
					KEN_SUPER[ENTRY_ID] -= meter_half as i32;
					println!("meter spent! {} ", KEN_SUPER[ENTRY_ID]);
					EX_DOWNB[ENTRY_ID] = true;
					StatusModule::change_status_request_from_script(boma, *FIGHTER_RYU_STATUS_KIND_SPECIAL_LW_STEP_F, true);
				} else if stick_x <= -0.665 {
					println!("meter spent! {} ", KEN_SUPER[ENTRY_ID]);
					KEN_SUPER[ENTRY_ID] -= meter_half as i32;
					EX_DOWNB[ENTRY_ID] = true;
					StatusModule::change_status_request_from_script(boma, *FIGHTER_RYU_STATUS_KIND_SPECIAL_LW_STEP_B, true);
				};
			};

			//EX
			if KEN_SUPER[ENTRY_ID] >= meter_half as i32 && !AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_ALL) {
				if [*FIGHTER_STATUS_KIND_SPECIAL_HI].contains(&status_kind) {
					if [hash40("special_hi"), hash40("special_air_hi")].contains(&motion_kind) && MotionModule::frame(boma) < 5.0 {
						if is_attack_btn(boma) {
							KEN_SUPER[ENTRY_ID] -= meter_half as i32;
							println!("meter spent! {} ", KEN_SUPER[ENTRY_ID]);
							MotionModule::change_motion(boma, Hash40::new("special_hi_ex"), -1.0, 1.0, false, 0.0, false, false);
						};
					};
				};
				if [*FIGHTER_STATUS_KIND_SPECIAL_S].contains(&status_kind) {
					if [hash40("special_s_start"), hash40("special_air_s_start")].contains(&motion_kind) && MotionModule::frame(boma) < 5.0 {
						if is_attack_btn(boma) {
							KEN_SUPER[ENTRY_ID] -= meter_half as i32;
							println!("meter spent! {} ", KEN_SUPER[ENTRY_ID]);
							MotionModule::change_motion(boma, Hash40::new("special_s_ex"), -1.0, 1.0, false, 0.0, false, false);
						};
					};
				};
				if ![*FIGHTER_STATUS_KIND_SPECIAL_S, *FIGHTER_RYU_STATUS_KIND_SPECIAL_S_COMMAND, *FIGHTER_RYU_STATUS_KIND_SPECIAL_S_LOOP, *FIGHTER_RYU_STATUS_KIND_SPECIAL_S_END].contains(&status_kind) {
					EffectModule::kill_kind(boma, Hash40::new("ken_tatsumaki_wind_r"), false, false);
					EffectModule::kill_kind(boma, Hash40::new("ken_tatsumaki_wind_l"), false, false);
				};
			};
			if [hash40("special_s_ex")].contains(&motion_kind) {
				if MotionModule::frame(boma) >= 63.0 {
					StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_FALL, false);
				};
				if MotionModule::frame(boma) >= 2.0 {
					StatusModule::set_situation_kind(boma, smash::app::SituationKind(*SITUATION_KIND_AIR), true);
					StatusModule::set_keep_situation_air(boma, true);
					if KineticModule::get_kinetic_type(boma) != *FIGHTER_KINETIC_TYPE_MOTION_AIR {
						KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_MOTION_AIR);
					};
				};
			};

			//Focus
			if [*FIGHTER_STATUS_KIND_SPECIAL_HI, *FIGHTER_RYU_STATUS_KIND_SPECIAL_HI_COMMAND].contains(&status_kind)  && MotionModule::frame(boma) < 8.0 && (ControlModule::get_command_flag_cat(boma, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_LW) != 0 && KEN_SUPER[ENTRY_ID] >= meter_half as i32  {
				StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_SPECIAL_LW, true);
			};
			if [*FIGHTER_STATUS_KIND_SPECIAL_N, *FIGHTER_RYU_STATUS_KIND_SPECIAL_N_COMMAND].contains(&status_kind) && MotionModule::frame(boma) < 14.0 && (ControlModule::get_command_flag_cat(boma, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_LW) != 0 && KEN_SUPER[ENTRY_ID] >= meter_half as i32  {
				StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_SPECIAL_LW, true);
			};
			if [*FIGHTER_STATUS_KIND_SPECIAL_S, *FIGHTER_RYU_STATUS_KIND_SPECIAL_S_COMMAND, *FIGHTER_RYU_STATUS_KIND_SPECIAL_S_LOOP, *FIGHTER_RYU_STATUS_KIND_SPECIAL_S_END].contains(&status_kind) && StatusModule::is_situation_changed(boma) && StatusModule::situation_kind(boma) == *SITUATION_KIND_GROUND {
				StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_LANDING, true);
			};
		};
	};
}	