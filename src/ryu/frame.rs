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

//RYU SUPER
#[fighter_frame_callback]
pub fn supers(fighter : &mut L2CFighterCommon) {
    unsafe {
        let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);  
		let fighter_kind = smash::app::utility::get_kind(boma);
		let status_kind = smash::app::lua_bind::StatusModule::status_kind(boma);
		let lua_state = fighter.lua_state_agent;
		let color_id = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);
		let motion_kind = MotionModule::motion_kind(boma);
		let frame = MotionModule::frame(boma);
		let cancel_frame = FighterMotionModuleImpl::get_cancel_frame(boma,smash::phx::Hash40::new_raw(MotionModule::motion_kind(boma)),false) as f32;
		let mut fighter_num = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
		if fighter_num > 7 {
			fighter_num = 7;
		};
		let aura_size = RYU_SUPER[fighter_num] as f32 * 0.1;
		if fighter_kind == *FIGHTER_KIND_RYU || WorkModule::get_int(boma, *FIGHTER_KIRBY_INSTANCE_WORK_ID_INT_COPY_CHARA) == *FIGHTER_KIND_RYU {
			RYU_FX_TIMER[fighter_num] += 1;
			max_meter = 51; //51
			if smash::app::smashball::is_training_mode() == true {
				if status_kind == *FIGHTER_STATUS_KIND_APPEAL {
					RYU_SUPER[fighter_num] = max_meter;
					CancelModule::enable_cancel(boma);
				};
			};
			if RYU_FX_TIMER[fighter_num] == 6 {
				let meter_half = max_meter as f32 * 0.5;
				let meter_1: u32 = EffectModule::req_follow(boma, smash::phx::Hash40::new("sys_damage_elec"), smash::phx::Hash40::new("haver"), &HANDS, &HANDS, 0.2, true, 0, 0, 0, 0, 0, true, true) as u32;
				let meter_2: u32 = EffectModule::req_follow(boma, smash::phx::Hash40::new("sys_damage_elec"), smash::phx::Hash40::new("havel"), &HANDS, &HANDS, 0.2, true, 0, 0, 0, 0, 0, true, true) as u32;
				let meter_3: u32 = EffectModule::req_follow(boma, smash::phx::Hash40::new("sys_damage_elec"), smash::phx::Hash40::new("footr"), &FEET, &FEET, 0.25, true, 0, 0, 0, 0, 0, true, true) as u32;
				let meter_4: u32 = EffectModule::req_follow(boma, smash::phx::Hash40::new("sys_damage_elec"), smash::phx::Hash40::new("footl"), &FEET, &FEET, 0.25, true, 0, 0, 0, 0, 0, true, true) as u32;
				if RYU_SUPER[fighter_num] < max_meter && RYU_SUPER[fighter_num] >= meter_half as i32 {
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
				} else if RYU_SUPER[fighter_num] >= max_meter {
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
				RYU_FX_TIMER[fighter_num] = 0;
			};
				
			if [*FIGHTER_STATUS_KIND_ENTRY, *FIGHTER_STATUS_KIND_WIN].contains(&status_kind) {
				println!("Reset!");
				RYU_SUPER[fighter_num] = 0;
				RYU_FX_TIMER[fighter_num] = 0;
			};
			if !AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_ALL) {
				HAS_ADDED[fighter_num] = false;
			};
			if AttackModule::is_infliction(boma, *COLLISION_KIND_MASK_ALL) && status_kind != *FIGHTER_STATUS_KIND_CATCH_ATTACK && !HAS_ADDED[fighter_num] {
				RYU_SUPER[fighter_num] += 3;
				println!("attacks! {} ", RYU_SUPER[fighter_num]);
				if RYU_SUPER[fighter_num] == max_meter {
					println!("Super active {} ", RYU_SUPER[fighter_num]);
				};
				HAS_ADDED[fighter_num] = true;
			};
			if RYU_SUPER[fighter_num] >= max_meter {
				RYU_SUPER[fighter_num] = max_meter;
			};
			if RYU_SUPER[fighter_num] >= max_meter && WorkModule::is_flag(boma, *FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL) && (ControlModule::get_command_flag_cat(boma, 3) & *FIGHTER_PAD_CMD_CAT4_FLAG_SPECIAL_N_COMMAND) != 0 {
				StatusModule::change_status_request_from_script(boma, *FIGHTER_RYU_STATUS_KIND_SPECIAL_N_COMMAND, true);
			};
			if RYU_SUPER[fighter_num] == max_meter && status_kind == *FIGHTER_RYU_STATUS_KIND_SPECIAL_N_COMMAND && MotionModule::frame(boma) < 14.0  && MotionModule::frame(boma) > 12.9 {
				ArticleModule::generate_article(boma, *FIGHTER_RYU_GENERATE_ARTICLE_SHINKUHADOKEN, false, 0);
				RYU_SUPER[fighter_num] = 0;
				println!("Shinsu!!!");
				println!("The aura_size is {}", aura_size);
			} else if [hash40("special_n"), hash40("special_air_n")].contains(&MotionModule::motion_kind(boma)) && MotionModule::frame(boma) < 14.0  && MotionModule::frame(boma) > 12.9 {
				WorkModule::on_flag(boma, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_N_FLAG_SHOOT);
			};
		};
	};
}

pub fn install() {
    smashline::install_agent_frame_callbacks!(supers);
}