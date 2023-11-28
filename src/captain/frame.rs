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
pub fn install() {
    smashline::install_agent_frames!(captain_frame);
}


#[fighter_frame( agent = FIGHTER_KIND_CAPTAIN )]
fn captain_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent); 
		if is_default(boma) {
            let status_kind = smash::app::lua_bind::StatusModule::status_kind(boma);
            let ENTRY_ID = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
            let motion_kind = MotionModule::motion_kind(boma);
            if ![*FIGHTER_STATUS_KIND_THROW, *FIGHTER_STATUS_KIND_SPECIAL_N, *FIGHTER_CAPTAIN_STATUS_KIND_SPECIAL_N_TURN].contains(&status_kind) &&  AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_ALL) {
                let cat1 = ControlModule::get_command_flag_cat(boma, 0);
                if (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_N) != 0 && !is_hitlag(boma){
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_SPECIAL_N, true);
                };
            };
            //Raptor Boost Cancel
            if [hash40("attack_s3_s"), hash40("attack_s3_lw"), hash40("attack_s3_hi")].contains(&motion_kind) && (AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT) || AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_SHIELD)){
                if (ControlModule::get_command_flag_cat(boma, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_S) != 0{ 
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_SPECIAL_S, true);
                };
            };
            //Falcon Kick wall jump cancel
            if motion_kind == hash40("special_air_lw") {
                let cat = fighter.global_table[0x20].get_int() as i32;
                if ((cat & *FIGHTER_PAD_CMD_CAT1_FLAG_WALL_JUMP_LEFT) != 0 && GroundModule::get_touch_flag(boma) == *GROUND_TOUCH_FLAG_LEFT as u64) || ((cat & *FIGHTER_PAD_CMD_CAT1_FLAG_WALL_JUMP_RIGHT) != 0 && GroundModule::get_touch_flag(boma) == *GROUND_TOUCH_FLAG_RIGHT as u64) {
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_WALL_JUMP, true);
                }
            }
        }
    }
}