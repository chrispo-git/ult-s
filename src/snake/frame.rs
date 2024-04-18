use smash::app::sv_animcmd::*;
use smash::phx::Hash40;
use smash::app::lua_bind::*;
use smash::lib::lua_const::*;
use smash::app::utility::get_kind;
use smash::hash40;
use smash::lua2cpp::*;
use smashline::*;
use smash_script::*;
use smash::lib::{L2CValue, L2CAgent};
use std::mem;
use smash::app::*;
use smash::phx::Vector3f;
use crate::util::*;
use super::*;

pub fn install() {
    Agent::new("snake")
    .on_line(Main, snake_frame)
    .install();
}

unsafe extern "C" fn snake_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent); 
        if is_default(boma) {
            let status_kind = smash::app::lua_bind::StatusModule::status_kind(boma);
            let motion_kind = MotionModule::motion_kind(boma);
            let frame = MotionModule::frame(boma);
            let stick_y = ControlModule::get_stick_y(boma);
            let ENTRY_ID = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
            if ItemModule::is_have_item(boma, 0) {
                WorkModule::unable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_THROW_DASH);
            }
            if motion_kind == hash40("attack_dash")
            && frame >= 12.0 {
                if ItemModule::is_have_item(boma, 0) {
                    if !ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_ATTACK) {
                        MotionModule::change_motion(boma, Hash40::new("attack_dash_item_light_throw"), 0.0, 1.0, false, 0.0, false, false);
                        AttackModule::clear_all(boma);
                    }
                }
            }
            if motion_kind == hash40("attack_dash_item_light_throw") {
                if fighter.global_table[0x16].get_i32() != *SITUATION_KIND_AIR && stick_y <= -0.6875 {
                    if GroundModule::is_passable_ground(boma) {
                        StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_PASS, true);
                    }
                }
            }
            if StatusModule::status_kind(fighter.module_accessor) == *FIGHTER_STATUS_KIND_ATTACK_S4 {
                if SNAKE_FLAG_ATTACK_S4_COMBO_IS_BUFFERED[ENTRY_ID] == false {
                    if ControlModule::check_button_on_trriger(fighter.module_accessor, *CONTROL_PAD_BUTTON_ATTACK)
                    || ControlModule::check_button_on_trriger(fighter.module_accessor, *CONTROL_PAD_BUTTON_SMASH) {
                        SNAKE_FLAG_ATTACK_S4_COMBO_IS_BUFFERED[ENTRY_ID] = true;
                    }
                }
                if SNAKE_FLAG_ATTACK_S4_COMBO_ENABLE[ENTRY_ID]
                && SNAKE_FLAG_ATTACK_S4_COMBO_IS_BUFFERED[ENTRY_ID] {
                    SNAKE_FLAG_ATTACK_S4_COMBO_ENABLE[ENTRY_ID] = false;
                    SNAKE_FLAG_ATTACK_S4_COMBO_IS_BUFFERED[ENTRY_ID] = false;
                    if SNAKE_INT_ATTACK_S4_COMBO_COUNT[ENTRY_ID] == 0 {
                        SNAKE_INT_ATTACK_S4_COMBO_COUNT[ENTRY_ID] = 1;
                        ControlModule::reset_trigger(fighter.module_accessor);
                        MotionModule::change_motion(fighter.module_accessor, Hash40::new("attack_s4_s2"), 0.0, 1.0, false, 0.0, false, false);
                    }else {
                        MotionModule::change_motion(fighter.module_accessor, Hash40::new("attack_s4_s3"), 0.0, 1.0, false, 0.0, false, false);
                    }
                }
            }
        }
    }
}