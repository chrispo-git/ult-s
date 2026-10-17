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

pub unsafe fn opff(fighter : &mut L2CFighterCommon, status_kind : i32, situation_kind : i32) {
    if !is_gamemode("pogo".to_string()){
        return;
    }
    if situation_kind == *SITUATION_KIND_GROUND && !crate::is_in!(status_kind, *FIGHTER_STATUS_KIND_ENTRY, *FIGHTER_STATUS_KIND_JUMP_SQUAT) {
        StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_JUMP_SQUAT, true);
    }
}