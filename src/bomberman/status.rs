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
use smash::phx::Vector2f;
use crate::util::*;
use super::*;
use crate::bomberman::*;
use std::arch::asm;

pub fn install() {
    Agent::new("pacman")
    .status(Pre, *FIGHTER_STATUS_KIND_SPECIAL_N, special_n_pre)
    .status(Pre, *FIGHTER_STATUS_KIND_SPECIAL_HI, special_hi_pre)
    .status(Pre, *FIGHTER_PACMAN_STATUS_KIND_SPECIAL_HI_END, special_hi_end_pre)
    .status(Main, *FIGHTER_PACMAN_STATUS_KIND_SPECIAL_HI_LOOP, main_upb)
    .status(Main, *FIGHTER_STATUS_KIND_SPECIAL_N, main_neutralb)
    .status(Pre, *FIGHTER_STATUS_KIND_CATCH, catch_pre)
    .status(Main, *FIGHTER_STATUS_KIND_CATCH, catch_main)
    .status(Pre, *FIGHTER_STATUS_KIND_CATCH_PULL, catch_pull_pre)
    .status(Main, *FIGHTER_STATUS_KIND_CATCH_PULL, catch_pull_main)
    .status(Main, *FIGHTER_STATUS_KIND_SPECIAL_S, main_sideb)
    .status(Pre, *FIGHTER_STATUS_KIND_THROW_KIRBY, throw_kirby_pre)
    .status(Pre, *FIGHTER_STATUS_KIND_SPECIAL_S, special_s_pre)
    .install();
}

unsafe extern "C" fn special_n_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent); 
	let fighter_kind = smash::app::utility::get_kind(boma);
    if is_added(boma) && fighter_kind == *FIGHTER_KIND_PACMAN { 
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
            (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_N | *FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_AIR_LASSO | *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON) as u64,
            *FIGHTER_STATUS_ATTR_START_TURN as u32,
            *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_N as u32,
            0
        );
        0.into()
    }
    else{
        return smashline::original_status(Pre, fighter, *FIGHTER_STATUS_KIND_SPECIAL_N)(fighter);
    }
}
unsafe extern "C" fn special_hi_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent); 
	let fighter_kind = smash::app::utility::get_kind(boma);
    if is_added(boma) && fighter_kind == *FIGHTER_KIND_PACMAN { 
        fighter.change_status(FIGHTER_PACMAN_STATUS_KIND_SPECIAL_HI_LOOP.into(), true.into());
        0.into()
    }
    else{
        return smashline::original_status(Pre, fighter, *FIGHTER_STATUS_KIND_SPECIAL_HI)(fighter);
    }
}
unsafe extern "C" fn special_hi_end_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent); 
	let fighter_kind = smash::app::utility::get_kind(boma);
    if is_added(boma) && fighter_kind == *FIGHTER_KIND_PACMAN { 
        fighter.change_status(FIGHTER_STATUS_KIND_FALL_SPECIAL.into(), true.into());
        0.into()
    }
    else{
        return smashline::original_status(Pre, fighter, *FIGHTER_PACMAN_STATUS_KIND_SPECIAL_HI_END)(fighter);
    }
}
unsafe extern "C" fn main_upb(fighter: &mut L2CFighterCommon) -> L2CValue {
    let motion_kind = MotionModule::motion_kind(fighter.module_accessor);
    let ENTRY_ID = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    let frame = MotionModule::frame(fighter.module_accessor);
	let cancel_frame = FighterMotionModuleImpl::get_cancel_frame(fighter.module_accessor,smash::phx::Hash40::new_raw(MotionModule::motion_kind(fighter.module_accessor)),false) as f32; //Cancel frame
	let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent); 
	let fighter_kind = smash::app::utility::get_kind(boma);
    let situation_kind = StatusModule::situation_kind(boma);
    let is_ground = situation_kind == *SITUATION_KIND_GROUND;
    let is_end = MotionModule::is_end(fighter.module_accessor);
    let fighter_kinetic_energy_motion = mem::transmute::<u64, &mut smash::app::FighterKineticEnergyMotion>(KineticModule::get_energy(boma, *FIGHTER_KINETIC_ENERGY_ID_MOTION));  
    let is_near_ground = GroundModule::ray_check(fighter.module_accessor, &Vector2f{ x: PostureModule::pos_x(fighter.module_accessor), y: PostureModule::pos_y(fighter.module_accessor)}, &Vector2f{ x: 0.0, y: -1.0}, true);
    if is_added(boma) && fighter_kind == *FIGHTER_KIND_PACMAN { 
        if ![hash40("special_hi_loop"), hash40("special_air_hi_loop")].contains(&motion_kind){
            if is_ground || is_near_ground == 1{
                MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_hi_loop"), -1.0, 1.0, false, 0.0, false, false);
            } else {
                MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_hi_loop"), -1.0, 1.0, false, 0.0, false, false);
            }
        } else {
            if is_end {
                fighter.change_status(FIGHTER_STATUS_KIND_FALL_SPECIAL.into(), true.into());
            }
        }
        0.into() 
    }
    else{
        return smashline::original_status(Main, fighter, *FIGHTER_PACMAN_STATUS_KIND_SPECIAL_HI_LOOP)(fighter);
    }
} 

