use smash::hash40;
use smash::lib::lua_const::*;
use smash::app::lua_bind::*;
use smash::lua2cpp::*;
use smashline::*;
use smash_script::*;
use smash::app::sv_animcmd::*;
use smash::phx::Hash40;
use smash::app::ArticleOperationTarget;
use smash::lib::L2CValue;
use smash::phx::Vector3f;

static mut FIREBALL : [i32; 8] = [0; 8];
static mut UPB_ANGLE : [i32; 8] = [1; 8];
static mut SPECIAL_ZOOM_GFX : [i32; 8] = [0; 8];
static mut KOOPA_EXCELLENT_SMASH : [bool; 8] = [false; 8];
static NONE :  smash::phx::Vector3f =  smash::phx::Vector3f { x: 0.0, y: 0.0, z: 0.0 };

#[acmd_script(
    agent = "koopa",
    script =  "game_attackairn",
    category = ACMD_GAME,
	low_priority)]
unsafe fn bowser_nair(fighter: &mut L2CAgentBase) {
    	let lua_state = fighter.lua_state_agent;
        if macros::is_excute(fighter) {
            WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
        }
        frame(fighter.lua_state_agent, 8.0);
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("bust"), /*Damage*/ 14.0, /*Angle*/ 50, /*KBG*/ 65, /*FKB*/ 0, /*BKB*/ 50, 12.0, 0.0, -3.0, 0.0, None, None, None, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_PUNCH, /*Type*/ *ATTACK_REGION_PUNCH);
        }
        wait(fighter.lua_state_agent, 2.0);
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("bust"), /*Damage*/ 7.0, /*Angle*/ 90, /*KBG*/ 90, /*FKB*/ 0, /*BKB*/ 40, 9.0, 0.0, -3.0, 0.0, None, None, None, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_KICK, /*Type*/ *ATTACK_REGION_PUNCH);
        }
        frame(fighter.lua_state_agent, 39.0);
        if macros::is_excute(fighter) {
            AttackModule::clear_all(fighter.module_accessor);
        }
        frame(fighter.lua_state_agent, 47.0);
        if macros::is_excute(fighter) {
            WorkModule::off_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
        }
}
#[acmd_script(
    agent = "koopa",
    script =  "effect_attackairn",
    category = ACMD_EFFECT,
	low_priority)]
unsafe fn bowser_nair_eff(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 8.0);
    for _ in 0..2 {
        if macros::is_excute(agent) {
            macros::EFFECT_FOLLOW_FLIP(agent, Hash40::new("koopa_shell_a"), Hash40::new("koopa_shell_b"), Hash40::new("rot"), 0, 2, 0, 0, 0, 0, 1.45, true, *EF_FLIP_NONE);
        }
        wait(agent.lua_state_agent, 5.0);
        if macros::is_excute(agent) {
            macros::EFFECT_FOLLOW_FLIP(agent, Hash40::new("koopa_shell_a"), Hash40::new("koopa_shell_b"), Hash40::new("rot"), 0, 4, 0, 0, 0, 0, 1.45, true, *EF_FLIP_NONE);
        }
        wait(agent.lua_state_agent, 5.0);
    }
    frame(agent.lua_state_agent, 29.0);
    if macros::is_excute(agent) {
        macros::EFFECT_OFF_KIND(agent, Hash40::new("koopa_shell_a"), false, true);
    }
}
#[acmd_script(
    agent = "koopa",
    script =  "sound_attackairn",
    category = ACMD_SOUND,
	low_priority)]
unsafe fn bowser_nair_snd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 8.0);
    if macros::is_excute(agent) {
        macros::PLAY_STATUS(agent, Hash40::new("se_koopa_special_h01"));
    }
}
#[acmd_script(
    agent = "koopa",
    script =  "game_attackdash",
    category = ACMD_GAME,
	low_priority)]
