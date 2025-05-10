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
use crate::jack::*;
use super::*;

pub fn install() {
    Agent::new("jack")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .acmd("game_specials2", joker_eiagon, Priority::Low)    
    .acmd("game_specialairs2", joker_eiagon_air, Priority::Low)    
    .acmd("game_specials1", joker_sideb, Priority::Low)    
    .acmd("game_specialairs1", joker_sideb, Priority::Low)    
    .acmd("effect_specials1", joker_sideb_eff, Priority::Low)    
    .acmd("effect_specialairs1", joker_sideb_eff, Priority::Low)    
    .acmd("sound_specials1", joker_sideb_snd, Priority::Low)    
    .acmd("sound_specialairs1", joker_sideb_snd, Priority::Low)    
    .acmd("sound_specials1", joker_sideb_expr, Priority::Low)    
    .acmd("sound_specialairs1", joker_sideb_expr, Priority::Low)    
    .install();
}
	
unsafe extern "C" fn joker_eiagon(fighter: &mut L2CAgentBase) {
    	let lua_state = fighter.lua_state_agent;
		frame(fighter.lua_state_agent, 1.0);
		macros::FT_MOTION_RATE(fighter, /*FSM*/ 1.4375);
		frame(fighter.lua_state_agent, 18.0);
		macros::FT_MOTION_RATE(fighter, /*FSM*/ 1);
}	
			
unsafe extern "C" fn joker_eiagon_air(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
		frame(fighter.lua_state_agent, 1.0);
		macros::FT_MOTION_RATE(fighter, /*FSM*/ 1.4375);
		frame(fighter.lua_state_agent, 18.0);
		macros::FT_MOTION_RATE(fighter, /*FSM*/ 1);
		if macros::is_excute(fighter) {
			WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_JACK_STATUS_SPECIAL_S_FLAG_ENABLE_CONTROL_ENERGY);
		}
		frame(fighter.lua_state_agent, 32.0);
		if macros::is_excute(fighter) {
			WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_JACK_STATUS_SPECIAL_S_FLAG_SET_FALL_NORMAL);
		}
}
	
