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
use crate::util::*;


static mut KEN_MAX_METER : i32 = 44;
static mut PPP: i32 = 3;
static mut KEN_SUPER : [i32; 8] = [0; 8];
static mut KEN_IS_EX : [bool; 8] = [false; 8];
static mut HAS_ADDED : [bool; 8] = [false; 8];
static mut KEN_FX_TIMER : [i32; 8] = [0; 8];
static mut IS_SUPER : [bool; 8] = [false; 8];
static mut EX_DOWNB : [bool; 8] = [false; 8];
static mut HANDS :  smash::phx::Vector3f =  smash::phx::Vector3f { x: 0.0, y: 0.0, z: 0.0 };
static mut FEET :  smash::phx::Vector3f =  smash::phx::Vector3f { x: 1.0, y: 0.0, z: 0.0 };

pub(crate) unsafe fn is_attack_btn(boma: &mut smash::app::BattleObjectModuleAccessor) -> bool {
	return (ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_SPECIAL_RAW)  && ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_ATTACK_RAW)) &&  
	((ControlModule::check_button_off(boma, *CONTROL_PAD_BUTTON_CATCH) && ControlModule::check_button_off(boma, *CONTROL_PAD_BUTTON_GUARD)) || 
	StatusModule::situation_kind(boma) == *SITUATION_KIND_GROUND )
}

#[acmd_script(
    agent = "ken",
    script =  "game_attacklw4",
    category = ACMD_GAME,
	low_priority)]
unsafe fn ken_dsmash(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
		frame(fighter.lua_state_agent, 3.0);
		if macros::is_excute(fighter) {
			WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
		}
		frame(fighter.lua_state_agent, 5.0);
		if macros::is_excute(fighter) {
			macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 16.0, /*Angle*/ 25, /*KBG*/ 47, /*FKB*/ 0, /*BKB*/ 50, /*Size*/ 3.6, /*X*/ 0.0, /*Y*/ 2.5, /*Z*/ 12.0, /*X2*/ Some(0.0), /*Y2*/ Some(3.0), /*Z2*/ Some(8.0), /*Hitlag*/ 1.5, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ -8, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_KEN_KICK, /*Type*/ *ATTACK_REGION_KICK);
			macros::ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 12.0, /*Angle*/ 25, /*KBG*/ 50, /*FKB*/ 0, /*BKB*/ 50, /*Size*/ 2.5, /*X*/ 0.0, /*Y*/ 3.0, /*Z*/ 2.5, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.5, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ -6, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_KEN_KICK, /*Type*/ *ATTACK_REGION_KICK);
			macros::ATTACK(fighter, /*ID*/ 2, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 12.0, /*Angle*/ 25, /*KBG*/ 50, /*FKB*/ 0, /*BKB*/ 50, /*Size*/ 2.5, /*X*/ 0.0, /*Y*/ 3.0, /*Z*/ 2.5, /*X2*/ Some(0.0), /*Y2*/ Some(3.0), /*Z2*/ Some(0.0), /*Hitlag*/ 1.5, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ -6, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_KEN_KICK, /*Type*/ *ATTACK_REGION_KICK);
		}
		wait(fighter.lua_state_agent, 2.0);
		if macros::is_excute(fighter) {
			AttackModule::clear_all(fighter.module_accessor);
		}
}		
#[acmd_script(
    agent = "ken",
    script =  "game_attack13",
    category = ACMD_GAME,
	low_priority)]
unsafe fn ken_jab3(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
		if macros::is_excute(fighter) {
			WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
			WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_RYU_STATUS_ATTACK_FLAG_HIT_CANCEL);
		}
		frame(fighter.lua_state_agent, 8.0);
		if macros::is_excute(fighter) {
			macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 5.0, /*Angle*/ 45, /*KBG*/ 50, /*FKB*/ 0, /*BKB*/ 70, /*Size*/ 3.0, /*X*/ 0.0, /*Y*/ 11.0, /*Z*/ 11.5, /*X2*/ Some(0.0), /*Y2*/ Some(11.0), /*Z2*/ Some(3.0), /*Hitlag*/ 1.5, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_KEN_PUNCH, /*Type*/ *ATTACK_REGION_PUNCH);
			macros::ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 5.0, /*Angle*/ 45, /*KBG*/ 50, /*FKB*/ 0, /*BKB*/ 70, /*Size*/ 1.8, /*X*/ 0.0, /*Y*/ 8.0, /*Z*/ 8.0, /*X2*/ Some(0.0), /*Y2*/ Some(8.0), /*Z2*/ Some(3.0), /*Hitlag*/ 1.5, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_KEN_PUNCH, /*Type*/ *ATTACK_REGION_PUNCH);
		}
		wait(fighter.lua_state_agent, 3.0);
		if macros::is_excute(fighter) {
			AttackModule::clear_all(fighter.module_accessor);
			WorkModule::off_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
			WorkModule::off_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_RYU_STATUS_ATTACK_FLAG_HIT_CANCEL);
		}
}	

