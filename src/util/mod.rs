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
use std::{fs, path::Path};
use crate::controls::ext::*;
use crate::common::*;
use cached::proc_macro::cached;
use std::collections::{HashMap, HashSet};
use once_cell::sync::Lazy;
use smash::lib::lua_const::*;
use smash::hash40;
use std::time::Instant;

pub static mut GAMEMODES : Vec<String> = Vec::new();

pub static mut PREV_SCALE : [f32; 8] = [0.0; 8];
pub static mut IS_AB : [bool; 8] = [false; 8];
pub static mut IS_KD_THROW : [bool; 8] = [false; 8];

pub static mut TAP_JUMP_BUFFER : [i32; 8] = [0; 8];
pub const TAP_JUMP_BUFFER_MAX : i32 = 6;


pub static mut JC_GRAB_LOCKOUT : [i32; 8] = [0; 8];
pub const MAX_LOCKOUT : i32 = 10;

//Universal Settings

pub static mut IS_MECHANICS_ENABLED : bool = true;
pub static mut IS_SMALL_HOLD_BUFFER : bool = false;
pub static mut IS_SH_AERIAL : bool = true;


// Info State
#[derive(Clone, Copy)]
pub struct InfoState {
	pub pos_x: f32,
	pub pos_y: f32,
	pub speed_y: f32,
	pub speed_x: f32,
	pub prev_speed_x: f32,
	pub prev_speed_y: f32,
	pub accel_x : f32,
	pub accel_y : f32,
	pub stick_dir : f32,
	pub status_duration : i32,
	pub motion_duration : i32,
	pub __status : i32,
	pub __motion : u64,
	pub sub_stick : Vector2f,
	pub full_hop_enable_delay : i32,
}
impl Default for InfoState {
    fn default() -> Self {
        Self {
            sub_stick: Vector2f { x: 0.0, y: 0.0 },
            ..unsafe { std::mem::zeroed() } 
        }
    }
}


// Transition Enable State
#[derive(Default, Clone, Copy)]
pub struct TransitionEnableState {
    pub can_upb: i32,
    pub can_sideb: i32,
    pub can_downb: i32,
    pub can_neutralb: i32,
    pub can_jump_squat: i32,
    pub can_double_jump: i32,
    pub can_cliff: i32,
    pub can_attack_air: i32,
    pub can_airdodge: i32,
	pub can_spotdodge: i32,
    pub can_rapid_jab: i32,
    pub can_jab: i32,
    pub can_dash: i32,
    pub can_grab: i32,
    pub can_turndash: i32,
}
#[macro_export]
macro_rules! transitions_reset_all {
    ($entry:expr) => {
        $crate::set_state!($entry, $crate::util::TransitionEnableState::default())
    };
}

#[macro_export]
macro_rules! is_transition_set {
    ($entry:expr, $field:ident) => {
        // Returns true if the field is 1, false otherwise
        $crate::get_state!($entry as usize, $crate::util::TransitionEnableState).$field == 1
    };
}

#[macro_export]
macro_rules! transition_reset {
    ($entry:expr, $field:ident) => {
        $crate::with_state!($entry, $crate::util::TransitionEnableState, state, {
            state.$field = 0;
        })
    };
}

#[macro_export]
macro_rules! transition_set {
    ($entry:expr, $field:ident) => {
        $crate::with_state!($entry, $crate::util::TransitionEnableState, state, {
            state.$field = 1;
        })
    };
}
pub static mut TO_RUN_FLAG: [bool; 8] = [false; 8];

//Jab Flags
#[derive(Default, Clone, Copy)]
pub struct JabState {
	pub has_enable_combo_on : bool,
	pub has_enable_100_on : bool,
	pub has_enable_no_hit_combo_on : bool,
}


