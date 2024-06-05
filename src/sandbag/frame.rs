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
use crate::sandbag::*;



pub fn install() {
    Agent::new("mariod")
        .on_line(Main, sandbag_frame)
        .install();
}



unsafe extern "C" fn sandbag_frame(fighter: &mut L2CFighterCommon) {
    let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent); 
    let status_kind = smash::app::lua_bind::StatusModule::status_kind(boma);
    let lua_state = fighter.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    let fighter_kind = smash::app::utility::get_kind(boma);
    let motion_kind = MotionModule::motion_kind(boma);
    let situation_kind = StatusModule::situation_kind(boma);
    let stick_x = ControlModule::get_stick_x(boma) * PostureModule::lr(boma);
    let stick_y = ControlModule::get_stick_y(boma);
    let ENTRY_ID = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    let rate = MotionModule::rate(boma);
    //let new_speed = 0.0;
    STICK_DIRECTION[ENTRY_ID] = ControlModule::get_stick_dir(boma) * DIR_MULT;
    
    if is_added(boma) && fighter_kind == *FIGHTER_KIND_MARIOD { //silver
        if situation_kind != *SITUATION_KIND_AIR || (*FIGHTER_STATUS_KIND_DAMAGE..*FIGHTER_STATUS_KIND_DAMAGE_FALL).contains(&status_kind) {
            CAN_UPB[ENTRY_ID] = 0;
        }
        if MotionModule::motion_kind(fighter.module_accessor) == hash40("special_lw3") {
            if MotionModule::frame(boma) >= 26.0 {
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_WAIT, true);
                StatusModule::set_situation_kind(boma, smash::app::SituationKind(*SITUATION_KIND_GROUND), true);
            };
        };
        if [hash40("attack_s3_hi"),hash40("attack_s3_s"),hash40("attack_s3_lw")].contains(&MotionModule::motion_kind(boma)) {
            if MotionModule::frame(boma) <= 47.0 {
                StatusModule::set_situation_kind(boma, smash::app::SituationKind(*SITUATION_KIND_GROUND), true);
                KineticModule::clear_speed_all(fighter.module_accessor);
            };
        };
        if [hash40("attack_12"),hash40("attack_13")].contains(&MotionModule::motion_kind(boma)) {
            StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_WAIT, true);
            StatusModule::set_situation_kind(boma, smash::app::SituationKind(*SITUATION_KIND_GROUND), true);
        };

        if [hash40("catch"),hash40("catch_turn"),hash40("catch_dash")].contains(&MotionModule::motion_kind(boma)) {
            let int_frame = MotionModule::frame(boma) as i32;
            if int_frame == 14 {
                macros::EFFECT(fighter, Hash40::new("sys_erace_smoke"), Hash40::new("throw"), 0, 0, 0, 0, 0, 0, 1.3, 0, 0, 0, 0, 0, 0, false);
            }
            if int_frame >= 16 {
                ModelModule::set_mesh_visibility(boma, Hash40::new("hand"), false);
            }
        }
        //side b shit
        if WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_HIT_STOP_ATTACK_SUSPEND_FRAME) < 1 && status_kind == *FIGHTER_STATUS_KIND_SPECIAL_S {
            if [hash40("special_air_s"),hash40("special_s")].contains(&MotionModule::motion_kind(boma)) {
                if [hash40("special_air_s")].contains(&MotionModule::motion_kind(boma)) {
                    if SIDEB_COUNT[ENTRY_ID] == 0 {
                        macros::SET_SPEED_EX(fighter, 0.0, -0.325, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
                    };
                }
                else if [hash40("special_s")].contains(&MotionModule::motion_kind(boma)) {
                    //macros::SET_SPEED_EX(fighter, 0.0, 0.05, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
                    StatusModule::set_situation_kind(boma, smash::app::SituationKind(*SITUATION_KIND_AIR), true);
                    StatusModule::set_keep_situation_air(boma, true);
                };
                KineticModule::suspend_energy(boma, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
                SIDEB_CHARGE[ENTRY_ID] += 1.0;
                if SIDEB_CHARGE[ENTRY_ID] >= SIDEB_MAX || (SIDEB_CHARGE[ENTRY_ID] >= 10.0 && ControlModule::check_button_off(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL)) {
                    MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_s_fire"), 0.0, 1.0, false, 0.0, false, false);
                    //StatusModule::set_situation_kind(boma, smash::app::SituationKind(*SITUATION_KIND_AIR), true);
                    //StatusModule::set_keep_situation_air(boma, true);
                };
            }
            else {
                macros::EFFECT_OFF_KIND(fighter, Hash40::new("sys_explosion_sign"), false, true);
            }
            if [hash40("special_air_s_fire")].contains(&MotionModule::motion_kind(boma)) {
                //StatusModule::set_situation_kind(boma, smash::app::SituationKind(*SITUATION_KIND_AIR), true);
                //StatusModule::set_keep_situation_air(boma, true);
                //KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_MOTION_AIR);
                //KineticModule::suspend_energy(boma, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
                SIDEB_COUNT[ENTRY_ID] = 1;
                if SIDEB_FIRE_COUNT[ENTRY_ID] == 0 {
                    KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_MOTION_AIR);
                    KineticModule::clear_speed_energy_id(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_DAMAGE);
                    KineticModule::clear_speed_all(fighter.module_accessor);
                    macros::EFFECT(fighter, Hash40::new("sys_erace_smoke"), Hash40::new("top"), -4, 8, 0, 0, 0, 0, 2.0, 0, 0, 0, 0, 0, 0, false);
                    macros::LAST_EFFECT_SET_RATE(fighter, 0.92);
                };
                if SIDEB_FIRE_COUNT[ENTRY_ID] <= 24 {
                    println!("{}", SIDEB_CHARGE[ENTRY_ID]);
                    //let new_speed = NEW_SPEED_MUT[ENTRY_ID];
                    macros::SET_SPEED_EX(fighter, (SIDEB_CHARGE[ENTRY_ID] / 45.0) + 0.6, 0.33, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
                    SIDEB_FIRE_COUNT[ENTRY_ID] += 1;
                }
                else if (SIDEB_FIRE_COUNT[ENTRY_ID] > 24 && SIDEB_FIRE_COUNT[ENTRY_ID] <= 48) {
                    println!("{}", SIDEB_CHARGE[ENTRY_ID]);
                    //let new_speed = NEW_SPEED_MUT[ENTRY_ID];
                    macros::SET_SPEED_EX(fighter, (SIDEB_CHARGE[ENTRY_ID] / 45.0) + 0.6, -0.33, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
                    SIDEB_FIRE_COUNT[ENTRY_ID] += 1;
                }
                else if SIDEB_FIRE_COUNT[ENTRY_ID] > 48 && !SIDEB_HIT[ENTRY_ID] {
                    //SIDEB_CHARGE[ENTRY_ID] = 0.0;
                    SIDEB_FIRE_COUNT[ENTRY_ID] = 0;
                    MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_s_fire_end"), 0.0, 1.0, false, 0.0, false, false);
                    StatusModule::set_situation_kind(boma, smash::app::SituationKind(*SITUATION_KIND_AIR), true);
                };
                if AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT) || AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_SHIELD){
                    SIDEB_HIT[ENTRY_ID] = true;
                    StopModule::end_stop(boma);
                    MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_s_fire_hit"), 0.0, 1.0, false, 0.0, false, false);
                    StatusModule::set_situation_kind(boma, smash::app::SituationKind(*SITUATION_KIND_AIR), true);
                };
                GroundModule::clear_pass_floor(boma);
                if ray_check_pos(boma, 0.0, -0.6, true) == 1 && SIDEB_FIRE_COUNT[ENTRY_ID] > 24 {
                        StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_LANDING, true);
                        StatusModule::set_situation_kind(boma, smash::app::SituationKind(*SITUATION_KIND_GROUND), true);
                        macros::SET_SPEED_EX(fighter, 0.0, 0.0, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
                };
            };
            if [hash40("special_air_s_fire_hit")].contains(&MotionModule::motion_kind(boma)) {
                macros::SET_SPEED_EX(fighter, -0.55, 0.55, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
                if MotionModule::frame(boma) >= 25.0 {
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_FALL, true);
                    StatusModule::set_situation_kind(boma, smash::app::SituationKind(*SITUATION_KIND_AIR), true);
                };
            };
            if [hash40("special_air_s_fire_end")].contains(&MotionModule::motion_kind(boma)) {
                if MotionModule::frame(boma) >= 11.0 {
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_FALL, true);
                    StatusModule::set_situation_kind(boma, smash::app::SituationKind(*SITUATION_KIND_AIR), true);
                };
            };
        };
        if ![hash40("special_air_s"),hash40("special_s"),hash40("special_air_s_fire")].contains(&MotionModule::motion_kind(boma)) {
            SIDEB_FIRE_COUNT[ENTRY_ID] = 0;
            SIDEB_CHARGE[ENTRY_ID] = 0.0;
            SIDEB_HIT[ENTRY_ID] = false;
            SIDEB_SOUND[ENTRY_ID] = false;
            SIDEB_PRESS[ENTRY_ID] = false;
        };
        if situation_kind != *SITUATION_KIND_AIR {
            SIDEB_COUNT[ENTRY_ID] = 0;
            BAN_UPB_TECH[ENTRY_ID] = false;
        };

        //up b shit
        if [hash40("special_hi"),hash40("special_air_hi")].contains(&MotionModule::motion_kind(boma)) {
            //New method 
            if DamageModule::damage(boma, 0) < 300.0 {
                SPEED_MUL[ENTRY_ID] = 0.95 + 0.5*(DamageModule::damage(boma, 0)/300.0);
            } else {
                SPEED_MUL[ENTRY_ID] = 1.45;
            }
            if MotionModule::frame(boma) < 18.0 {
                macros::SET_SPEED_EX(fighter, 0.0, 0.0, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
            }
            else {
                let fighter_kinetic_energy_motion = mem::transmute::<u64, &mut smash::app::FighterKineticEnergyMotion>(KineticModule::get_energy(boma, *FIGHTER_KINETIC_ENERGY_ID_MOTION));  
                smash::app::lua_bind::FighterKineticEnergyMotion::set_speed_mul(fighter_kinetic_energy_motion, SPEED_MUL[ENTRY_ID]);
            };
            if GroundModule::is_wall_touch_line(boma, *GROUND_TOUCH_FLAG_ALL as u32) && MotionModule::frame(boma) > 24.0 {//&& !BAN_UPB_TECH[ENTRY_ID] {
                CAN_UPB[ENTRY_ID] = 1;
                if /*DamageModule::damage(boma, 0) >= 50.0 &&*/ DamageModule::damage(boma, 0) < 300.0 {
                    if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_GUARD) {
                        StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_PASSIVE_WALL, true);
                        //BAN_UPB_TECH[ENTRY_ID] = true;
                    }
                    else {
                        DamageModule::add_damage(boma, 8.0, 0);
                        StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_LR, true);
                    };
                };
            };
            if MotionModule::frame(boma) > 14.0 {
                ModelModule::set_mesh_visibility(boma, Hash40::new("rockman_ex_batm"), false);
                ModelModule::set_mesh_visibility(boma, Hash40::new("rockman_ex_bat"), false);
            }
        }
        else {
            macros::EFFECT_OFF_KIND(fighter, Hash40::new("mariod_superjump_power"), false, true);
            //UPB_CAN_CANCEL[ENTRY_ID] = false;
        };

        //down b shit
        if [*FIGHTER_STATUS_KIND_DEAD, *FIGHTER_STATUS_KIND_LOSE, *FIGHTER_STATUS_KIND_WIN].contains(&status_kind) || smash::app::sv_information::is_ready_go() == false {
            IS_METAL[ENTRY_ID] = false;
            DOWNB_COUNT[ENTRY_ID] = 0;
        };
        if WorkModule::is_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_METAL) {
            IS_METAL[ENTRY_ID] = true;
        }
        else {
            IS_METAL[ENTRY_ID] = false;
        };
        if [hash40("special_lw"),hash40("special_air_lw")].contains(&MotionModule::motion_kind(boma)) {
            if DOWNB_COUNT[ENTRY_ID] == 0 && !IS_METAL[ENTRY_ID] {
                if MotionModule::frame(boma) >= 5.0 && MotionModule::frame(boma) <= 44.0 {
                    ModelModule::set_mesh_visibility(boma, Hash40::new("rockman_ex_box"), true);
                    ModelModule::set_mesh_visibility(boma, Hash40::new("rockman_ex_box2"), true);
                    ModelModule::set_mesh_visibility(boma, Hash40::new("rockman_ex_bomb"), false);
                }
                else {
                    ModelModule::set_mesh_visibility(boma, Hash40::new("rockman_ex_box"), false);
                    ModelModule::set_mesh_visibility(boma, Hash40::new("rockman_ex_box2"), false);
                    ModelModule::set_mesh_visibility(boma, Hash40::new("rockman_ex_bomb"), false);
                };
            }
            else if DOWNB_COUNT[ENTRY_ID] == 1 && !IS_METAL[ENTRY_ID] {
                if MotionModule::frame(boma) >= 5.0 && MotionModule::frame(boma) <= 44.0 {
                    ModelModule::set_mesh_visibility(boma, Hash40::new("rockman_ex_box"), true);
                    ModelModule::set_mesh_visibility(boma, Hash40::new("rockman_ex_box2"), false);
                    ModelModule::set_mesh_visibility(boma, Hash40::new("rockman_ex_bomb"), false);
                }
                else {
                    ModelModule::set_mesh_visibility(boma, Hash40::new("rockman_ex_box"), false);
                    ModelModule::set_mesh_visibility(boma, Hash40::new("rockman_ex_box2"), false);
                    ModelModule::set_mesh_visibility(boma, Hash40::new("rockman_ex_bomb"), false);
                };
            }
            else {
                if MotionModule::frame(boma) >= 1.0 && MotionModule::frame(boma) <= 13.0 {
                    ModelModule::set_mesh_visibility(boma, Hash40::new("rockman_ex_box"), false);
                    ModelModule::set_mesh_visibility(boma, Hash40::new("rockman_ex_box2"), false);
                    ModelModule::set_mesh_visibility(boma, Hash40::new("rockman_ex_bomb"), true);
                }
                else {
                    ModelModule::set_mesh_visibility(boma, Hash40::new("rockman_ex_box"), false);
                    ModelModule::set_mesh_visibility(boma, Hash40::new("rockman_ex_box2"), false);
                    ModelModule::set_mesh_visibility(boma, Hash40::new("rockman_ex_bomb"), false);
                };
            };
        }
        else {
            ModelModule::set_mesh_visibility(boma, Hash40::new("rockman_ex_box"), false);
            ModelModule::set_mesh_visibility(boma, Hash40::new("rockman_ex_box2"), false);
            ModelModule::set_mesh_visibility(boma, Hash40::new("rockman_ex_bomb"), false);
        };
        if [hash40("throw_hi")].contains(&MotionModule::motion_kind(boma)) {
            if PostureModule::lr(boma) == 1.0 {
                if MotionModule::frame(boma) >= 5.0 && MotionModule::frame(boma) <= 25.0 {
                    ModelModule::set_mesh_visibility(boma, Hash40::new("rockman_ex_bumper"), true);
                    ModelModule::set_mesh_visibility(boma, Hash40::new("rockman_ex_bumperb"), false);
                }
                else {
                    ModelModule::set_mesh_visibility(boma, Hash40::new("rockman_ex_bumper"), false);
                    ModelModule::set_mesh_visibility(boma, Hash40::new("rockman_ex_bumperb"), false);
                };
            };
            if PostureModule::lr(boma) == -1.0 {
                if MotionModule::frame(boma) >= 5.0 && MotionModule::frame(boma) <= 25.0 {
                    ModelModule::set_mesh_visibility(boma, Hash40::new("rockman_ex_bumper"), false);
                    ModelModule::set_mesh_visibility(boma, Hash40::new("rockman_ex_bumperb"), true);
                }
                else {
                    ModelModule::set_mesh_visibility(boma, Hash40::new("rockman_ex_bumper"), false);
                    ModelModule::set_mesh_visibility(boma, Hash40::new("rockman_ex_bumperb"), false);
                };
            };
        }
        else {
            ModelModule::set_mesh_visibility(boma, Hash40::new("rockman_ex_bumper"), false);
            ModelModule::set_mesh_visibility(boma, Hash40::new("rockman_ex_bumperb"), false);
        };
    };
}