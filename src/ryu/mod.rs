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


static mut max_meter : i32 = 1;
static mut PPP: i32 = 3;
static mut RYU_SUPER : [i32; 8] = [0; 8];
static mut HAS_ADDED : [bool; 8] = [false; 8];
static mut RYU_FX_TIMER : [i32; 8] = [0; 8];
static mut IS_SUPER : [bool; 8] = [false; 8];
static mut HANDS :  smash::phx::Vector3f =  smash::phx::Vector3f { x: 0.0, y: 0.0, z: 0.0 };
static mut FEET :  smash::phx::Vector3f =  smash::phx::Vector3f { x: 1.0, y: 0.0, z: 0.0 };

#[acmd_script(
    agent = "ryu",
    script =  "game_attacklw4",
    category = ACMD_GAME,
	low_priority)]
unsafe fn ryu_dsmash(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
		frame(fighter.lua_state_agent, 1.0);
		if macros::is_excute(fighter) {
			WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_RYU_STATUS_ATTACK_FLAG_HIT_CANCEL);
		}
		frame(fighter.lua_state_agent, 3.0);
		if macros::is_excute(fighter) {
			WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
		}
		frame(fighter.lua_state_agent, 5.0);
		if macros::is_excute(fighter) {
			WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
			macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 12.0, /*Angle*/ 80, /*KBG*/ 47, /*FKB*/ 0, /*BKB*/ 50, /*Size*/ 3.6, /*X*/ 0.0, /*Y*/ 2.5, /*Z*/ 12.0, /*X2*/ Some(0.0), /*Y2*/ Some(3.0), /*Z2*/ Some(8.0), /*Hitlag*/ 1.5, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ -8, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_RYU_KICK, /*Type*/ *ATTACK_REGION_KICK);
			macros::ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 8.0, /*Angle*/ 80, /*KBG*/ 50, /*FKB*/ 0, /*BKB*/ 50, /*Size*/ 2.5, /*X*/ 0.0, /*Y*/ 3.0, /*Z*/ 2.5, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.5, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ -6, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_RYU_KICK, /*Type*/ *ATTACK_REGION_KICK);
			macros::ATTACK(fighter, /*ID*/ 2, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 8.0, /*Angle*/ 80, /*KBG*/ 50, /*FKB*/ 0, /*BKB*/ 50, /*Size*/ 2.5, /*X*/ 0.0, /*Y*/ 3.0, /*Z*/ 2.5, /*X2*/ Some(0.0), /*Y2*/ Some(3.0), /*Z2*/ Some(0.0), /*Hitlag*/ 1.5, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ -6, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_RYU_KICK, /*Type*/ *ATTACK_REGION_KICK);
			AttackModule::set_add_reaction_frame(fighter.module_accessor, /*ID*/ 0, /*Frames*/ 5.0, /*Unk*/ false);
			AttackModule::set_add_reaction_frame(fighter.module_accessor, /*ID*/ 1, /*Frames*/ 5.0, /*Unk*/ false);
			AttackModule::set_add_reaction_frame(fighter.module_accessor, /*ID*/ 2, /*Frames*/ 5.0, /*Unk*/ false);
		}
		wait(fighter.lua_state_agent, 2.0);
		if macros::is_excute(fighter) {
			AttackModule::clear_all(fighter.module_accessor);
		}
		frame(fighter.lua_state_agent, 31.0);
		if macros::is_excute(fighter) {
			WorkModule::off_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_RYU_STATUS_ATTACK_FLAG_HIT_CANCEL);
			WorkModule::off_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
		}
}
#[acmd_script(
    agent = "ryu",
    script =  "game_attackairf",
    category = ACMD_GAME,
	low_priority)]