#[skyline::hook(replace = smash::app::lua_bind::WorkModule::enable_transition_term)]
pub unsafe fn enable_transition_term_hook(boma: &mut smash::app::BattleObjectModuleAccessor, term: i32) -> () {
		let ENTRY_ID = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
		if smash::app::utility::get_category(boma) == *BATTLE_OBJECT_CATEGORY_FIGHTER && term == *FIGHTER_STATUS_TRANSITION_TERM_ID_DASH_TO_RUN {
			TO_RUN_FLAG[ENTRY_ID] = true;
		}
		original!()(boma, term)
}
#[skyline::hook(replace = smash::app::lua_bind::WorkModule::is_enable_transition_term)]
pub unsafe fn is_enable_transition_term_hook(boma: &mut smash::app::BattleObjectModuleAccessor, flag: i32) -> bool {
		let ENTRY_ID = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
		if smash::app::utility::get_category(boma) != *BATTLE_OBJECT_CATEGORY_FIGHTER {
			original!()(boma, flag);
		}
		let state = crate::get_state!(ENTRY_ID, TransitionEnableState);
		if 	(state.can_upb == 1 && flag == *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_HI) ||
			(state.can_sideb == 1 && flag == *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_S) ||
			(state.can_downb == 1 && flag == *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_LW) ||
			(state.can_neutralb == 1 && flag == *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_N) ||
			(state.can_cliff == 1 && flag == *FIGHTER_STATUS_TRANSITION_TERM_ID_CLIFF_CATCH) ||
			(state.can_airdodge == 1 && flag == *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ESCAPE_AIR) ||
			(state.can_spotdodge == 1 && flag == *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ESCAPE) ||
			(state.can_attack_air == 1 && flag == *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_AIR) ||
			(state.can_dash == 1 && flag == *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_DASH) ||
			(state.can_grab == 1 && flag == *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_CATCH) ||
			(state.can_turndash == 1 && flag == *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_TURN_DASH) ||
			(state.can_jump_squat == 1 && (flag == *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_SQUAT || flag == *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_SQUAT_BUTTON)) ||
			(state.can_double_jump == 1 && (flag == *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_AERIAL || flag == *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_AERIAL_BUTTON)) 
			{
				false
		} else {
				original!()(boma, flag)
		}
}
#[skyline::hook(replace = smash::app::lua_bind::WorkModule::on_flag)]
pub unsafe fn on_flag_hook(boma: &mut smash::app::BattleObjectModuleAccessor, int: c_int) -> () {
	if smash::app::utility::get_category(boma) == *BATTLE_OBJECT_CATEGORY_FIGHTER { 
		if int == *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_100 && is_mechanics_enabled() {
			let ENTRY_ID = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
			crate::with_state!(ENTRY_ID, JabState, state, {
				state.has_enable_100_on = true;
			});
			let status_kind = smash::app::lua_bind::StatusModule::status_kind(boma);
			let fighter_kind = smash::app::utility::get_kind(boma);
			if ![*FIGHTER_STATUS_KIND_ATTACK, *FIGHTER_DEMON_STATUS_KIND_ATTACK_COMBO].contains(&status_kind) {
				original!()(boma, int)
			};
		} else if int == *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO  && is_mechanics_enabled() {
			let ENTRY_ID = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
			crate::with_state!(ENTRY_ID, JabState, state, {
				state.has_enable_combo_on = true;
			});
			let status_kind = smash::app::lua_bind::StatusModule::status_kind(boma);
			let fighter_kind = smash::app::utility::get_kind(boma);
			if status_kind != *FIGHTER_STATUS_KIND_ATTACK  {
				original!()(boma, int)
			};
		} else if int == *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_NO_HIT_COMBO  && is_mechanics_enabled() {
			let ENTRY_ID = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
			crate::with_state!(ENTRY_ID, JabState, state, {
				state.has_enable_no_hit_combo_on = true;
			});
			let status_kind = smash::app::lua_bind::StatusModule::status_kind(boma);
			let fighter_kind = smash::app::utility::get_kind(boma);
			if status_kind != *FIGHTER_STATUS_KIND_ATTACK  {
				original!()(boma, int)
			};
		} else if int == *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_JUMP_MINI {
			let ENTRY_ID =  WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
			//full_hop_enable_delay allows fullhop button to not give shorthops.
			if IS_SH_AERIAL {
				if !(crate::get_state!(ENTRY_ID, InfoState).full_hop_enable_delay > 0){
					original!()(boma, int)
				} else {
					println!("SH height banned");
				}
			} else {
				if (ControlModule::check_button_off(boma, *CONTROL_PAD_BUTTON_JUMP) || ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_JUMP_MINI)) && !(crate::get_state!(ENTRY_ID, InfoState).full_hop_enable_delay > 0) {
					original!()(boma, int)
				} else {
					println!("SH height banned");
				}
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
		let ENTRY_ID = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
		crate::with_state!(ENTRY_ID, JabState, state, {
			state.has_enable_100_on = false;
		});
		original!()(boma, int)
	} else if int == *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO {
		let ENTRY_ID = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
		crate::with_state!(ENTRY_ID, JabState, state, {
			state.has_enable_combo_on = false;
		});
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
	if int == *FIGHTER_WARIO_GENERATE_ARTICLE_WARIOBIKE && smash::app::utility::get_kind(boma) == *FIGHTER_KIND_WARIO && Path::new("sd:/ultimate/ult-s/wario.flag").is_file() {
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
		let motion_kind = smash::app::lua_bind::MotionModule::motion_kind(boma);
		let prev_status = StatusModule::prev_status_kind(boma, 0);
		//Checks Frames since entering a status/frame
		crate::with_state!(ENTRY_ID, InfoState, state, {
			if status_kind != state.__status || is_reset() {
				state.status_duration = 0;
			} else {
				state.status_duration+= 1;
			};
			if motion_kind != state.__motion || is_reset() {
				state.motion_duration = 0;
			} else {
				state.motion_duration+= 1;
			};
			state.__status = status_kind;
			state.__motion = motion_kind;
		});
		//Resets inability to special
		if is_reset() {
			crate::transitions_reset_all!(ENTRY_ID);
			crate::with_state!(ENTRY_ID, InfoState, state, {
				state.full_hop_enable_delay = 0;
				state.stick_dir = 0.0;
			});
			crate::with_state!(ENTRY_ID, JabState, state, {
				state.has_enable_combo_on = false;
				state.has_enable_100_on = false;
				state.has_enable_no_hit_combo_on = false;
			});
			if smash::app::smashball::is_training_mode() {
				reset_gamemodes();
			}
		};
		crate::with_state!(ENTRY_ID, InfoState, state, {
			if state.full_hop_enable_delay > 0 {
				state.full_hop_enable_delay-= 1;
			}

		});
		if JC_GRAB_LOCKOUT[ENTRY_ID] > 0 {
			JC_GRAB_LOCKOUT[ENTRY_ID] -= 1;
		};
		if (ControlModule::get_stick_y(boma) >= 0.6875 && ControlModule::is_enable_flick_jump(boma)) {
			TAP_JUMP_BUFFER[ENTRY_ID] = TAP_JUMP_BUFFER_MAX;
		}
		if TAP_JUMP_BUFFER[ENTRY_ID] > 0 {
			TAP_JUMP_BUFFER[ENTRY_ID] -= 1;
			//println!("Tap Jump Buffer: {}", TAP_JUMP_BUFFER[ENTRY_ID]);
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
		crate::with_state!(ENTRY_ID, InfoState, state, {
			if triggered_buttons.intersects(Buttons::FullHop) {
				state.full_hop_enable_delay = 14;
			};
			if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_JUMP_MINI) { //Removes possibility of FH coming out of a SH. Shorthop button has priority over Fullhop
				state.full_hop_enable_delay = 0;
			};
			if status_kind == *FIGHTER_STATUS_KIND_JUMP_SQUAT && ControlModule::check_button_on_trriger(boma, *CONTROL_PAD_BUTTON_GUARD) {
				state.full_hop_enable_delay = 0;
				WorkModule::set_flag(boma, true, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_JUMP_MINI);
			};
		});
		crate::with_state!(ENTRY_ID, InfoState, state, {
			if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_CSTICK_ON) {
				if ControlModule::get_stick_x(boma) != 0.0 {
					state.sub_stick.x = ControlModule::get_stick_x(boma);
				};
				if ControlModule::get_stick_y(boma) != 0.0 {
					state.sub_stick.y = ControlModule::get_stick_y(boma);
				};
			} else {
				state.sub_stick.x = 0.0;
				state.sub_stick.y = 0.0;
			};
		});
		//Reset run flag check
		if ![*FIGHTER_STATUS_KIND_DASH, *FIGHTER_STATUS_KIND_TURN_DASH].contains(&status_kind) {
			TO_RUN_FLAG[ENTRY_ID] = false;
		}

		if motion_duration(boma) == 0 {
			crate::with_state!(ENTRY_ID, JabState, state, {
				state.has_enable_combo_on = false;
				state.has_enable_100_on = false;
				state.has_enable_no_hit_combo_on = false;
			});
		};
		//Speed and acceleration checks

		crate::with_state!(ENTRY_ID, InfoState, state, {
			if is_reset() {
				state.prev_speed_x = 0.0;
				state.prev_speed_y = 0.0;
				state.speed_x = 0.0;
				state.speed_y = 0.0;
				state.accel_x = 0.0;
				state.accel_y = 0.0;
			};
			let new_speed_x = KineticModule::get_sum_speed_x(boma, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
			let new_speed_y = KineticModule::get_sum_speed_y(boma, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
			state.accel_x = state.speed_x - new_speed_x;
			state.accel_y = state.speed_y - new_speed_y;
			state.prev_speed_x = state.speed_x;
			state.prev_speed_y = state.speed_y;
			state.speed_x = new_speed_x;
			state.speed_y = new_speed_y;

			state.pos_x = PostureModule::pos_x(boma);
			state.pos_y = PostureModule::pos_y(boma);

			let lr = PostureModule::lr(boma);
			let stick_x = ControlModule::get_stick_x(boma) * lr;
			let stick_y = ControlModule::get_stick_y(boma);
			if !(stick_x.abs() < 0.05 && stick_y.abs() < 0.05) {
				let mut angle = stick_x.atan2(stick_y).to_degrees();
				if angle < 0.0 {
					angle += 360.0;
				}
				if angle > 360.0 {
					angle -= 360.0;
				}
				state.stick_dir = angle;
			} else {
				state.stick_dir = -1.0;
			}
		});
    };
}

//Status and motion duration
pub(crate) unsafe fn status_duration(boma: &mut smash::app::BattleObjectModuleAccessor) -> i32 {
	let ENTRY_ID = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
	return crate::get_state!(ENTRY_ID, InfoState).status_duration;
}
pub(crate) unsafe fn motion_duration(boma: &mut smash::app::BattleObjectModuleAccessor) -> i32 {
	let ENTRY_ID = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
	return crate::get_state!(ENTRY_ID, InfoState).motion_duration;
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

#[inline(always)]
pub(crate) unsafe fn ray_check_pos(boma: &mut smash::app::BattleObjectModuleAccessor, x_distance : f32, y_distance: f32, ignore_plat: bool) -> u64 {
	GroundModule::ray_check(boma, &Vector2f{ x: PostureModule::pos_x(boma), y: PostureModule::pos_y(boma)}, &Vector2f{ x: x_distance, y: y_distance}, ignore_plat)
}

#[inline(always)]
pub(crate) unsafe fn get_prev_speed_x(boma: &mut smash::app::BattleObjectModuleAccessor) -> f32 {
	let ENTRY_ID = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
	return crate::get_state!(ENTRY_ID, InfoState).prev_speed_x
}

#[inline(always)]
pub(crate) unsafe fn get_prev_speed_y(boma: &mut smash::app::BattleObjectModuleAccessor) -> f32 {
	let ENTRY_ID = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
	return crate::get_state!(ENTRY_ID, InfoState).prev_speed_y
}

#[inline(always)]
pub(crate) unsafe fn get_speed_x(boma: &mut smash::app::BattleObjectModuleAccessor) -> f32 {
	let ENTRY_ID = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
	return crate::get_state!(ENTRY_ID, InfoState).speed_x
}

#[inline(always)]
pub(crate) unsafe fn get_speed_y(boma: &mut smash::app::BattleObjectModuleAccessor) -> f32 {
	let ENTRY_ID = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
	return crate::get_state!(ENTRY_ID, InfoState).speed_y
}

#[inline(always)]
pub(crate) unsafe fn get_accel_x(boma: &mut smash::app::BattleObjectModuleAccessor) -> f32 {
	let ENTRY_ID = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
	return crate::get_state!(ENTRY_ID, InfoState).accel_x
}

#[inline(always)]
pub(crate) unsafe fn get_accel_y(boma: &mut smash::app::BattleObjectModuleAccessor) -> f32 {
	let ENTRY_ID = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
	return crate::get_state!(ENTRY_ID, InfoState).accel_y
}

#[inline(always)]
pub(crate) unsafe fn get_to_run_flag(boma: &mut smash::app::BattleObjectModuleAccessor) -> bool {
	let ENTRY_ID = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
	return TO_RUN_FLAG[ENTRY_ID]
}

//Hitlag and Hitstun

#[inline(always)]
pub(crate) unsafe fn is_hitlag(boma: &mut smash::app::BattleObjectModuleAccessor) -> bool {
	if WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_HIT_STOP_ATTACK_SUSPEND_FRAME) < 1 {
		return false
	} else {
		return true
	}
}

#[inline(always)]
pub(crate) unsafe fn boma(fighter: &mut L2CFighterCommon) -> &mut smash::app::BattleObjectModuleAccessor {
	return smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
}

#[inline(always)]
pub(crate) unsafe fn get_hitlag(boma: &mut smash::app::BattleObjectModuleAccessor) -> i32 {
	return WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_HIT_STOP_ATTACK_SUSPEND_FRAME)
}

#[inline(always)]
pub(crate) unsafe fn get_stick_angle(boma: &mut smash::app::BattleObjectModuleAccessor) -> f32 {
	let ENTRY_ID = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
	return crate::get_state!(ENTRY_ID, InfoState).stick_dir
}

//Misc.

#[inline(always)]
pub(crate) unsafe fn is_reset() -> bool {
	if smash::app::sv_information::is_ready_go() {
		return false
	} else {
		return true
	}
}
//Fighter Manager Stuff (PROBABLY DOESNT WORK)

#[inline(always)]
pub(crate) unsafe fn get_fighter_manager() -> *mut smash::app::FighterManager {
	return  *(FIGHTER_MANAGER as *mut *mut smash::app::FighterManager)
}

#[inline(always)]
pub(crate) unsafe fn get_FighterEntryID(boma: &mut smash::app::BattleObjectModuleAccessor) -> smash::app::FighterEntryID {
	let ENTRY_ID = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID);
	return  smash::app::FighterEntryID(ENTRY_ID)
}

#[inline(always)]
pub(crate) unsafe fn get_fighter_info(boma: &mut smash::app::BattleObjectModuleAccessor) -> *mut smash::app::FighterInformation {
	return  smash::app::lua_bind::FighterManager::get_fighter_information(get_fighter_manager(), get_FighterEntryID(boma))
}


#[inline(always)]
pub(crate) unsafe fn is_results() -> bool {
	smash::app::lua_bind::FighterManager::is_result_mode(get_fighter_manager())
}

#[inline(always)]
pub(crate) unsafe fn is_angel_plat(boma: &mut smash::app::BattleObjectModuleAccessor) -> bool {
	smash::app::lua_bind::FighterManager::is_rebirth_plate_line(get_fighter_manager(), get_FighterEntryID(boma))
}

#[inline(always)]
pub(crate) unsafe fn total_fighters() -> i32 {
	smash::app::lua_bind::FighterManager::total_fighter_num(get_fighter_manager())
}


#[inline(always)]
pub(crate) unsafe fn stock_count(boma: &mut smash::app::BattleObjectModuleAccessor) -> u64 {
	smash::app::lua_bind::FighterInformation::stock_count(get_fighter_info(boma))
}


#[inline(always)]
pub(crate) unsafe fn is_default(boma: &mut smash::app::BattleObjectModuleAccessor) -> bool {
	if WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) < 16  {
		return true 
	} else {
		return false
	}
}