unsafe extern "C" fn main_neutralb(fighter: &mut L2CFighterCommon) -> L2CValue {
    let motion_kind = MotionModule::motion_kind(fighter.module_accessor);
    let ENTRY_ID = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    let frame = MotionModule::frame(fighter.module_accessor);
	let cancel_frame = FighterMotionModuleImpl::get_cancel_frame(fighter.module_accessor,smash::phx::Hash40::new_raw(MotionModule::motion_kind(fighter.module_accessor)),false) as f32; //Cancel frame
	let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent); 
	let fighter_kind = smash::app::utility::get_kind(boma);
    let situation_kind = StatusModule::situation_kind(boma);
    let is_ground = situation_kind == *SITUATION_KIND_GROUND;
    let is_end = MotionModule::is_end(fighter.module_accessor);
    let fighter_kinetic_energy_motion = mem::transmute::<u64, &mut smash::app::FighterKineticEnergyMotion>(KineticModule::get_energy(boma, *FIGHTER_KINETIC_ENERGY_ID_MOTION));  
    let is_near_ground = GroundModule::ray_check(fighter.module_accessor, &Vector2f{ x: PostureModule::pos_x(fighter.module_accessor), y: PostureModule::pos_y(fighter.module_accessor)}, &Vector2f{ x: 0.0, y: -1.0}, true);
    if is_added(boma) && fighter_kind == *FIGHTER_KIND_PACMAN { 
        if ![hash40("special_n"), hash40("special_n_hold"), hash40("special_n_shoot"), hash40("special_air_n"), hash40("special_air_n_hold"), hash40("special_air_n_shoot")].contains(&motion_kind){
            if is_ground {
                MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_n"), -1.0, 1.0, false, 0.0, false, false);
            } else {
                MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_n"), -1.0, 1.0, false, 0.0, false, false);
            }
        } else {
            if [hash40("special_n"), hash40("special_air_n")].contains(&motion_kind) {
                KineticModule::suspend_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
                if is_end {
                    if NEUTRALB_CHARGE[ENTRY_ID] >= NEUTRALB_MAX {
                        NEUTRALB_DIST[ENTRY_ID] = MIN_DISTANCE + ((MAX_DISTANCE-MIN_DISTANCE))*((NEUTRALB_CHARGE[ENTRY_ID] as f32)/(NEUTRALB_MAX as f32));
                        if is_ground {
                            MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_n_shoot"), -1.0, 1.0, false, 0.0, false, false);
                        } else {
                            MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_n_shoot"), -1.0, 1.0, false, 0.0, false, false);
                        }
                    } else {
                        if is_ground {
                            MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_n_hold"), -1.0, 1.0, false, 0.0, false, false);
                        } else {
                            MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_n_hold"), -1.0, 1.0, false, 0.0, false, false);
                        }
                    }
                }
                if (motion_kind == hash40("special_n") && !is_ground) || (motion_kind == hash40("special_air_n") && is_ground) {
                    if is_ground {
                        MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_n"), frame+1.0, 1.0, false, 0.0, false, false);
                    } else {
                        MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_n"), frame+1.0, 1.0, false, 0.0, false, false);
                    }
                }
            }
            if [hash40("special_n_hold"), hash40("special_air_n_hold")].contains(&motion_kind) {
                KineticModule::suspend_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
                if is_ground {
                    KineticModule::clear_speed_all(fighter.module_accessor);
                }
                if is_end {
                    if is_ground {
                        MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_n_hold"), -1.0, 1.0, false, 0.0, false, false);
                    } else {
                        MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_n_hold"), -1.0, 1.0, false, 0.0, false, false);
                    }
                }
                NEUTRALB_CHARGE[ENTRY_ID] += 1;
                let jc = is_jump(boma) && (WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT) < WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT_MAX) || is_ground);
                let sc = ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_GUARD) && (!WorkModule::is_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_DISABLE_ESCAPE_AIR) || is_ground);
                if NEUTRALB_CHARGE[ENTRY_ID] >= NEUTRALB_MAX {
					smash::app::FighterUtil::flash_eye_info(boma);
                }
                if NEUTRALB_CHARGE[ENTRY_ID] >= NEUTRALB_MAX || jc || sc  {
                    if is_ground {
                        fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
                    } else {
                        fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
                    }
                }
                if ControlModule::check_button_on_trriger(boma, *CONTROL_PAD_BUTTON_SPECIAL) {
                    NEUTRALB_DIST[ENTRY_ID] = MIN_DISTANCE + ((MAX_DISTANCE-MIN_DISTANCE))*((NEUTRALB_CHARGE[ENTRY_ID] as f32)/(NEUTRALB_MAX as f32));
                    if is_ground || is_near_ground == 1 {
                        MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_n_shoot"), -1.0, 1.0, false, 0.0, false, false);
                    } else {
                        MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_n_shoot"), -1.0, 1.0, false, 0.0, false, false);
                    }
                }
                if (motion_kind == hash40("special_n_hold") && !is_ground) || (motion_kind == hash40("special_air_n_hold") && is_ground) {
                    if is_ground {
						StatusModule::set_situation_kind(boma, smash::app::SituationKind(*SITUATION_KIND_GROUND), true);
                        MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_n_hold"), frame+1.0, 1.0, false, 0.0, false, false);
                    } else {
						StatusModule::set_situation_kind(boma, smash::app::SituationKind(*SITUATION_KIND_AIR), true);
                        MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_n_hold"), frame+1.0, 1.0, false, 0.0, false, false);
                    }
                }
            }
            if [hash40("special_n_shoot"), hash40("special_air_n_shoot")].contains(&motion_kind) {
                if is_end {
                    if is_ground {
                        fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
                    } else {
                        fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
                    }
                }
                smash::app::lua_bind::FighterKineticEnergyMotion::set_speed_mul(fighter_kinetic_energy_motion, NEUTRALB_DIST[ENTRY_ID]);
				if !is_ground {
                    if frame < 23.0  {
                        if KineticModule::get_kinetic_type(boma) != *FIGHTER_KINETIC_TYPE_MOTION_AIR{
                            KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_MOTION_AIR);
                        };
                    } else {
                        KineticModule::resume_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
                        if KineticModule::get_kinetic_type(boma) != *FIGHTER_KINETIC_TYPE_MOTION_FALL {
                            KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_MOTION_FALL);
                        };
                    };
                } else {
                        if KineticModule::get_kinetic_type(boma) != *FIGHTER_KINETIC_TYPE_MOTION {
                            KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_MOTION);
                        };
                };
                if frame >= cancel_frame {
                    reimpl_cancel_frame(fighter);
                }
                if (motion_kind == hash40("special_air_n_shoot") && is_ground) {
                    WorkModule::set_float(boma, 15.0, *FIGHTER_INSTANCE_WORK_ID_FLOAT_LANDING_FRAME);
                    fighter.change_status(FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL.into(), true.into());
                }
            }
        }

        0.into() 
    }
    else{
        return smashline::original_status(Main, fighter, *FIGHTER_STATUS_KIND_SPECIAL_N)(fighter);
    }
} 

