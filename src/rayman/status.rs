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

#[status_script(agent = "pikmin", status = FIGHTER_STATUS_KIND_CATCH_PULL, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
pub unsafe fn main_catch_pull(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.status_CatchPull();
	0.into() 
} 
#[status_script(agent = "pikmin", status = FIGHTER_STATUS_KIND_CATCH_PULL, condition = LUA_SCRIPT_STATUS_FUNC_INIT_STATUS)]
unsafe fn catch_pull_init(fighter: &mut L2CFighterCommon) -> L2CValue {
    L2CFighterCommon::sub_catch_pull_uniq_process_init(fighter);
    0.into()
}
#[status_script(agent = "pikmin", status = FIGHTER_STATUS_KIND_CATCH_PULL, condition = LUA_SCRIPT_STATUS_FUNC_EXIT_STATUS)]
unsafe fn catch_pull_exit(fighter: &mut L2CFighterCommon) -> L2CValue {
    L2CFighterCommon::sub_catch_pull_uniq_process_exit(fighter);
    0.into()
}
#[status_script(agent = "pikmin", status = FIGHTER_STATUS_KIND_CATCH_CUT, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
pub unsafe fn main_catch_cut(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.status_CatchCut();
	0.into() 
} 
#[status_script(agent = "pikmin", status = FIGHTER_STATUS_KIND_CATCH_CUT, condition = LUA_SCRIPT_STATUS_FUNC_INIT_STATUS)]
unsafe fn catch_cut_init(fighter: &mut L2CFighterCommon) -> L2CValue {
    L2CFighterCommon::sub_catch_cut_uniq_process_init(fighter);
    0.into()
}
#[status_script(agent = "pikmin", status = FIGHTER_STATUS_KIND_CATCH_WAIT, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
pub unsafe fn main_catch_wait(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.status_CatchWait();
	0.into() 
} 
#[status_script(agent = "pikmin", status = FIGHTER_STATUS_KIND_CATCH_WAIT, condition = LUA_SCRIPT_STATUS_FUNC_INIT_STATUS)]
unsafe fn catch_wait_init(fighter: &mut L2CFighterCommon) -> L2CValue {
    L2CFighterCommon::sub_catch_wait_uniq_process_init(fighter);
    0.into()
}
#[status_script(agent = "pikmin", status = FIGHTER_STATUS_KIND_CATCH_WAIT, condition = LUA_SCRIPT_STATUS_FUNC_EXIT_STATUS)]
unsafe fn catch_wait_exit(fighter: &mut L2CFighterCommon) -> L2CValue {
    L2CFighterCommon::sub_catch_wait_uniq_process_exit(fighter);
    0.into()
}
#[status_script(agent = "pikmin", status = FIGHTER_STATUS_KIND_CATCH_ATTACK, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
pub unsafe fn main_catch_attack(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.status_CatchAttack();
	0.into() 
} 
#[status_script(agent = "pikmin", status = FIGHTER_STATUS_KIND_THROW, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
pub unsafe fn main_throw(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.status_Throw();
	0.into() 
} 
#[status_script(agent = "pikmin", status = FIGHTER_STATUS_KIND_THROW, condition = LUA_SCRIPT_STATUS_FUNC_EXEC_STATUS)]
pub unsafe fn exec_throw(fighter: &mut L2CFighterCommon) -> L2CValue {
	0.into() 
} 
#[status_script(agent = "pikmin", status = FIGHTER_STATUS_KIND_THROW, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_PRE)]
pub unsafe fn pre_throw(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.status_pre_Throw();
	0.into() 
} 
#[status_script(agent = "pikmin", status = FIGHTER_STATUS_KIND_THROW, condition = LUA_SCRIPT_STATUS_FUNC_INIT_STATUS)]
unsafe fn throw_init(fighter: &mut L2CFighterCommon) -> L2CValue {
    let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent); 
    let is_ray = (WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) >= 120 && WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) <= 127);
    if is_ray {
        L2CFighterCommon::sub_throw_uniq_process_init(fighter);
        0.into()
    } else {
        original!(fighter)
    }
}
#[status_script(agent = "pikmin", status = FIGHTER_STATUS_KIND_THROW, condition = LUA_SCRIPT_STATUS_FUNC_EXIT_STATUS)]
unsafe fn throw_exit(fighter: &mut L2CFighterCommon) -> L2CValue {
    L2CFighterCommon::sub_throw_uniq_process_exit(fighter);
    0.into()
}
#[status_script(agent = "pikmin", status = FIGHTER_STATUS_KIND_ATTACK_LW3, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
pub unsafe fn main_dtilt(fighter: &mut L2CFighterCommon) -> L2CValue {
    let motion_kind = MotionModule::motion_kind(fighter.module_accessor);
    let ENTRY_ID = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent); 
	let fighter_kind = smash::app::utility::get_kind(boma);
    let is_ray = (WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) >= 120 && WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) <= 127);
    if is_ray && fighter_kind == *FIGHTER_KIND_PIKMIN { //rayman slots
        if !IS_SLIDE_MOVE[ENTRY_ID] && motion_kind != hash40("slide_attack_lw"){
            original!(fighter)
        } else {
            if motion_kind != hash40("slide_attack_lw") && motion_kind != hash40("slide_stand") {
                MotionModule::change_motion(fighter.module_accessor, Hash40::new("slide_attack_lw"), -1.0, 1.0, false, 0.0, false, false);
                IS_SLIDE_MOVE[ENTRY_ID] = false;
                WAS_SLIDE[ENTRY_ID] = false;
            }
            0.into() 
        }
    }
    else{
        original!(fighter)
    }
} 
#[status_script(agent = "pikmin", status = FIGHTER_STATUS_KIND_ATTACK, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
pub unsafe fn main_jab(fighter: &mut L2CFighterCommon) -> L2CValue {
    let motion_kind = MotionModule::motion_kind(fighter.module_accessor);
    let ENTRY_ID = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent); 
	let fighter_kind = smash::app::utility::get_kind(boma);
    let is_ray = (WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) >= 120 && WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) <= 127);
    if is_ray && fighter_kind == *FIGHTER_KIND_PIKMIN { //rayman slots
        if !IS_SLIDE_MOVE[ENTRY_ID] && motion_kind != hash40("slide_attack"){
            original!(fighter)
        } else {
            if motion_kind != hash40("slide_attack") {
                MotionModule::change_motion(fighter.module_accessor, Hash40::new("slide_attack"), -1.0, 1.0, false, 0.0, false, false);
                IS_SLIDE_MOVE[ENTRY_ID] = false;
                WAS_SLIDE[ENTRY_ID] = false;
            }
            if MotionModule::is_end(fighter.module_accessor) {
                fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
            }
            0.into() 
        }
    }
    else{
        original!(fighter)
    }
} 
#[status_script(agent = "pikmin", status = FIGHTER_STATUS_KIND_JUMP_SQUAT, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
pub unsafe fn main_jumpsquat(fighter: &mut L2CFighterCommon) -> L2CValue {
    let motion_kind = MotionModule::motion_kind(fighter.module_accessor);
    let end_frame = MotionModule::end_frame(fighter.module_accessor);
    let ENTRY_ID = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent); 
	let fighter_kind = smash::app::utility::get_kind(boma);
    let is_ray = (WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) >= 120 && WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) <= 127);
    if is_ray && fighter_kind == *FIGHTER_KIND_PIKMIN { //rayman slots
        if !IS_SLIDE_MOVE[ENTRY_ID] && motion_kind != hash40("slide_jump_squat"){
            original!(fighter)
        } else {
            if motion_kind != hash40("slide_jump_squat") {
                MotionModule::change_motion(fighter.module_accessor, Hash40::new("slide_jump_squat"), -1.0, (end_frame/3.0)*2.0, false, 0.0, false, false);
            }
            original!(fighter)
        }
    }
    else{
        original!(fighter)
    }
} 
#[status_script(agent = "pikmin", status = FIGHTER_STATUS_KIND_FINAL, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
pub unsafe fn main_final(fighter: &mut L2CFighterCommon) -> L2CValue {
    let motion_kind = MotionModule::motion_kind(fighter.module_accessor);
    let ENTRY_ID = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    let frame = MotionModule::frame(fighter.module_accessor);
    let situation_kind = StatusModule::situation_kind(fighter.module_accessor);
    let is_near_ground = GroundModule::ray_check(fighter.module_accessor, &Vector2f{ x: PostureModule::pos_x(fighter.module_accessor), y: PostureModule::pos_y(fighter.module_accessor)}, &Vector2f{ x: 0.0, y: -1.0}, true) == 1;
	let cancel_frame = FighterMotionModuleImpl::get_cancel_frame(fighter.module_accessor,smash::phx::Hash40::new_raw(MotionModule::motion_kind(fighter.module_accessor)),false) as f32; //Cancel frame
    let stick_x = ControlModule::get_stick_x(fighter.module_accessor) * PostureModule::lr(fighter.module_accessor);
    let stick_y = ControlModule::get_stick_y(fighter.module_accessor);
    let speed_x = KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    let speed_y = KineticModule::get_sum_speed_y(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent); 
	let fighter_kind = smash::app::utility::get_kind(boma);
    let is_ray = (WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) >= 120 && WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) <= 127);
    if is_ray && fighter_kind == *FIGHTER_KIND_PIKMIN { //rayman slots
        ArticleModule::remove_exist(fighter.module_accessor, *FIGHTER_PIKMIN_GENERATE_ARTICLE_DOLFIN,smash::app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
        StatusModule::set_keep_situation_air(fighter.module_accessor, true);
        if ![hash40("final"), hash40("final_shoot")].contains(&motion_kind) {
            MotionModule::change_motion(fighter.module_accessor, Hash40::new("final"), -1.0, 1.0, false, 0.0, false, false);
            macros::CHECK_VALID_FINAL_START_CAMERA(fighter, 0, 1, 20, 0, 0, 0);
            macros::FT_SET_FINAL_FEAR_FACE(fighter, 40);
            macros::FT_START_CUTIN(fighter, );
            macros::EFFECT(fighter, Hash40::new("sys_item_arrival"), Hash40::new("top"), 9, 3.5, 1.5, 0, 0, 0, 1.5, 0, 0, 0, 0, 0, 0, true);
            macros::EFFECT(fighter, Hash40::new("sys_erace_smoke"), Hash40::new("top"), 9, 3.5, 1, 0, 0, 0, 4.0, 0, 0, 0, 0, 0, 0, false);
            macros::LAST_EFFECT_SET_RATE(fighter, 0.5);
            macros::PLAY_SE(fighter, Hash40::new("se_common_final_cutin"));
            macros::PLAY_SE(fighter, Hash40::new("se_common_spirits_pfog_loop"));
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
            KineticModule::suspend_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
            KineticModule::suspend_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
            FINAL_DURATION[ENTRY_ID] = 900;
            X[ENTRY_ID] = 0.0;
            Y[ENTRY_ID] = 0.0;
            ItemModule::remove_item(fighter.module_accessor, 0);
            ItemModule::set_have_item_visibility(fighter.module_accessor, false, 0);
            ItemModule::have_item(fighter.module_accessor, smash::app::ItemKind(*ITEM_KIND_SUPERSCOPE), 0, 0, false, false);
            macros::STOP_SE(fighter, Hash40::new("se_item_item_get"));
            HitModule::set_whole(fighter.module_accessor, smash::app::HitStatus(*HIT_STATUS_INVINCIBLE), 0);
        } else {
            if MotionModule::is_end(fighter.module_accessor) {
                MotionModule::change_motion(fighter.module_accessor, Hash40::new("final"), 0.0, 1.0, false, 0.0, false, false);
            }
            if FINAL_DURATION[ENTRY_ID] < 990 {
                if ControlModule::check_button_on_trriger(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) || ControlModule::check_button_on_trriger(fighter.module_accessor, *CONTROL_PAD_BUTTON_ATTACK) {
                    MotionModule::change_motion(fighter.module_accessor, Hash40::new("final_shoot"), 0.0, 1.0, false, 0.0, false, false);
                }
            }
            let mut y_add = 0.0;
            let mut x_add = 0.0;
            if stick_x > 0.2 {
                x_add = ((stick_x-0.2)*X_ACCEL_MUL) + X_ACCEL_ADD;
                if speed_x > X_MAX || speed_x < -X_MAX{
                    x_add = 0.0;
                };
            };
            if stick_x < -0.2 {
                x_add = ((stick_x+0.2)*X_ACCEL_MUL) + X_ACCEL_ADD;
                if speed_x > X_MAX || speed_x < -X_MAX{
                    x_add = 0.0;
                };
            };
            if stick_y > 0.2 {
                y_add = ((stick_y-0.2)*Y_ACCEL_MUL) + Y_ACCEL_ADD;
                if speed_y > Y_MAX || speed_y < -Y_MAX{
                    y_add = 0.0;
                };
            };
            if stick_y < -0.2 {
                y_add = ((stick_y+0.2)*Y_ACCEL_MUL) + Y_ACCEL_ADD;
                if speed_y > Y_MAX || speed_y < -Y_MAX{
                    y_add = 0.0;
                };
            };
            if stick_x > -0.2 && stick_x < 0.2 && stick_y > -0.2 && stick_y < 0.2 {
                if speed_y > 0.0 {
                    y_add = -Y_ACCEL_MUL - Y_ACCEL_ADD;
                } else if speed_y < 0.0{
                    y_add = Y_ACCEL_MUL + Y_ACCEL_ADD;
                };
                let mut x_add = 0.0;
                if speed_x > 0.0 {
                    x_add = -X_ACCEL_MUL - X_ACCEL_ADD;
                } else if speed_x < 0.0{
                    x_add = X_ACCEL_MUL + X_ACCEL_ADD;
                };
            };
            x_add = (stick_x)*X_ACCEL_MUL;
            y_add = (stick_y)*X_ACCEL_MUL;
            if x_add > 0.0 && X[ENTRY_ID] > X_MAX {
                x_add = 0.0;
            };
            if x_add < 0.0 && X[ENTRY_ID] < X_MAX*-1.0 {
                x_add = 0.0;
            };
            if y_add > 0.0 && Y[ENTRY_ID] > Y_MAX {
                y_add = 0.0;
            };
            if y_add < 0.0 && Y[ENTRY_ID] < Y_MAX*-1.0 {
                y_add = 0.0;
            };
            println!("x{}, y{}", X[ENTRY_ID], Y[ENTRY_ID]);
            println!("x_add{}, y_add{}", x_add, y_add);
            let speed = smash::phx::Vector3f { x: x_add, y: y_add, z: 0.0 };
            KineticModule::add_speed(fighter.module_accessor, &speed);
            X[ENTRY_ID] += x_add;
            Y[ENTRY_ID] += y_add;
            FINAL_DURATION[ENTRY_ID] -= 1;
            if FINAL_DURATION[ENTRY_ID] <= 0 {
                macros::STOP_SE(fighter, Hash40::new("se_common_spirits_pfog_loop"));
                macros::PLAY_SE(fighter, Hash40::new("se_pikmin_attackhard_l01"));
                macros::PLAY_SE(fighter, Hash40::new("se_common_blowaway_s"));
                macros::EFFECT(fighter, Hash40::new("sys_item_arrival"), Hash40::new("top"), 9, 3.5, 1.5, 0, 0, 0, 1.5, 0, 0, 0, 0, 0, 0, true);
                macros::EFFECT(fighter, Hash40::new("sys_erace_smoke"), Hash40::new("top"), 9, 3.5, 1, 0, 0, 0, 4.0, 0, 0, 0, 0, 0, 0, false);
                ItemModule::set_have_item_visibility(fighter.module_accessor, true, 0);
                ItemModule::remove_item(fighter.module_accessor, 0);
                StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_CAPTURE_JUMP, false);
            }
        }
        0.into() 
    }
    else{
        original!(fighter)
    }
} 
#[status_script(agent = "pikmin", status = FIGHTER_STATUS_KIND_SPECIAL_LW, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
pub unsafe fn main_downb(fighter: &mut L2CFighterCommon) -> L2CValue {
    let motion_kind = MotionModule::motion_kind(fighter.module_accessor);
    let ENTRY_ID = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    let frame = MotionModule::frame(fighter.module_accessor);
    let situation_kind = StatusModule::situation_kind(fighter.module_accessor);
    let is_near_ground = GroundModule::ray_check(fighter.module_accessor, &Vector2f{ x: PostureModule::pos_x(fighter.module_accessor), y: PostureModule::pos_y(fighter.module_accessor)}, &Vector2f{ x: 0.0, y: -1.0}, true) == 1;
	let cancel_frame = FighterMotionModuleImpl::get_cancel_frame(fighter.module_accessor,smash::phx::Hash40::new_raw(MotionModule::motion_kind(fighter.module_accessor)),false) as f32; //Cancel frame
    let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent); 
	let fighter_kind = smash::app::utility::get_kind(boma);
    let is_ray = (WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) >= 120 && WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) <= 127);
    if is_ray && fighter_kind == *FIGHTER_KIND_PIKMIN { //rayman slots
        if ![hash40("special_lw"), hash40("special_air_lw"), hash40("special_air_lw_loop"), hash40("special_lw_land")].contains(&motion_kind) {
            if situation_kind == *SITUATION_KIND_AIR {
                MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_lw"), -1.0, 1.0, false, 0.0, false, false);
                KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
            } else {
                MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_lw"), -1.0, 1.0, false, 0.0, false, false);
                KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION_AIR);
            }
        } else {
            if motion_kind == hash40("special_air_lw_loop") {
                if (
                    (ControlModule::get_command_flag_cat(fighter.module_accessor, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_S4) != 0 ||
                    (ControlModule::get_command_flag_cat(fighter.module_accessor, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_LW4) != 0 ||
                    (ControlModule::get_command_flag_cat(fighter.module_accessor, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_HI4) != 0 ||
                    (ControlModule::get_command_flag_cat(fighter.module_accessor, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_N) != 0 ||
                    (ControlModule::get_command_flag_cat(fighter.module_accessor, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_S3) != 0 ||
                    (ControlModule::get_command_flag_cat(fighter.module_accessor, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_LW3) != 0 ||
                    (ControlModule::get_command_flag_cat(fighter.module_accessor, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_HI3) != 0 ||
                    (ControlModule::get_command_flag_cat(fighter.module_accessor, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_ANY) != 0 ||
                    ControlModule::check_button_on_trriger(fighter.module_accessor, *CONTROL_PAD_BUTTON_GUARD) ||
                    (ControlModule::check_button_on_trriger(fighter.module_accessor, *CONTROL_PAD_BUTTON_JUMP) && WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT) < WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT_MAX)) ||
                    (ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_FLICK_JUMP) && ControlModule::get_flick_y(fighter.module_accessor) >= 3 && ControlModule::get_stick_y(fighter.module_accessor) >= 0.7 && WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT) < WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT_MAX))
                ) {
                    StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_FALL, false);
                }
                if situation_kind == *SITUATION_KIND_GROUND || is_near_ground{
                    MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_lw_land"), -1.0, 1.0, false, 0.0, false, false);
                }
            }
            if motion_kind == hash40("special_lw") {
                if KineticModule::get_kinetic_type(fighter.module_accessor) != *FIGHTER_KINETIC_TYPE_MOTION_AIR {
                    KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION_AIR);
                }
            }
            if motion_kind == hash40("special_air_lw") {
                if situation_kind == *SITUATION_KIND_GROUND {
                    macros::QUAKE(fighter, *CAMERA_QUAKE_KIND_S);
                    MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_lw_land"), -1.0, 1.0, false, 0.0, false, false);
                }
            }
            if motion_kind == hash40("special_lw_land") {
                KineticModule::clear_speed_all(fighter.module_accessor);
                AttackModule::clear_all(fighter.module_accessor);
                if frame >= cancel_frame || (frame >= 15.0 && AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_HIT)) {
                    if (
                        (ControlModule::get_command_flag_cat(fighter.module_accessor, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_S4) != 0 ||
                        (ControlModule::get_command_flag_cat(fighter.module_accessor, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_LW4) != 0 ||
                        (ControlModule::get_command_flag_cat(fighter.module_accessor, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_HI4) != 0 ||
                        (ControlModule::get_command_flag_cat(fighter.module_accessor, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_N) != 0 ||
                        (ControlModule::get_command_flag_cat(fighter.module_accessor, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_S3) != 0 ||
                        (ControlModule::get_command_flag_cat(fighter.module_accessor, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_LW3) != 0 ||
                        (ControlModule::get_command_flag_cat(fighter.module_accessor, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_HI3) != 0 ||
                        (ControlModule::get_command_flag_cat(fighter.module_accessor, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_ANY) != 0 ||
                        (ControlModule::get_command_flag_cat(fighter.module_accessor, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_TURN_DASH) != 0 ||
                        (ControlModule::get_command_flag_cat(fighter.module_accessor, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_DASH) != 0 ||
                        ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_GUARD) ||
                        (ControlModule::check_button_on_trriger(fighter.module_accessor, *CONTROL_PAD_BUTTON_JUMP) && WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT) < WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT_MAX))||
                        (ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_FLICK_JUMP) && ControlModule::get_flick_y(fighter.module_accessor) >= 3 && ControlModule::get_stick_y(fighter.module_accessor) >= 0.7 && WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT) < WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT_MAX))
                    ) {
                        StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_WAIT, false);
                    }
                }
                if MotionModule::is_end(fighter.module_accessor) {
                    StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_WAIT, true);
                }
            }
            if [hash40("special_lw"), hash40("special_air_lw")].contains(&motion_kind) && MotionModule::is_end(fighter.module_accessor) {
                KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
                MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_lw_loop"), -1.0, 1.0, false, 0.0, false, false);
            }
        }
        0.into() 
    }
    else{
        original!(fighter)
    }
} 
#[status_script(agent = "pikmin", status = FIGHTER_STATUS_KIND_ATTACK_HI3, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
pub unsafe fn main_utilt(fighter: &mut L2CFighterCommon) -> L2CValue {
    let motion_kind = MotionModule::motion_kind(fighter.module_accessor);
    let ENTRY_ID = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    let frame = MotionModule::frame(fighter.module_accessor);
	let cancel_frame = FighterMotionModuleImpl::get_cancel_frame(fighter.module_accessor,smash::phx::Hash40::new_raw(MotionModule::motion_kind(fighter.module_accessor)),false) as f32; //Cancel frame
	let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent); 
	let fighter_kind = smash::app::utility::get_kind(boma);
    let is_ray = (WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) >= 120 && WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) <= 127);
    if is_ray && fighter_kind == *FIGHTER_KIND_PIKMIN { //rayman slots
        if !IS_SLIDE_MOVE[ENTRY_ID] && motion_kind != hash40("slide_attack_hi"){
            if motion_kind != hash40("attack_hi3") {
                MotionModule::change_motion(fighter.module_accessor, Hash40::new("attack_hi3"), -1.0, 1.0, false, 0.0, false, false);
            }
            if MotionModule::is_end(fighter.module_accessor) {
                fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
            }
        } else {
            if motion_kind != hash40("slide_attack_hi") {
                MotionModule::change_motion(fighter.module_accessor, Hash40::new("slide_attack_hi"), -1.0, 1.0, false, 0.0, false, false);
                IS_SLIDE_MOVE[ENTRY_ID] = false;
                WAS_SLIDE[ENTRY_ID] = false;
            }
            if MotionModule::is_end(fighter.module_accessor) {
                fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
            }
        }
        0.into() 
    }
    else{
        original!(fighter)
    }
} 


