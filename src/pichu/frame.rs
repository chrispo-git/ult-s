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

#[fighter_frame( agent = FIGHTER_KIND_PICHU )]
fn pichu_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent); 
		if is_default(boma) {
			let status_kind = smash::app::lua_bind::StatusModule::status_kind(boma);
			let motion_kind = MotionModule::motion_kind(boma);
			let ENTRY_ID = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
			let total_hitstun = WorkModule::get_float(boma, *FIGHTER_INSTANCE_WORK_ID_FLOAT_DAMAGE_REACTION_FRAME_LAST);
			let cancel_frame = FighterMotionModuleImpl::get_cancel_frame(boma,smash::phx::Hash40::new_raw(MotionModule::motion_kind(boma)),false) as f32;
			let frame = MotionModule::frame(boma);
			let fallspeed = WorkModule::get_param_float(fighter.module_accessor, hash40("air_speed_y_stable"), 0);
			let is_near_ground = GroundModule::ray_check(boma, &Vector2f{ x: PostureModule::pos_x(boma), y: PostureModule::pos_y(boma)}, &Vector2f{ x: 0.0, y: -1.0}, true);
			if StatusModule::situation_kind(boma) == *SITUATION_KIND_AIR && [*FIGHTER_PIKACHU_STATUS_KIND_SPECIAL_HI_END].contains(&status_kind) {
				StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_FALL, false);
				if WorkModule::get_int(boma,  *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT) >= WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT_MAX) {
					WorkModule::set_int(boma, 0, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT);
				}
				CAN_UPB[ENTRY_ID] = 1;
			};
			if  StatusModule::situation_kind(boma) != *SITUATION_KIND_AIR || (*FIGHTER_STATUS_KIND_DAMAGE..*FIGHTER_STATUS_KIND_DAMAGE_FALL).contains(&status_kind){
				CAN_UPB[ENTRY_ID] = 0;
			};
			if [hash40("attack_air_lw")].contains(&MotionModule::motion_kind(boma)) && !ONE_DAIR[ENTRY_ID] {
				if AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_ALL) && !AttackModule::is_infliction(boma, *COLLISION_KIND_MASK_ALL) && MotionModule::frame(boma) < 46.0{
					KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_JUMP);
					MotionModule::set_frame_sync_anim_cmd(boma, 47.0, true, true, false);
					EffectModule::kill_kind(boma, smash::phx::Hash40::new("sys_attack_speedline"), false, false);
					EffectModule::kill_kind(boma, smash::phx::Hash40::new("pichu_ear_elec"), false, false);
					EffectModule::kill_kind(boma, smash::phx::Hash40::new("pichu_cheek2"), false, false);
					if !AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT) {
						MotionModule::set_rate(boma, 0.5);
					};
					ONE_DAIR[ENTRY_ID] = true;
				};
			}
			if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_LW {
					StatusModule::change_status_request_from_script(boma, *FIGHTER_PIKACHU_STATUS_KIND_SPECIAL_LW_HIT, true);
					KineticModule::clear_speed_all(boma);
			};
			if status_kind == *FIGHTER_PIKACHU_STATUS_KIND_SPECIAL_LW_HIT {
				if StatusModule::situation_kind(boma) == *SITUATION_KIND_AIR {
					CAN_DOWNB[ENTRY_ID] = 1;
				};
			};
			if StatusModule::situation_kind(boma) != *SITUATION_KIND_AIR {
				HAS_DOWNB[ENTRY_ID] = false;
				DO_STALL[ENTRY_ID] = false;
				ONE_DAIR[ENTRY_ID] = false;
				CAN_DOWNB[ENTRY_ID] = 0;
			};
			if ![*FIGHTER_STATUS_KIND_ATTACK_AIR, *FIGHTER_STATUS_KIND_LANDING_ATTACK_AIR, *FIGHTER_PIKACHU_STATUS_KIND_SPECIAL_HI_END].contains(&status_kind) {
				LAG_INCREASE[ENTRY_ID] = false;
			};
		}
    }
}

pub fn install() {
    smashline::install_agent_frames!(
        pichu_frame
    );
}