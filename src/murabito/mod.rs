use smash::app::sv_animcmd::*;
use smash::phx::{Hash40, Vector2f};
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

static mut LAND_SIDEB_BOUNCE: [i32; 8] = [0; 8];
static mut BEFORE_SIDEB_BOUNCE: [i32; 8] = [0; 8];
static mut HAS_DOWNB: [bool; 8] = [false; 8];

#[acmd_script(
    agent = "murabito",
    script =  "game_attack11",
    category = ACMD_GAME,
	low_priority)]
unsafe fn toad_jab1(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		frame(Frame=2)
		if(is_excute){
			ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=2.0, Angle=361, KBG=25, FKB=0, BKB=20, Size=2.5, X=0.0, Y=5.5, Z=6.5, X2=0.0, Y2=5.5, Z2=7.5, Hitlag=1.5, SDI=0.8, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_PUNCH)
			ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=2.0, Angle=180, KBG=10, FKB=0, BKB=15, Size=3.0, X=-1.5, Y=5.5, Z=11.0, X2=-1.5, Y2=4.5, Z2=11.0, Hitlag=1.5, SDI=0.8, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_FIGHTER, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_PUNCH)
			ATTACK(ID=2, Part=0, Bone=hash40("top"), Damage=2.0, Angle=180, KBG=10, FKB=0, BKB=15, Size=3.0, X=1.5, Y=5.5, Z=11.0, X2=1.5, Y2=4.5, Z2=11.0, Hitlag=1.5, SDI=0.8, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_FIGHTER, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_PUNCH)
			ATTACK(ID=3, Part=0, Bone=hash40("top"), Damage=2.0, Angle=361, KBG=10, FKB=0, BKB=15, Size=3.0, X=0.0, Y=5.5, Z=11.0, X2=0.0, Y2=4.5, Z2=11.0, Hitlag=1.5, SDI=0.8, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_PUNCH)
		}
		wait(Frames=2)
		if(is_excute){
			AttackModule::clear_all()
			WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO)
		}
		frame(Frame=7)
		if(is_excute){
			WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_FLAG_ENABLE_NO_HIT_COMBO)
		}
    });
}	
#[acmd_script(
    agent = "murabito",
    script =  "effect_attack11",
    category = ACMD_EFFECT,
	low_priority)]
unsafe fn toad_jab1_eff(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		frame(Frame=2)
		if(is_excute){
			EFFECT(hash40("sys_attack_impact"), hash40("top"), 11.0, 5.5, 0, 0, 0, 0, 1, 0, 1, 1, 0, 0, 360, false)
			LAST_EFFECT_SET_ALPHA(0.7)
		}
    });
}	
#[acmd_script(
    agent = "murabito",
    script =  "game_attack12",
    category = ACMD_GAME,
	low_priority)]
unsafe fn toad_jab2(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		frame(Frame=2)
		if(is_excute){
			ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=2.0, Angle=361, KBG=25, FKB=0, BKB=20, Size=2.5, X=0.0, Y=5.5, Z=6.5, X2=0.0, Y2=5.5, Z2=7.5, Hitlag=1.5, SDI=0.8, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_PUNCH)
			ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=2.0, Angle=361, KBG=30, FKB=0, BKB=20, Size=3.0, X=-1.5, Y=5.5, Z=11.0, X2=-1.5, Y2=4.5, Z2=11.0, Hitlag=1.5, SDI=0.8, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_FIGHTER, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_PUNCH)
			ATTACK(ID=2, Part=0, Bone=hash40("top"), Damage=2.0, Angle=361, KBG=30, FKB=0, BKB=20, Size=3.0, X=1.5, Y=5.5, Z=11.0, X2=1.5, Y2=4.5, Z2=11.0, Hitlag=1.5, SDI=0.8, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_FIGHTER, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_PUNCH)
			ATTACK(ID=3, Part=0, Bone=hash40("top"), Damage=2.0, Angle=361, KBG=30, FKB=0, BKB=20, Size=3.0, X=0.0, Y=5.5, Z=11.0, X2=0.0, Y2=4.5, Z2=11.0, Hitlag=1.5, SDI=0.8, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_PUNCH)
			AttackModule::set_add_reaction_frame(ID=0, Frames=7.0, Unk=false)
			AttackModule::set_add_reaction_frame(ID=1, Frames=7.0, Unk=false)
			AttackModule::set_add_reaction_frame(ID=2, Frames=7.0, Unk=false)
			AttackModule::set_add_reaction_frame(ID=3, Frames=7.0, Unk=false)
		}
		wait(Frames=2)
		if(is_excute){
			AttackModule::clear_all()
		}
		frame(Frame=7)
		if(is_excute){
			WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO)
			WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_FLAG_ENABLE_NO_HIT_COMBO)
		}
    });
}	
#[acmd_script(
    agent = "murabito",
    script =  "sound_attack11",
    category = ACMD_SOUND,
	low_priority)]
unsafe fn toad_jab1_snd(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		frame(Frame=1)
		if(is_excute){
			PLAY_SE(hash40("se_murabito_swing_s"))
		}
    });
}	
#[acmd_script(
    agent = "murabito",
    script =  "effect_attack12",
    category = ACMD_EFFECT,
	low_priority)]
unsafe fn toad_jab2_eff(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		frame(Frame=2)
		if(is_excute){
			EFFECT(hash40("sys_attack_impact"), hash40("top"), 11.0, 5.5, 0, 0, 0, 0, 1, 0, 1, 1, 0, 0, 360, false)
			LAST_EFFECT_SET_ALPHA(0.7)
		}
    });
}	
#[acmd_script(
    agent = "murabito",
    script =  "sound_attack12",
    category = ACMD_SOUND,
	low_priority)]
unsafe fn toad_jab2_snd(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		frame(Frame=1)
		if(is_excute){
			PLAY_SE(hash40("se_murabito_swing_s"))
		}
    });
}	
#[acmd_script(
    agent = "murabito",
    script =  "game_attack13",
    category = ACMD_GAME,
	low_priority)]
unsafe fn toad_jab3(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		FT_MOTION_RATE(FSM=0.6)
		wait(Frames=10)
		FT_MOTION_RATE(FSM=1)
		frame(Frame=12)
		if(is_excute){
			ATTACK(ID=0, Part=0, Bone=hash40("head"), Damage=10.0, Angle=43, KBG=83, FKB=0, BKB=57, Size=8.5, X=4.4, Y=-1.0, Z=0.0, X=4.4, Y=-1.0, Z=0.0, Hitlag=1.2, SDI=0.8, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_PUNCH)
		}
		frame(Frame=14)
		if(is_excute){
			AttackModule::clear_all()
		}
    });
}	
#[acmd_script(
    agent = "murabito",
    script =  "effect_attack13",
    category = ACMD_EFFECT,
	low_priority)]
unsafe fn toad_jab3_eff(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		frame(Frame=12)
		if(is_excute){
			EFFECT(hash40("sys_attack_impact"), hash40("head"), 4.4, -1.0, 0.0, 0, 0, 0, 2.5, 0, 1, 1, 0, 0, 360, false)
			LAST_EFFECT_SET_ALPHA(0.7)
		}
		frame(Frame=26)
		if(is_excute){
			LANDING_EFFECT(hash40("sys_down_smoke"), hash40("top"), 3, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false)
		}
    });
}	
#[acmd_script(
    agent = "murabito",
    script =  "sound_attack13",
    category = ACMD_SOUND,
	low_priority)]
unsafe fn toad_jab3_snd(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	frame(fighter.lua_state_agent, 10.0);
	if macros::is_excute(fighter) {
		attack_vc(fighter);
		macros::PLAY_SE(fighter, Hash40::new("se_murabito_swing_l"));
	};
}	
#[acmd_script(
    agent = "murabito",
    script =  "game_attackairn",
    category = ACMD_GAME,
	low_priority)]
unsafe fn toad_nair(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		frame(Frame=4)
		if(is_excute){
			WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
			ATTACK(ID=0, Part=0, Bone=hash40("kneer"), Damage=10.0, Angle=361, KBG=100, FKB=0, BKB=10, Size=5.0, X=0.8, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_KICK)
			ATTACK(ID=1, Part=0, Bone=hash40("kneel"), Damage=10.0, Angle=361, KBG=100, FKB=0, BKB=10, Size=5.0, X=0.8, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_KICK)
		}
		wait(Frames=3)
		if(is_excute){
			ATTACK(ID=0, Part=0, Bone=hash40("legr"), Damage=7.0, Angle=361, KBG=70, FKB=0, BKB=15, Size=3.8, X=0.8, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
			ATTACK(ID=1, Part=0, Bone=hash40("kneel"), Damage=7.0, Angle=361, KBG=70, FKB=0, BKB=15, Size=3.8, X=0.8, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
		}
		frame(Frame=26)
		if(is_excute){
			AttackModule::clear_all()
		}
		frame(Frame=30)
		if(is_excute){
			WorkModule::off_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
		}
    });
}
#[acmd_script(
    agent = "murabito",
    script =  "sound_attackairn",
    category = ACMD_SOUND,
	low_priority)]
unsafe fn toad_nair_snd(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	frame(fighter.lua_state_agent, 3.0);
	if macros::is_excute(fighter) {
		attack_vc(fighter);
		macros::PLAY_SE(fighter, Hash40::new("se_murabito_swing_m"));
	};
}
#[acmd_script(
    agent = "murabito",
    script =  "game_attackairb",
    category = ACMD_GAME,
	low_priority)]
unsafe fn toad_bair(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		frame(Frame=4)
		if(is_excute){
			WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
		}
		frame(Frame=7)
		if(is_excute){
			ATTACK(ID=0, Part=0, Bone=hash40("legc"), Damage=9.0, Angle=62, KBG=85, FKB=0, BKB=47, Size=6.8, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_B, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
		}
		frame(Frame=9)
		if(is_excute){
			ATTACK(ID=0, Part=0, Bone=hash40("legc"), Damage=5.0, Angle=62, KBG=85, FKB=0, BKB=50, Size=4.0, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_B, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
		}
		frame(Frame=15)
		if(is_excute){
			AttackModule::clear_all()
		}
		frame(Frame=30)
		if(is_excute){
			WorkModule::off_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
		}
    });
}
#[acmd_script(
    agent = "murabito",
    script =  "sound_attackairb",
    category = ACMD_SOUND,
	low_priority)]
