
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

pub unsafe fn float_setter(fighter : &mut L2CFighterCommon) {
    WorkModule::on_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_DISABLE_ESCAPE_AIR);    
    
    WorkModule::set_float(fighter.module_accessor, 0.0, *FIGHTER_STATUS_DAMAGE_WORK_FLOAT_VECOR_CORRECT_STICK_X);
    WorkModule::set_float(fighter.module_accessor, 0.0, *FIGHTER_STATUS_DAMAGE_WORK_FLOAT_VECOR_CORRECT_STICK_Y);
    
    WorkModule::set_float(fighter.module_accessor, 65.0, *FIGHTER_INSTANCE_WORK_ID_FLOAT_GUARD_SHIELD_MAX);
}
pub unsafe fn status_force_change(fighter : & mut L2CFighterCommon, status_kind : i32) {
    match status_kind {
        n if n == *FIGHTER_STATUS_KIND_ATTACK_S4_HOLD => {
            StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_ATTACK_S4, false);
        },
        n if n == *FIGHTER_STATUS_KIND_ATTACK_HI4_HOLD => {
            StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_ATTACK_HI4, false);
        },
        n if n == *FIGHTER_STATUS_KIND_ATTACK_LW4_HOLD => {
            StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_ATTACK_LW4, false);
        },
        n if n == *FIGHTER_STATUS_KIND_CATCH_DASH => {
            StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_CATCH, false);
        },
        _ => {},
    };
}
pub unsafe fn slow_dash(fighter : & mut L2CFighterCommon, status_kind : i32) {
    if !crate::is_in!(status_kind, *FIGHTER_STATUS_KIND_DASH, *FIGHTER_STATUS_KIND_TURN_DASH) {
        return;
    }
    MotionModule::set_rate(fighter.module_accessor, 0.8);
}

pub unsafe fn shieldstun_alter(fighter : &mut L2CFighterCommon, status_kind : i32) {
    let shieldstun_mul = (1.75/0.8) / match status_kind {
        n if n == *FIGHTER_STATUS_KIND_ATTACK_AIR => 0.33,
        n if crate::is_in!(n, *FIGHTER_STATUS_KIND_ATTACK_S4, *FIGHTER_STATUS_KIND_ATTACK_LW4, *FIGHTER_STATUS_KIND_ATTACK_HI4) => 0.725,
        _ => 1.0,
    };

    for i in 0..6 {
        if AttackModule::is_attack(fighter.module_accessor, i, false) {
            macros::ATK_SET_SHIELD_SETOFF_MUL(fighter, /*ID*/ i as u64, /*ShieldstunMul*/ shieldstun_mul);
        }
    }
}
pub unsafe fn opff(fighter : &mut L2CFighterCommon, status_kind : i32) {
    unsafe {
        if !is_gamemode("sixtyfour".to_string()) {
            return;
        }
        float_setter(fighter);
        status_force_change(fighter, status_kind);
        slow_dash(fighter, status_kind);
        shieldstun_alter(fighter, status_kind);
    };
}