//RYU SUPER
#[fighter_frame_callback]
pub fn supers(fighter : &mut L2CFighterCommon) {
    unsafe {
        let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);  
		let fighter_kind = smash::app::utility::get_kind(boma);
		let status_kind = smash::app::lua_bind::StatusModule::status_kind(boma);
		let lua_state = fighter.lua_state_agent;
		let color_id = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);
		let motion_kind = MotionModule::motion_kind(boma);
		let frame = MotionModule::frame(boma);
		let cancel_frame = FighterMotionModuleImpl::get_cancel_frame(boma,smash::phx::Hash40::new_raw(MotionModule::motion_kind(boma)),false) as f32;
		let mut ENTRY_ID = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
		if fighter_kind == *FIGHTER_KIND_KEN {
			let meter_half = KEN_MAX_METER as f32 * 0.5;
			KEN_FX_TIMER[ENTRY_ID] += 1;
			if smash::app::smashball::is_training_mode() == true {
				if status_kind == *FIGHTER_STATUS_KIND_APPEAL {
					KEN_SUPER[ENTRY_ID] = KEN_MAX_METER;
					CancelModule::enable_cancel(boma);
				};
			};
			if KEN_FX_TIMER[ENTRY_ID] == 6 {
				let meter_1: u32 = EffectModule::req_follow(boma, smash::phx::Hash40::new("sys_damage_elec"), smash::phx::Hash40::new("haver"), &HANDS, &HANDS, 0.2, true, 0, 0, 0, 0, 0, true, true) as u32;
				let meter_2: u32 = EffectModule::req_follow(boma, smash::phx::Hash40::new("sys_damage_elec"), smash::phx::Hash40::new("havel"), &HANDS, &HANDS, 0.2, true, 0, 0, 0, 0, 0, true, true) as u32;
				let meter_3: u32 = EffectModule::req_follow(boma, smash::phx::Hash40::new("sys_damage_elec"), smash::phx::Hash40::new("footr"), &FEET, &FEET, 0.25, true, 0, 0, 0, 0, 0, true, true) as u32;
				let meter_4: u32 = EffectModule::req_follow(boma, smash::phx::Hash40::new("sys_damage_elec"), smash::phx::Hash40::new("footl"), &FEET, &FEET, 0.25, true, 0, 0, 0, 0, 0, true, true) as u32;
				if KEN_SUPER[ENTRY_ID] < KEN_MAX_METER && KEN_SUPER[ENTRY_ID] >= meter_half as i32 {
					EffectModule::set_rgb(boma, meter_1, 5.0, 2.75, 0.0);
                    EffectModule::set_rgb(boma, meter_2, 5.0, 2.75, 0.0);
					EffectModule::set_visible(boma, meter_1, true);
					EffectModule::set_visible(boma, meter_2, true);
					EffectModule::set_rgb(boma, meter_3, 5.0, 2.75, 0.0);
                    EffectModule::set_rgb(boma, meter_4, 5.0, 2.75, 0.0);
					EffectModule::set_visible(boma, meter_3, true);
					EffectModule::set_visible(boma, meter_4, true);
					if color_id == 7 {
						EffectModule::set_rgb(boma, meter_3, 0.0, 7.75, 0.0);
						EffectModule::set_rgb(boma, meter_4, 0.0, 7.75, 0.0);
						EffectModule::set_rgb(boma, meter_1, 0.0, 7.75, 0.0);
						EffectModule::set_rgb(boma, meter_2, 0.0, 7.75, 0.0);	
					};
				} else if KEN_SUPER[ENTRY_ID] >= KEN_MAX_METER {
					EffectModule::set_rgb(boma, meter_1, 0.0, 5.0, 5.0);
                    EffectModule::set_rgb(boma, meter_2, 0.0, 5.0, 5.0);
					EffectModule::set_visible(boma, meter_1, true);
					EffectModule::set_visible(boma, meter_2, true);
					EffectModule::set_rgb(boma, meter_3, 0.0, 5.0, 5.0);
                    EffectModule::set_rgb(boma, meter_4, 0.0, 5.0, 5.0);
					EffectModule::set_visible(boma, meter_3, true);
					EffectModule::set_visible(boma, meter_4, true);
					if color_id == 7 {
						EffectModule::set_rgb(boma, meter_3, 10.0, 0.0, 0.0);
						EffectModule::set_rgb(boma, meter_4, 10.0, 0.0, 0.0);
						EffectModule::set_rgb(boma, meter_1, 10.0, 0.0, 0.0);
						EffectModule::set_rgb(boma, meter_2, 10.0, 0.0, 0.0);	
					};
				} else {
					EffectModule::set_visible(boma, meter_1, false);
					EffectModule::set_visible(boma, meter_2, false);
					EffectModule::set_visible(boma, meter_3, false);
					EffectModule::set_visible(boma, meter_4, false)
				}
				KEN_FX_TIMER[ENTRY_ID] = 0;
			};
			if [*FIGHTER_STATUS_KIND_ENTRY, *FIGHTER_STATUS_KIND_WIN].contains(&status_kind) {
				println!("Reset!");
				KEN_SUPER[ENTRY_ID] = 0;
				KEN_FX_TIMER[ENTRY_ID] = 0;
				KEN_IS_EX[ENTRY_ID] = false;
			};
			if !AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_ALL) {
				HAS_ADDED[ENTRY_ID] = false;
			};
			if AttackModule::is_infliction(boma, *COLLISION_KIND_MASK_ALL) && status_kind != *FIGHTER_STATUS_KIND_CATCH_ATTACK && !HAS_ADDED[ENTRY_ID] {
				KEN_SUPER[ENTRY_ID] += 3;
				println!("attacks! {} ", KEN_SUPER[ENTRY_ID]);
				HAS_ADDED[ENTRY_ID] = true;
			};
			if KEN_SUPER[ENTRY_ID] >= KEN_MAX_METER{
				KEN_SUPER[ENTRY_ID] = KEN_MAX_METER;
			};
			if status_kind == *FIGHTER_RYU_STATUS_KIND_SPECIAL_LW_STEP_F || status_kind == *FIGHTER_RYU_STATUS_KIND_SPECIAL_LW_STEP_B {
				if EX_DOWNB[ENTRY_ID] {
					if MotionModule::frame(boma) < 2.0 {
						HitModule::set_whole(boma, smash::app::HitStatus(*HIT_STATUS_XLU), 0);
					};
					if MotionModule::frame(boma) > 3.0 {
						HitModule::set_whole(boma, smash::app::HitStatus(*HIT_STATUS_NORMAL), 0);
					};
					if MotionModule::frame(boma) > 10.0 {
						CancelModule::enable_cancel(boma);
					};
				};
			} else {
				EX_DOWNB[ENTRY_ID] = false;
			};
			if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_LW {
				let mut stick_x = ControlModule::get_stick_x(boma) ;
				stick_x *= PostureModule::lr(boma);
				let is_eligble_meter = (4..25).contains(&(MotionModule::frame(boma) as i32)) && KEN_SUPER[ENTRY_ID] >= meter_half as i32;
				
				if MotionModule::frame(boma) > 25.0 || is_eligble_meter {
					if stick_x >= 0.665 {
						if is_eligble_meter {
							KEN_SUPER[ENTRY_ID] -= meter_half as i32;
							println!("meter spent! {} ", KEN_SUPER[ENTRY_ID]);
							EX_DOWNB[ENTRY_ID] = true;
						}; 
						StatusModule::change_status_request_from_script(boma, *FIGHTER_RYU_STATUS_KIND_SPECIAL_LW_STEP_F, true);
					} else if stick_x <= -0.665 {
						if is_eligble_meter {
							KEN_SUPER[ENTRY_ID] -= meter_half as i32;
							println!("meter spent! {} ", KEN_SUPER[ENTRY_ID]);
							EX_DOWNB[ENTRY_ID] = true;
						};
						StatusModule::change_status_request_from_script(boma, *FIGHTER_RYU_STATUS_KIND_SPECIAL_LW_STEP_B, true);
					};
				};
			};
			if motion_kind == hash40("attack_near_w") && AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT) && !is_hitlag(boma) {
				if (ControlModule::get_command_flag_cat(boma, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_HI3) != 0 {
					StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_HI3, true);
				} else if (ControlModule::get_command_flag_cat(boma, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_LW3) != 0 {
					StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_LW3, true);
				}else if (ControlModule::get_command_flag_cat(boma, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_S3) != 0 {
					StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_DASH, true);
				};
			};
			if [hash40("attack_s3_s_s"), hash40("attack_s3_s_w"), hash40("attack_lw3_s"), hash40("attack_hi3_s")].contains(&motion_kind) && AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_ALL) && !is_hitlag(boma) && KEN_SUPER[ENTRY_ID] >= meter_half as i32 && cancel_frame-frame > 9.0 && ControlModule::check_button_off(boma, *CONTROL_PAD_BUTTON_SPECIAL){
				let mut stick_x = ControlModule::get_stick_x(boma) ;
				stick_x *= PostureModule::lr(boma);
				if stick_x >= 0.665 && (ControlModule::get_command_flag_cat(boma, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_DASH) != 0 {
					KEN_SUPER[ENTRY_ID] -= meter_half as i32;
					println!("meter spent! {} ", KEN_SUPER[ENTRY_ID]);
					EX_DOWNB[ENTRY_ID] = true;
					StatusModule::change_status_request_from_script(boma, *FIGHTER_RYU_STATUS_KIND_SPECIAL_LW_STEP_F, true);
				} else if stick_x <= -0.665 {
					println!("meter spent! {} ", KEN_SUPER[ENTRY_ID]);
					KEN_SUPER[ENTRY_ID] -= meter_half as i32;
					EX_DOWNB[ENTRY_ID] = true;
					StatusModule::change_status_request_from_script(boma, *FIGHTER_RYU_STATUS_KIND_SPECIAL_LW_STEP_B, true);
				};
			};

			//EX
			if KEN_SUPER[ENTRY_ID] >= meter_half as i32 && !AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_ALL) {
				if [*FIGHTER_STATUS_KIND_SPECIAL_HI].contains(&status_kind) {
					if [hash40("special_hi"), hash40("special_air_hi")].contains(&motion_kind) && MotionModule::frame(boma) < 5.0 {
						if is_attack_btn(boma) {
							KEN_SUPER[ENTRY_ID] -= meter_half as i32;
							println!("meter spent! {} ", KEN_SUPER[ENTRY_ID]);
							MotionModule::change_motion(boma, Hash40::new("special_hi_ex"), -1.0, 1.0, false, 0.0, false, false);
						};
					};
				};
				if [*FIGHTER_STATUS_KIND_SPECIAL_S].contains(&status_kind) {
					if [hash40("special_s_start"), hash40("special_air_s_start")].contains(&motion_kind) && MotionModule::frame(boma) < 5.0 {
						if is_attack_btn(boma) {
							KEN_SUPER[ENTRY_ID] -= meter_half as i32;
							println!("meter spent! {} ", KEN_SUPER[ENTRY_ID]);
							MotionModule::change_motion(boma, Hash40::new("special_s_ex"), -1.0, 1.0, false, 0.0, false, false);
						};
					};
				};
				if ![*FIGHTER_STATUS_KIND_SPECIAL_S, *FIGHTER_RYU_STATUS_KIND_SPECIAL_S_COMMAND, *FIGHTER_RYU_STATUS_KIND_SPECIAL_S_LOOP, *FIGHTER_RYU_STATUS_KIND_SPECIAL_S_END].contains(&status_kind) {
					EffectModule::kill_kind(boma, Hash40::new("ken_tatsumaki_wind_r"), false, false);
					EffectModule::kill_kind(boma, Hash40::new("ken_tatsumaki_wind_l"), false, false);
				};
			};
			if [hash40("special_s_ex")].contains(&motion_kind) {
				if MotionModule::frame(boma) >= 63.0 {
					StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_FALL, false);
				};
				if MotionModule::frame(boma) >= 2.0 {
					StatusModule::set_situation_kind(boma, smash::app::SituationKind(*SITUATION_KIND_AIR), true);
					StatusModule::set_keep_situation_air(boma, true);
					if KineticModule::get_kinetic_type(boma) != *FIGHTER_KINETIC_TYPE_MOTION_AIR {
						KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_MOTION_AIR);
					};
				};
			};

			//Focus
			if [*FIGHTER_STATUS_KIND_SPECIAL_HI, *FIGHTER_RYU_STATUS_KIND_SPECIAL_HI_COMMAND].contains(&status_kind)  && MotionModule::frame(boma) < 8.0 && (ControlModule::get_command_flag_cat(boma, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_LW) != 0 && KEN_SUPER[ENTRY_ID] >= meter_half as i32  {
				StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_SPECIAL_LW, true);
			};
			if [*FIGHTER_STATUS_KIND_SPECIAL_N, *FIGHTER_RYU_STATUS_KIND_SPECIAL_N_COMMAND].contains(&status_kind) && MotionModule::frame(boma) < 14.0 && (ControlModule::get_command_flag_cat(boma, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_LW) != 0 && KEN_SUPER[ENTRY_ID] >= meter_half as i32  {
				StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_SPECIAL_LW, true);
			};
			if [*FIGHTER_STATUS_KIND_SPECIAL_S, *FIGHTER_RYU_STATUS_KIND_SPECIAL_S_COMMAND, *FIGHTER_RYU_STATUS_KIND_SPECIAL_S_LOOP, *FIGHTER_RYU_STATUS_KIND_SPECIAL_S_END].contains(&status_kind) && StatusModule::is_situation_changed(boma) && StatusModule::situation_kind(boma) == *SITUATION_KIND_GROUND {
				StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_LANDING, true);
			};
		};
	};
}	
#[acmd_script(
    agent = "ken",
    script =  "game_attacknearw",
    category = ACMD_GAME,
	low_priority)]
