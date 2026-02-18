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
use once_cell::sync::Lazy;
static mut PREVENT_LOOP: [bool; 8] = [false; 8];

//Landing Lag Platform Cancel
pub unsafe fn llpc(fighter : &mut L2CFighterCommon, status_kind : i32, motion_kind : u64) {
    unsafe {
		if !GroundModule::is_passable_ground(fighter.module_accessor) {
			return;
		}
		let flick_y = ControlModule::get_flick_y(fighter.module_accessor);
		if flick_y < 3 || flick_y > 20 {
			return;
		}
		let llpc_threshold = -0.6875;
		if ControlModule::get_stick_y(fighter.module_accessor) > llpc_threshold {
			return;
		}
		if StatusModule::situation_kind(fighter.module_accessor) != *SITUATION_KIND_GROUND {
			return;
		}
		let cancel_frame = FighterMotionModuleImpl::get_cancel_frame(fighter.module_accessor,smash::phx::Hash40::new_raw(motion_kind),false) as f32;
		let frame = MotionModule::frame(fighter.module_accessor);
		if (frame/cancel_frame) < (1.0/6.0) {
			return;
		}
		if crate::is_motion!(motion_kind, "landing_air_n", "landing_air_f", "landing_air_b", "landing_air_hi", "landing_air_lw") {
			let cat1 = ControlModule::get_command_flag_cat(fighter.module_accessor, 0);
			let checks = 	*FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_LW4 | 
                          	*FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_LW3 | 
                          	*FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_LW | 
                          	*FIGHTER_PAD_CMD_CAT1_FLAG_ESCAPE;
			if (cat1 & checks) == 0 && ControlModule::check_button_off(fighter.module_accessor, *CONTROL_PAD_BUTTON_JUMP) {
				StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_PASS, true);
			};
		}
	}
}	
// Shield Drop
pub unsafe fn shielddrop(fighter : &mut L2CFighterCommon, status_kind : i32) {
    unsafe {
		if !GroundModule::is_passable_ground(fighter.module_accessor) {
			return;
		}
		let flick_y = ControlModule::get_flick_y(fighter.module_accessor);
		if flick_y > 3 && flick_y < 20 {
			return;
		}
		let shielddrop_threshold = -0.45;
		if ControlModule::get_stick_y(fighter.module_accessor) > shielddrop_threshold {
			return;
		}
		if !crate::is_in!(status_kind, *FIGHTER_STATUS_KIND_GUARD_ON, *FIGHTER_STATUS_KIND_GUARD) {
			return;
		}
		if StatusModule::situation_kind(fighter.module_accessor) != *SITUATION_KIND_GROUND {
			return;
		}
		if (ControlModule::get_command_flag_cat(fighter.module_accessor, 1) & *FIGHTER_PAD_CMD_CAT2_FLAG_STICK_ESCAPE) != 0 {
			return;
		}
		StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_PASS, true);
    };
}	
// Endlag Drop
pub unsafe fn endlag_drop(fighter : &mut L2CFighterCommon, status_kind : i32) {
    unsafe {
		if !GroundModule::is_passable_ground(fighter.module_accessor) {
			return;
		}
		let shielddrop_threshold = -0.95;
		if ControlModule::get_stick_y(fighter.module_accessor) > shielddrop_threshold {
			return;
		}
		if !crate::is_in!(status_kind, *FIGHTER_STATUS_KIND_SQUAT, *FIGHTER_STATUS_KIND_SQUAT_WAIT) {
			return;
		}
		if StatusModule::situation_kind(fighter.module_accessor) != *SITUATION_KIND_GROUND {
			return;
		}
		let cat1 = ControlModule::get_command_flag_cat(fighter.module_accessor, 0);
		let checks = 	*FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_LW4 | 
                        *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_LW3 | 
                      	*FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_LW | 
                      	*FIGHTER_PAD_CMD_CAT1_FLAG_ESCAPE;
		if 	(cat1 & checks == 0) &&
			(ControlModule::check_button_off(fighter.module_accessor, *CONTROL_PAD_BUTTON_JUMP)) &&
			(ControlModule::check_button_off(fighter.module_accessor, *CONTROL_PAD_BUTTON_GUARD)) {
				StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_PASS, true);
		}
    };
}	
pub unsafe fn respawn_wakeup(fighter : &mut L2CFighterCommon, status_kind : i32, motion_kind : u64, ENTRY_ID : usize) {
    unsafe {
		if status_kind != *FIGHTER_STATUS_KIND_REBIRTH {
			PREVENT_LOOP[ENTRY_ID] = false;
			return;
		}
		if PREVENT_LOOP[ENTRY_ID] {
			return;
		}
    	let fighter_kind = smash::app::utility::get_kind(boma(fighter));
		if crate::is_in!(fighter_kind, *FIGHTER_KIND_PFUSHIGISOU, *FIGHTER_KIND_PZENIGAME, *FIGHTER_KIND_PLIZARDON) {
			return;
		}
		let frame = MotionModule::frame(fighter.module_accessor);
		let end_frame = MotionModule::end_frame(fighter.module_accessor);
		let is_end = end_frame-frame < 3.0;
		match motion_kind {
			m if m == hash40("wait") => {
					MotionModule::change_motion(fighter.module_accessor, Hash40::new("down_stand_u"), -1.0, 1.0, false, 0.0, false, false);
				},
			m if m != hash40("down_stand_u") => {},
			m if is_end => {
					PREVENT_LOOP[ENTRY_ID] = true;
					MotionModule::change_motion(fighter.module_accessor, Hash40::new("wait"), -1.0, 1.0, false, 0.0, false, false);
				},
			m if frame < 3.0 => {
					MotionModule::set_rate(fighter.module_accessor, 0.05);
				},
			_ => {
					MotionModule::set_rate(fighter.module_accessor, 1.0);
				},
		};
    };
}	

