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

pub unsafe fn opff(fighter : &mut L2CFighterCommon, status_kind : i32) {
    if !is_gamemode("hitfall".to_string()) && !is_gamemode("rivals".to_string()){
        return;
    }
    let situation_kind = StatusModule::situation_kind(fighter.module_accessor);
    if AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_HIT) && status_kind == *FIGHTER_STATUS_KIND_ATTACK_AIR && is_hitlag(boma(fighter)) {
        let cat2 = ControlModule::get_command_flag_cat(fighter.module_accessor, 1);
        if (cat2 & *FIGHTER_PAD_CMD_CAT2_FLAG_FALL_JUMP) != 0 && ControlModule::get_stick_y(fighter.module_accessor) < -0.66 {
            WorkModule::set_flag(fighter.module_accessor, true, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_DIVE);
        }
    }
}