unsafe fn toad_bair_snd(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	frame(fighter.lua_state_agent, 6.0);
	if macros::is_excute(fighter) {
		attack_vc(fighter);
		macros::PLAY_SE(fighter, Hash40::new("se_murabito_swing_m"));
	};
}	
#[acmd_script(
    agent = "murabito",
    script =  "effect_attackairb",
    category = ACMD_EFFECT,
	low_priority)]
unsafe fn toad_bair_eff(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		frame(Frame=7)
		if(is_excute){
			EFFECT(hash40("sys_attack_impact"), hash40("legc"), 0, 0.0, 0.0, 0, 0, 0, 1.5, 0, 1, 1, 0, 0, 0, false)
		}
    });
}	
#[acmd_script(
    agent = "murabito",
    script =  "game_landingairhi",
    category = ACMD_GAME,
	low_priority)]
unsafe fn toad_uair_land(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		if(is_excute){
			ArticleModule::remove_exist(FIGHTER_MURABITO_GENERATE_ARTICLE_UMBRELLA,smash::app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL))
		}
    });
}
#[acmd_script(
    agent = "murabito",
    script =  "game_attackairhi",
    category = ACMD_GAME,
	low_priority)]
unsafe fn toad_uair(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		frame(Frame=2)
		if(is_excute){
			WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
		}
		frame(Frame=6)
		if(is_excute){
			ATTACK(ID=0, Part=0, Bone=hash40("head"), Damage=12.0, Angle=77, KBG=85, FKB=0, BKB=30, Size=4.2, X=4.4, Y=-1.0, Z=0.0, X2=2.4, Y2=-1.0, Z2=-3.5, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_HEAD)
		}
		frame(Frame=12)
		if(is_excute){
			AttackModule::clear_all()
		}
		frame(Frame=25)
		if(is_excute){
			WorkModule::off_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
		}
    });
}
#[acmd_script(
    agent = "murabito",
    script =  "effect_attackairhi",
    category = ACMD_EFFECT,
	low_priority)]
unsafe fn toad_uair_eff(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		frame(Frame=6)
		if(is_excute){
			EFFECT_FOLLOW_FLIP(hash40("sys_attack_arc_d"), hash40("sys_attack_arc_d"), hash40("top"), 1.5, 7.5, 0, 0, -80, -105, 0.7, true, EF_FLIP_YZ)
		}
    });
}
#[acmd_script(
    agent = "murabito",
    script =  "sound_attackairhi",
    category = ACMD_SOUND,
	low_priority)]
unsafe fn toad_uair_snd(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	frame(fighter.lua_state_agent, 5.0);
	if macros::is_excute(fighter) {
		attack_vc(fighter);
		macros::PLAY_SE(fighter, Hash40::new("se_murabito_swing_m"));
	};
}
#[acmd_script(
    agent = "murabito",
    script =  "game_attackairf",
    category = ACMD_GAME,
	low_priority)]
unsafe fn toad_fair(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		frame(Frame=4)
		if(is_excute){
			WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
		}
		frame(Frame=16)
		if(is_excute){
			ATTACK(ID=0, Part=0, Bone=hash40("head"), Damage=15.0, Angle=361, KBG=97, FKB=0, BKB=30, Size=6.2, X=4.4, Y=-1.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_HEAD)
		}
		frame(Frame=19)
		if(is_excute){
			AttackModule::clear_all()
		}
		frame(Frame=40)
		if(is_excute){
			WorkModule::off_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
		}
    });
}
#[acmd_script(
    agent = "murabito",
    script =  "effect_attackairf",
    category = ACMD_EFFECT,
	low_priority)]
unsafe fn toad_fair_eff(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		frame(Frame=15)
		if(is_excute){
			EFFECT_FOLLOW_FLIP(hash40("sys_attack_arc_b"), hash40("sys_attack_arc_b"), hash40("top"), 0, 7, 4, -3, -11, -113, 1.1, true, EF_FLIP_YZ)
			LAST_EFFECT_SET_RATE(0.8)
		}
    });
}
#[acmd_script(
    agent = "murabito",
    script =  "sound_attackairf",
    category = ACMD_SOUND,
	low_priority)]
unsafe fn toad_fair_snd(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	frame(fighter.lua_state_agent, 15.0);
	if macros::is_excute(fighter) {
		attack_vc(fighter);
		macros::PLAY_SE(fighter, Hash40::new("se_murabito_swing_l"));
	};
}
#[acmd_script(
    agent = "murabito",
    script =  "effect_burst",
    category = ACMD_EFFECT,
	low_priority)]
unsafe fn toad_shell_burst_eff(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
}
#[acmd_script(
    agent = "murabito",
    script =  "game_attackdash",
    category = ACMD_GAME,
	low_priority)]
unsafe fn toad_da(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		frame(Frame=6)
		if(is_excute){
			ArticleModule::generate_article(FIGHTER_MURABITO_GENERATE_ARTICLE_FLOWERPOT, false, 0)
			sv_module_access::damage(MSC=MA_MSC_DAMAGE_DAMAGE_NO_REACTION, Type=DAMAGE_NO_REACTION_MODE_DAMAGE_POWER, DamageThreshold=7)
			ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=8.5, Angle=50, KBG=59, FKB=0, BKB=79, Size=6.5, X=0.0, Y=5.5, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=1, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_HEAD)
		}
		frame(Frame=10)
		if(is_excute){
			ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=6.0, Angle=60, KBG=69, FKB=0, BKB=74, Size=5.0, X=0.0, Y=5.5, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=1, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_KAMEHIT, Type=ATTACK_REGION_HEAD)
		}
		frame(Frame=30)
		if(is_excute){
			sv_module_access::damage(MSC=MA_MSC_DAMAGE_DAMAGE_NO_REACTION, Type=DAMAGE_NO_REACTION_MODE_NORMAL, DamageThreshold=0)
			AttackModule::clear_all()
			ArticleModule::remove_exist(FIGHTER_MURABITO_GENERATE_ARTICLE_FLOWERPOT,smash::app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL))
		}
    });
}
#[acmd_script(
    agent = "murabito",
    script =  "effect_attackdash",
    category = ACMD_EFFECT,
	low_priority)]
unsafe fn toad_da_eff(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		frame(Frame=6)
		if(is_excute){
			FOOT_EFFECT(hash40("sys_run_smoke"), hash40("top"), 2, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false)
		}
		frame(Frame=11)
		if(is_excute){
			FOOT_EFFECT(hash40("sys_run_smoke"), hash40("top"), 2, 0, 0, 0, 0, 0, 0.75, 0, 0, 0, 0, 0, 0, false)
		}
		frame(Frame=15)
		if(is_excute){
			FOOT_EFFECT(hash40("sys_run_smoke"), hash40("top"), 2, 0, 0, 0, 0, 0, 0.75, 0, 0, 0, 0, 0, 0, false)
		}
		frame(Frame=19)
		if(is_excute){
			FOOT_EFFECT(hash40("sys_run_smoke"), hash40("top"), 2, 0, 0, 0, 0, 0, 0.75, 0, 0, 0, 0, 0, 0, false)
		}
		frame(Frame=23)
		if(is_excute){
			FOOT_EFFECT(hash40("sys_run_smoke"), hash40("top"), 2, 0, 0, 0, 0, 0, 0.75, 0, 0, 0, 0, 0, 0, false)
		}
		frame(Frame=30)
		if(is_excute){
			LANDING_EFFECT(hash40("sys_down_smoke"), hash40("top"), 3, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false)
			EFFECT(hash40("sys_erace_smoke"), hash40("top"), 0, 5.5, 0, 0, 0, 0, 1.0, 0, 0, 0, 0, 0, 0, false)
		}
    });
}
#[acmd_script(
    agent = "murabito",
    script =  "sound_attackdash",
    category = ACMD_SOUND,
	low_priority)]
unsafe fn toad_da_snd(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		frame(Frame=5)
		for(5 Iterations){
			if(is_excute){
				PLAY_SE(hash40("se_common_step_sand"))
			}
			wait(Frames=1)
			if(is_excute){
				PLAY_SE(hash40("se_common_step_snow"))
				PLAY_SE(hash40("se_common_step_sand"))
			}
			wait(Frames=1)
			if(is_excute){
				PLAY_SE(hash40("se_common_step_snow"))
				PLAY_SE(hash40("se_common_step_sand"))
				PLAY_SE(hash40("se_common_step_sand"))
			}
			wait(Frames=1)
			if(is_excute){
				PLAY_SE(hash40("se_common_step_sand"))
				PLAY_SE(hash40("se_common_step_sand"))
			}
			wait(Frames=1)
			if(is_excute){
				PLAY_SE(hash40("se_common_step_snow"))
				PLAY_SE(hash40("se_common_step_snow"))
			}
			wait(Frames=1)
		}
    });
}
#[acmd_script(
    agent = "murabito",
    script =  "game_attackhi3",
    category = ACMD_GAME,
	low_priority)]
unsafe fn toad_utilt(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		frame(Frame=2)
		if(is_excute){
			ArticleModule::generate_article(FIGHTER_MURABITO_GENERATE_ARTICLE_UMBRELLA, false, 0)
			ArticleModule::change_motion(FIGHTER_MURABITO_GENERATE_ARTICLE_UMBRELLA,smash::phx::Hash40::new("attack_air_hi"),false,0.0)
		}
		frame(Frame=6)
		if(is_excute){
			ATTACK(ID=0, Part=0, Bone=hash40("head"), Damage=10.0, Angle=90, KBG=100, FKB=0, BKB=50, Size=7.5, X=4.4, Y=-1.0, Z=0.0, X=LUA_VOID, Y=LUA_VOID, Z=LUA_VOID, Hitlag=1.0, SDI=0.8, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_sting"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_PUNCH)
			ATTACK(ID=1, Part=0, Bone=hash40("legc"), Damage=6.5, Angle=90, KBG=100, FKB=0, BKB=50, Size=4.5, X=0.0, Y=0.0, Z=0.0, X=LUA_VOID, Y=LUA_VOID, Z=LUA_VOID, Hitlag=1.0, SDI=0.8, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_PUNCH)
		}
		frame(Frame=13)
		if(is_excute){
			AttackModule::clear_all()
		}
		frame(Frame=20)
		if(is_excute){
			ArticleModule::remove_exist(FIGHTER_MURABITO_GENERATE_ARTICLE_UMBRELLA,smash::app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL))
		}
    });
}	
#[acmd_script(
    agent = "murabito",
    script =  "sound_attackhi3",
    category = ACMD_SOUND,
	low_priority)]
