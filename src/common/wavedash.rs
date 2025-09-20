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
use std::os::raw::c_int;
use std::os::raw::c_ulong;
use std::{fs, path::Path};

static mut IS_WAVEDASH: [bool; 8] = [false; 8];
static mut FORCE_WAVEDASH: [bool; 8] = [false; 8];
static mut WAVEDASH_DONE: [bool; 8] = [false; 8];
const TRACTION_MAX: f32 = 0.086;
const TRACTION_HIGH: f32 = 0.075;
const TRACTION_MID: f32 = 0.06;
const TRACTION_LOW: f32 = 0.054;
const TRACTION_MIN: f32 = 0.045;

pub(crate) fn get_wd_length(fighter_kind : i32) -> f32 {
	let max = [
		*FIGHTER_KIND_LITTLEMAC, *FIGHTER_KIND_CHROM, *FIGHTER_KIND_DEMON,
		*FIGHTER_KIND_PEACH, *FIGHTER_KIND_DAISY,
		*FIGHTER_KIND_ROCKMAN, *FIGHTER_KIND_CAPTAIN, *FIGHTER_KIND_ROBOT,
		*FIGHTER_KIND_KEN, *FIGHTER_KIND_RYU, *FIGHTER_KIND_ELIGHT
	];
	let high = [
		*FIGHTER_KIND_MURABITO, *FIGHTER_KIND_ZELDA, *FIGHTER_KIND_GANON,
		*FIGHTER_KIND_ROY, *FIGHTER_KIND_MIIFIGHTER, *FIGHTER_KIND_LUCAS,
		*FIGHTER_KIND_KROOL, *FIGHTER_KIND_MARIOD, *FIGHTER_KIND_WOLF,
		*FIGHTER_KIND_PIKACHU, *FIGHTER_KIND_PICKEL, *FIGHTER_KIND_YOUNGLINK,
		*FIGHTER_KIND_DONKEY, *FIGHTER_KIND_WIIFIT, *FIGHTER_KIND_PITB,
		*FIGHTER_KIND_SHEIK, *FIGHTER_KIND_GAOGAEN, *FIGHTER_KIND_DOLLY,
		*FIGHTER_KIND_DIDDY, *FIGHTER_KIND_PLIZARDON
	];
	let low = [
		*FIGHTER_KIND_PURIN, *FIGHTER_KIND_TANTAN, *FIGHTER_KIND_SONIC,
		*FIGHTER_KIND_GEKKOUGA, *FIGHTER_KIND_PALUTENA, *FIGHTER_KIND_MARTH,
		*FIGHTER_KIND_IKE, *FIGHTER_KIND_TOONLINK, *FIGHTER_KIND_LUCARIO,
		*FIGHTER_KIND_KAMUI, *FIGHTER_KIND_GAMEWATCH, *FIGHTER_KIND_JACK,
		*FIGHTER_KIND_ROSETTA, *FIGHTER_KIND_RIDLEY, *FIGHTER_KIND_TRAIL,
		*FIGHTER_KIND_MASTER, *FIGHTER_KIND_BRAVE, *FIGHTER_KIND_SZEROSUIT, *FIGHTER_KIND_PACMAN,
		*FIGHTER_KIND_SNAKE, *FIGHTER_KIND_METAKNIGHT
	];
	let min = [
		*FIGHTER_KIND_INKLING, *FIGHTER_KIND_LUIGI, *FIGHTER_KIND_SHIZUE,
		*FIGHTER_KIND_BUDDY, *FIGHTER_KIND_NANA, *FIGHTER_KIND_POPO,
		*FIGHTER_KIND_PIKMIN, *FIGHTER_KIND_KOOPAJR, *FIGHTER_KIND_PACKUN, 
		*FIGHTER_KIND_SAMUSD, *FIGHTER_KIND_PZENIGAME, *FIGHTER_KIND_EFLAME
	];
	if max.contains(&fighter_kind) {
		return TRACTION_MAX;
	};
	if high.contains(&fighter_kind) {
		return TRACTION_HIGH;
	};
	if low.contains(&fighter_kind) {
		return TRACTION_LOW;
	};
	if min.contains(&fighter_kind) {
		return TRACTION_MIN;
	};
	return TRACTION_MID;
}

