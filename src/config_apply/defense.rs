use smash::hash40;
use smash::lib::lua_const::*;
use smash::app::*;
use smash::app::lua_bind::*;
use smash::lua2cpp::{L2CFighterCommon, L2CAgentBase};
use smashline::*;
use smash_script::*;
use std::os::raw::c_int;
use std::os::raw::c_ulong;
use crate::common::*;
use crate::util::*;
use crate::config;
static mut PARRY_DURATION : [i32; 8] = [0; 8];

static NONE :  smash::phx::Vector3f =  smash::phx::Vector3f { x: 0.0, y: 0.0, z: 0.0 };

pub unsafe fn airdodge(fighter : &mut L2CFighterCommon, config : &config::Config) {
    if crate::is_in!(config.defense.airdodge, 0, 1, 4) {
        return;
    }
    if config.defense.airdodge == 2 {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_DISABLE_ESCAPE_AIR);
    }else if config.defense.airdodge == 3 {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_DISABLE_ESCAPE_AIR);    
    }
}

pub unsafe fn di(fighter : &mut L2CFighterCommon,config : &config::Config) {
    if config.defense.di == 0 {
        return;
    }
    WorkModule::set_float(fighter.module_accessor, 0.0, *FIGHTER_STATUS_DAMAGE_WORK_FLOAT_VECOR_CORRECT_STICK_X);
    WorkModule::set_float(fighter.module_accessor, 0.0, *FIGHTER_STATUS_DAMAGE_WORK_FLOAT_VECOR_CORRECT_STICK_Y);	
}

pub unsafe fn hitstun_cancel(fighter : &mut L2CFighterCommon, config : &config::Config, status_kind : i32, entry_id : usize) {
    if config.defense.hitstun_cancel == 0 {
        return;
    }
    let remaining_hitstun = WorkModule::get_float(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLOAT_DAMAGE_REACTION_FRAME);
    let total_hitstun = WorkModule::get_float(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLOAT_DAMAGE_REACTION_FRAME_LAST);
	if remaining_hitstun > 0.0 && crate::is_in!(status_kind, *FIGHTER_STATUS_KIND_DAMAGE_AIR, *FIGHTER_STATUS_KIND_DAMAGE_FALL, *FIGHTER_STATUS_KIND_DAMAGE_FLY, *FIGHTER_STATUS_KIND_DAMAGE_FLY_ROLL, *FIGHTER_STATUS_KIND_DAMAGE_FLY_METEOR) {
        if config.defense.hitstun_cancel == 1 {
            if total_hitstun - remaining_hitstun > 15.0 {
                if ControlModule::check_button_on_trriger(fighter.module_accessor, *CONTROL_PAD_BUTTON_GUARD) {
                    StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_ESCAPE_AIR, false);
                };
                if ControlModule::check_button_on_trriger(fighter.module_accessor, *CONTROL_PAD_BUTTON_ATTACK) {
                    StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_ATTACK_AIR, false);
                };
            }
        } else {
            crate::transition_set!(entry_id, can_attack_air);
            crate::transition_set!(entry_id, can_airdodge);
        }
    } else {
        crate::transition_reset!(entry_id, can_attack_air);
        crate::transition_reset!(entry_id, can_airdodge);
    }
}
pub unsafe fn shield_health(fighter : &mut L2CFighterCommon, config : &config::Config) {
    if config.defense.shield_health == 0 {
        return;
    }
    let shield_health = match config.defense.shield_health {
        0 => 50.0,
        1 => 65.0,
        _ => 35.0,
    };
    WorkModule::set_float(fighter.module_accessor, shield_health, *FIGHTER_INSTANCE_WORK_ID_FLOAT_GUARD_SHIELD_MAX);
}

pub unsafe fn shieldstun(fighter : &mut L2CFighterCommon, config : &config::Config, status_kind : i32) {
    if config.defense.shieldstun == 0 {
        return;
    }
    let stun_mul = match config.defense.shieldstun {
        0 => 0.8,
        1 => 1.75,
        _ => 0.4
    };

    let shieldstun_mul = (stun_mul/0.8) / match status_kind {
        n if n == *FIGHTER_STATUS_KIND_ATTACK_AIR => 0.33,
        n if crate::is_in!(n, *FIGHTER_STATUS_KIND_ATTACK_S4, *FIGHTER_STATUS_KIND_ATTACK_LW4, *FIGHTER_STATUS_KIND_ATTACK_HI4) => 0.725,
        _ => 1.0,
    };

    for i in 0..6 {
        if AttackModule::is_attack(fighter.module_accessor, i, false) {
            macros::ATK_SET_SHIELD_SETOFF_MUL(fighter, /*ID*/ i as u64, /*ShieldstunMul*/ shieldstun_mul);
        }
    }
}