unsafe extern "C" fn catch_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    let situation = if SIDEB_CATCH[WorkModule::get_int(smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent), *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize] && is_added(smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent)) {
        *SITUATION_KIND_NONE
    }
    else {
        *SITUATION_KIND_GROUND
    };
    let correct_kind = if SIDEB_CATCH[WorkModule::get_int(smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent), *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize] && is_added(smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent)) {
        *GROUND_CORRECT_KIND_KEEP
    }
    else {
        *GROUND_CORRECT_KIND_GROUND_CLIFF_STOP
    };
    StatusModule::init_settings(
        fighter.module_accessor,
        SituationKind(situation),
        *FIGHTER_KINETIC_TYPE_MOTION,
        correct_kind as u32,
        GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE),
        true,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT,
        *FS_SUCCEEDS_KEEP_ATTACK_ABSOLUTE
    );
    FighterStatusModuleImpl::set_fighter_status_data(
        fighter.module_accessor,
        false,
        *FIGHTER_TREADED_KIND_NO_REAC,
        false,
        true,
        false,
        0,
        (
            *FIGHTER_STATUS_ATTR_DISABLE_JUMP_BOARD_EFFECT |
            *FIGHTER_STATUS_ATTR_DISABLE_TURN_DAMAGE
        ) as u32,
        0,
        0
    );
    0.into()
}
unsafe extern "C" fn catch_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let situation_kind = StatusModule::situation_kind(fighter.module_accessor);
    let is_ground = situation_kind == *SITUATION_KIND_GROUND;
    let motion_kind = MotionModule::motion_kind(fighter.module_accessor);
    let is_end = MotionModule::is_end(fighter.module_accessor);
    if !(SIDEB_CATCH[WorkModule::get_int(smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent), *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize] && is_added(smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent))) {
       fighter.status_Catch();
    } else {
        if is_end && ![hash40("special_s_dash"), hash40("special_air_s_dash"), hash40("special_s_start"), hash40("special_air_s_start")].contains(&motion_kind){
            if is_ground {
                StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_WAIT, false);
            } else {
                StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_FALL, false);
            }
        }
    }
    0.into()
}
unsafe extern "C" fn catch_pull_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    
    let situation = if SIDEB_CATCH[WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize] && is_added(smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent)) {
        *SITUATION_KIND_NONE
    }
    else {
        *SITUATION_KIND_GROUND
    };
    let keep = if SIDEB_CATCH[WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize] && is_added(smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent)) {
        *GROUND_CORRECT_KIND_AIR as u32
    } else {
        *GROUND_CORRECT_KIND_GROUND_CLIFF_STOP as u32
    };
    StatusModule::init_settings(
        fighter.module_accessor,
        SituationKind(situation),
        *FIGHTER_KINETIC_TYPE_MOTION,
        keep as u32,
        GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE),
        true,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT,
        *FS_SUCCEEDS_KEEP_ATTACK_ABSOLUTE
    );
    FighterStatusModuleImpl::set_fighter_status_data(
        fighter.module_accessor,
        false,
        *FIGHTER_TREADED_KIND_NO_REAC,
        false,
        true,
        false,
        0,
        (
            *FIGHTER_STATUS_ATTR_DISABLE_JUMP_BOARD_EFFECT |
            *FIGHTER_STATUS_ATTR_DISABLE_TURN_DAMAGE
        ) as u32,
        0,
        0
    );
    /*if SIDEB_CATCH[WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize] && is_added(smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent)) {
        fighter.change_status(FIGHTER_STATUS_KIND_THROW_KIRBY.into(), false.into());
    }*/
    0.into()
}

