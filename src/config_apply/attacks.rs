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

#[derive(Default, Clone, Copy)]
pub struct SpecialCancelState {
	pub can_cancel_timer : i32,
	pub can_cancel : bool,
}
const SPECIAL_CANCEL_WINDOW: i32 = 20;
pub unsafe fn grab(_fighter : &mut L2CFighterCommon, config : &config::Config, entry_id : usize) {
    if config.attacks.grab == 0 {
        return;
    }
	crate::transition_set!(entry_id, can_grab);
}

pub unsafe fn ac(fighter : &mut L2CFighterCommon, config : &config::Config, status_kind : i32) {
    if config.attacks.ac == 0 {
        return;
    }
    if !crate::is_in!(status_kind, *FIGHTER_STATUS_KIND_ATTACK, *FIGHTER_STATUS_KIND_ATTACK_DASH, *FIGHTER_STATUS_KIND_ATTACK_S3, *FIGHTER_STATUS_KIND_ATTACK_HI3, *FIGHTER_STATUS_KIND_ATTACK_LW3,  *FIGHTER_STATUS_KIND_ATTACK_S4_START, *FIGHTER_STATUS_KIND_ATTACK_HI4_START, *FIGHTER_STATUS_KIND_ATTACK_LW4_START){
        return;
    }
    WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_ATTACK_DISABLE_MINI_JUMP_ATTACK);
}

pub unsafe fn lcancel(fighter : &mut L2CFighterCommon, config : &config::Config, status_kind : i32, motion_kind : u64) {
    if config.attacks.lcancel == 0 {
        return;
    }
    if config.attacks.lcancel == 1 {
        if (2..8).contains(&ControlModule::get_trigger_count(fighter.module_accessor, *CONTROL_PAD_BUTTON_GUARD as u8)) &&
        status_kind == *FIGHTER_STATUS_KIND_LANDING_ATTACK_AIR && MotionModule::frame(fighter.module_accessor) < 4.0 {
            let cancel_frame = FighterMotionModuleImpl::get_cancel_frame(fighter.module_accessor,smash::phx::Hash40::new_raw(motion_kind), false) as f32;
            if motion_kind == hash40("landing_air_n") {
                let landing = 1.0/(((WorkModule::get_param_float(fighter.module_accessor, hash40("landing_attack_air_frame_n"), 0)*0.5))/ cancel_frame);
                MotionModule::set_rate(fighter.module_accessor, landing);
            } else if motion_kind == hash40("landing_air_f") {
                let landing = 1.0/(((WorkModule::get_param_float(fighter.module_accessor, hash40("landing_attack_air_frame_f"), 0)*0.5))/ cancel_frame);
                MotionModule::set_rate(fighter.module_accessor, landing);
            } else if motion_kind == hash40("landing_air_b") {
                let landing = 1.0/(((WorkModule::get_param_float(fighter.module_accessor, hash40("landing_attack_air_frame_b"), 0)*0.5))/ cancel_frame);
                MotionModule::set_rate(fighter.module_accessor, landing);
            } else if motion_kind == hash40("landing_air_hi") {
                let landing = 1.0/(((WorkModule::get_param_float(fighter.module_accessor, hash40("landing_attack_air_frame_hi"), 0)*0.5))/ cancel_frame);
                MotionModule::set_rate(fighter.module_accessor, landing);
            } else {
                let landing = 1.0/(((WorkModule::get_param_float(fighter.module_accessor, hash40("landing_attack_air_frame_lw"), 0)*0.5))/ cancel_frame);
                MotionModule::set_rate(fighter.module_accessor, landing);
            };
		}
    } else {
        if (2..8).contains(&ControlModule::get_trigger_count(fighter.module_accessor, *CONTROL_PAD_BUTTON_GUARD as u8)) &&
        status_kind == *FIGHTER_STATUS_KIND_LANDING_ATTACK_AIR && MotionModule::frame(fighter.module_accessor) < 4.0 {
            StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_LANDING, false);
        }
    }
}

pub unsafe fn special_cancels(fighter: &mut L2CFighterCommon, config : &config::Config, status_kind : i32, entry_id : usize) {
	if config.attacks.special_cancel == 0 {
        return;
    }
    // A hit can only be inflicted (and a special cancelled into) while the fighter is in
    // one of these attack statuses - hitboxes don't exist otherwise. Checking this first,
    // before the gamemode check and the AttackModule reads below, skips a String allocation
    // and two moderately expensive engine calls on every non-attacking frame, which is most
    // of them. can_cancel_timer simply doesn't tick while outside these statuses; that's
    // fine because it's only ever consulted again while back inside one of them, at which
    // point it's freshly reset if no hit landed.
    if !crate::is_in!(status_kind,
        *FIGHTER_STATUS_KIND_ATTACK_S4, *FIGHTER_STATUS_KIND_ATTACK_HI4,
        *FIGHTER_STATUS_KIND_ATTACK_LW4, *FIGHTER_STATUS_KIND_ATTACK,
        *FIGHTER_STATUS_KIND_ATTACK_S3, *FIGHTER_STATUS_KIND_ATTACK_HI3,
        *FIGHTER_STATUS_KIND_ATTACK_LW3, *FIGHTER_STATUS_KIND_ATTACK_AIR) {
        return;
    }
    if is_gamemode("fgmode".to_string()) {
		return;
	}
	let cat1 = ControlModule::get_command_flag_cat(fighter.module_accessor, 0);
	let is_infliction_status = AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_ALL);
	let is_infliction_hit = AttackModule::is_infliction(fighter.module_accessor, *COLLISION_KIND_MASK_HIT);
	let hit_stop_clear = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_HIT_STOP_ATTACK_SUSPEND_FRAME) < 1;

	// Single lock/read/write pass instead of the up-to-5 separate state accesses this used to do.
	let can_cancel = crate::with_state!(entry_id, SpecialCancelState, state, {
		if !is_infliction_status {
			state.can_cancel = false;
		}
		if state.can_cancel_timer > 0 {
			if hit_stop_clear {
				state.can_cancel_timer -= 1;
			}
		} else {
			state.can_cancel = false;
		}
		if is_infliction_hit {
			state.can_cancel = true;
			state.can_cancel_timer = SPECIAL_CANCEL_WINDOW;
		}
		state.can_cancel
	});

	// status_kind is already known to be one of the cancel-eligible attack
	// statuses - the early return above guarantees it.
	if can_cancel && !StopModule::is_stop(fighter.module_accessor) && hit_stop_clear {
		// Special Cancels
		match cat1 {
			n if (n & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_N) != 0 => {
					StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_SPECIAL_N, true)
				},
			n if (n & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_HI) != 0 => {
					StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_SPECIAL_HI, true)
				},
			n if (n & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_LW) != 0 => {
					StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_SPECIAL_LW, true)
				},
			n if (n & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_S) != 0 => {
					StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_SPECIAL_S, true)
				},
			_ => 0,
		};
	}
}

pub unsafe fn opff(fighter: &mut L2CFighterCommon, config : &config::Config, status_kind : i32, entry_id : usize) {
    grab(fighter, config, entry_id);
    special_cancels(fighter, config, status_kind, entry_id);
    let motion_kind = if config.attacks.lcancel != 0 {
        MotionModule::motion_kind(fighter.module_accessor)
    } else {
        0
    };
    lcancel(fighter, config, status_kind, motion_kind);
    ac(fighter, config, status_kind);
}