mod hitstun;
mod dacus;
mod landing;
mod wavedash;
mod jab;
mod movement;
mod bone;
mod projectile_invuln;
mod remove_quake;
mod cancel;
mod css;
mod gamemodes;

use smash::app::lua_bind::*;
use smash::lib::lua_const::*;
use smash::app::utility::get_kind;
use smash::hash40;
use smash::lua2cpp::*;
use smashline::*;
use smash_script::*;
use smash::phx::Hash40;
use crate::util::*;

pub const MAX_WEIGHT : i32 = 150;
pub const MIN_WEIGHT : i32 = 60;
pub const MAX_GRAVITY : f32 = 0.1;
pub const MIN_GRAVITY : f32 = 0.065;

use std::time::Instant;

static mut FRAME_COUNT : i32 = 0;
static mut OPERATION_COUNT : f32 = 0.0;
static mut MAX_SPIKE : f32 = 0.0;

unsafe extern "C" fn common(fighter : &mut L2CFighterCommon) {
    unsafe {
        let start = Instant::now();

        let status_kind = StatusModule::status_kind(fighter.module_accessor);
        let motion_kind = MotionModule::motion_kind(fighter.module_accessor);
		let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
        dacus::opff(fighter, status_kind, entry_id);
        cancel::opff(fighter, status_kind);
        hitstun::opff(fighter, status_kind);
        jab::opff(fighter, status_kind, motion_kind, entry_id);
        landing::opff(fighter, status_kind, motion_kind, entry_id);
        movement::opff(fighter, status_kind, motion_kind, entry_id);
        projectile_invuln::opff(fighter, motion_kind);
        wavedash::opff(fighter, status_kind, motion_kind, entry_id);
        gamemodes::opff(fighter, status_kind, motion_kind, entry_id);
        /*
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
        }*/
    }
}

pub unsafe fn lazy_warm() {
    cancel::lazy_warm();
    dacus::lazy_warm();
    landing::lazy_warm();
    movement::lazy_warm();
    projectile_invuln::lazy_warm();
    wavedash::lazy_warm();
    for entry_id in 0..8 {
        crate::warm_up_states!(entry_id, [
            crate::util::InfoState,
            crate::util::TransitionEnableState,
            crate::util::JabState,
        ]);
    }
}

pub fn install() {
    Agent::new("fighter")
	.on_line(Main, common)
	.install();

    gamemodes::install();
	css::install();
    hitstun::install();
    landing::install();
    wavedash::install();
    movement::install();
    bone::install();
	remove_quake::install();
}