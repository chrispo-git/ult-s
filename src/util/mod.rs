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
use crate::controls::ext::*;

static mut STATUS_DURATION : [i32; 8] = [0; 8];
static mut MOTION_DURATION : [i32; 8] = [0; 8];
pub static mut POS_X : [f32; 8] = [0.0; 8];
pub static mut POS_Y : [f32; 8] = [0.0; 8];
pub static mut SPEED_X : [f32; 8] = [0.0; 8];
pub static mut SPEED_Y : [f32; 8] = [0.0; 8];
pub static mut ACCEL_X : [f32; 8] = [0.0; 8];
pub static mut ACCEL_Y : [f32; 8] = [0.0; 8];
static mut FULL_HOP_ENABLE_DELAY : [i32; 8] = [0; 8];
pub static mut PREV_SCALE : [f32; 8] = [0.0; 8];
pub static mut IS_AB : [bool; 8] = [false; 8];
pub static mut IS_KD_THROW : [bool; 8] = [false; 8];


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
pub static mut CAN_GRAB: [i32; 8] = [0; 8];
pub static mut CAN_TURNDASH: [i32; 8] = [0; 8];

//Jab Flags
pub static mut HAS_ENABLE_COMBO_ON: [bool; 8] = [false; 8];
pub static mut HAS_ENABLE_NO_HIT_COMBO_ON: [bool; 8] = [false; 8];
pub static mut HAS_ENABLE_100_ON: [bool; 8] = [false; 8];