unsafe fn toad_utilt_snd(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	frame(fighter.lua_state_agent, 5.0);
	if macros::is_excute(fighter) {
		attack_vc(fighter);
		macros::PLAY_SE(fighter, Hash40::new("se_murabito_swing_s"));
	};
}
#[acmd_script(
    agent = "murabito",
    script =  "effect_attackhi3",
    category = ACMD_EFFECT,
	low_priority)]
unsafe fn toad_utilt_eff(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		frame(Frame=6)
		if(is_excute){
			EFFECT_FOLLOW(hash40("sys_jump_smoke"), hash40("top"), 0, 0, 0, 0, 0, 0, 1, false)
			EFFECT_FOLLOW_FLIP_ALPHA(hash40("sys_attack_speedline"), hash40("sys_attack_speedline"), hash40("top"), 0, 4, 0, -90, 0, 0, 1.1, true, EF_FLIP_YZ, 0.3)
		}
		frame(Frame=24)
		if(is_excute){
			LANDING_EFFECT(hash40("sys_down_smoke"), hash40("top"), 3, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false)
		}
    });
}	
#[acmd_script(
    agent = "murabito",
    script =  "game_attackhi4",
    category = ACMD_GAME,
	low_priority)]
unsafe fn toad_usmash(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		frame(Frame=4)
		if(is_excute){
			WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD)
		}
		frame(Frame=5)
		if(is_excute){
			ArticleModule::generate_article(FIGHTER_MURABITO_GENERATE_ARTICLE_SLINGSHOT, true, 0)
		}
		frame(Frame=6)
		if(is_excute){
			ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=1.0, Angle=100, KBG=100, FKB=125, BKB=0, Size=3.5, X=0.0, Y=6.5, Z=8.0, X=LUA_VOID, Y=LUA_VOID, Z=LUA_VOID, Hitlag=1.0, SDI=0.8, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_PUNCH)
		}
		frame(Frame=7)
		if(is_excute){
			AttackModule::clear_all()
		}
		frame(Frame=11)
		if(is_excute){
			ATTACK(ID=0, Part=0, Bone=hash40("havel"), Damage=16.0, Angle=70, KBG=78, FKB=0, BKB=58, Size=10.0, X=0.0, Y=0.0, Z=0.0, X=LUA_VOID, Y=LUA_VOID, Z=LUA_VOID, Hitlag=1.0, SDI=0.8, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_PUNCH)
		}
		frame(Frame=13)
		if(is_excute){
			AttackModule::clear_all()
		}
		frame(Frame=14)
		if(is_excute){
			ArticleModule::remove_exist(FIGHTER_MURABITO_GENERATE_ARTICLE_SLINGSHOT,smash::app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL))
		}
    });
}	
#[acmd_script(
    agent = "murabito",
    script =  "effect_attackhi4",
    category = ACMD_EFFECT,
	low_priority)]
unsafe fn toad_usmash_eff(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		frame(Frame=13)
		if(is_excute){
			EFFECT(0x12de01e4e5u64, hash40("havel"), 0, 0, 0, 0, 0, 0, 0.75, 0, 0, 0, 0, 0, 0, false)
			EFFECT(0x12de01e4e5u64, hash40("havel"), 0, 0, 0, 270, 0, 0, 0.75, 0, 0, 0, 0, 0, 0, false)
		}
		frame(Frame=48)
		if(is_excute){
			LANDING_EFFECT(hash40("sys_down_smoke"), hash40("top"), 3, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false)
		}
    });
}	
#[acmd_script(
    agent = "murabito",
    script =  "sound_attackhi4",
    category = ACMD_SOUND,
	low_priority)]
unsafe fn toad_usmash_snd(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	frame(fighter.lua_state_agent, 8.0);
	if macros::is_excute(fighter) {
		attack_vc(fighter);
	}
	frame(fighter.lua_state_agent, 13.0);
	if macros::is_excute(fighter) {
		macros::PLAY_SE(fighter, Hash40::new("se_common_smashswing_01"));
		macros::PLAY_SE(fighter, Hash40::new("se_common_pitin_move_00"));
		macros::PLAY_SE(fighter, Hash40::new("se_common_landing_brick"));
	}
	frame(fighter.lua_state_agent, 14.0);
	if macros::is_excute(fighter) {
		macros::PLAY_SE(fighter, Hash40::new("se_murabito_special_h01"));
	}
	frame(fighter.lua_state_agent, 47.0);
	if macros::is_excute(fighter) {
		macros::PLAY_SE(fighter, Hash40::new("se_common_landing_soil"));
	}
}	
#[acmd_script(
    agent = "murabito_slingshot",
    scripts =  ["game_attackairf", "game_attackairb"],
    category = ACMD_GAME,
	low_priority)]
unsafe fn toad_usmash_coin(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		frame(Frame=2)
		if(is_excute){
			ArticleModule::generate_article(WEAPON_MURABITO_SLINGSHOT_GENERATE_ARTICLE_BULLET, true, 0)
			ArticleModule::shoot(WEAPON_MURABITO_SLINGSHOT_GENERATE_ARTICLE_BULLET, smash::app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_LAST), false)
		}
    });
}	
#[acmd_script(
    agent = "murabito_slingshot",
    scripts =  ["sound_attackairf", "sound_attackairb"],
    category = ACMD_SOUND,
	low_priority)]
unsafe fn toad_usmash_coin_snd(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
}	
#[acmd_script(
    agent = "murabito_slingshot",
    scripts =  ["effect_attackairf", "effect_attackairb"],
    category = ACMD_EFFECT,
	low_priority)]
unsafe fn toad_usmash_coin_eff(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
} 
#[acmd_script(
    agent = "murabito_bullet",
    scripts =  ["game_shootf", "game_shootb"],
    category = ACMD_GAME,
	low_priority)]
unsafe fn toad_usmash_coin_bullet(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		frame(Frame=5)
		if(is_excute){
			ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=5.0, Angle=90, KBG=100, FKB=0, BKB=55, Size=5.0, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.4, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=-3.5, Trip=0.0, Rehit=0, Reflectable=true, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_COIN, Type=ATTACK_REGION_OBJECT)
			AttackModule::enable_safe_pos()
		}
		wait(Frames=3)
		if(is_excute){
			ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=3.0, Angle=90, KBG=100, FKB=0, BKB=10, Size=1.2, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.4, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=-2, Trip=0.0, Rehit=0, Reflectable=true, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_COIN, Type=ATTACK_REGION_OBJECT)
		}
		wait(Frames=6)
		if(is_excute){
			ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=1.5, Angle=90, KBG=100, FKB=0, BKB=10, Size=1.2, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.4, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=-1, Trip=0.0, Rehit=0, Reflectable=true, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_COIN, Type=ATTACK_REGION_OBJECT)
		}
    });
}	
#[acmd_script(
    agent = "murabito_bullet",
    scripts =  ["sound_shootf", "sound_shootb"],
    category = ACMD_SOUND,
	low_priority)]
unsafe fn toad_usmash_coin_bullet_snd(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
}	
#[acmd_script(
    agent = "murabito_bullet",
    scripts =  ["effect_shootf", "effect_shootb"],
    category = ACMD_EFFECT,
	low_priority)]
unsafe fn toad_usmash_coin_bullet_eff(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
}	
#[acmd_script(
    agent = "murabito",
    script =  "game_attacks3",
    category = ACMD_GAME,
	low_priority)]
unsafe fn toad_ftilt(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		if(is_excute){
			ArticleModule::generate_article(FIGHTER_MURABITO_GENERATE_ARTICLE_UMBRELLA, false, 0)
			ArticleModule::change_motion(FIGHTER_MURABITO_GENERATE_ARTICLE_UMBRELLA,smash::phx::Hash40::new("attack_s3_s"),false,0.0)
		}
		frame(Frame=5)
		FT_MOTION_RATE(FSM=0.6)
		wait(Frames=10)
		FT_MOTION_RATE(FSM=1)
		wait(Frames=1)
		if(is_excute){
			ATTACK(ID=0, Part=0, Bone=hash40("haver"), Damage=11.0, Angle=38, KBG=76, FKB=0, BKB=48, Size=5.5, X=0.0, Y=10.5, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_OBJECT)
			ATTACK(ID=1, Part=0, Bone=hash40("haver"), Damage=11.0, Angle=38, KBG=76, FKB=0, BKB=48, Size=4.5, X=0.0, Y=7.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_OBJECT)
			ATTACK(ID=2, Part=0, Bone=hash40("haver"), Damage=11.0, Angle=38, KBG=76, FKB=0, BKB=48, Size=4.5, X=0.0, Y=3.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_OBJECT)
		}
		frame(Frame=18)
		if(is_excute){
			AttackModule::clear_all()
		}
		frame(Frame=46)
		if(is_excute){
			ArticleModule::remove_exist(FIGHTER_MURABITO_GENERATE_ARTICLE_UMBRELLA,smash::app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL))
		}
    });
}	
#[acmd_script(
    agent = "murabito",
    script =  "effect_attacks3",
    category = ACMD_EFFECT,
	low_priority)]
unsafe fn toad_ftilt_eff(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		frame(Frame=16)
		if(is_excute){
			EFFECT_FOLLOW_FLIP(hash40("sys_attack_arc_c"), hash40("sys_attack_arc_c"), hash40("top"), 1, 9, 4.5, 0, -50, -90, 1.2, true, EF_FLIP_YZ)
			LAST_EFFECT_SET_RATE(1.7)
		}
		frame(Frame=18)
		if(is_excute){
			LANDING_EFFECT(hash40("sys_down_smoke"), hash40("top"), 12, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false)
		}
    });
}	
#[acmd_script(
    agent = "murabito",
    script =  "sound_attacks3",
    category = ACMD_SOUND,
	low_priority)]
unsafe fn toad_ftilt_snd(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	frame(fighter.lua_state_agent, 15.0);
	if macros::is_excute(fighter) {
		attack_vc(fighter);
		macros::PLAY_SE(fighter, Hash40::new("se_murabito_swing_l"));
	};
}	
#[acmd_script(
    agent = "murabito",
    scripts =  ["game_specials", "game_specialairs"],
    category = ACMD_GAME,
	low_priority)]
unsafe fn toad_sideb(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
	KineticModule::suspend_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
}	
#[acmd_script(
    agent = "murabito",
    script =  "game_attacklw3",
    category = ACMD_GAME,
	low_priority)]