unsafe extern "C" fn wavedash(fighter : &mut L2CFighterCommon) {
    unsafe {
		if !is_mechanics_enabled() {
			return;
		}
        let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);    
		let fighter_kind = smash::app::utility::get_kind(boma);
		let status_kind = smash::app::lua_bind::StatusModule::status_kind(boma);
		let ENTRY_ID = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
		let frame = MotionModule::frame(boma);
		let prev_status_kind = StatusModule::prev_status_kind(boma, 0);
		let stick_y = ControlModule::get_stick_y(boma);
		let stick_x = ControlModule::get_stick_x(boma);
		let is_neutral = stick_y.abs() < 0.66 && stick_x.abs() < 0.66;
		if status_kind == *FIGHTER_STATUS_KIND_LANDING {
			//I fucking hate that i had to do this
			//Gets new traction by subbing it from the old traction, getting the difference, and making sure it behaves properly. Gets the proper traction
			let desired_brake = get_wd_length(fighter_kind);
			let brake = WorkModule::get_param_float(fighter.module_accessor, hash40("ground_brake"), 0);
			let speed = get_speed_x(boma) * PostureModule::lr(boma);
			let mut added_speed = brake - desired_brake;
			if speed < 0.0 {
				added_speed *= -1.0;
			};
			if (speed <= 0.0 && (speed + added_speed) > 0.0) || (speed >= 0.0 && (speed + added_speed) < 0.0) {
				added_speed = 0.0;
			};
			let speed = smash::phx::Vector3f { x: added_speed, y: 0.0, z: 0.0 };
			KineticModule::add_speed(boma, &speed);


			//Fixes rolling out of wavedash
			if prev_status_kind == *FIGHTER_STATUS_KIND_JUMP_SQUAT && ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_GUARD) {
				ControlModule::clear_command_one(fighter.module_accessor, *FIGHTER_PAD_COMMAND_CATEGORY1, *FIGHTER_PAD_CMD_CAT1_ESCAPE);
				ControlModule::clear_command_one(fighter.module_accessor, *FIGHTER_PAD_COMMAND_CATEGORY1, *FIGHTER_PAD_CMD_CAT1_ESCAPE_B);
				ControlModule::clear_command_one(fighter.module_accessor, *FIGHTER_PAD_COMMAND_CATEGORY1, *FIGHTER_PAD_CMD_CAT1_ESCAPE_F);
				ControlModule::reset_main_stick_x(boma);
			}
		};
		if [*FIGHTER_STATUS_KIND_JUMP_SQUAT].contains(&status_kind) && ControlModule::check_button_on_trriger(boma, *CONTROL_PAD_BUTTON_GUARD) {
			IS_WAVEDASH[ENTRY_ID] = true;
		}
		let is_from_tumble = [*FIGHTER_STATUS_KIND_DAMAGE_FLY, *FIGHTER_STATUS_KIND_DAMAGE_FLY_ROLL, *FIGHTER_STATUS_KIND_DAMAGE_FALL].contains(&StatusModule::prev_status_kind(boma, 0));
		if [*FIGHTER_STATUS_KIND_ESCAPE_AIR, *FIGHTER_STATUS_KIND_ESCAPE_AIR_SLIDE].contains(&status_kind) && frame < 4.0 && !is_neutral && !is_from_tumble {
			if stick_y < 0.5 && (IS_WAVEDASH[ENTRY_ID] || ray_check_pos(boma, 0.0, -6.0, true) == 1){
				let distance_to_floor = 14;
				if ray_check_pos(boma, 0.0, -(distance_to_floor as f32), true) == 1 {
					GroundModule::attach_ground(fighter.module_accessor, true);
        			GroundModule::set_attach_ground(fighter.module_accessor, true);
					let mut teleport_distance = -(distance_to_floor as f32);
					for x in 0..distance_to_floor {
						if ray_check_pos(boma, 0.0, -(x as f32), true) == 1 {
							teleport_distance = -(x as f32);
							break;
						}
					}
					let pos = smash::phx::Vector3f { x: PostureModule::pos_x(boma), y: PostureModule::pos_y(boma)+(teleport_distance as f32), z: 0.0 };
					PostureModule::set_pos(boma, &pos);
        			fighter.change_status(FIGHTER_STATUS_KIND_LANDING.into(), false.into());
				}
			}
			IS_WAVEDASH[ENTRY_ID] = false;
		}
		if ![*FIGHTER_STATUS_KIND_JUMP_SQUAT, *FIGHTER_STATUS_KIND_ESCAPE_AIR, *FIGHTER_STATUS_KIND_ESCAPE_AIR_SLIDE, *FIGHTER_STATUS_KIND_JUMP].contains(&status_kind){
			IS_WAVEDASH[ENTRY_ID] = false;
		}
    };
}

