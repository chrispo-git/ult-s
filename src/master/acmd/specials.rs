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
use crate::master::*;
use super::*;

pub fn install() {
    Agent::new("master")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .acmd("game_specialn", byleth_neutralb, Priority::Low)    
    .acmd("game_specialairn", byleth_neutralb, Priority::Low)    
    .acmd("effect_specialn", byleth_neutralb_eff, Priority::Low)    
    .acmd("effect_specialairn", byleth_neutralb_eff, Priority::Low)    
    .acmd("sound_specialn", byleth_neutralb_snd, Priority::Low)    
    .acmd("sound_specialairn", byleth_neutralb_snd, Priority::Low)    
    .acmd("expression_specialairn", byleth_air_neutralb_exp, Priority::Low)    
    .acmd("game_specialnstart", byleth_neutralb_start, Priority::Low)    
    .acmd("game_specialairnstart", byleth_neutralb_start, Priority::Low)    
    .acmd("game_specialnmax", byleth_neutralbmax, Priority::Low)    
    .acmd("effect_specialnmax", byleth_neutralbmaxe, Priority::Low)    
    .acmd("sound_specialnmax", byleth_neutralbmaxs, Priority::Low)    
    .acmd("game_specialairnmax", byleth_neutralbmaxair, Priority::Low)    
    .acmd("effect_specialairnmax", byleth_neutralbmaxaire, Priority::Low)    
    .acmd("sound_specialairnmax", byleth_neutralbmaxairs, Priority::Low)    
    .acmd("game_speciallw", byleth_downb, Priority::Low)    
    .acmd("game_specialairlw", byleth_downb, Priority::Low)    
    .install();

	Agent::new("master_arrow1")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .acmd("effect_fly", byleth_arrow_effect, Priority::Low)    
    .acmd("game_fly", byleth_arrow, Priority::Low)    
    .install();

	Agent::new("master_axe")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .acmd("game_speciallw", axe_downb, Priority::Low)    
    .acmd("game_specialairlw", axe_downb, Priority::Low)    
    .install();

	Agent::new("kirby")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .acmd("effect_masterspecialn", kirby_byleth_neutralb_eff, Priority::Low)    
    .acmd("effect_masterspecialairn", kirby_byleth_neutralb_eff, Priority::Low)    
    .install();
}
		
unsafe extern "C" fn byleth_arrow_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
		for _ in 0..10 {
			if macros::is_excute(fighter) {
				let scale = smash::phx::Vector3f { x: 0.75, y: 0.75, z: 0.75};
				EffectModule::set_scale_last(fighter.module_accessor, &scale);
				macros::EFFECT(fighter, Hash40::new("sys_fireflower_shot"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.25, 0, 0, 0, 0, 0, 0, true);
				EffectModule::set_rate_last(fighter.module_accessor, 0.75);
			}
			wait(fighter.lua_state_agent, 1.0);
		}
		wait(fighter.lua_state_agent, 1.0);
		if macros::is_excute(fighter) {
			let scale = smash::phx::Vector3f { x: 0.75, y: 0.75, z: 0.75};
			EffectModule::set_scale_last(fighter.module_accessor, &scale);
		}
	}
unsafe extern "C" fn byleth_arrow(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
		if macros::is_excute(fighter) {
			macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 7.0, /*Angle*/ 361, /*KBG*/ 70, /*FKB*/ 0, /*BKB*/ 20, /*Size*/ 4.0, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ -1.5, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.8, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 5, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ true, /*Absorbable*/ true, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_fire"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L,  /*SFXType*/ *COLLISION_SOUND_ATTR_FIRE, /*Type*/ *ATTACK_REGION_OBJECT);
			AttackModule::enable_safe_pos(fighter.module_accessor);
		}
	}							