unsafe fn toad_dtilt(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		if(is_excute){
			ArticleModule::generate_article(FIGHTER_MURABITO_GENERATE_ARTICLE_UMBRELLA, false, 0)
			ArticleModule::change_motion(FIGHTER_MURABITO_GENERATE_ARTICLE_UMBRELLA,smash::phx::Hash40::new("attack_lw3"),false,0.0)
		}
		frame(Frame=7)
		if(is_excute){
			ATTACK(ID=0, Part=0, Bone=hash40("haver"), Damage=8.0, Angle=80, KBG=50, FKB=0, BKB=50, Size=3.3, X=0.0, Y=3.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_TAIL)
			ATTACK(ID=1, Part=0, Bone=hash40("haver"), Damage=8.0, Angle=80, KBG=50, FKB=0, BKB=50, Size=4.0, X=0.0, Y=6.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_TAIL)
			ATK_SET_SHIELD_SETOFF_MUL(ID=0, ShieldstunMul=0.4)
			ATK_SET_SHIELD_SETOFF_MUL(ID=1, ShieldstunMul=0.4)
		}
		wait(Frames=2)
		if(is_excute){
			AttackModule::clear_all()
		}
		frame(Frame=16)
		if(is_excute){
			ArticleModule::remove_exist(FIGHTER_MURABITO_GENERATE_ARTICLE_UMBRELLA,smash::app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL))
		}
    });
}	
#[acmd_script(
    agent = "murabito",
    script =  "effect_attacklw3",
    category = ACMD_EFFECT,
	low_priority)]
unsafe fn toad_dtilt_eff(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		
    });
}	
#[acmd_script(
    agent = "murabito",
    script =  "sound_attacklw3",
    category = ACMD_SOUND,
	low_priority)]
unsafe fn toad_dtilt_snd(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	frame(fighter.lua_state_agent, 6.0);
	if macros::is_excute(fighter) {
		attack_vc(fighter);
		macros::PLAY_SE(fighter, Hash40::new("se_murabito_smash_h01"));
	};
}	
#[acmd_script(
    agent = "murabito",
    script =  "expression_attacklw3",
    category = ACMD_EXPRESSION,
	low_priority)]
unsafe fn toad_dtilt_expr(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		if(is_excute){
			ItemModule::set_have_item_visibility(false, 0)
		}
		frame(Frame=40)
		if(is_excute){
			ItemModule::set_have_item_visibility(true, 0)
		}
    });
}	
#[acmd_script(
    agent = "murabito",
    scripts =  ["game_specialn", "game_specialairn"],
    category = ACMD_GAME,
	low_priority)]
unsafe fn toad_neutralb(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		frame(Frame=10)
		if(is_excute){
			rust {
				ItemModule::have_item(fighter.module_accessor, smash::app::ItemKind(*ITEM_KIND_WOOD), 0, 0, false, false);
			}
		}
    });
}	
#[acmd_script(
    agent = "murabito",
    scripts =  ["effect_specialn", "effect_specialairn"],
    category = ACMD_EFFECT,
	low_priority)]
unsafe fn toad_neutralb_eff(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		frame(Frame=9)
		if(is_excute){
			EFFECT(hash40("murabito_grass"), hash40("top"), 1, 0, 0.5, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false)
		}
    });
}	
#[acmd_script(
    agent = "murabito",
    scripts =  ["sound_specialn", "sound_specialairn"],
    category = ACMD_SOUND,
	low_priority)]
unsafe fn toad_neutralb_snd(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	frame(fighter.lua_state_agent, 9.0);
	if macros::is_excute(fighter) {
		macros::PLAY_SE(fighter, Hash40::new("se_murabito_special_h02"));
	};
}	
#[acmd_script(
    agent = "murabito",
    scripts =  ["game_catch"],
    category = ACMD_GAME,
	low_priority)]
unsafe fn toad_catch(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		frame(Frame=5)
		if(is_excute){
			GrabModule::set_rebound(CanCatchRebound=true)
		}
		frame(Frame=6)
		if(is_excute){
			CATCH(ID=0, Bone=hash40("top"), Size=3.3, X=0.0, Y=6.0, Z=4.0, X2=0.0, Y2=6.0, Z2=9.7, Status=FIGHTER_STATUS_KIND_CAPTURE_PULLED, Ground_or_Air=COLLISION_SITUATION_MASK_G)
			CATCH(ID=1, Bone=hash40("top"), Size=1.65, X=0.0, Y=6.0, Z=2.35, X2=0.0, Y2=6.0, Z2=11.35, Status=FIGHTER_STATUS_KIND_CAPTURE_PULLED, Ground_or_Air=COLLISION_SITUATION_MASK_A)
		}
		rust{
			macros::game_CaptureCutCommon(fighter);
		}
		wait(Frames=2)
		if(is_excute){
			sv_module_access::grab(MA_MSC_CMD_GRAB_CLEAR_ALL)
			WorkModule::on_flag(Flag=FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT)
			GrabModule::set_rebound(CanCatchRebound=false)
		}
		frame(Frame=40)
		if(is_excute){
			ArticleModule::remove_exist(FIGHTER_MURABITO_GENERATE_ARTICLE_BUTTERFLYNET,smash::app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL))
		}
    });
}	
#[acmd_script(
    agent = "murabito",
    scripts =  ["sound_catch"],
    category = ACMD_SOUND,
	low_priority)]
unsafe fn toad_catch_snd(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
	frame(fighter.lua_state_agent, 5.0);
	if macros::is_excute(fighter) {
		attack_vc(fighter);
		macros::PLAY_SE(fighter, Hash40::new("se_murabito_swing_s"));
	};
}	
#[acmd_script(
    agent = "murabito",
    scripts =  ["game_catchdash"],
    category = ACMD_GAME,
	low_priority)]
unsafe fn toad_catchdash(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		frame(Frame=8)
		if(is_excute){
			GrabModule::set_rebound(CanCatchRebound=true)
		}
		frame(Frame=9)
		if(is_excute){
			CATCH(ID=0, Bone=hash40("top"), Size=3.3, X=0.0, Y=6.0, Z=4.0, X2=0.0, Y2=6.0, Z2=10.7, Status=FIGHTER_STATUS_KIND_CAPTURE_PULLED, Ground_or_Air=COLLISION_SITUATION_MASK_G)
			CATCH(ID=1, Bone=hash40("top"), Size=1.65, X=0.0, Y=6.0, Z=2.35, X2=0.0, Y2=6.0, Z2=12.35, Status=FIGHTER_STATUS_KIND_CAPTURE_PULLED, Ground_or_Air=COLLISION_SITUATION_MASK_A)
		}
		rust{
			macros::game_CaptureCutCommon(fighter);
		}
		wait(Frames=2)
		if(is_excute){
			sv_module_access::grab(MA_MSC_CMD_GRAB_CLEAR_ALL)
			WorkModule::on_flag(Flag=FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT)
			GrabModule::set_rebound(CanCatchRebound=false)
		}
		frame(Frame=40)
		if(is_excute){
			ArticleModule::remove_exist(FIGHTER_MURABITO_GENERATE_ARTICLE_BUTTERFLYNET,smash::app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL))
		}
    });
}	
#[acmd_script(
    agent = "murabito",
    scripts =  ["sound_catchdash"],
    category = ACMD_SOUND,
	low_priority)]
unsafe fn toad_catchdash_snd(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
	frame(fighter.lua_state_agent, 8.0);
	if macros::is_excute(fighter) {
		attack_vc(fighter);
		macros::PLAY_SE(fighter, Hash40::new("se_murabito_swing_s"));
	};
}	
#[acmd_script(
    agent = "murabito",
    scripts =  ["game_catchturn"],
    category = ACMD_GAME,
	low_priority)]
unsafe fn toad_catchturn(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		frame(Frame=9)
		if(is_excute){
			GrabModule::set_rebound(CanCatchRebound=true)
		}
		frame(Frame=10)
		if(is_excute){
			CATCH(ID=0, Bone=hash40("top"), Size=3.3, X=0.0, Y=6.0, Z=-4.0, X2=0.0, Y2=6.0, Z2=-15.7, Status=FIGHTER_STATUS_KIND_CAPTURE_PULLED, Ground_or_Air=COLLISION_SITUATION_MASK_G)
			CATCH(ID=1, Bone=hash40("top"), Size=1.65, X=0.0, Y=6.0, Z=-2.35, X2=0.0, Y2=6.0, Z2=-17.35, Status=FIGHTER_STATUS_KIND_CAPTURE_PULLED, Ground_or_Air=COLLISION_SITUATION_MASK_A)
		}
		rust{
			macros::game_CaptureCutCommon(fighter);
		}
		wait(Frames=2)
		if(is_excute){
			sv_module_access::grab(MA_MSC_CMD_GRAB_CLEAR_ALL)
			WorkModule::on_flag(Flag=FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT)
			GrabModule::set_rebound(CanCatchRebound=false)
		}
		frame(Frame=40)
		if(is_excute){
			ArticleModule::remove_exist(FIGHTER_MURABITO_GENERATE_ARTICLE_BUTTERFLYNET,smash::app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL))
		}
    });
}	
#[acmd_script(
    agent = "murabito",
    scripts =  ["sound_catchturn"],
    category = ACMD_SOUND,
	low_priority)]
unsafe fn toad_catchturn_snd(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
	frame(fighter.lua_state_agent, 9.0);
	if macros::is_excute(fighter) {
		attack_vc(fighter);
		macros::PLAY_SE(fighter, Hash40::new("se_murabito_swing_s"));
	};
}	
#[acmd_script(
    agent = "murabito",
    scripts =  ["game_catchattack"],
    category = ACMD_GAME,
	low_priority)]
unsafe fn toad_catchattack(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		frame(Frame=1)
		if(is_excute){
			ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=1.5, Angle=361, KBG=100, FKB=30, BKB=0, Size=5.0, X=0.0, Y=6.0, Z=10.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=2.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_HEAD)
			AttackModule::set_catch_only_all(true, false)
		}
		wait(Frames=1)
		if(is_excute){
			AttackModule::clear_all()
		}
    });
}
#[acmd_script(
    agent = "murabito",
    scripts =  ["game_attackairlw"],
    category = ACMD_GAME,
	low_priority)]
