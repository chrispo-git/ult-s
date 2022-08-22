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
static mut STALE_MAX : f32 = 1.0;
static mut STALE_TIMER_MAX : i32 = 480;
static mut FOOTSTOOL_STALE: [f32; 8] = [21.0; 8];
static mut FOOTSTOOL_STALE_TIMER: [i32; 8] = [0; 8];
static mut PERFECT_PIVOT: [bool; 8] = [false; 8];

//Perfect Pivot
#[fighter_frame_callback]
pub fn perfectpivot(fighter : &mut L2CFighterCommon) {
    unsafe {
        let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);  
		let mut stickx = ControlModule::get_stick_x(boma);		
		let status_kind = smash::app::lua_bind::StatusModule::status_kind(boma);
		let lr = PostureModule::lr(boma);
		stickx = stickx * lr;
		let ENTRY_ID = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
        if [*FIGHTER_STATUS_KIND_DASH, *FIGHTER_STATUS_KIND_TURN_DASH].contains(&status_kind) {
			if MotionModule::frame(boma) <= 4.0 {
				CAN_DASH[ENTRY_ID] = 1;
				CAN_TURNDASH[ENTRY_ID] = 1;
				if stickx <= -0.5 {
					StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_TURN, true);
				};
			} else {
				CAN_DASH[ENTRY_ID] = 0;
				CAN_TURNDASH[ENTRY_ID] = 0;
			};
		} else {
			CAN_DASH[ENTRY_ID] = 0;
			CAN_TURNDASH[ENTRY_ID] = 0;
		};
		if status_kind == *FIGHTER_STATUS_KIND_TURN {
			JostleModule::set_status(boma, false);
		};
    };
}

#[fighter_frame_callback]
pub fn footstool(fighter : &mut L2CFighterCommon) {
    unsafe {	
        let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);    
		let status_kind = smash::app::lua_bind::StatusModule::status_kind(boma);
		let fighter_kind = smash::app::utility::get_kind(boma);
		let ENTRY_ID = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
		let lua_state = fighter.lua_state_agent;
		if status_kind == *FIGHTER_STATUS_KIND_TREAD_DAMAGE && MotionModule::frame(boma) < 2.0 {
			if FOOTSTOOL_STALE[ENTRY_ID] > STALE_MAX {
				FOOTSTOOL_STALE[ENTRY_ID] *= 0.75;
				println!("Footstool Stale");
			};
			FOOTSTOOL_STALE_TIMER[ENTRY_ID] = STALE_TIMER_MAX;
		};
		if status_kind == *FIGHTER_STATUS_KIND_TREAD_DAMAGE && FOOTSTOOL_STALE_TIMER[ENTRY_ID] != 0 && FOOTSTOOL_STALE[ENTRY_ID] < 21.0 {
			// This formula is: 1/(staled footstool adv/regular footstool adv)
			FOOTSTOOL_STALE_TIMER[ENTRY_ID] = STALE_TIMER_MAX;
			MotionModule::set_rate(boma, 1.0/(FOOTSTOOL_STALE[ENTRY_ID]/ MotionModule::end_frame(boma))); 
			println!("footstool clause - Rate:{}, End Frame:{}, Frame/Rate:{}", MotionModule::rate(boma), MotionModule::end_frame(boma), (MotionModule::end_frame(boma)/MotionModule::rate(boma)));
		};
		if FOOTSTOOL_STALE_TIMER[ENTRY_ID] > 0 {
			FOOTSTOOL_STALE_TIMER[ENTRY_ID] -= 1;
		};
		if [*FIGHTER_STATUS_KIND_ENTRY, *FIGHTER_STATUS_KIND_WIN].contains(&status_kind) || smash::app::sv_information::is_ready_go() == false {
			FOOTSTOOL_STALE[ENTRY_ID] = 21.0;
			FOOTSTOOL_STALE_TIMER[ENTRY_ID] = 0;
		};
    }
}

//DJC
#[fighter_frame_callback]
pub fn djc(fighter : &mut L2CFighterCommon) {
    unsafe {
        let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);    
		let ENTRY_ID = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
		let fighter_kind = smash::app::utility::get_kind(boma);
		let status_kind = smash::app::lua_bind::StatusModule::status_kind(boma);
		if [*FIGHTER_KIND_NESS, *FIGHTER_KIND_LUCAS, *FIGHTER_KIND_YOSHI, *FIGHTER_KIND_MEWTWO, *FIGHTER_KIND_TRAIL].contains(&fighter_kind) {
			if [*FIGHTER_KINETIC_TYPE_JUMP_AERIAL_MOTION_2ND, *FIGHTER_KINETIC_TYPE_JUMP_AERIAL_MOTION, *FIGHTER_KINETIC_TYPE_JUMP_AERIAL].contains(&KineticModule::get_kinetic_type(boma)) {
				if ControlModule::check_button_off(boma, *CONTROL_PAD_BUTTON_JUMP) && [*FIGHTER_TRAIL_STATUS_KIND_ATTACK_AIR_F, *FIGHTER_TRAIL_STATUS_KIND_ATTACK_AIR_N, *FIGHTER_STATUS_KIND_ATTACK_AIR, *FIGHTER_STATUS_KIND_AIR_LASSO].contains(&status_kind) {
					KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_MOTION_FALL);
				};
				if KineticModule::get_kinetic_type(boma) == *FIGHTER_KINETIC_TYPE_JUMP_AERIAL {
					KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_JUMP_AERIAL_MOTION);
				};
			};
		};
    };
}

//Dash changes
#[fighter_frame_callback]
pub fn dash(fighter : &mut L2CFighterCommon) {
    unsafe {
        let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);  
		let status_kind = smash::app::lua_bind::StatusModule::status_kind(boma);
		if [*FIGHTER_STATUS_KIND_DASH, *FIGHTER_STATUS_KIND_TURN_DASH].contains(&status_kind) && status_duration(boma) >= 7 {
			if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_GUARD) && ControlModule::check_button_off(boma, *CONTROL_PAD_BUTTON_ATTACK) {
					StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_GUARD_ON, true);
			};
		};
		if [*FIGHTER_STATUS_KIND_DASH, *FIGHTER_STATUS_KIND_TURN_DASH, *FIGHTER_STATUS_KIND_RUN, *FIGHTER_STATUS_KIND_RUN_BRAKE, *FIGHTER_STATUS_KIND_TURN_RUN, *FIGHTER_STATUS_KIND_TURN_RUN_BRAKE].contains(&status_kind) {
            if GroundModule::is_passable_ground(fighter.module_accessor) {
                if ControlModule::get_stick_y(boma) <= -0.6875 && ControlModule::get_flick_y(boma) >= 5 && ControlModule::get_flick_y(boma) < 20 {
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_PASS, true);
                };
            }
        }
    };
}
//Parry Cancellable into a dash
#[fighter_frame_callback]
pub fn parrycanceldash(fighter : &mut L2CFighterCommon) {
    unsafe {
        let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);  
		let mut stickx = ControlModule::get_stick_x(boma);		
		let mut lr = PostureModule::lr(boma);
		stickx = stickx * lr;
		let cat1 = ControlModule::get_command_flag_cat(boma, 0);
        if MotionModule::motion_kind(boma) == hash40("just_shield_off") {
			if (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_DASH) != 0 {
				StopModule::end_stop(boma);
				StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_DASH, true);
				//println!("Parry Cancel Dash!");
			};
		};
    };
}

pub fn install() {
    smashline::install_agent_frame_callbacks!(
		perfectpivot,
		parrycanceldash,
		dash,
		djc
	);
	smashline::install_agent_frame_callbacks!(footstool);
}