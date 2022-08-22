use smash::hash40;
use smash::phx::Hash40;
use smash::lib::lua_const::*;
use smash::app::*;
use smash::app::lua_bind::*;
use smash::lua2cpp::{L2CFighterCommon, L2CAgentBase};
use smashline::*;
use smash_script::*;
use smash::phx::Vector2f;
use crate::FIGHTER_MANAGER;
use std::os::raw::c_int;
use std::os::raw::c_ulong;

static mut STATUS_DURATION : [i32; 8] = [0; 8];
static mut MOTION_DURATION : [i32; 8] = [0; 8];
pub static mut SPEED_X : [f32; 8] = [0.0; 8];
pub static mut SPEED_Y : [f32; 8] = [0.0; 8];
pub static mut ACCEL_X : [f32; 8] = [0.0; 8];
pub static mut ACCEL_Y : [f32; 8] = [0.0; 8];

//Cstick
pub static mut SUB_STICK: [Vector2f;9] = [Vector2f{x:0.0, y: 0.0};9];


// Transition Hook static muts:
// 0 - Don't change 
// 1 - Force off
// 2 - Force on 
pub static mut CAN_UPB: [i32; 8] = [0; 8];
pub static mut CAN_SIDEB: [i32; 8] = [0; 8];
pub static mut CAN_DOWNB: [i32; 8] = [0; 8];
pub static mut CAN_NEUTRALB: [i32; 8] = [0; 8];
pub static mut CAN_JUMP_SQUAT: [i32; 8] = [0; 8];
pub static mut CAN_DOUBLE_JUMP: [i32; 8] = [0; 8];
pub static mut CAN_CLIFF: [i32; 8] = [0; 8];
pub static mut CAN_ATTACK_AIR: [i32; 8] = [0; 8];
pub static mut CAN_AIRDODGE: [i32; 8] = [0; 8];
pub static mut CAN_RAPID_JAB: [i32; 8] = [0; 8];
pub static mut CAN_JAB: [i32; 8] = [0; 8];
pub static mut CAN_DASH: [i32; 8] = [0; 8];
pub static mut CAN_TURNDASH: [i32; 8] = [0; 8];

//Jab Flags
pub static mut HAS_ENABLE_COMBO_ON: [bool; 8] = [false; 8];
pub static mut HAS_ENABLE_NO_HIT_COMBO_ON: [bool; 8] = [false; 8];
pub static mut HAS_ENABLE_100_ON: [bool; 8] = [false; 8];