#[inline(always)]
pub(crate) unsafe fn is_added(boma: &mut smash::app::BattleObjectModuleAccessor) -> bool {
	if (WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) >= 120 && WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) <= 127)  {
		return true 
	} else {
		return false
	}
}

#[inline(always)]
pub(crate) unsafe fn set_knockdown_throw(fighter: &mut L2CAgentBase) -> () {
	let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);    
	let opponent_id = LinkModule::get_node_object_id(boma, *LINK_NO_CAPTURE) as u32;
	let grabber_boma = smash::app::sv_battle_object::module_accessor(opponent_id);
	let grabber_kind = smash::app::utility::get_kind(&mut *grabber_boma);
	let grabber_entry_id = WorkModule::get_int(&mut *grabber_boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
	IS_KD_THROW[grabber_entry_id] = true;
}

#[inline(always)]
pub(crate) fn is_on_ryujinx() -> bool {
    unsafe {
        // Ryujinx skip based on text addr
        let text_addr = skyline::hooks::getRegionAddress(skyline::hooks::Region::Text) as u64;
        if text_addr == 0x8504000 || text_addr == 0x80004000 {
            println!("we are on Emulator");
            return true;
        } else {
            println!("we are not on Emulator");
            return false;
        }
    }
}



#[inline(always)]
pub(crate) unsafe fn is_tap_djc(boma: &mut smash::app::BattleObjectModuleAccessor) -> bool {
	let ENTRY_ID = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
	return TAP_JUMP_BUFFER[ENTRY_ID] <= 0;
}