unsafe fn ryu_fair(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
		if macros::is_excute(fighter) {
			WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
			WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_RYU_STATUS_ATTACK_FLAG_HIT_CANCEL);
		}
		macros::FT_MOTION_RATE(fighter, /*FSM*/ 1.34);
		frame(fighter.lua_state_agent, 2.0);
		if macros::is_excute(fighter) {
			WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
		}
		frame(fighter.lua_state_agent, 5.5);
		if macros::is_excute(fighter) {
			macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 8.0, /*Angle*/ 61, /*KBG*/ 56, /*FKB*/ 0, /*BKB*/ 65, /*Size*/ 4.3, /*X*/ 0.0, /*Y*/ 5.5, /*Z*/ 10.6, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.5, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_RYU_KICK, /*Type*/ *ATTACK_REGION_KICK);
			macros::ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 8.0, /*Angle*/ 70, /*KBG*/ 25, /*FKB*/ 0, /*BKB*/ 65, /*Size*/ 3.8, /*X*/ 0.0, /*Y*/ 5.5, /*Z*/ 2.5, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.5, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 15, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_RYU_KICK, /*Type*/ *ATTACK_REGION_KICK);
		}
		frame(fighter.lua_state_agent, 7.0);
		macros::FT_MOTION_RATE(fighter, /*FSM*/ 0.6);
		if macros::is_excute(fighter) {
			macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 7.0, /*Angle*/ 61, /*KBG*/ 56, /*FKB*/ 0, /*BKB*/ 65, /*Size*/ 4.0, /*X*/ 0.0, /*Y*/ 5.5, /*Z*/ 10.6, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.5, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_RYU_KICK, /*Type*/ *ATTACK_REGION_KICK);
			macros::ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 7.0, /*Angle*/ 70, /*KBG*/ 25, /*FKB*/ 0, /*BKB*/ 65, /*Size*/ 4.6, /*X*/ 0.0, /*Y*/ 5.5, /*Z*/ 2.5, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.5, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 15, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_RYU_KICK, /*Type*/ *ATTACK_REGION_KICK);
		}
		frame(fighter.lua_state_agent, 15.0);
		macros::FT_MOTION_RATE(fighter, /*FSM*/ 0.8);
		if macros::is_excute(fighter) {
			AttackModule::clear_all(fighter.module_accessor);
		}
		frame(fighter.lua_state_agent, 35.0);
		if macros::is_excute(fighter) {
			WorkModule::off_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
			WorkModule::off_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_RYU_STATUS_ATTACK_FLAG_HIT_CANCEL);
			WorkModule::off_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
		}
}
#[acmd_script(
    agent = "ryu",
    script =  "game_attackdash",
    category = ACMD_GAME,
	low_priority)]
