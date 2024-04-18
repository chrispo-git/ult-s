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
use crate::murabito::*;
use super::*;

pub fn install() {
    Agent::new("murabito")
    .on_line(Main, murabito_frame2)
    .on_line(Main, murabito_frame)
    .install();

    Agent::new("murabito_tree")
    .on_line(Main, tree_frame)
    .install();

    Agent::new("murabito_sprout")
    .on_line(Main, seed_frame)
    .install();
}

unsafe extern "C" fn murabito_frame2(fighter: &mut L2CFighterCommon) {
    unsafe {
        let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent); 
		if is_default(boma) {
            let frame = MotionModule::frame(boma);
            let situation_kind = StatusModule::situation_kind(boma);
			let ENTRY_ID = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
			let cancel_frame = FighterMotionModuleImpl::get_cancel_frame(boma,smash::phx::Hash40::new_raw(MotionModule::motion_kind(boma)),false) as f32;
			let motion_kind = MotionModule::motion_kind(boma);
			let status_kind = smash::app::lua_bind::StatusModule::status_kind(boma);
        }
    }
}
unsafe extern "C" fn murabito_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent); 
		if is_default(boma) {
			let status_kind = smash::app::lua_bind::StatusModule::status_kind(boma);
			let frame = MotionModule::frame(boma);
			let ENTRY_ID = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
			let cancel_frame = FighterMotionModuleImpl::get_cancel_frame(boma,smash::phx::Hash40::new_raw(MotionModule::motion_kind(boma)),false) as f32;
			let motion_kind = MotionModule::motion_kind(boma);
            let situation_kind = StatusModule::situation_kind(boma);
            let pos_x = PostureModule::pos_x(boma);
            let pos_y = PostureModule::pos_y(boma);
            let end_frame = MotionModule::end_frame(boma);
            let lr = PostureModule::lr(boma);
            let is_facing_tree = lr*(TREE_POS_X[ENTRY_ID]-pos_x) > 0.0;
            if  [*FIGHTER_MURABITO_STATUS_KIND_SPECIAL_N_SEARCH, *FIGHTER_STATUS_KIND_SPECIAL_N, *FIGHTER_MURABITO_STATUS_KIND_SPECIAL_N_POCKET].contains(&status_kind) {
                if frame >= cancel_frame {
                    CHANGE_FRAME[ENTRY_ID] = false;
                    reimpl_cancel_frame(fighter);
                }
            }
            if [hash40("special_air_n")].contains(&MotionModule::motion_kind(boma)) {
                if !DO_BOUNCE[ENTRY_ID] {
                    if AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_ALL) && !AttackModule::is_infliction(boma, *COLLISION_KIND_MASK_ALL){
                        KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_JUMP);
                        if !AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT) {
                            MotionModule::set_rate(boma, 0.5);
                        };
                        DO_BOUNCE[ENTRY_ID] = true;
                    };
                }
                if SPEED_Y[ENTRY_ID] > 2.0 {
                    let speed = smash::phx::Vector3f { x: 0.0, y: (SPEED_Y[ENTRY_ID]-2.0)*-1.0, z: 0.0 };
                    KineticModule::add_speed(fighter.module_accessor, &speed);
                }
            };
            if situation_kind != *SITUATION_KIND_AIR {
                DO_BOUNCE[ENTRY_ID] = false;
            }
            if  ![*FIGHTER_MURABITO_STATUS_KIND_SPECIAL_N_SEARCH, *FIGHTER_STATUS_KIND_SPECIAL_N, *FIGHTER_MURABITO_STATUS_KIND_SPECIAL_N_POCKET, *FIGHTER_STATUS_KIND_ATTACK_LW4, *FIGHTER_STATUS_KIND_ATTACK_LW4_START, *FIGHTER_STATUS_KIND_ATTACK_LW4_HOLD].contains(&status_kind) {
                ModelModule::set_mesh_visibility(boma,Hash40::new("murabito_shovel"),false);
                ModelModule::set_mesh_visibility(boma,Hash40::new("murabito_shovelflip"),false);
            }
            if WorkModule::get_int(boma, *FIGHTER_MURABITO_INSTANCE_WORK_ID_INT_SPECIAL_N_TIME_LIMIT) > 1 {
                WorkModule::set_int(boma, 1, *FIGHTER_MURABITO_INSTANCE_WORK_ID_INT_SPECIAL_N_TIME_LIMIT);
            }
            if  [*FIGHTER_MURABITO_STATUS_KIND_SPECIAL_N_SEARCH, *FIGHTER_STATUS_KIND_SPECIAL_N, *FIGHTER_MURABITO_STATUS_KIND_SPECIAL_N_POCKET].contains(&status_kind) {

                if StatusModule::is_situation_changed(boma) {
                    CHANGE_FRAME[ENTRY_ID] = false;
                    StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_LANDING, false);
                } else {
                    if HAS_BEEN_AIR[ENTRY_ID] && situation_kind == SITUATION_KIND_GROUND {
                        StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_LANDING, false);
                        HAS_BEEN_AIR[ENTRY_ID] = false;
                    }
                    if frame < 2.0 && situation_kind == *SITUATION_KIND_GROUND && ![hash40("special_n3"), hash40("special_n2_fail"), hash40("special_n2")].contains(&motion_kind) && !CHANGE_FRAME[ENTRY_ID]{
                        if ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_MURABITO_GENERATE_ARTICLE_TREE) && ![hash40("special_n3"), hash40("special_n2_fail")].contains(&motion_kind) {
                            if (TREE_POS_X[ENTRY_ID]-pos_x).abs() < X_DIST && (TREE_POS_Y[ENTRY_ID]-pos_y).abs() < Y_DIST && is_facing_tree && !IS_FALLEN[ENTRY_ID] {
                                MotionModule::change_motion(boma, Hash40::new("special_n3"), 0.0, 1.0, false, 0.0, false, false);
                                println!("special_n3");
                                CHANGE_FRAME[ENTRY_ID] = true;
                            } else {
                                MotionModule::change_motion(boma, Hash40::new("special_n2_fail"), 0.0, 1.0, false, 0.0, false, false);
                                println!("special_n2_fail");
                                CHANGE_FRAME[ENTRY_ID] = true;
                            }
                        } else if ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_MURABITO_GENERATE_ARTICLE_SPROUT) && ![hash40("special_n2"), hash40("special_n2_fail")].contains(&motion_kind) {
                            if (TREE_POS_X[ENTRY_ID]-pos_x).abs() < X_DIST && (TREE_POS_Y[ENTRY_ID]-pos_y).abs() < Y_DIST && is_facing_tree  {
                                MotionModule::change_motion(boma, Hash40::new("special_n2"), 0.0, 1.0, false, 0.0, false, false);
                                println!("special_n2");
                                CHANGE_FRAME[ENTRY_ID] = true;
                            } else {
                                MotionModule::change_motion(boma, Hash40::new("special_n2_fail"), 0.0, 1.0, false, 0.0, false, false);
                                println!("special_n2_fail");
                                CHANGE_FRAME[ENTRY_ID] = true;
                            }
                        } else if motion_kind != hash40("special_n") {
                            MotionModule::change_motion(boma, Hash40::new("special_n"), 0.0, 1.0, false, 0.0, false, false);
                        } else {
                            CHANGE_FRAME[ENTRY_ID] = true;
                        }
                    }
                    if situation_kind != SITUATION_KIND_GROUND && motion_kind != hash40("special_air_n") {
                        MotionModule::change_motion(boma, Hash40::new("special_air_n"), 0.0, 1.0, false, 0.0, false, false);
                        HAS_BEEN_AIR[ENTRY_ID] = true;
                    }
                    if [hash40("special_n3")].contains(&motion_kind) {
                        if (!ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_MURABITO_GENERATE_ARTICLE_TREE) || IS_FALLEN[ENTRY_ID] ) {
                            StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_SLIP, false);
                        }
                        ModelModule::set_mesh_visibility(boma,Hash40::new("murabito_shovel"),false);
                        ModelModule::set_mesh_visibility(boma,Hash40::new("murabito_shovelflip"),false);
                    }
                    if frame >= end_frame {
                        if StatusModule::situation_kind(boma) == *SITUATION_KIND_AIR {
                            StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_FALL, false);
                        } else {
                            StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_WAIT, false);
                        };
                    }
                    if frame > 4.0 {
                        CHANGE_FRAME[ENTRY_ID] = false;
                    }
                    if frame >= cancel_frame {
                        CHANGE_FRAME[ENTRY_ID] = false;
                        reimpl_cancel_frame(fighter);
                    }
                }
            } else {
                CHANGE_FRAME[ENTRY_ID] = false;
                HAS_BEEN_AIR[ENTRY_ID] = false;
            }
		}
    }
}
unsafe extern "C" fn tree_frame(weapon: &mut L2CFighterBase) {
    unsafe {
        let otarget_id = WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER) as u32;
		let status_kind = smash::app::lua_bind::StatusModule::status_kind(weapon.module_accessor);
        let boma = smash::app::sv_battle_object::module_accessor(otarget_id);
		let ENTRY_ID = WorkModule::get_int(&mut *boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
        if smash::app::utility::get_kind(&mut *boma) == *FIGHTER_KIND_MURABITO && is_default(&mut *boma) {
            if status_kind == *WEAPON_MURABITO_TREE_STATUS_KIND_STAND || status_kind == *WEAPON_MURABITO_TREE_STATUS_KIND_APPEAR {
                IS_FALLEN[ENTRY_ID] = false;
            } else {
                IS_FALLEN[ENTRY_ID] = true;
            }
            TREE_POS_X[ENTRY_ID] = PostureModule::pos_x(weapon.module_accessor);
            TREE_POS_Y[ENTRY_ID] = PostureModule::pos_y(weapon.module_accessor);
		};
    }
}
unsafe extern "C" fn seed_frame(weapon: &mut L2CFighterBase) {
    unsafe {
        let otarget_id = WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER) as u32;
		let status_kind = smash::app::lua_bind::StatusModule::status_kind(weapon.module_accessor);
        let boma = smash::app::sv_battle_object::module_accessor(otarget_id);
		let ENTRY_ID = WorkModule::get_int(&mut *boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
        if smash::app::utility::get_kind(&mut *boma) == *FIGHTER_KIND_MURABITO && is_default(&mut *boma) {
            TREE_POS_X[ENTRY_ID] = PostureModule::pos_x(weapon.module_accessor);
            TREE_POS_Y[ENTRY_ID] = PostureModule::pos_y(weapon.module_accessor);
		};
    }
}
