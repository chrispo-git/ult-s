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

pub fn install() {
    Agent::new("pacman")
    .on_line(Exec, bomber_main_frame)
    .on_line(Main, bomberman_frame)
    .install();

    Agent::new("pacman_firehydrant")
    .on_line(Main, bomb_frame)
    .install();
}
//This sideb code may be the worst code i've ever written. I am ashamed.
unsafe extern "C" fn bomber_main_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent); 
		let status_kind = smash::app::lua_bind::StatusModule::status_kind(boma);
		let motion_kind = MotionModule::motion_kind(boma);
		let frame = MotionModule::frame(boma);
		let ENTRY_ID = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
		let situation_kind = StatusModule::situation_kind(boma);
		let end_frame = MotionModule::end_frame(boma);
		let cancel_frame = FighterMotionModuleImpl::get_cancel_frame(boma,smash::phx::Hash40::new_raw(MotionModule::motion_kind(boma)),false) as f32;
        let lr = PostureModule::lr(boma);
        let is_ground = situation_kind == *SITUATION_KIND_GROUND;
        let is_end = MotionModule::is_end(fighter.module_accessor);
        let is_near_ground = GroundModule::ray_check(fighter.module_accessor, &Vector2f{ x: PostureModule::pos_x(fighter.module_accessor), y: PostureModule::pos_y(fighter.module_accessor)}, &Vector2f{ x: 0.0, y: -1.0}, true);
        let is_kinda_near = GroundModule::ray_check(fighter.module_accessor, &Vector2f{ x: PostureModule::pos_x(fighter.module_accessor), y: PostureModule::pos_y(fighter.module_accessor)}, &Vector2f{ x: 0.0, y: -2.0}, true);
        if is_added(boma) {
            if motion_kind == hash40("special_s") {
                //println!("in special S, {}", frame);
                if frame >= 47.0 {
                    //println!("special s endless fall");
                    MotionModule::set_rate(boma, 0.0);
                    macros::SET_SPEED_EX(fighter, 0.0, -2.6, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
                }
                damage!(fighter, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, /*Type*/ *DAMAGE_NO_REACTION_MODE_ALWAYS, /*DamageThreshold*/ 0);
                if ((frame > 26.0 && SPEED_Y[ENTRY_ID] < 0.0 && is_near_ground == 1 ) || (is_kinda_near == 1 && frame > 39.0)) {
                    println!("special s land");
                    StatusModule::set_situation_kind(boma, smash::app::SituationKind(*SITUATION_KIND_AIR), true);
                    StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_CATCH_CUT, true);
                    StatusModule::set_keep_situation_air(boma, true);
                }
            }
            if SIDEB_CATCH[ENTRY_ID] {
                if status_kind == *FIGHTER_STATUS_KIND_CATCH {
                    if ![hash40("special_s_dash"), hash40("special_air_s_dash"), hash40("special_s_miss"), hash40("special_air_s_miss")].contains(&motion_kind) {
                        if is_ground {
                            MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_s_dash"), -1.0, 1.0, false, 0.0, false, false);
                        } else {
                            MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_s_dash"), -1.0, 1.0, false, 0.0, false, false);
                            CAN_SIDEB[ENTRY_ID] = 1;
                        }
                    } else if is_end && ![hash40("special_s_miss"), hash40("special_air_s_miss")].contains(&motion_kind){
                        if is_ground {
                            MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_s_miss"), -1.0, 1.0, false, 0.0, false, false);
                        } else {
                            MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_s_miss"), -1.0, 1.0, false, 0.0, false, false);
                        }
                    }
                }
                if status_kind == *FIGHTER_STATUS_KIND_CATCH_PULL {
                    if ![hash40("special_s_pull"), hash40("special_air_s_pull"), hash40("special_s"), hash40("special_s_end")].contains(&motion_kind) {
                        println!("switch to special_s");
                        MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_s"), -1.0, 1.0, false, 0.0, false, false);
                    }
                    println!("Hello! I am in catch pull status! Here is my Motion Kind {}", motion_kind);
                }
                if motion_kind == hash40("special_s_end") && frame > 25.0 {
                    StatusModule::set_keep_situation_air(boma, false);
                    if situation_kind == *SITUATION_KIND_GROUND || (is_near_ground == 1 && SPEED_Y[ENTRY_ID] <= 0.0) {
                        StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_LANDING, false);
                    }
                }
                if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_S {
                    if ![hash40("special_s_start"), hash40("special_air_s_start")].contains(&motion_kind) {
                        if is_ground {
                            MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_s_start"), -1.0, 1.0, false, 0.0, false, false);
                        } else {
                            MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_s_start"), -1.0, 1.0, false, 0.0, false, false);
                        }
                    }
                    macros::SET_SPEED_EX(fighter, 0.0, 0.0, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
                }
                if status_kind == *FIGHTER_STATUS_KIND_THROW_KIRBY {
                    if ![hash40("special_s"), hash40("special_s_end")].contains(&motion_kind) {
                        //if FORCE_END[ENTRY_ID] {
                            MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_s_end"), -1.0, 1.0, false, 0.0, false, false);
                            let opponent_id = LinkModule::get_node_object_id(boma, *LINK_NO_CAPTURE) as u32;
	                        let grabber_boma = smash::app::sv_battle_object::module_accessor(opponent_id);
                            ControlModule::set_clatter_time(grabber_boma, 0.0, 0);
                            ControlModule::set_dec_time(grabber_boma, 0.0, 0);
                            KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_CAPTURE_JUMP);
                            FORCE_END[ENTRY_ID] = false;
                        //}
                    }
                }
            }
            if [hash40("special_s_end")].contains(&motion_kind) {
                if is_end || (is_near_ground == 1 && SPEED_Y[ENTRY_ID] <= 0.0 && frame > 26.0) {
                    StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_FALL, false);
                }
                if frame >= cancel_frame {
                    reimpl_cancel_frame(fighter);
                }
            }
            if [hash40("special_s")].contains(&motion_kind) && is_end {
                macros::SET_SPEED_EX(fighter, 0.0, -1.5, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
            }
            if [hash40("special_s_miss")].contains(&motion_kind) && end_frame-frame <= 3.0 {
                StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_WAIT, false);
                CAN_GRAB[ENTRY_ID] = 1;
            }
            if situation_kind != *SITUATION_KIND_AIR {
                CAN_SIDEB[ENTRY_ID] = 0;
            }
        }
    }
}
unsafe extern "C" fn bomberman_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent); 
		let status_kind = smash::app::lua_bind::StatusModule::status_kind(boma);
		let motion_kind = MotionModule::motion_kind(boma);
		let frame = MotionModule::frame(boma);
		let ENTRY_ID = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
		let situation_kind = StatusModule::situation_kind(boma);
        let is_ground = situation_kind == *SITUATION_KIND_GROUND;
		let end_frame = MotionModule::end_frame(boma);
		let cancel_frame = FighterMotionModuleImpl::get_cancel_frame(boma,smash::phx::Hash40::new_raw(MotionModule::motion_kind(boma)),false) as f32;
        let lr = PostureModule::lr(boma);
        if is_added(boma) {
            if ModelModule::scale(boma) == WorkModule::get_param_float(fighter.module_accessor, hash40("scale"), 0) {
                ModelModule::set_scale(boma, 0.94);
                GrabModule::set_size_mul(boma, 0.94);
            }
            if !ArticleModule::is_exist(boma, *FIGHTER_PACMAN_GENERATE_ARTICLE_FIREHYDRANT) {
                EXPLODE[ENTRY_ID] = false;
            } else {
                if [
                    *FIGHTER_STATUS_KIND_ATTACK, *FIGHTER_STATUS_KIND_ATTACK_AIR, *FIGHTER_STATUS_KIND_ATTACK_S3, *FIGHTER_STATUS_KIND_ATTACK_LW3, *FIGHTER_STATUS_KIND_ATTACK_DASH,
                    *FIGHTER_STATUS_KIND_ATTACK_HI3, *FIGHTER_STATUS_KIND_ATTACK_S4, *FIGHTER_STATUS_KIND_ATTACK_HI4, *FIGHTER_STATUS_KIND_ATTACK_LW4
                ].contains(&status_kind) &&  (ControlModule::get_command_flag_cat(boma, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_LW) != 0 {
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_SPECIAL_LW, true);
                }
            }
            if status_kind != *FIGHTER_STATUS_KIND_SPECIAL_LW {
                ModelModule::set_mesh_visibility(boma,Hash40::new("bomber_switchhavel"),false);
                ModelModule::set_mesh_visibility(boma,Hash40::new("bomber_switchhaver"),false);
            } else {
				if StatusModule::situation_kind(boma) == *SITUATION_KIND_AIR {
					let cat2 = ControlModule::get_command_flag_cat(boma, 1);
					if (cat2 & *FIGHTER_PAD_CMD_CAT2_FLAG_FALL_JUMP) != 0 && ControlModule::get_stick_y(boma) < -0.66 && SPEED_Y[ENTRY_ID] <= 0.0 {
						WorkModule::set_flag(boma, true, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_DIVE);
					}
				};
            }
            /*if status_kind == *FIGHTER_PACMAN_STATUS_KIND_FINAL_END && frame > 3.0{
                CameraModule::reset_all(boma);
            }*/
            if [hash40("final_start"), hash40("final_air_start")].contains(&motion_kind) {
                fighter.clear_lua_stack();
                lua_args!(fighter, *FIGHTER_KINETIC_ENERGY_ID_DAMAGE);
                smash::app::sv_kinetic_energy::clear_speed(fighter.lua_state_agent);
                KineticModule::clear_speed_all(fighter.module_accessor);


                if frame >= end_frame-3.0 {
                    notify_event_msc_cmd!(fighter, Hash40::new_raw(0x1e0aba2d68));
                    if is_ground {
                        StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_WAIT, true);
                    } else {
                        StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_FALL, true);
                    }
                }
            }
            if ![*FIGHTER_STATUS_KIND_SPECIAL_N, *FIGHTER_STATUS_KIND_ATTACK_HI4, *FIGHTER_STATUS_KIND_ATTACK_HI4_START, *FIGHTER_STATUS_KIND_ATTACK_HI4_HOLD].contains(&status_kind) && 
            ![hash40("cliff_attack_quick"), hash40("appeal_s_r"), hash40("appeal_s_l")].contains(&motion_kind) {
                ModelModule::set_mesh_visibility(boma,Hash40::new("bomber_boxing_l"),false);
                ModelModule::set_mesh_visibility(boma,Hash40::new("bomber_handl"),true);
                ModelModule::set_mesh_visibility(boma,Hash40::new("bomber_boxing_r"),false);
                ModelModule::set_mesh_visibility(boma,Hash40::new("bomber_handr"),true);
            } else {
                let mut left_right_mul = 1.0;
                if [*FIGHTER_STATUS_KIND_ATTACK_HI4, *FIGHTER_STATUS_KIND_ATTACK_HI4_START, *FIGHTER_STATUS_KIND_ATTACK_HI4_HOLD].contains(&status_kind) {
                    left_right_mul *= -1.0;
                }
                if lr*left_right_mul > 0.0 {
                    ModelModule::set_mesh_visibility(boma,Hash40::new("bomber_boxing_r"),true);
                    ModelModule::set_mesh_visibility(boma,Hash40::new("bomber_handr"),false);
                    ModelModule::set_mesh_visibility(boma,Hash40::new("bomber_boxing_l"),false);
                    ModelModule::set_mesh_visibility(boma,Hash40::new("bomber_handl"),true);
                } else {
                    ModelModule::set_mesh_visibility(boma,Hash40::new("bomber_boxing_l"),true);
                    ModelModule::set_mesh_visibility(boma,Hash40::new("bomber_handl"),false);
                    ModelModule::set_mesh_visibility(boma,Hash40::new("bomber_boxing_r"),false);
                    ModelModule::set_mesh_visibility(boma,Hash40::new("bomber_handr"),true);
                }
                if [*FIGHTER_STATUS_KIND_ATTACK_HI4].contains(&status_kind) && frame >= 23.0 {
                    ModelModule::set_mesh_visibility(boma,Hash40::new("bomber_boxing_l"),false);
                    ModelModule::set_mesh_visibility(boma,Hash40::new("bomber_handl"),true);
                    ModelModule::set_mesh_visibility(boma,Hash40::new("bomber_boxing_r"),false);
                    ModelModule::set_mesh_visibility(boma,Hash40::new("bomber_handr"),true);
                }
            }
            if status_kind == *FIGHTER_STATUS_KIND_ATTACK_S4 && lr < 0.0{
                if frame < 14.0 {
                    ModelModule::set_mesh_visibility(boma,Hash40::new("bomber_bombhavel"),true);
                    ModelModule::set_mesh_visibility(boma,Hash40::new("bomber_bombhaver"),false);
                    BOMB_TO_REMOVE[ENTRY_ID] = true;
                } else {
                    ModelModule::set_mesh_visibility(boma,Hash40::new("bomber_bombhavel"),false);
                    ModelModule::set_mesh_visibility(boma,Hash40::new("bomber_bombhaver"),false);
                    BOMB_TO_REMOVE[ENTRY_ID] = false;
                }
            } else if [hash40("throw_f")].contains(&motion_kind) && lr < 0.0{
                if frame < 31.0 && frame >= 3.0 {
                    ModelModule::set_mesh_visibility(boma,Hash40::new("bomber_bombhaver"),true);
                    ModelModule::set_mesh_visibility(boma,Hash40::new("bomber_bombhavel"),false);
                    BOMB_TO_REMOVE[ENTRY_ID] = true;
                } else {
                    ModelModule::set_mesh_visibility(boma,Hash40::new("bomber_bombhavel"),false);
                    ModelModule::set_mesh_visibility(boma,Hash40::new("bomber_bombhaver"),false);
                    BOMB_TO_REMOVE[ENTRY_ID] = false;
                }
            } else if [hash40("throw_lw")].contains(&motion_kind) && lr < 0.0{
                if frame < 30.0 && frame >= 9.0 {
                    ModelModule::set_mesh_visibility(boma,Hash40::new("bomber_bombhavel"),true);
                    ModelModule::set_mesh_visibility(boma,Hash40::new("bomber_bombhaver"),false);
                    BOMB_TO_REMOVE[ENTRY_ID] = true;
                } else {
                    ModelModule::set_mesh_visibility(boma,Hash40::new("bomber_bombhavel"),false);
                    ModelModule::set_mesh_visibility(boma,Hash40::new("bomber_bombhaver"),false);
                    BOMB_TO_REMOVE[ENTRY_ID] = false;
                }
            } else if BOMB_TO_REMOVE[ENTRY_ID] {
                ModelModule::set_mesh_visibility(boma,Hash40::new("bomber_bombhavel"),false);
                ModelModule::set_mesh_visibility(boma,Hash40::new("bomber_bombhaver"),false);
                BOMB_TO_REMOVE[ENTRY_ID] = false;
            }
            
            if is_reset() || status_kind == *FIGHTER_STATUS_KIND_DEAD {
                NEUTRALB_CHARGE[ENTRY_ID] = 0;
            }
            if [hash40("special_n_hold"), hash40("special_air_n_hold")].contains(&motion_kind) {
                MotionModule::set_rate(boma, 0.5);
            }
            if status_kind == *FIGHTER_PACMAN_STATUS_KIND_SPECIAL_HI_LOOP {
                if KineticModule::get_kinetic_type(boma) != *FIGHTER_KINETIC_TYPE_FALL {
                    fighter.clear_lua_stack();
                    lua_args!(fighter, *FIGHTER_KINETIC_ENERGY_ID_DAMAGE);
                    smash::app::sv_kinetic_energy::clear_speed(fighter.lua_state_agent);
                    KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_FALL);
                };
                if ControlModule::get_stick_y(boma) > -0.5 {
                    if GroundModule::can_entry_cliff(boma) == 1 && WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_CLIFF_COUNT) < 7 && WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_CLIFF_NO_CATCH_FRAME) < 1 {
                        fighter.change_status(FIGHTER_STATUS_KIND_CLIFF_CATCH_MOVE.into(),false.into());
                    };
                }
                if frame as i32 == 5 {
                    if ControlModule::get_stick_x(boma)*lr < -0.5 {
                        PostureModule::reverse_lr(boma);
                        PostureModule::update_rot_y_lr(boma);
                    }
                }
            }
            WorkModule::on_flag(boma, *FIGHTER_PACMAN_INSTANCE_WORK_ID_FLAG_SPECIAL_N_REMOVE_ITEM);
            if ![*FIGHTER_STATUS_KIND_THROW_KIRBY, *FIGHTER_STATUS_KIND_CATCH_PULL, *FIGHTER_STATUS_KIND_CATCH, *FIGHTER_STATUS_KIND_SPECIAL_S, *FIGHTER_STATUS_KIND_CATCH_CUT].contains(&status_kind) {
                if SIDEB_CATCH[ENTRY_ID] {
                    println!("sideb catch over");
                }
                SIDEB_CATCH[ENTRY_ID] = false;
                FORCE_END[ENTRY_ID] = false;
                CAN_GRAB[ENTRY_ID] = 0;
            }
            if StatusModule::prev_status_kind(fighter.module_accessor, 2) == *FIGHTER_STATUS_KIND_SPECIAL_S  && StatusModule::prev_status_kind(fighter.module_accessor, 0) == *FIGHTER_STATUS_KIND_CATCH_PULL  && status_kind == *FIGHTER_STATUS_KIND_CATCH_CUT{
                if motion_kind != hash40("special_s_end") {
                    println!("sideb end");
                    MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_s_end"), -1.0, 1.0, false, 0.0, false, false);
                    let opponent_id = LinkModule::get_node_object_id(boma, *LINK_NO_CAPTURE) as u32;
                    let grabber_boma = smash::app::sv_battle_object::module_accessor(opponent_id);
                    ControlModule::set_clatter_time(grabber_boma, 0.0, 0);
                    ControlModule::set_dec_time(grabber_boma, 0.0, 0);
                    KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_CAPTURE_JUMP);
                    FORCE_END[ENTRY_ID] = false;
                }
            }
        }
    }
}
unsafe extern "C" fn bomb_frame(weapon: &mut L2CFighterBase) {
    unsafe {
        let otarget_id = WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER) as u32;
        let boma = smash::app::sv_battle_object::module_accessor(otarget_id);
        let woma = smash::app::sv_system::battle_object_module_accessor(weapon.lua_state_agent); 
		let status_kind = smash::app::lua_bind::StatusModule::status_kind(weapon.module_accessor);
        let situation = StatusModule::situation_kind(weapon.module_accessor);
		let ENTRY_ID = WorkModule::get_int(&mut *boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
		let is_near_ground = GroundModule::ray_check(weapon.module_accessor, &Vector2f{ x: PostureModule::pos_x(weapon.module_accessor), y: PostureModule::pos_y(weapon.module_accessor)}, &Vector2f{ x: 0.0, y: -1.0}, true);
        if smash::app::utility::get_kind(&mut *boma) == *FIGHTER_KIND_PACMAN  && is_added(&mut *boma) {
			ModelModule::set_scale(weapon.module_accessor, 0.769);
            if MAKE_NEW_BOMB[ENTRY_ID] {
                MAKE_NEW_BOMB[ENTRY_ID] = false;
                let pos = smash::phx::Vector3f { x: NEW_BOMB_X[ENTRY_ID], y: NEW_BOMB_Y[ENTRY_ID]+4.0, z: 0.0 };
                PostureModule::set_pos(weapon.module_accessor, &pos);
                PostureModule::init_pos(weapon.module_accessor, &pos, true, true);
                println!("New Bombed");
            }
            if EXPLODE_END_TIMER[ENTRY_ID] == 0 {
                if status_kind == *WEAPON_PACMAN_FIREHYDRANT_STATUS_KIND_FLY && (situation == *SITUATION_KIND_GROUND){
                    MAKE_NEW_BOMB[ENTRY_ID] = true;
                    NEW_BOMB_X[ENTRY_ID] = PostureModule::pos_x(weapon.module_accessor);
                    NEW_BOMB_Y[ENTRY_ID] = PostureModule::pos_y(weapon.module_accessor);
                    ArticleModule::generate_article(&mut *boma, *FIGHTER_PACMAN_GENERATE_ARTICLE_FIREHYDRANT, false, -1);
                    println!("End Bombed");
                }
            }
            if EXPLODE_END_TIMER[ENTRY_ID] > 0{
                EXPLODE_END_TIMER[ENTRY_ID] -= 1;
            }
            if EXPLODE_END_TIMER[ENTRY_ID] == 1{
                EXPLODE[ENTRY_ID] = false;
                StatusModule::change_status_request_from_script(weapon.module_accessor, *WEAPON_PACMAN_FIREHYDRANT_STATUS_KIND_REMOVE, false);
            }
            if EXPLODE_END_TIMER[ENTRY_ID] == 15{
                AttackModule::clear_all(weapon.module_accessor);
            }
            if EXPLODE_END_TIMER[ENTRY_ID] == 0 && (AttackModule::is_infliction_status(weapon.module_accessor, *COLLISION_KIND_MASK_ALL) || EXPLODE[ENTRY_ID] == true) {
                KineticModule::clear_speed_all(weapon.module_accessor);
                EXPLODE_END_TIMER[ENTRY_ID] = 19;
                AttackModule::clear_all(weapon.module_accessor);
                VisibilityModule::set_model_visible(weapon.module_accessor, false);
                macros::EFFECT(weapon, Hash40::new("sys_bomb_a"), Hash40::new("rot"), 0, 0, 0, 0, 0, 0, 1.25, 0, 0, 0, 0, 0, 0, true);
                macros::ATTACK(weapon, 0, 0, Hash40::new("rot"), 13.0, 45, 90, 0, 30, 13.0, 0.0, 0.0, 0.0, None, None, None, 1.4, 1.4, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_SPEED, false, -3.3, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_OBJECT);
            }
            if smash::app::sv_math::rand(hash40("fighter"), 5) == 0{
                let fire: u32 = EffectModule::req_follow(weapon.module_accessor, smash::phx::Hash40::new("sys_damage_fire"), smash::phx::Hash40::new("rot"), &BOMB, &BOMB, 0.5, true, 0, 0, 0, 0, 0, true, true) as u32;
            }
        };
    }
}