unsafe fn bowser_da(fighter: &mut L2CAgentBase) {
    	let lua_state = fighter.lua_state_agent;
		frame(fighter.lua_state_agent, 15.0);
		if macros::is_excute(fighter) {
			macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("handr"), /*Damage*/ 15.0, /*Angle*/ 50, /*KBG*/ 70, /*FKB*/ 0, /*BKB*/ 80, 6.0, 3.5, -0.6, 0.5, None, None, None, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_PUNCH);
			macros::ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 15.0, /*Angle*/ 50, /*KBG*/ 70, /*FKB*/ 0, /*BKB*/ 80, 5.5, 0.0, 24.0, 3.0, Some(0.0), Some(17.0), Some(7.0), /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_PUNCH);
			macros::ATTACK(fighter, /*ID*/ 2, /*Part*/ 0, /*Bone*/ Hash40::new("armr"), /*Damage*/ 15.0, /*Angle*/ 50, /*KBG*/ 70, /*FKB*/ 0, /*BKB*/ 80, 5.0, 0.0, 0.0, 0.5, None, None, None, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_PUNCH);
		}
		wait(fighter.lua_state_agent, 3.0);
		if macros::is_excute(fighter) {
			AttackModule::clear_all(fighter.module_accessor);
		}
}
#[acmd_script(
    agent = "koopa",
    script =  "effect_attackdash",
    category = ACMD_EFFECT,
	low_priority)]
unsafe fn bowser_da_eff(fighter: &mut L2CAgentBase) {
    	let lua_state = fighter.lua_state_agent;
		frame(lua_state, 13.0);
		if macros::is_excute(fighter) {
			macros::FOOT_EFFECT(fighter, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.2, 0, 0, 0, 0, 0, 0, false);
			macros::LAST_EFFECT_SET_RATE(fighter, 1.5);
		}
		frame(lua_state, 15.0);
		if  macros::is_excute(fighter) {
			macros::EFFECT_FOLLOW_FLIP(fighter, Hash40::new("koopa_scratch"), Hash40::new("koopa_scratch"), Hash40::new("top"), -2, 20, 5.5, 19, -8, -52, 1.6, true, *EF_FLIP_YZ);
			macros::LAST_EFFECT_SET_RATE(fighter, 1.5);
			EffectModule::set_visible_kind(fighter.module_accessor, Hash40::new("koopa_scratch"), false);
	
			macros::EFFECT_FOLLOW(fighter, Hash40::new("sys_attack_speedline"), Hash40::new("top"), -4, 13, -30, 0, 0, 0, 2.5, true);
			macros::LAST_EFFECT_SET_RATE(fighter, 1.75);
		}
		frame(lua_state, 16.0);
		if macros::is_excute(fighter) {
			EffectModule::set_visible_kind(fighter.module_accessor, Hash40::new("koopa_scratch"), true);
		}
		frame(lua_state, 17.0);
		if macros::is_excute(fighter) {
			if PostureModule::lr(fighter.module_accessor) > 0.0 {
				macros::EFFECT(fighter, Hash40::new("sys_damage_spark_s"), Hash40::new("top"), 18.0, 2.0, 0.0, 0,0,140,0.9, 0, 0, 0, 0, 0, 0, false);
			}
			else{
				macros::EFFECT(fighter, Hash40::new("sys_damage_spark_s"), Hash40::new("top"), 18.0, 2.0, 0.0, 0,0,-140,0.9, 0, 0, 0, 0, 0, 0, false);
			}
			macros::LAST_EFFECT_SET_RATE(fighter,1.5);
		}
}
#[acmd_script(
    agent = "koopa",
    script =  "sound_attackdash",
    category = ACMD_SOUND,
	low_priority)]
unsafe fn bowser_da_snd(fighter: &mut L2CAgentBase) {
    	let lua_state = fighter.lua_state_agent;
		frame(lua_state, 15.0);
		if macros::is_excute(fighter) {
			let play_vc = smash::app::sv_math::rand(hash40("fighter"), 3) == 0;
			if play_vc {
				macros::PLAY_SE(fighter, Hash40::new("vc_koopa_attack07"));
			}
		}
		frame(lua_state, 16.0);
		if macros::is_excute(fighter) {
			macros::PLAY_SE(fighter, Hash40::new("se_koopa_smash_l02"));
		}
}
#[acmd_script(
    agent = "koopa",
    script =  "expression_attackdash",
    category = ACMD_EXPRESSION,
	low_priority)]
unsafe fn bowser_da_expr(fighter: &mut L2CAgentBase) {
    	let lua_state = fighter.lua_state_agent;
		if macros::is_excute(fighter) {
			slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
		}
		frame(lua_state, 16.0);
		if macros::is_excute(fighter) {
			macros::RUMBLE_HIT(fighter, Hash40::new("rbkind_slashl"), 0);
		}
}
#[acmd_script(
    agent = "koopa",
    script =  "game_attackairlw",
    category = ACMD_GAME,
	low_priority)]