#[inline(always)]
pub(crate) unsafe fn is_mechanics_enabled() -> bool {
	return IS_MECHANICS_ENABLED || is_gamemode("rivals".to_string());
}

pub(crate) unsafe fn update_enabled_checks() -> () {
	IS_MECHANICS_ENABLED = !Path::new("sd:/ultimate/ult-s/sys-flags/mechanics.flag").is_file();
	IS_SMALL_HOLD_BUFFER = Path::new("sd:/ultimate/ult-s/sys-flags/hold.flag").is_file();
	IS_SH_AERIAL = Path::new("sd:/ultimate/ult-s/sys-flags/sh.flag").is_file();

	let all: Vec<i32> = vec![-1];
	if IS_MECHANICS_ENABLED  {
		//Setting values for everybody!
		param_config::update_float_2(*FIGHTER_KIND_ALL, all.clone(), (smash::hash40("tread_mini_jump_speed_y_mul"), 0, 1.0));
		param_config::update_float_2(*FIGHTER_KIND_ALL, all.clone(), (smash::hash40("tread_mini_jump_speed_y_mul"), 0, 0.475));
		param_config::update_attribute_mul_2(*FIGHTER_KIND_ALL, all.clone(), (smash::hash40("damage_fly_top_air_accel_y"), 0, 1.05));
		param_config::update_float_2(*FIGHTER_KIND_ALL, all.clone(), (smash::hash40("damage_fly_top_speed_y_stable"), 0, 1.84));
		param_config::update_int_2(*FIGHTER_KIND_ALL, all.clone(), (smash::hash40("param_motion"), smash::hash40("escape_air_slide_back_end_frame"), -1));
		param_config::update_float_2(*FIGHTER_KIND_ALL, all.clone(), (smash::hash40("param_motion"), smash::hash40("landing_frame_escape_air_slide_max"), 12.0));
		param_config::update_float_2(*FIGHTER_KIND_ALL, all.clone(), (smash::hash40("param_motion"), smash::hash40("landing_frame_escape_air_slide"), 12.0));
	} else if !is_gamemode("rivals".to_string()) {
		param_config::update_attribute_mul_2(*FIGHTER_KIND_ALL, all.clone(), (smash::hash40("damage_fly_top_air_accel_y"), 0, 1.0));
		param_config::update_float_2(*FIGHTER_KIND_ALL, all.clone(), (smash::hash40("damage_fly_top_speed_y_stable"), 0, 1.8));
		param_config::update_int_2(*FIGHTER_KIND_ALL, all.clone(), (smash::hash40("param_motion"), smash::hash40("escape_air_slide_back_end_frame"), 4));
		param_config::update_float_2(*FIGHTER_KIND_ALL, all.clone(), (smash::hash40("param_motion"), smash::hash40("landing_frame_escape_air_slide_max"), 20.0));
		param_config::update_float_2(*FIGHTER_KIND_ALL, all.clone(), (smash::hash40("param_motion"), smash::hash40("landing_frame_escape_air_slide"), 10.0));
	}
	if is_gamemode("rivals".to_string()){
		param_config::update_int_2(*FIGHTER_KIND_ALL, all.clone(), (smash::hash40("param_motion"), smash::hash40("escape_air_slide_back_end_frame"), -1));
		param_config::update_float_2(*FIGHTER_KIND_ALL, all.clone(), (smash::hash40("param_motion"), smash::hash40("landing_frame_escape_air_slide_max"), 5.0));
		param_config::update_float_2(*FIGHTER_KIND_ALL, all.clone(), (smash::hash40("param_motion"), smash::hash40("landing_frame_escape_air_slide"), 5.0));
		if !IS_MECHANICS_ENABLED {
			param_config::update_attribute_mul_2(*FIGHTER_KIND_ALL, all.clone(), (smash::hash40("damage_fly_top_air_accel_y"), 0, 1.0));
			param_config::update_float_2(*FIGHTER_KIND_ALL, all.clone(), (smash::hash40("damage_fly_top_speed_y_stable"), 0, 1.8));
		}
	}
}


