use smash::hash40;
use smash::lib::lua_const::*;
use smash::lua2cpp::*;
use smash::app::lua_bind::*;
use smashline::*;
use smash_script::*;
use crate::common::*;
use crate::util::*;

unsafe extern "C" fn training(fighter : &mut L2CFighterCommon) {
    unsafe {
        let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);  
        let fighter_kind = smash::app::utility::get_kind(boma);
		let cancel_frame = FighterMotionModuleImpl::get_cancel_frame(boma,smash::phx::Hash40::new_raw(MotionModule::motion_kind(boma)),false) as f32; //Cancel frame
		let frame = MotionModule::frame(boma);
        let motion_kind = MotionModule::motion_kind(boma);
        let status_kind = StatusModule::status_kind(boma);
        let prev_status = StatusModule::prev_status_kind(boma, 0);
        let situation_kind = StatusModule::situation_kind(boma);
		let ENTRY_ID = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
        let lr = PostureModule::lr(boma);
        let fallspeed = WorkModule::get_param_float(boma, hash40("common"), hash40("air_speed_y_stable"));
        let airspeed = WorkModule::get_param_float(boma, hash40("common"), hash40("air_speed_x_stable"));
        let gravity = WorkModule::get_param_float(boma, hash40("common"), hash40("air_accel_y"));
        let mut speed_after_20_frames = gravity*20.0;
        if speed_after_20_frames > fallspeed {
            speed_after_20_frames = fallspeed;
        }
        if is_reset() {
            LEDGE_OPTION = 0;
            LEDGE_OPTION_AFTER = 0;
            DJ_DELAY[ENTRY_ID] = 0;
            LEDGE_DELAY[ENTRY_ID] = 0;
        }
        if !smash::app::smashball::is_training_mode() {
            IS_GLOW = false;
            DI_DIR = 0;
            LEDGE_OPTION = 0;
            LEDGE_OPTION_AFTER = 0;
        } else {
            if ControlModule::check_button_on_trriger(boma, *CONTROL_PAD_BUTTON_APPEAL_LW) || ([hash40("appeal_lw_l"),hash40("appeal_lw_r")].contains(&motion_kind) && frame < 2.0) {
                if IS_GLOW {
                    IS_GLOW = false;
                } else {
                    IS_GLOW = true;
                };
            };
            if IS_GLOW { 
                if ControlModule::check_button_on_trriger(boma, *CONTROL_PAD_BUTTON_APPEAL_S_L) {
                    DI_DIR = -1;
                }
                if ControlModule::check_button_on_trriger(boma, *CONTROL_PAD_BUTTON_APPEAL_S_R) {
                    DI_DIR = 1;
                }
                if ControlModule::check_button_on_trriger(boma, *CONTROL_PAD_BUTTON_APPEAL_HI) {
                    DI_DIR = 0;
                }
            } else {
                DI_DIR = 0;
            }
        };
        if LEDGE_DELAY[ENTRY_ID] > 0 {
            LEDGE_DELAY[ENTRY_ID] -= 1;
        }
        if DJ_DELAY[ENTRY_ID] > 0 {
            DJ_DELAY[ENTRY_ID] -= 1;
        }
        if IS_GLOW {
                //Ledge Stuff 
                let ftilt_list = [
                    *FIGHTER_KIND_YOSHI, *FIGHTER_KIND_DAISY, *FIGHTER_KIND_KOOPA,
                    *FIGHTER_KIND_SHEIK, *FIGHTER_KIND_PICHU, *FIGHTER_KIND_PZENIGAME,
                    *FIGHTER_KIND_WIIFIT, *FIGHTER_KIND_KOOPAJR, *FIGHTER_KIND_SIMON, 
                    *FIGHTER_KIND_BUDDY, *FIGHTER_KIND_PICKEL
                ];
                let dtilt_list = [
                    *FIGHTER_KIND_MARIO, *FIGHTER_KIND_DONKEY, *FIGHTER_KIND_SAMUS, *FIGHTER_KIND_SAMUSD, *FIGHTER_KIND_LUIGI,
                    *FIGHTER_KIND_NESS, *FIGHTER_KIND_PEACH, *FIGHTER_KIND_ZELDA, *FIGHTER_KIND_MARIOD, *FIGHTER_KIND_MARTH,
                    *FIGHTER_KIND_YOUNGLINK, *FIGHTER_KIND_MEWTWO, *FIGHTER_KIND_GAMEWATCH, *FIGHTER_KIND_METAKNIGHT,
                    *FIGHTER_KIND_PIT, *FIGHTER_KIND_PITB, *FIGHTER_KIND_WARIO, *FIGHTER_KIND_PFUSHIGISOU, *FIGHTER_KIND_DIDDY,
                    *FIGHTER_KIND_LUCAS, *FIGHTER_KIND_DEDEDE, *FIGHTER_KIND_LUCARIO, *FIGHTER_KIND_ROBOT, *FIGHTER_KIND_TOONLINK,
                    *FIGHTER_KIND_WOLF, *FIGHTER_KIND_ROCKMAN, *FIGHTER_KIND_GEKKOUGA, *FIGHTER_KIND_PALUTENA, *FIGHTER_KIND_DUCKHUNT,
                    *FIGHTER_KIND_RYU, *FIGHTER_KIND_KEN, *FIGHTER_KIND_CLOUD, *FIGHTER_KIND_BAYONETTA, *FIGHTER_KIND_INKLING,
                    *FIGHTER_KIND_RICHTER, *FIGHTER_KIND_JACK, *FIGHTER_KIND_DOLLY, *FIGHTER_KIND_TANTAN, *FIGHTER_KIND_EDGE,
                    *FIGHTER_KIND_TRAIL
                ];
                if ENTRY_ID > 0 && ![*FIGHTER_KIND_POPO, *FIGHTER_KIND_NANA].contains(&fighter_kind) {

                    
                    if (*FIGHTER_STATUS_KIND_DAMAGE..*FIGHTER_STATUS_KIND_DAMAGE_FALL).contains(&status_kind) || status_kind == *FIGHTER_STATUS_KIND_CLIFF_CATCH {
                        DJ_DELAY[ENTRY_ID] = 0;
                        LEDGE_DELAY[ENTRY_ID] = 0;
                        LEDGE_OPTION = 0;
                        LEDGE_OPTION_AFTER = 0;
                    }

                    if WorkModule::is_enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_CLIFF_ATTACK) && LEDGE_DELAY[ENTRY_ID] == 0  && DJ_DELAY[ENTRY_ID] == 0{ //If can do ledge option
                        LEDGE_OPTION = smash::app::sv_math::rand(hash40("fighter"), 6); //Sets Ledge Option
                        LEDGE_OPTION_AFTER = smash::app::sv_math::rand(hash40("fighter"), 3); //Sets Afterwards Option
                        if LEDGE_OPTION == 1 && LEDGE_OPTION_AFTER == 2 {
                            LEDGE_OPTION_AFTER = 0;
                        }
                        // 0 - Neutral Getup
                        // 1 - Ledge Attack
                        // 2 - Ledge Roll
                        // 3 - Ledge Jump
                        // 4 - Ledge Drop -> Double Jump
                        // 5 - Wait at ledge for 30 more frames
                        if LEDGE_OPTION == 0 {
                            StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_CLIFF_CLIMB, true);
                        } else if LEDGE_OPTION == 1 {
                            StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_CLIFF_ATTACK, true);
                        } else if LEDGE_OPTION == 2 {
                            StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_CLIFF_ESCAPE, true);
                        } else if LEDGE_OPTION == 3 {
                            StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_CLIFF_JUMP1, true);
                        } else if LEDGE_OPTION == 4 {
                            DJ_DELAY[ENTRY_ID] = 20;
                            StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_FALL, true);
                            macros::SET_SPEED_EX(fighter, -airspeed, 0.0, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
                        } else if LEDGE_OPTION == 5 {
                            LEDGE_DELAY[ENTRY_ID] = DELAY_FRAMES;
                        }
                    }
                    if LEDGE_OPTION == 4 {
                        if DJ_DELAY[ENTRY_ID] <= 6 {
                            ControlModule::set_main_stick_x(boma, lr);
                        } else {
                            ControlModule::set_main_stick_x(boma, -lr);
                        }
                        if DJ_DELAY[ENTRY_ID] == 5 {
                            if WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT) < WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT_MAX) {
                               StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_JUMP_AERIAL, true);
                            } else {
                                LEDGE_OPTION = 0;
                            }
                        }
                    }
                    if LEDGE_OPTION_AFTER != 0 && DJ_DELAY[ENTRY_ID] == 0 {
                        if LEDGE_OPTION_AFTER == 1 {
                            if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ESCAPE_AIR) ||
                            WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_GUARD_ON) {
                                if situation_kind == *SITUATION_KIND_GROUND {
                                    StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_GUARD, true);
                                } else {
                                    StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ESCAPE_AIR, true);
                                }
                                LEDGE_OPTION_AFTER = 0;
                            }
                        } else if LEDGE_OPTION_AFTER == 2 {
                            if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ESCAPE_AIR) ||
                            WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_GUARD_ON) {
                                if situation_kind == *SITUATION_KIND_GROUND {
                                    if  ftilt_list.contains(&fighter_kind) {
                                        StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_S3, true);
                                    } else if  dtilt_list.contains(&fighter_kind) {
                                        StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_LW3, true);
                                    } else {
                                        StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK, true);
                                    }
                                } else {
                                    StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_AIR, true);
                                }
                                LEDGE_OPTION_AFTER = 0;
                            }
                        }
                    }
                }

                //Glow Stuff
                macros::COL_NORMAL(fighter);
                //Resets glow so it doesnt stack after each frame
                let banned_check = ![
                    *FIGHTER_STATUS_KIND_SQUAT, *FIGHTER_STATUS_KIND_SQUAT_WAIT, *FIGHTER_STATUS_KIND_SQUAT_RV, *FIGHTER_STATUS_KIND_DASH, 
                    *FIGHTER_STATUS_KIND_TURN_DASH, *FIGHTER_STATUS_KIND_TURN_RUN_BRAKE, *FIGHTER_STATUS_KIND_RUN_BRAKE, 
                    *FIGHTER_STATUS_KIND_GUARD, *FIGHTER_STATUS_KIND_GUARD_ON, *FIGHTER_STATUS_KIND_GUARD_OFF, *FIGHTER_STATUS_KIND_GUARD_DAMAGE,
                    *FIGHTER_STATUS_KIND_WAIT
                ].contains(&status_kind);    //These statuses do not need to glow for endlag
                if WorkModule::get_float(boma, *FIGHTER_INSTANCE_WORK_ID_FLOAT_DAMAGE_REACTION_FRAME) > 0.0 && (*FIGHTER_STATUS_KIND_DAMAGE..*FIGHTER_STATUS_KIND_DAMAGE_FALL).contains(&status_kind) {
                    macros::FLASH(fighter, 2.52, 1.86, 0.03, 0.7);
                    LEDGE_OPTION_AFTER = 0;
                }else if ((cancel_frame > 0.0 && frame >= cancel_frame) || (CancelModule::is_enable_cancel(boma))) && !(*FIGHTER_STATUS_KIND_DAMAGE..*FIGHTER_STATUS_KIND_DAMAGE_FALL).contains(&status_kind) && banned_check {
                     macros::FLASH(fighter, 1.59, 0.0, 2.22, 0.7);
                } else if status_kind == *FIGHTER_STATUS_KIND_GUARD_OFF && frame <=  4.0 {
                    macros::FLASH(fighter, 0.31, 2.01, 2.07, 0.7);
                };



                //After Hitstun
                if ENTRY_ID > 0 {
                    if 
                        (*FIGHTER_STATUS_KIND_DAMAGE..*FIGHTER_STATUS_KIND_DAMAGE_FALL).contains(&status_kind) || 
                        (
                            (*FIGHTER_STATUS_KIND_DAMAGE..*FIGHTER_STATUS_KIND_DAMAGE_FALL).contains(&prev_status) && 
                            ![*FIGHTER_STATUS_KIND_GUARD, *FIGHTER_STATUS_KIND_GUARD_OFF, *FIGHTER_STATUS_KIND_GUARD_ON, *FIGHTER_STATUS_KIND_ESCAPE_AIR, *FIGHTER_STATUS_KIND_GUARD].contains(&status_kind)
                        ) 
                    {
                       if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ESCAPE_AIR) ||
                        WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_GUARD_ON) {
                            if situation_kind == *SITUATION_KIND_GROUND {
                                if ![*FIGHTER_STATUS_KIND_DOWN, *FIGHTER_STATUS_KIND_DOWN_WAIT].contains(&status_kind) {
                                    StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_GUARD, true);
                                }
                            } else {
                                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ESCAPE_AIR, true);
                            }
                       }
                    }
                }
        };
    };
}
pub fn install() {
    Agent::new("fighter")
	.on_line(Main, training)
	.install();
}
