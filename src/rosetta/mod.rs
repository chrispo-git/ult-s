use smash::hash40;
use smash::lib::lua_const::*;
use smash::app::lua_bind::*;
use smash::lua2cpp::*;
use smashline::*;
use smash_script::*;
use smash::app::sv_animcmd::*;
use smash::phx::*;
use smash::app::ArticleOperationTarget;
use smash::lib::L2CValue;
use crate::util::*;

static mut TICO_X : [f32; 8] = [0.0; 8];
static mut TICO_Y : [f32; 8] = [0.0; 8];
static mut ROSA_X : [f32; 8] = [0.0; 8];
static mut ROSA_Y : [f32; 8] = [0.0; 8];
static mut IS_TELEPORT : [bool; 8] = [false; 8];
static mut IS_TICO_DEAD : [bool; 8] = [false; 8];
static mut INVIS_FRAMES : [i32; 8] = [0; 8];
static mut MAX_INVIS_FRAMES : i32 = 20;
static mut COOLDOWN : [i32; 8] = [0; 8];
static mut TELEPORT_COOLDOWN : i32 = 300;
static mut HANDS :  smash::phx::Vector3f =  smash::phx::Vector3f { x: 0.0, y: 0.0, z: 0.0 };

#[acmd_script(
    agent = "rosetta",
    script =  "game_attack11",
    category = ACMD_GAME,
	low_priority)]
unsafe fn rosa_jab(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
		frame(fighter.lua_state_agent, 7.0);
		if macros::is_excute(fighter) {
			macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 2.0, /*Angle*/ 361, /*KBG*/ 35, /*FKB*/ 0, /*BKB*/ 25, /*Size*/ 3.2, /*X*/ 0.0, /*Y*/ 11.0, /*Z*/ 6.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.5, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_magic"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_MAGIC, /*Type*/ *ATTACK_REGION_MAGIC);
			macros::ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 2.0, /*Angle*/ 180, /*KBG*/ 20, /*FKB*/ 0, /*BKB*/ 20, /*Size*/ 3.2, /*X*/ 0.0, /*Y*/ 11.0, /*Z*/ 11.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.5, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_FIGHTER, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_magic"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_MAGIC, /*Type*/ *ATTACK_REGION_MAGIC);
			macros::ATTACK(fighter, /*ID*/ 2, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 2.0, /*Angle*/ 361, /*KBG*/ 20, /*FKB*/ 0, /*BKB*/ 20, /*Size*/ 3.2, /*X*/ 0.0, /*Y*/ 11.0, /*Z*/ 11.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.5, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_magic"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_MAGIC, /*Type*/ *ATTACK_REGION_MAGIC);
			AttackModule::set_add_reaction_frame(fighter.module_accessor, /*ID*/ 0, /*Frames*/ 6.0, /*Unk*/ false);
			AttackModule::set_add_reaction_frame(fighter.module_accessor, /*ID*/ 1, /*Frames*/ 6.0, /*Unk*/ false);
			AttackModule::set_add_reaction_frame(fighter.module_accessor, /*ID*/ 2, /*Frames*/ 6.0, /*Unk*/ false);
		}
		wait(fighter.lua_state_agent, 1.0);
		if macros::is_excute(fighter) {
			AttackModule::clear_all(fighter.module_accessor);
		}
		frame(fighter.lua_state_agent, 10.0);
		if macros::is_excute(fighter) {
			WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO);
		}
}
#[acmd_script(
    agent = "rosetta",
    scripts =  ["game_specialhistart", "game_specialairhistart"],
    category = ACMD_GAME,
	low_priority)]
unsafe fn rosa_upb_start(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
		frame(fighter.lua_state_agent, 6.0);
		if macros::is_excute(fighter) {
			macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("hip"), /*Damage*/ 5.0, /*Angle*/ 367, /*KBG*/ 100, /*FKB*/ 90, /*BKB*/ 0, /*Size*/ 13.5, /*X*/ 0.0, /*Y*/ 11.0, /*Z*/ 10.0, /*X2*/ Some(0.0), /*Y2*/ Some(11.0), /*Z2*/ Some(15.0), /*Hitlag*/ 0.75, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_magic"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_MAGIC, /*Type*/ *ATTACK_REGION_MAGIC);
		}
}	
#[acmd_script(
    agent = "rosetta",
    scripts =  ["game_specialhi", "game_specialairhi"],
    category = ACMD_GAME,
	low_priority)]