#[inline(always)]
pub (crate) unsafe fn reset_gamemodes() -> () {
	GAMEMODES = Vec::new();
	reset_attack_blockers();
}

#[inline(always)]
pub (crate) unsafe fn reset_attack_blockers() {
    for id in 0..8 {
        transitions_reset_all!(id);
    }
}


#[inline(always)]
pub (crate) unsafe fn add_gamemode(mode: String) -> () {
	GAMEMODES.push(mode);
}


#[inline(always)]
pub (crate) unsafe fn is_gamemode(mode: String) -> bool {
	return GAMEMODES.contains(&mode);
}
/*
MARKER SEARCH! Very big and annoying, and I hate it.
*/
pub struct ModRegistry {
    registry: HashMap<String, HashMap<usize, HashSet<String>>>,
}
impl ModRegistry {
    pub fn fill() -> Self {
        let mut registry = HashMap::new();
        let mods_root = Path::new("sd:/ultimate/mods/");
		let start = Instant::now();

        let Ok(mod_folders) = fs::read_dir(mods_root) else { 
            return Self { registry };
        };

        for mod_entry in mod_folders.flatten() {
            let fighter_path = mod_entry.path().join("fighter");
            let Ok(char_entries) = fs::read_dir(fighter_path) else { continue; };

            for char_entry in char_entries.flatten() {
                let char_name = char_entry.file_name().to_string_lossy().into_owned();
                let body_path = char_entry.path().join("model/body");
                
                let Ok(c_entries) = fs::read_dir(body_path) else { continue; };

                for c_entry in c_entries.flatten() {
                    let path = c_entry.path();
                    let Some(folder_name) = path.file_name().and_then(|n| n.to_str()) else { continue; };
                    
                    if !folder_name.starts_with('c') || folder_name.len() < 2 { continue; }
                    let Ok(slot_idx) = folder_name[1..].parse::<usize>() else { continue; };

                    let Ok(files) = fs::read_dir(&path) else { continue; };
                    for file in files.flatten() {
                        let file_path = file.path();
                        if file_path.extension().and_then(|s| s.to_str()) == Some("marker") {
                            if let Some(marker_base) = file_path.file_stem().and_then(|s| s.to_str()) {
                                registry.entry(char_name.clone())
                                    .or_insert_with(HashMap::new)
                                    .entry(slot_idx)
                                    .or_insert_with(HashSet::new)
                                    .insert(marker_base.to_string());
                            }
                        }
                    }
                }
            }
        }
        let duration = start.elapsed();
		println!("Mod Registry Filled! Time Taken {:.4}s ({:.4}ms)", duration.as_micros() as f32 / 1000000.0, duration.as_micros() as f32 / 1000.0);
        Self { registry }
    }
    pub fn get_marked_costumes(&self, char_folder: &str, marker_name: &str) -> Vec<usize> {
        let mut slots = Vec::new();
        if let Some(char_data) = self.registry.get(char_folder) {
            for (&slot, markers) in char_data {
                if markers.contains(marker_name) {
                    slots.push(slot);
                }
            }
        }
        slots.sort_unstable();
        slots
    }

