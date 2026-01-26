use smash::app::lua_bind::*;
use smash::lib::lua_const::*;
use smash::app::utility::get_kind;
use smash::hash40;
use smash::lua2cpp::*;
use smashline::*;
use smash_script::*;
use smash::phx::*;
use smash::lib::{L2CValue, L2CAgent};
use smash::phx::Vector2f;
use crate::util::*;

static mut PAUSE : [bool; 8] = [false; 8];
static mut HAS_WALLJUMPED : [bool; 8] = [false; 8];
static mut DRIFT_AMOUNT : [f32; 8] = [0.0; 8];

pub unsafe fn rivals_pivot(fighter : &mut L2CFighterCommon, status_kind : i32, ENTRY_ID : usize, stickx : f32) {
    unsafe {
        if status_kind == *FIGHTER_STATUS_KIND_TURN {
			JostleModule::set_status(fighter.module_accessor, false);
            return;
        }
        if !crate::is_in!(status_kind, *FIGHTER_STATUS_KIND_DASH, *FIGHTER_STATUS_KIND_TURN_DASH) {
			CAN_DASH[ENTRY_ID] = 0;
			CAN_TURNDASH[ENTRY_ID] = 0;
            return;
        }
        let frame = MotionModule::frame(fighter.module_accessor);
        if frame <= 6.0 || frame > 10.0 {
			CAN_DASH[ENTRY_ID] = 0;
			CAN_TURNDASH[ENTRY_ID] = 0;
            return;
        }



        CAN_DASH[ENTRY_ID] = 1;
        CAN_TURNDASH[ENTRY_ID] = 1;
        if stickx <= -0.5 {
			StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_TURN, true);
        }
    };
}
pub unsafe fn drift_di(fighter : &mut L2CFighterCommon, status_kind : i32, ENTRY_ID : usize, stickx : f32) {
    unsafe {
		if !is_gamemode("rivals".to_string()) {
			return;
		}

        if crate::is_in!(status_kind, *FIGHTER_STATUS_KIND_DAMAGE_FLY, *FIGHTER_STATUS_KIND_DAMAGE_FLY_ROLL) {
            let max_drift_change = 0.7;
            let drift_add = 0.005;
            let drift_mul = 0.01;
            if DRIFT_AMOUNT[ENTRY_ID].abs() < max_drift_change  && stickx.abs() > 0.2{
                let add_amount = drift_add*stickx.signum() + drift_mul*(stickx/1.0);
                let speed = smash::phx::Vector3f { x: add_amount, y: 0.0, z: 0.0 };
                KineticModule::add_speed(boma, &speed);
            }
        } else {
            DRIFT_AMOUNT[ENTRY_ID] = 0.0;
        }
    };
}
pub unsafe fn airdodge(fighter : &mut L2CFighterCommon, status_kind : i32, ENTRY_ID : usize, stickx : f32) {
    if !crate::is_in!(status_kind, *FIGHTER_STATUS_KIND_ESCAPE_AIR, *FIGHTER_STATUS_KIND_ESCAPE_AIR_SLIDE) {
        PAUSE[ENTRY_ID] = false;
        return;
    }
    let frame = MotionModule::frame(fighter.module_accessor);
    if frame > 1.0 && frame < 3.0 {
        if stickx.abs() < 0.2 && (ControlModule::get_stick_y(fighter.module_accessor)).abs() < 0.2 {
            PAUSE[ENTRY_ID] = true;
        }
        return;
    }
    PAUSE[ENTRY_ID] = match frame {
        n if n > 28.0 => false,
        n if n > 16.0 => true,
        _ => PAUSE[ENTRY_ID],
    };
	let gravity = WorkModule::get_param_float(fighter.module_accessor, hash40("air_accel_y"), 0);
    if PAUSE[ENTRY_ID] { 
        KineticModule::clear_speed_all(fighter.module_accessor);
        macros::SET_SPEED_EX(fighter, 0.0, gravity, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    }
}

pub unsafe fn expanded_walljump(fighter : &mut L2CFighterCommon, status_kind : i32, ENTRY_ID : usize) {
    let is_high_flick = ControlModule::get_flick_x(fighter.module_accessor) >= 3;
    if !is_high_flick {
        return;
    }
    let frame = MotionModule::frame(fighter.module_accessor);
    let cancel_frame = FighterMotionModuleImpl::get_cancel_frame(boma,smash::phx::Hash40::new_raw(MotionModule::motion_kind(boma)),false) as f32;
	if  !crate::is_in!(status_kind, *FIGHTER_STATUS_KIND_FALL, *FIGHTER_STATUS_KIND_JUMP, *FIGHTER_STATUS_KIND_JUMP_AERIAL) &&
        (!crate::is_in!(status_kind, *FIGHTER_STATUS_KIND_FALL_SPECIAL, *FIGHTER_STATUS_KIND_SPECIAL_HI) || HAS_WALLJUMPED[ENTRY_ID]) &&
        (status_kind != *FIGHTER_STATUS_KIND_ATTACK_AIR || frame < cancel_frame) {
            return;
    }
    
    let stickx = ControlModule::get_stick_x(fighter.module_accessor);
    let touch_flag = GroundModule::get_touch_flag(fighter.module_accessor);
    if  (touch_flag == *GROUND_TOUCH_FLAG_LEFT as u64 && stickx >= 0.7) ||
        (touch_flag == *GROUND_TOUCH_FLAG_RIGHT as u64 && stickx <= -0.7) {
                HAS_WALLJUMPED[ENTRY_ID] = true;
                StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_WALL_JUMP, true);
    }
}
pub unsafe fn expanded_walljump(fighter : &mut L2CFighterCommon, status_kind : i32, ENTRY_ID : usize) {
	let situation_kind = StatusModule::situation_kind(fighter.module_accessor);
    if situation_kind == *SITUATION_KIND_GROUND {
        HAS_WALLJUMPED[ENTRY_ID] = false;
        return;
    }
    let is_high_flick = ControlModule::get_flick_x(fighter.module_accessor) >= 3;
    if !is_high_flick {
        return;
    }
    let frame = MotionModule::frame(fighter.module_accessor);
    let cancel_frame = FighterMotionModuleImpl::get_cancel_frame(boma,smash::phx::Hash40::new_raw(MotionModule::motion_kind(boma)),false) as f32;
	if  !crate::is_in!(status_kind, *FIGHTER_STATUS_KIND_FALL, *FIGHTER_STATUS_KIND_JUMP, *FIGHTER_STATUS_KIND_JUMP_AERIAL) &&
        (!crate::is_in!(status_kind, *FIGHTER_STATUS_KIND_FALL_SPECIAL, *FIGHTER_STATUS_KIND_SPECIAL_HI) || HAS_WALLJUMPED[ENTRY_ID]) &&
        (status_kind != *FIGHTER_STATUS_KIND_ATTACK_AIR || frame < cancel_frame) {
            return;
    }
    
    let stickx = ControlModule::get_stick_x(fighter.module_accessor);
    let touch_flag = GroundModule::get_touch_flag(fighter.module_accessor);
    if  (touch_flag == *GROUND_TOUCH_FLAG_LEFT as u64 && stickx >= 0.7) ||
        (touch_flag == *GROUND_TOUCH_FLAG_RIGHT as u64 && stickx <= -0.7) {
                HAS_WALLJUMPED[ENTRY_ID] = true;
                StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_WALL_JUMP, true);
    }
}

