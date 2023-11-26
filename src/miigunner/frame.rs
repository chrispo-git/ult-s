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

#[fighter_frame( agent = FIGHTER_KIND_MIIGUNNER )]
fn gunner_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        //println!("It'sa me, Mario, wahoooooooo!");
        let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent); 
		let status_kind = smash::app::lua_bind::StatusModule::status_kind(boma);
		let ENTRY_ID = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
		let motion_kind = MotionModule::motion_kind(boma);
		let frame = MotionModule::frame(boma);
        let stick_y = ControlModule::get_stick_y(boma);
        if [hash40("special_n3_start"), hash40("special_air_n3_start")].contains(&MotionModule::motion_kind(boma)) {
            if MotionModule::frame(boma) > 28.0 && MotionModule::frame(boma) < 30.0{
                if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_SPECIAL) && CHARGE_FRAMES[ENTRY_ID] < MAX_FRAMES{
                    CHARGE_FRAMES[ENTRY_ID] += 1;
                    MotionModule::set_rate(boma, 0.01);
                } else {
                    MotionModule::set_rate(boma, 0.5);
                };
            };
        } else {
            CHARGE_FRAMES[ENTRY_ID] = 0;
        };
        if [*FIGHTER_STATUS_KIND_SPECIAL_N].contains(&status_kind) && ![*FIGHTER_MIIGUNNER_STATUS_KIND_SPECIAL_N3_END, *FIGHTER_MIIGUNNER_STATUS_KIND_SPECIAL_N1_FIRE, *FIGHTER_MIIGUNNER_STATUS_KIND_SPECIAL_N1_HOLD, *FIGHTER_MIIGUNNER_STATUS_KIND_SPECIAL_N3_LOOP, *FIGHTER_MIIGUNNER_STATUS_KIND_SPECIAL_N1_START, *FIGHTER_MIIGUNNER_STATUS_KIND_SPECIAL_N1_CANCEL].contains(&status_kind){
            if StatusModule::is_situation_changed(boma) {
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL, true);
                WorkModule::set_float(boma, 10.0, *FIGHTER_INSTANCE_WORK_ID_FLOAT_LANDING_FRAME);
            };
            if StatusModule::situation_kind(boma) == *SITUATION_KIND_AIR {
                let cat2 = ControlModule::get_command_flag_cat(boma, 1);
                if (cat2 & *FIGHTER_PAD_CMD_CAT2_FLAG_FALL_JUMP) != 0 && stick_y < -0.66 && SPEED_Y[ENTRY_ID] <= 0.0 {
                    WorkModule::set_flag(boma, true, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_DIVE);
                }
            };
        };
        if StatusModule::situation_kind(boma) != *SITUATION_KIND_AIR {
            CAN_UPB[ENTRY_ID] = 0;
        };
        if [hash40("special_hi1"), hash40("special_air_hi1")].contains(&MotionModule::motion_kind(boma)) {
            CAN_UPB[ENTRY_ID] = 1;
            if MotionModule::frame(boma) >= 30.0 {
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_FALL, false);
            };
            WorkModule::set_int(boma, WorkModule::get_int(boma,*FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT_MAX), *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT);
        };
        /*if [hash40("special_s1"), hash40("special_air_s1")][ENTRY_ID] == true || COOLDOWN[ENTRY_ID] > 0{
            CAN_SIDEB[ENTRY_ID] = 1;
        } else {
            CAN_SIDEB[ENTRY_ID] = 0;
        };*/
        /*if [*FIGHTER_STATUS_KIND_SPECIAL_LW].contains(&status_kind) && ![*FIGHTER_MIIGUNNER_STATUS_KIND_SPECIAL_LW3_END, *FIGHTER_MIIGUNNER_STATUS_KIND_SPECIAL_LW3_HIT, *FIGHTER_MIIGUNNER_STATUS_KIND_SPECIAL_LW3_HOLD].contains(&status_kind) && (AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT) || AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_SHIELD)){
            if (ControlModule::get_command_flag_cat(boma, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_DASH) != 0{ 
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_CATCH_DASH, true);
            };
        };*/
        /*if [hash40("attack_s3_s")].contains(&motion_kind) && (AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT) || AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_SHIELD)){
            if (ControlModule::get_command_flag_cat(boma, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_N) != 0{ 
                StatusModule::change_status_request_from_script(boma, *FIGHTER_MIIGUNNER_STATUS_KIND_SPECIAL_N1_FIRE, true);
            };
        };
        if [hash40("attack_s3_s")].contains(&motion_kind) && (AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT) || AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_SHIELD)){
            if (ControlModule::get_command_flag_cat(boma, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_N) != 0{ 
                StatusModule::change_status_request_from_script(boma, *FIGHTER_MIIGUNNER_STATUS_KIND_SPECIAL_N3_END, true);
            };
        };
        if [hash40("attack_s3_s")].contains(&motion_kind) && (AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT) || AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_SHIELD)){
            if (ControlModule::get_command_flag_cat(boma, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_DASH) != 0{ 
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_CATCH_DASH, true);
            };
        };*/
    }
}

pub fn install() {
    smashline::install_agent_frames!(gunner_frame);
}