    pub fn get_costume_count(&self, char_folder: &str, marker_name: &str) -> u8 {
		let Some(char_data) = self.registry.get(char_folder) else { 
			return 0; 
		};
		
		let start_idx = char_data.iter()
			.filter(|(_, markers)| {
				let found = markers.contains(marker_name);
				found
			})
			.map(|(&slot, _)| slot)
			.min();

		if let Some(start) = start_idx {
			let mut count = 0;
			for i in start..256 {
				let has_marker = char_data.get(&i).map_or(false, |m| m.contains(marker_name));
				if has_marker {
					count += 1;
				} else {
					break;
				}
			}
			count as u8
		} else {
			0
		}
    }
    pub fn get_lowest_marked_costume(&self, char_folder: &str, marker_name: &str) -> u8 {
        let Some(char_data) = self.registry.get(char_folder) else {return 255;};

        let lowest = char_data.iter().filter(|(_, markers)| markers.contains(marker_name)).map(|(&slot, _)| slot).min();

        lowest.map(|idx| idx as u8).unwrap_or(255)
    }
}
pub static REGISTRY: Lazy<ModRegistry> = Lazy::new(|| ModRegistry::fill());
/*#[cached(
    key = "String", 
    convert = r#"{ format!("{}_{}", char_folder, marker_name) }"#
)]*/
pub(crate) fn get_marked_costumes(char_folder: &str, marker_name: &str) -> Vec<usize> {
    let marked_slots = REGISTRY.get_marked_costumes(char_folder, marker_name);
    println!("{}-{} slots - {:?}", char_folder, marker_name, marked_slots);
	marked_slots
}