#[skyline::hook(replace = smash::app::lua_bind::StatusModule::change_status_request)]
pub unsafe fn change_status_request_hook(boma: &mut smash::app::BattleObjectModuleAccessor, status_kind: i32, arg3: bool) -> u64 {
	let next_status = status_kind;
	let curr_status = StatusModule::status_kind(boma);
	let prev_status_1 = StatusModule::prev_status_kind(boma, 0);
	let prev_status_2 = StatusModule::prev_status_kind(boma, 1);
	let is_clear_buffer = arg3;
	if smash::app::utility::get_category(boma) == *BATTLE_OBJECT_CATEGORY_FIGHTER {
		if [*FIGHTER_STATUS_KIND_ESCAPE, *FIGHTER_STATUS_KIND_ESCAPE_F, *FIGHTER_STATUS_KIND_ESCAPE_B].contains(&next_status) {
			if !is_mechanics_enabled() {
				return original!()(boma, status_kind, arg3);
			}
			if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_JUMP) || ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_JUMP_MINI) {
				 original!()(boma, *FIGHTER_STATUS_KIND_JUMP_SQUAT, false)
			} else if (prev_status_1 == *FIGHTER_STATUS_KIND_LANDING && prev_status_2 == *FIGHTER_STATUS_KIND_JUMP_SQUAT && curr_status == *FIGHTER_STATUS_KIND_WAIT) ||
			(prev_status_1 == *FIGHTER_STATUS_KIND_JUMP_SQUAT && curr_status == *FIGHTER_STATUS_KIND_LANDING)
			{
				return 0 as u64
			}else {
				original!()(boma, status_kind, arg3)
			}
		}  else if next_status == *FIGHTER_STATUS_KIND_ICE_JUMP {
			if !is_mechanics_enabled() {
				return original!()(boma, status_kind, arg3);
			}
			original!()(boma, *FIGHTER_STATUS_KIND_TREAD_FALL, true)
		}else if next_status == *FIGHTER_STATUS_KIND_TURN && curr_status == *FIGHTER_STATUS_KIND_LANDING{
			if !is_mechanics_enabled() {
				return original!()(boma, status_kind, arg3);
			}
			return 0 as u64
		}  else if [*FIGHTER_STATUS_KIND_DOWN, *FIGHTER_STATUS_KIND_DOWN_WAIT, *FIGHTER_STATUS_KIND_SLIP_WAIT, *FIGHTER_STATUS_KIND_DAMAGE].contains(&curr_status) && next_status == *FIGHTER_STATUS_KIND_FALL{
			//Clears buffer when sliding off in a damage state to prevent accidental airdodges/aerials
			original!()(boma, status_kind, true)
		} else if smash::app::utility::get_kind(boma) == *FIGHTER_KIND_TRAIL && [*FIGHTER_TRAIL_STATUS_KIND_ATTACK_AIR_F].contains(&status_kind) && Path::new("sd:/ultimate/ult-s/trail.flag").is_file(){
			return 0 as u64
		} else if smash::app::utility::get_kind(boma) == *FIGHTER_KIND_MURABITO && [*FIGHTER_MURABITO_STATUS_KIND_SPECIAL_N_POCKET].contains(&status_kind) && Path::new("sd:/ultimate/ult-s/murabito.flag").is_file(){
			original!()(boma, *FIGHTER_STATUS_KIND_ITEM_THROW, arg3)
		} else {
			original!()(boma, status_kind, arg3)
		}
	} else {
		original!()(boma, status_kind, arg3)
	}
}
unsafe extern "C" fn status_pre_EscapeAir(fighter: &mut L2CFighterCommon) -> L2CValue {
    let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);    
	let ENTRY_ID = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
	let y = ControlModule::get_stick_y(boma);
	let fighter_kind = smash::app::utility::get_kind(boma);
    //Handles wavedash
	if !is_mechanics_enabled() {
		return smashline::original_status(Pre, fighter, *FIGHTER_STATUS_KIND_ESCAPE_AIR)(fighter);
	}
    if IS_WAVEDASH[ENTRY_ID] == true && y < 0.5 /*&& (ControlModule::check_button_off(boma, *CONTROL_PAD_BUTTON_JUMP) || ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_JUMP_MINI))*/ && GroundModule::ray_check(boma, &Vector2f{ x: PostureModule::pos_x(boma), y: PostureModule::pos_y(boma)}, &Vector2f{ x: 0.0, y: -3.0}, true) == 1 && fighter_kind != *FIGHTER_KIND_DEMON{
        GroundModule::attach_ground(fighter.module_accessor, true);
        GroundModule::set_attach_ground(fighter.module_accessor, true);
        fighter.change_status(FIGHTER_STATUS_KIND_LANDING.into(), false.into());
        return 0.into();
    }
    return smashline::original_status(Pre, fighter, *FIGHTER_STATUS_KIND_ESCAPE_AIR)(fighter);
}
#[skyline::hook(replace = smash::app::lua_bind::StatusModule::change_status_request_from_script)]
pub unsafe fn change_status_request_script_hook(boma: &mut smash::app::BattleObjectModuleAccessor, status_kind: i32, arg3: bool) -> u64 {
	let next_status = status_kind;
	let is_clear_buffer = arg3;
	let prev_status_1 = StatusModule::prev_status_kind(boma, 0);
	let prev_status_2 = StatusModule::prev_status_kind(boma, 1);
	let curr_status = StatusModule::status_kind(boma);
	if !is_mechanics_enabled() {
		return original!()(boma, status_kind, arg3);
	}
	if smash::app::utility::get_category(boma) == *BATTLE_OBJECT_CATEGORY_FIGHTER {
		if [*FIGHTER_STATUS_KIND_ESCAPE, *FIGHTER_STATUS_KIND_ESCAPE_F, *FIGHTER_STATUS_KIND_ESCAPE_B].contains(&next_status) {
			if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_JUMP) || ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_JUMP_MINI) {
				 original!()(boma, *FIGHTER_STATUS_KIND_JUMP_SQUAT, false)
			} else if (prev_status_1 == *FIGHTER_STATUS_KIND_LANDING && prev_status_2 == *FIGHTER_STATUS_KIND_JUMP_SQUAT && curr_status == *FIGHTER_STATUS_KIND_WAIT) ||
			(prev_status_1 == *FIGHTER_STATUS_KIND_JUMP_SQUAT && curr_status == *FIGHTER_STATUS_KIND_LANDING)
			{
				return 0 as u64
			}else {
				original!()(boma, status_kind, arg3)
			}
		} else if next_status == *FIGHTER_STATUS_KIND_TURN && curr_status == *FIGHTER_STATUS_KIND_LANDING{
			return 0 as u64
		} else if smash::app::utility::get_kind(boma) == *FIGHTER_KIND_TRAIL && [*FIGHTER_TRAIL_STATUS_KIND_ATTACK_AIR_F].contains(&status_kind) && Path::new("sd:/ultimate/ult-s/trail.flag").is_file(){
			return 0 as u64
		}else {
			original!()(boma, status_kind, arg3)
		}
	} else {
		original!()(boma, status_kind, arg3)
	}
}
 

pub fn install() {
    Agent::new("fighter")
    .set_costume((0..255).collect())
	.on_line(Main, wavedash)
    .install();
	skyline::install_hooks!(
		change_status_request_hook,
		change_status_request_script_hook
    );
}