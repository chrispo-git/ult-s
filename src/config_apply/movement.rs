use smash::hash40;
use smash::lib::lua_const::*;
use smash::app::*;
use smash::app::lua_bind::*;
use smash::lua2cpp::{L2CFighterCommon, L2CAgentBase};
use smashline::*;
use smash_script::*;
use std::os::raw::c_int;
use std::os::raw::c_ulong;
use crate::common::*;
use crate::util::*;
use crate::config;

pub unsafe fn agt(fighter : &mut L2CFighterCommon,config : &config::Config) {
    if config.movement.agt == 0 {
        return;
    }
    let status_kind = StatusModule::status_kind(fighter.module_accessor);
    let curr_frame = WorkModule::get_int(fighter.module_accessor, *FIGHTER_STATUS_ESCAPE_WORK_INT_FRAME);
    if crate::is_in!(status_kind, *FIGHTER_STATUS_KIND_ESCAPE_AIR, *FIGHTER_STATUS_KIND_ESCAPE_AIR_SLIDE) {
        if 	ItemModule::is_have_item(fighter.module_accessor, 0)
            && ControlModule::check_button_on_trriger(fighter.module_accessor, *CONTROL_PAD_BUTTON_ATTACK)
            && curr_frame < 6 {
            fighter.clear_lua_stack();
            lua_args!(fighter, MA_MSC_ITEM_CHECK_HAVE_ITEM_TRAIT, ITEM_TRAIT_FLAG_NO_THROW);
            smash::app::sv_module_access::item(fighter.lua_state_agent);
            let throwable = !fighter.pop_lua_stack(1).get_bool();
            if throwable {
                StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_ITEM_THROW, false);
            }
        }
    }
}

pub unsafe fn g2a(fighter : &mut L2CFighterCommon,config : &config::Config) {
    if config.movement.g2a == 0 {
        return;
    }
    let status_kind = StatusModule::status_kind(fighter.module_accessor);
    if !crate::is_in!(status_kind, 
        *FIGHTER_GAOGAEN_STATUS_KIND_SPECIAL_HI_END, *FIGHTER_GAOGAEN_STATUS_KIND_SPECIAL_HI_LOOP, *FIGHTER_GAOGAEN_STATUS_KIND_SPECIAL_HI_FALL
    ) {
        if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_JUMP_NO_LIMIT) {
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_JUMP_NO_LIMIT);
        };
    }else {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_JUMP_NO_LIMIT);
    };
}

pub unsafe fn hitfall(fighter : &mut L2CFighterCommon,config : &config::Config) {
    if config.movement.hitfall == 0 {
        return;
    }
    let status_kind = StatusModule::status_kind(fighter.module_accessor);
    if AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_HIT) && status_kind == *FIGHTER_STATUS_KIND_ATTACK_AIR && is_hitlag(boma(fighter)) {
        let cat2 = ControlModule::get_command_flag_cat(fighter.module_accessor, 1);
        if (cat2 & *FIGHTER_PAD_CMD_CAT2_FLAG_FALL_JUMP) != 0 && ControlModule::get_stick_y(fighter.module_accessor) < -0.66 {
            WorkModule::set_flag(fighter.module_accessor, true, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_DIVE);
        }
    }
}

pub unsafe fn opff(fighter : &mut L2CFighterCommon,config : &config::Config) {
    hitfall(fighter, config);
    g2a(fighter, config);
    agt(fighter, config);
}