unsafe fn toad_dair(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		if(is_excute){
			WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_LANDING_CLEAR_SPEED)
			WorkModule::on_flag(Flag=FIGHTER_INSTANCE_WORK_ID_FLAG_NO_SPEED_OPERATION_CHK)
			SET_SPEED_EX(0, 1.5, KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN)
			WorkModule::off_flag(Flag=FIGHTER_INSTANCE_WORK_ID_FLAG_NO_SPEED_OPERATION_CHK)
			KineticModule::suspend_energy(FIGHTER_KINETIC_ENERGY_ID_CONTROL)
		}
		frame(Frame=3)
		if(is_excute){
			WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
		}
		frame(Frame=20)
		if(is_excute){
			WorkModule::on_flag(Flag=FIGHTER_INSTANCE_WORK_ID_FLAG_NO_SPEED_OPERATION_CHK)
			SET_SPEED_EX(0, -4, KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN)
			WorkModule::off_flag(Flag=FIGHTER_INSTANCE_WORK_ID_FLAG_NO_SPEED_OPERATION_CHK)
			ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=12.0, Angle=280, KBG=90, FKB=0, BKB=30, Size=6.0, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_BODY)
			ATTACK(ID=1, Part=0, Bone=hash40("kneer"), Damage=12.0, Angle=280, KBG=90, FKB=0, BKB=30, Size=5.0, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_BODY)
			ATTACK(ID=2, Part=0, Bone=hash40("kneel"), Damage=12.0, Angle=280, KBG=90, FKB=0, BKB=30, Size=5.0, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_BODY)
		}
		frame(Frame=26)
		if(is_excute){
			ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=12.0, Angle=60, KBG=90, FKB=0, BKB=30, Size=6.0, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_BODY)
			ATTACK(ID=1, Part=0, Bone=hash40("kneer"), Damage=12.0, Angle=60, KBG=90, FKB=0, BKB=30, Size=5.0, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_BODY)
			ATTACK(ID=2, Part=0, Bone=hash40("kneel"), Damage=12.0, Angle=60, KBG=90, FKB=0, BKB=30, Size=5.0, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_BODY)
		}
		frame(Frame=37)
		if(is_excute){
			AttackModule::clear_all()
		}
		frame(Frame=48)
		if(is_excute){
			WorkModule::off_flag(Flag=FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_GRAVITY_STABLE_UNABLE)
			KineticModule::resume_energy(FIGHTER_KINETIC_ENERGY_ID_CONTROL)
		}
		frame(Frame=53)
		if(is_excute){
			WorkModule::off_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
		}
    });
}
#[acmd_script(
    agent = "murabito",
    scripts =  ["effect_attackairlw"],
    category = ACMD_EFFECT,
	low_priority)]
unsafe fn toad_dair_eff(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		if(is_excute){
			EFFECT(hash40("sys_smash_flash"), hash40("haver"), -2, 5, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, true)
		}
    });
}
#[acmd_script(
    agent = "murabito",
    scripts =  ["sound_attackairlw"],
    category = ACMD_SOUND,
	low_priority)]
unsafe fn toad_dair_snd(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		if(is_excute){
			PLAY_SE(hash40("se_murabito_attackdash03"))
		}
    });
}
#[acmd_script(
    agent = "murabito",
    scripts =  ["game_landingairlw"],
    category = ACMD_GAME,
	low_priority)]
unsafe fn toad_land_dair(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		frame(Frame=1)
		if(is_excute){
			QUAKE(CAMERA_QUAKE_KIND_M)
			ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=5.5, Angle=60, KBG=110, FKB=0, BKB=70, Size=9.0, X=0.0, Y=7.0, Z=4.0, X2=0.0, Y2=7.0, Z2=-4.5, Hitlag=0.3, SDI=0.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
		}
		frame(Frame=3)
		if(is_excute){
			AttackModule::clear_all()
		}
    });
}
#[acmd_script(
    agent = "murabito",
    scripts =  ["sound_landingairlw"],
    category = ACMD_SOUND,
	low_priority)]
unsafe fn toad_land_dair_snd(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		if(is_excute){
			PLAY_SE(hash40("se_murabito_attackhard_l01"))
		}
    });
}
#[acmd_script(
    agent = "murabito",
    script =  "game_attacklw4",
    category = ACMD_GAME,
	low_priority)]
unsafe fn toad_dsmash(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		frame(Frame=5)
		if(is_excute){
			ArticleModule::generate_article(FIGHTER_MURABITO_GENERATE_ARTICLE_FIREWORK, false, 0)
			WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD)
		}
		frame(Frame=21)
		if(is_excute){
			ATTACK(ID=0, Part=0, Bone=hash40("throw"), Damage=10.0, Angle=270, KBG=100, FKB=0, BKB=40, Size=10.0, X=0.0, Y=8.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=5, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_A, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_HEAVY, Type=ATTACK_REGION_KICK)
			ATTACK(ID=1, Part=0, Bone=hash40("throw"), Damage=10.0, Angle=270, KBG=100, FKB=0, BKB=40, Size=10.0, X=0.0, Y=8.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=5, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_G, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_bury"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_HEAVY, Type=ATTACK_REGION_KICK)
		}
		frame(Frame=31)
		if(is_excute){
			AttackModule::clear_all()
			QUAKE(CAMERA_QUAKE_KIND_L)
			ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=19.0, Angle=30, KBG=88, FKB=0, BKB=35, Size=7.0, X=0.0, Y=3.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.1, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_HEAVY, Type=ATTACK_REGION_KICK)
			ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=19.0, Angle=30, KBG=88, FKB=0, BKB=35, Size=7.0, X=0.0, Y=3.0, Z=9.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.1, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_HEAVY, Type=ATTACK_REGION_KICK)
			ATTACK(ID=2, Part=0, Bone=hash40("top"), Damage=19.0, Angle=30, KBG=88, FKB=0, BKB=35, Size=7.0, X=0.0, Y=3.0, Z=-9.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.1, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_HEAVY, Type=ATTACK_REGION_KICK)
		}
		frame(Frame=33)
		if(is_excute){
			AttackModule::clear_all()
		}
		frame(Frame=53)
		if(is_excute){
			ArticleModule::remove_exist(FIGHTER_MURABITO_GENERATE_ARTICLE_FIREWORK,smash::app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL))
		}
    });
}	
#[acmd_script(
    agent = "murabito",
    script =  "effect_attacklw4",
    category = ACMD_EFFECT,
	low_priority)]
unsafe fn toad_dsmash_eff(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		frame(Frame=6)
		if(is_excute){
			EFFECT_FOLLOW(hash40("sys_jump_smoke"), hash40("top"), 0, 0, 0, 0, 0, 0, 1, true)
		}
		frame(Frame=31)
		if(is_excute){
			LANDING_EFFECT(hash40("sys_down_smoke"), hash40("top"), 0, 0, 0, 0, 0, 0, 1.0, 0, 0, 0, 0, 0, 0, false)
		}
		frame(Frame=52)
		if(is_excute){
			EFFECT(hash40("sys_erace_smoke"), hash40("throw"), 0, 8.0, 0, 0, 0, 0, 2.0, 0, 0, 0, 0, 0, 0, false)
		}
    });
}	
#[acmd_script(
    agent = "murabito",
    script =  "sound_attacklw4",
    category = ACMD_SOUND,
	low_priority)]
unsafe fn toad_dsmash_snd(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	frame(fighter.lua_state_agent, 7.0);
	if macros::is_excute(fighter) {
		macros::PLAY_SE(fighter, Hash40::new("se_murabito_attackair_l02"));
	};
	frame(fighter.lua_state_agent, 29.0);
	if macros::is_excute(fighter) {
		macros::PLAY_SE(fighter, Hash40::new("se_murabito_smash_l02"));
	};
	frame(fighter.lua_state_agent, 31.0);
	if macros::is_excute(fighter) {
		dmg_vc(fighter);
		macros::PLAY_SE(fighter, Hash40::new("se_murabito_smash_h02"));
	};
}	
#[acmd_script(
    agent = "murabito",
    script =  "game_attacklw4charge",
    category = ACMD_GAME,
	low_priority)]
unsafe fn toad_dsmash_charge(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
	
	});
}
#[acmd_script(
    agent = "murabito",
    script =  "effect_attacklw4charge",
    category = ACMD_EFFECT,
	low_priority)]
unsafe fn toad_dsmash_charge_eff(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		frame(Frame=1)
		if(is_excute){
			FOOT_EFFECT(hash40("sys_run_smoke"), hash40("top"), 1, 0, -2, 0, 0, 0, 1, 15, 0, 4, 0, 0, 0, true)
		}
		wait(Frames=5)
		EFFECT(hash40("sys_smash_flash_s"), hash40("haver"), 0, 0, 0, 0, 0, 0, 1, 4, 4, 4, 0, 0, 0, true)
    });
}
#[acmd_script(
    agent = "murabito",
    script =  "expression_attacklw4charge",
    category = ACMD_EXPRESSION,
	low_priority)]
unsafe fn toad_dsmash_charge_expr(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		
    });
}
#[acmd_script(
    agent = "murabito",
    script =  "game_specialairhidetach",
    category = ACMD_GAME,
	low_priority)]
unsafe fn toad_upb(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		if(is_excute){
			ArticleModule::generate_article(FIGHTER_MURABITO_GENERATE_ARTICLE_HELMET, false, 0)
			KineticModule::clear_speed_all()
		}
		frame(Frame=5)
		if(is_excute){
			rust {
				let x_speed = PostureModule::lr(boma) * ControlModule::get_stick_x(boma) * 0.75;
				macros::SET_SPEED_EX(fighter, x_speed, 3.2, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
			}
			ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=12.0, Angle=89, KBG=71, FKB=0, BKB=74, Size=6.7, X=0.0, Y=16.0, Z=-3.0, X2=0.0, Y2=16.0, Z2=3.0, Hitlag=1.7, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_PUNCH)
		}
		frame(Frame=8)
		if(is_excute){
			ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=8.0, Angle=89, KBG=71, FKB=0, BKB=74, Size=5.7, X=0.0, Y=16.0, Z=-3.0, X2=0.0, Y2=16.0, Z2=3.0, Hitlag=1.7, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_PUNCH)
		}
		frame(Frame=13)
		if(is_excute){
			sv_battle_object::notify_event_msc_cmd(0x2127e37c07u64, GROUND_CLIFF_CHECK_KIND_ALWAYS)
			rust{
				notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS);
			}
		}
		frame(Frame=34)
		if(is_excute){
			AttackModule::clear_all()
		}
	});
}
#[acmd_script(
    agent = "murabito",
    script =  "sound_specialairhidetach",
    category = ACMD_SOUND,
	low_priority)]
unsafe fn toad_upb_snd(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		frame(Frame=4)
		if(is_excute){
			PLAY_SE(hash40("se_murabito_smash_s01"))
			PLAY_SE(hash40("se_murabito_smash_s02"))
		}
	});
}
#[acmd_script(
    agent = "murabito",
    script =  "effect_specialairhidetach",
    category = ACMD_EFFECT,
	low_priority)]
