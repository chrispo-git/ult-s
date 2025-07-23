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

unsafe extern "C" fn airdash(fighter : &mut L2CFighterCommon) {
    unsafe {
        if !is_gamemode("airdash".to_string()) {
            return;
        }
		let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent); 
		let ENTRY_ID = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
        let status_kind = StatusModule::status_kind(boma);
		let gravity = WorkModule::get_param_float(fighter.module_accessor, hash40("air_accel_y"), 0);
        if [*FIGHTER_STATUS_KIND_ESCAPE_AIR, *FIGHTER_STATUS_KIND_ESCAPE_AIR_SLIDE].contains(&status_kind) {
            CancelModule::enable_cancel(boma);
            fighter.sub_air_check_fall_common();
			HitModule::set_whole(boma, smash::app::HitStatus(*HIT_STATUS_NORMAL), 0);
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
    };
}
pub fn install() {
    Agent::new("fighter")
	.on_line(Main, airdash)
	.install();
}