unsafe extern "C" fn catch_pull_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let ENTRY_ID = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    if SIDEB_CATCH[ENTRY_ID] && is_added(smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent)) {
        sv_kinetic_energy!(
            clear_speed,
            fighter,
            FIGHTER_KINETIC_ENERGY_ID_MOTION
        );
        sv_kinetic_energy!(
            reset_energy,
            fighter,
            FIGHTER_KINETIC_ENERGY_ID_MOTION,
            ENERGY_MOTION_RESET_TYPE_AIR_TRANS,
            0.0,
            0.0,
            0.0,
            0.0,
            0.0
        );
    }
    fighter.status_CatchPull_common(hash40("catch_wait").into());
    fighter.sub_shift_status_main(L2CValue::Ptr(catch_pull_main_loop as *const () as _))
}

unsafe extern "C" fn catch_pull_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let ENTRY_ID = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    if SIDEB_CATCH[ENTRY_ID] && is_added(smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent)) {
        1.into()
    }
    else {
        fighter.status_CatchPull_Main()
    }
}


unsafe extern "C" fn main_sideb(fighter: &mut L2CFighterCommon) -> L2CValue {
    let motion_kind = MotionModule::motion_kind(fighter.module_accessor);
    let ENTRY_ID = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    let frame = MotionModule::frame(fighter.module_accessor);
	let cancel_frame = FighterMotionModuleImpl::get_cancel_frame(fighter.module_accessor,smash::phx::Hash40::new_raw(MotionModule::motion_kind(fighter.module_accessor)),false) as f32; //Cancel frame
	let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent); 
	let fighter_kind = smash::app::utility::get_kind(boma);
    let situation_kind = StatusModule::situation_kind(boma);
    let is_ground = situation_kind == *SITUATION_KIND_GROUND;
    let is_end = MotionModule::is_end(fighter.module_accessor);
    if is_added(boma) && fighter_kind == *FIGHTER_KIND_PACMAN { 
        if ![hash40("special_s_start"), hash40("special_air_s_start")].contains(&motion_kind){
            if is_ground {
                MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_s_start"), -1.0, 1.0, false, 0.0, false, false);
            } else {
                MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_s_start"), -1.0, 1.0, false, 0.0, false, false);
            }
            SIDEB_CATCH[ENTRY_ID] = true;
        } else {
            if is_end {
                SIDEB_CATCH[ENTRY_ID] = true;
                fighter.change_status(FIGHTER_STATUS_KIND_CATCH.into(), false.into());
            }
        }

        0.into() 
    }
    else{
        return smashline::original_status(Main, fighter, *FIGHTER_STATUS_KIND_SPECIAL_S)(fighter);
    }
} 