pub unsafe fn opff(fighter : &mut L2CFighterCommon, status_kind : i32, motion_kind : u64, entry_id : usize) {
		if !is_mechanics_enabled() {
			return;
		}
		endlag_drop(fighter, status_kind);
		shielddrop(fighter, status_kind);
		llpc(fighter, status_kind, motion_kind);
		respawn_wakeup(fighter, status_kind, motion_kind, entry_id);
}


struct EdgeCancelEntry {
    pub fighter_kind : i32,
    pub status_kind : i32,
    pub slot_start: i32,
    pub slot_count : i32,
}
impl EdgeCancelEntry {
    pub const fn new(kind: i32, status: i32, start: i32, slots : i32) -> Self {
        Self {
            fighter_kind : kind,
            status_kind: status,
            slot_start : start,
            slot_count : slots,
        }
    }
}
static EDGE_CANCEL_LIST: Lazy<Vec<EdgeCancelEntry>> = Lazy::new(|| {
    vec![
        EdgeCancelEntry::new(*FIGHTER_KIND_LUCARIO, *FIGHTER_STATUS_KIND_ATTACK_DASH, 0, 16),
        EdgeCancelEntry::new(*FIGHTER_KIND_LUCARIO, *FIGHTER_STATUS_KIND_SPECIAL_LW, 0, 16),
        EdgeCancelEntry::new(*FIGHTER_KIND_DONKEY, *FIGHTER_STATUS_KIND_ATTACK_DASH, 0, 16),
        EdgeCancelEntry::new(*FIGHTER_KIND_BUDDY, *FIGHTER_STATUS_KIND_ATTACK_DASH, 0, 16),
        EdgeCancelEntry::new(*FIGHTER_KIND_KAMUI, *FIGHTER_STATUS_KIND_ATTACK_DASH, 0, 16),
        EdgeCancelEntry::new(*FIGHTER_KIND_PURIN, *FIGHTER_STATUS_KIND_ATTACK_DASH, 0, 16),
        EdgeCancelEntry::new(*FIGHTER_KIND_CAPTAIN, *FIGHTER_STATUS_KIND_ATTACK_DASH, 0, 16),
        EdgeCancelEntry::new(*FIGHTER_KIND_RIDLEY, *FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL, 0, 16),
        EdgeCancelEntry::new(*FIGHTER_KIND_RICHTER, *FIGHTER_STATUS_KIND_ATTACK_LW3, 0, 16),
        EdgeCancelEntry::new(*FIGHTER_KIND_SAMUS, *FIGHTER_STATUS_KIND_ATTACK_LW3, 0, 16),
        EdgeCancelEntry::new(*FIGHTER_KIND_SONIC, *FIGHTER_STATUS_KIND_SPECIAL_S, 0, 16),
        EdgeCancelEntry::new(*FIGHTER_KIND_ROY, *FIGHTER_STATUS_KIND_ATTACK_DASH, 0, 16),
        EdgeCancelEntry::new(*FIGHTER_KIND_YOUNGLINK, *FIGHTER_STATUS_KIND_ATTACK_DASH, 0, 16),
        EdgeCancelEntry::new(*FIGHTER_KIND_CAPTAIN, *FIGHTER_STATUS_KIND_SPECIAL_LW, 0, 16),
        EdgeCancelEntry::new(*FIGHTER_KIND_EDGE, *FIGHTER_STATUS_KIND_ATTACK_LW3, 0, 16),
        EdgeCancelEntry::new(*FIGHTER_KIND_FALCO, *FIGHTER_STATUS_KIND_ATTACK_DASH, 0, 16),
        EdgeCancelEntry::new(*FIGHTER_KIND_PIKMIN, *FIGHTER_STATUS_KIND_RUN_BRAKE, 120, 8), // Rayman
        EdgeCancelEntry::new(*FIGHTER_KIND_MURABITO, *FIGHTER_STATUS_KIND_SPECIAL_S, 120, 8), // Toad
        EdgeCancelEntry::new(*FIGHTER_KIND_KIRBY, *FIGHTER_KIRBY_STATUS_KIND_PIKMIN_SPECIAL_N, 0, 16),
        EdgeCancelEntry::new(*FIGHTER_KIND_INKLING, 25, 0, 16),
        EdgeCancelEntry::new(*FIGHTER_KIND_MIIFIGHTER, *FIGHTER_MIIFIGHTER_STATUS_KIND_SPECIAL_LW2_KICK_LANDING, 0, 16),
    ]
});