unsafe fn ken_prox_ftilt(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
		if macros::is_excute(fighter) {
			WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
		}
		frame(fighter.lua_state_agent, 3.0);
		if macros::is_excute(fighter) {
			macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 6.0, /*Angle*/ 180, /*KBG*/ 100, /*FKB*/ 10, /*BKB*/ 0, /*Size*/ 3.2, /*X*/ 0.0, /*Y*/ 12.5, /*Z*/ 9.0, /*X2*/ Some(0.0), /*Y2*/ Some(7.5), /*Z2*/ Some(9.0), /*Hitlag*/ 1.0, /*SDI*/ 0.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_G, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_KEN_PUNCH, /*Type*/ *ATTACK_REGION_PUNCH);
			macros::ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 6.0, /*Angle*/ 75, /*KBG*/ 23, /*FKB*/ 0, /*BKB*/ 16, /*Size*/ 3.2, /*X*/ 0.0, /*Y*/ 12.5, /*Z*/ 9.0, /*X2*/ Some(0.0), /*Y2*/ Some(7.5), /*Z2*/ Some(9.0), /*Hitlag*/ 1.0, /*SDI*/ 0.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_A, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_KEN_PUNCH, /*Type*/ *ATTACK_REGION_PUNCH);
			AttackModule::set_add_reaction_frame(fighter.module_accessor, /*ID*/ 0, /*Frames*/ 9.0, /*Unk*/ false);
		}
		frame(fighter.lua_state_agent, 5.0);
		if macros::is_excute(fighter) {
			AttackModule::clear_all(fighter.module_accessor);
			WorkModule::off_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
		}
		frame(fighter.lua_state_agent, 8.0);
		if macros::is_excute(fighter) {
			WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_RYU_STATUS_ATTACK_FLAG_HIT_CANCEL);
		}
		frame(fighter.lua_state_agent, 25.0);
		if macros::is_excute(fighter) {
			WorkModule::off_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_RYU_STATUS_ATTACK_FLAG_HIT_CANCEL);
		}
}		
#[acmd_script(
    agent = "ken",
    script =  "game_specialhiex",
    category = ACMD_GAME,
	low_priority)]