unsafe fn bowser_dair(fighter: &mut L2CAgentBase) {
    	let lua_state = fighter.lua_state_agent;
		frame(fighter.lua_state_agent, 14.0);
		if macros::is_excute(fighter) {
			WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
		}
		frame(fighter.lua_state_agent, 17.0);
		if macros::is_excute(fighter) {
			macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("haver"), /*Damage*/ 16.0, /*Angle*/ 270, /*KBG*/ 95, /*FKB*/ 0, /*BKB*/ 20, /*Size*/ 8.5, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.5, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_A, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_PUNCH, /*Type*/ *ATTACK_REGION_PUNCH);
			macros::ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("haver"), /*Damage*/ 16.0, /*Angle*/ 361, /*KBG*/ 95, /*FKB*/ 0, /*BKB*/ 20, /*Size*/ 8.5, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.5, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_G_d, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_PUNCH, /*Type*/ *ATTACK_REGION_PUNCH);
			macros::ATTACK(fighter, /*ID*/ 2, /*Part*/ 0, /*Bone*/ Hash40::new("haver"), /*Damage*/ 20.0, /*Angle*/ 270, /*KBG*/ 95, /*FKB*/ 0, /*BKB*/ 20, /*Size*/ 8.5, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.5, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_G, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_PUNCH, /*Type*/ *ATTACK_REGION_PUNCH);
		}
		frame(fighter.lua_state_agent, /*Frames*/ 20.0);
		if macros::is_excute(fighter) {
			AttackModule::clear_all(fighter.module_accessor);
		}
		frame(fighter.lua_state_agent, /*Frames*/ 22.0);
		macros::FT_MOTION_RATE(fighter, /*FSM*/ 2.0);
		wait(fighter.lua_state_agent, 9.0);
		if macros::is_excute(fighter) {
			WorkModule::off_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
		}
		frame(fighter.lua_state_agent, 40.0);
		if macros::is_excute(fighter) {
			CancelModule::enable_cancel(fighter.module_accessor);
		}
}
#[acmd_script(
    agent = "koopa_breath",
    script =  "game_move",
    category = ACMD_GAME,
	low_priority)]
unsafe fn bowser_fireball(fighter: &mut L2CAgentBase) {
    	let lua_state = fighter.lua_state_agent;
		if macros::is_excute(fighter) {
			macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 14.0, /*Angle*/ 45, /*KBG*/ 60, /*FKB*/ 0, /*BKB*/ 40, /*Size*/ 4.5, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.5, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ true, /*Absorbable*/ true, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_fire"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_FIRE, /*Type*/ *ATTACK_REGION_NONE);
			AttackModule::enable_safe_pos(fighter.module_accessor);
		}
}
#[acmd_script(
    agent = "koopa",
    script =  "effect_attackairlw",
    category = ACMD_EFFECT,
	low_priority)]
unsafe fn bowser_eff_dair(fighter: &mut L2CAgentBase) {
    	let lua_state = fighter.lua_state_agent;
		frame(fighter.lua_state_agent, 16.0);
		if macros::is_excute(fighter) {
			macros::EFFECT_FOLLOW_ALPHA(fighter, Hash40::new("sys_attack_impact"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 2.0, true, 2);
		}
}		
#[acmd_script(
    agent = "koopa",
    script =  "effect_landingairlw",
    category = ACMD_EFFECT,
	low_priority)]
unsafe fn bowser_eff_land_dair(fighter: &mut L2CAgentBase) {
    	let lua_state = fighter.lua_state_agent;
		if macros::is_excute(fighter) {
			macros::LANDING_EFFECT(fighter, Hash40::new("sys_landing_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.3, 0, 0, 0, 0, 0, 0, false);
		}
}	
#[acmd_script(
    agent = "koopa",
    script =  "game_landingairlw",
    category = ACMD_GAME,
	low_priority)]
unsafe fn bowser_land_dair(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
}	
#[acmd_script(
    agent = "koopa",
    script =  "sound_landingairlw",
    category = ACMD_SOUND,
	low_priority)]