//Edge Cancel List
#[inline]
pub(crate) fn is_edge_cancel(fighter_kind : i32, status_kind : i32, costume : i32) -> bool {
	let fighter = fighter_kind;
	return EDGE_CANCEL_LIST.iter().any(|i| i.fighter_kind == fighter && i.status_kind == status_kind && costume >= i.slot_start && costume < (i.slot_start+i.slot_count));
}

//Edge Cancelling Part A
#[skyline::hook(replace = smash::app::lua_bind::StatusModule::init_settings)]
unsafe fn init_settings_replace(module_accessor: &mut smash::app::BattleObjectModuleAccessor, situation_kind: i32, arg3: i32, arg4: u64, ground_cliff_check_kind: u64, arg6: bool, arg7: i32, arg8: i32, arg9: i32, arg10: i32) -> u64 {
    if !is_mechanics_enabled() && !is_gamemode("rivals".to_string()) {
        return original!()(module_accessor, situation_kind, arg3, arg4, ground_cliff_check_kind, arg6, arg7, arg8, arg9, arg10);
	}
    if smash::app::utility::get_category(module_accessor) != *BATTLE_OBJECT_CATEGORY_FIGHTER {
        return original!()(module_accessor, situation_kind, arg3, arg4, ground_cliff_check_kind, arg6, arg7, arg8, arg9, arg10);
    }
	let status_kind = StatusModule::status_kind(module_accessor);
    let fighter_kind = smash::app::utility::get_kind(module_accessor);
	let costume = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);
    if is_edge_cancel(fighter_kind, status_kind, costume) && situation_kind == SITUATION_KIND_GROUND {
        original!()(module_accessor, situation_kind, arg3, 1 as u64, ground_cliff_check_kind, arg6, arg7, arg8, arg9, arg10)
    } 
    else if [*FIGHTER_STATUS_KIND_LANDING_ATTACK_AIR, *FIGHTER_STATUS_KIND_LANDING, *FIGHTER_STATUS_KIND_DASH, *FIGHTER_STATUS_KIND_TURN_DASH].contains(&status_kind) {
        original!()(module_accessor, situation_kind, arg3, 1 as u64, ground_cliff_check_kind, arg6, arg7, arg8, arg9, arg10)
    }
    else {
        original!()(module_accessor, situation_kind, arg3, arg4, ground_cliff_check_kind, arg6, arg7, arg8, arg9, arg10)
    }
}

//Edge Cancelling Part B
#[skyline::hook(replace = smash::app::lua_bind::GroundModule::correct)]
unsafe fn correct_replace(module_accessor: &mut smash::app::BattleObjectModuleAccessor, ground_correct_kind: u32) -> u64 {
	if !is_mechanics_enabled() {
        return original!()(module_accessor, ground_correct_kind);
	}
	let status_kind = StatusModule::status_kind(module_accessor);
    let fighter_kind = smash::app::utility::get_kind(module_accessor);
	let costume = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);
    if smash::app::utility::get_category(module_accessor) != *BATTLE_OBJECT_CATEGORY_FIGHTER {
        original!()(module_accessor, ground_correct_kind);
    }
    if [*FIGHTER_STATUS_KIND_LANDING_ATTACK_AIR, *FIGHTER_STATUS_KIND_LANDING, *FIGHTER_STATUS_KIND_DASH, *FIGHTER_STATUS_KIND_TURN_DASH].contains(&status_kind) {
        original!()(module_accessor, 1 as u32)
    }
    else if is_edge_cancel(fighter_kind, status_kind, costume) {
        original!()(module_accessor, *GROUND_CORRECT_KIND_GROUND as u32)
    }
    else {
        original!()(module_accessor, ground_correct_kind)
    }
}
pub fn install() {
	skyline::install_hooks!(
        init_settings_replace,
        correct_replace
    );
}

pub unsafe fn lazy_warm() {
	Lazy::force(&EDGE_CANCEL_LIST);
}