unsafe extern "C" fn throw_kirby_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent); 
	let fighter_kind = smash::app::utility::get_kind(boma);

    StatusModule::init_settings(
        fighter.module_accessor,
        SituationKind(*SITUATION_KIND_AIR),
        *FIGHTER_KINETIC_TYPE_MOTION_AIR,
        *GROUND_CORRECT_KIND_AIR as u32,
        GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE),
        true,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT,
        *FS_SUCCEEDS_KEEP_ATTACK_ABSOLUTE
    );
    FighterStatusModuleImpl::set_fighter_status_data(
        fighter.module_accessor,
        false,
        *FIGHTER_TREADED_KIND_NO_REAC,
        false,
        true,
        false,
        0,
        (
            *FIGHTER_STATUS_ATTR_DISABLE_JUMP_BOARD_EFFECT |
            *FIGHTER_STATUS_ATTR_DISABLE_TURN_DAMAGE
        ) as u32,
        0,
        0
    );
    //fighter.status_pre_ThrowKirby();
    0.into()
}
unsafe extern "C" fn special_s_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent); 
	let fighter_kind = smash::app::utility::get_kind(boma);
    if is_added(boma) && fighter_kind == *FIGHTER_KIND_PACMAN { 
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
    else{
        return smashline::original_status(Pre, fighter, *FIGHTER_STATUS_KIND_SPECIAL_S)(fighter);
    }
}