unsafe fn ken_ex_shoryu(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
		frame(fighter.lua_state_agent, 6.0);
		if macros::is_excute(fighter) {
			HitModule::set_status_all(fighter.module_accessor, smash::app::HitStatus(*HIT_STATUS_XLU), 0);
			WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
			macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 12.0, /*Angle*/ 48, /*KBG*/ 100, /*FKB*/ 77, /*BKB*/ 0, /*Size*/ 5.6, /*X*/ 0.0, /*Y*/ 14.5, /*Z*/ 7.1, /*X2*/ Some(0.0), /*Y2*/ Some(12.5), /*Z2*/ Some(8.1), /*Hitlag*/ 1.0, /*SDI*/ 0.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_fire"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_KEN_PUNCH, /*Type*/ *ATTACK_REGION_PUNCH);
			AttackModule::set_add_reaction_frame(fighter.module_accessor, /*ID*/ 0, /*Frames*/ 8.0, /*Unk*/ false);
		}
		frame(fighter.lua_state_agent, 9.0);
		if macros::is_excute(fighter) {
			macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 8.0, /*Angle*/ 50, /*KBG*/ 100, /*FKB*/ 0, /*BKB*/ 0, /*Size*/ 3.6, /*X*/ 0.0, /*Y*/ 14.5, /*Z*/ 7.1, /*X2*/ Some(0.0), /*Y2*/ Some(12.5), /*Z2*/ Some(8.1), /*Hitlag*/ 0.8, /*SDI*/ 0.5, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_fire"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_KEN_PUNCH, /*Type*/ *ATTACK_REGION_PUNCH);
			WorkModule::off_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
			WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_HI_FLAG_JUMP);
		}
		frame(fighter.lua_state_agent, 13.0);
		if macros::is_excute(fighter) {
			HitModule::set_status_all(fighter.module_accessor, smash::app::HitStatus(*HIT_STATUS_NORMAL), 0);
			AttackModule::clear_all(fighter.module_accessor);
		}
		frame(fighter.lua_state_agent, 27.0);
		if macros::is_excute(fighter) {
			macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 15.0, /*Angle*/ 80, /*KBG*/ 40, /*FKB*/ 0, /*BKB*/ 80, /*Size*/ 6.6, /*X*/ 0.0, /*Y*/ 10.0, /*Z*/ 7.6, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 2.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_fire"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_KEN_PUNCH, /*Type*/ *ATTACK_REGION_PUNCH);
		}
		frame(fighter.lua_state_agent, 36.0);
		if macros::is_excute(fighter) {
			notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
			AttackModule::clear_all(fighter.module_accessor);
		}
}
#[acmd_script(
    agent = "ken",
    script =  "effect_specialhiex",
    category = ACMD_EFFECT,
	low_priority)]