/*#[cached(
    key = "String", 
    convert = r#"{ format!("{}_{}", char_folder, marker_name) }"#
)]*/
pub(crate) fn get_lowest_marked_costume(char_folder: &str, marker_name: &str) -> u8 {
    let lowest_marked = REGISTRY.get_lowest_marked_costume(char_folder, marker_name);
    println!("{}-{} lowest slot - {:?}", char_folder, marker_name, lowest_marked);
	lowest_marked
}

/*#[cached(
    key = "String", 
    convert = r#"{ format!("{}_{}", char_folder, marker_name) }"#
)]*/
pub(crate) fn get_costume_count(char_folder: &str, marker_name: &str) -> u8 {
    let costume_count = REGISTRY.get_costume_count(char_folder, marker_name);
	println!("{}-{} costume count - {:?}", char_folder, marker_name, costume_count);
	costume_count
}

pub fn install() {
    Agent::new("fighter")
	.on_line(Main, util_update)
	.install();
	skyline::install_hook!(is_enable_transition_term_hook);
	skyline::install_hook!(enable_transition_term_hook);
	skyline::install_hook!(on_flag_hook);
	skyline::install_hook!(off_flag_hook);
	skyline::install_hook!(article_hook);
	unsafe {
		update_enabled_checks();
	}
}