unsafe fn rosa_upb(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
		if macros::is_excute(fighter) {
			JostleModule::set_status(fighter.module_accessor, false);
			macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("hip"), /*Damage*/ 2.5, /*Angle*/ 367, /*KBG*/ 100, /*FKB*/ 90, /*BKB*/ 0, /*Size*/ 13.5, /*X*/ 0.0, /*Y*/ 11.0, /*Z*/ 10.0, /*X2*/ Some(0.0), /*Y2*/ Some(11.0), /*Z2*/ Some(15.0), /*Hitlag*/ 0.75, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 4, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_magic"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_MAGIC, /*Type*/ *ATTACK_REGION_MAGIC);
		}
		frame(fighter.lua_state_agent, 12.0);
		if macros::is_excute(fighter) {
			notify_event_msc_cmd!(fighter, 0x2127e37c07u64, *GROUND_CLIFF_CHECK_KIND_ALWAYS);
		}
		frame(fighter.lua_state_agent, 17.0);
		if macros::is_excute(fighter) {	
			AttackModule::clear_all(fighter.module_accessor);
			macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("hip"), /*Damage*/ 8.5, /*Angle*/ 363, /*KBG*/ 70, /*FKB*/ 0, /*BKB*/ 74, /*Size*/ 17.5, /*X*/ 0.0, /*Y*/ 11.0, /*Z*/ 10.0, /*X2*/ Some(0.0), /*Y2*/ Some(11.0), /*Z2*/ Some(15.0), /*Hitlag*/ 1.2, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_magic"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_MAGIC, /*Type*/ *ATTACK_REGION_MAGIC);
		}
		frame(fighter.lua_state_agent, 19.0);
		if macros::is_excute(fighter) {
			AttackModule::clear_all(fighter.module_accessor);
		}
}		
#[acmd_script(
    agent = "rosetta",
    script =  "game_attackhi3",
    category = ACMD_GAME,
	low_priority)]
unsafe fn rosa_utilt(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
		frame(fighter.lua_state_agent, 2.0);
		macros::FT_MOTION_RATE(fighter, /*FSM*/ 3);
		wait(fighter.lua_state_agent, 1.0);
		macros::FT_MOTION_RATE(fighter, /*FSM*/ 1);
		frame(fighter.lua_state_agent, 4.0);
		if macros::is_excute(fighter) {
			ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_ROSETTA_GENERATE_ARTICLE_RING, false, 0);
		}
		frame(fighter.lua_state_agent, 7.0);
		if macros::is_excute(fighter) {
			macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("throw"), /*Damage*/ 10.0, /*Angle*/ 88, /*KBG*/ 65, /*FKB*/ 0, /*BKB*/ 75, /*Size*/ 4.0, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ -3.5, /*X2*/ Some(0.0), /*Y2*/ Some(0.0), /*Z2*/ Some(3.5), /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_magic"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_MAGIC, /*Type*/ *ATTACK_REGION_MAGIC);
		}
		frame(fighter.lua_state_agent, 11.0);
		if macros::is_excute(fighter) {
			macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("throw"), /*Damage*/ 8.0, /*Angle*/ 88, /*KBG*/ 66, /*FKB*/ 0, /*BKB*/ 75, /*Size*/ 4.0, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ -3.5, /*X2*/ Some(0.0), /*Y2*/ Some(0.0), /*Z2*/ Some(3.5), /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_magic"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_MAGIC, /*Type*/ *ATTACK_REGION_MAGIC);
		}
		frame(fighter.lua_state_agent, 18.0);
		if macros::is_excute(fighter) {
			AttackModule::clear_all(fighter.module_accessor);
			ArticleModule::remove(fighter.module_accessor, *FIGHTER_ROSETTA_GENERATE_ARTICLE_RING,smash::app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
		}
}
#[acmd_script(
    agent = "rosetta_tico",
    script =  "game_explode",
    category = ACMD_GAME,
	low_priority)]
unsafe fn luma_boom(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
	let status_kind = smash::app::lua_bind::StatusModule::status_kind(fighter.module_accessor);
	if ![*WEAPON_ROSETTA_TICO_STATUS_KIND_DEAD, *WEAPON_ROSETTA_TICO_STATUS_KIND_NONE].contains(&status_kind) {
		frame(fighter.lua_state_agent, 21.0);
		if macros::is_excute(fighter) {
			macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("rot"), /*Damage*/ 18.0, /*Angle*/ 361, /*KBG*/ 102, /*FKB*/ 0, /*BKB*/ 35, /*Size*/ 15.0, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.5, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_fire"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_BOMB, /*Type*/ *ATTACK_REGION_MAGIC);
		}
		frame(fighter.lua_state_agent, 24.0);
		if macros::is_excute(fighter) {
			AttackModule::clear_all(fighter.module_accessor);
			StatusModule::change_status_request_from_script(fighter.module_accessor, *WEAPON_ROSETTA_TICO_STATUS_KIND_DEAD, false);
		}
	}
}
#[acmd_script(
    agent = "rosetta_tico",
    script =  "effect_explode",
    category = ACMD_EFFECT,
	low_priority)]