unsafe fn bowser_snd_land_dair(fighter: &mut L2CAgentBase) {
    	let lua_state = fighter.lua_state_agent;
		frame(fighter.lua_state_agent, 1.0);
		if macros::is_excute(fighter) {
			macros::PLAY_LANDING_SE(fighter, Hash40::new("se_koopa_landing02"));
		}
	}	
#[acmd_script(
    agent = "koopa",
    script =  "sound_attackairlw",
    category = ACMD_SOUND,
	low_priority)]
unsafe fn bowser_snd_dair(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
		frame(fighter.lua_state_agent, 14.0);
		if macros::is_excute(fighter) {
			macros::PLAY_SE(fighter, Hash40::new("se_koopa_nailswing02"));
			macros::PLAY_SE(fighter, Hash40::new("vc_koopa_attack06"));
		}
		frame(fighter.lua_state_agent, 16.0);
		if macros::is_excute(fighter) {
			macros::PLAY_SE(fighter, Hash40::new("se_koopa_smash_h01"));
		}
}		

#[acmd_script(
    agent = "koopa",
    script =  "game_attacks4",
    category = ACMD_GAME,
	low_priority)]
unsafe fn bowser_fsmash(fighter: &mut L2CAgentBase) {
    	let lua_state = fighter.lua_state_agent;
		frame(fighter.lua_state_agent, 13.0);
        if macros::is_excute(fighter) {
            WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
        }
		frame(fighter.lua_state_agent, 22.0);
		if macros::is_excute(fighter) {
			macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("haver"), /*Damage*/ 22.0, /*Angle*/ 361, /*KBG*/ 97, /*FKB*/ 0, /*BKB*/ 28, /*Size*/ 8.5, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_PUNCH, /*Type*/ *ATTACK_REGION_PUNCH);
			macros::ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("haver"), /*Damage*/ 22.0, /*Angle*/ 361, /*KBG*/ 97, /*FKB*/ 0, /*BKB*/ 28, /*Size*/ 8.5, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_PUNCH, /*Type*/ *ATTACK_REGION_PUNCH);
			macros::ATTACK(fighter, /*ID*/ 2, /*Part*/ 0, /*Bone*/ Hash40::new("haver"), /*Damage*/ 25.0, /*Angle*/ 361, /*KBG*/ 97, /*FKB*/ 0, /*BKB*/ 28, /*Size*/ 8.5, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_PUNCH, /*Type*/ *ATTACK_REGION_PUNCH);
		}
		frame(fighter.lua_state_agent, /*Frames*/ 26.0);
		if macros::is_excute(fighter) {
			AttackModule::clear_all(fighter.module_accessor);
		}
}
#[acmd_script(
    agent = "koopa",
    script =  "effect_attacks4",
    category = ACMD_EFFECT,
	low_priority)]
unsafe fn bowser_fsmash_eff(fighter: &mut L2CAgentBase) {
    	let lua_state = fighter.lua_state_agent;
		frame(fighter.lua_state_agent, 21.0);
		if macros::is_excute(fighter) {
			macros::EFFECT_FOLLOW_ALPHA(fighter, Hash40::new("sys_attack_impact"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 2.0, true, 2);
			macros::FOOT_EFFECT(fighter, Hash40::new("sys_turn_smoke"), Hash40::new("top"), -2.0, 0, 0, 0, 0, 0, 2.0, 0, 0, 0, 0, 0, 0, false);
		}
}	
#[acmd_script(
    agent = "koopa",
    script =  "sound_attacks4",
    category = ACMD_SOUND,
	low_priority)]
unsafe fn bowser_fsmash_snd(fighter: &mut L2CAgentBase) {
    	let lua_state = fighter.lua_state_agent;
		frame(fighter.lua_state_agent, 19.0);
		if macros::is_excute(fighter) {
			macros::PLAY_SE(fighter, Hash40::new("se_koopa_nailswing02"));
			macros::PLAY_SE(fighter, Hash40::new("vc_koopa_attack06"));
		}
		frame(fighter.lua_state_agent, 21.0);
		if macros::is_excute(fighter) {
			macros::PLAY_SE(fighter, Hash40::new("se_koopa_smash_h01"));
		}
}	