unsafe extern "C" fn joker_sideb(fighter: &mut L2CAgentBase) {
        let lua_state = fighter.lua_state_agent;
		let ENTRY_ID = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
		if macros::is_excute(fighter) {
			BATON_TYPE[ENTRY_ID] += 1;
			if BATON_TYPE[ENTRY_ID] > BATON_MAX {
				BATON_TYPE[ENTRY_ID] = 0;
			}
		};
		frame(fighter.lua_state_agent, 5.0);
		if macros::is_excute(fighter) {
			if BATON_TYPE[ENTRY_ID] == 0 {
				ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_JACK_GENERATE_ARTICLE_MONA, false, 0);
				ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_JACK_GENERATE_ARTICLE_MONA,smash::phx::Hash40::new("special_s1"),false,0.0);
			} else if BATON_TYPE[ENTRY_ID] == 1 {
				ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_JACK_GENERATE_ARTICLE_MONA, false, 0);
				ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_JACK_GENERATE_ARTICLE_MONA,smash::phx::Hash40::new("special_s2"),false,0.0);
			} else {
				ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_JACK_GENERATE_ARTICLE_MONA, false, 0);
				ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_JACK_GENERATE_ARTICLE_MONA,smash::phx::Hash40::new("special_s3"),false,0.0);
			};
		}
		//ACMD for each one of the baton passes
		if BATON_TYPE[ENTRY_ID] == 0 { //Skull (Ryuji)
			frame(fighter.lua_state_agent, 15.0);
			if macros::is_excute(fighter) {
				macros::ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 6.0, /*Angle*/ 270, /*KBG*/ 90, /*FKB*/ 0, /*BKB*/ 40, /*Size*/ 4.0, /*X*/ 0.0, /*Y*/ 55.0, /*Z*/ 28.0, /*X2*/ Some(0.0), /*Y2*/ Some(40.0), /*Z2*/ Some(28.0), /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 3, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ true, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_elec"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_ELEC, /*Type*/ *ATTACK_REGION_ENERGY);
				macros::ATTACK(fighter, /*ID*/ 2, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 6.0, /*Angle*/ 361, /*KBG*/ 90, /*FKB*/ 0, /*BKB*/ 40, /*Size*/ 3.6, /*X*/ 0.0, /*Y*/ 40.0, /*Z*/ 28.0, /*X2*/ Some(0.0), /*Y2*/ Some(4.0), /*Z2*/ Some(28.0), /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 3, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ true, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_elec"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_ELEC, /*Type*/ *ATTACK_REGION_ENERGY);
				macros::ATTACK(fighter, /*ID*/ 	0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 10.0, /*Angle*/ 361, /*KBG*/ 90, /*FKB*/ 0, /*BKB*/ 40, /*Size*/ 6.0, /*X*/ 0.0, /*Y*/ 5.0, /*Z*/ 28.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.5, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 3, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ true, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_fire"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_ELEC, /*Type*/ *ATTACK_REGION_ENERGY);
			};
			wait(fighter.lua_state_agent, 2.0);
			if macros::is_excute(fighter) {
				AttackModule::clear_all(fighter.module_accessor);
			};
		} else if BATON_TYPE[ENTRY_ID] == 1 { // Panther (Ann)
			frame(fighter.lua_state_agent, 20.0);
			if macros::is_excute(fighter) {
				macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 14.0, /*Angle*/ 361, /*KBG*/ 92, /*FKB*/ 0, /*BKB*/ 40, /*Size*/ 13.0, /*X*/ 0.0, /*Y*/ 8.0, /*Z*/ 30.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_fire"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_FIRE, /*Type*/ *ATTACK_REGION_BOMB);
			};
			wait(fighter.lua_state_agent, 2.0);
			if macros::is_excute(fighter) {
				AttackModule::clear_all(fighter.module_accessor);
			};
		} else { //Mona (Morgana)
			frame(fighter.lua_state_agent, 15.0);
			if macros::is_excute(fighter) {
				macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 1.0, /*Angle*/ 361, /*KBG*/ 10, /*FKB*/ 0, /*BKB*/ 15, /*Size*/ 11.0, /*X*/ 0.0, /*Y*/ 8.0, /*Z*/ 30.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.4, /*SDI*/ 0.6, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 3, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_PUNCH);
				AttackModule::set_add_reaction_frame(fighter.module_accessor, /*ID*/ 0, /*Frames*/ 6.0, /*Unk*/ false);
			};
			frame(fighter.lua_state_agent, 30.0);
			if macros::is_excute(fighter) {
				AttackModule::clear_all(fighter.module_accessor);
				macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 5.0, /*Angle*/ 110, /*KBG*/ 60, /*FKB*/ 0, /*BKB*/ 67, /*Size*/ 12.0, /*X*/ 0.0, /*Y*/ 8.0, /*Z*/ 30.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.4, /*SDI*/ 0.6, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 3, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_PUNCH);
			};
			wait(fighter.lua_state_agent, 2.0);
			if macros::is_excute(fighter) {
				AttackModule::clear_all(fighter.module_accessor);
			};
		}
		frame(fighter.lua_state_agent, 48.0);
		if macros::is_excute(fighter) {
			ArticleModule::remove_exist(fighter.module_accessor, *FIGHTER_JACK_GENERATE_ARTICLE_MONA,smash::app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
		}
}	
unsafe extern "C" fn joker_sideb_eff(fighter: &mut L2CAgentBase) {
        let lua_state = fighter.lua_state_agent;
		let ENTRY_ID = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
		frame(fighter.lua_state_agent, 2.0);
		if BATON_TYPE[ENTRY_ID] == 0 { //Skull (Ryuji)
			frame(fighter.lua_state_agent, 14.0);
			if macros::is_excute(fighter) {
				macros::EFFECT(fighter, Hash40::new("sys_thunder_flash"), Hash40::new("top"), 0, 0, 28, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
			};
			println!("");
		} else if BATON_TYPE[ENTRY_ID] == 1 { // Panther (Ann)
			frame(fighter.lua_state_agent, 19.0);
			if macros::is_excute(fighter) {
				macros::EFFECT(fighter, Hash40::new("sys_bomb_a"), Hash40::new("top"), 0.0, 8.0, 30.0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, true);
			};
		} else { //Mona (Morgana)
			frame(fighter.lua_state_agent, 14.0);
			for _ in 0..5 {
				if macros::is_excute(fighter) {
					macros::EFFECT_FOLLOW(fighter, Hash40::new("sys_spin_wind"), Hash40::new("top"), 0.0, 12.0, 30.0, 0, 0, 0, 1.2*0.7, true);
					macros::LAST_EFFECT_SET_COLOR(fighter, 0.69, 2.45, 0.66);
					macros::EFFECT_FOLLOW(fighter, Hash40::new("sys_spin_wind"), Hash40::new("top"), 0.0, 7.5, 30.0, 0, 10, 0, 1.0*0.7, true);
					macros::LAST_EFFECT_SET_COLOR(fighter, 0.69, 2.45, 0.66);
					macros::EFFECT_FOLLOW(fighter, Hash40::new("sys_spin_wind"), Hash40::new("top"), 0.0, 5.5, 30.0, 0, -18, 0, 0.9*0.7, true);
					macros::LAST_EFFECT_SET_COLOR(fighter, 0.69, 2.45, 0.66);
				};	
				wait(fighter.lua_state_agent, 1.0);
				if macros::is_excute(fighter) {
					macros::EFFECT_FOLLOW(fighter, Hash40::new("sys_spin_wind"), Hash40::new("top"), 0.0, 9.0, 30.0, 0, 25, 0, 1.1*0.7, true);
					macros::LAST_EFFECT_SET_COLOR(fighter, 0.69, 2.45, 0.66);
					macros::EFFECT_FOLLOW(fighter, Hash40::new("sys_spin_wind"), Hash40::new("top"), 0.0, 10.2, 30.0, 0, -16, 0, 0.8*0.7, true);
					macros::LAST_EFFECT_SET_COLOR(fighter, 0.69, 2.45, 0.66);
					macros::EFFECT_FOLLOW(fighter, Hash40::new("sys_spin_wind"), Hash40::new("top"), 0.0, 5.5, 30.0, 0, 3, 0, 1.3*0.7, true);
					macros::LAST_EFFECT_SET_COLOR(fighter, 0.69, 2.45, 0.66);
				};	
				wait(fighter.lua_state_agent, 1.0);
				if macros::is_excute(fighter) {
					macros::EFFECT_FOLLOW(fighter, Hash40::new("sys_spin_wind"), Hash40::new("top"), 0.0, 7.0, 30.0, 0, 11, 0, 1.0*0.7, true);
					macros::LAST_EFFECT_SET_COLOR(fighter, 0.69, 2.45, 0.66);
					macros::EFFECT_FOLLOW(fighter, Hash40::new("sys_spin_wind"), Hash40::new("top"), 0.0, 10.5, 30.0, 0, 13, 0, 1.325*0.7, true);
					macros::LAST_EFFECT_SET_COLOR(fighter, 0.69, 2.45, 0.66);
					macros::EFFECT_FOLLOW(fighter, Hash40::new("sys_spin_wind"), Hash40::new("top"), 0.0, 3.5, 30.0, 0, -5, 0, 1.15*0.7, true);
					macros::LAST_EFFECT_SET_COLOR(fighter, 0.69, 2.45, 0.66);
				};	
				wait(fighter.lua_state_agent, 1.0);
			};
			frame(fighter.lua_state_agent, 30.0);
			if macros::is_excute(fighter) {
				macros::EFFECT_FOLLOW(fighter, Hash40::new("sys_spin_wind"), Hash40::new("top"), 0.0, 12.0, 30.0, 0, 0, 0, 1.2*1.5*0.7, true);
				macros::LAST_EFFECT_SET_COLOR(fighter, 0.69, 2.45, 0.66);
				macros::EFFECT_FOLLOW(fighter, Hash40::new("sys_spin_wind"), Hash40::new("top"), 0.0, 5.5, 30.0, 0, 10, 0, 1.0*1.5*0.7, true);
				macros::LAST_EFFECT_SET_COLOR(fighter, 0.69, 2.45, 0.66);
				macros::EFFECT_FOLLOW(fighter, Hash40::new("sys_spin_wind"), Hash40::new("top"), 0.0, 7.5, 30.0, 0, -18, 0, 0.9*1.5*0.7, true);
				macros::LAST_EFFECT_SET_COLOR(fighter, 0.69, 2.45, 0.66);
			};
		}
		frame(fighter.lua_state_agent, 47.0);
		if macros::is_excute(fighter) {
			if BATON_TYPE[ENTRY_ID] != 2 { //Not Mona
				macros::EFFECT(fighter, Hash40::new("jack_mona_smoke"), Hash40::new("top"), 0.0, 6.0, 15.0, 0, 0, 0, 3.0, 0, 0, 0, 0, 0, 0, true);
			} else {
				macros::EFFECT(fighter, Hash40::new("jack_mona_smoke"), Hash40::new("top"), 0.0, 2.0, 15.0, 0, 0, 0, 1.0, 0, 0, 0, 0, 0, 0, true);
			};
		}
}	
unsafe extern "C" fn joker_sideb_snd(fighter: &mut L2CAgentBase) {
        let lua_state = fighter.lua_state_agent;
		let ENTRY_ID = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
		frame(fighter.lua_state_agent, 3.0);
		let rand_val = smash::app::sv_math::rand(hash40("fighter"), 3);
		if BATON_TYPE[ENTRY_ID] == 0 { //Skull (Ryuji)
			frame(fighter.lua_state_agent, 6.0);
			if macros::is_excute(fighter) {
				if rand_val == 0 {
					macros::PLAY_SE(fighter, Hash40::new("vc_jack_special_s02")); // vc 2
				} else {
					macros::PLAY_SE(fighter, Hash40::new("vc_jack_special_n03")); // vc 1
				};
			};
			frame(fighter.lua_state_agent, 14.0);
			if macros::is_excute(fighter) {
				macros::PLAY_SE(fighter, Hash40::new("se_common_smashswing_02"));
				macros::PLAY_SE(fighter, Hash40::new("se_common_down_m_02"));
				macros::PLAY_SE(fighter, Hash40::new("se_common_electric_hit_s"));
			};
			println!("");
		} else if BATON_TYPE[ENTRY_ID] == 1 { // Panther (Ann)
			frame(fighter.lua_state_agent, 8.0);
			if macros::is_excute(fighter) {
				if rand_val == 0 {
					macros::PLAY_SE(fighter, Hash40::new("vc_jack_special_n02")); // vc 1
				} else {
					macros::PLAY_SE(fighter, Hash40::new("vc_jack_special_s01")); // vc 2
				};
			};
			frame(fighter.lua_state_agent, 19.0);
			if macros::is_excute(fighter) {
				macros::PLAY_SE(fighter, Hash40::new("se_common_bomb_ll"));
			};
		} else { //Mona (Morgana)
			frame(fighter.lua_state_agent, 6.0);
			if macros::is_excute(fighter) {
				if rand_val == 0 {
					macros::PLAY_SE(fighter, Hash40::new("vc_jack_special_n01")); // vc 1
				} else {
					macros::PLAY_SE(fighter, Hash40::new("vc_jack_special_n04")); // vc 2
				};
			};
			frame(fighter.lua_state_agent, 14.0);
			for _ in 0..5 {
				if macros::is_excute(fighter) {
					macros::PLAY_SE(fighter, Hash40::new("se_common_swing_01"));
				}
				wait(fighter.lua_state_agent, 1.0);
				if macros::is_excute(fighter) {
					macros::PLAY_SE(fighter, Hash40::new("se_common_swing_03"));
				}
				wait(fighter.lua_state_agent, 1.0);
				if macros::is_excute(fighter) {
					macros::PLAY_SE(fighter, Hash40::new("se_common_swing_02"));
				}
				wait(fighter.lua_state_agent, 1.0);
			};
			frame(fighter.lua_state_agent, 30.0);
			if macros::is_excute(fighter) {
				macros::PLAY_SE(fighter, Hash40::new("se_common_swing_08"));
			}
		}
}	


unsafe extern "C" fn joker_sideb_expr(agent: &mut L2CAgentBase) {
	let lua_state = agent.lua_state_agent;
	let ENTRY_ID = WorkModule::get_int(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
	if BATON_TYPE[ENTRY_ID] == 0 { //Skull (Ryuji)
		frame(agent.lua_state_agent, 15.0);
		if macros::is_excute(agent) {
			macros::RUMBLE_HIT(agent, Hash40::new("rbkind_attackl"), 0);
			ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_elecattack"), 0, true, *BATTLE_OBJECT_ID_INVALID as u32);
		}
	} else if BATON_TYPE[ENTRY_ID] == 1 { // Panther (Ann)
		frame(agent.lua_state_agent, 20.0);
		if macros::is_excute(agent) {
			macros::RUMBLE_HIT(agent, Hash40::new("rbkind_explosion"), 0);
			ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohit_explosion"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
		}
	} else { //Mona (Morgana)
		frame(agent.lua_state_agent, 15.0);
		if macros::is_excute(agent) {
			macros::RUMBLE_HIT(agent, Hash40::new("rbkind_attackm"), 0);
			ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohitm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
		}
	}
}	