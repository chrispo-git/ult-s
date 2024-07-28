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
use crate::kirby::*;
use super::*;

pub fn install() {
	Agent::new("kirby")
    .status(Exec, *FIGHTER_KIRBY_STATUS_KIND_SPECIAL_N_DRINK, kirby_drink_exec)
    .status(Exec, *FIGHTER_STATUS_KIND_SPECIAL_LW, exec_downb)
    .status(Pre, *FIGHTER_STATUS_KIND_SPECIAL_LW, special_lw_pre)
    .status(Pre, *FIGHTER_KIRBY_STATUS_KIND_SPECIAL_S_ATTACK, special_s_pre)
    .install();
}

#[skyline::from_offset(0xb96770)]
fn copy_ability_reset(fighter: *mut Fighter, some_miifighter_bool: bool);

pub const FIGHTER_KIRBY_STATUS_SPECIAL_N_FLAG_CUSTOM_CHARACTER: i32 = 0x2100001E;

unsafe extern "C" fn kirby_drink_exec(fighter: &mut L2CFighterCommon) -> L2CValue {
    let original = smashline::original_status(Exec, fighter, *FIGHTER_KIRBY_STATUS_KIND_SPECIAL_N_DRINK)(fighter);

    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_KIRBY_STATUS_SPECIAL_N_FLAG_DRINK_WEAPON) {
        return original;
    }
    
    //While Kirby is drinking opponent...
    if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_KIRBY_STATUS_SPECIAL_N_FLAG_SPIT_END) {
        //Check for custom character
        if !WorkModule::is_flag(fighter.module_accessor, FIGHTER_KIRBY_STATUS_SPECIAL_N_FLAG_CUSTOM_CHARACTER) {
            if LinkModule::get_node_object_id(fighter.module_accessor, *LINK_NO_CAPTURE) != 0x50000000 {
                let opponent_id = LinkModule::get_node_object_id(fighter.module_accessor, *LINK_NO_CAPTURE) as u32;
                let opp = sv_battle_object::module_accessor(opponent_id);
                let opp_kind = utility::get_kind(&mut *opp);
                if (opp_kind == *FIGHTER_KIND_MURABITO && (WorkModule::get_int(opp, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) >= 120 && WorkModule::get_int(opp, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) <= 127)) //toad
                    || (opp_kind == *FIGHTER_KIND_PIKMIN && (WorkModule::get_int(opp, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) >= 120 && WorkModule::get_int(opp, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) <= 127)) //rayman
                    || (opp_kind == *FIGHTER_KIND_PACMAN && (WorkModule::get_int(opp, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) >= 120 && WorkModule::get_int(opp, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) <= 127)) //bomberman
                    || (opp_kind == *FIGHTER_KIND_MARIOD && (WorkModule::get_int(opp, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) >= 120 && WorkModule::get_int(opp, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) <= 127)) //sandbag
                    || (opp_kind == *FIGHTER_KIND_LUCAS && (WorkModule::get_int(opp, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) >= 120 && WorkModule::get_int(opp, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) <= 127)) //masked man
                {
                     WorkModule::on_flag(fighter.module_accessor, FIGHTER_KIRBY_STATUS_SPECIAL_N_FLAG_CUSTOM_CHARACTER);
                }
            }
        }
    } 
    //After getting the ability...
    else { 
        //Reset ability on spit out
        if WorkModule::get_int(fighter.module_accessor, *FIGHTER_KIRBY_INSTANCE_WORK_ID_INT_COPY_CHARA) != *FIGHTER_KIND_KIRBY
        && WorkModule::is_flag(fighter.module_accessor, FIGHTER_KIRBY_STATUS_SPECIAL_N_FLAG_CUSTOM_CHARACTER) {
            WorkModule::off_flag(fighter.module_accessor, FIGHTER_KIRBY_STATUS_SPECIAL_N_FLAG_CUSTOM_CHARACTER);
            let kirb = fighter.battle_object.cast::<Fighter>();
            copy_ability_reset(kirb, false);

            let star_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_KIRBY_INSTANCE_WORK_ID_INT_WN_STAR_TASK_ID) as u32;
            if sv_battle_object::is_active(star_id) {
                let star = sv_battle_object::module_accessor(star_id);
                WorkModule::set_int(star, *FIGHTER_KIND_KIRBY, *WEAPON_KIRBY_STARMISSILE_STATUS_WORK_INT_KIND);
            }
        }
    }
    
    return original;
}

unsafe extern "C" fn special_lw_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(
        fighter.module_accessor,
        smash::app::SituationKind(*SITUATION_KIND_NONE),
        *FIGHTER_KINETIC_TYPE_NONE,
        *GROUND_CORRECT_KIND_KEEP as u32,
		smash::app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE),
        true,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT,
        0
    );

    FighterStatusModuleImpl::set_fighter_status_data(
        fighter.module_accessor,
        false,
        *FIGHTER_TREADED_KIND_NO_REAC,
        false,
        false,
        false,
        (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_S | *FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_AIR_LASSO | *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON) as u64,
        0,
        *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_S as u32,
        0
    );
    0.into()
}
unsafe extern "C" fn exec_downb(fighter: &mut L2CFighterCommon) -> L2CValue {
    let motion_kind = MotionModule::motion_kind(fighter.module_accessor);
    let frame = MotionModule::frame(fighter.module_accessor);
    let ENTRY_ID = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    if [hash40("special_lw"), hash40("special_lw2")].contains(&motion_kind) {
        KineticModule::suspend_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
        0.into()
    } else {
        return smashline::original_status(Exec, fighter, *FIGHTER_STATUS_KIND_SPECIAL_LW)(fighter);
    }
}

unsafe extern "C" fn special_s_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(
        fighter.module_accessor,
        smash::app::SituationKind(*SITUATION_KIND_NONE),
        *FIGHTER_KINETIC_TYPE_NONE,
        *GROUND_CORRECT_KIND_KEEP as u32,
		smash::app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE),
        true,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT,
        0
    );

    FighterStatusModuleImpl::set_fighter_status_data(
        fighter.module_accessor,
        false,
        *FIGHTER_TREADED_KIND_NO_REAC,
        false,
        false,
        false,
        (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_S | *FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_AIR_LASSO | *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON) as u64,
        *FIGHTER_STATUS_ATTR_START_TURN as u32,
        *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_S as u32,
        0
    );
    0.into()
}