// Use this for general per-frame fighter-level hooks
#[skyline::hook(replace = smash::app::lua_bind::WorkModule::is_enable_transition_term)]
pub unsafe fn is_enable_transition_term_hook(boma: &mut smash::app::BattleObjectModuleAccessor, flag: i32) -> bool {
		let ENTRY_ID = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
		if CAN_UPB[ENTRY_ID] != 0 && flag == *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_HI {
			if CAN_UPB[ENTRY_ID] == 1 {
				return false
			} else {
				return true 
			}
		} else if CAN_SIDEB[ENTRY_ID] != 0  && flag == *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_S {
			if CAN_SIDEB[ENTRY_ID] == 1 {
				return false
			} else {
				return true 
			}
		}else if CAN_DOWNB[ENTRY_ID] != 0  && flag == *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_LW {
			if CAN_DOWNB[ENTRY_ID] == 1 {
				return false
			} else {
				return true 
			}
		}else if CAN_CLIFF[ENTRY_ID] != 0  && flag == *FIGHTER_STATUS_TRANSITION_TERM_ID_CLIFF_CATCH{
			if CAN_CLIFF[ENTRY_ID] == 1 {
				return false
			} else {
				return true 
			}
		} else if CAN_AIRDODGE[ENTRY_ID] != 0  && flag == *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ESCAPE_AIR {
			if CAN_AIRDODGE[ENTRY_ID] == 1 {
				return false
			} else {
				return true 
			}
		}  else if CAN_NEUTRALB[ENTRY_ID] != 0  && flag == *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_N {
			if CAN_NEUTRALB[ENTRY_ID] == 1 {
				return false
			} else {
				return true 
			}
		} else if CAN_JUMP_SQUAT[ENTRY_ID] != 0  && (flag == *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_SQUAT || flag == *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_SQUAT_BUTTON) {
			if CAN_JUMP_SQUAT[ENTRY_ID] == 1 {
				return false
			} else {
				return true 
			}
		}  else if CAN_DOUBLE_JUMP[ENTRY_ID] != 0  && (flag == *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_AERIAL || flag == *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_AERIAL_BUTTON) {
			if CAN_DOUBLE_JUMP[ENTRY_ID] == 1 {
				return false
			} else {
				return true 
			}
		}  else if CAN_ATTACK_AIR[ENTRY_ID] != 0  && flag == *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_AIR  {
			if CAN_ATTACK_AIR[ENTRY_ID] == 1 {
				return false
			} else {
				return true 
			}
		}  else if CAN_DASH[ENTRY_ID] != 0  && flag == *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_DASH {
			if CAN_DASH[ENTRY_ID] == 1 {
				return false
			} else {
				return true 
			}
		}  else if CAN_TURNDASH[ENTRY_ID] != 0  && flag == *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_TURN_DASH {
			if CAN_TURNDASH[ENTRY_ID] == 1 {
				return false
			} else {
				return true 
			}
		}   else {
			original!()(boma, flag)
		}
}
#[skyline::hook(replace = smash::app::lua_bind::WorkModule::on_flag)]
pub unsafe fn on_flag_hook(boma: &mut smash::app::BattleObjectModuleAccessor, int: c_int) -> () {
	if int == *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_100 {
		HAS_ENABLE_100_ON[WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize] = true;
		let status_kind = smash::app::lua_bind::StatusModule::status_kind(boma);
		let fighter_kind = smash::app::utility::get_kind(boma);
		if ![*FIGHTER_STATUS_KIND_ATTACK, *FIGHTER_DEMON_STATUS_KIND_ATTACK_COMBO].contains(&status_kind) || [*FIGHTER_KIND_MURABITO].contains(&fighter_kind){
			original!()(boma, int)
		};
	} else if int == *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO {
		HAS_ENABLE_COMBO_ON[WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize] = true;
		let status_kind = smash::app::lua_bind::StatusModule::status_kind(boma);
		let fighter_kind = smash::app::utility::get_kind(boma);
		if status_kind != *FIGHTER_STATUS_KIND_ATTACK || [*FIGHTER_KIND_MURABITO].contains(&fighter_kind) {
			original!()(boma, int)
		};
	} else if int == *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_NO_HIT_COMBO {
		HAS_ENABLE_NO_HIT_COMBO_ON[WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize] = true;
		let status_kind = smash::app::lua_bind::StatusModule::status_kind(boma);
		let fighter_kind = smash::app::utility::get_kind(boma);
		if status_kind != *FIGHTER_STATUS_KIND_ATTACK || [*FIGHTER_KIND_MURABITO].contains(&fighter_kind) {
			original!()(boma, int)
		};
	} else {
		original!()(boma, int)
	}
}
#[skyline::hook(replace = smash::app::lua_bind::WorkModule::off_flag)]
pub unsafe fn off_flag_hook(boma: &mut smash::app::BattleObjectModuleAccessor, int: c_int) -> () {
	if int == *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_100 {
		HAS_ENABLE_100_ON[WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize] = false;
		original!()(boma, int)
	} else if int == *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO {
		HAS_ENABLE_COMBO_ON[WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize] = false;
		original!()(boma, int)
	} else {
		original!()(boma, int)
	}
}