unsafe fn ken_ex_shoryu_eff(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
		frame(fighter.lua_state_agent, 4.0);
		if macros::is_excute(fighter) {
			macros::FOOT_EFFECT(fighter, Hash40::new("sys_v_smoke_a"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
			macros::LAST_EFFECT_SET_RATE(fighter, 0.7);
		}
		frame(fighter.lua_state_agent, 5.0);
		if macros::is_excute(fighter) {
			macros::BURN_COLOR(fighter, 2.47, 1.42, 0.05, 1.2);
		}
		frame(fighter.lua_state_agent, 10.0);
		if macros::is_excute(fighter) {
			macros::BURN_COLOR(fighter, 2.47, 1.42, 0.05, 1.2);
		}
		frame(fighter.lua_state_agent, 15.0);
		if macros::is_excute(fighter) {
			macros::BURN_COLOR(fighter, 2.47, 1.42, 0.05, 1.2);
		}
		frame(fighter.lua_state_agent, 22.0);
		if macros::is_excute(fighter) {
			macros::LANDING_EFFECT(fighter, Hash40::new("sys_down_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
			macros::BURN_COLOR(fighter, 2.47, 1.42, 0.05, 1.2);
		}
		frame(fighter.lua_state_agent, 27.0);
		if macros::is_excute(fighter) {
			macros::EFFECT_FOLLOW_NO_STOP(fighter, Hash40::new("ken_syoryuken_fire"), Hash40::new("handr"), 0, 0, 0, 0, 0, 0, 1, false);
			EffectModule::enable_sync_init_pos_last(fighter.module_accessor);
			macros::BURN_COLOR(fighter, 2.47, 1.42, 0.05, 1.2);
			if PostureModule::lr(fighter.module_accessor) < 0.0 {
				macros::EFFECT_FOLLOW(fighter, Hash40::new("ken_syoryuken_firearc"), Hash40::new("trans"), 3, 2, 2, 5, 0, 5, 1, false);
            	macros::EFFECT_FOLLOW(fighter, Hash40::new("ken_syoryuken_firearc2"), Hash40::new("trans"), 3, 2, 2, 5, 0, 5, 1, false);
			} else {
				macros::EFFECT_FOLLOW(fighter, Hash40::new("ken_syoryuken_firearc"), Hash40::new("trans"), -3, 2, 2, 5, 0, -5, 1, false);
            	macros::EFFECT_FOLLOW(fighter, Hash40::new("ken_syoryuken_firearc2"), Hash40::new("trans"), -3, 2, 2, 5, 0, -5, 1, false);
            	EffectModule::enable_sync_init_pos_last(fighter.module_accessor);
			}
		}
		frame(fighter.lua_state_agent, 31.0);
		if macros::is_excute(fighter) {
			macros::EFFECT_DETACH_KIND(fighter, Hash40::new("ken_syoryuken_firearc"), -1);
			macros::BURN_COLOR_FRAME(fighter, 20, 1, 1, 1, 0);
			macros::BURN_COLOR_NORMAL(fighter, );
		}
		frame(fighter.lua_state_agent, 43.0);
		if macros::is_excute(fighter) {
			macros::EFFECT_OFF_KIND(fighter, Hash40::new("ken_syoryuken_fire"), false, true);
		}
}
#[acmd_script(
    agent = "ken",
    script =  "sound_specialhiex",
    category = ACMD_SOUND,
	low_priority)]
unsafe fn ken_ex_shoryu_snd(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
		frame(fighter.lua_state_agent, 2.0);
		if macros::is_excute(fighter) {
			macros::PLAY_SE(fighter, Hash40::new("vc_ken_special_h01"));
		}
		frame(fighter.lua_state_agent, 6.0);
		if macros::is_excute(fighter) {
			macros::PLAY_SE(fighter, Hash40::new("se_ken_special_h03"));
		}
		frame(fighter.lua_state_agent, 22.0);
		if macros::is_excute(fighter) {
			macros::PLAY_LANDING_SE(fighter, Hash40::new("se_ken_landing03"));
		}
		frame(fighter.lua_state_agent, 27.0);
		if macros::is_excute(fighter) {
			macros::PLAY_SE(fighter, Hash40::new("se_ken_special_h01"));
		}
		frame(fighter.lua_state_agent, 33.0);
		if macros::is_excute(fighter) {
			macros::PLAY_SE(fighter, Hash40::new("se_ken_special_h02"));
		}
}
#[acmd_script(
    agent = "ken",
    script =  "game_specialsex",
    category = ACMD_GAME,
	low_priority)]
