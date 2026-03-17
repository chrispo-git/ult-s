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
static mut PARRY_DURATION : [i32; 8] = [0; 8];
static NONE :  smash::phx::Vector3f =  smash::phx::Vector3f { x: 0.0, y: 0.0, z: 0.0 };

pub unsafe fn opff(fighter : &mut L2CFighterCommon, status_kind : i32, motion_kind : u64, ENTRY_ID : usize) {
    unsafe {
        if !is_gamemode("parry".to_string()) &&  !is_gamemode("rivals".to_string()){
            return;
        }
        if PARRY_DURATION[ENTRY_ID] > 0 {
            PARRY_DURATION[ENTRY_ID] -= 1;
        }
        if crate::is_in!(status_kind, *FIGHTER_STATUS_KIND_GUARD, *FIGHTER_STATUS_KIND_GUARD_ON, *FIGHTER_STATUS_KIND_GUARD_DAMAGE) {
			StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_GUARD_OFF, false);
            return;
        }
        if crate::is_motion!(motion_kind, "just_shield", "just_shield_off") {
            PARRY_DURATION[ENTRY_ID] = 10;
            return;
        }
		let situation_kind = StatusModule::situation_kind(fighter.module_accessor);
        if situation_kind == *SITUATION_KIND_GROUND && PARRY_DURATION[ENTRY_ID] == 1 && !(*FIGHTER_STATUS_KIND_DAMAGE..*FIGHTER_STATUS_KIND_DAMAGE_FALL).contains(&status_kind) {
			StopModule::end_stop(fighter.module_accessor);
            //println!("End Stun Early");
			StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_WAIT, false);
            return;
        }
        let frame = MotionModule::frame(fighter.module_accessor);
        if status_kind == *FIGHTER_STATUS_KIND_REBOUND && (frame as i32) == 1 {
            if is_gamemode("rivals".to_string()) && MotionModule::rate(fighter.module_accessor) == 0.5 {
                macros::FLASH(fighter, 0.25, 0.25, 0.25, 0.5);
            }
        }
        if status_kind == *FIGHTER_STATUS_KIND_GUARD_OFF {
            WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ESCAPE);
            WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ESCAPE_F);
            WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ESCAPE_B);
            WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_SQUAT_BUTTON);
            WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_SQUAT);
            WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_HI4_START);
            WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_HI);
            WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_HI_COMMAND);

            match (frame as i32) {
                1 => {
                    macros::PLAY_SE(fighter, Hash40::new("se_common_step_snow"));
                    macros::PLAY_SE(fighter, Hash40::new("se_common_step_sand"));
                    macros::PLAY_SE(fighter, Hash40::new("se_common_step_sand"));

                    macros::FLASH(fighter, 0.92, 0.99, 1, 0.5);
                    EffectModule::req_follow(fighter.module_accessor, smash::phx::Hash40::new("sys_just_shield_hit"), smash::phx::Hash40::new("hip"), &NONE, &NONE, 0.7, true, 0, 0, 0, 0, 0, true, true) as u32;
                },
                n if n > 6 => {
                    macros::COL_NORMAL(fighter);
                    let rate = if is_gamemode("rivals".to_string()) { 0.5 } else { 1.0 };
                    WorkModule::set_float(fighter.module_accessor, rate, *FIGHTER_STATUS_WORK_ID_FLOAT_REBOUND_MOTION_RATE);
                    StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_REBOUND, true);
                },
                n if n > 4 => {
                    WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_CATCH);
                },
                _ => {},
            };
        }
    };
}
//Parry Reflects
#[skyline::hook(replace=smash::app::FighterUtil::is_valid_just_shield_reflector)]
unsafe fn is_valid_just_shield_reflector(module_accessor: &mut smash::app::BattleObjectModuleAccessor) -> bool {
    if is_gamemode("parry".to_string()) {return true; }
    original!()(module_accessor)
}
pub fn install() {
	skyline::install_hooks!(
		is_valid_just_shield_reflector
    );
}