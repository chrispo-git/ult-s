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

static mut JUMPSQUAT_SPEED: [f32; 8] = [0.0; 8];
static mut AIR_X: [f32; 8] = [0.0; 8];
static mut FOOTSTOOL_STALE: [f32; 8] = [21.0; 8];
static mut FOOTSTOOL_STALE_TIMER: [i32; 8] = [0; 8];
static mut STALE_MAX : f32 = 1.0;
static mut STALE_TIMER_MAX : i32 = 480;
static mut IS_WAVEDASH: [bool; 8] = [false; 8];
static mut WAVEDASH_DONE: [bool; 8] = [false; 8];
static mut HAS_HIT_IN_ATTACK: [bool; 8] = [false; 8];
static mut ASDI_START: [bool; 8] = [false; 8];
static mut LANDING_COUNTER: [i32; 8] = [0; 8];
static mut ASDI : f32 = 3.0; //15 for testing purposes, should be 3
static mut DIR_MULT : f32 = 57.295776842880464966688235343549; //Very fun number that turns direction that spits out ControlModule::get_stick_dir(boma) as an angle in degrees


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

//Dash Attack Cancel Smashes
#[fighter_frame_callback]
pub fn dacus(fighter : &mut L2CFighterCommon) {
    unsafe {
        let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);    
		let fighter_kind = smash::app::utility::get_kind(boma);
		let status_kind = smash::app::lua_bind::StatusModule::status_kind(boma);
		let f6 = [
			*FIGHTER_KIND_SAMUSD, *FIGHTER_KIND_LUIGI, *FIGHTER_KIND_WARIO, *FIGHTER_KIND_JACK, *FIGHTER_KIND_EDGE,
			*FIGHTER_KIND_LUCAS, *FIGHTER_KIND_SZEROSUIT, *FIGHTER_KIND_PACMAN, *FIGHTER_KIND_SHEIK, *FIGHTER_KIND_MIIFIGHTER
		];
		let f10 = [
			*FIGHTER_KIND_CHROM, *FIGHTER_KIND_CLOUD, *FIGHTER_KIND_LITTLEMAC, *FIGHTER_KIND_TRAIL, *FIGHTER_KIND_REFLET,
			*FIGHTER_KIND_FOX, *FIGHTER_KIND_SHIZUE, *FIGHTER_KIND_ELIGHT, *FIGHTER_KIND_PITB, *FIGHTER_KIND_MARIO,
			*FIGHTER_KIND_MARIOD, *FIGHTER_KIND_MURABITO, *FIGHTER_KIND_METAKNIGHT
		];
		let f12 = [
			*FIGHTER_KIND_KOOPAJR, *FIGHTER_KIND_KOOPA, *FIGHTER_KIND_CAPTAIN, *FIGHTER_KIND_KAMUI, *FIGHTER_KIND_DIDDY, 
			*FIGHTER_KIND_GAOGAEN, *FIGHTER_KIND_DONKEY, *FIGHTER_KIND_DUCKHUNT, *FIGHTER_KIND_MASTER, *FIGHTER_KIND_YOSHI,
			*FIGHTER_KIND_SIMON, *FIGHTER_KIND_ROY, *FIGHTER_KIND_GEKKOUGA, *FIGHTER_KIND_ROCKMAN
		];
		let f14 = [
			*FIGHTER_KIND_BAYONETTA, *FIGHTER_KIND_MARTH, *FIGHTER_KIND_NESS, *FIGHTER_KIND_DEDEDE, 
			*FIGHTER_KIND_BRAVE, *FIGHTER_KIND_DEMON, *FIGHTER_KIND_LUCINA,
			*FIGHTER_KIND_DEDEDE
		];
		let f16 = [
			*FIGHTER_KIND_IKE, *FIGHTER_KIND_SHULK, *FIGHTER_KIND_EFLAME
		];
        if status_kind == *FIGHTER_STATUS_KIND_ATTACK_DASH && AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_SHIELD) == false{
				if ((f16.contains(&fighter_kind) && motion_duration(boma) <= 16) || 
				(f14.contains(&fighter_kind) && motion_duration(boma) <= 14) || 
				(f12.contains(&fighter_kind) && motion_duration(boma) <= 12) ||
				(f10.contains(&fighter_kind) && motion_duration(boma) <= 10) ||
				(f6.contains(&fighter_kind) && motion_duration(boma) <= 6) ||
				(f6.contains(&fighter_kind) == false && motion_duration(boma) <= 8)){
					if (ControlModule::get_command_flag_cat(boma, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_HI4) != 0 {
						StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_HI4_START, true);
					} else if (ControlModule::get_command_flag_cat(boma, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_LW4) != 0 {
						StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_LW4_START, true);
					};
				};
        };
    };
}
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
				if y < 0.3 && (ControlModule::check_button_off(boma, *CONTROL_PAD_BUTTON_JUMP) || ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_JUMP_MINI)){
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
				};
				IS_WAVEDASH[ENTRY_ID] = false;
			};
			if WAVEDASH_DONE[ENTRY_ID] == true && status_kind == *FIGHTER_STATUS_KIND_LANDING{
				let stop_rise  = smash::phx::Vector3f { x: 0.0, y: 1.0, z: 1.0 };
				KineticModule::mul_speed(boma, &stop_rise, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
				let speed = smash::phx::Vector3f { x: 0.1, y: 0.0, z: 0.0 };
				KineticModule::add_speed(boma, &speed);
				WAVEDASH_DONE[ENTRY_ID] = false;
			};
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

//Jab Cancel
#[fighter_frame_callback]
pub fn jabcancel(fighter : &mut L2CFighterCommon) {
    unsafe {
        let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);    
		let fighter_kind = smash::app::utility::get_kind(boma);
		let status_kind = smash::app::lua_bind::StatusModule::status_kind(boma);
		let motion_kind = MotionModule::motion_kind(boma);
		let ENTRY_ID = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
		let stick_x = ControlModule::get_stick_x(boma);
		let stick_y = ControlModule::get_stick_y(boma);
		let frame = MotionModule::frame(boma);
		//Speeds up jab 2s and 3s for certain chars
		if [*FIGHTER_KIND_DOLLY, *FIGHTER_KIND_YOUNGLINK].contains(&fighter_kind) {
			if [hash40("attack_12")].contains(&motion_kind) && frame < 2.0 {
					MotionModule::change_motion(boma, Hash40::new("attack_12"), 2.0, 1.0, false, 0.0, false, false);
			};
			if [hash40("attack_13")].contains(&motion_kind) && frame < 2.0 {
					MotionModule::change_motion(boma, Hash40::new("attack_13"), 2.0, 1.0, false, 0.0, false, false);
			};
		};
		//Prevents jab overriding
		if [*FIGHTER_STATUS_KIND_ATTACK_100, *FIGHTER_STATUS_KIND_ATTACK, *FIGHTER_DEMON_STATUS_KIND_ATTACK_COMBO].contains(&status_kind) && ![*FIGHTER_KIND_MURABITO].contains(&fighter_kind){
			if ((stick_x <= 0.2 && stick_x >= -0.2) && (stick_y <= 0.2 && stick_y >= -0.2)) && ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_ATTACK) && ControlModule::check_button_off(boma, *CONTROL_PAD_BUTTON_CATCH) && 
			(
				(ControlModule::get_command_flag_cat(boma, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_S4) == 0 &&
				(ControlModule::get_command_flag_cat(boma, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_HI4) == 0 &&
				(ControlModule::get_command_flag_cat(boma, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_LW4) == 0 &&
				(ControlModule::get_command_flag_cat(boma, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_S3) == 0 &&
				(ControlModule::get_command_flag_cat(boma, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_HI3) == 0 &&
				(ControlModule::get_command_flag_cat(boma, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_LW3) == 0 &&
				(ControlModule::get_command_flag_cat(boma, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_DASH) == 0 &&
				ControlModule::check_button_off(boma, *CONTROL_PAD_BUTTON_JUMP)
			){
				CAN_JAB[ENTRY_ID] = 0;
				CAN_RAPID_JAB[ENTRY_ID] = 0;
				if HAS_ENABLE_100_ON[ENTRY_ID] {
					WorkModule::set_flag(boma, true, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_100);
				};
				if HAS_ENABLE_COMBO_ON[ENTRY_ID] {
					WorkModule::set_flag(boma, true, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO);
				};
				if HAS_ENABLE_NO_HIT_COMBO_ON[ENTRY_ID] {
					WorkModule::set_flag(boma, true, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO);
				};
			} else {
				CAN_JAB[ENTRY_ID] = 1;
				CAN_RAPID_JAB[ENTRY_ID] = 1;
				if HAS_ENABLE_100_ON[ENTRY_ID] {
					WorkModule::set_flag(boma, false, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_100);
				};
				if HAS_ENABLE_COMBO_ON[ENTRY_ID] {
					WorkModule::set_flag(boma, false, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO);
				};
				if HAS_ENABLE_NO_HIT_COMBO_ON[ENTRY_ID] {
					WorkModule::set_flag(boma, false, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO);
				};
			};
		} else {
			CAN_JAB[ENTRY_ID] = 0;
			CAN_RAPID_JAB[ENTRY_ID] = 0;
		};
		if [*FIGHTER_STATUS_KIND_ATTACK_100, *FIGHTER_STATUS_KIND_ATTACK, *FIGHTER_DEMON_STATUS_KIND_ATTACK_COMBO].contains(&status_kind) && !is_hitlag(boma) && (HAS_ENABLE_100_ON[ENTRY_ID] || HAS_ENABLE_COMBO_ON[ENTRY_ID]){
				if  (AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT) || (fighter_kind == *FIGHTER_KIND_SHEIK && AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_ALL))) {
						if (ControlModule::get_command_flag_cat(boma, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_S4) != 0 {
							StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_S4_START, true);
						} else if (ControlModule::get_command_flag_cat(boma, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_HI4) != 0 {
							StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_HI4_START, true);
						} else if (ControlModule::get_command_flag_cat(boma, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_LW4) != 0 {
							StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_LW4_START, true);
						} else if (ControlModule::get_command_flag_cat(boma, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_S3) != 0 {
							StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_S3, true);
						} else if (ControlModule::get_command_flag_cat(boma, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_HI3) != 0 {
							StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_HI3, true);
						} else if (ControlModule::get_command_flag_cat(boma, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_LW3) != 0 {
							StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_LW3, true);
						};
				};
		};
    };
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

//Perfect Pivot
#[fighter_frame_callback]
pub fn perfectpivot(fighter : &mut L2CFighterCommon) {
    unsafe {
        let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);  
		let mut stickx = ControlModule::get_stick_x(boma);		
		let status_kind = smash::app::lua_bind::StatusModule::status_kind(boma);
		let lr = PostureModule::lr(boma);
		stickx = stickx * lr;
        if status_kind == *FIGHTER_STATUS_KIND_DASH && status_duration(boma) <= 8 {
			if stickx < -0.5 {
				StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_TURN, true);
				//println!("Perfect pivot lets go!");
			};
		};
		if status_kind == *FIGHTER_STATUS_KIND_TURN {
			JostleModule::set_status(boma, false);
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
//Landing Lag Platform Cancel
#[fighter_frame_callback]
pub fn llpc(fighter : &mut L2CFighterCommon) {
    unsafe {
        let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);  
		let status_kind = smash::app::lua_bind::StatusModule::status_kind(boma);
		let sticky = ControlModule::get_stick_y(boma);	
		let cancel_frame = FighterMotionModuleImpl::get_cancel_frame(boma,smash::phx::Hash40::new_raw(MotionModule::motion_kind(boma)),false) as f32;
		let frame = MotionModule::frame(boma);
		let situation_kind = StatusModule::situation_kind(boma);
        if ([hash40("landing_heavy"), hash40("landing_air_f"), hash40("landing_air_b"), hash40("landing_air_hi"), hash40("landing_air_n")].contains(&MotionModule::motion_kind(boma))) {
			if GroundModule::is_passable_ground(fighter.module_accessor) && frame/cancel_frame >= (1.0/6.0){
                if sticky <= -0.6875 && ((ControlModule::get_flick_y(boma) >= 3 && ControlModule::get_flick_y(boma) < 20) || sticky <= -1.0) {
					if (
						(ControlModule::get_command_flag_cat(boma, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_LW4) == 0 &&
						(ControlModule::get_command_flag_cat(boma, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_LW3) == 0 &&
						(ControlModule::get_command_flag_cat(boma, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_LW) == 0 &&
						(ControlModule::get_command_flag_cat(boma, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_ESCAPE) == 0
					) {
						StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_PASS, true);
					};
                };
            }
		};
    };
}	
pub(crate) unsafe fn bone_const(boma: &mut smash::app::BattleObjectModuleAccessor, new_fighter_kind : i32, new_motion_kind : u64, bone : u64,
	start_frame : f32, end_frame : f32, 
	x_rotate : f32, x_rotate_end : f32, y_rotate : f32, y_rotate_end : f32,  z_rotate : f32, z_rotate_end : f32
) -> () {
	let fighter_kind = smash::app::utility::get_kind(boma);
	let motion_kind = MotionModule::motion_kind(boma);
	let frame = MotionModule::frame(boma);
	let duration = end_frame-start_frame;
	if fighter_kind == new_fighter_kind && motion_kind == new_motion_kind && frame >= start_frame && frame <= end_frame {
		let mut rotation = Vector3f{x: x_rotate + (((x_rotate_end-x_rotate)/duration)*(frame-start_frame)), y: y_rotate + (((y_rotate_end-y_rotate)/duration)*(frame-start_frame)) , z: z_rotate + (((z_rotate_end-z_rotate)/duration)*(frame-start_frame))};
	    ModelModule::set_joint_rotate(boma, Hash40::new_raw(bone), &rotation,  smash::app::MotionNodeRotateCompose{_address: *MOTION_NODE_ROTATE_COMPOSE_AFTER as u8},  smash::app::MotionNodeRotateOrder{_address: *MOTION_NODE_ROTATE_ORDER_XYZ as u8});
	};
}
pub(crate) unsafe fn bone_inst(boma: &mut smash::app::BattleObjectModuleAccessor, bone : u64,
	x_rotate : f32, y_rotate : f32, z_rotate : f32
) -> () {
	let fighter_kind = smash::app::utility::get_kind(boma);
	let motion_kind = MotionModule::motion_kind(boma);
	let frame = MotionModule::frame(boma);
	let mut rotation = Vector3f{x: x_rotate, y: y_rotate , z: z_rotate };
	ModelModule::set_joint_rotate(boma, Hash40::new_raw(bone), &rotation,  smash::app::MotionNodeRotateCompose{_address: *MOTION_NODE_ROTATE_COMPOSE_AFTER as u8},  smash::app::MotionNodeRotateOrder{_address: *MOTION_NODE_ROTATE_ORDER_XYZ as u8});
}
#[fighter_frame_callback]
pub fn bone_rot(fighter : &mut L2CFighterCommon) {
    unsafe {
        let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent); 
		let status_kind = smash::app::lua_bind::StatusModule::status_kind(boma);
		let ENTRY_ID = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
		let fighter_kind = smash::app::utility::get_kind(boma);
		let motion_kind = MotionModule::motion_kind(boma);
		let stick_x = ControlModule::get_stick_x(boma) * PostureModule::lr(boma);
		let stick_y = ControlModule::get_stick_y(boma);
		//Example: Diddy Kong Fair Rotation being angled
		//bone_const(boma, *FIGHTER_KIND_DIDDY, hash40("attack_air_f"), hash40("rot"), 0.0, 999.0, -45.0, 0.0, 0.0, 0.0, 0.0, 0.0);
		
		//Dark Samus Fair angle down
		bone_const(boma, *FIGHTER_KIND_SAMUSD, hash40("attack_air_f"), hash40("rot"), 0.0, 47.0, 22.5, 22.5, 0.0, 0.0, 0.0, 0.0);
		bone_const(boma, *FIGHTER_KIND_SAMUSD, hash40("attack_air_f"), hash40("rot"), 47.0, 59.0, 22.5, 0.0, 0.0, 0.0, 0.0, 0.0);
		
		//Falcon nair angle down
		bone_const(boma, *FIGHTER_KIND_CAPTAIN, hash40("attack_air_n"), hash40("rot"), 0.0, 24.0, 10.0, 10.0, 0.0, 0.0, 0.0, 0.0);
		bone_const(boma, *FIGHTER_KIND_CAPTAIN, hash40("attack_air_n"), hash40("rot"), 24.0, 38.0, 10.0, 0.0, 0.0, 0.0, 0.0, 0.0);
		
		//Roy Fair angle down
		bone_const(boma, *FIGHTER_KIND_ROY, hash40("attack_air_f"), hash40("rot"), 0.0, 12.0, 0.0, -8.0, 0.0, 0.0, 0.0, 0.0);
		bone_const(boma, *FIGHTER_KIND_ROY, hash40("attack_air_f"), hash40("rot"), 12.0, 16.0, -8.0, 5.0, 0.0, 0.0, 0.0, 0.0);
		bone_const(boma, *FIGHTER_KIND_ROY, hash40("attack_air_f"), hash40("rot"), 16.0, 31.0, 5.0, 0.0, 0.0, 0.0, 0.0, 0.0);
		
		//Roy Dair angle side
		bone_const(boma, *FIGHTER_KIND_ROY, hash40("attack_air_lw"), hash40("rot"), 0.0, 37.0, 0.0, 0.0, 45.0, 45.0, 0.0, 0.0);
		bone_const(boma, *FIGHTER_KIND_ROY, hash40("attack_air_lw"), hash40("rot"), 37.0, 62.0, 0.0, 0.0, 45.0, 0.0, 0.0, 0.0);
		
		//Villy fair foot angle down
		bone_const(boma, *FIGHTER_KIND_MURABITO, hash40("attack_air_f"), hash40("footl"), 0.0, 44.0, 35.0, 35.0, 0.0, 0.0, 0.0, 0.0);
		
		//Byleth feet fix
		bone_const(boma, *FIGHTER_KIND_MASTER, hash40("special_air_s_front"), hash40("footl"), 0.0, 180.0, 0.0, 0.0, 0.0, 0.0, 55.0, 55.0);
		bone_const(boma, *FIGHTER_KIND_MASTER, hash40("special_air_s_front"), hash40("footr"), 0.0, 180.0, 0.0, 0.0, 0.0, 0.0, 55.0, 55.0);
		bone_const(boma, *FIGHTER_KIND_MASTER, hash40("special_air_s_front_dash"), hash40("footl"), 0.0, 180.0, 0.0, 0.0, 0.0, 0.0, 55.0, 55.0);
		bone_const(boma, *FIGHTER_KIND_MASTER, hash40("special_air_s_front_dash"), hash40("footr"), 0.0, 180.0, 0.0, 0.0, 0.0, 0.0, 55.0, 55.0);
		
		//Falco 
		bone_const(boma, *FIGHTER_KIND_FALCO, hash40("throw_hi"), hash40("handr"), 13.0, 37.0, 0.0, 0.0, stick_x*20.0 -15.0, stick_x*20.0 -15.0, 0.0, 0.0);
		//Gun
		bone_const(boma, *FIGHTER_KIND_MIIGUNNER, hash40("throw_hi"), hash40("handr"), 10.0, 37.0, 0.0, 0.0, stick_x*20.0 -15.0, stick_x*20.0 -15.0, 0.0, 0.0);
	};
}
#[fighter_frame_callback]
pub fn melee_hat(fighter : &mut L2CFighterCommon) {
    unsafe {
        let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent); 
		let status_kind = smash::app::lua_bind::StatusModule::status_kind(boma);
		let ENTRY_ID = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
		let fighter_kind = smash::app::utility::get_kind(boma); 
		let stickx = ControlModule::get_stick_x(boma);		 
		let sticky = ControlModule::get_stick_y(boma);	
		let substickx = ControlModule::get_sub_stick_x(boma);		 
		let substicky = ControlModule::get_sub_stick_y(boma);	
		let remaining_hitstun = WorkModule::get_float(boma, *FIGHTER_INSTANCE_WORK_ID_FLOAT_DAMAGE_REACTION_FRAME);
		let total_hitstun = WorkModule::get_float(boma, *FIGHTER_INSTANCE_WORK_ID_FLOAT_DAMAGE_REACTION_FRAME_LAST);		
		let cancel_frame = FighterMotionModuleImpl::get_cancel_frame(boma,smash::phx::Hash40::new_raw(MotionModule::motion_kind(boma)),false) as f32;
		let end_frame = MotionModule::end_frame(boma);
		if ItemModule::is_attach_item(boma, smash::app::ItemKind(*ITEM_KIND_USAGIHAT)) {
			//Removes ground to air limit
			if ![
				*FIGHTER_GAOGAEN_STATUS_KIND_SPECIAL_HI_END, *FIGHTER_GAOGAEN_STATUS_KIND_SPECIAL_HI_LOOP, *FIGHTER_GAOGAEN_STATUS_KIND_SPECIAL_HI_FALL
			].contains(&status_kind) {
				if WorkModule::is_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_JUMP_NO_LIMIT) == false{
					WorkModule::on_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_JUMP_NO_LIMIT);
				};
			}else {
				WorkModule::off_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_JUMP_NO_LIMIT);
			};
			//JC Grab
			if status_kind == *FIGHTER_STATUS_KIND_JUMP_SQUAT && ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_CATCH) {
				StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_CATCH, true);
			};
			//Shield Drop
			if [*FIGHTER_STATUS_KIND_GUARD, *FIGHTER_STATUS_KIND_GUARD_ON].contains(&status_kind) {
				if GroundModule::is_passable_ground(fighter.module_accessor) {
					if ControlModule::get_stick_y(boma) <= -0.6875 {
						StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_PASS, true);
					};
				}
			};
			//Ledge Snap stuff
			if get_speed_y(boma) > 0.2 {
				CAN_CLIFF[ENTRY_ID] = 1;
			} else {
				CAN_CLIFF[ENTRY_ID] = 0;
			};
			//Disable Airdodge/Attack out of hitstun
			if remaining_hitstun > 0.0 && [*FIGHTER_STATUS_KIND_DAMAGE_AIR, *FIGHTER_STATUS_KIND_DAMAGE_FALL, *FIGHTER_STATUS_KIND_DAMAGE_FLY, *FIGHTER_STATUS_KIND_DAMAGE_FLY_ROLL, *FIGHTER_STATUS_KIND_DAMAGE_FLY_METEOR].contains(&status_kind) {
				CAN_AIRDODGE[ENTRY_ID] = 1;
				CAN_ATTACK_AIR[ENTRY_ID] = 1;
			} else {
				CAN_AIRDODGE[ENTRY_ID] = 0;
				CAN_ATTACK_AIR[ENTRY_ID] = 0;
			};
			//ASDI
			if StopModule::is_damage(boma) {
				ASDI_START[ENTRY_ID] = true;
			};
			if status_kind == *FIGHTER_STATUS_KIND_LANDING {
				if remaining_hitstun > 0.0 {
					LANDING_COUNTER[ENTRY_ID] +=1 ;
					if LANDING_COUNTER[ENTRY_ID] >= 4 {
						CancelModule::enable_cancel(boma);
					};
				};
			} else {
				LANDING_COUNTER[ENTRY_ID] = 0;
			};
			if status_kind == *FIGHTER_STATUS_KIND_ESCAPE_AIR_SLIDE {
				if MotionModule::frame(boma) > 27.0 && MotionModule::frame(boma) < 42.0{
					KineticModule::clear_speed_all(boma);
				};
				if MotionModule::frame(boma) > 3.0 && MotionModule::frame(boma) < 30.0 {
					HitModule::set_whole(boma, smash::app::HitStatus(*HIT_STATUS_XLU), 0);
				} else {
					HitModule::set_whole(boma, smash::app::HitStatus(*HIT_STATUS_NORMAL), 0);
				};
				MotionModule::set_rate(boma, 1.0);
			}else if status_kind == *FIGHTER_STATUS_KIND_ESCAPE_AIR {
				if MotionModule::frame(boma) > 8.0 && MotionModule::frame(boma) < 42.0{
					KineticModule::clear_speed_all(boma);
				};
				if MotionModule::frame(boma) > 3.0 && MotionModule::frame(boma) < 30.0 {
					HitModule::set_whole(boma, smash::app::HitStatus(*HIT_STATUS_XLU), 0);
				} else {
					HitModule::set_whole(boma, smash::app::HitStatus(*HIT_STATUS_NORMAL), 0);
				};
				MotionModule::set_rate(boma, 1.0);
			};
			if ASDI_START[ENTRY_ID] && !StopModule::is_damage(boma){
				let mut stick_dir = ControlModule::get_stick_dir(boma) * DIR_MULT;
				let sub_stick_dir = ControlModule::get_sub_stick_dir(boma) * DIR_MULT;
				let mut hypotenuse = ASDI;
				if stickx < 0.2 && stickx > -0.2 && sticky > -0.2 && sticky < 0.2 {
					hypotenuse = 0.001;
				};
				if (substickx < 0.2 && substickx > -0.2 && substicky > -0.2 && substicky < 0.2) == false {
					stick_dir = sub_stick_dir;
				};
				let mut stick_lr = 1.0;
				if stickx < 0.0 {
					stick_lr = -1.0;
				};
				let mut stick_ud = 1.0;
				if sticky < 0.0 {
					stick_ud = -1.0;
				};
				let x_distance = stick_dir.cos()*hypotenuse*stick_lr;
				let y_distance = stick_dir.sin()*hypotenuse*stick_ud;
				//println!("Hitstop {}, Stick_Dir {}, X {}, Y {}", StopModule::get_hit_stop_real_frame(boma), stick_dir, x_distance, y_distance);
				if ray_check_pos(boma, x_distance, y_distance, false) == 0 {
					let pos = smash::phx::Vector3f { x: PostureModule::pos_x(boma)+x_distance, y: PostureModule::pos_y(boma)+y_distance, z: 0.0 };
					PostureModule::set_pos(boma, &pos);
					PostureModule::init_pos(boma, &pos, true, true);
				} else if ray_check_pos(boma, x_distance/2.0, y_distance/2.0, false) == 0{
					let pos = smash::phx::Vector3f { x: PostureModule::pos_x(boma)+x_distance*0.5, y: PostureModule::pos_y(boma)+y_distance*0.5, z: 0.0 };
					PostureModule::set_pos(boma, &pos);
					PostureModule::init_pos(boma, &pos, true, true);
				};
				ASDI_START[ENTRY_ID] = false;
			};
		} else {
			if WorkModule::is_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_JUMP_NO_LIMIT) != false {
				WorkModule::off_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_JUMP_NO_LIMIT);
			};
			LANDING_COUNTER[ENTRY_ID] = 0;
		};
	};
}	
//Edge Cancel List
pub(crate) fn is_edge_cancel(fighter_kind : i32, status_kind : i32) -> bool {
	let edge_cancel = [
		[*FIGHTER_KIND_LUCARIO, *FIGHTER_STATUS_KIND_ATTACK_DASH],
		[*FIGHTER_KIND_DIDDY, *FIGHTER_STATUS_KIND_ATTACK_DASH],
		[*FIGHTER_KIND_BUDDY, *FIGHTER_STATUS_KIND_ATTACK_DASH],
		[*FIGHTER_KIND_LITTLEMAC, *FIGHTER_STATUS_KIND_ATTACK_DASH],
		[*FIGHTER_KIND_KAMUI, *FIGHTER_STATUS_KIND_ATTACK_DASH],
		[*FIGHTER_KIND_PURIN, *FIGHTER_STATUS_KIND_ATTACK_DASH],
		[*FIGHTER_KIND_CAPTAIN, *FIGHTER_STATUS_KIND_ATTACK_DASH],
		[*FIGHTER_KIND_RIDLEY, *FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL],
		[*FIGHTER_KIND_RIDLEY, *FIGHTER_STATUS_KIND_ATTACK_DASH],
		[*FIGHTER_KIND_RICHTER, *FIGHTER_STATUS_KIND_ATTACK_LW3]
	];
	for i in &edge_cancel {
		if fighter_kind == i[0] && status_kind == i[1] {
			return true;
		};
	};
	return false;
}