pub unsafe fn parry_recoil(fighter : &mut L2CFighterCommon, status_kind : i32) {
    if !AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_SHIELD) {
        return;
    }
    if crate::is_in!(status_kind, *FIGHTER_STATUS_KIND_ATTACK, *FIGHTER_STATUS_KIND_ATTACK_100) {
        return;
    }
	let situation_kind = StatusModule::situation_kind(fighter.module_accessor);
    match situation_kind {
        n if n == *SITUATION_KIND_GROUND => {
            StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_SAVING_DAMAGE, true);
        },
        _ => {
            StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_FALL_SPECIAL, true);
        },
    };
}

pub unsafe fn babydash(fighter : &mut L2CFighterCommon, status_kind : i32, stickx : f32) {
    let frame = MotionModule::frame(fighter.module_accessor) as i32;
    if frame > 4 {
        return;
    }
    if !crate::is_in!(status_kind, *FIGHTER_STATUS_KIND_DASH, *FIGHTER_STATUS_KIND_TURN_DASH){
        return;
    }
    let baby_frame = if status_kind == *FIGHTER_STATUS_KIND_DASH {3} else {4};
    if frame == baby_frame && stickx.abs() < 0.2 {
        StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_WAIT, true);
    }
}

pub unsafe fn opff(fighter : &mut L2CFighterCommon, status_kind : i32, ENTRY_ID : usize) {
    if !is_gamemode("rivals".to_string()){
        return;
    }
    let lr = PostureModule::lr(fighter.module_accessor);
    let stickx = ControlModule::get_stick_x(fighter.module_accessor) * lr;		

    CAN_GRAB[ENTRY_ID] = 1;
    GroundModule::set_cliff_check(fighter.module_accessor, smash::app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE));

    rivals_pivot(fighter, status_kind, ENTRY_ID, stickx);
    drift_di(fighter, status_kind, ENTRY_ID, stickx);
    airdodge(fighter, status_kind, ENTRY_ID, stickx);
    expanded_walljump(fighter, status_kind, ENTRY_ID);
    babydash(fighter, status_kind, stickx);
}