// Use this for general per-frame fighter-level hooks
#[skyline::hook(replace = smash::app::lua_bind::WorkModule::is_enable_transition_term)]
pub unsafe fn is_enable_transition_term_hook(boma: &mut smash::app::BattleObjectModuleAccessor, flag: i32) -> bool {
		let ENTRY_ID = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
		if smash::app::utility::get_category(boma) != *BATTLE_OBJECT_CATEGORY_FIGHTER {
			original!()(boma, flag);
		}
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
		}  else if CAN_GRAB[ENTRY_ID] != 0  && flag == *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_CATCH {
			if CAN_GRAB[ENTRY_ID] == 1 {
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
	if smash::app::utility::get_category(boma) == *BATTLE_OBJECT_CATEGORY_FIGHTER { 
		if int == *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_100 {
			HAS_ENABLE_100_ON[WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize] = true;
			let status_kind = smash::app::lua_bind::StatusModule::status_kind(boma);
			let fighter_kind = smash::app::utility::get_kind(boma);
			if ![*FIGHTER_STATUS_KIND_ATTACK, *FIGHTER_DEMON_STATUS_KIND_ATTACK_COMBO].contains(&status_kind) {
				original!()(boma, int)
			};
		} else if int == *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO {
			HAS_ENABLE_COMBO_ON[WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize] = true;
			let status_kind = smash::app::lua_bind::StatusModule::status_kind(boma);
			let fighter_kind = smash::app::utility::get_kind(boma);
			if status_kind != *FIGHTER_STATUS_KIND_ATTACK  {
				original!()(boma, int)
			};
		} else if int == *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_NO_HIT_COMBO {
			HAS_ENABLE_NO_HIT_COMBO_ON[WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize] = true;
			let status_kind = smash::app::lua_bind::StatusModule::status_kind(boma);
			let fighter_kind = smash::app::utility::get_kind(boma);
			if status_kind != *FIGHTER_STATUS_KIND_ATTACK  {
				original!()(boma, int)
			};
		} else if int == *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_JUMP_MINI {
			let ENTRY_ID =  WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
			//Removal of SH macro via hooking on_flag. FULL_HOP_ENABLE_DELAY allows fullhop button to not give shorthops. 
				if (ControlModule::check_button_off(boma, *CONTROL_PAD_BUTTON_JUMP) || ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_JUMP_MINI)) && !(FULL_HOP_ENABLE_DELAY[ENTRY_ID] > 0) {
					original!()(boma, int)
				} else {
					println!("SH height banned");
				}
		} else if int == *FIGHTER_INSTANCE_WORK_ID_FLAG_CATCHED_BUTTERFLYNET {
				original!()(boma, int)
		}	else {
			original!()(boma, int)
		}
	} else {
		original!()(boma, int)
	}
}
#[skyline::hook(replace = smash::app::lua_bind::WorkModule::off_flag)]
pub unsafe fn off_flag_hook(boma: &mut smash::app::BattleObjectModuleAccessor, int: c_int) -> () {
	if smash::app::utility::get_category(boma) != *BATTLE_OBJECT_CATEGORY_FIGHTER {
		original!()(boma, int)
	}
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
#[skyline::hook(replace = smash::app::lua_bind::ArticleModule::generate_article)]
pub unsafe fn article_hook(boma: &mut smash::app::BattleObjectModuleAccessor, int: c_int, arg3: bool, arg4: c_int) -> u64 {
	if smash::app::utility::get_category(boma) != *BATTLE_OBJECT_CATEGORY_FIGHTER {
		return original!()(boma, int, arg3, arg4)
	}
	if int == *FIGHTER_WARIO_GENERATE_ARTICLE_WARIOBIKE && smash::app::utility::get_kind(boma) == *FIGHTER_KIND_WARIO {
		let status_kind = smash::app::lua_bind::StatusModule::status_kind(boma);
		if ![*FIGHTER_STATUS_KIND_ENTRY, *FIGHTER_STATUS_KIND_WIN].contains(&status_kind) && smash::app::sv_information::is_ready_go() {
			return 0
		} else {
			return original!()(boma, int, arg3, arg4)
		}
	} else if smash::app::utility::get_kind(boma) == *FIGHTER_KIND_MURABITO && is_added(boma) {
		let status_kind = smash::app::lua_bind::StatusModule::status_kind(boma);
		if int == *FIGHTER_MURABITO_GENERATE_ARTICLE_CLAYROCKET {
			if ![*FIGHTER_STATUS_KIND_FINAL, *FIGHTER_MURABITO_STATUS_KIND_FINAL_END, *FIGHTER_MURABITO_STATUS_KIND_FINAL_CHEER, *FIGHTER_MURABITO_STATUS_KIND_FINAL_HAPPY, *FIGHTER_MURABITO_STATUS_KIND_FINAL_MONEY, *FIGHTER_MURABITO_STATUS_KIND_FINAL_SURPRISE].contains(&status_kind) {
				return 0
			}
		}
		if [*FIGHTER_MURABITO_GENERATE_ARTICLE_TOMNOOK, *FIGHTER_MURABITO_GENERATE_ARTICLE_MONEYBAG, *FIGHTER_MURABITO_GENERATE_ARTICLE_FURNITURE].contains(&int){
			return 0
		}
		if [*FIGHTER_MURABITO_GENERATE_ARTICLE_BEETLE].contains(&int){
			if [hash40("win_3"), hash40("win_3_wait")].contains(&MotionModule::motion_kind(boma)) {
				return original!()(boma, int, arg3, arg4)
			} else {
				return 0
			}
		}
		if smash::app::sv_information::is_ready_go() && int == *FIGHTER_MURABITO_GENERATE_ARTICLE_HOUSE{
			return 0
		}
		return original!()(boma, int, arg3, arg4)
	} else {
		return original!()(boma, int, arg3, arg4)
	}
}


unsafe extern "C" fn util_update(fighter : &mut L2CFighterCommon) {
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
			FULL_HOP_ENABLE_DELAY[ENTRY_ID] = 0;
			println!("Does Entry {} have AB Smash? {}",ENTRY_ID, IS_AB[ENTRY_ID]);
		};
		if FULL_HOP_ENABLE_DELAY[ENTRY_ID] > 0 {
			FULL_HOP_ENABLE_DELAY[ENTRY_ID] -= 1;
		};
		if  PostureModule::scale(boma) != 0.001345 {
			PREV_SCALE[ENTRY_ID] = PostureModule::scale(boma);
		};
		if [*FIGHTER_STATUS_KIND_CAPTURE_PULLED, *FIGHTER_STATUS_KIND_CAPTURE_WAIT, *FIGHTER_STATUS_KIND_THROWN].contains(&status_kind) {
			let opponent_id = LinkModule::get_parent_object_id(boma, *LINK_NO_CAPTURE) as u32;
			let grabber_boma = smash::app::sv_battle_object::module_accessor(opponent_id);
			let grabber_kind = smash::app::utility::get_kind(&mut *grabber_boma);
			let graber_entry_id = WorkModule::get_int(&mut *grabber_boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
			//Toad Specific Code. Has the opponent be made real small while in the pipe, and removes the grab model changes present in villy
			if grabber_kind == *FIGHTER_KIND_MURABITO && is_added(&mut *grabber_boma) {
				println!("Turning off butterfly net flag");
				let grabber_motion = MotionModule::motion_kind(grabber_boma);
				let grabber_frame = MotionModule::frame(grabber_boma);
				if grabber_motion == hash40("throw_hi") && (3..35).contains(&(grabber_frame as i32)) {
					PostureModule::set_scale(boma, 0.001345, false);
				} else {
					PostureModule::set_scale(boma, PREV_SCALE[ENTRY_ID], false);
				};
				WorkModule::off_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_CATCHED_BUTTERFLYNET);
			} else {
				PostureModule::set_scale(boma, PREV_SCALE[ENTRY_ID], false);
			};
		} else {
			PREV_SCALE[ENTRY_ID] = PostureModule::scale(boma);
		};
		//This checks if the Full Hop button is pressed
		let triggered_buttons: Buttons = unsafe {
			Buttons::from_bits_unchecked(ControlModule::get_button(boma) & !ControlModule::get_button_prev(boma))
		};
		if triggered_buttons.intersects(Buttons::FullHop) {
			FULL_HOP_ENABLE_DELAY[ENTRY_ID] = 14;
		};
		if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_JUMP_MINI) { //Removes possibility of FH coming out of a SH. Shorthop button has priority over Fullhop
			FULL_HOP_ENABLE_DELAY[ENTRY_ID] = 0;
		};
		if status_kind == *FIGHTER_STATUS_KIND_JUMP_SQUAT && ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_GUARD) {
			FULL_HOP_ENABLE_DELAY[ENTRY_ID] = 0;
			WorkModule::set_flag(boma, true, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_JUMP_MINI);
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
		POS_X[ENTRY_ID] = PostureModule::pos_x(boma);
		POS_Y[ENTRY_ID] = PostureModule::pos_y(boma);
		//println!("X Accel: {}, Y Accel: {}, X Speed: {}, Y Speed: {}", ACCEL_X[ENTRY_ID], ACCEL_Y[ENTRY_ID], SPEED_X[ENTRY_ID], SPEED_Y[ENTRY_ID]);
		/*if ENTRY_ID == 0 {
			println!("Can Neutralb: {}, Can Sideb: {}, Can Upb: {}, Can Downb: {}", CAN_NEUTRALB[ENTRY_ID], CAN_SIDEB[ENTRY_ID], CAN_UPB[ENTRY_ID], CAN_DOWNB[ENTRY_ID]);
		}*/
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

pub(crate) fn is_jump(boma: &mut smash::app::BattleObjectModuleAccessor) -> bool {
	unsafe {
		if ControlModule::check_button_on_trriger(boma, *CONTROL_PAD_BUTTON_JUMP) {
			return true;
		};
		if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_FLICK_JUMP) {
			if ControlModule::get_flick_y(boma) >= 3 && ControlModule::get_stick_y(boma) >= 0.7 {
				return true;
			};
		};
		if ControlModule::check_button_on_trriger(boma, *CONTROL_PAD_BUTTON_JUMP_MINI) {
			return true;
		};
		return false;
	}
}
//Cancel Frames
pub(crate) unsafe fn reimpl_cancel_frame(fighter: &mut L2CAgentBase) -> () {
	let frame = MotionModule::frame(fighter.module_accessor);
	let cancel_frame = FighterMotionModuleImpl::get_cancel_frame(fighter.module_accessor,smash::phx::Hash40::new_raw(MotionModule::motion_kind(fighter.module_accessor)),false) as f32; //Cancel frame
	let situation_kind = StatusModule::situation_kind(fighter.module_accessor);
	if situation_kind == *SITUATION_KIND_GROUND {
		if frame >= cancel_frame{
			if (
				(ControlModule::get_command_flag_cat(fighter.module_accessor, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_S4) != 0 ||
				(ControlModule::get_command_flag_cat(fighter.module_accessor, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_LW4) != 0 ||
				(ControlModule::get_command_flag_cat(fighter.module_accessor, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_HI4) != 0 ||
				(ControlModule::get_command_flag_cat(fighter.module_accessor, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_N) != 0 ||
				(ControlModule::get_command_flag_cat(fighter.module_accessor, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_S3) != 0 ||
				(ControlModule::get_command_flag_cat(fighter.module_accessor, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_LW3) != 0 ||
				(ControlModule::get_command_flag_cat(fighter.module_accessor, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_HI3) != 0 ||
				(ControlModule::get_command_flag_cat(fighter.module_accessor, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_ANY) != 0 ||
				(ControlModule::get_command_flag_cat(fighter.module_accessor, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_TURN_DASH) != 0 ||
				(ControlModule::get_command_flag_cat(fighter.module_accessor, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_DASH) != 0 ||
				ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_GUARD) ||
				(ControlModule::check_button_on_trriger(fighter.module_accessor, *CONTROL_PAD_BUTTON_JUMP) && WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT) < WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT_MAX))||
				(ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_FLICK_JUMP) && ControlModule::get_flick_y(fighter.module_accessor) >= 3 && ControlModule::get_stick_y(fighter.module_accessor) >= 0.7 && WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT) < WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT_MAX))
			) {
				StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_WAIT, false);
			}
		}
	}
	if situation_kind == *SITUATION_KIND_GROUND {
		if frame >= cancel_frame{
			if (
				(ControlModule::get_command_flag_cat(fighter.module_accessor, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_S4) != 0 ||
				(ControlModule::get_command_flag_cat(fighter.module_accessor, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_LW4) != 0 ||
				(ControlModule::get_command_flag_cat(fighter.module_accessor, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_HI4) != 0 ||
				(ControlModule::get_command_flag_cat(fighter.module_accessor, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_N) != 0 ||
				(ControlModule::get_command_flag_cat(fighter.module_accessor, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_S3) != 0 ||
				(ControlModule::get_command_flag_cat(fighter.module_accessor, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_LW3) != 0 ||
				(ControlModule::get_command_flag_cat(fighter.module_accessor, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_HI3) != 0 ||
				(ControlModule::get_command_flag_cat(fighter.module_accessor, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_ANY) != 0 ||
				(ControlModule::get_command_flag_cat(fighter.module_accessor, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_TURN_DASH) != 0 ||
				(ControlModule::get_command_flag_cat(fighter.module_accessor, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_DASH) != 0 ||
				ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_GUARD) ||
				(ControlModule::check_button_on_trriger(fighter.module_accessor, *CONTROL_PAD_BUTTON_JUMP) && WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT) < WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT_MAX))||
				(ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_FLICK_JUMP) && ControlModule::get_flick_y(fighter.module_accessor) >= 3 && ControlModule::get_stick_y(fighter.module_accessor) >= 0.7 && WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT) < WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT_MAX))
			) {
				StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_WAIT, false);
			}
		}
	} else {
		
		if frame >= cancel_frame{
			if (
				(ControlModule::get_command_flag_cat(fighter.module_accessor, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_S4) != 0 ||
				(ControlModule::get_command_flag_cat(fighter.module_accessor, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_LW4) != 0 ||
				(ControlModule::get_command_flag_cat(fighter.module_accessor, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_HI4) != 0 ||
				(ControlModule::get_command_flag_cat(fighter.module_accessor, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_N) != 0 ||
				(ControlModule::get_command_flag_cat(fighter.module_accessor, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_S3) != 0 ||
				(ControlModule::get_command_flag_cat(fighter.module_accessor, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_LW3) != 0 ||
				(ControlModule::get_command_flag_cat(fighter.module_accessor, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_HI3) != 0 ||
				(ControlModule::get_command_flag_cat(fighter.module_accessor, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_ANY) != 0 ||
				ControlModule::check_button_on_trriger(fighter.module_accessor, *CONTROL_PAD_BUTTON_GUARD) ||
				(ControlModule::check_button_on_trriger(fighter.module_accessor, *CONTROL_PAD_BUTTON_JUMP) && WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT) < WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT_MAX)) ||
				(ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_FLICK_JUMP) && ControlModule::get_flick_y(fighter.module_accessor) >= 3 && ControlModule::get_stick_y(fighter.module_accessor) >= 0.7 && WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT) < WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT_MAX))
			) {
				StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_FALL, false);
			}
		}
	}
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

pub(crate) unsafe fn is_default(boma: &mut smash::app::BattleObjectModuleAccessor) -> bool {
	if WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) < 16  {
		return true 
	} else {
		return false
	}
}
pub(crate) unsafe fn is_added(boma: &mut smash::app::BattleObjectModuleAccessor) -> bool {
	if (WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) >= 120 && WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) <= 127)  {
		return true 
	} else {
		return false
	}
}
pub(crate) unsafe fn set_knockdown_throw(fighter: &mut L2CAgentBase) -> () {
	let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);    
	let opponent_id = LinkModule::get_node_object_id(boma, *LINK_NO_CAPTURE) as u32;
	let grabber_boma = smash::app::sv_battle_object::module_accessor(opponent_id);
	let grabber_kind = smash::app::utility::get_kind(&mut *grabber_boma);
	let grabber_entry_id = WorkModule::get_int(&mut *grabber_boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
	IS_KD_THROW[grabber_entry_id] = true;
}


pub fn install() {
    Agent::new("fighter")
	.on_line(Main, util_update)
	.install();
	skyline::install_hook!(is_enable_transition_term_hook);
	skyline::install_hook!(on_flag_hook);
	skyline::install_hook!(off_flag_hook);
	skyline::install_hook!(article_hook);
}