#[fighter_frame_callback]
pub fn sword_size(fighter : &mut L2CFighterCommon) {
    unsafe {
        let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent); 
		let status_kind = smash::app::lua_bind::StatusModule::status_kind(boma);
		let fighter_kind = smash::app::utility::get_kind(boma);
		let lua_state = fighter.lua_state_agent;
		let ENTRY_ID = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
		if [*FIGHTER_KIND_TOONLINK].contains(&fighter_kind) {
			let joint_scale = smash::phx::Vector3f { x: 1.12, y: 1.12, z: 1.12 };
			ModelModule::set_joint_scale(boma, smash::phx::Hash40::new("havel"), &joint_scale);
		};
		if [*FIGHTER_KIND_DEDEDE].contains(&fighter_kind) {
			let max_size = 1.75;
			if MotionModule::motion_kind(boma) == hash40("attack_air_b") {
				if MotionModule::frame(boma) < 5.0 {
					let mut mul = 1.0;
					mul += (MotionModule::frame(boma) * (max_size/5.0));
					let joint_scale = smash::phx::Vector3f { x: mul, y: mul, z: mul };
					ModelModule::set_joint_scale(boma, smash::phx::Hash40::new("footr"), &joint_scale);
				} else if 4.0 < MotionModule::frame(boma) && 13.0 >= MotionModule::frame(boma) {
					let mut mul = max_size;
					let joint_scale = smash::phx::Vector3f { x: mul, y: mul, z: mul };
					ModelModule::set_joint_scale(boma, smash::phx::Hash40::new("footr"), &joint_scale);
				} else if 13.0 < MotionModule::frame(boma){
					if MotionModule::frame(boma) < 15.0 {
						let mut mul = 1.65;
						let joint_scale = smash::phx::Vector3f { x: mul, y: mul, z: mul };
						ModelModule::set_joint_scale(boma, smash::phx::Hash40::new("footr"), &joint_scale);
					}else if MotionModule::frame(boma)  < 16.0  {
						let mut mul = 1.55;
						let joint_scale = smash::phx::Vector3f { x: mul, y: mul, z: mul };
						ModelModule::set_joint_scale(boma, smash::phx::Hash40::new("footr"), &joint_scale);
					}else if MotionModule::frame(boma) < 17.0  {
						let mut mul = 1.45;
						let joint_scale = smash::phx::Vector3f { x: mul, y: mul, z: mul };
						ModelModule::set_joint_scale(boma, smash::phx::Hash40::new("footr"), &joint_scale);
					}else if MotionModule::frame(boma)  < 18.0 {
						let mut mul = 1.35;
						let joint_scale = smash::phx::Vector3f { x: mul, y: mul, z: mul };
						ModelModule::set_joint_scale(boma, smash::phx::Hash40::new("footr"), &joint_scale);
					}else if MotionModule::frame(boma)  < 19.0  {
						let mut mul = 1.25;
						let joint_scale = smash::phx::Vector3f { x: mul, y: mul, z: mul };
						ModelModule::set_joint_scale(boma, smash::phx::Hash40::new("footr"), &joint_scale);
					}else if MotionModule::frame(boma)  < 20.0  {
						let mut mul = 1.15;
						let joint_scale = smash::phx::Vector3f { x: mul, y: mul, z: mul };
						ModelModule::set_joint_scale(boma, smash::phx::Hash40::new("footr"), &joint_scale);
					}else {
						let mut mul = 1.0;
						let joint_scale = smash::phx::Vector3f { x: mul, y: mul, z: mul };
						ModelModule::set_joint_scale(boma, smash::phx::Hash40::new("footr"), &joint_scale);
					};
				};
			};
		};
		if [*FIGHTER_KIND_PIKMIN].contains(&fighter_kind) {
			let max_size = 3.5;
			if MotionModule::motion_kind(boma) == hash40("attack_s4_s") {
				if 9.0 < MotionModule::frame(boma) && 13.0 >= MotionModule::frame(boma) {
					let mut mul = max_size;
					let joint_scale = smash::phx::Vector3f { x: mul, y: mul, z: mul };
					ModelModule::set_joint_scale(boma, smash::phx::Hash40::new("handl"), &joint_scale);
				} else if 13.0 < MotionModule::frame(boma){
					if MotionModule::frame(boma) < 15.0 {
						let mut mul = 3.1;
						let joint_scale = smash::phx::Vector3f { x: mul, y: mul, z: mul };
						ModelModule::set_joint_scale(boma, smash::phx::Hash40::new("handl"), &joint_scale);
					}else if MotionModule::frame(boma)  < 16.0  {
						let mut mul = 2.7;
						let joint_scale = smash::phx::Vector3f { x: mul, y: mul, z: mul };
						ModelModule::set_joint_scale(boma, smash::phx::Hash40::new("handl"), &joint_scale);
					}else if MotionModule::frame(boma) < 17.0  {
						let mut mul = 2.3;
						let joint_scale = smash::phx::Vector3f { x: mul, y: mul, z: mul };
						ModelModule::set_joint_scale(boma, smash::phx::Hash40::new("handl"), &joint_scale);
					}else if MotionModule::frame(boma)  < 18.0 {
						let mut mul = 1.9;
						let joint_scale = smash::phx::Vector3f { x: mul, y: mul, z: mul };
						ModelModule::set_joint_scale(boma, smash::phx::Hash40::new("handl"), &joint_scale);
					}else if MotionModule::frame(boma)  < 19.0  {
						let mut mul = 1.5;
						let joint_scale = smash::phx::Vector3f { x: mul, y: mul, z: mul };
						ModelModule::set_joint_scale(boma, smash::phx::Hash40::new("handl"), &joint_scale);
					}else if MotionModule::frame(boma)  < 20.0  {
						let mut mul = 1.1;
						let joint_scale = smash::phx::Vector3f { x: mul, y: mul, z: mul };
						ModelModule::set_joint_scale(boma, smash::phx::Hash40::new("handl"), &joint_scale);
					}else {
						let mut mul = 1.0;
						let joint_scale = smash::phx::Vector3f { x: mul, y: mul, z: mul };
						ModelModule::set_joint_scale(boma, smash::phx::Hash40::new("handl"), &joint_scale);
					};
				};
			};
			if MotionModule::motion_kind(boma) == hash40("attack_hi4") {
				if 9.0 < MotionModule::frame(boma) && 13.0 >= MotionModule::frame(boma) {
					let mut mul = max_size;
					let joint_scale = smash::phx::Vector3f { x: mul, y: mul, z: mul };
					ModelModule::set_joint_scale(boma, smash::phx::Hash40::new("handl"), &joint_scale);
				} else if 13.0 < MotionModule::frame(boma){
					if MotionModule::frame(boma) < 15.0 {
						let mut mul = 3.1;
						let joint_scale = smash::phx::Vector3f { x: mul, y: mul, z: mul };
						ModelModule::set_joint_scale(boma, smash::phx::Hash40::new("handl"), &joint_scale);
					}else if MotionModule::frame(boma)  < 16.0  {
						let mut mul = 2.7;
						let joint_scale = smash::phx::Vector3f { x: mul, y: mul, z: mul };
						ModelModule::set_joint_scale(boma, smash::phx::Hash40::new("handl"), &joint_scale);
					}else if MotionModule::frame(boma) < 17.0  {
						let mut mul = 2.3;
						let joint_scale = smash::phx::Vector3f { x: mul, y: mul, z: mul };
						ModelModule::set_joint_scale(boma, smash::phx::Hash40::new("handl"), &joint_scale);
					}else if MotionModule::frame(boma)  < 18.0 {
						let mut mul = 1.9;
						let joint_scale = smash::phx::Vector3f { x: mul, y: mul, z: mul };
						ModelModule::set_joint_scale(boma, smash::phx::Hash40::new("handl"), &joint_scale);
					}else if MotionModule::frame(boma)  < 19.0  {
						let mut mul = 1.5;
						let joint_scale = smash::phx::Vector3f { x: mul, y: mul, z: mul };
						ModelModule::set_joint_scale(boma, smash::phx::Hash40::new("handl"), &joint_scale);
					}else if MotionModule::frame(boma)  < 20.0  {
						let mut mul = 1.1;
						let joint_scale = smash::phx::Vector3f { x: mul, y: mul, z: mul };
						ModelModule::set_joint_scale(boma, smash::phx::Hash40::new("handl"), &joint_scale);
					}else {
						let mut mul = 1.0;
						let joint_scale = smash::phx::Vector3f { x: mul, y: mul, z: mul };
						ModelModule::set_joint_scale(boma, smash::phx::Hash40::new("handl"), &joint_scale);
					};
				};
			};
			if MotionModule::motion_kind(boma) == hash40("attack_lw4") {
				if 9.0 < MotionModule::frame(boma) && 13.0 >= MotionModule::frame(boma) {
					let mut mul = max_size;
					let joint_scale = smash::phx::Vector3f { x: mul, y: mul, z: mul };
					ModelModule::set_joint_scale(boma, smash::phx::Hash40::new("handl"), &joint_scale);
					ModelModule::set_joint_scale(boma, smash::phx::Hash40::new("handr"), &joint_scale);
				} else if 13.0 < MotionModule::frame(boma){
					if MotionModule::frame(boma) < 15.0 {
						let mut mul = 3.1;
						let joint_scale = smash::phx::Vector3f { x: mul, y: mul, z: mul };
						ModelModule::set_joint_scale(boma, smash::phx::Hash40::new("handl"), &joint_scale);
						ModelModule::set_joint_scale(boma, smash::phx::Hash40::new("handr"), &joint_scale);
					}else if MotionModule::frame(boma)  < 16.0  {
						let mut mul = 2.7;
						let joint_scale = smash::phx::Vector3f { x: mul, y: mul, z: mul };
						ModelModule::set_joint_scale(boma, smash::phx::Hash40::new("handl"), &joint_scale);
						ModelModule::set_joint_scale(boma, smash::phx::Hash40::new("handr"), &joint_scale);
					}else if MotionModule::frame(boma) < 17.0  {
						let mut mul = 2.3;
						let joint_scale = smash::phx::Vector3f { x: mul, y: mul, z: mul };
						ModelModule::set_joint_scale(boma, smash::phx::Hash40::new("handl"), &joint_scale);
						ModelModule::set_joint_scale(boma, smash::phx::Hash40::new("handr"), &joint_scale);
					}else if MotionModule::frame(boma)  < 18.0 {
						let mut mul = 1.9;
						let joint_scale = smash::phx::Vector3f { x: mul, y: mul, z: mul };
						ModelModule::set_joint_scale(boma, smash::phx::Hash40::new("handl"), &joint_scale);
						ModelModule::set_joint_scale(boma, smash::phx::Hash40::new("handr"), &joint_scale);
					}else if MotionModule::frame(boma)  < 19.0  {
						let mut mul = 1.5;
						let joint_scale = smash::phx::Vector3f { x: mul, y: mul, z: mul };
						ModelModule::set_joint_scale(boma, smash::phx::Hash40::new("handl"), &joint_scale);
						ModelModule::set_joint_scale(boma, smash::phx::Hash40::new("handr"), &joint_scale);
					}else if MotionModule::frame(boma)  < 20.0  {
						let mut mul = 1.1;
						let joint_scale = smash::phx::Vector3f { x: mul, y: mul, z: mul };
						ModelModule::set_joint_scale(boma, smash::phx::Hash40::new("handl"), &joint_scale);
						ModelModule::set_joint_scale(boma, smash::phx::Hash40::new("handr"), &joint_scale);
					}else {
						let mut mul = 1.0;
						let joint_scale = smash::phx::Vector3f { x: mul, y: mul, z: mul };
						ModelModule::set_joint_scale(boma, smash::phx::Hash40::new("handl"), &joint_scale);
						ModelModule::set_joint_scale(boma, smash::phx::Hash40::new("handr"), &joint_scale);
					};
				};
			};
		};
	};
}
//Edge Cancelling Part A
#[skyline::hook(replace = smash::app::lua_bind::StatusModule::init_settings)]
unsafe fn init_settings_replace(module_accessor: &mut smash::app::BattleObjectModuleAccessor, situation_kind: i32, arg3: i32, arg4: u64, ground_cliff_check_kind: u64, arg6: bool, arg7: i32, arg8: i32, arg9: i32, arg10: i32) -> u64 {
    let status_kind = StatusModule::status_kind(module_accessor);
    let fighter_kind = smash::app::utility::get_kind(module_accessor);
    if is_edge_cancel(fighter_kind, status_kind) && situation_kind == SITUATION_KIND_GROUND {
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
    let status_kind = StatusModule::status_kind(module_accessor);
    let fighter_kind = smash::app::utility::get_kind(module_accessor);
    if [*FIGHTER_STATUS_KIND_LANDING_ATTACK_AIR, *FIGHTER_STATUS_KIND_LANDING, *FIGHTER_STATUS_KIND_DASH, *FIGHTER_STATUS_KIND_TURN_DASH].contains(&status_kind) {
        original!()(module_accessor, 1 as u32)
    }
    else if is_edge_cancel(fighter_kind, status_kind) {
        original!()(module_accessor, *GROUND_CORRECT_KIND_GROUND as u32)
    }
    else {
        original!()(module_accessor, ground_correct_kind)
    }
}


pub fn install() {
    smashline::install_agent_frame_callbacks!(
		dacus,
		jabcancel,
		perfectpivot,
		parrycanceldash,
		llpc,
		dash,
		melee_hat,
		djc,
		bone_rot,
		wavedash,
		sword_size
	);
	smashline::install_agent_frame_callbacks!(footstool);
	skyline::install_hooks!(
        init_settings_replace,
        correct_replace
    );
}