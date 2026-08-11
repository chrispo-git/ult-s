mod attacks;
mod defense;
mod general;
mod movement;
mod special;


use smash::app::lua_bind::*;
use smash::lib::lua_const::*;
use smash::app::utility::get_kind;
use smash::hash40;
use smash::lua2cpp::*;
use smashline::*;
use smash_script::*;
use smash::phx::Hash40;
use crate::util::*;
use crate::config;

use std::time::Instant;

static mut FRAME_COUNT : i32 = 0;
static mut OPERATION_COUNT : f32 = 0.0;
static mut MAX_SPIKE : f32 = 0.0;

unsafe extern "C" fn config_apply(fighter : &mut L2CFighterCommon) {
    unsafe {
        let start = Instant::now();
        let config = config::get();

        let status_kind = StatusModule::status_kind(fighter.module_accessor);
        let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

        attacks::opff(fighter, &config, status_kind, entry_id);
        defense::opff(fighter, &config, status_kind, entry_id);
        general::opff(fighter, &config, status_kind, entry_id);
        movement::opff(fighter, &config, status_kind);
        special::opff(fighter, &config, status_kind, entry_id);


        let duration = start.elapsed();
        FRAME_COUNT += 1;
        OPERATION_COUNT += duration.as_micros() as f32;
        if duration.as_micros() as f32 > MAX_SPIKE {
            MAX_SPIKE = duration.as_micros() as f32;
        }
        if entry_id == 0 {
            if smash::app::sv_math::rand(hash40("fighter"), 30) == 0 {
                println!("Time to run logic this frame was was {}µs ({:.4} ms)", duration.as_micros(), duration.as_micros() as f32 / 1000.0);
                let average = OPERATION_COUNT / (FRAME_COUNT as f32);
                println!("Current average is {}µs ({:.4} ms", average, average / 1000.0);
                println!("Max Spike is {}µs ({:.4} ms", MAX_SPIKE, MAX_SPIKE / 1000.0);
            }
        }
        if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_APPEAL_HI) {
            FRAME_COUNT = 0;
            OPERATION_COUNT = 0.0;
            MAX_SPIKE = 0.0;
        }
    }
}
pub fn install() {
    Agent::new("fighter")
	.on_line(Main, config_apply)
	.install();
}