unsafe fn ryu_da(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
		if macros::is_excute(fighter) {
			WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
			WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_RYU_STATUS_ATTACK_FLAG_HIT_CANCEL);
		}
		frame(fighter.lua_state_agent, 7.0);
		if macros::is_excute(fighter) {
			macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 12.0, /*Angle*/ 75, /*KBG*/ 35, /*FKB*/ 0, /*BKB*/ 80, /*Size*/ 4.2, /*X*/ 0.0, /*Y*/ 9.0, /*Z*/ 10.0, /*X2*/ Some(0.0), /*Y2*/ Some(9.0), /*Z2*/ Some(7.2), /*Hitlag*/ 1.5, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 2, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_RYU_KICK, /*Type*/ *ATTACK_REGION_KICK);
		}
		wait(fighter.lua_state_agent, 3.0);
		if macros::is_excute(fighter) {
			macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 8.0, /*Angle*/ 80, /*KBG*/ 30, /*FKB*/ 0, /*BKB*/ 80, /*Size*/ 3.5, /*X*/ 0.0, /*Y*/ 9.0, /*Z*/ 8.5, /*X2*/ Some(0.0), /*Y2*/ Some(9.0), /*Z2*/ Some(7.2), /*Hitlag*/ 1.5, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 2, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_RYU_KICK, /*Type*/ *ATTACK_REGION_KICK);
		}
		wait(fighter.lua_state_agent, 6.0);
		if macros::is_excute(fighter) {
			AttackModule::clear_all(fighter.module_accessor);
		}
		wait(fighter.lua_state_agent, 12.0);
		if macros::is_excute(fighter) {
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
		let mut fighter_num = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
		if fighter_num > 7 {
			fighter_num = 7;
		};
		let aura_size = RYU_SUPER[fighter_num] as f32 * 0.1;
		if fighter_kind == *FIGHTER_KIND_RYU || WorkModule::get_int(boma, *FIGHTER_KIRBY_INSTANCE_WORK_ID_INT_COPY_CHARA) == *FIGHTER_KIND_RYU {
			RYU_FX_TIMER[fighter_num] += 1;
			max_meter = 51; //51
			if smash::app::smashball::is_training_mode() == true {
				if status_kind == *FIGHTER_STATUS_KIND_APPEAL {
					RYU_SUPER[fighter_num] = max_meter;
					CancelModule::enable_cancel(boma);
				};
			};
			if RYU_FX_TIMER[fighter_num] == 6 {
				let meter_half = max_meter as f32 * 0.5;
				let meter_1: u32 = EffectModule::req_follow(boma, smash::phx::Hash40::new("sys_damage_elec"), smash::phx::Hash40::new("haver"), &HANDS, &HANDS, 0.2, true, 0, 0, 0, 0, 0, true, true) as u32;
				let meter_2: u32 = EffectModule::req_follow(boma, smash::phx::Hash40::new("sys_damage_elec"), smash::phx::Hash40::new("havel"), &HANDS, &HANDS, 0.2, true, 0, 0, 0, 0, 0, true, true) as u32;
				let meter_3: u32 = EffectModule::req_follow(boma, smash::phx::Hash40::new("sys_damage_elec"), smash::phx::Hash40::new("footr"), &FEET, &FEET, 0.25, true, 0, 0, 0, 0, 0, true, true) as u32;
				let meter_4: u32 = EffectModule::req_follow(boma, smash::phx::Hash40::new("sys_damage_elec"), smash::phx::Hash40::new("footl"), &FEET, &FEET, 0.25, true, 0, 0, 0, 0, 0, true, true) as u32;
				if RYU_SUPER[fighter_num] < max_meter && RYU_SUPER[fighter_num] >= meter_half as i32 {
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
				} else if RYU_SUPER[fighter_num] >= max_meter {
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
				RYU_FX_TIMER[fighter_num] = 0;
			};
				
			if [*FIGHTER_STATUS_KIND_ENTRY, *FIGHTER_STATUS_KIND_WIN].contains(&status_kind) {
				println!("Reset!");
				RYU_SUPER[fighter_num] = 0;
				RYU_FX_TIMER[fighter_num] = 0;
			};
			if !AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_ALL) {
				HAS_ADDED[fighter_num] = false;
			};
			if AttackModule::is_infliction(boma, *COLLISION_KIND_MASK_ALL) && status_kind != *FIGHTER_STATUS_KIND_CATCH_ATTACK && !HAS_ADDED[fighter_num] {
				RYU_SUPER[fighter_num] += 3;
				println!("attacks! {} ", RYU_SUPER[fighter_num]);
				if RYU_SUPER[fighter_num] == max_meter {
					println!("Super active {} ", RYU_SUPER[fighter_num]);
				};
				HAS_ADDED[fighter_num] = true;
			};
			if RYU_SUPER[fighter_num] >= max_meter {
				RYU_SUPER[fighter_num] = max_meter;
			};
			if RYU_SUPER[fighter_num] >= max_meter && WorkModule::is_flag(boma, *FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL) && (ControlModule::get_command_flag_cat(boma, 3) & *FIGHTER_PAD_CMD_CAT4_FLAG_SPECIAL_N_COMMAND) != 0 {
				StatusModule::change_status_request_from_script(boma, *FIGHTER_RYU_STATUS_KIND_SPECIAL_N_COMMAND, true);
			};
			if RYU_SUPER[fighter_num] == max_meter && status_kind == *FIGHTER_RYU_STATUS_KIND_SPECIAL_N_COMMAND && MotionModule::frame(boma) < 14.0  && MotionModule::frame(boma) > 12.9 {
				ArticleModule::generate_article(boma, *FIGHTER_RYU_GENERATE_ARTICLE_SHINKUHADOKEN, false, 0);
				RYU_SUPER[fighter_num] = 0;
				println!("Shinsu!!!");
				println!("The aura_size is {}", aura_size);
			} else if [hash40("special_n"), hash40("special_air_n")].contains(&MotionModule::motion_kind(boma)) && MotionModule::frame(boma) < 14.0  && MotionModule::frame(boma) > 12.9 {
				WorkModule::on_flag(boma, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_N_FLAG_SHOOT);
			};
		};
	};
}
#[acmd_script(
    agent = "ryu_shinkuhadoken",
    script =  "game_move",
    category = ACMD_GAME,
	low_priority)]