unsafe fn ken_ex_tatsu(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
		frame(fighter.lua_state_agent, 8.0);
		if macros::is_excute(fighter) {
			macros::HIT_NODE(fighter, Hash40::new("kneer"), *HIT_STATUS_XLU);
			macros::HIT_NODE(fighter, Hash40::new("legr"), *HIT_STATUS_XLU);
			WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
			macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 3.0, /*Angle*/ 65, /*KBG*/ 100, /*FKB*/ 100, /*BKB*/ 0, /*Size*/ 4.5, /*X*/ 0.0, /*Y*/ 9.0, /*Z*/ 5.5, /*X2*/ Some(0.0), /*Y2*/ Some(9.0), /*Z2*/ Some(-1.5), /*Hitlag*/ 1.5, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_THRU, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_KEN_KICK, /*Type*/ *ATTACK_REGION_KICK);
			FighterAreaModuleImpl::enable_fix_jostle_area_xy(fighter.module_accessor, 3.0, 3.5, 8.5, 4.5);
		}
		frame(fighter.lua_state_agent, 9.0);
		if macros::is_excute(fighter) {
			AttackModule::clear_all(fighter.module_accessor);
		}
		frame(fighter.lua_state_agent, 14.0);
		if macros::is_excute(fighter) {
			WorkModule::off_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
			macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 3.0, /*Angle*/ 367, /*KBG*/ 100, /*FKB*/ 100, /*BKB*/ 0, /*Size*/ 3.5, /*X*/ 0.0, /*Y*/ 12.5, /*Z*/ 12.5, /*X2*/ Some(0.0), /*Y2*/ Some(12.5), /*Z2*/ Some(0.5), /*Hitlag*/ 1.0, /*SDI*/ 0.5, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_THRU, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_KEN_KICK, /*Type*/ *ATTACK_REGION_KICK);
		}
		frame(fighter.lua_state_agent, 17.0);
		if macros::is_excute(fighter) {
			AttackModule::clear_all(fighter.module_accessor);
		}
		frame(fighter.lua_state_agent, 22.0);
		if macros::is_excute(fighter) {
			macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 3.0, /*Angle*/ 367, /*KBG*/ 100, /*FKB*/ 100, /*BKB*/ 0, /*Size*/ 3.5, /*X*/ 0.0, /*Y*/ 12.5, /*Z*/ 12.5, /*X2*/ Some(0.0), /*Y2*/ Some(12.5), /*Z2*/ Some(0.5), /*Hitlag*/ 1.0, /*SDI*/ 0.5, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_THRU, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_KEN_KICK, /*Type*/ *ATTACK_REGION_KICK);
		}
		frame(fighter.lua_state_agent, 25.0);
		if macros::is_excute(fighter) {
			AttackModule::clear_all(fighter.module_accessor);
		}
		frame(fighter.lua_state_agent, 30.0);
		if macros::is_excute(fighter) {
			macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 3.0, /*Angle*/ 367, /*KBG*/ 100, /*FKB*/ 100, /*BKB*/ 0, /*Size*/ 3.5, /*X*/ 0.0, /*Y*/ 12.5, /*Z*/ 12.5, /*X2*/ Some(0.0), /*Y2*/ Some(12.5), /*Z2*/ Some(0.5), /*Hitlag*/ 1.0, /*SDI*/ 0.5, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_THRU, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_KEN_KICK, /*Type*/ *ATTACK_REGION_KICK);
		}
		frame(fighter.lua_state_agent, 33.0);
		if macros::is_excute(fighter) {
			AttackModule::clear_all(fighter.module_accessor);
		}
		frame(fighter.lua_state_agent, 38.0);
		if macros::is_excute(fighter) {
			macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 6.0, /*Angle*/ 37, /*KBG*/ 108, /*FKB*/ 0, /*BKB*/ 65, /*Size*/ 5.5, /*X*/ 0.0, /*Y*/ 12.5, /*Z*/ 12.5, /*X2*/ Some(0.0), /*Y2*/ Some(12.5), /*Z2*/ Some(-2.5), /*Hitlag*/ 1.5, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_THRU, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_KEN_KICK, /*Type*/ *ATTACK_REGION_KICK);
		}
		frame(fighter.lua_state_agent, 42.0);
		if macros::is_excute(fighter) {
			macros::HIT_NODE(fighter, Hash40::new("kneer"), *HIT_STATUS_NORMAL);
			macros::HIT_NODE(fighter, Hash40::new("legr"), *HIT_STATUS_NORMAL);
			AttackModule::clear_all(fighter.module_accessor);
		}
}
#[acmd_script(
    agent = "ken",
    script =  "effect_specialsex",
    category = ACMD_EFFECT,
	low_priority)]