#[fighter_frame( agent = FIGHTER_KIND_KIRBY )]
fn kirby_bowser_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
		let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);   
		let status_kind = smash::app::lua_bind::StatusModule::status_kind(boma);
		let ENTRY_ID = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize; 
		let frame = MotionModule::frame(boma);
		let end_frame = MotionModule::end_frame(boma);
		if [hash40("koopa_special_n")].contains(&MotionModule::motion_kind(boma)){
			if end_frame-frame < 5.0 {
				StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_WAIT, true);
			};
			if frame >= 19.0 {
				CancelModule::enable_cancel(boma);
			};
			MotionModule::set_rate(boma, 0.775);
		};
		if [hash40("koopa_special_air_n")].contains(&MotionModule::motion_kind(boma)){
			if end_frame-frame < 5.0 {
				StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_FALL, true);
			};
			if frame >= 19.0 {
				CancelModule::enable_cancel(boma);
			};
			MotionModule::set_rate(boma, 0.775);
		};
		if [hash40("koopa_special_n_end")].contains(&MotionModule::motion_kind(boma)){
			StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_WAIT, true);
		};
		if [hash40("koopa_special_air_n_end")].contains(&MotionModule::motion_kind(boma)){
				StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_FALL, true);
		};
		if ArticleModule::is_exist(boma, *FIGHTER_KOOPA_GENERATE_ARTICLE_BREATH) {
			FIREBALL[ENTRY_ID] += 1;
		} else {
			FIREBALL[ENTRY_ID] = 0;
		};
		macros::EFFECT_OFF_KIND(fighter, Hash40::new("koopa_breath_m_fire"), false, true);
	}
}		
#[fighter_frame( agent = FIGHTER_KIND_KOOPA )]
fn bowser_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
		let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);   
		let status_kind = smash::app::lua_bind::StatusModule::status_kind(boma);
		let ENTRY_ID = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize; 
		let frame = MotionModule::frame(boma);
		let end_frame = MotionModule::end_frame(boma);
        if [hash40("special_s_catch"), hash40("special_s_air_catch")].contains(&MotionModule::motion_kind(boma)){
			if MotionModule::frame(boma) < 6.0 {
				MotionModule::set_rate(boma, 0.6);
			} else {
				MotionModule::set_rate(boma, 1.0);
			};
		};
        if [hash40("attack_air_lw")].contains(&MotionModule::motion_kind(boma)){
			if PostureModule::lr(boma) == -1.0 {
				PostureModule::reverse_lr(boma);
				PostureModule::update_rot_y_lr(boma);
			};
		};
		if [hash40("special_n")].contains(&MotionModule::motion_kind(boma)){
			if end_frame-frame < 5.0 {
				StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_WAIT, true);
			};
			if frame >= 19.0 {
				CancelModule::enable_cancel(boma);
			};
			MotionModule::set_rate(boma, 0.775);
		};
		if [hash40("special_air_n")].contains(&MotionModule::motion_kind(boma)){
			if end_frame-frame < 5.0 {
				StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_FALL, true);
			};
			if frame >= 19.0 {
				CancelModule::enable_cancel(boma);
			};
			MotionModule::set_rate(boma, 0.775);
		};
		if [hash40("special_n_end")].contains(&MotionModule::motion_kind(boma)){
			StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_WAIT, true);
		};
		if [hash40("special_air_n_end")].contains(&MotionModule::motion_kind(boma)){
				StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_FALL, true);
		};
		if ArticleModule::is_exist(boma, *FIGHTER_KOOPA_GENERATE_ARTICLE_BREATH) {
			FIREBALL[ENTRY_ID] += 1;
		} else {
			FIREBALL[ENTRY_ID] = 0;
		};
		macros::EFFECT_OFF_KIND(fighter, Hash40::new("koopa_breath_m_fire"), false, true);

		//Fsmash shit
		if status_kind == *FIGHTER_STATUS_KIND_ATTACK_S4_HOLD {
            if frame >= 50.0 && frame < 60.0 {
                KOOPA_EXCELLENT_SMASH[ENTRY_ID] = true;
            }
            else {
                KOOPA_EXCELLENT_SMASH[ENTRY_ID] = false;
            }
        }
		if [*FIGHTER_STATUS_KIND_ATTACK_S4_START, *FIGHTER_STATUS_KIND_ATTACK_S4].contains(&status_kind) {
            if KOOPA_EXCELLENT_SMASH[ENTRY_ID] == true {
                if AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_HIT) {
                    SPECIAL_ZOOM_GFX[ENTRY_ID] += 1;
                    if SPECIAL_ZOOM_GFX[ENTRY_ID] < 2 {
                        SlowModule::set_whole(boma, 8, 80);
                        macros::CAM_ZOOM_IN_arg5(fighter, /*frames*/ 2.0,/*no*/ 0.0,/*zoom*/ 1.8,/*yrot*/ 0.0,/*xrot*/ 0.0);
                        EffectModule::req_follow(boma, Hash40::new("sys_bg_criticalhit"), Hash40::new("top"), &Vector3f{x: 0.0, y: 0.0, z: 0.0} as *const Vector3f, &Vector3f{x: 0.0, y: 0.0, z: 0.0} as *const Vector3f, 1.0, false, 0, 0, 0, 0, 0, false, false);
                        macros::PLAY_SE(fighter, Hash40::new("se_common_criticalhit"));
                        macros::QUAKE(fighter, *CAMERA_QUAKE_KIND_XL);
                    }
                    if SPECIAL_ZOOM_GFX[ENTRY_ID] >= 6 {
                        SlowModule::clear_whole(boma);
                        CameraModule::reset_all(boma);
                        EffectModule::kill_kind(boma, Hash40::new("sys_bg_criticalhit"), false, false);
                        macros::CAM_ZOOM_OUT(fighter);
                    }
                }
            }
        }
		if ![*FIGHTER_STATUS_KIND_ATTACK_S4_HOLD, *FIGHTER_STATUS_KIND_ATTACK_S4_START, *FIGHTER_STATUS_KIND_ATTACK_S4].contains(&status_kind) {
			KOOPA_EXCELLENT_SMASH[ENTRY_ID] = false;
			SPECIAL_ZOOM_GFX[ENTRY_ID] = 0;
		}
	}
}		
#[weapon_frame( agent = WEAPON_KIND_KOOPA_BREATH )]
pub fn fireball_frame(weapon : &mut L2CFighterBase) {
    unsafe {
        let otarget_id = WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER) as u32;
        let boma = smash::app::sv_battle_object::module_accessor(otarget_id);
		let ENTRY_ID = WorkModule::get_int(&mut *boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
		if smash::app::utility::get_kind(&mut *boma) == *FIGHTER_KIND_KIRBY {
			if FIREBALL[ENTRY_ID] % 4 == 0 {
					EffectModule::kill_kind(weapon.module_accessor, Hash40::new("sys_fireflower_shot"), false, true);
					let f1: u32 = EffectModule::req_follow(weapon.module_accessor, smash::phx::Hash40::new("sys_fireflower_shot"), smash::phx::Hash40::new("top"), &NONE, &NONE, 0.9, true, 0, 0, 0, 0, 0, true, true) as u32;
					EffectModule::set_rgb(boma, f1, 1.5, 0.5, 0.5);
			};
		} else {
			if FIREBALL[ENTRY_ID] % 14 == 0 {
					EffectModule::kill_kind(weapon.module_accessor, Hash40::new("koopa_breath_m_fire"), false, true);
					let f1: u32 = EffectModule::req_follow(weapon.module_accessor, smash::phx::Hash40::new("sys_fireflower_shot"), smash::phx::Hash40::new("top"), &NONE, &NONE, 0.8, true, 0, 0, 0, 0, 0, true, true) as u32;
					EffectModule::set_rgb(boma, f1, 1.5, 0.5, 0.5);
					EffectModule::req_follow(weapon.module_accessor, smash::phx::Hash40::new("koopa_breath_m_fire"), smash::phx::Hash40::new("top"), &NONE, &NONE, 0.4, true, 0, 0, 0, 0, 0, true, true) as u32;
			};
		};
    }
}
		
pub fn install() {
    smashline::install_acmd_scripts!(
		bowser_dair,
		bowser_eff_dair,
		bowser_snd_dair,
		bowser_eff_land_dair,
		bowser_land_dair,
		bowser_snd_land_dair,
		bowser_fireball,
		bowser_fsmash,
		bowser_fsmash_eff,
		bowser_fsmash_snd,
		bowser_da, bowser_da_eff, bowser_da_snd, bowser_da_expr,
		bowser_nair, bowser_nair_eff, bowser_nair_snd
    );
    smashline::install_agent_frames!(
        bowser_frame,
		fireball_frame,
		kirby_bowser_frame
    );
}