unsafe fn toad_upb_eff(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		frame(Frame=4)
		if(is_excute){
			LANDING_EFFECT(hash40("sys_landing_smoke"), hash40("top"), 0, 0, 0, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, false)
		}
	});
}
#[acmd_script(
    agent = "murabito",
    script =  "game_throwlw",
    category = ACMD_GAME,
	low_priority)]
unsafe fn toad_dthrow(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		if(is_excute){
			FT_LEAVE_NEAR_OTTOTTO(-1, 1)
			ATTACK_ABS(Kind=FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, ID=0, Damage=5.0, Angle=45, KBG=90, FKB=0, BKB=65, Hitlag=0.0, Unk=1.0, FacingRestrict=ATTACK_LR_CHECK_F, Unk=0.0, Unk=true, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_NONE, Type=ATTACK_REGION_THROW)
			ATTACK_ABS(Kind=FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, ID=0, Damage=3.0, Angle=361, KBG=100, FKB=0, BKB=60, Hitlag=0.0, Unk=1.0, FacingRestrict=ATTACK_LR_CHECK_F, Unk=0.0, Unk=true, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_NONE, Type=ATTACK_REGION_THROW)
		}
		frame(Frame=18)
		if(is_excute){
			ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=2.0, Angle=361, KBG=100, FKB=0, BKB=30, Size=5.0, X=0.0, Y=3.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_NONE, Type=ATTACK_REGION_HIP)
			AttackModule::set_catch_only_all(true, false)
		}
		frame(Frame=18)
		if(is_excute){
			CHECK_FINISH_CAMERA(0, 0)
		}
		frame(Frame=29)
		if(is_excute){
			ATK_HIT_ABS(FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, hash40("throw"), WorkModule::get_int64(module_accessor,*FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT), WorkModule::get_int64(module_accessor,*FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP), WorkModule::get_int64(module_accessor,*FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO))
			AttackModule::clear_all()
		}
	});
}
#[acmd_script(
    agent = "murabito",
    script =  "game_throwb",
    category = ACMD_GAME,
	low_priority)]
unsafe fn toad_bthrow(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		if(is_excute){
			ATTACK_ABS(Kind=FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, ID=0, Damage=11.0, Angle=45, KBG=66, FKB=0, BKB=70, Hitlag=0.0, Unk=1.0, FacingRestrict=ATTACK_LR_CHECK_F, Unk=0.0, Unk=true, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_NONE, Type=ATTACK_REGION_THROW)
			ATTACK_ABS(Kind=FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, ID=0, Damage=3.0, Angle=361, KBG=100, FKB=0, BKB=40, Hitlag=0.0, Unk=1.0, FacingRestrict=ATTACK_LR_CHECK_F, Unk=0.0, Unk=true, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_NONE, Type=ATTACK_REGION_THROW)
		}
		frame(Frame=13)
		if(is_excute){
			ATTACK_IGNORE_THROW(ID=0, Part=0, Bone=hash40("hip"), Damage=8.0, Angle=50, KBG=70, FKB=0, BKB=100, Size=4.0, X=-1.0, Y=6.0, Z=0.0, X2=-3.2, Y2=13.0, Z2=0.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_THROW)
		}
		frame(Frame=33)
		if(is_excute){
			AttackModule::clear_all()
			REVERSE_LR()
		}
		frame(Frame=34)
		if(is_excute){
			ATK_HIT_ABS(FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, hash40("throw"), WorkModule::get_int64(module_accessor,*FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT), WorkModule::get_int64(module_accessor,*FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP), WorkModule::get_int64(module_accessor,*FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO))
		}
	});
}
#[acmd_script(
    agent = "murabito",
    script =  "game_throwf",
    category = ACMD_GAME,
	low_priority)]
unsafe fn toad_fthrow(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		if(is_excute){
			ATTACK_ABS(Kind=FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, ID=0,  Damage=8.0, Angle=35, KBG=5, FKB=0, BKB=80, Hitlag=0.0, Unk=1.0, FacingRestrict=ATTACK_LR_CHECK_F, Unk=0.0, Unk=true, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_NONE, Type=ATTACK_REGION_THROW)
			ATTACK_ABS(Kind=FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, ID=0, Damage=3.0, Angle=361, KBG=100, FKB=0, BKB=40, Hitlag=0.0, Unk=1.0, FacingRestrict=ATTACK_LR_CHECK_F, Unk=0.0, Unk=true, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_NONE, Type=ATTACK_REGION_THROW)
		}
		frame(Frame=28)
		if(is_excute){
			WorkModule::on_flag(Flag=FIGHTER_STATUS_THROW_FLAG_START_AIR)
		}
		frame(Frame=54)
		if(is_excute){
			WorkModule::on_flag(Flag=FIGHTER_STATUS_THROW_FLAG_STOP)
			ATK_HIT_ABS(FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, hash40("throw"), WorkModule::get_int64(module_accessor,*FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT), WorkModule::get_int64(module_accessor,*FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP), WorkModule::get_int64(module_accessor,*FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO))
		}
	});
}
#[acmd_script(
    agent = "murabito",
    script =  "sound_throwf",
    category = ACMD_SOUND,
	low_priority)]
unsafe fn toad_fthrow_snd(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		frame(Frame=30)
		if(is_excute){
			PLAY_SE(hash40("se_murabito_jump01"))
		}
		frame(Frame=54)
		if(is_excute){
			PLAY_SE(hash40("se_common_throw_02"))
			PLAY_SE(hash40("se_murabito_attackair_f02"))
		}
		frame(Frame=56)
		if(is_excute){
			PLAY_SE(hash40("se_murabito_attackdash02"))
		}
	});
}
#[acmd_script(
    agent = "murabito",
    scripts =  ["game_specialairlw2", "game_speciallw2"],
    category = ACMD_GAME,
	low_priority)]
unsafe fn toad_downb(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		frame(Frame=8)
		if(is_excute){
			ATTACK(ID=0, Part=0, Bone=hash40("hip"), Damage=8.0, Angle=70, KBG=100, FKB=0, BKB=20, Size=3.3, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
			ATTACK(ID=1, Part=0, Bone=hash40("kneer"), Damage=8.0, Angle=70, KBG=100, FKB=0, BKB=20, Size=5.2, X=4.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
		}
		frame(Frame=55)
		if(is_excute){
			AttackModule::clear_all()
		}
	});
}
#[acmd_script(
    agent = "murabito",
    scripts =  ["sound_specialairlw2", "sound_speciallw2"],
    category = ACMD_SOUND,
	low_priority)]
unsafe fn toad_downb_snd(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
}
#[acmd_script(
    agent = "murabito",
    scripts =  ["effect_specialairlw2", "effect_speciallw2"],
    category = ACMD_EFFECT,
	low_priority)]
unsafe fn toad_downb_eff(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		frame(Frame=7)
		if(is_excute){
			EFFECT_FOLLOW_FLIP(hash40("sys_spin_wind"), hash40("sys_spin_wind"), hash40("top"), 0, 7, 0, 0, 0, 0, 0.5, true, EF_FLIP_YZ)
			EFFECT_FOLLOW_FLIP(hash40("sys_spin_wind"), hash40("sys_spin_wind"), hash40("top"), 0, 2.5, 0, 0, 0, 0, 0.5, true, EF_FLIP_YZ)
			EFFECT_FOLLOW_FLIP(hash40("sys_spin_wind"), hash40("sys_spin_wind"), hash40("top"), 0, 4, 0, 0, 0, 0, 0.5, true, EF_FLIP_YZ)
		}
		frame(Frame=20)
		if(is_excute){
			EFFECT_FOLLOW_FLIP(hash40("sys_spin_wind"), hash40("sys_spin_wind"), hash40("top"), 0, 7, 0, 0, 0, 0, 0.5, true, EF_FLIP_YZ)
			EFFECT_FOLLOW_FLIP(hash40("sys_spin_wind"), hash40("sys_spin_wind"), hash40("top"), 0, 2.5, 0, 0, 0, 0, 0.5, true, EF_FLIP_YZ)
			EFFECT_FOLLOW_FLIP(hash40("sys_spin_wind"), hash40("sys_spin_wind"), hash40("top"), 0, 4, 0, 0, 0, 0, 0.5, true, EF_FLIP_YZ)
		}
		frame(Frame=33)
		if(is_excute){
			EFFECT_FOLLOW_FLIP(hash40("sys_spin_wind"), hash40("sys_spin_wind"), hash40("top"), 0, 7, 0, 0, 0, 0, 0.5, true, EF_FLIP_YZ)
			EFFECT_FOLLOW_FLIP(hash40("sys_spin_wind"), hash40("sys_spin_wind"), hash40("top"), 0, 2.5, 0, 0, 0, 0, 0.5, true, EF_FLIP_YZ)
			EFFECT_FOLLOW_FLIP(hash40("sys_spin_wind"), hash40("sys_spin_wind"), hash40("top"), 0, 4, 0, 0, 0, 0, 0.5, true, EF_FLIP_YZ)
		}
		frame(Frame=46)
		if(is_excute){
			EFFECT_FOLLOW_FLIP(hash40("sys_spin_wind"), hash40("sys_spin_wind"), hash40("top"), 0, 7, 0, 0, 0, 0, 0.5, true, EF_FLIP_YZ)
			EFFECT_FOLLOW_FLIP(hash40("sys_spin_wind"), hash40("sys_spin_wind"), hash40("top"), 0, 2.5, 0, 0, 0, 0, 0.5, true, EF_FLIP_YZ)
			EFFECT_FOLLOW_FLIP(hash40("sys_spin_wind"), hash40("sys_spin_wind"), hash40("top"), 0, 4, 0, 0, 0, 0, 0.5, true, EF_FLIP_YZ)
		}
	});
}
#[acmd_script(
    agent = "murabito",
    scripts =  ["game_specialairlw1failure"],
    category = ACMD_GAME,
	low_priority)]