pub unsafe fn parry_only(fighter : &mut L2CFighterCommon, status_kind : i32, motion_kind : u64, situation_kind : i32, ENTRY_ID : usize) {
    unsafe {
        if PARRY_DURATION[ENTRY_ID] > 0 {
            PARRY_DURATION[ENTRY_ID] -= 1;
        }
        if crate::is_in!(status_kind, *FIGHTER_STATUS_KIND_GUARD, *FIGHTER_STATUS_KIND_GUARD_ON, *FIGHTER_STATUS_KIND_GUARD_DAMAGE) {
			StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_GUARD_OFF, false);
            return;
        }
        if crate::is_motion!(motion_kind, "just_shield", "just_shield_off") {
            PARRY_DURATION[ENTRY_ID] = 10;
            return;
        }
        if situation_kind == *SITUATION_KIND_GROUND && PARRY_DURATION[ENTRY_ID] == 1 && !(*FIGHTER_STATUS_KIND_DAMAGE..*FIGHTER_STATUS_KIND_DAMAGE_FALL).contains(&status_kind) {
			StopModule::end_stop(fighter.module_accessor);
            //println!("End Stun Early");
			StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_WAIT, false);
            return;
        }
        let frame = MotionModule::frame(fighter.module_accessor);
        if status_kind == *FIGHTER_STATUS_KIND_REBOUND && (frame as i32) == 1 {
            if is_gamemode("rivals".to_string()) && MotionModule::rate(fighter.module_accessor) == 0.5 {
                macros::FLASH(fighter, 0.25, 0.25, 0.25, 0.5);
            }
        }
        if status_kind == *FIGHTER_STATUS_KIND_GUARD_OFF {
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_DISABLE_GUARD);
            WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ESCAPE);
            WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ESCAPE_F);
            WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ESCAPE_B);
            WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_SQUAT_BUTTON);
            WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_SQUAT);
            WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_HI4_START);
            WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_HI);
            WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_HI_COMMAND);

            match (frame as i32) {
                1 => {
                    macros::PLAY_SE(fighter, Hash40::new("se_common_step_snow"));
                    macros::PLAY_SE(fighter, Hash40::new("se_common_step_sand"));

                    macros::FLASH(fighter, 0.92, 0.99, 1, 0.5);
                    EffectModule::req_follow(fighter.module_accessor, smash::phx::Hash40::new("sys_just_shield_hit"), smash::phx::Hash40::new("hip"), &NONE, &NONE, 0.7, true, 0, 0, 0, 0, 0, true, true) as u32;
                },
                n if n > 6 => {
                    macros::COL_NORMAL(fighter);
                    let rate = if is_gamemode("rivals".to_string()) { 0.5 } else { 1.0 };
                    WorkModule::set_float(fighter.module_accessor, rate, *FIGHTER_STATUS_WORK_ID_FLOAT_REBOUND_MOTION_RATE);
                    StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_REBOUND, true);
                },
                n if n > 4 => {
                    WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_CATCH);
                },
                _ => {},
            };
        } else {
            WorkModule::off_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_DISABLE_GUARD);
        }
    };
}
pub unsafe fn parry_recoil(fighter : &mut L2CFighterCommon, status_kind : i32, situation_kind : i32) {
    if !AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_SHIELD) {
        return;
    }
    if crate::is_in!(status_kind, *FIGHTER_STATUS_KIND_ATTACK, *FIGHTER_STATUS_KIND_ATTACK_100) {
        return;
    }
    match situation_kind {
        n if n == *SITUATION_KIND_GROUND => {
            StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_SAVING_DAMAGE, true);
        },
        _ => {
            StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_FALL_SPECIAL, true);
        },
    };
}
pub unsafe fn shield(fighter : &mut L2CFighterCommon, config : &config::Config, status_kind : i32, entry_id : usize) {
    match config.defense.shield {
        0 => {},
        1 => {
            let motion_kind = MotionModule::motion_kind(fighter.module_accessor);
            let situation_kind = StatusModule::situation_kind(fighter.module_accessor);
	        parry_only(fighter, status_kind, motion_kind, situation_kind, entry_id);
            parry_recoil(fighter, status_kind, situation_kind);
        },
        _ => {
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_DISABLE_GUARD);
        },
    };
}

pub unsafe fn opff(fighter : &mut L2CFighterCommon, config : &config::Config, status_kind : i32, entry_id : usize) {
    shield(fighter, config, status_kind, entry_id);
    shieldstun(fighter, config, status_kind);
    shield_health(fighter, config);
    airdodge(fighter, config);
    di(fighter, config);
    hitstun_cancel(fighter, config, status_kind, entry_id);
}