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

pub unsafe fn opff(fighter : &mut L2CFighterCommon, status_kind : i32, ENTRY_ID : usize) {
    unsafe {
        if !is_gamemode("airdash".to_string()) {
            return;
        }
		let gravity = WorkModule::get_param_float(fighter.module_accessor, hash40("air_accel_y"), 0);
        if [*FIGHTER_STATUS_KIND_ESCAPE_AIR, *FIGHTER_STATUS_KIND_ESCAPE_AIR_SLIDE].contains(&status_kind) {
            CancelModule::enable_cancel(fighter.module_accessor);
            fighter.sub_air_check_fall_common();
			HitModule::set_whole(fighter.module_accessor, smash::app::HitStatus(*HIT_STATUS_NORMAL), 0);
            if MotionModule::frame(fighter.module_accessor) > 1.0 && MotionModule::frame(fighter.module_accessor) < 3.0 {
                if ControlModule::get_stick_x(fighter.module_accessor).abs() < 0.2 && ControlModule::get_stick_y(fighter.module_accessor).abs() < 0.2 {
                    PAUSE[ENTRY_ID] = true;
                }
            }
            if MotionModule::frame(fighter.module_accessor) > 28.0 {
                PAUSE[ENTRY_ID] = false;
            } else if MotionModule::frame(fighter.module_accessor) > 16.0 {
                PAUSE[ENTRY_ID] = true;
            }
            if PAUSE[ENTRY_ID] { 
				KineticModule::clear_speed_all(fighter.module_accessor);
				macros::SET_SPEED_EX(fighter, 0.0, gravity, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
            }
        } else {
            PAUSE[ENTRY_ID] = false;
        }
    };
}