unsafe fn toad_downb_air(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
	let ENTRY_ID = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    acmd!(lua_state, {
		frame(Frame=5)
		if(is_excute){
			rust{
				if !HAS_DOWNB[ENTRY_ID] {
					macros::SET_SPEED_EX(fighter, SPEED_X[ENTRY_ID]*PostureModule::lr(fighter.module_accessor), 0.6, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
					HAS_DOWNB[ENTRY_ID] = true;
				};
			}
			ATTACK(ID=0, Part=0, Bone=hash40("handr"), Damage=1.0, Angle=361, KBG=100, FKB=80, BKB=0, Size=3.0, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_turn"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_KAMEHIT, Type=ATTACK_REGION_PUNCH)
			ATTACK(ID=1, Part=0, Bone=hash40("handl"), Damage=1.0, Angle=361, KBG=100, FKB=80, BKB=0, Size=3.0, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_turn"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_KAMEHIT, Type=ATTACK_REGION_PUNCH)
		}
		frame(Frame=10)
		if(is_excute){
			AttackModule::clear_all()
		}
	});
}
#[acmd_script(
    agent = "murabito",
    scripts =  ["sound_specialairlw1failure"],
    category = ACMD_SOUND,
	low_priority)]
unsafe fn toad_downb_air_snd(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
	let ENTRY_ID = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    acmd!(lua_state, {
		frame(Frame=5)
		if(is_excute){
			PLAY_SE(hash40("se_murabito_attackair_n01"))
		}
	});
}
#[acmd_script(
    agent = "murabito",
    scripts =  ["effect_specialairlw1failure"],
    category = ACMD_EFFECT,
	low_priority)]
unsafe fn toad_downb_air_eff(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		frame(Frame=4)
		if(is_excute){
			EFFECT_FOLLOW_FLIP(hash40("sys_spin_wind"), hash40("sys_spin_wind"), hash40("top"), 0, 5, 0, 0, 0, 0, 0.75, true, EF_FLIP_YZ)
		}
	});
}
#[acmd_script(
    agent = "murabito",
    scripts =  ["sound_damagehi1", "sound_damagehi2", "sound_damagehi3", "sound_damagen1", "sound_damagen2", "sound_damagen3", "sound_damagelw1", "sound_damagelw2", "sound_damagelw3", "sound_damageair1", "sound_damageair2", "sound_damageair3", "sound_damageelec"],
    category = ACMD_SOUND,
	low_priority)]
unsafe fn toad_dmg_snd(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	if macros::is_excute(fighter) {
		dmg_vc(fighter);
	};
}	
#[acmd_script(
    agent = "murabito",
    scripts =  ["sound_damageflyhi", "sound_damageflyn", "sound_damageflylw", "sound_damageflytop", "sound_damageflytop", "sound_damageflyroll", "sound_damageflymeteor"],
    category = ACMD_SOUND,
	low_priority)]
unsafe fn toad_dmg_fly_snd(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	if macros::is_excute(fighter) {
		dmg_fly_vc(fighter);
	};
}	
#[fighter_frame_callback]
pub fn toad(fighter : &mut L2CFighterCommon) {
    unsafe {
        let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent); 
		let status_kind = smash::app::lua_bind::StatusModule::status_kind(boma);
		let ENTRY_ID = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
		let motion_kind = MotionModule::motion_kind(boma);
		let frame = MotionModule::frame(boma);
		let fighter_kind = smash::app::utility::get_kind(boma);
		let situation_kind = StatusModule::situation_kind(boma);
		let end_frame = MotionModule::end_frame(boma);
		if fighter_kind == *FIGHTER_KIND_MURABITO {
			let scale = smash::phx::Vector3f { x: 0.8, y: 1.0, z: 1.0 };
			ModelModule::set_joint_scale(boma, Hash40::new("shoulderl"), &scale);
			ModelModule::set_joint_scale(boma, Hash40::new("shoulderr"), &scale);
			ModelModule::set_mesh_visibility(fighter.module_accessor,Hash40::new("murabito_glove_l"),false);
			ModelModule::set_mesh_visibility(fighter.module_accessor,Hash40::new("murabito_glove_r"),false);
			ModelModule::set_mesh_visibility(fighter.module_accessor,Hash40::new("murabito_turnip_1"),false);
			ModelModule::set_mesh_visibility(fighter.module_accessor,Hash40::new("murabito_turnip_2"),false);
			ModelModule::set_mesh_visibility(fighter.module_accessor,Hash40::new("murabito_turnip_3"),false);
			ModelModule::set_mesh_visibility(fighter.module_accessor,Hash40::new("murabito_turnip_1"),false);
			ModelModule::set_mesh_visibility(fighter.module_accessor,Hash40::new("murabito_turnipFLIP_2"),false);
			ModelModule::set_mesh_visibility(fighter.module_accessor,Hash40::new("murabito_turnipFLIP_3"),false);
			ModelModule::set_mesh_visibility(fighter.module_accessor,Hash40::new("murabito_handpa_l"),true);
			ModelModule::set_mesh_visibility(fighter.module_accessor,Hash40::new("murabito_handpa_r"),true);
			WorkModule::set_int(boma, 1, *FIGHTER_MURABITO_INSTANCE_WORK_ID_INT_SPECIAL_N_TIME_LIMIT);
			if ![*FIGHTER_STATUS_KIND_ATTACK_HI3, *FIGHTER_STATUS_KIND_ATTACK_S3, *FIGHTER_STATUS_KIND_ATTACK_LW3].contains(&status_kind) {
				ArticleModule::remove_exist(boma, *FIGHTER_MURABITO_GENERATE_ARTICLE_UMBRELLA,smash::app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
			};
			if ![*FIGHTER_STATUS_KIND_ATTACK_LW4_HOLD, *FIGHTER_STATUS_KIND_ATTACK_LW4_START, *FIGHTER_STATUS_KIND_ATTACK_LW4].contains(&status_kind) {
				ArticleModule::remove_exist(boma, *FIGHTER_MURABITO_GENERATE_ARTICLE_FIREWORK,smash::app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
			};
			if ![*FIGHTER_MURABITO_STATUS_KIND_SPECIAL_HI_DETACH].contains(&status_kind) {
				ArticleModule::remove_exist(boma, *FIGHTER_MURABITO_GENERATE_ARTICLE_HELMET,smash::app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
			};
			if [*FIGHTER_MURABITO_STATUS_KIND_SPECIAL_LW_PLANT].contains(&status_kind) {
				StatusModule::change_status_request_from_script(boma, *FIGHTER_MURABITO_STATUS_KIND_SPECIAL_LW_WATER_JUMP, true);
			};
			if [*FIGHTER_MURABITO_STATUS_KIND_SPECIAL_LW_PLANT_FAIL].contains(&status_kind) && situation_kind == *SITUATION_KIND_GROUND {
				if frame < 2.0 {
					StatusModule::change_status_request_from_script(boma, *FIGHTER_MURABITO_STATUS_KIND_SPECIAL_LW_WATER_JUMP, true);
				} else {
					StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_LANDING, true);
				};
			};
			if [*FIGHTER_MURABITO_STATUS_KIND_SPECIAL_LW_WATER_JUMP].contains(&status_kind) {
				if frame >= 66.0 {
					StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_FALL, true);
				};
				if frame > 30.0 && situation_kind == *SITUATION_KIND_GROUND {
					StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_LANDING, true);
				}
				if AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT) && !AttackModule::is_infliction(boma, *COLLISION_KIND_MASK_HIT) && frame < 50.0{
					macros::SET_SPEED_EX(fighter, SPEED_X[ENTRY_ID]*PostureModule::lr(boma), 1.0, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
					MotionModule::change_motion(boma, smash::phx::Hash40::new("special_air_lw_water"), 51.0, 1.0, false, 0.0, false, false);
				};
				if frame > 5.0 && frame < 7.0{
					macros::PLAY_SE(fighter, Hash40::new("se_murabito_attackdash01"));
				}
			};
			if [*FIGHTER_MURABITO_STATUS_KIND_SPECIAL_LW_WATER_LANDING, *FIGHTER_MURABITO_STATUS_KIND_SPECIAL_LW_WATER_WAIT].contains(&status_kind) {
				if situation_kind == *SITUATION_KIND_GROUND {
					StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_LANDING, true);
				};
			};
			if situation_kind == *SITUATION_KIND_GROUND {
				HAS_DOWNB[ENTRY_ID] = false;
			};
			if [hash40("special_air_hi"), hash40("special_hi"), hash40("special_air_hi_wait1"), hash40("special_air_hi_wait2"), hash40("special_air_hi_flap1"), hash40("special_air_hi_flap2"), hash40("special_air_hi_flap1")].contains(&MotionModule::motion_kind(boma)) {
				fighter.clear_lua_stack();
				lua_args!(fighter, *FIGHTER_KINETIC_ENERGY_ID_DAMAGE);
				smash::app::sv_kinetic_energy::clear_speed(fighter.lua_state_agent);
				StatusModule::change_status_request_from_script(boma, *FIGHTER_MURABITO_STATUS_KIND_SPECIAL_HI_DETACH, true);
			};
			if [hash40("throw_f")].contains(&MotionModule::motion_kind(boma)) {
				if status_kind != *FIGHTER_STATUS_KIND_THROW_KIRBY {
					StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_THROW_KIRBY, true);
				};
				if frame > 60.0 {
					if KineticModule::get_kinetic_type(boma) != *FIGHTER_KINETIC_TYPE_FALL {
						KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_FALL);
					};
					if situation_kind == *SITUATION_KIND_GROUND {
						StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_LANDING, true);
					};
				};
			};
			if status_kind == *FIGHTER_STATUS_KIND_ATTACK_LW3 {
				if frame > 40.0 {
					StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_WAIT, true);
				};
			};
			if status_kind == *FIGHTER_MURABITO_STATUS_KIND_SPECIAL_N_SEARCH {
				if frame > 38.0 {
					if situation_kind == *SITUATION_KIND_GROUND {
						StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_WAIT, true);
					} else {
						StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_FALL, true);
					};
				};
			};
			if ItemModule::is_have_item(boma, 0) {
				CAN_NEUTRALB[ENTRY_ID] = 1;
			} else {
				CAN_NEUTRALB[ENTRY_ID] = 0;
			};
			ArticleModule::remove_exist(boma, *FIGHTER_MURABITO_GENERATE_ARTICLE_CLAYROCKET,smash::app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
			ArticleModule::remove_exist(boma, *FIGHTER_MURABITO_GENERATE_ARTICLE_BUTTERFLYNET,smash::app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
			ArticleModule::remove_exist(boma, *FIGHTER_MURABITO_GENERATE_ARTICLE_BALLOON,smash::app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
			WorkModule::off_flag(boma, *FIGHTER_MURABITO_INSTANCE_WORK_ID_FLAG_CATCHING);
			if [*FIGHTER_STATUS_KIND_SPECIAL_S].contains(&status_kind) {
				if BEFORE_SIDEB_BOUNCE[ENTRY_ID] > 0 {
					BEFORE_SIDEB_BOUNCE[ENTRY_ID] -= 1;
				};
				if situation_kind == *SITUATION_KIND_GROUND {
					if KineticModule::get_kinetic_type(boma) != *FIGHTER_KINETIC_TYPE_MOTION {
						KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_MOTION);
					};
					if LAND_SIDEB_BOUNCE[ENTRY_ID] > 0 {
						LAND_SIDEB_BOUNCE[ENTRY_ID] -= 1;
						println!("Land Sideb: {}", LAND_SIDEB_BOUNCE[ENTRY_ID]);
					};
					if (11..20).contains(&(frame as i32)) || (LAND_SIDEB_BOUNCE[ENTRY_ID] > 0 && frame < 52.0){
						if ControlModule::check_button_on_trriger(boma, *CONTROL_PAD_BUTTON_SPECIAL) {
							StatusModule::change_status_request_from_script(boma, *FIGHTER_MURABITO_STATUS_KIND_SPECIAL_S_JUMP, true);
						};
					};
					if BEFORE_SIDEB_BOUNCE[ENTRY_ID] > 0 {
						StatusModule::change_status_request_from_script(boma, *FIGHTER_MURABITO_STATUS_KIND_SPECIAL_S_JUMP, true);
					};
				} else {
					if KineticModule::get_kinetic_type(boma) != *FIGHTER_KINETIC_TYPE_MOTION_FALL {
						KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_MOTION_FALL);
					};
					if frame > 30.0 {
						let cat1 = ControlModule::get_command_flag_cat(boma, 0);
						if (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_HI) != 0{
							StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_SPECIAL_HI, true);
						};
						if ControlModule::check_button_on_trriger(boma, *CONTROL_PAD_BUTTON_SPECIAL) {
							BEFORE_SIDEB_BOUNCE[ENTRY_ID] = 3;
						};
					};
					LAND_SIDEB_BOUNCE[ENTRY_ID] = 8;
				};
			} else {
				LAND_SIDEB_BOUNCE[ENTRY_ID] = 0;
				BEFORE_SIDEB_BOUNCE[ENTRY_ID] = 0;
			};
			if [*FIGHTER_MURABITO_STATUS_KIND_SPECIAL_S_JUMP].contains(&status_kind) {
				if KineticModule::get_kinetic_type(boma) != *FIGHTER_KINETIC_TYPE_MOTION_FALL {
					KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_MOTION_FALL);
				};
				if frame >= 13.0 {
					CancelModule::enable_cancel(boma);
				};
				if frame >= 14.0 {
					StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_FALL, true);
				};
			};
		};
	};
}
#[status_script(agent = "murabito", status = FIGHTER_STATUS_KIND_CATCH_PULL, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
pub unsafe fn main_catch_pull(fighter: &mut L2CFighterCommon) -> L2CValue {
    let motion_kind = MotionModule::motion_kind(fighter.module_accessor);
    let frame = MotionModule::frame(fighter.module_accessor);
    fighter.status_CatchPull();
	0.into()
}
#[status_script(agent = "murabito", status = FIGHTER_STATUS_KIND_CATCH_WAIT, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
pub unsafe fn main_catch_wait(fighter: &mut L2CFighterCommon) -> L2CValue {
    let motion_kind = MotionModule::motion_kind(fighter.module_accessor);
    let frame = MotionModule::frame(fighter.module_accessor);
    fighter.status_CatchWait();
	0.into()
}
#[status_script(agent = "murabito", status = FIGHTER_STATUS_KIND_THROW, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
pub unsafe fn main_throw(fighter: &mut L2CFighterCommon) -> L2CValue {
    let motion_kind = MotionModule::motion_kind(fighter.module_accessor);
    let frame = MotionModule::frame(fighter.module_accessor);
	if motion_kind == hash40("throw_f") {
		fighter.change_status(
			L2CValue::I32(*FIGHTER_STATUS_KIND_THROW_KIRBY),
			L2CValue::Bool(false)
		);
	}
    original!(fighter)
}
#[status_script(agent = "murabito", status = FIGHTER_STATUS_KIND_THROW_KIRBY, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
pub unsafe fn main_throw_kirby(fighter: &mut L2CFighterCommon) -> L2CValue {
    /*let motion_kind = MotionModule::motion_kind(fighter.module_accessor);
    let frame = MotionModule::frame(fighter.module_accessor);
	if motion_kind != hash40("throw_f") {
		MotionModule::change_motion(fighter.module_accessor, Hash40::new("throw_f"), -1.0, 1.0, false, 0.0, false, false);
	}*/
    fighter.status_ThrowKirby();
	0.into()
}
#[status_script(agent = "murabito", status = FIGHTER_STATUS_KIND_THROW_KIRBY, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_PRE)]
unsafe fn throw_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.status_pre_ThrowKirby();
    0.into()
}
#[status_script(agent = "murabito", status = FIGHTER_STATUS_KIND_THROW_KIRBY, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_END)]
unsafe fn throw_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.status_end_ThrowKirby();
    0.into()
}
#[status_script(agent = "murabito", status = FIGHTER_STATUS_KIND_THROW_KIRBY, condition = LUA_SCRIPT_STATUS_FUNC_INIT_STATUS)]
unsafe fn throw_init(fighter: &mut L2CFighterCommon) -> L2CValue {
    L2CFighterCommon::sub_status_uniq_process_ThrowKirby_initStatus(fighter);
    0.into()
}
#[status_script(agent = "murabito", status = FIGHTER_STATUS_KIND_THROW_KIRBY, condition = LUA_SCRIPT_STATUS_FUNC_EXIT_STATUS)]
unsafe fn throw_exit(fighter: &mut L2CFighterCommon) -> L2CValue {
    L2CFighterCommon::sub_status_uniq_process_ThrowKirby_exitStatus(fighter);
    0.into()
}
#[fighter_reset]
fn agent_reset(fighter: &mut L2CFighterCommon) {
    unsafe {
        let fighter_kind = utility::get_kind(&mut *fighter.module_accessor);
        if fighter_kind != *FIGHTER_KIND_MURABITO {
            return;
        }
        fighter.global_table[0x45].assign(&FIGHTER_STATUS_KIND_THROW_KIRBY.into());
    }
}