unsafe fn luma_boom_eff(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
	let status_kind = smash::app::lua_bind::StatusModule::status_kind(fighter.module_accessor);
	if ![*WEAPON_ROSETTA_TICO_STATUS_KIND_DEAD, *WEAPON_ROSETTA_TICO_STATUS_KIND_NONE].contains(&status_kind) {
		frame(fighter.lua_state_agent, 21.0);
		if macros::is_excute(fighter) {
			macros::EFFECT(fighter, Hash40::new("rosetta_tico_dead"), Hash40::new("hip"), 1, 0, 0, 0, 0, -90, 1.6, 0, 0, 0, 0, 0, 0, true);
			macros::QUAKE(fighter, *CAMERA_QUAKE_KIND_M);
			WorkModule::on_flag(fighter.module_accessor, *WEAPON_ROSETTA_TICO_INSTANCE_WORK_ID_FLAG_CAMERA_OFF);
		}
	}
}
#[acmd_script(
    agent = "rosetta_tico",
    script =  "sound_explode",
    category = ACMD_SOUND,
	low_priority)]
unsafe fn luma_boom_snd(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
	let status_kind = smash::app::lua_bind::StatusModule::status_kind(fighter.module_accessor);
	if ![*WEAPON_ROSETTA_TICO_STATUS_KIND_DEAD, *WEAPON_ROSETTA_TICO_STATUS_KIND_NONE].contains(&status_kind) {
		frame(fighter.lua_state_agent, 21.0);
		if macros::is_excute(fighter) {
			macros::PLAY_SE(fighter, Hash40::new("se_common_bomb_ll"));
		}
	}
}
#[fighter_frame( agent = FIGHTER_KIND_ROSETTA )]
fn rosa_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
			let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent); 
			let status_kind = smash::app::lua_bind::StatusModule::status_kind(boma);
			let fighter_kind = smash::app::utility::get_kind(boma);
			let ENTRY_ID = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
			let frame = MotionModule::frame(boma);
			if IS_TICO_DEAD[ENTRY_ID] == true || COOLDOWN[ENTRY_ID] > 0{
				CAN_DOWNB[ENTRY_ID] = 1;
			} else {
				CAN_DOWNB[ENTRY_ID] = 0;
			};
			if smash::app::sv_information::is_ready_go() == false {
				COOLDOWN[ENTRY_ID] = 0;
				IS_TICO_DEAD[ENTRY_ID] = false;
			};
			//Teleport!
			if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_LW && IS_TICO_DEAD[ENTRY_ID] == false && COOLDOWN[ENTRY_ID] == 0{
				if frame > 11.0 && frame < 13.0 {
					macros::EFFECT(fighter, Hash40::new("rosetta_escape"), Hash40::new("top"), 0, 0, -3, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
					IS_TELEPORT[ENTRY_ID] = true;
					INVIS_FRAMES[ENTRY_ID] = 1;
				};
				if frame > 16.0 && frame < 19.0 {
					HitModule::set_whole(boma, smash::app::HitStatus(*HIT_STATUS_XLU), 0);
					VisibilityModule::set_whole(boma, false);
					JostleModule::set_status(boma, false);	
					let pos = smash::phx::Vector3f { x: TICO_X[ENTRY_ID], y: TICO_Y[ENTRY_ID], z: 0.0 };
					PostureModule::set_pos(boma, &pos);
					PostureModule::init_pos(boma, &pos, true, true);
					INVIS_FRAMES[ENTRY_ID] = 2;
				};
				if frame > 24.0 && frame < 26.0 {
					macros::EFFECT(fighter, Hash40::new("rosetta_escape_end"), Hash40::new("top"), 0, 0, -1.5, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
					INVIS_FRAMES[ENTRY_ID] = 3;
				};
				if frame > 25.0{
					VisibilityModule::set_whole(boma, true);
					JostleModule::set_status(boma, true);	
					INVIS_FRAMES[ENTRY_ID] = 4;
					HitModule::set_whole(boma, smash::app::HitStatus(*HIT_STATUS_NORMAL), 0);
					CancelModule::enable_cancel(boma);
				};
			} else {
				ROSA_Y[ENTRY_ID] = PostureModule::pos_y(boma);
				ROSA_X[ENTRY_ID] = PostureModule::pos_x(boma);
				INVIS_FRAMES[ENTRY_ID] = 0;
			};
			if COOLDOWN[ENTRY_ID] > 0 {
				COOLDOWN[ENTRY_ID] -= 1;
			};
			if COOLDOWN[ENTRY_ID] == 1 {
				smash::app::FighterUtil::flash_eye_info(boma);
				EffectModule::req_follow(boma, smash::phx::Hash40::new("sys_smash_flash"), smash::phx::Hash40::new("haver"), &HANDS, &HANDS, 0.325, true, 0, 0, 0, 0, 0, true, true) as u32;
				EffectModule::req_follow(boma, smash::phx::Hash40::new("sys_smash_flash"), smash::phx::Hash40::new("havel"), &HANDS, &HANDS, 0.325, true, 0, 0, 0, 0, 0, true, true) as u32;
			};
			if status_kind == *FIGHTER_STATUS_KIND_DEAD {
				IS_TICO_DEAD[ENTRY_ID] = false;
			};
	}
}
#[weapon_frame( agent = WEAPON_KIND_ROSETTA_TICO )]
fn tico_frame(weapon: &mut L2CFighterBase) {
    unsafe {
        let otarget_id = WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER) as u32;
		let status_kind = smash::app::lua_bind::StatusModule::status_kind(weapon.module_accessor);
        let boma = smash::app::sv_battle_object::module_accessor(otarget_id);
		let ENTRY_ID = WorkModule::get_int(&mut *boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
        if smash::app::utility::get_kind(&mut *boma) == *FIGHTER_KIND_ROSETTA {
			if IS_TELEPORT[ENTRY_ID] == false {
				TICO_Y[ENTRY_ID] = PostureModule::pos_y(weapon.module_accessor);
				TICO_X[ENTRY_ID] = PostureModule::pos_x(weapon.module_accessor);
			};
			if [*WEAPON_ROSETTA_TICO_STATUS_KIND_DEAD, *WEAPON_ROSETTA_TICO_STATUS_KIND_NONE].contains(&status_kind) {
				IS_TICO_DEAD[ENTRY_ID] = true;
			};
			if [*WEAPON_ROSETTA_TICO_STATUS_KIND_REBIRTH].contains(&status_kind) {
				IS_TICO_DEAD[ENTRY_ID] = false;
			};
			println!("TICO_X {}, TICO_Y {}",TICO_X[ENTRY_ID], TICO_Y[ENTRY_ID] );
			if IS_TELEPORT[ENTRY_ID] == true {
				if INVIS_FRAMES[ENTRY_ID] == 1 {
					macros::EFFECT(weapon, Hash40::new("rosetta_escape"), Hash40::new("rot"), 0, 0, 0, 0, 0, 0, 0.5, 0, 0, 0, 0, 0, 0, true);
				};
				if INVIS_FRAMES[ENTRY_ID] == 2 {
					HitModule::set_whole(boma, smash::app::HitStatus(*HIT_STATUS_XLU), 0);
					VisibilityModule::set_whole(weapon.module_accessor, false);
					JostleModule::set_status(weapon.module_accessor, false);	
					let pos = smash::phx::Vector3f { x: ROSA_X[ENTRY_ID], y: ROSA_Y[ENTRY_ID], z: 0.0 };
					PostureModule::set_pos(weapon.module_accessor, &pos);
					PostureModule::init_pos(weapon.module_accessor, &pos, true, true);
				};
				if INVIS_FRAMES[ENTRY_ID] == 3 {
					macros::EFFECT(weapon, Hash40::new("rosetta_escape_end"), Hash40::new("rot"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
				};
				if INVIS_FRAMES[ENTRY_ID] == 4 {
					JostleModule::set_status(weapon.module_accessor, true);	
					VisibilityModule::set_whole(weapon.module_accessor, true);
					IS_TELEPORT[ENTRY_ID] = false;
					COOLDOWN[ENTRY_ID] = TELEPORT_COOLDOWN;
					INVIS_FRAMES[ENTRY_ID] = 0;
					HitModule::set_whole(boma, smash::app::HitStatus(*HIT_STATUS_NORMAL), 0);
				};
			};
		};
    }
}
pub fn install() {
    smashline::install_acmd_scripts!(
		rosa_jab,
		rosa_utilt,
		rosa_upb,
		rosa_upb_start,
		luma_boom, luma_boom_eff, luma_boom_snd
    );
    smashline::install_agent_frames!(
        tico_frame,
		rosa_frame
    );
}
