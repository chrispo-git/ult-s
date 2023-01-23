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
    acmd!(lua_state, {
		frame(Frame=10)
		if(is_excute){
			PLAY_SE(hash40("se_murabito_swing_l"))
		}
    });
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
		frame(Frame=6)
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
    acmd!(lua_state, {
		frame(Frame=5)
		if(is_excute){
			PLAY_SE(hash40("se_murabito_swing_l"))
		}
    });
}	
#[acmd_script(
    agent = "murabito",
    script =  "effect_attackairb",
    category = ACMD_EFFECT,
	low_priority)]
unsafe fn toad_bair_eff(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		frame(Frame=12)
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
    acmd!(lua_state, {
		frame(Frame=5)
		if(is_excute){
			PLAY_SE(hash40("se_murabito_swing_m"))
		}
    });
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
			ATTACK(ID=0, Part=0, Bone=hash40("head"), Damage=15.0, Angle=361, KBG=102, FKB=0, BKB=30, Size=6.2, X=4.4, Y=-1.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_HEAD)
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
}
#[acmd_script(
    agent = "murabito",
    script =  "sound_attackairf",
    category = ACMD_SOUND,
	low_priority)]
unsafe fn toad_fair_snd(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		frame(Frame=15)
		if(is_excute){
			PLAY_SE(hash40("se_murabito_swing_l"))
		}
    });
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
			ATTACK(ID=0, Part=0, Bone=hash40("head"), Damage=10.0, Angle=90, KBG=100, FKB=0, BKB=50, Size=6.5, X=4.4, Y=-1.0, Z=0.0, X=LUA_VOID, Y=LUA_VOID, Z=LUA_VOID, Hitlag=1.0, SDI=0.8, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_sting"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_PUNCH)
			ATTACK(ID=1, Part=0, Bone=hash40("legc"), Damage=6.5, Angle=90, KBG=100, FKB=0, BKB=50, Size=4.5, X=4.4, Y=-1.0, Z=0.0, X=LUA_VOID, Y=LUA_VOID, Z=LUA_VOID, Hitlag=1.0, SDI=0.8, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_PUNCH)
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
    acmd!(lua_state, {
		frame(Frame=5)
		if(is_excute){
			PLAY_SE(hash40("se_murabito_swing_s"))
		}
    });
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
			ATTACK(ID=0, Part=0, Bone=hash40("havel"), Damage=17.0, Angle=82, KBG=78, FKB=0, BKB=58, Size=10.0, X=0.0, Y=0.0, Z=0.0, X=LUA_VOID, Y=LUA_VOID, Z=LUA_VOID, Hitlag=1.0, SDI=0.8, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_PUNCH)
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
    acmd!(lua_state, {
		frame(Frame=6)
		if(is_excute){
			PLAY_SE(hash40("se_common_step_jump"))
		}
		frame(Frame=13)
		if(is_excute){
			PLAY_SE(hash40("se_common_smashswing_01"))
			PLAY_SE(hash40("se_common_pitin_move_00"))
			PLAY_SE(hash40("se_common_landing_brick"))
		}
		frame(Frame=47)
		if(is_excute){
			PLAY_SE(hash40("se_common_landing_soil"))
		}
    });
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
			ATTACK(ID=0, Part=0, Bone=hash40("haver"), Damage=11.0, Angle=35, KBG=76, FKB=0, BKB=48, Size=5.5, X=0.0, Y=10.5, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_OBJECT)
			ATTACK(ID=1, Part=0, Bone=hash40("haver"), Damage=11.0, Angle=35, KBG=76, FKB=0, BKB=48, Size=4.5, X=0.0, Y=7.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_OBJECT)
			ATTACK(ID=2, Part=0, Bone=hash40("haver"), Damage=11.0, Angle=35, KBG=76, FKB=0, BKB=48, Size=4.5, X=0.0, Y=3.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_OBJECT)
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
		frame(Frame=23)
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
    acmd!(lua_state, {
		frame(Frame=6)
		if(is_excute){
			PLAY_SE(hash40("se_murabito_swing_s"))
		}
    });
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
    acmd!(lua_state, {
		frame(Frame=9)
		if(is_excute){
			PLAY_SE(hash40("se_murabito_attackhard_l01"))
		}
    });
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
			//ArticleModule::remove_exist(boma, *FIGHTER_MURABITO_GENERATE_ARTICLE_BUTTERFLYNET,smash::app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
			if [*FIGHTER_STATUS_KIND_SPECIAL_S].contains(&status_kind) {
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
				} else {
					if KineticModule::get_kinetic_type(boma) != *FIGHTER_KINETIC_TYPE_MOTION_FALL {
						KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_MOTION_FALL);
					};
					LAND_SIDEB_BOUNCE[ENTRY_ID] = 8;
				};
			} else {
				LAND_SIDEB_BOUNCE[ENTRY_ID] = 0;
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
    fighter.status_CatchPull_common();
	0.into()
}
#[status_script(agent = "murabito", status = FIGHTER_STATUS_KIND_CATCH_WAIT, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
pub unsafe fn main_catch_wait(fighter: &mut L2CFighterCommon) -> L2CValue {
    let motion_kind = MotionModule::motion_kind(fighter.module_accessor);
    let frame = MotionModule::frame(fighter.module_accessor);
    fighter.status_CatchWait_common();
	0.into()
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
		toad_dtilt,
		toad_dtilt_eff,
		toad_dtilt_snd,
		toad_dtilt_expr,

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

		//Specials
		toad_neutralb,
		toad_neutralb_eff,
		toad_neutralb_snd,
		toad_sideb,

		//Throws
		toad_catch,
		toad_catchattack,
		toad_catchdash,
		toad_catchturn
    );
    smashline::install_agent_frame_callbacks!(toad);
	install_status_scripts!(main_catch_pull, main_catch_wait);
}