unsafe fn ryu_shinsu(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
		frame(fighter.lua_state_agent, 1.0);
		if macros::is_excute(fighter) {
			PostureModule::set_scale(fighter.module_accessor, 0.5, false);
			macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 0.8, /*Angle*/ 366, /*KBG*/ 100, /*FKB*/ 90, /*BKB*/ 0, /*Size*/ 8.0, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.0, /*SDI*/ 0.5, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ -5, /*Trip*/ 0.0, /*Rehit*/ 5, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ true, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_FIRE, /*Type*/ *ATTACK_REGION_NONE);
			macros::ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 0.8, /*Angle*/ 366, /*KBG*/ 100, /*FKB*/ 75, /*BKB*/ 0, /*Size*/ 18.0, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.0, /*SDI*/ 0.5, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ -5, /*Trip*/ 0.0, /*Rehit*/ 5, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ true, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_NONE, /*Type*/ *ATTACK_REGION_NONE);
			AttackModule::set_add_reaction_frame(fighter.module_accessor, /*ID*/ 0, /*Frames*/ 10.0, /*Unk*/ false);
			AttackModule::set_add_reaction_frame(fighter.module_accessor, /*ID*/ 1, /*Frames*/ 10.0, /*Unk*/ false);
		}
		frame(fighter.lua_state_agent, 90.0);
		if macros::is_excute(fighter) {
			AttackModule::clear_all(fighter.module_accessor);
			macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 1, /*Bone*/ Hash40::new("top"), /*Damage*/ 7.0, /*Angle*/ 75, /*KBG*/ 127, /*FKB*/ 0, /*BKB*/ 90, /*Size*/ 16.0, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.5, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ -5, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_FIRE, /*Type*/ *ATTACK_REGION_NONE);
		}
		wait(fighter.lua_state_agent, 6.0);
		if macros::is_excute(fighter) {
			AttackModule::clear_all(fighter.module_accessor);
		}
}
#[acmd_script(
    agent = "ryu_shinkuhadoken",
    script =  "effect_move",
    category = ACMD_EFFECT,
	low_priority)]
unsafe fn ryu_shinsue(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
		if macros::is_excute(fighter) {
			macros::EFFECT_FOLLOW(fighter, Hash40::new("ryu_final_shinkuhado_bullet"), Hash40::new("top"), 0, 0, 1, 0, 0, 0, 1.2, true);
		}
		frame(fighter.lua_state_agent, 90.0);
		if macros::is_excute(fighter) {
			macros::EFFECT(fighter, Hash40::new("ryu_final_shinkuhado_finish"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
		}
}			
#[acmd_script(
    agent = "ryu",
    script =  "game_specialn",
    category = ACMD_GAME,
	low_priority)]
unsafe fn ryu_shinsu_hadou(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
		wait(fighter.lua_state_agent, 1.0);
		macros::FT_MOTION_RATE(fighter, /*FSM*/ 0.916);
		frame(fighter.lua_state_agent, 10.0);
		if macros::is_excute(fighter) {
			WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_N_FLAG_SPECIAL_FALL);
		}
		macros::FT_MOTION_RATE(fighter, /*FSM*/ 1);
		frame(fighter.lua_state_agent, 14.0);
		if macros::is_excute(fighter) {
			WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
		}
		frame(fighter.lua_state_agent, 22.0);
		if macros::is_excute(fighter) {
			WorkModule::off_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
		}
		frame(fighter.lua_state_agent, 28.0);
		if macros::is_excute(fighter) {
			WorkModule::off_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_N_FLAG_SPECIAL_FALL);
		}
}			
#[acmd_script(
    agent = "ryu",
    script =  "game_specialairn",
    category = ACMD_GAME,
	low_priority)]
unsafe fn ryu_shinsu_hadou_air(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
		wait(fighter.lua_state_agent, 1.0);
		macros::FT_MOTION_RATE(fighter, /*FSM*/ 0.916);
		frame(fighter.lua_state_agent, 10.0);
		if macros::is_excute(fighter) {
			WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_N_FLAG_SPECIAL_FALL);
		}
		macros::FT_MOTION_RATE(fighter, /*FSM*/ 1);
		frame(fighter.lua_state_agent, 14.0);
		if macros::is_excute(fighter) {
			WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
		}
		frame(fighter.lua_state_agent, 22.0);
		if macros::is_excute(fighter) {
			WorkModule::off_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
		}
		frame(fighter.lua_state_agent, 28.0);
		if macros::is_excute(fighter) {
			WorkModule::off_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_N_FLAG_SPECIAL_FALL);
		}
}	
pub fn install() {
    smashline::install_acmd_scripts!(
		ryu_shinsu_hadou,
		ryu_shinsu_hadou_air,
		ryu_shinsu,
		ryu_shinsue,
		ryu_dsmash,
		ryu_da,
		ryu_fair
    );
    smashline::install_agent_frame_callbacks!(supers);
}
