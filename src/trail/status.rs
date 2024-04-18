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
    Agent::new("trail")
    .status(Init, *FIGHTER_TRAIL_STATUS_KIND_ATTACK_AIR_F, fair_init)
    .status(Init, *FIGHTER_STATUS_KIND_ATTACK_AIR, init_attack_air)
    .status(Pre, *FIGHTER_STATUS_KIND_ATTACK_AIR, pre_attack_air)
    .status(Exec, *FIGHTER_STATUS_KIND_ATTACK_AIR, exec_attack_air)
    //.status(Main, *FIGHTER_STATUS_KIND_ATTACK_AIR, main_attack_air)
    .install();
}

unsafe extern "C" fn fair_init(fighter: &mut L2CFighterCommon) -> L2CValue {
    let motion_kind = MotionModule::motion_kind(fighter.module_accessor);
    let frame = MotionModule::frame(fighter.module_accessor);

    fighter.sub_attack_air_kind();
    if motion_kind == smash::hash40("jump_aerial_f") || motion_kind == smash::hash40("jump_aerial_b") {
        if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_JUMP) {
            MotionModule::add_motion_2nd(fighter.module_accessor, Hash40::new_raw(motion_kind), frame, 1.0, false, 1.0);
            MotionModule::set_weight(fighter.module_accessor, 1.0, true);
        }
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_IGNORE_2ND_MOTION){
        }
        if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_JUMP) {
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_JUMP_AERIAL_MOTION_2ND);
        } else {
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION_FALL);
        }
    } else {
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION_FALL);
    }
    let _ = fighter.sub_attack_air_uniq_process_init();
	let ENTRY_ID = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
	TO_FAIR[ENTRY_ID] = true;
    StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_ATTACK_AIR, false);
    0.into()
}
//fighter.status_AttackAir()
unsafe extern "C" fn init_attack_air(fighter: &mut L2CFighterCommon) -> L2CValue {
    let motion_kind = MotionModule::motion_kind(fighter.module_accessor);
    let frame = MotionModule::frame(fighter.module_accessor);
	let ENTRY_ID = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    fighter.sub_attack_air_kind();
    if motion_kind != hash40("jump_aerial_f") {
        if motion_kind == hash40("jump_aerial_b"){
                if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_IGNORE_2ND_MOTION) {
                    if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_JUMP) {
                        MotionModule::add_motion_2nd(fighter.module_accessor, Hash40::new_raw(motion_kind), frame, 1.0, false, 1.0);
                        MotionModule::set_weight(fighter.module_accessor, 1.0, true);
                    }
                }
                if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_JUMP) {
                    KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_JUMP_AERIAL_MOTION_2ND);
                }
                else {
                    KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION_FALL);
					if SPEED_Y[ENTRY_ID] > 2.5 {
						let new_speed = SPEED_X[ENTRY_ID]*PostureModule::lr(fighter.module_accessor);
						macros::SET_SPEED_EX(fighter, new_speed, 3.0, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
					};
                }
                fighter.sub_attack_air_uniq_process_init();
                return L2CValue::I32(0);
        }
    }
    else {
            if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_IGNORE_2ND_MOTION) {
                if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_JUMP) {
                    MotionModule::add_motion_2nd(fighter.module_accessor, Hash40::new_raw(motion_kind), frame, 1.0, false, 1.0);
                    MotionModule::set_weight(fighter.module_accessor, 1.0, true);
                }
            }
            if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_JUMP) {
                KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_JUMP_AERIAL_MOTION_2ND);
            }
            else {
                KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION_FALL);
				if SPEED_Y[ENTRY_ID] > 3.0 {
					let new_speed = SPEED_X[ENTRY_ID]*PostureModule::lr(fighter.module_accessor);
					macros::SET_SPEED_EX(fighter, new_speed, 3.0, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
				};
            }
            fighter.sub_attack_air_uniq_process_init();
            return L2CValue::I32(0);
    }
    KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION_FALL);
    fighter.sub_attack_air_uniq_process_init();
    0.into()
}
unsafe extern "C" fn pre_attack_air(fighter: &mut L2CFighterCommon) -> L2CValue {
    let motion_kind = MotionModule::motion_kind(fighter.module_accessor);
    let frame = MotionModule::frame(fighter.module_accessor);
	let stick_x = ControlModule::get_stick_x(fighter.module_accessor);
	let stick_y = ControlModule::get_stick_y(fighter.module_accessor);
	if true{ //(stick_x <= -0.4 || stick_x >= 0.4) || (stick_y <= -0.4 || stick_y >= 0.4) {//motion_kind != hash40("attack_air_n") {
    	fighter.status_pre_AttackAir();
		0.into()
	} else {
		fighter.change_status(
			L2CValue::I32(*FIGHTER_TRAIL_STATUS_KIND_ATTACK_AIR_N),
			L2CValue::Bool(false)
		);
		0.into()
		//original!(fighter)
	}
}
unsafe extern "C" fn main_attack_air(fighter: &mut L2CFighterCommon) -> L2CValue {
    let motion_kind = MotionModule::motion_kind(fighter.module_accessor);
    let frame = MotionModule::frame(fighter.module_accessor);
    fighter.status_AttackAir_Main();
	0.into()
}
unsafe extern "C" fn exec_attack_air(fighter: &mut L2CFighterCommon) -> L2CValue {
    let motion_kind = MotionModule::motion_kind(fighter.module_accessor);
    let frame = MotionModule::frame(fighter.module_accessor);
	let ENTRY_ID = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
	if [hash40("attack_air_n"), 0x0d7484f6cfu64, 0x0d0383c659u64].contains(&motion_kind) {
    	if AttackModule::is_infliction(fighter.module_accessor, *COLLISION_KIND_MASK_ALL) && !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_DIVE) && frame > 4.0{
			let new_speed = SPEED_X[ENTRY_ID]*PostureModule::lr(fighter.module_accessor);
			macros::SET_SPEED_EX(fighter, new_speed, 1.0, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
		};
		if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_TRAIL_STATUS_ATTACK_AIR_N_FLAG_CHECK_COMBO_BUTTON_ON) && WorkModule::is_flag(fighter.module_accessor, *FIGHTER_TRAIL_STATUS_ATTACK_AIR_N_FLAG_ENABLE_COMBO) && ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_ATTACK) {
			WorkModule::off_flag(fighter.module_accessor, *FIGHTER_TRAIL_STATUS_ATTACK_AIR_N_FLAG_CHECK_COMBO_BUTTON_ON);
			if motion_kind == 0x0d7484f6cfu64 {
				MotionModule::enable_remove_2nd_change_motion(fighter.module_accessor, false);
				MotionModule::change_motion(fighter.module_accessor, Hash40::new_raw(0x0d0383c659), -1.0, 1.0, false, 0.0, false, false);
				MotionModule::enable_remove_2nd_change_motion(fighter.module_accessor, true);
			};
			if motion_kind == hash40("attack_air_n") {
				MotionModule::enable_remove_2nd_change_motion(fighter.module_accessor, false);
				MotionModule::change_motion(fighter.module_accessor, Hash40::new_raw(0x0d7484f6cf), -1.0, 1.0, false, 0.0, false, false);
				MotionModule::enable_remove_2nd_change_motion(fighter.module_accessor, true);
			};
		};
	};
	return smashline::original_status(Exec, fighter, *FIGHTER_STATUS_KIND_ATTACK_AIR)(fighter);
}