
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

pub unsafe fn opff(fighter : &mut L2CFighterCommon, status_kind : i32, ENTRY_ID : usize) {
    if !is_gamemode("hitstun".to_string()) && !is_gamemode("sixtyfour".to_string()) {
        return;
    }
    let situation_kind = StatusModule::situation_kind(fighter.module_accessor);
    let remaining_hitstun = WorkModule::get_float(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLOAT_DAMAGE_REACTION_FRAME);
    let total_hitstun = WorkModule::get_float(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLOAT_DAMAGE_REACTION_FRAME_LAST);
    let mut hitstun_mul = 1.5;
    if !is_gamemode("hitstun".to_string()) {
        hitstun_mul = 0.533/0.4;
    }
    if remaining_hitstun > 0.0 {
        if remaining_hitstun == total_hitstun {
            //println!("Old hitstun {}, New Hitstun {}", remaining_hitstun, remaining_hitstun*hitstun_mul);
            WorkModule::set_float(fighter.module_accessor, remaining_hitstun*hitstun_mul, *FIGHTER_INSTANCE_WORK_ID_FLOAT_DAMAGE_REACTION_FRAME);
            WorkModule::set_float(fighter.module_accessor, (remaining_hitstun*hitstun_mul)+1.0, *FIGHTER_INSTANCE_WORK_ID_FLOAT_DAMAGE_REACTION_FRAME_LAST);
        }
    }
    if remaining_hitstun > 0.0 && [*FIGHTER_STATUS_KIND_DAMAGE_AIR, *FIGHTER_STATUS_KIND_DAMAGE_FALL, *FIGHTER_STATUS_KIND_DAMAGE_FLY, *FIGHTER_STATUS_KIND_DAMAGE_FLY_ROLL, *FIGHTER_STATUS_KIND_DAMAGE_FLY_METEOR].contains(&status_kind) {
        crate::transition_set!(ENTRY_ID, can_airdodge);
        crate::transition_set!(ENTRY_ID, can_attack_air);
    } else {
        crate::transition_reset!(ENTRY_ID, can_airdodge);
        crate::transition_reset!(ENTRY_ID, can_attack_air);
    };
}