unsafe extern "C" fn byleth_neutralb(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
		macros::FT_MOTION_RATE(fighter, /*FSM*/ 1.7);
		if macros::is_excute(fighter) {
			WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_MASTER_STATUS_SPECIAL_N_FLAG_ENABLE_CONTROL_ENERGY);
		}
		frame(fighter.lua_state_agent, 9.0);
		if macros::is_excute(fighter) {
				let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);   
				let ENTRY_ID = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
				if ArticleModule::is_exist(boma, *FIGHTER_MASTER_GENERATE_ARTICLE_ARROW1) == false && IS_THUNDER[ENTRY_ID] == false {
					ArticleModule::generate_article(boma, *FIGHTER_MASTER_GENERATE_ARTICLE_ARROW1,false,0);
				};
		}
		frame(fighter.lua_state_agent, 10.0);
		if macros::is_excute(fighter) {
				let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);   
				let ENTRY_ID = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
				if IS_THUNDER[ENTRY_ID] == false {
					ArticleModule::shoot_exist(boma, *FIGHTER_MASTER_GENERATE_ARTICLE_ARROW1, smash::app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL), false);
				} else {
					macros::ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 6.0, /*Angle*/ 270, /*KBG*/ 95, /*FKB*/ 0, /*BKB*/ 40, /*Size*/ 3.6, /*X*/ 0.0, /*Y*/ 55.0, /*Z*/ 28.0, /*X2*/ Some(0.0), /*Y2*/ Some(40.0), /*Z2*/ Some(28.0), /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 3, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ true, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_elec"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_ELEC, /*Type*/ *ATTACK_REGION_ENERGY);
					AttackModule::enable_safe_pos(boma);
					macros::ATTACK(fighter, /*ID*/ 2, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 6.0, /*Angle*/ 80, /*KBG*/ 98, /*FKB*/ 0, /*BKB*/ 40, /*Size*/ 3.6, /*X*/ 0.0, /*Y*/ 40.0, /*Z*/ 28.0, /*X2*/ Some(0.0), /*Y2*/ Some(4.0), /*Z2*/ Some(28.0), /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 3, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ true, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_elec"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_ELEC, /*Type*/ *ATTACK_REGION_ENERGY);
					AttackModule::enable_safe_pos(boma);
					macros::ATTACK(fighter, /*ID*/ 	0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 12.0, /*Angle*/ 361, /*KBG*/ 90, /*FKB*/ 0, /*BKB*/ 40, /*Size*/ 6.0, /*X*/ 0.0, /*Y*/ 5.0, /*Z*/ 28.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.5, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 3, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ true, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_fire"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_ELEC, /*Type*/ *ATTACK_REGION_ENERGY);
					AttackModule::enable_safe_pos(boma);
				};
		}
		frame(fighter.lua_state_agent, 14.0);
		if macros::is_excute(fighter) {
				AttackModule::clear_all(fighter.module_accessor);
				let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);   
				let ENTRY_ID = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
				if IS_THUNDER[ENTRY_ID] == false {
					IS_THUNDER[ENTRY_ID] = true;
				} else {
					IS_THUNDER[ENTRY_ID] = false;
				};
		}
		frame(fighter.lua_state_agent, 27.0);
        if macros::is_excute(fighter) {
            CancelModule::enable_cancel(fighter.module_accessor);
        }
	}		
unsafe extern "C" fn byleth_neutralb_eff(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
		frame(fighter.lua_state_agent, 9.0);
		if macros::is_excute(fighter) {
				let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);  
				let ENTRY_ID = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize; 
				if IS_THUNDER[ENTRY_ID] == true {
					if StatusModule::situation_kind(boma) == *SITUATION_KIND_AIR {
						macros::EFFECT_FOLLOW(fighter, Hash40::new("sys_thunder"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 0.75, true);
					} else {
						macros::EFFECT(fighter, Hash40::new("sys_thunder"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 0.75, 0, 0, 0, 0, 0, 0, true);
					};
					macros::EFFECT(fighter, Hash40::new("sys_thunder_flash"), Hash40::new("top"), 0, 0, 25, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
				}
		}
	}	
unsafe extern "C" fn kirby_byleth_neutralb_eff(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
		frame(fighter.lua_state_agent, 9.0);
		if macros::is_excute(fighter) {
				let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);  
				let ENTRY_ID = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize; 
				if IS_THUNDER[ENTRY_ID] == true {
					if StatusModule::situation_kind(boma) == *SITUATION_KIND_AIR {
						macros::EFFECT_FOLLOW(fighter, Hash40::new("sys_thunder"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 0.75, true);
					} else {
						macros::EFFECT(fighter, Hash40::new("sys_thunder"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 0.75, 0, 0, 0, 0, 0, 0, true);
					};
					macros::EFFECT(fighter, Hash40::new("sys_thunder_flash"), Hash40::new("top"), 0, 0, 25, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
				}
		}
	}	
unsafe extern "C" fn byleth_neutralb_snd(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
		frame(fighter.lua_state_agent, 10.0);
		if macros::is_excute(fighter) {
				let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);  
				let ENTRY_ID = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize; 
				if IS_THUNDER[ENTRY_ID] == true {
					macros::PLAY_SE(fighter, Hash40::new("se_common_smashswing_02"));
					macros::PLAY_SE(fighter, Hash40::new("se_common_down_m_02"));
					macros::PLAY_SE(fighter, Hash40::new("se_common_electric_hit_s"));
				};
				macros::PLAY_SEQUENCE(fighter, Hash40::new("seq_master_rnd_attack02"));
		}
	}				
unsafe extern "C" fn byleth_air_neutralb_exp(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
	}			
	
unsafe extern "C" fn byleth_neutralb_start(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
		if macros::is_excute(fighter) {
				let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent); 
				if smash::app::utility::get_kind(boma) == *FIGHTER_KIND_MASTER {
					StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_MASTER_STATUS_KIND_SPECIAL_N_SHOOT, true);
				} else {
					StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_KIRBY_STATUS_KIND_MASTER_SPECIAL_N_SHOOT, true);
				};
		}
	}									
