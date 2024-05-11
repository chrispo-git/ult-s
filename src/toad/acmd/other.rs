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
use super::super::*;

unsafe extern "C" fn toad_catch(fighter: &mut L2CAgentBase) {
    	let lua_state = fighter.lua_state_agent;
		ArticleModule::remove_exist(fighter.module_accessor, *FIGHTER_MURABITO_GENERATE_ARTICLE_BUTTERFLYNET,smash::app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
		frame(fighter.lua_state_agent, 5.0);
		if macros::is_excute(fighter) {
			GrabModule::set_rebound(fighter.module_accessor, /*CanCatchRebound*/ true);
		}
		frame(fighter.lua_state_agent, 6.0);
		if macros::is_excute(fighter) {
			macros::CATCH(fighter, /*ID*/ 0, /*Bone*/ Hash40::new("top"), /*Size*/ 3.3, /*X*/ 0.0, /*Y*/ 6.0, /*Z*/ 4.0, /*X2*/ Some(0.0), /*Y2*/ Some(6.0), /*Z2*/ Some(9.7), /*Status*/ *FIGHTER_STATUS_KIND_CAPTURE_PULLED, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_G);
			macros::CATCH(fighter, /*ID*/ 1, /*Bone*/ Hash40::new("top"), /*Size*/ 1.65, /*X*/ 0.0, /*Y*/ 6.0, /*Z*/ 2.35, /*X2*/ Some(0.0), /*Y2*/ Some(6.0), /*Z2*/ Some(11.35), /*Status*/ *FIGHTER_STATUS_KIND_CAPTURE_PULLED, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_A);
		}
		if true{
			macros::game_CaptureCutCommon(fighter);
		}
		wait(fighter.lua_state_agent, 2.0);
		if macros::is_excute(fighter) {
			grab!(fighter, *MA_MSC_CMD_GRAB_CLEAR_ALL);
			WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT);
			GrabModule::set_rebound(fighter.module_accessor, /*CanCatchRebound*/ false);
		}
		frame(fighter.lua_state_agent, 40.0);
		if macros::is_excute(fighter) {
			ArticleModule::remove_exist(fighter.module_accessor, *FIGHTER_MURABITO_GENERATE_ARTICLE_BUTTERFLYNET,smash::app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
		}
}	
unsafe extern "C" fn toad_catch_snd(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
	frame(fighter.lua_state_agent, 5.0);
	if macros::is_excute(fighter) {
		attack_vc(fighter);
		macros::PLAY_SE(fighter, Hash40::new("se_murabito_swing_s"));
	};
}	
unsafe extern "C" fn toad_catchdash(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
	ArticleModule::remove_exist(fighter.module_accessor, *FIGHTER_MURABITO_GENERATE_ARTICLE_BUTTERFLYNET,smash::app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
		frame(fighter.lua_state_agent, 8.0);
		if macros::is_excute(fighter) {
			GrabModule::set_rebound(fighter.module_accessor, /*CanCatchRebound*/ true);
		}
		frame(fighter.lua_state_agent, 9.0);
		if macros::is_excute(fighter) {
			macros::CATCH(fighter, /*ID*/ 0, /*Bone*/ Hash40::new("top"), /*Size*/ 3.3, /*X*/ 0.0, /*Y*/ 6.0, /*Z*/ 4.0, /*X2*/ Some(0.0), /*Y2*/ Some(6.0), /*Z2*/ Some(10.7), /*Status*/ *FIGHTER_STATUS_KIND_CAPTURE_PULLED, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_G);
			macros::CATCH(fighter, /*ID*/ 1, /*Bone*/ Hash40::new("top"), /*Size*/ 1.65, /*X*/ 0.0, /*Y*/ 6.0, /*Z*/ 2.35, /*X2*/ Some(0.0), /*Y2*/ Some(6.0), /*Z2*/ Some(12.35), /*Status*/ *FIGHTER_STATUS_KIND_CAPTURE_PULLED, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_A);
		}
		if true{
			macros::game_CaptureCutCommon(fighter);
		}
		wait(fighter.lua_state_agent, 2.0);
		if macros::is_excute(fighter) {
			grab!(fighter, *MA_MSC_CMD_GRAB_CLEAR_ALL);
			WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT);
			GrabModule::set_rebound(fighter.module_accessor, /*CanCatchRebound*/ false);
		}
		frame(fighter.lua_state_agent, 40.0);
		if macros::is_excute(fighter) {
			ArticleModule::remove_exist(fighter.module_accessor, *FIGHTER_MURABITO_GENERATE_ARTICLE_BUTTERFLYNET,smash::app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
		}
}	
unsafe extern "C" fn toad_catchdash_snd(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
	frame(fighter.lua_state_agent, 8.0);
	if macros::is_excute(fighter) {
		attack_vc(fighter);
		macros::PLAY_SE(fighter, Hash40::new("se_murabito_swing_s"));
	};
}	
unsafe extern "C" fn toad_catchturn(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
	ArticleModule::remove_exist(fighter.module_accessor, *FIGHTER_MURABITO_GENERATE_ARTICLE_BUTTERFLYNET,smash::app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
		frame(fighter.lua_state_agent, 9.0);
		if macros::is_excute(fighter) {
			GrabModule::set_rebound(fighter.module_accessor, /*CanCatchRebound*/ true);
		}
		frame(fighter.lua_state_agent, 10.0);
		if macros::is_excute(fighter) {
			macros::CATCH(fighter, /*ID*/ 0, /*Bone*/ Hash40::new("top"), /*Size*/ 3.3, /*X*/ 0.0, /*Y*/ 6.0, /*Z*/ -4.0, /*X2*/ Some(0.0), /*Y2*/ Some(6.0), /*Z2*/ Some(-15.7), /*Status*/ *FIGHTER_STATUS_KIND_CAPTURE_PULLED, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_G);
			macros::CATCH(fighter, /*ID*/ 1, /*Bone*/ Hash40::new("top"), /*Size*/ 1.65, /*X*/ 0.0, /*Y*/ 6.0, /*Z*/ -2.35, /*X2*/ Some(0.0), /*Y2*/ Some(6.0), /*Z2*/ Some(-17.35), /*Status*/ *FIGHTER_STATUS_KIND_CAPTURE_PULLED, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_A);
		}
		if true{
			macros::game_CaptureCutCommon(fighter);
		}
		wait(fighter.lua_state_agent, 2.0);
		if macros::is_excute(fighter) {
			grab!(fighter, *MA_MSC_CMD_GRAB_CLEAR_ALL);
			WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT);
			GrabModule::set_rebound(fighter.module_accessor, /*CanCatchRebound*/ false);
		}
		frame(fighter.lua_state_agent, 40.0);
		if macros::is_excute(fighter) {
			ArticleModule::remove_exist(fighter.module_accessor, *FIGHTER_MURABITO_GENERATE_ARTICLE_BUTTERFLYNET,smash::app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
		}
}	
unsafe extern "C" fn toad_catchturn_snd(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
	frame(fighter.lua_state_agent, 9.0);
	if macros::is_excute(fighter) {
		attack_vc(fighter);
		macros::PLAY_SE(fighter, Hash40::new("se_murabito_swing_s"));
	};
}	
unsafe extern "C" fn toad_catchattack(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
		frame(fighter.lua_state_agent, 1.0);
		if macros::is_excute(fighter) {
			macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 1.5, /*Angle*/ 361, /*KBG*/ 100, /*FKB*/ 30, /*BKB*/ 0, /*Size*/ 5.0, /*X*/ 0.0, /*Y*/ 6.0, /*Z*/ 10.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 2.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_KICK, /*Type*/ *ATTACK_REGION_HEAD);
			AttackModule::set_catch_only_all(fighter.module_accessor, true, false);
		}
		wait(fighter.lua_state_agent, 1.0);
		if macros::is_excute(fighter) {
			AttackModule::clear_all(fighter.module_accessor);
		}
}

unsafe extern "C" fn toad_dmg_snd(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	if macros::is_excute(fighter) {
		dmg_vc(fighter);
	};
}	
unsafe extern "C" fn toad_dmg_fly_snd(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	if macros::is_excute(fighter) {
		dmg_fly_vc(fighter);
	};
}	
unsafe extern "C" fn toad_star_ko_snd(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	if macros::is_excute(fighter) {
		if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_DEAD_UP_FALL) {
			macros::PLAY_SE(fighter, Hash40::new("se_murabito_catch_net"));
		};
	};
}

unsafe extern "C" fn toad_final(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
		if macros::is_excute(fighter) {
			macros::CHECK_VALID_FINAL_START_CAMERA(fighter, 0, 7, 20, 0, 0, 0);
			macros::SLOW_OPPONENT(fighter, 5.0, 20.0);
			macros::FT_SET_FINAL_FEAR_FACE(fighter, 40);
			macros::FT_START_CUTIN(fighter, );
		}
		macros::FT_MOTION_RATE(fighter, /*FSM*/ 2);
		frame(fighter.lua_state_agent, 5.0);
		if macros::is_excute(fighter) {
			macros::EFFECT(fighter, Hash40::new("sys_erace_smoke"), Hash40::new("top"), 10, 6, 8, 0, 0, 0, 1.5, 0, 0, 0, 0, 0, 0, false);
			macros::EFFECT(fighter, Hash40::new("sys_erace_smoke"), Hash40::new("top"), -10, 6, 8, 0, 0, 0, 1.5, 0, 0, 0, 0, 0, 0, false);
			macros::EFFECT(fighter, Hash40::new("sys_erace_smoke"), Hash40::new("top"), 0, 6, 8, 0, 0, 0, 1.5, 0, 0, 0, 0, 0, 0, false);
		}
		frame(fighter.lua_state_agent, 6.0);
		macros::FT_MOTION_RATE(fighter, /*FSM*/ 1);
		if macros::is_excute(fighter) {
			ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_MURABITO_GENERATE_ARTICLE_CLAYROCKET, false, 0);
			WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_MURABITO_STATUS_SPECIAL_S_FLAG_CLAYROCKET);
			ArticleModule::shoot(fighter.module_accessor, *FIGHTER_MURABITO_GENERATE_ARTICLE_CLAYROCKET, smash::app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL), false);
		}
		frame(fighter.lua_state_agent, 12.0);
		macros::FT_MOTION_RATE(fighter, /*FSM*/ 10);
		frame(fighter.lua_state_agent, 15.0);
		if macros::is_excute(fighter) {
			StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_MURABITO_STATUS_KIND_FINAL_END, false);
		}
	}
	unsafe extern "C" fn toad_final_toad_army(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
		if macros::is_excute(fighter) {
			macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 15.0, /*Angle*/ 361, /*KBG*/ 100, /*FKB*/ 0, /*BKB*/ 30, /*Size*/ 10.3, /*X*/ 0.0, /*Y*/ 5.0, /*Z*/ -10.0, /*X2*/ Some(0.0), /*Y2*/ Some(5.0), /*Z2*/ Some(12.0), /*Hitlag*/ 0.4, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_B, /*SetWeight*/ false, /*ShieldDamage*/ f32::NAN, /*Trip*/ -1.0, /*Rehit*/ 5, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_PUNCH, /*Type*/ *ATTACK_REGION_OBJECT);
		}
	}
	unsafe extern "C" fn toad_final_toad_army_eff(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
	let otarget_id = WorkModule::get_int(fighter.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER) as u32;
	let boma = smash::app::sv_battle_object::module_accessor(otarget_id);
    for _ in 0..50 {
		if macros::is_excute(fighter) {
			macros::EFFECT(fighter, Hash40::new("sys_erace_smoke"), Hash40::new("top"), 12, 0, 0, 0, 0, 0, 1.0, 0, 0, 0, 0, 0, 0, false);
			macros::EFFECT(fighter, Hash40::new("sys_erace_smoke"), Hash40::new("top"), 2, 0, 0, 0, 0, 0, 1.0, 0, 0, 0, 0, 0, 0, false);
			macros::EFFECT(fighter, Hash40::new("sys_erace_smoke"), Hash40::new("top"), -0.5, 0, 5, 0, 0, 0, 1.0, 0, 0, 0, 0, 0, 0, false);
			macros::EFFECT(fighter, Hash40::new("sys_erace_smoke"), Hash40::new("top"), 6, 0, 0, 5, 0, 0, 1.0, 0, 0, 0, 0, 0, 0, false);
		}
		wait(fighter.lua_state_agent, 1.0);
		if macros::is_excute(fighter) {
			macros::EFFECT(fighter, Hash40::new("sys_erace_smoke"), Hash40::new("top"), 20, 0, 5, 0, 0, 0, 1.0, 0, 0, 0, 0, 0, 0, false);
			macros::EFFECT(fighter, Hash40::new("sys_erace_smoke"), Hash40::new("top"), 20, 0, -5, 0, 0, 0, 1.0, 0, 0, 0, 0, 0, 0, false);
			macros::EFFECT(fighter, Hash40::new("sys_erace_smoke"), Hash40::new("top"), 9.5, 0, -5, 0, 0, 0, 1.0, 0, 0, 0, 0, 0, 0, false);
			macros::EFFECT(fighter, Hash40::new("sys_erace_smoke"), Hash40::new("top"), -3, 0, -5, 0, 0, 0, 1.0, 0, 0, 0, 0, 0, 0, false);
		}
		wait(fighter.lua_state_agent, 1.0);
		if macros::is_excute(fighter) {
			macros::EFFECT(fighter, Hash40::new("sys_erace_smoke"), Hash40::new("top"), -8, 0, 0, 0, 0, 0, 1.0, 0, 0, 0, 0, 0, 0, false);
			macros::EFFECT(fighter, Hash40::new("sys_erace_smoke"), Hash40::new("top"), 4.5, 0, 5, 0, 0, 0, 1.0, 0, 0, 0, 0, 0, 0, false);
			macros::EFFECT(fighter, Hash40::new("sys_erace_smoke"), Hash40::new("top"), -5.5, 0, 5, 0, 0, 0, 1.0, 0, 0, 0, 0, 0, 0, false);
		}
		wait(fighter.lua_state_agent, 1.0);
		if macros::is_excute(fighter) {
			macros::EFFECT(fighter, Hash40::new("sys_erace_smoke"), Hash40::new("top"), 15, 0, 0, 0, 0, 0, 1.0, 0, 0, 0, 0, 0, 0, false);
			macros::EFFECT(fighter, Hash40::new("sys_erace_smoke"), Hash40::new("top"), 9.5, 0, -5, 0, 0, 0, 1.0, 0, 0, 0, 0, 0, 0, false);
			macros::EFFECT(fighter, Hash40::new("sys_erace_smoke"), Hash40::new("top"), -3, 0, -5, 0, 0, 0, 1.0, 0, 0, 0, 0, 0, 0, false);
		}
		wait(fighter.lua_state_agent, 1.0);
	}
	wait(fighter.lua_state_agent, 1.0);
	if macros::is_excute(fighter) {
		ArticleModule::remove_exist(boma, *FIGHTER_MURABITO_GENERATE_ARTICLE_CLAYROCKET,smash::app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
	}
}
unsafe extern "C" fn toad_down_attack_d(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
		frame(fighter.lua_state_agent, 19.0);
		if macros::is_excute(fighter) {
			macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 5.0, /*Angle*/ 361, /*KBG*/ 50, /*FKB*/ 0, /*BKB*/ 60, /*Size*/ 4.5, /*X*/ 0.0, /*Y*/ 4.5, /*Z*/ -7.0, /*X2*/ Some(0.0), /*Y2*/ Some(4.5), /*Z2*/ Some(-3.0), /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 8, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_KICK, /*Type*/ *ATTACK_REGION_KICK);
		}
		wait(fighter.lua_state_agent, 2.0);
		if macros::is_excute(fighter) {
			AttackModule::clear_all(fighter.module_accessor);
		}
		frame(fighter.lua_state_agent, 28.0);
		if macros::is_excute(fighter) {
			macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 5.0, /*Angle*/ 361, /*KBG*/ 50, /*FKB*/ 0, /*BKB*/ 60, /*Size*/ 4.5, /*X*/ 0.0, /*Y*/ 4.5, /*Z*/ 7.5, /*X2*/ Some(0.0), /*Y2*/ Some(4.5), /*Z2*/ Some(4.0), /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 8, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_KICK, /*Type*/ *ATTACK_REGION_KICK);
		}
		wait(fighter.lua_state_agent, 2.0);
		if macros::is_excute(fighter) {
			AttackModule::clear_all(fighter.module_accessor);
		}
}	
unsafe extern "C" fn toad_down_attack_d_eff(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
		frame(fighter.lua_state_agent, 16.0);
		if macros::is_excute(fighter) {
			macros::FOOT_EFFECT(fighter, Hash40::new("sys_turn_smoke"), Hash40::new("top"), 2, 0, 0, 0, 180, 0, 1, 0, 0, 0, 0, 0, 0, false);
		}
		frame(fighter.lua_state_agent, 18.0);
		if macros::is_excute(fighter) {
			macros::EFFECT(fighter, Hash40::new("sys_attack_speedline"), Hash40::new("top"), 0, 3.5, 5, 0, 180, 0, 0.8, 0, 0, 0, 0, 0, 360, true);
			macros::LAST_EFFECT_SET_COLOR(fighter, 0.7, 1, 0.9);
			macros::LAST_EFFECT_SET_ALPHA(fighter, 0.7);
		}
		frame(fighter.lua_state_agent, 19.0);
		if macros::is_excute(fighter) {
			macros::EFFECT(fighter, Hash40::new("sys_attack_impact"), Hash40::new("top"), 0, 4, -7, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
			macros::LAST_EFFECT_SET_ALPHA(fighter, 0.7);
		}
		frame(fighter.lua_state_agent, 26.0);
		if macros::is_excute(fighter) {
			macros::FOOT_EFFECT(fighter, Hash40::new("sys_turn_smoke"), Hash40::new("top"), -3, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
		}
		frame(fighter.lua_state_agent, 27.0);
		if macros::is_excute(fighter) {
			macros::EFFECT(fighter, Hash40::new("sys_attack_speedline"), Hash40::new("top"), 0, 3.5, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 360, true);
			macros::LAST_EFFECT_SET_COLOR(fighter, 0.7, 1, 0.9);
			macros::LAST_EFFECT_SET_ALPHA(fighter, 0.7);
		}
		frame(fighter.lua_state_agent, 29.0);
		if macros::is_excute(fighter) {
			macros::EFFECT(fighter, Hash40::new("sys_attack_impact"), Hash40::new("top"), 0, 4, 8, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
			macros::LAST_EFFECT_SET_ALPHA(fighter, 0.7);
		}
}	
unsafe extern "C" fn toad_down_attack_u(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
		frame(fighter.lua_state_agent, 19.0);
		if macros::is_excute(fighter) {
			macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 5.0, /*Angle*/ 361, /*KBG*/ 50, /*FKB*/ 0, /*BKB*/ 60, /*Size*/ 4.5, /*X*/ 0.0, /*Y*/ 4.5, /*Z*/ 7.5, /*X2*/ Some(0.0), /*Y2*/ Some(4.5), /*Z2*/ Some(4.0), /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 8, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_KICK, /*Type*/ *ATTACK_REGION_KICK);
		}
		wait(fighter.lua_state_agent, 2.0);
		if macros::is_excute(fighter) {
			AttackModule::clear_all(fighter.module_accessor);
		}
		frame(fighter.lua_state_agent, 28.0);
		if macros::is_excute(fighter) {
			macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 5.0, /*Angle*/ 361, /*KBG*/ 50, /*FKB*/ 0, /*BKB*/ 60, /*Size*/ 4.5, /*X*/ 0.0, /*Y*/ 4.5, /*Z*/ -7.0, /*X2*/ Some(0.0), /*Y2*/ Some(4.5), /*Z2*/ Some(-3.0), /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 8, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_KICK, /*Type*/ *ATTACK_REGION_KICK);
		}
		wait(fighter.lua_state_agent, 2.0);
		if macros::is_excute(fighter) {
			AttackModule::clear_all(fighter.module_accessor);
		}
}	
unsafe extern "C" fn toad_down_attack_u_eff(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
		frame(fighter.lua_state_agent, 16.0);
		if macros::is_excute(fighter) {
			macros::FOOT_EFFECT(fighter, Hash40::new("sys_turn_smoke"), Hash40::new("top"), -3, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
		}
		frame(fighter.lua_state_agent, 17.0);
		if macros::is_excute(fighter) {
			macros::EFFECT(fighter, Hash40::new("sys_attack_speedline"), Hash40::new("top"), 0, 3.5, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 360, true);
			macros::LAST_EFFECT_SET_COLOR(fighter, 0.7, 1, 0.9);
			macros::LAST_EFFECT_SET_ALPHA(fighter, 0.7);
		}
		frame(fighter.lua_state_agent, 19.0);
		if macros::is_excute(fighter) {
			macros::EFFECT(fighter, Hash40::new("sys_attack_impact"), Hash40::new("top"), 0, 4, 8, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
			macros::LAST_EFFECT_SET_ALPHA(fighter, 0.7);
		}
		frame(fighter.lua_state_agent, 26.0);
		if macros::is_excute(fighter) {
			macros::FOOT_EFFECT(fighter, Hash40::new("sys_turn_smoke"), Hash40::new("top"), 2, 0, 0, 0, 180, 0, 1, 0, 0, 0, 0, 0, 0, false);
		}
		frame(fighter.lua_state_agent, 28.0);
		if macros::is_excute(fighter) {
			macros::EFFECT(fighter, Hash40::new("sys_attack_speedline"), Hash40::new("top"), 0, 3.5, 5, 0, 180, 0, 0.8, 0, 0, 0, 0, 0, 360, true);
			macros::LAST_EFFECT_SET_COLOR(fighter, 0.7, 1, 0.9);
			macros::LAST_EFFECT_SET_ALPHA(fighter, 0.7);
		}
		frame(fighter.lua_state_agent, 29.0);
		if macros::is_excute(fighter) {
			macros::EFFECT(fighter, Hash40::new("sys_attack_impact"), Hash40::new("top"), 0, 4, -7, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
			macros::LAST_EFFECT_SET_ALPHA(fighter, 0.7);
		}
}	
unsafe extern "C" fn toad_utaunt_eff(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
		frame(fighter.lua_state_agent, 50.0);
		if macros::is_excute(fighter) {
			macros::LANDING_EFFECT(fighter, Hash40::new("sys_down_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.6, 0, 0, 0, 0, 0, 0, false);
			macros::LANDING_EFFECT(fighter, Hash40::new("sys_down_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.7, 0, 0, 0, 0, 0, 0, false);
			macros::LANDING_EFFECT(fighter, Hash40::new("sys_landing_smoke_s"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.7, 0, 0, 0, 0, 0, 0, false);
		}
}	
unsafe extern "C" fn toad_utaunt_snd(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
		frame(fighter.lua_state_agent, 14.0);
		if macros::is_excute(fighter) {
			macros::PLAY_SE(fighter, Hash40::new("se_murabito_attackair_b02"));
		}
		frame(fighter.lua_state_agent, 50.0);
		if macros::is_excute(fighter) {
			macros::PLAY_SE(fighter, Hash40::new("se_common_landing_soil"));
		}
		frame(fighter.lua_state_agent, 53.0);
		if macros::is_excute(fighter) {
			macros::PLAY_SE(fighter, Hash40::new("se_murabito_attackair_h02"));
		}
}	
unsafe extern "C" fn toad_staunt_eff(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
		frame(fighter.lua_state_agent, 103.0);
		if macros::is_excute(fighter) {
			macros::LANDING_EFFECT(fighter, Hash40::new("sys_down_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.5, 0, 0, 0, 0, 0, 0, false);
		}
}	
unsafe extern "C" fn toad_staunt_snd(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
		frame(fighter.lua_state_agent, 48.0);
		if macros::is_excute(fighter) {
			macros::PLAY_SE(fighter, Hash40::new("se_murabito_attackair_h03"));
		}
		frame(fighter.lua_state_agent, 103.0);
		if macros::is_excute(fighter) {
			macros::PLAY_SE(fighter, Hash40::new("se_murabito_attackhard_l01"));
		}
		frame(fighter.lua_state_agent, 127.0);
		if macros::is_excute(fighter) {
			macros::PLAY_SE(fighter, Hash40::new("se_murabito_attackhard_s01"));
		}
}	
unsafe extern "C" fn toad_dtaunt(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
		frame(fighter.lua_state_agent, 16.0);
		if macros::is_excute(fighter) {
			macros::EFFECT(fighter, Hash40::new("sys_erace_smoke"), Hash40::new("haver"), 0, 1, 0, 0, 0, 0, 0.85, 0, 0, 0, 0, 0, 0, false);
		}
		frame(fighter.lua_state_agent, 17.0);
		if macros::is_excute(fighter) {
			ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_MURABITO_GENERATE_ARTICLE_BUTTERFLYNET, false, 0);
		}
		frame(fighter.lua_state_agent, 46.0);
		if macros::is_excute(fighter) {
			macros::EFFECT(fighter, Hash40::new("sys_erace_smoke"), Hash40::new("haver"), 0, 1, 0, 0, 0, 0, 0.85, 0, 0, 0, 0, 0, 0, false);
		}
		frame(fighter.lua_state_agent, 48.0);
		if macros::is_excute(fighter) {
			ArticleModule::remove_exist(fighter.module_accessor, *FIGHTER_MURABITO_GENERATE_ARTICLE_BUTTERFLYNET,smash::app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
		}
}	
unsafe extern "C" fn toad_dtaunt_snd(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
		frame(fighter.lua_state_agent, 16.0);
		if macros::is_excute(fighter) {
			macros::PLAY_SE(fighter, Hash40::new("se_murabito_special_h03"));
		}
		frame(fighter.lua_state_agent, 46.0);
		if macros::is_excute(fighter) {
			macros::PLAY_SE(fighter, Hash40::new("se_murabito_appear03"));
		}
}	
unsafe extern "C" fn toad_win_snd(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
}	
unsafe extern "C" fn toad_win1(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
		if macros::is_excute(fighter) {
			ArticleModule::remove_exist(fighter.module_accessor, *FIGHTER_MURABITO_GENERATE_ARTICLE_BEETLE,smash::app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
		}
}	
unsafe extern "C" fn toad_win3(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
		if macros::is_excute(fighter) {
			ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_MURABITO_GENERATE_ARTICLE_BEETLE, false, 0);
			ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_MURABITO_GENERATE_ARTICLE_BEETLE,smash::phx::Hash40::new("win_1"),false,0.0);
		}
}	
unsafe extern "C" fn toad_win2_eff(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
		for _ in 0..5 
 {
			if macros::is_excute(fighter) {
				macros::LANDING_EFFECT(fighter, Hash40::new("sys_landing_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.85, 0, 0, 0, 0, 0, 0, true);
			}
			wait(fighter.lua_state_agent, 16.0);
		}
}	
unsafe extern "C" fn toad_win1_eff(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
		frame(fighter.lua_state_agent, 116.0);
		if macros::is_excute(fighter) {
			macros::LANDING_EFFECT(fighter, Hash40::new("sys_landing_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.85, 0, 0, 0, 0, 0, 0, true);
		}
}	
unsafe extern "C" fn toad_win1wait_eff(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
}	
unsafe extern "C" fn toad_win3_eff(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
		for _ in 0..9999 
 {
			if macros::is_excute(fighter) {
				macros::EFFECT(fighter, Hash40::new("sys_erace_smoke"), Hash40::new("top"), 37, 7, 20, 0, 0, 0, 1.2, 0, 0, 0, 0, 0, 0, false);
				macros::EFFECT(fighter, Hash40::new("sys_erace_smoke"), Hash40::new("top"), 7, 7, 20, 0, 0, 0, 1.2, 0, 0, 0, 0, 0, 0, false);
				macros::EFFECT(fighter, Hash40::new("sys_erace_smoke"), Hash40::new("top"), 37, 7, -25, 0, 0, 0, 1.2, 0, 0, 0, 0, 0, 0, false);
				macros::EFFECT(fighter, Hash40::new("sys_erace_smoke"), Hash40::new("top"), 7, 7, -25, 0, 0, 0, 1.2, 0, 0, 0, 0, 0, 0, false);
			}
			wait(fighter.lua_state_agent, 10.0);
		}
}	
unsafe extern "C" fn toad_entry(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
		frame(fighter.lua_state_agent, 46.0);
		if macros::is_excute(fighter) {
			ArticleModule::remove_exist(fighter.module_accessor, *FIGHTER_MURABITO_GENERATE_ARTICLE_HOUSE,smash::app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
			macros::EFFECT(fighter, Hash40::new("sys_erace_smoke"), Hash40::new("top"), 0, 6, -8, 0, 0, 0, 1.0, 0, 0, 0, 0, 0, 0, false);
		}
}	

pub fn install() {
	Agent::new("murabito")
		.game_acmd("game_catchtoad", toad_catch, Priority::Low)
		.sound_acmd("sound_catchtoad", toad_catch_snd, Priority::Low)
		.game_acmd("game_catchdashtoad", toad_catchdash, Priority::Low)
		.sound_acmd("sound_catchdashtoad", toad_catchdash_snd, Priority::Low)
		.game_acmd("game_catchturntoad", toad_catchturn, Priority::Low)
		.sound_acmd("sound_catchturntoad", toad_catchturn_snd, Priority::Low)
		.game_acmd("game_catchattacktoad", toad_catchattack, Priority::Low)
		.sound_acmd("sound_damagehi1toad", toad_dmg_snd, Priority::Low)
		.sound_acmd("sound_damagehi2toad", toad_dmg_snd, Priority::Low)
		.sound_acmd("sound_damagehi3toad", toad_dmg_snd, Priority::Low)
		.sound_acmd("sound_damagen1toad", toad_dmg_snd, Priority::Low)
		.sound_acmd("sound_damagen2toad", toad_dmg_snd, Priority::Low)
		.sound_acmd("sound_damagen3toad", toad_dmg_snd, Priority::Low)
		.sound_acmd("sound_damagelw1toad", toad_dmg_snd, Priority::Low)
		.sound_acmd("sound_damagelw2toad", toad_dmg_snd, Priority::Low)
		.sound_acmd("sound_damagelw3toad", toad_dmg_snd, Priority::Low)
		.sound_acmd("sound_damageair1toad", toad_dmg_snd, Priority::Low)
		.sound_acmd("sound_damageair2toad", toad_dmg_snd, Priority::Low)
		.sound_acmd("sound_damageair3toad", toad_dmg_snd, Priority::Low)
		.sound_acmd("sound_damageelectoad", toad_dmg_snd, Priority::Low)
		.sound_acmd("sound_damageflyhi", toad_dmg_fly_snd, Priority::Low)
		.sound_acmd("sound_damageflyntoad", toad_dmg_fly_snd, Priority::Low)
		.sound_acmd("sound_damageflylwtoad", toad_dmg_fly_snd, Priority::Low)
		.sound_acmd("sound_damageflytoptoad", toad_dmg_fly_snd, Priority::Low)
		.sound_acmd("sound_damageflyrolltoad", toad_dmg_fly_snd, Priority::Low)
		.sound_acmd("sound_damageflymeteortoad", toad_dmg_fly_snd, Priority::Low)
		.sound_acmd("sound_damagefalltoad", toad_star_ko_snd, Priority::Low)
		.game_acmd("game_finalrtoad", toad_final, Priority::Low)
		.game_acmd("game_finalltoad", toad_final, Priority::Low)
		.game_acmd("game_finalairrtoad", toad_final, Priority::Low)
		.game_acmd("game_finalairltoad", toad_final, Priority::Low)
		.game_acmd("game_downattackdtoad", toad_down_attack_d, Priority::Low)
		.effect_acmd("effect_downattackdtoad", toad_down_attack_d_eff, Priority::Low)
		.game_acmd("game_downattackutoad", toad_down_attack_u, Priority::Low)
		.effect_acmd("effect_downattackutoad", toad_down_attack_u_eff, Priority::Low)
		.effect_acmd("effect_appealhirtoad", toad_utaunt_eff, Priority::Low)
		.effect_acmd("effect_appealhiltoad", toad_utaunt_eff, Priority::Low)
		.sound_acmd("sound_appealhirtoad", toad_utaunt_snd, Priority::Low)
		.sound_acmd("sound_appealhiltoad", toad_utaunt_snd, Priority::Low)
		.effect_acmd("effect_appealsrtoad", toad_staunt_eff, Priority::Low)
		.effect_acmd("effect_appealsltoad", toad_staunt_eff, Priority::Low)
		.sound_acmd("sound_appealsrtoad", toad_staunt_snd, Priority::Low)
		.sound_acmd("sound_appealsltoad", toad_staunt_snd, Priority::Low)
		.game_acmd("game_appeallwltoad", toad_dtaunt, Priority::Low)
		.game_acmd("game_appeallwrtoad", toad_dtaunt, Priority::Low)
		.sound_acmd("sound_appeallwrtoad", toad_dtaunt_snd, Priority::Low)
		.sound_acmd("sound_appeallwltoad", toad_dtaunt_snd, Priority::Low)
		.game_acmd("game_win1toad", toad_win1, Priority::Low)
		.game_acmd("game_win3toad", toad_win3, Priority::Low)
		.effect_acmd("effect_win2toad", toad_win2_eff, Priority::Low)
		.effect_acmd("effect_win1toad", toad_win1_eff, Priority::Low)
		.effect_acmd("effect_win1waittoad", toad_win1wait_eff, Priority::Low)
		.effect_acmd("effect_win3toad", toad_win3_eff, Priority::Low)
		.effect_acmd("effect_win3waittoad", toad_win3_eff, Priority::Low)
		.game_acmd("game_entryltoad", toad_entry, Priority::Low)
		.game_acmd("game_entryrtoad", toad_entry, Priority::Low)
		.install();

	Agent::new("murabito_clayrocket")
		.game_acmd("game_flytoad", toad_final_toad_army, Priority::Low)
		.game_acmd("game_falltoad", toad_final_toad_army, Priority::Low)
		.effect_acmd("effect_flytoad", toad_final_toad_army_eff, Priority::Low)
		.effect_acmd("effect_falltoad", toad_final_toad_army_eff, Priority::Low)
		.effect_acmd("effect_readytoad", toad_final_toad_army_eff, Priority::Low)
		.install();
}