#[fighter_frame_callback]
pub fn util_update(fighter : &mut L2CFighterCommon) {
    unsafe {
        let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);    
		let ENTRY_ID = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
		let fighter_kind = smash::app::utility::get_kind(boma);
		let status_kind = smash::app::lua_bind::StatusModule::status_kind(boma);
		let prev_status = StatusModule::prev_status_kind(boma, 0);
		//Checks Frames since entering a status
		if status_kind != prev_status || is_reset() {
			STATUS_DURATION[ENTRY_ID] = 0;
		} else {
			STATUS_DURATION[ENTRY_ID] += 1;
		};
		//Resets inability to special
		if is_reset() {
			CAN_ATTACK_AIR[ENTRY_ID] = 0;
			CAN_JUMP_SQUAT[ENTRY_ID] = 0;
			CAN_DOUBLE_JUMP[ENTRY_ID] = 0;
			CAN_DOWNB[ENTRY_ID] = 0;
			CAN_NEUTRALB[ENTRY_ID] = 0;
			CAN_SIDEB[ENTRY_ID] = 0;
			CAN_UPB[ENTRY_ID] = 0;
			CAN_CLIFF[ENTRY_ID] = 0;
			CAN_AIRDODGE[ENTRY_ID] = 0;
			CAN_RAPID_JAB[ENTRY_ID] = 0;
			CAN_JAB[ENTRY_ID] = 0;
			HAS_ENABLE_COMBO_ON[ENTRY_ID] = false;
			HAS_ENABLE_100_ON[ENTRY_ID] = false;
		};
		if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_CSTICK_ON) {
			if ControlModule::get_stick_x(boma) != 0.0 {
				SUB_STICK[ENTRY_ID].x = ControlModule::get_stick_x(boma);
			};
			if ControlModule::get_stick_y(boma) != 0.0 {
				SUB_STICK[ENTRY_ID].y = ControlModule::get_stick_y(boma);
			};
		} else {
			SUB_STICK[ENTRY_ID].x = 0.0;
			SUB_STICK[ENTRY_ID].y = 0.0;
		};
		//Checks Frames since entering a motion
		if MotionModule::frame(boma) < 2.0 || is_reset() {
			MOTION_DURATION[ENTRY_ID] = 0;
		} else {
			MOTION_DURATION[ENTRY_ID] += 1;
		};
		if motion_duration(boma) < 1 {
			HAS_ENABLE_COMBO_ON[ENTRY_ID] = false;
			HAS_ENABLE_100_ON[ENTRY_ID] = false;
			HAS_ENABLE_NO_HIT_COMBO_ON[ENTRY_ID] = false;
		};
		//Speed and acceleration checks
		if is_reset() {
			SPEED_X[ENTRY_ID] = 0.0;
			SPEED_Y[ENTRY_ID] = 0.0;
			ACCEL_X[ENTRY_ID] = 0.0;
			ACCEL_Y[ENTRY_ID] = 0.0;
		};
		ACCEL_X[ENTRY_ID] = SPEED_X[ENTRY_ID] - KineticModule::get_sum_speed_x(boma, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
		ACCEL_Y[ENTRY_ID] = SPEED_Y[ENTRY_ID] - KineticModule::get_sum_speed_y(boma, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
		SPEED_X[ENTRY_ID] = KineticModule::get_sum_speed_x(boma, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
		SPEED_Y[ENTRY_ID] = KineticModule::get_sum_speed_y(boma, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
		println!("X Accel: {}, Y Accel: {}, X Speed: {}, Y Speed: {}", ACCEL_X[ENTRY_ID], ACCEL_Y[ENTRY_ID], SPEED_X[ENTRY_ID], SPEED_Y[ENTRY_ID]);
		/*if ENTRY_ID < 2 {
			println!("MOTION_DURATION {}, STATUS_DURATION {}, SPEED_X {}, SPEED_Y {}, ACCEL_X {}, ACCEL_Y {}", motion_duration(boma), status_duration(boma), get_speed_x(boma), get_speed_y(boma), get_accel_x(boma), get_accel_y(boma));
			println!("total fighters {}, ray_check_pos {}, is_angel_plat {}, stock_count{}", total_fighters(), ray_check_pos(boma, 0.0, -10.0, false), is_angel_plat(boma), stock_count(boma));
		}*/
    };
}

//Status and motion duration
pub(crate) unsafe fn status_duration(boma: &mut smash::app::BattleObjectModuleAccessor) -> i32 {
	let ENTRY_ID = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
	return STATUS_DURATION[ENTRY_ID]
}
pub(crate) unsafe fn motion_duration(boma: &mut smash::app::BattleObjectModuleAccessor) -> i32 {
	let ENTRY_ID = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
	return MOTION_DURATION[ENTRY_ID]
}

//Position and speed
pub(crate) unsafe fn ray_check_pos(boma: &mut smash::app::BattleObjectModuleAccessor, x_distance : f32, y_distance: f32, ignore_plat: bool) -> u64 {
	GroundModule::ray_check(boma, &Vector2f{ x: PostureModule::pos_x(boma), y: PostureModule::pos_y(boma)}, &Vector2f{ x: x_distance, y: y_distance}, ignore_plat)
}
pub(crate) unsafe fn get_speed_x(boma: &mut smash::app::BattleObjectModuleAccessor) -> f32 {
	let ENTRY_ID = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
	return SPEED_X[ENTRY_ID]
}
pub(crate) unsafe fn get_speed_y(boma: &mut smash::app::BattleObjectModuleAccessor) -> f32 {
	let ENTRY_ID = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
	return SPEED_Y[ENTRY_ID]
}
pub(crate) unsafe fn get_accel_x(boma: &mut smash::app::BattleObjectModuleAccessor) -> f32 {
	let ENTRY_ID = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
	return ACCEL_X[ENTRY_ID]
}
pub(crate) unsafe fn get_accel_y(boma: &mut smash::app::BattleObjectModuleAccessor) -> f32 {
	let ENTRY_ID = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
	return ACCEL_X[ENTRY_ID]
}

//Hitlag and Hitstun
pub(crate) unsafe fn is_hitlag(boma: &mut smash::app::BattleObjectModuleAccessor) -> bool {
	if WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_HIT_STOP_ATTACK_SUSPEND_FRAME) < 1 {
		return false
	} else {
		return true
	}
}
pub(crate) unsafe fn get_hitlag(boma: &mut smash::app::BattleObjectModuleAccessor) -> i32 {
	return WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_HIT_STOP_ATTACK_SUSPEND_FRAME)
}

