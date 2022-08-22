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

static mut IS_WAVEDASH: [bool; 8] = [false; 8];
static mut WAVEDASH_DONE: [bool; 8] = [false; 8];

#[fighter_frame_callback]
pub fn wavedash(fighter : &mut L2CFighterCommon) {
    unsafe {
        let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);    
		let fighter_kind = smash::app::utility::get_kind(boma);
		let status_kind = smash::app::lua_bind::StatusModule::status_kind(boma);
		let ENTRY_ID = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
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
				/*if y < 0.3 && (ControlModule::check_button_off(boma, *CONTROL_PAD_BUTTON_JUMP) || ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_JUMP_MINI)){
					let stop_rise  = smash::phx::Vector3f { x: 1.0, y: 0.0, z: 1.0 };
					KineticModule::mul_speed(boma, &stop_rise, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
					let mut z = 0;
					while GroundModule::ray_check(boma, &Vector2f{ x: PostureModule::pos_x(boma), y: PostureModule::pos_y(boma)}, &Vector2f{ x: 0.0, y: -0.25}, true) == 0  && z < 30{
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
				};*/
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
				IS_WAVEDASH[ENTRY_ID] = true;
			};
			if [*FIGHTER_STATUS_KIND_JUMP_SQUAT, *FIGHTER_STATUS_KIND_ESCAPE_AIR, *FIGHTER_STATUS_KIND_ESCAPE_AIR_SLIDE].contains(&status_kind) == false {
				IS_WAVEDASH[ENTRY_ID] = false;
				WAVEDASH_DONE[ENTRY_ID] = false;
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
	let is_clear_buffer = arg3;
	if smash::app::utility::get_category(boma) == *BATTLE_OBJECT_CATEGORY_FIGHTER {
		if [*FIGHTER_STATUS_KIND_ESCAPE, *FIGHTER_STATUS_KIND_ESCAPE_F, *FIGHTER_STATUS_KIND_ESCAPE_B].contains(&next_status) {
			if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_JUMP) || ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_JUMP_MINI) {
				 original!()(boma, *FIGHTER_STATUS_KIND_JUMP_SQUAT, false)
			} else {
				original!()(boma, status_kind, arg3)
			}
		} else if [*FIGHTER_STATUS_KIND_ESCAPE_AIR, *FIGHTER_STATUS_KIND_ESCAPE_AIR_SLIDE].contains(&next_status) {
			let ENTRY_ID = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
			if IS_WAVEDASH[ENTRY_ID] == true {
				StatusModule::set_situation_kind(boma, smash::app::SituationKind(*SITUATION_KIND_GROUND), true);
			}
			original!()(boma, status_kind, arg3)
		} else {
			original!()(boma, status_kind, arg3)
		}
	} else {
		original!()(boma, status_kind, arg3)
	}
}
#[common_status_script(status = FIGHTER_STATUS_KIND_ESCAPE_AIR, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_PRE,
    symbol = "_ZN7lua2cpp16L2CFighterCommon20status_pre_EscapeAirEv")]
pub unsafe fn status_pre_EscapeAir(fighter: &mut L2CFighterCommon) -> L2CValue {
    let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);    
	let ENTRY_ID = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
	let y = ControlModule::get_stick_y(boma);
    //Handles wavedash
    if IS_WAVEDASH[ENTRY_ID] == true && y < 0.5 && (ControlModule::check_button_off(boma, *CONTROL_PAD_BUTTON_JUMP) || ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_JUMP_MINI)){
        GroundModule::attach_ground(fighter.module_accessor, true);
        fighter.change_status(FIGHTER_STATUS_KIND_LANDING.into(), false.into());
        return 0.into();
    }
    call_original!(fighter)
}
#[skyline::hook(replace = smash::app::lua_bind::StatusModule::change_status_request_from_script)]
pub unsafe fn change_status_request_script_hook(boma: &mut smash::app::BattleObjectModuleAccessor, status_kind: i32, arg3: bool) -> u64 {
	let next_status = status_kind;
	let is_clear_buffer = arg3;
	let curr_status = StatusModule::status_kind(boma);
	if smash::app::utility::get_category(boma) == *BATTLE_OBJECT_CATEGORY_FIGHTER {
		if [*FIGHTER_STATUS_KIND_ESCAPE, *FIGHTER_STATUS_KIND_ESCAPE_F, *FIGHTER_STATUS_KIND_ESCAPE_B].contains(&next_status) {
			if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_JUMP) || ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_JUMP_MINI) {
				 original!()(boma, *FIGHTER_STATUS_KIND_JUMP_SQUAT, false)
			} else {
				original!()(boma, status_kind, arg3)
			}
		} else if [*FIGHTER_STATUS_KIND_ESCAPE_AIR, *FIGHTER_STATUS_KIND_ESCAPE_AIR_SLIDE].contains(&next_status) {
			let ENTRY_ID = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
			if IS_WAVEDASH[ENTRY_ID] == true {
				StatusModule::set_situation_kind(boma, smash::app::SituationKind(*SITUATION_KIND_GROUND), true);
			}
			original!()(boma, status_kind, arg3)
		} else {
			original!()(boma, status_kind, arg3)
		}
	} else {
		original!()(boma, status_kind, arg3)
	}
}


pub fn install() {
    smashline::install_agent_frame_callbacks!(
		wavedash
	);
	skyline::install_hooks!(
		change_status_request_hook,
		change_status_request_script_hook
    );
	smashline::install_status_scripts!(status_pre_EscapeAir);
}