pub(crate) unsafe fn attack_vc(fighter: &mut L2CAgentBase) -> () {
	let rand_val = smash::app::sv_math::rand(hash40("fighter"), 7);
	match rand_val {
		0 => macros::PLAY_SE(fighter, Hash40::new("se_murabito_attackair_b01")),
		1 => macros::PLAY_SE(fighter, Hash40::new("se_murabito_attackair_b02")),
		2 => macros::PLAY_SE(fighter, Hash40::new("se_murabito_attackair_f01")),
		3 => macros::PLAY_SE(fighter, Hash40::new("se_murabito_attackair_f02")),
		4 => macros::PLAY_SE(fighter, Hash40::new("se_murabito_attackair_h01")),
		_ => println!("toad is silent"),
	}
}
pub(crate) unsafe fn dmg_vc(fighter: &mut L2CAgentBase) -> () {
	let rand_val = smash::app::sv_math::rand(hash40("fighter"), 3);
	match rand_val {
		0 => macros::PLAY_SE(fighter, Hash40::new("se_murabito_attackair_h02")),
		1 => macros::PLAY_SE(fighter, Hash40::new("se_murabito_attackair_h03")),
		_ => macros::PLAY_SE(fighter, Hash40::new("se_murabito_attackair_h01")),
	}
}
pub(crate) unsafe fn dmg_fly_vc(fighter: &mut L2CAgentBase) -> () {
	let rand_val = smash::app::sv_math::rand(hash40("fighter"), 3);
	match rand_val {
		0 => macros::PLAY_SE(fighter, Hash40::new("se_murabito_attackair_l02")),
		1 => macros::PLAY_SE(fighter, Hash40::new("se_murabito_attackair_l03")),
		_ => macros::PLAY_SE(fighter, Hash40::new("se_murabito_attackair_l01")),
	}
}

pub fn install() {
    smashline::install_acmd_scripts!(
		//Jab
		toad_jab1,
		toad_jab1_eff,
		toad_jab1_snd,
		toad_jab2,
		toad_jab2_eff,
		toad_jab2_snd,
		toad_jab3,
		toad_jab3_eff,
		toad_jab3_snd,

		//Dash Attack
		toad_da,
		toad_da_eff,
		toad_da_snd,
		toad_shell_burst_eff,

		//Tilts
		toad_utilt,
		toad_utilt_eff,
		toad_utilt_snd,
		toad_ftilt,
		toad_ftilt_eff,
		toad_ftilt_snd,
		toad_dtilt,
		toad_dtilt_eff,
		toad_dtilt_snd,
		toad_dtilt_expr,
		toad_dsmash,
		toad_dsmash_snd,
		toad_dsmash_eff,
		toad_dsmash_charge,
		toad_dsmash_charge_eff,
		toad_dsmash_charge_expr,

		//Smashes
		toad_usmash,
		toad_usmash_eff,
		toad_usmash_snd,
		toad_usmash_coin,
		//toad_usmash_coin_eff,
		//toad_usmash_coin_snd,
		toad_usmash_coin_bullet,
		toad_usmash_coin_bullet_eff,
		toad_usmash_coin_bullet_snd,

		//Aerials
		toad_nair,
		toad_nair_snd,
		toad_uair_land,
		toad_uair,
		toad_uair_eff,
		toad_uair_snd,
		toad_fair,
		toad_fair_eff,
		toad_fair_snd,
		toad_bair,
		toad_bair_snd,
		toad_bair_eff,
		toad_dair,
		toad_dair_snd,
		toad_land_dair_snd,
		toad_land_dair,
		toad_dair_eff,

		//Specials
		toad_neutralb,
		toad_neutralb_eff,
		toad_neutralb_snd,
		toad_sideb,
		toad_upb,
		toad_upb_snd,
		toad_upb_eff,
		toad_downb,
		toad_downb_eff,
		toad_downb_snd,
		toad_downb_air,
		toad_downb_air_snd,
		toad_downb_air_eff,

		//Throws
		toad_catch,
		toad_catch_snd,
		toad_catchattack,
		toad_catchdash,
		toad_catchdash_snd,
		toad_catchturn,
		toad_catchturn_snd,
		toad_dthrow,
		toad_bthrow,
		toad_fthrow,
		toad_fthrow_snd,

		//Unique
		toad_dmg_fly_snd,
		toad_dmg_snd
    );
    install_agent_resets!(
        agent_reset
    );
    smashline::install_agent_frame_callbacks!(toad);
	install_status_scripts!(main_catch_pull, main_catch_wait, throw_pre, throw_exit, throw_end, throw_init, main_throw_kirby);
}