//Misc.
pub(crate) unsafe fn is_reset() -> bool {
	if smash::app::sv_information::is_ready_go() {
		return false
	} else {
		return true
	}
}
//Fighter Manager Stuff (PROBABLY DOESNT WORK)
pub(crate) unsafe fn get_fighter_manager() -> *mut smash::app::FighterManager {
	return  *(FIGHTER_MANAGER as *mut *mut smash::app::FighterManager)
}
pub(crate) unsafe fn get_FighterEntryID(boma: &mut smash::app::BattleObjectModuleAccessor) -> smash::app::FighterEntryID {
	let ENTRY_ID = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID);
	return  smash::app::FighterEntryID(ENTRY_ID)
}
pub(crate) unsafe fn get_fighter_info(boma: &mut smash::app::BattleObjectModuleAccessor) -> *mut smash::app::FighterInformation {
	return  smash::app::lua_bind::FighterManager::get_fighter_information(get_fighter_manager(), get_FighterEntryID(boma))
}

pub(crate) unsafe fn is_results() -> bool {
	smash::app::lua_bind::FighterManager::is_result_mode(get_fighter_manager())
}
pub(crate) unsafe fn is_angel_plat(boma: &mut smash::app::BattleObjectModuleAccessor) -> bool {
	smash::app::lua_bind::FighterManager::is_rebirth_plate_line(get_fighter_manager(), get_FighterEntryID(boma))
}
pub(crate) unsafe fn total_fighters() -> i32 {
	smash::app::lua_bind::FighterManager::total_fighter_num(get_fighter_manager())
}

pub(crate) unsafe fn stock_count(boma: &mut smash::app::BattleObjectModuleAccessor) -> u64 {
	smash::app::lua_bind::FighterInformation::stock_count(get_fighter_info(boma))
}




pub fn install() {
    smashline::install_agent_frame_callbacks!(util_update);
	skyline::install_hook!(is_enable_transition_term_hook);
	skyline::install_hook!(on_flag_hook);
	skyline::install_hook!(off_flag_hook);
}
