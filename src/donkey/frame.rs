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

use crate::donkey::*;

pub fn install() {
	smashline::install_agent_frames!(dk_frame);
}

#[fighter_frame( agent = FIGHTER_KIND_DONKEY )]
fn dk_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
		let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);   
		if is_default(boma) {
            let status_kind = smash::app::lua_bind::StatusModule::status_kind(boma);
            let ENTRY_ID = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize; 
            let frame = MotionModule::frame(boma);
            let end_frame = MotionModule::end_frame(boma);
            let motion_kind = MotionModule::motion_kind(boma);
            let rate = MotionModule::rate(boma);
			let stick_y = ControlModule::get_stick_y(boma);
			let fallspeed = WorkModule::get_param_float(fighter.module_accessor, hash40("air_speed_y_stable"), 0);
            UPB_30_X = 30.0_f32.sin() * UPB_SPEED;
            UPB_30_Y = 0.89 * UPB_SPEED;
            if IS_DK_START_ITEM_CHUCK[ENTRY_ID] == true {
                if ![*FIGHTER_STATUS_KIND_SPECIAL_S].contains(&status_kind) {
                    ItemModule::throw_item(fighter.module_accessor, 0.0, 0.0, 1.0, 0, true, 0.0);
                    IS_DK_START_ITEM_CHUCK[ENTRY_ID] = false;
                };
            };
            if [hash40("appeal_s_r"), hash40("appeal_s_l")].contains(&motion_kind) {
                if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_APPEAL_S_L) || ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_APPEAL_S_R) {
                    if frame >= 134.0 {
                        MotionModule::change_motion(fighter.module_accessor, Hash40::new_raw(motion_kind), 33.0, 1.0, false, 0.0, false, false);
                    }
                }
                if frame > 33.0 {
                    MotionModule::set_rate(boma, 1.5);
                }
            }
            if [hash40("special_air_hi"), hash40("special_hi")].contains(&motion_kind) {
                StatusModule::set_situation_kind(boma, smash::app::SituationKind(*SITUATION_KIND_AIR), true);
                StatusModule::set_keep_situation_air(boma, true);
                macros::SET_SPEED_EX(fighter, 0.0, 0.0, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
                KineticModule::suspend_energy(boma, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
                IS_DK_UPB_BARREL[ENTRY_ID] = true;
                UPB_TIMER[ENTRY_ID] += 1;
                //This sucks! Don't do it :)
                MotionModule::set_rate(boma, 1.3);
                if frame < 2.0 {
                    UPB_ANGLE_X[ENTRY_ID] = 0.0;
                    UPB_ANGLE_Y[ENTRY_ID] = UPB_SPEED;
                }
                if frame < 30.0 {
                    UPB_ANGLE_Y[ENTRY_ID] += ((UPB_30_Y - UPB_SPEED)/30.0)*rate;
                    UPB_ANGLE_X[ENTRY_ID] += (1.0/30.0 * (UPB_30_X))*rate;
                } else if frame < 90.0 {
                    if frame < 60.0 {
                        UPB_ANGLE_Y[ENTRY_ID] += ((UPB_SPEED - UPB_30_Y)/30.0)*rate;
                    } else {
                        UPB_ANGLE_Y[ENTRY_ID] += ((UPB_30_Y - UPB_SPEED)/30.0)*rate;
                    }
                    UPB_ANGLE_X[ENTRY_ID] -= (1.0/30.0 * (UPB_30_X))*rate;
                } else {
                    UPB_ANGLE_Y[ENTRY_ID] += ((UPB_SPEED - UPB_30_Y)/30.0)*rate;
                    UPB_ANGLE_X[ENTRY_ID] += (1.0/30.0 * (UPB_30_X))*rate;
                }
                println!("X:{}, Y:{}", UPB_ANGLE_X[ENTRY_ID], UPB_ANGLE_Y[ENTRY_ID]);
                if UPB_TIMER[ENTRY_ID] > 240 || (UPB_TIMER[ENTRY_ID] > 5 && ControlModule::check_button_on_trriger(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL)){
                    MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_hi_shoot"), 0.0, 1.0, false, 0.0, false, false);
                }
            } else {
                UPB_TIMER[ENTRY_ID] = 0;
                if IS_DK_UPB_BARREL[ENTRY_ID] {
                    IS_DK_UPB_BARREL[ENTRY_ID] = false;
                    ItemModule::remove_item(boma, 0);
                }
            }
            if status_kind == *FIGHTER_STATUS_KIND_FALL_SPECIAL {
                KineticModule::resume_energy(boma, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
                let fighter_kinetic_energy_control = mem::transmute::<u64, &mut smash::app::FighterKineticEnergyController>(KineticModule::get_energy(boma, *FIGHTER_KINETIC_ENERGY_ID_CONTROL));  
                smash::app::lua_bind::FighterKineticEnergyController::mul_x_accel_mul(fighter_kinetic_energy_control, 0.95);
                let cat2 = ControlModule::get_command_flag_cat(boma, 1);
                if (cat2 & *FIGHTER_PAD_CMD_CAT2_FLAG_FALL_JUMP) != 0 && stick_y < -0.66 && SPEED_Y[ENTRY_ID] <= 0.0 {
                    WorkModule::set_flag(boma, true, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_DIVE);
                }
            }
            if [hash40("special_hi_shoot")].contains(&motion_kind) {
                let cat2 = ControlModule::get_command_flag_cat(boma, 1);
                if (cat2 & *FIGHTER_PAD_CMD_CAT2_FLAG_FALL_JUMP) != 0 && stick_y < -0.66 && SPEED_Y[ENTRY_ID] < 0.0 {
                    WorkModule::set_flag(boma, true, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_DIVE);
                }
                MotionModule::set_rate(boma, 2.0);
                KineticModule::suspend_energy(boma, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
                macros::SET_SPEED_EX(fighter, UPB_ANGLE_X[ENTRY_ID]*-0.7, UPB_ANGLE_Y[ENTRY_ID], *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
            };
        }
	}
}	 