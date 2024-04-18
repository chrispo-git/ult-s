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
		*FIGHTER_KIND_PEACH, *FIGHTER_KIND_DAISY, *FIGHTER_KIND_SZEROSUIT,
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
		*FIGHTER_KIND_MASTER, *FIGHTER_KIND_BRAVE, *FIGHTER_KIND_PACMAN,
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
        let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);    
		let fighter_kind = smash::app::utility::get_kind(boma);
		let status_kind = smash::app::lua_bind::StatusModule::status_kind(boma);
		let ENTRY_ID = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
		let frame = MotionModule::frame(boma);
		let prev_status_kind = StatusModule::prev_status_kind(boma, 0);
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
		if ![*FIGHTER_KIND_POPO, *FIGHTER_KIND_NANA].contains(&fighter_kind) {
			if status_kind == *FIGHTER_STATUS_KIND_JUMP_SQUAT {
				IS_WAVEDASH[ENTRY_ID] = true;
			};
			if [*FIGHTER_STATUS_KIND_JUMP_SQUAT, *FIGHTER_STATUS_KIND_ESCAPE_AIR, *FIGHTER_STATUS_KIND_ESCAPE_AIR_SLIDE].contains(&status_kind) == false {
				IS_WAVEDASH[ENTRY_ID] = false;
				WAVEDASH_DONE[ENTRY_ID] = false;
			};
			if [*FIGHTER_STATUS_KIND_ESCAPE_AIR, *FIGHTER_STATUS_KIND_ESCAPE_AIR_SLIDE].contains(&status_kind) && IS_WAVEDASH[ENTRY_ID] == true {
				let y = ControlModule::get_stick_y(boma);
				let x = ControlModule::get_stick_x(boma);
				if fighter_kind == *FIGHTER_KIND_DEMON && GroundModule::ray_check(boma, &Vector2f{ x: PostureModule::pos_x(boma), y: PostureModule::pos_y(boma)}, &Vector2f{ x: 0.0, y: -3.0}, true) == 1{
					if y < 0.3 {//&& (ControlModule::check_button_off(boma, *CONTROL_PAD_BUTTON_JUMP) || ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_JUMP_MINI)){
						let stop_rise  = smash::phx::Vector3f { x: 1.0, y: 0.0, z: 1.0 };
						KineticModule::mul_speed(boma, &stop_rise, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
						let mut z = 0;
						while GroundModule::ray_check(boma, &Vector2f{ x: PostureModule::pos_x(boma), y: PostureModule::pos_y(boma)}, &Vector2f{ x: 0.0, y: -0.25}, true) == 0  && z < 30 && GroundModule::ray_check(boma, &Vector2f{ x: PostureModule::pos_x(boma), y: PostureModule::pos_y(boma)}, &Vector2f{ x: 0.0, y: -3.0}, true) == 1{
							let pos = smash::phx::Vector3f { x: PostureModule::pos_x(boma), y: PostureModule::pos_y(boma)-0.25, z: 0.0 };
							PostureModule::set_pos(boma, &pos);
							PostureModule::init_pos(boma, &pos, true, true);
							z += 1;
						};
						if x > -0.2 && x < 0.2 {
							let stop_rise  = smash::phx::Vector3f { x: 0.0, y: 1.0, z: 1.0 };
							KineticModule::mul_speed(boma, &stop_rise, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
							WAVEDASH_DONE[ENTRY_ID] = true;
						};
						StatusModule::set_situation_kind(boma, smash::app::SituationKind(*SITUATION_KIND_GROUND), true);
					};
				};
				IS_WAVEDASH[ENTRY_ID] = false;
			};
			/*if WAVEDASH_DONE[ENTRY_ID] == true && status_kind == *FIGHTER_STATUS_KIND_LANDING{
				let stop_rise  = smash::phx::Vector3f { x: 0.0, y: 1.0, z: 1.0 };
				KineticModule::mul_speed(boma, &stop_rise, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
				let speed = smash::phx::Vector3f { x: 0.1, y: 0.0, z: 0.0 };
				KineticModule::add_speed(boma, &speed);
				WAVEDASH_DONE[ENTRY_ID] = false;
			};*/
		} else {
			if status_kind == *FIGHTER_STATUS_KIND_JUMP_SQUAT {
				if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_GUARD) {
					FORCE_WAVEDASH[ENTRY_ID] = true;
				};
				IS_WAVEDASH[ENTRY_ID] = true;

			};
			if [*FIGHTER_STATUS_KIND_JUMP_SQUAT, *FIGHTER_STATUS_KIND_ESCAPE_AIR, *FIGHTER_STATUS_KIND_ESCAPE_AIR_SLIDE].contains(&status_kind) == false {
				IS_WAVEDASH[ENTRY_ID] = false;
				WAVEDASH_DONE[ENTRY_ID] = false;
				FORCE_WAVEDASH[ENTRY_ID] = false;
			};
			if [*FIGHTER_STATUS_KIND_ESCAPE_AIR, *FIGHTER_STATUS_KIND_ESCAPE_AIR_SLIDE].contains(&status_kind) && IS_WAVEDASH[ENTRY_ID] == true {
				let y = ControlModule::get_stick_y(boma);
				if y < 0.3 && y > -0.3 {
					let speed = smash::phx::Vector3f { x: 0.0, y: -3.0, z: 0.0 };
					KineticModule::add_speed(boma, &speed);
					WAVEDASH_DONE[ENTRY_ID] = true;
				};
				IS_WAVEDASH[ENTRY_ID] = false;
			};
			if WAVEDASH_DONE[ENTRY_ID] == true {
				if MotionModule::frame(boma) > 3.0 && MotionModule::frame(boma) < 6.0 {
					let stop_rise  = smash::phx::Vector3f { x: 1.0, y: 0.0, z: 1.0 };
					KineticModule::mul_speed(boma, &stop_rise, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
					WAVEDASH_DONE[ENTRY_ID] = false;
				};
			};
		};
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
		} 
		 else if [*FIGHTER_STATUS_KIND_ESCAPE_AIR, *FIGHTER_STATUS_KIND_ESCAPE_AIR_SLIDE, *FIGHTER_STATUS_KIND_JUMP].contains(&next_status)  {
			let ENTRY_ID = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
			if IS_WAVEDASH[ENTRY_ID] == true {
				StatusModule::set_situation_kind(boma, smash::app::SituationKind(*SITUATION_KIND_GROUND), true);
				if next_status == *FIGHTER_STATUS_KIND_JUMP && FORCE_WAVEDASH[ENTRY_ID] {
					original!()(boma, *FIGHTER_STATUS_KIND_ESCAPE_AIR_SLIDE, arg3)
				} else {
					original!()(boma, status_kind, arg3)
				}
			} else {
				original!()(boma, status_kind, arg3)
			}
		} else if [*FIGHTER_STATUS_KIND_DOWN, *FIGHTER_STATUS_KIND_DOWN_WAIT, *FIGHTER_STATUS_KIND_SLIP_WAIT, *FIGHTER_STATUS_KIND_DAMAGE].contains(&curr_status) && next_status == *FIGHTER_STATUS_KIND_FALL{
			//Clears buffer when sliding off in a damage state to prevent accidental airdodges/aerials
			original!()(boma, status_kind, true)
		}else if smash::app::utility::get_kind(boma) == *FIGHTER_KIND_WARIO && [*FIGHTER_STATUS_KIND_SPECIAL_S, *FIGHTER_WARIO_STATUS_KIND_SPECIAL_S_BUMP, *FIGHTER_WARIO_STATUS_KIND_SPECIAL_S_DOWN, *FIGHTER_WARIO_STATUS_KIND_SPECIAL_S_RIDE, *FIGHTER_WARIO_STATUS_KIND_SPECIAL_S_DRIVE, *FIGHTER_WARIO_STATUS_KIND_SPECIAL_S_SEARCH, *FIGHTER_WARIO_STATUS_KIND_SPECIAL_S_APPEAL, *FIGHTER_WARIO_STATUS_KIND_SPECIAL_S_ESCAPE, *FIGHTER_WARIO_STATUS_KIND_SPECIAL_S_WHEELIE, *FIGHTER_WARIO_STATUS_KIND_SPECIAL_S_TURN_END, *FIGHTER_WARIO_STATUS_KIND_SPECIAL_S_TURN_LOOP, *FIGHTER_WARIO_STATUS_KIND_SPECIAL_S_THROWN_OUT, *FIGHTER_WARIO_STATUS_KIND_SPECIAL_S_TURN_START, *FIGHTER_WARIO_STATUS_KIND_SPECIAL_S_ESCAPE_START].contains(&status_kind){
			original!()(boma, *FIGHTER_WARIO_STATUS_KIND_SPECIAL_S_START, arg3)
		} else if smash::app::utility::get_kind(boma) == *FIGHTER_KIND_TRAIL && [*FIGHTER_TRAIL_STATUS_KIND_ATTACK_AIR_F].contains(&status_kind){
			return 0 as u64
		} else if smash::app::utility::get_kind(boma) == *FIGHTER_KIND_MURABITO && [*FIGHTER_MURABITO_STATUS_KIND_SPECIAL_N_POCKET].contains(&status_kind){
			original!()(boma, *FIGHTER_STATUS_KIND_ITEM_THROW, arg3)
		}
		else if next_status == *FIGHTER_STATUS_KIND_JUMP_SQUAT{
			let ENTRY_ID = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
			IS_WAVEDASH[ENTRY_ID] = true;
			if ControlModule::check_button_on_trriger(boma, *CONTROL_PAD_BUTTON_GUARD) {
				FORCE_WAVEDASH[ENTRY_ID] = true;
			};
			original!()(boma, status_kind, arg3)
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
		} else if [*FIGHTER_STATUS_KIND_ATTACK_S4_START, *FIGHTER_STATUS_KIND_ATTACK_HI4_START, *FIGHTER_STATUS_KIND_ATTACK_LW4_START].contains(&next_status){
			
			//Kills AB Smash
			let specials_list = [*CONTROL_PAD_BUTTON_SPECIAL_RAW, *CONTROL_PAD_BUTTON_SPECIAL_RAW2, *CONTROL_PAD_BUTTON_SPECIAL];
			for i in specials_list {
					if ControlModule::check_button_on(boma, i) {
						println!("Ban AB Smash");
						return 0 as u64
					}
			}
			println!("Keep Smash");
			original!()(boma, status_kind, arg3)
		} else if next_status == *FIGHTER_STATUS_KIND_TURN && curr_status == *FIGHTER_STATUS_KIND_LANDING{
			return 0 as u64
		} else if [*FIGHTER_STATUS_KIND_ESCAPE_AIR, *FIGHTER_STATUS_KIND_ESCAPE_AIR_SLIDE].contains(&next_status) {
			let ENTRY_ID = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize; 
			if IS_WAVEDASH[ENTRY_ID] == true {
				StatusModule::set_situation_kind(boma, smash::app::SituationKind(*SITUATION_KIND_GROUND), true);
			}
			original!()(boma, status_kind, arg3)
		} else if smash::app::utility::get_kind(boma) == *FIGHTER_KIND_WARIO && [*FIGHTER_STATUS_KIND_SPECIAL_S, *FIGHTER_WARIO_STATUS_KIND_SPECIAL_S_BUMP, *FIGHTER_WARIO_STATUS_KIND_SPECIAL_S_DOWN, *FIGHTER_WARIO_STATUS_KIND_SPECIAL_S_RIDE, *FIGHTER_WARIO_STATUS_KIND_SPECIAL_S_DRIVE, *FIGHTER_WARIO_STATUS_KIND_SPECIAL_S_SEARCH, *FIGHTER_WARIO_STATUS_KIND_SPECIAL_S_APPEAL, *FIGHTER_WARIO_STATUS_KIND_SPECIAL_S_ESCAPE, *FIGHTER_WARIO_STATUS_KIND_SPECIAL_S_WHEELIE, *FIGHTER_WARIO_STATUS_KIND_SPECIAL_S_TURN_END, *FIGHTER_WARIO_STATUS_KIND_SPECIAL_S_TURN_LOOP, *FIGHTER_WARIO_STATUS_KIND_SPECIAL_S_THROWN_OUT, *FIGHTER_WARIO_STATUS_KIND_SPECIAL_S_TURN_START, *FIGHTER_WARIO_STATUS_KIND_SPECIAL_S_ESCAPE_START].contains(&status_kind){
			original!()(boma, *FIGHTER_WARIO_STATUS_KIND_SPECIAL_S_START, arg3)
		} else if smash::app::utility::get_kind(boma) == *FIGHTER_KIND_TRAIL && [*FIGHTER_TRAIL_STATUS_KIND_ATTACK_AIR_F].contains(&status_kind){
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
	.on_line(Exec, wavedash)
	.status(Pre, *FIGHTER_STATUS_KIND_ESCAPE_AIR, status_pre_EscapeAir)
    .install();
	skyline::install_hooks!(
		change_status_request_hook,
		change_status_request_script_hook
    );
}