#[status_script(agent = "pikmin", status = FIGHTER_STATUS_KIND_ATTACK_HI3, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_PRE)]
unsafe fn utilt_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent); 
	let fighter_kind = smash::app::utility::get_kind(boma);
    let is_ray = (WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) >= 120 && WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) <= 127);
    if is_ray && fighter_kind == *FIGHTER_KIND_PIKMIN { //rayman slots
        fighter.sub_status_pre_SpecialNCommon();
        StatusModule::init_settings(
            fighter.module_accessor,
            smash::app::SituationKind(*SITUATION_KIND_NONE),
            *FIGHTER_KINETIC_TYPE_MOTION_AIR,
            *GROUND_CORRECT_KIND_AIR as u32,
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
            (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_N | *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON) as u64,
            *FIGHTER_STATUS_ATTR_START_TURN as u32,
            *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_N as u32,
            0
        );

        0.into()
    }
    else{
        original!(fighter)
    }
}
#[status_script(agent = "kirby", status = FIGHTER_KIRBY_STATUS_KIND_PIKMIN_SPECIAL_N, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_PRE)]
unsafe fn kirby_copy_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent); 
	let fighter_kind = smash::app::utility::get_kind(boma);
    let is_ray = (WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) >= 120 && WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) <= 127);
    if is_ray && fighter_kind == *FIGHTER_KIND_PIKMIN { //rayman slots
        fighter.sub_status_pre_SpecialNCommon();
        StatusModule::init_settings(
            fighter.module_accessor,
            smash::app::SituationKind(*SITUATION_KIND_NONE),
            *FIGHTER_KINETIC_TYPE_MOTION_AIR,
            *GROUND_CORRECT_KIND_AIR as u32,
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
            (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_N | *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON) as u64,
            *FIGHTER_STATUS_ATTR_START_TURN as u32,
            *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_LW as u32,
            0
        );

        0.into()
    }
    else{
        original!(fighter)
    }
}
#[status_script(agent = "pikmin", status = FIGHTER_STATUS_KIND_SPECIAL_LW, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_PRE)]
unsafe fn downb_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent); 
	let fighter_kind = smash::app::utility::get_kind(boma);
    let is_ray = (WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) >= 120 && WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) <= 127);
    if is_ray && fighter_kind == *FIGHTER_KIND_PIKMIN { //rayman slots
        fighter.sub_status_pre_SpecialNCommon();
        StatusModule::init_settings(
            fighter.module_accessor,
            smash::app::SituationKind(*SITUATION_KIND_NONE),
            *FIGHTER_KINETIC_TYPE_MOTION_AIR,
            *GROUND_CORRECT_KIND_AIR as u32,
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
            (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_N | *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON) as u64,
            *FIGHTER_STATUS_ATTR_START_TURN as u32,
            *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_N as u32,
            0
        );

        0.into()
    }
    else{
        original!(fighter)
    }
}

pub fn install() {
    install_status_scripts!(
        utilt_pre, main_utilt, //Puts Rayman in the air for utilt

        //Should fix olimar related rayman grab and throw bugs
        main_catch_attack, 
        main_catch_cut, catch_cut_init,
        main_catch_pull, catch_pull_exit, catch_pull_init,
        main_catch_wait, catch_wait_exit, catch_wait_init,
        main_throw, exec_throw, throw_init, throw_exit,

        //Slide Shit
        main_jab, main_dtilt, 
        main_jumpsquat,

        //Downb
        main_downb, downb_pre,

        //Final
        main_final,

        //Kirby
        kirby_copy_pre
    );
}