unsafe extern "C" fn byleth_neutralbmax(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
		if macros::is_excute(fighter) {
			CancelModule::enable_cancel(fighter.module_accessor);
		}
	}										
unsafe extern "C" fn byleth_neutralbmaxe(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
	}											
unsafe extern "C" fn byleth_neutralbmaxs(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
	}										
unsafe extern "C" fn byleth_neutralbmaxair(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
		if macros::is_excute(fighter) {
			CancelModule::enable_cancel(fighter.module_accessor);
		}
	}											
unsafe extern "C" fn byleth_neutralbmaxaire(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
	}											
unsafe extern "C" fn byleth_neutralbmaxairs(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
	}			
unsafe extern "C" fn axe_downb(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
		macros::FT_MOTION_RATE(fighter, /*FSM*/ 0.04761904);
		if macros::is_excute(fighter) {
			WorkModule::set_int(fighter.module_accessor, 6, *WEAPON_MASTER_AXE_INSTANCE_WORK_ID_INT_CRITICAL_ATTACK_ID);
		}
		frame(fighter.lua_state_agent, 42.0);
		macros::FT_MOTION_RATE(fighter, /*FSM*/ 1);
		frame(fighter.lua_state_agent, 61.0);
		if macros::is_excute(fighter) {
			macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("haver"),  /*Damage*/ 16.8, /*Angle*/ 361, /*KBG*/ 80, /*FKB*/ 0, /*BKB*/ 50, /*Size*/ 6.7, /*X*/ 0.0, /*Y*/ 14.0, /*Z*/ 1.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.45, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_THRU, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_MASTER_AXE, /*Type*/ *ATTACK_REGION_OBJECT);
		}
		frame(fighter.lua_state_agent, 67.0);
		if macros::is_excute(fighter) {
			AttackModule::clear_all(fighter.module_accessor);
		}
	}			
unsafe extern "C" fn byleth_downb(fighter: &mut L2CAgentBase) {
        let lua_state = fighter.lua_state_agent;
            macros::FT_MOTION_RATE(fighter, /*FSM*/ 0.04761904);
            if macros::is_excute(fighter) {
                WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_MASTER_STATUS_SPECIAL_LW_FLAG_INHERIT_LANDING_1);
            }
            frame(fighter.lua_state_agent, 42.0);
            macros::FT_MOTION_RATE(fighter, /*FSM*/ 1);
            frame(fighter.lua_state_agent, 51.0);
            if macros::is_excute(fighter) {
                WorkModule::off_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_MASTER_STATUS_SPECIAL_LW_FLAG_INHERIT_LANDING_1);
            }
            frame(fighter.lua_state_agent, 60.0);
            if macros::is_excute(fighter) {
                WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_MASTER_STATUS_SPECIAL_LW_FLAG_FORBID_LANDING);
            }
            frame(fighter.lua_state_agent, 64.0);
            if macros::is_excute(fighter) {
                ArticleModule::set_flag(fighter.module_accessor, *FIGHTER_MASTER_GENERATE_ARTICLE_AXE, true, *WEAPON_PIERCE_INSTANCE_WORK_ID_FLAG_PIERCE_GROUND);
            }
            frame(fighter.lua_state_agent, 65.0);
            if macros::is_excute(fighter) {
                ArticleModule::set_flag(fighter.module_accessor, *FIGHTER_MASTER_GENERATE_ARTICLE_AXE, false, *WEAPON_PIERCE_INSTANCE_WORK_ID_FLAG_PIERCE_GROUND);
                WorkModule::off_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_MASTER_STATUS_SPECIAL_LW_FLAG_FORBID_LANDING);
                WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_MASTER_STATUS_SPECIAL_LW_FLAG_INHERIT_LANDING_2);
            }
            frame(fighter.lua_state_agent, 96.0);
            if macros::is_excute(fighter) {
                WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_MASTER_STATUS_SPECIAL_LW_FLAG_CONTROL_ENERGY);
                WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_MASTER_STATUS_SPECIAL_LW_FLAG_REVERT_FALL_SPEED);
                CancelModule::enable_cancel(fighter.module_accessor);
            }
            frame(fighter.lua_state_agent, 117.0);
            if macros::is_excute(fighter) {
                WorkModule::off_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_MASTER_STATUS_SPECIAL_LW_FLAG_INHERIT_LANDING_2);
            }
            frame(fighter.lua_state_agent, 118.0);
            if macros::is_excute(fighter) {
                ArticleModule::remove_exist(fighter.module_accessor, *FIGHTER_MASTER_GENERATE_ARTICLE_AXE,smash::app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
            }
        }