unsafe fn ken_ex_tatsu_eff(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
		frame(fighter.lua_state_agent, 5.0);
		if macros::is_excute(fighter) {
			macros::BURN_COLOR(fighter, 2.47, 1.42, 0.05, 1.2);
		}
		frame(fighter.lua_state_agent, 8.0);
		if macros::is_excute(fighter) {
			macros::FOOT_EFFECT(fighter, Hash40::new("sys_dash_smoke"), Hash40::new("top"), -4, 0, 0, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, false);
			macros::EFFECT_ALPHA(fighter, Hash40::new("sys_attack_impact"), Hash40::new("top"), 0, 10.5, 6, 0, 0, 0, 1.1, 0, 0, 0, 0, 0, 0, true, 0.7);
		}
		frame(fighter.lua_state_agent, 10.0);
		if macros::is_excute(fighter) {
			macros::BURN_COLOR(fighter, 2.47, 1.42, 0.05, 1.2);
		}
		frame(fighter.lua_state_agent, 10.0);
		if macros::is_excute(fighter) {
			if PostureModule::lr(fighter.module_accessor) < 0.0 {
				macros::EFFECT_FOLLOW(fighter, Hash40::new("ken_tatsumaki_wind_l"), Hash40::new("top"), 0, 11.0, 0, 0, 0, 0, 1.1, false);
			} else {
				macros::EFFECT_FOLLOW(fighter, Hash40::new("ken_tatsumaki_wind_r"), Hash40::new("top"), 0, 11.0, 0, 0, 0, 0, 1.1, false);
			}
			macros::LAST_EFFECT_SET_RATE(fighter, 1.1);
			macros::LAST_EFFECT_SET_COLOR(fighter, 2.47, 1.42, 0.05);
			macros::BURN_COLOR(fighter, 2.47, 1.42, 0.05, 1.2);
		}
		frame(fighter.lua_state_agent, 14.0);
		for _ in 0..4  {
			if macros::is_excute(fighter) {
				macros::EFFECT_FLIP(fighter, Hash40::new("ken_tatsumaki_smoke_r"), Hash40::new("ken_tatsumaki_smoke_l"), Hash40::new("top"), 2, 0, 0, 0, 0, 0, 1.1, 0, 0, 0, 0, 0, 0, false, *EF_FLIP_NONE);
				macros::LAST_EFFECT_SET_RATE(fighter, 1.1);
				macros::LAST_EFFECT_SET_COLOR(fighter, 2.47, 1.42, 0.05);
				macros::BURN_COLOR(fighter, 2.47, 1.42, 0.05, 1.2);
			}
			wait(fighter.lua_state_agent, 8.0);
		}
		frame(fighter.lua_state_agent, 42.0);
		if macros::is_excute(fighter) {
			macros::BURN_COLOR_FRAME(fighter, 20, 1, 1, 1, 0);
			macros::BURN_COLOR_NORMAL(fighter, );
			macros::EFFECT_OFF_KIND(fighter, Hash40::new("ken_tatsumaki_wind_l"), false, true);
			macros::EFFECT_OFF_KIND(fighter, Hash40::new("ken_tatsumaki_wind_r"), false, true);
		}
}
#[acmd_script(
    agent = "ken",
    script =  "sound_specialsex",
    category = ACMD_SOUND,
	low_priority)]
unsafe fn ken_ex_tatsu_snd(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
		frame(fighter.lua_state_agent, 7.0);
		if macros::is_excute(fighter) {
			macros::PLAY_SE(fighter, Hash40::new("vc_ken_special_s02"));
			macros::PLAY_SE(fighter, Hash40::new("se_ken_command_success"));
		}
		frame(fighter.lua_state_agent, 14.0);
		if macros::is_excute(fighter) {
			macros::PLAY_SE(fighter, Hash40::new("se_ken_special_s01"));
		}
}
pub fn install() {
    smashline::install_acmd_scripts!(
		ken_dsmash,
		ken_jab3,
		ken_ex_shoryu,
		ken_ex_shoryu_eff,
		ken_ex_shoryu_snd,
		ken_ex_tatsu,
		ken_ex_tatsu_eff,
		ken_ex_tatsu_snd
    );
    smashline::install_agent_frame_callbacks!(supers);
}
