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

unsafe extern "C" fn kirby_rayman_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent); 
		let status_kind = smash::app::lua_bind::StatusModule::status_kind(boma);
		let copy_kind = WorkModule::get_int(boma, *FIGHTER_KIRBY_INSTANCE_WORK_ID_INT_COPY_CHARA);
        let situation_kind = StatusModule::situation_kind(boma);
		let motion_kind = MotionModule::motion_kind(boma);
		let ENTRY_ID = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
        let frame = MotionModule::frame(boma);
        let end_frame = MotionModule::end_frame(boma);
		let cancel_frame = FighterMotionModuleImpl::get_cancel_frame(boma,smash::phx::Hash40::new_raw(MotionModule::motion_kind(boma)),false) as f32;
		let is_near_ground = GroundModule::ray_check(boma, &Vector2f{ x: PostureModule::pos_x(boma), y: PostureModule::pos_y(boma)}, &Vector2f{ x: 0.0, y: -1.0}, true);
        
        if copy_kind == *FIGHTER_KIND_PIKMIN {
            EffectModule::kill_kind(boma, Hash40::new("pikmin_antenna"), false, false);
            EffectModule::kill_kind(boma, Hash40::new("pikmin_antenna_damage"), false, false);
            EffectModule::kill_kind(boma, Hash40::new("pikmin_antenna_damage"), true, true);
            EffectModule::kill_kind(boma, Hash40::new("pikmin_antenna_damage"), false, true);
            EffectModule::kill_kind(boma, Hash40::new("pikmin_antenna_damage"), true, false);

            if situation_kind != *SITUATION_KIND_AIR {
                CAN_NEUTRALB[ENTRY_ID] = 0;
            }
            if status_kind == *FIGHTER_KIRBY_STATUS_KIND_PIKMIN_SPECIAL_N {

                    CAN_NEUTRALB[ENTRY_ID] = 1;
                    if KineticModule::get_kinetic_type(boma) != *FIGHTER_KINETIC_TYPE_MOTION_AIR {
                        KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_MOTION_AIR);
                    }
                    if frame < 111.0 {
                        if GroundModule::is_wall_touch_line(boma, *GROUND_TOUCH_FLAG_SIDE as u32) || (AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_ALL) && WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_HIT_STOP_ATTACK_SUSPEND_FRAME) < 1){
                            if !AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_ALL) {
                                KineticModule::clear_speed_all(fighter.module_accessor);
                                let lr = PostureModule::lr(fighter.module_accessor);
                                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_PASSIVE_WALL_JUMP, true);
                                PostureModule::set_lr(fighter.module_accessor, lr*-1.0);
                                PostureModule::update_rot_y_lr(boma);
                                KineticModule::suspend_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
                                macros::QUAKE(fighter, *CAMERA_QUAKE_KIND_L);
                            } else {
                                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_CAPTURE_JUMP, true);
                            }
                        }
                    }
            }
        } else {
            CAN_NEUTRALB[ENTRY_ID] = 0;
        }
    }
}
unsafe extern "C" fn rayman(fighter: &mut L2CFighterCommon) {
    unsafe {
        let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent); 
		let status_kind = smash::app::lua_bind::StatusModule::status_kind(boma);
		let ENTRY_ID = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
        let situation_kind = StatusModule::situation_kind(boma);
        let fighter_kind = smash::app::utility::get_kind(boma);
		let motion_kind = MotionModule::motion_kind(boma);
        let frame = MotionModule::frame(boma);
        let end_frame = MotionModule::end_frame(boma);
		let cancel_frame = FighterMotionModuleImpl::get_cancel_frame(boma,smash::phx::Hash40::new_raw(MotionModule::motion_kind(boma)),false) as f32;
		let is_near_ground = GroundModule::ray_check(boma, &Vector2f{ x: PostureModule::pos_x(boma), y: PostureModule::pos_y(boma)}, &Vector2f{ x: 0.0, y: -1.0}, true);
        let stick_y = ControlModule::get_stick_y(boma);
        let lr = PostureModule::lr(boma);
        let is_ray = (WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) >= 120 && WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) <= 127);

        if is_ray && fighter_kind == *FIGHTER_KIND_PIKMIN { //rayman slots
            if status_kind == *FIGHTER_STATUS_KIND_ENTRY || is_reset() {
                let custom_hurtboxes = [
                    //["bone", x1, y1, z1, x2, y2, z2, scale, collision_part, hit height]
                    [hash40("waist") as f64, 0.2,-0.3, 0.0, 1.4, -0.3, 0.0,   2.2, *COLLISION_PART_BODY as f64, *HIT_HEIGHT_CENTER as f64],
                    [hash40("bag") as f64, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,   0.001, *COLLISION_PART_BODY as f64, *HIT_HEIGHT_CENTER as f64],
                    [hash40("head") as f64, 2.0, 0.5, 0.0, 1.2, 0.5, 0.0,   3.0, *COLLISION_PART_HEAD as f64, *HIT_HEIGHT_HIGH as f64],
                    [hash40("kneer") as f64, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,   0.001, *COLLISION_PART_BODY as f64, *HIT_HEIGHT_CENTER as f64],
                    [hash40("kneel") as f64, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,   0.001, *COLLISION_PART_BODY as f64, *HIT_HEIGHT_CENTER as f64],
                    [hash40("armr") as f64, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,   0.001, *COLLISION_PART_BODY as f64, *HIT_HEIGHT_CENTER as f64],
                    [hash40("arml") as f64, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,   0.001, *COLLISION_PART_BODY as f64, *HIT_HEIGHT_CENTER as f64],
                    [hash40("kneer") as f64, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,   0.001, *COLLISION_PART_BODY as f64, *HIT_HEIGHT_CENTER as f64],
                    [hash40("footr") as f64, 0.4, 0.3, 0.0, 0.4, -1.2, 0.0,  1.2, *COLLISION_PART_BODY_LEGS as f64, *HIT_HEIGHT_LOW as f64],
                    [hash40("footl") as f64, 0.4, 0.3, 0.0, 0.4, -1.2, 0.0,  1.2, *COLLISION_PART_BODY_LEGS as f64, *HIT_HEIGHT_LOW as f64]
                ];
                let mut f = 0;
                for i in custom_hurtboxes {
                    let mut vec1 = Vector3f{x: i[1] as f32, y: i[2] as f32, z: i[3] as f32};
                    let mut vec2 = Vector3f{x: i[4] as f32, y: i[5] as f32, z: i[6] as f32};
                    FighterUtil::set_hit_data(boma,f,0,&vec1,&vec2,i[7] as f32,Hash40::new_raw(i[0] as u64),smash::app::CollisionPart(i[8] as i32),smash::app::HitHeight(i[9] as i32),smash::app::HitStatus(*HIT_STATUS_NORMAL),smash::app::CollisionShapeType(*COLLISION_SHAPE_TYPE_CAPSULE));    
                    f += 1;
                }
            }
            if ModelModule::scale(boma) == WorkModule::get_param_float(fighter.module_accessor, hash40("scale"), 0) {
                ModelModule::set_scale(boma, 1.75);
                AttackModule::set_attack_scale(boma, 1.0, true);
                GrabModule::set_size_mul(boma, 1.75);
            }
            EffectModule::kill_kind(boma, Hash40::new("pikmin_antenna"), false, false);

            EffectModule::kill_kind(boma, Hash40::new("pikmin_antenna_damage"), false, false);
            EffectModule::kill_kind(boma, Hash40::new("pikmin_antenna_damage"), true, true);
            EffectModule::kill_kind(boma, Hash40::new("pikmin_antenna_damage"), false, true);
            EffectModule::kill_kind(boma, Hash40::new("pikmin_antenna_damage"), true, false);

            EffectModule::kill_kind(boma, Hash40::new("pikmin_wingpikmin_end"), false, false);
            EffectModule::kill_kind(boma, Hash40::new("pikmin_wingpikmin_wing"), false, false); 
            EffectModule::kill_kind(boma, Hash40::new("pikmin_wingpikmin2_line"), false, false);

            //Stops final smash sound
            if ![hash40("final"), hash40("final_shoot")].contains(&motion_kind) {
                macros::STOP_SE(fighter, Hash40::new("se_common_spirits_pfog_loop"));
            }

            //Stops Charge sound from playing after rayman is hit
            if ![*FIGHTER_STATUS_KIND_ATTACK_S4_HOLD, *FIGHTER_STATUS_KIND_ATTACK_S4, *FIGHTER_STATUS_KIND_ATTACK_S4_START].contains(&status_kind) {
                macros::STOP_SE(fighter, Hash40::new("se_pikmin_smash_s03"));
            }

            //Star KO Shit
            if status_kind == *FIGHTER_STATUS_KIND_DEAD {
                if MotionModule::motion_kind(boma) == hash40("fall_damage") && !HAS_DEADED[ENTRY_ID] {
                    macros::PLAY_SE(fighter, Hash40::new("se_pikmin_final01"));
                    HAS_DEADED[ENTRY_ID] = true;
                };
            } else {
                HAS_DEADED[ENTRY_ID] = false;
            };

            //Neutralb
            if ![hash40("special_air_n_pull"), hash40("special_n_pull")].contains(&motion_kind){
                DO_WALLJUMP_FORCE[ENTRY_ID] = false;
            }
            if [hash40("special_air_n_failure"), hash40("special_n_failure"), hash40("special_n_start")].contains(&motion_kind){
                StatusModule::set_keep_situation_air(boma, true);
                if is_near_ground == 1 && situation_kind == *SITUATION_KIND_AIR {
                    KineticModule::suspend_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
                    macros::SET_SPEED_EX(fighter, 0.0, 0.0, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
                    if frame > 40.0 {
                        StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_WAIT, false);
                    }
                }
                let mut into_frame = 0.0;
                let mut check_dist = 0.0;
                //Later into the pull anim if you hit neutralb earlier in the shot... because rayman gotta go less distance
                if PULL_DISTANCE[ENTRY_ID] == 0 {
                    into_frame = 7.0;
                    check_dist = 12.0;
                } else if PULL_DISTANCE[ENTRY_ID] == 1 {
                    into_frame = 5.0;
                    check_dist = 23.0;
                } else if PULL_DISTANCE[ENTRY_ID] == 2 {
                    into_frame = 4.0;
                    check_dist = 27.0;
                } else {
                    into_frame = 0.0;
                    check_dist = 32.0;
                }
                //let dist_pos = &mut Vector3f{x: 0.0, y: 0.0, z: check_dist};
                //let joint = ModelModule::joint_global_position(fighter.module_accessor, Hash40::new("throw"), dist_pos, false);
                let is_wall = (
                    (
                        (GroundModule::ray_check(boma, &Vector2f{ x: PostureModule::pos_x(boma)+((6.0+check_dist)*lr), y: PostureModule::pos_y(boma)+10.0}, &Vector2f{ x: 6.0*lr, y: 0.0}, true) == 1)
                    ) ||
                    (
                        (GroundModule::ray_check(boma, &Vector2f{ x: PostureModule::pos_x(boma)+((6.0+(check_dist*0.75))*lr), y: PostureModule::pos_y(boma)+10.0}, &Vector2f{ x: 6.0*lr, y: 0.0}, true) == 1)
                    ) ||
                    (
                        (GroundModule::ray_check(boma, &Vector2f{ x: PostureModule::pos_x(boma)+((6.0+(check_dist*0.5))*lr), y: PostureModule::pos_y(boma)+10.0}, &Vector2f{ x: 6.0*lr, y: 0.0}, true) == 1)
                    ) ||
                    (
                        (GroundModule::ray_check(boma, &Vector2f{ x: PostureModule::pos_x(boma)+((6.0+(check_dist*0.25))*lr), y: PostureModule::pos_y(boma)+10.0}, &Vector2f{ x: 6.0*lr, y: 0.0}, true) == 1)
                    ) ||
                    (
                        (GroundModule::ray_check(boma, &Vector2f{ x: PostureModule::pos_x(boma)+((6.0+(check_dist*0.0))*lr), y: PostureModule::pos_y(boma)+10.0}, &Vector2f{ x: 6.0*lr, y: 0.0}, true) == 1)
                    )
                ) && frame < 22.0 && frame > 13.0;
                //println!("X:{}, Y:{}, Z:{}, is_wall:{}", joint.x, joint.y, joint.z, is_wall);
                if is_wall {
                    DO_WALLJUMP_FORCE[ENTRY_ID] = true;
                }
                if AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT) || is_wall {
                    AttackModule::clear_all(fighter.module_accessor);
                    if situation_kind == *SITUATION_KIND_AIR {
                        MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_n_pull"), into_frame, 0.75, false, 0.0, false, false);
                    } else {
                        MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_n_pull"), into_frame, 0.75, false, 0.0, false, false);
                    }
                }
            }
            if [hash40("special_air_n_pull"), hash40("special_n_pull")].contains(&motion_kind){
                StatusModule::set_keep_situation_air(boma, true);
                if KineticModule::get_kinetic_type(boma) != *FIGHTER_KINETIC_TYPE_MOTION_AIR {
                    KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_MOTION_AIR);
                }
                if GroundModule::is_wall_touch_line(boma, *GROUND_TOUCH_FLAG_SIDE as u32) {
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_PASSIVE_WALL_JUMP, true);
                    CAN_NEUTRALB[ENTRY_ID] = 1;
                }
                if DO_WALLJUMP_FORCE[ENTRY_ID] {
                    let the_speed = smash::phx::Vector3f { x: 0.5, y: 0.0, z: 0.0 };
                    KineticModule::add_speed(boma, &the_speed);
                }
            };
            //Sideb
            if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_S {
                CAN_SIDEB[ENTRY_ID] = 1;
                if motion_kind != hash40("slide_jump_fall") {
                    StatusModule::set_situation_kind(boma, smash::app::SituationKind(*SITUATION_KIND_AIR), true);
                    StatusModule::set_keep_situation_air(boma, true);
                    if KineticModule::get_kinetic_type(boma) != *FIGHTER_KINETIC_TYPE_MOTION_AIR {
                        KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_MOTION_AIR);
                    }
                    if end_frame - frame < 3.0 {
                        MotionModule::change_motion(fighter.module_accessor, Hash40::new("slide_jump_fall"), 0.0, 1.0, false, 0.0, false, false);
                    }
                    if frame < 111.0 {
                        if GroundModule::is_wall_touch_line(boma, *GROUND_TOUCH_FLAG_SIDE as u32) || (AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_ALL) && WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_HIT_STOP_ATTACK_SUSPEND_FRAME) < 1){
                            StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_CAPTURE_JUMP, true);
                            if !AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_ALL) {
                                macros::QUAKE(fighter, *CAMERA_QUAKE_KIND_L);
                            }
                        }
                    }
                } else {
                    StatusModule::set_keep_situation_air(boma, false);
                    if is_near_ground == 1 {
                        StatusModule::set_keep_situation_air(boma, false);
                        StatusModule::set_situation_kind(boma, smash::app::SituationKind(*SITUATION_KIND_GROUND), true);
                        StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_LANDING, true);
                    }
                    if KineticModule::get_kinetic_type(boma) != *FIGHTER_KINETIC_TYPE_FALL {
                        KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_FALL);
                    }
                }
            }
            if situation_kind != *SITUATION_KIND_AIR {
                CAN_SIDEB[ENTRY_ID] = 0;
                CAN_NEUTRALB[ENTRY_ID] = 0;
            }
            if ![hash40("slide_jump_fall"),hash40("capture_jump"),hash40("special_s"),hash40("special_air_s")].contains(&MotionModule::motion_kind(boma)) {
                macros::STOP_SE(fighter, Hash40::new("se_pikmin_attackair_n01"));
            }
            if [hash40("slide_jump_fall")].contains(&MotionModule::motion_kind(boma)) {
                if MotionModule::frame(boma) > 10.0 {
                    macros::STOP_SE(fighter, Hash40::new("se_pikmin_attackair_n01"));
                }
            }
            if [*FIGHTER_STATUS_KIND_ATTACK_AIR, *FIGHTER_STATUS_KIND_FALL_SPECIAL, *FIGHTER_STATUS_KIND_CLIFF_CATCH].contains(&status_kind) || situation_kind == *SITUATION_KIND_GROUND {
                macros::STOP_SE(fighter, Hash40::new("se_pikmin_special_l03"));
            }
            //Slide Stuff
            if WAS_SLIDE[ENTRY_ID] {
                if motion_kind == hash40("fall") || motion_kind == hash40("fall_f") || motion_kind == hash40("fall_b"){
                    MotionModule::change_motion(fighter.module_accessor, Hash40::new("slide_fall"), 0.0, 1.0, false, 0.0, false, false);
                    WAS_SLIDE[ENTRY_ID] = false;
                }
            }
            if motion_kind == hash40("slide_fall"){
                if MotionModule::frame(boma) >= 18.0 {
                    MotionModule::change_motion(fighter.module_accessor, Hash40::new("slide_jump_fall"), 0.0, 1.0, false, 0.0, false, false);
                }
            }
            if status_kind == *FIGHTER_STATUS_KIND_RUN_BRAKE {
                if motion_kind != hash40("slide") && motion_kind != hash40("slide_stand"){
                    if  stick_y <= -0.5 &&
                    (ControlModule::check_button_off(boma, *CONTROL_PAD_BUTTON_ATTACK) && ControlModule::check_button_off(boma, *CONTROL_PAD_BUTTON_SPECIAL) && ControlModule::check_button_off(boma, *CONTROL_PAD_BUTTON_GUARD)) {
                        MotionModule::change_motion(fighter.module_accessor, Hash40::new("slide"), 0.0, 1.0, false, 0.0, false, false);
                    }
                } else {
                    let lr = PostureModule::lr(boma);
                    let speed = get_speed_x(boma) * lr;
                    MotionModule::set_rate(boma, 0.4);
                    WorkModule::unable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_HI4_START);
                    //WorkModule::unable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_LW4_START);
                    WorkModule::unable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_S4_START);
                    WorkModule::unable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_S3);
                    WorkModule::unable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_CATCH);
                    IS_SLIDE_MOVE[ENTRY_ID] = true;
                    WAS_SLIDE[ENTRY_ID] = true;
                }
            }
            if [hash40("slide")].contains(&motion_kind) {
                let speed = get_speed_x(boma) * lr;
                if speed < 0.1 {
                    MotionModule::change_motion(fighter.module_accessor, Hash40::new("slide_stand"), 0.0, 1.0, false, 0.0, false, false);
                }
            }
            if [hash40("slide_attack")].contains(&motion_kind) {
                if end_frame-frame < 3.0 {
                    MotionModule::change_motion(fighter.module_accessor, Hash40::new("slide_stand"), 0.0, 1.0, false, 0.0, false, false);
                }
            }
            if [hash40("slide_attack_lw")].contains(&motion_kind) {
                if end_frame-frame < 3.0 {
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_WAIT, false);
                }
                if frame >= (cancel_frame - 2.0) {
                    if (
                        (ControlModule::get_command_flag_cat(boma, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_S4) != 0 ||
                        (ControlModule::get_command_flag_cat(boma, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_LW4) != 0 ||
                        (ControlModule::get_command_flag_cat(boma, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_HI4) != 0 ||
                        (ControlModule::get_command_flag_cat(boma, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_N) != 0 ||
                        (ControlModule::get_command_flag_cat(boma, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_S3) != 0 ||
                        (ControlModule::get_command_flag_cat(boma, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_LW3) != 0 ||
                        (ControlModule::get_command_flag_cat(boma, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_HI3) != 0 ||
                        (ControlModule::get_command_flag_cat(boma, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_ANY) != 0 ||
                        (ControlModule::get_command_flag_cat(boma, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_TURN_DASH) != 0 ||
                        (ControlModule::get_command_flag_cat(boma, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_DASH) != 0 ||
                        ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_GUARD) ||
                        ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_JUMP) ||
                        ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_CATCH) 
                    ) {
                        StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_WAIT, false);
                    }
                }
                if frame > 30.0 {
                    MotionModule::set_rate(boma, 0.5);
                }
            }
            if [hash40("slide"), hash40("slide_attack_lw"), hash40("slide_attack")].contains(&motion_kind) {
                let desired_brake = 0.025;
                let lr = PostureModule::lr(boma);
                let brake = WorkModule::get_param_float(fighter.module_accessor, hash40("ground_brake"), 0);
                let speed = get_speed_x(boma) * lr;
                let mut added_speed = brake - desired_brake;
                if speed < 0.0 {
                    added_speed *= -1.0;
                };
                if (speed <= 0.0 && (speed + added_speed) > 0.0) || (speed >= 0.0 && (speed + added_speed) < 0.0) {
                    added_speed = 0.0;
                };
                let the_speed = smash::phx::Vector3f { x: added_speed, y: 0.0, z: 0.0 };
                KineticModule::add_speed(boma, &the_speed);
            }
            if motion_kind == hash40("slide_jump") && end_frame - frame < 3.0 {
                MotionModule::change_motion(fighter.module_accessor, Hash40::new("slide_jump_fall"), 0.0, 1.0, false, 0.0, false, false);
            }
            if ![*FIGHTER_STATUS_KIND_RUN_BRAKE, *FIGHTER_STATUS_KIND_JUMP_SQUAT, *FIGHTER_STATUS_KIND_JUMP, *FIGHTER_STATUS_KIND_ATTACK, *FIGHTER_STATUS_KIND_ATTACK_LW3, *FIGHTER_STATUS_KIND_ATTACK_S3, *FIGHTER_STATUS_KIND_ATTACK_HI3].contains(&status_kind) {
                IS_SLIDE_MOVE[ENTRY_ID] = false;
                WAS_SLIDE[ENTRY_ID] = false;
            } else if IS_SLIDE_MOVE[ENTRY_ID]{
                if status_kind == *FIGHTER_STATUS_KIND_ATTACK_LW3 {
                    if motion_kind != hash40("slide_attack_lw") {
                        MotionModule::change_motion(fighter.module_accessor, Hash40::new("slide_attack_lw"), -1.0, 1.0, false, 0.0, false, false);
                        IS_SLIDE_MOVE[ENTRY_ID] = false;
                        WAS_SLIDE[ENTRY_ID] = false;
                    }
                }
                if status_kind == *FIGHTER_STATUS_KIND_ATTACK {
                    StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_ATTACK_S3, true);
                }
                if status_kind == *FIGHTER_STATUS_KIND_ATTACK_HI3 {
                    if motion_kind != hash40("slide_attack_hi") {
                        MotionModule::change_motion(fighter.module_accessor, Hash40::new("slide_attack_hi"), -1.0, 1.0, false, 0.0, false, false);
                        IS_SLIDE_MOVE[ENTRY_ID] = false;
                        WAS_SLIDE[ENTRY_ID] = false;
                    }
                }
                if status_kind == *FIGHTER_STATUS_KIND_ATTACK_S3 {
                    if motion_kind != hash40("slide_attack") {
                        MotionModule::change_motion(fighter.module_accessor, Hash40::new("slide_attack"), -1.0, 1.0, false, 0.0, false, false);
                        IS_SLIDE_MOVE[ENTRY_ID] = false;
                        WAS_SLIDE[ENTRY_ID] = false;
                    }
                }
                if status_kind == *FIGHTER_STATUS_KIND_JUMP {
                    if motion_kind != hash40("slide_jump") {
                        MotionModule::change_motion(fighter.module_accessor, Hash40::new("slide_jump"), -1.0, 1.0, false, 0.0, false, false);
                        IS_SLIDE_MOVE[ENTRY_ID] = false;
                        WAS_SLIDE[ENTRY_ID] = false;
                    }
                }
            }
            /*if [hash40("slide_attack_lw")].contains(&motion_kind) {
                if MotionModule::frame(boma) >= 28.0 {
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_RUN_BRAKE, false);
                }
            }
            if [hash40("slide_attack")].contains(&motion_kind) {
                if MotionModule::frame(boma) >= 30.0 {
                    MotionModule::change_motion(fighter.module_accessor, Hash40::new("slide_stand"), 0.0, 1.0, false, 0.0, false, false);
                }
            }*/
            if [hash40("slide_attack_hi")].contains(&motion_kind) {
                if MotionModule::frame(boma) >= 46.0 {
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_FALL, true);
                    StatusModule::set_situation_kind(boma, smash::app::SituationKind(*SITUATION_KIND_AIR), true);
                }
            }
            if [hash40("slide_stand")].contains(&motion_kind) {
                if MotionModule::frame(boma) >= 10.0 {
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_WAIT, true);
                    StatusModule::set_situation_kind(boma, smash::app::SituationKind(*SITUATION_KIND_GROUND), true);
                }
            }
            if [hash40("slide_jump_fall")].contains(&motion_kind) {
                if stick_y <= -0.5 {
                    GroundModule::pass_floor(boma);
                    if ray_check_pos(boma, 0.0, -0.3, false) == 1 {
                        StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL, true);
                        StatusModule::set_situation_kind(boma, smash::app::SituationKind(*SITUATION_KIND_GROUND), true);
                        macros::SET_SPEED_EX(fighter, 0.0, 0.0, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
                    };
                } 
                else {
                    GroundModule::clear_pass_floor(boma);
                    if ray_check_pos(boma, 0.0, -0.3, true) == 1 {
                        StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL, true);
                        StatusModule::set_situation_kind(boma, smash::app::SituationKind(*SITUATION_KIND_GROUND), true);
                        macros::SET_SPEED_EX(fighter, 0.0, 0.0, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
                    };
                };
            }
            if [*FIGHTER_STATUS_KIND_ENTRY].contains(&status_kind) {
                if MotionModule::frame(boma) >= 78.0 {
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_WAIT, true);
                    StatusModule::set_situation_kind(boma, smash::app::SituationKind(*SITUATION_KIND_GROUND), true);
                }
            }
            if ![*FIGHTER_PIKMIN_STATUS_KIND_SPECIAL_HI_WAIT, *FIGHTER_PIKMIN_STATUS_KIND_SPECIAL_HI_END, *FIGHTER_STATUS_KIND_SPECIAL_HI, *FIGHTER_STATUS_KIND_ATTACK_AIR].contains(&status_kind) {
                SET_UPB_FREEFALL[ENTRY_ID] = false;
            } else if (frame >= cancel_frame - 5.0 || frame >= end_frame - 5.0) && SET_UPB_FREEFALL[ENTRY_ID]{
                StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_FALL_SPECIAL, false);
            }
            if [*FIGHTER_PIKMIN_STATUS_KIND_SPECIAL_HI_WAIT,  *FIGHTER_STATUS_KIND_SPECIAL_HI].contains(&status_kind) && ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_ATTACK){
                StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_ATTACK_AIR, false);
                SET_UPB_FREEFALL[ENTRY_ID] = true;
            }
            if [*FIGHTER_PIKMIN_STATUS_KIND_SPECIAL_HI_WAIT, *FIGHTER_STATUS_KIND_SPECIAL_HI].contains(&status_kind)  {
                if ![124, 127].contains(&WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR)) {
                    ModelModule::set_mesh_visibility(fighter.module_accessor,Hash40::new("pikmin_hair"),false);
                    let lr = PostureModule::lr(fighter.module_accessor);
                    macros::EFFECT_FOLLOW_FLIP(fighter, Hash40::new("sys_spin_wind"), Hash40::new("sys_spin_wind"), Hash40::new("head"), 3.3*lr, 0, 0, 0, 0, 90, 0.25, true, *EF_FLIP_YZ);
                    macros::LAST_EFFECT_SET_RATE(fighter, 2.0);
                    if WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) == 121 { //raymesis
                        macros::LAST_EFFECT_SET_COLOR(fighter, 1.07, 0.38, 1.76);
                    } 
                    else if WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) == 122 { //glowbox
                        macros::LAST_EFFECT_SET_COLOR(fighter, 0.0, 0.29, 0.89);
                    }
                    else if WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) == 123 { //caveman
                        macros::LAST_EFFECT_SET_COLOR(fighter, 1.09, 0.79, 0.47);
                    }
                    else if WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) == 126 { //afro
                        macros::LAST_EFFECT_SET_COLOR(fighter, 0.17, 0.0, 0.25);
                    }
                    else if WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) == 127 { //mario
                        macros::LAST_EFFECT_SET_COLOR(fighter, 0.13, 0.05, 0.0);
                    }
                    else {
                        macros::LAST_EFFECT_SET_COLOR(fighter, 1.15, 0.61, 0.0);
                    }
                    macros::EFFECT_FOLLOW_FLIP(fighter, Hash40::new("sys_spin_wind"), Hash40::new("sys_spin_wind"), Hash40::new("head"), 3.3*lr, 0, 0, 0, 0, 90, 0.23, true, *EF_FLIP_YZ);
                    macros::LAST_EFFECT_SET_RATE(fighter, 2.0);
                    if WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) == 121 { //raymesis
                        macros::LAST_EFFECT_SET_COLOR(fighter, 1.07, 0.38, 1.76);
                    } 
                    else if WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) == 122 { //glowbox
                        macros::LAST_EFFECT_SET_COLOR(fighter, 0.0, 0.29, 0.89);
                    }
                    else if WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) == 123 { //caveman
                        macros::LAST_EFFECT_SET_COLOR(fighter, 1.09, 0.79, 0.47);
                    }
                    else if WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) == 126 { //afro
                        macros::LAST_EFFECT_SET_COLOR(fighter, 0.17, 0.0, 0.25);
                    }
                    else if WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) == 127 { //mario
                        macros::LAST_EFFECT_SET_COLOR(fighter, 0.13, 0.05, 0.0);
                    }
                    else {
                        macros::LAST_EFFECT_SET_COLOR(fighter, 1.15, 0.61, 0.0);
                    }
                    macros::EFFECT_FOLLOW_FLIP(fighter, Hash40::new("sys_spin_wind"), Hash40::new("sys_spin_wind"), Hash40::new("head"), 2.8*lr, 0, 0, 0, 0, 90, 0.21, true, *EF_FLIP_YZ);
                    macros::LAST_EFFECT_SET_RATE(fighter, 2.0);
                    if WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) == 121 { //raymesis
                        macros::LAST_EFFECT_SET_COLOR(fighter, 1.07, 0.38, 1.76);
                    } 
                    else if WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) == 122 { //glowbox
                        macros::LAST_EFFECT_SET_COLOR(fighter, 0.0, 0.29, 0.89);
                    }
                    else if WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) == 123 { //caveman
                        macros::LAST_EFFECT_SET_COLOR(fighter, 1.09, 0.79, 0.47);
                    }
                    else if WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) == 126 { //afro
                        macros::LAST_EFFECT_SET_COLOR(fighter, 0.17, 0.0, 0.25);
                    }
                    else if WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) == 127 { //mario
                        macros::LAST_EFFECT_SET_COLOR(fighter, 0.13, 0.05, 0.0);
                    }
                    else {
                        macros::LAST_EFFECT_SET_COLOR(fighter, 1.15, 0.61, 0.0);
                    }
                    macros::EFFECT_FOLLOW_FLIP(fighter, Hash40::new("sys_spin_wind"), Hash40::new("sys_spin_wind"), Hash40::new("head"), 3.3*lr, 0, 0, 0, 0, 90, 0.17, true, *EF_FLIP_YZ);
                    macros::LAST_EFFECT_SET_RATE(fighter, 2.0);
                    if WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) == 121 { //raymesis
                        macros::LAST_EFFECT_SET_COLOR(fighter, 1.07, 0.38, 1.76);
                    } 
                    else if WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) == 122 { //glowbox
                        macros::LAST_EFFECT_SET_COLOR(fighter, 0.0, 0.29, 0.89);
                    }
                    else if WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) == 123 { //caveman
                        macros::LAST_EFFECT_SET_COLOR(fighter, 1.09, 0.79, 0.47);
                    }
                    else if WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) == 126 { //afro
                        macros::LAST_EFFECT_SET_COLOR(fighter, 0.17, 0.0, 0.25);
                    }
                    else if WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) == 127 { //mario
                        macros::LAST_EFFECT_SET_COLOR(fighter, 0.13, 0.05, 0.0);
                    }
                    else {
                        macros::LAST_EFFECT_SET_COLOR(fighter, 1.15, 0.61, 0.0);
                    }
                    macros::EFFECT_FOLLOW_FLIP(fighter, Hash40::new("sys_spin_wind"), Hash40::new("sys_spin_wind"), Hash40::new("head"), 3.3*lr, 0, 0, 0, 0, 90, 0.13, true, *EF_FLIP_YZ);
                    macros::LAST_EFFECT_SET_RATE(fighter, 2.0);
                    if WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) == 121 { //raymesis
                        macros::LAST_EFFECT_SET_COLOR(fighter, 1.07, 0.38, 1.76);
                    } 
                    else if WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) == 122 { //glowbox
                        macros::LAST_EFFECT_SET_COLOR(fighter, 0.0, 0.29, 0.89);
                    }
                    else if WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) == 123 { //caveman
                        macros::LAST_EFFECT_SET_COLOR(fighter, 1.09, 0.79, 0.47);
                    }
                    else if WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) == 126 { //afro
                        macros::LAST_EFFECT_SET_COLOR(fighter, 0.17, 0.0, 0.25);
                    }
                    else if WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) == 127 { //mario
                        macros::LAST_EFFECT_SET_COLOR(fighter, 0.13, 0.05, 0.0);
                    }
                    else {
                        macros::LAST_EFFECT_SET_COLOR(fighter, 1.15, 0.61, 0.0);
                    }
                    macros::EFFECT_FOLLOW_FLIP(fighter, Hash40::new("sys_spin_wind"), Hash40::new("sys_spin_wind"), Hash40::new("head"), 3.3*lr, 0, 0, 0, 0, 90, 0.09, true, *EF_FLIP_YZ);
                    macros::LAST_EFFECT_SET_RATE(fighter, 2.0);
                    if WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) == 121 { //raymesis
                        macros::LAST_EFFECT_SET_COLOR(fighter, 1.07, 0.38, 1.76);
                    } 
                    else if WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) == 122 { //glowbox
                        macros::LAST_EFFECT_SET_COLOR(fighter, 0.0, 0.29, 0.89);
                    }
                    else if WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) == 123 { //caveman
                        macros::LAST_EFFECT_SET_COLOR(fighter, 1.09, 0.79, 0.47);
                    }
                    else if WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) == 126 { //afro
                        macros::LAST_EFFECT_SET_COLOR(fighter, 0.17, 0.0, 0.25);
                    }
                    else if WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) == 127 { //mario
                        macros::LAST_EFFECT_SET_COLOR(fighter, 0.13, 0.05, 0.0);
                    }
                    else {
                        macros::LAST_EFFECT_SET_COLOR(fighter, 1.15, 0.61, 0.0);
                    }
                    macros::EFFECT_FOLLOW_FLIP(fighter, Hash40::new("sys_spin_wind"), Hash40::new("sys_spin_wind"), Hash40::new("head"), 3.3*lr, 0, 0, 0, 0, 90, 0.05, true, *EF_FLIP_YZ);
                    macros::LAST_EFFECT_SET_RATE(fighter, 2.0);
                    if WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) == 121 { //raymesis
                        macros::LAST_EFFECT_SET_COLOR(fighter, 1.07, 0.38, 1.76);
                    } 
                    else if WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) == 122 { //glowbox
                        macros::LAST_EFFECT_SET_COLOR(fighter, 0.0, 0.29, 0.89);
                    }
                    else if WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) == 123 { //caveman
                        macros::LAST_EFFECT_SET_COLOR(fighter, 1.09, 0.79, 0.47);
                    }
                    else if WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) == 126 { //afro
                        macros::LAST_EFFECT_SET_COLOR(fighter, 0.17, 0.0, 0.25);
                    }
                    else if WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) == 127 { //mario
                        macros::LAST_EFFECT_SET_COLOR(fighter, 0.13, 0.05, 0.0);
                    }
                    else {
                        macros::LAST_EFFECT_SET_COLOR(fighter, 1.15, 0.61, 0.0);
                    }
                }
                WorkModule::off_flag(boma, *FIGHTER_PIKMIN_STATUS_SPECIAL_HI_COMMON_FLAG_TURN);
            } else {
                ModelModule::set_mesh_visibility(fighter.module_accessor,Hash40::new("pikmin_hair"),true);
            }
        }
    }
}
unsafe extern "C" fn kill_pikmin(weapon: &mut L2CFighterBase) {
    unsafe {
        let otarget_id = WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER) as u32;
        let boma = smash::app::sv_system::battle_object_module_accessor(weapon.lua_state_agent); 
		let boma_reference = &mut *boma;
	    let fighter_kind = smash::app::utility::get_kind(boma);
        let owner_module_accessor = &mut *sv_battle_object::module_accessor((WorkModule::get_int(boma, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
	    let ENTRY_ID = WorkModule::get_int(owner_module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
        let is_rayman_weapon = (WorkModule::get_int(owner_module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) >= 120 && WorkModule::get_int(owner_module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) <= 127);
        
        if fighter_kind == *WEAPON_KIND_PIKMIN_PIKMIN && is_rayman_weapon {
           ModelModule::set_scale(weapon.module_accessor, 0.00001);
            PostureModule::set_scale(weapon.module_accessor, 0.00001, false);
            let pos = smash::phx::Vector3f { x: 0.0, y: 160.0, z: 0.0 };
            PostureModule::set_pos(weapon.module_accessor, &pos);
            PostureModule::init_pos(weapon.module_accessor, &pos, true, true);
            SoundModule::stop_all_sound(weapon.module_accessor);
            //WorkModule::set_int(weapon.module_accessor, 0, *WEAPON_PIKMIN_PIKMIN_INSTANCE_WORK_ID_INT_HP);
            //if StatusModule::status_kind(weapon.module_accessor) != *WEAPON_PIKMIN_PIKMIN_STATUS_KIND_DEATH {
            //StatusModule::change_status_request_from_script(weapon.module_accessor, *WEAPON_PIKMIN_PIKMIN_STATUS_KIND_LANDING, false);
            //};
        } else {
            if smash::app::sv_battle_object::is_active(weapon.battle_object_id) {
                CURRENT_PIKMIN[ENTRY_ID] = WorkModule::get_int(weapon.module_accessor, *WEAPON_PIKMIN_PIKMIN_INSTANCE_WORK_ID_INT_VARIATION);
            }
        }
    }
}

pub fn install() {
    Agent::new("pikmin")
        .on_line(Main, rayman)
        .install();

    Agent::new("pikmin_pikmin")
        .on_line(Main, kill_pikmin)
        .install();
}