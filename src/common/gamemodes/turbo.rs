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
    unsafe {
        if !is_gamemode("turbo".to_string()) {
            return;
        }
        if status_kind == *FIGHTER_STATUS_KIND_CATCH_ATTACK{
            return;
        }
		let situation_kind = StatusModule::situation_kind(fighter.module_accessor);
        if !AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_HIT) {
            return;
        }
        CancelModule::enable_cancel(fighter.module_accessor);
        if situation_kind == *SITUATION_KIND_GROUND {
            fighter.sub_wait_ground_check_common(false.into());
        } else {
            fighter.sub_air_check_fall_common();
        }
    };
}