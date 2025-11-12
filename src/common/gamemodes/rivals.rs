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

static mut PAUSE : [bool; 8] = [false; 8];
static mut HAS_WALLJUMPED : [bool; 8] = [false; 8];

unsafe extern "C" fn rivals(fighter : &mut L2CFighterCommon) {
    unsafe {
        if !is_gamemode("rivals".to_string()){
            return;
        }
		let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent); 
		let ENTRY_ID = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
        let status_kind = StatusModule::status_kind(boma);
		let situation_kind = StatusModule::situation_kind(boma);
		let gravity = WorkModule::get_param_float(fighter.module_accessor, hash40("air_accel_y"), 0);
		let frame = MotionModule::frame(boma);
		let cancel_frame = FighterMotionModuleImpl::get_cancel_frame(boma,smash::phx::Hash40::new_raw(MotionModule::motion_kind(boma)),false) as f32;

        if status_kind == *FIGHTER_STATUS_KIND_DASH {
            if (frame as i32 == 3) && ControlModule::get_stick_x(boma).abs() < 0.2 {
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_WAIT, true);
            }
        }
        if status_kind == *FIGHTER_STATUS_KIND_TURN_DASH {
            if (frame as i32 == 4) && ControlModule::get_stick_x(boma).abs() < 0.2 {
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_WAIT, true);
            }
        }
        
        CAN_GRAB[ENTRY_ID] = 1;
		let cat = ControlModule::get_command_flag_cat(fighter.module_accessor, 0);
        if 
            [*FIGHTER_STATUS_KIND_FALL, *FIGHTER_STATUS_KIND_JUMP, *FIGHTER_STATUS_KIND_JUMP_AERIAL].contains(&status_kind) ||
            ([*FIGHTER_STATUS_KIND_FALL_SPECIAL, *FIGHTER_STATUS_KIND_SPECIAL_HI].contains(&status_kind) && !HAS_WALLJUMPED[ENTRY_ID]) ||
            ([*FIGHTER_STATUS_KIND_ATTACK_AIR].contains(&status_kind) && frame >= cancel_frame)
        {
            if (ControlModule::get_flick_x(boma) >= 3 && ControlModule::get_stick_x(boma) >= 0.7 && GroundModule::get_touch_flag(boma) == *GROUND_TOUCH_FLAG_LEFT as u64) 
            || (ControlModule::get_flick_x(boma) >= 3 && ControlModule::get_stick_x(boma) <= -0.7 && GroundModule::get_touch_flag(boma) == *GROUND_TOUCH_FLAG_RIGHT as u64)
            {
                HAS_WALLJUMPED[ENTRY_ID] = true;
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_WALL_JUMP, true);
            }
        }
        if situation_kind == *SITUATION_KIND_GROUND {
            HAS_WALLJUMPED[ENTRY_ID] = false;
        }
        if [*FIGHTER_STATUS_KIND_ESCAPE_AIR, *FIGHTER_STATUS_KIND_ESCAPE_AIR_SLIDE].contains(&status_kind) {
            if MotionModule::frame(boma) > 1.0 && MotionModule::frame(boma) < 3.0 {
                if ControlModule::get_stick_x(boma).abs() < 0.2 && ControlModule::get_stick_y(boma).abs() < 0.2 {
                    PAUSE[ENTRY_ID] = true;
                }
            }
            if MotionModule::frame(boma) > 28.0 {
                PAUSE[ENTRY_ID] = false;
            } else if MotionModule::frame(boma) > 16.0 {
                PAUSE[ENTRY_ID] = true;
            }
            if PAUSE[ENTRY_ID] { 
				KineticModule::clear_speed_all(boma);
				macros::SET_SPEED_EX(fighter, 0.0, gravity, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
            }
        } else {
            PAUSE[ENTRY_ID] = false;
        }
        
        if AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_SHIELD) {
            if situation_kind == *SITUATION_KIND_GROUND {
			    StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_SAVING_DAMAGE, true);
                //WorkModule::set_int(boma, 70, *FIGHTER_STATUS_SAVING_DAMAGE_WORK_INT_STUN_FRAME);
            } else {
			    StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_FALL_SPECIAL, true);
            }
        }
        GroundModule::set_cliff_check(fighter.module_accessor, smash::app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE));
    };
}
pub fn install() {
    Agent::new("fighter")
	.on_line(Main, rivals)
	.install();
}