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

static mut NONE :  smash::phx::Vector3f =  smash::phx::Vector3f { x: 0.0, y: 0.0, z: 0.0 };
static mut SHOOT :  smash::phx::Vector3f =  smash::phx::Vector3f { x: 0.0, y: 10.0, z: 20.0 };

#[acmd_script(
    agent = "link",
    script =  "effect_landingheavy",
    category = ACMD_EFFECT)]
unsafe fn link_landing(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		frame(Frame=2)
		if(is_excute){
			LANDING_EFFECT(hash40("sys_landing_smoke"), hash40("top"), 0, 0, 0, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, true)
		}
    });
}		

#[acmd_script(
    agent = "link",
    script =  "game_attack11",
    category = ACMD_GAME,
	low_priority)]
unsafe fn link_jab(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		frame(Frame=1)
		FT_MOTION_RATE(FSM=0.5)
		wait(Frames=6)
		FT_MOTION_RATE(FSM=1)
		frame(Frame=8)
		if(is_excute){
			ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=3.0, Angle=361, KBG=25, FKB=0, BKB=25, Size=1.8, X=0.0, Y=8.0, Z=10.0, X2=0.0, Y2=8.0, Z2=7.0, Hitlag=1.6, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
			ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=3.0, Angle=361, KBG=25, FKB=0, BKB=25, Size=1.8, X=0.0, Y=8.0, Z=14.0, X2=0.0, Y2=8.0, Z2=7.0, Hitlag=1.6, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
			ATTACK(ID=2, Part=0, Bone=hash40("top"), Damage=3.0, Angle=180, KBG=15, FKB=0, BKB=20, Size=1.8, X=0.0, Y=8.0, Z=18.0, X2=0.0, Y2=8.0, Z2=7.0, Hitlag=1.6, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_FIGHTER, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
			ATTACK(ID=3, Part=0, Bone=hash40("top"), Damage=3.0, Angle=361, KBG=15, FKB=0, BKB=20, Size=1.8, X=0.0, Y=8.0, Z=18.0, X2=0.0, Y2=8.0, Z2=7.0, Hitlag=1.6, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
		}
		wait(Frames=2)
		if(is_excute){
			AttackModule::clear_all()
			WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO)
			WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_FLAG_ENABLE_NO_HIT_COMBO)
		}
    });
}
#[acmd_script(
    agent = "link",
    script =  "game_attack12",
    category = ACMD_GAME,
	low_priority)]
unsafe fn link_jab2(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		frame(Frame=5)
		if(is_excute){
			ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=3.0, Angle=361, KBG=25, FKB=0, BKB=25, Size=2.8, X=0.0, Y=8.5, Z=9.5, X2=0.0, Y2=8.5, Z2=8.5, Hitlag=1.2, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
			ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=3.0, Angle=361, KBG=25, FKB=0, BKB=25, Size=3.5, X=0.0, Y=8.5, Z=14.5, X2=0.0, Y2=8.5, Z2=8.5, Hitlag=1.2, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
			ATTACK(ID=2, Part=0, Bone=hash40("top"), Damage=3.0, Angle=180, KBG=15, FKB=0, BKB=20, Size=3.5, X=0.0, Y=8.5, Z=18.5, X2=0.0, Y2=8.5, Z2=8.5, Hitlag=1.2, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_FIGHTER, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
			ATTACK(ID=3, Part=0, Bone=hash40("top"), Damage=3.0, Angle=361, KBG=15, FKB=0, BKB=20, Size=3.5, X=0.0, Y=8.5, Z=18.5, X2=0.0, Y2=8.5, Z2=8.5, Hitlag=1.2, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
			AttackModule::set_add_reaction_frame(ID=0, Frames=1.0, Unk=false)
			AttackModule::set_add_reaction_frame(ID=1, Frames=1.0, Unk=false)
			AttackModule::set_add_reaction_frame(ID=2, Frames=1.0, Unk=false)
			AttackModule::set_add_reaction_frame(ID=3, Frames=1.0, Unk=false)
		}
		wait(Frames=2)
		if(is_excute){
			AttackModule::clear_all()
			WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO)
		}
		frame(Frame=12)
		if(is_excute){
			WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_FLAG_ENABLE_NO_HIT_COMBO)
		}
    });
}
#[acmd_script(
    agent = "link",
    script =  "game_attacklw3",
    category = ACMD_GAME,
	low_priority)]
unsafe fn link_dtilt(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		frame(Frame=1)
		FT_MOTION_RATE(FSM=0.7)
		frame(Frame=13)
		FT_MOTION_RATE(FSM=1)
		if(is_excute){
			ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=5.0, Angle=80, KBG=30, FKB=0, BKB=90, Size=2.0, X=0.0, Y=1.3, Z=17.5, X2=0.0, Y2=5.0, Z2=5.5, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
			AttackModule::set_add_reaction_frame(ID=0, Frames=4.0, Unk=false)
			ATK_SET_SHIELD_SETOFF_MUL(ID=0, ShieldstunMul=0.05)
		}
		wait(Frames=2)
		if(is_excute){
			AttackModule::clear_all()
		}
    });
}
#[acmd_script(
    agent = "link",
    script =  "game_attackhi3",
    category = ACMD_GAME,
	low_priority)]
unsafe fn link_utilt(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		frame(Frame=8)
		if(is_excute){
			ATTACK(ID=0, Part=0, Bone=hash40("sword2"), Damage=11.0, Angle=95, KBG=103, FKB=0, BKB=30, Size=3.4, X=8.4, Y=0.0, Z=-2.5, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.9, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
			ATTACK(ID=1, Part=0, Bone=hash40("sword2"), Damage=11.0, Angle=85, KBG=111, FKB=0, BKB=30, Size=4.3, X=2.0, Y=0.0, Z=-3.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.9, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
			ATTACK(ID=2, Part=0, Bone=hash40("armr"), Damage=11.0, Angle=85, KBG=105, FKB=0, BKB=30, Size=2.7, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.9, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
			ATTACK(ID=3, Part=0, Bone=hash40("colonells"), Damage=11.0, Angle=85, KBG=104, FKB=0, BKB=30, Size=2.16, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.9, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
		}
		frame(Frame=14)
		if(is_excute){
			AttackModule::clear_all()
		}
    });
}
#[acmd_script(
    agent = "link",
    script =  "game_attackairb",
    category = ACMD_GAME,
	low_priority)]
unsafe fn link_bair(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		FT_MOTION_RATE(FSM=2)
		if(is_excute){
			WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
		}
		frame(Frame=6)
		FT_MOTION_RATE(FSM=1)
		if(is_excute){
			ATTACK(ID=0, Part=0, Bone=hash40("kneer"), Damage=5.0, Angle=73, KBG=100, FKB=50, BKB=0, Size=4.0, X=5.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.8, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_B, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_G, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
			ATTACK(ID=1, Part=0, Bone=hash40("kneer"), Damage=5.0, Angle=70, KBG=100, FKB=36, BKB=0, Size=4.0, X=5.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.8, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_B, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_A, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
			ATTACK(ID=2, Part=0, Bone=hash40("kneer"), Damage=5.0, Angle=70, KBG=100, FKB=50, BKB=0, Size=4.5, X=-0.5, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.8, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_B, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_G, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
			ATTACK(ID=3, Part=0, Bone=hash40("kneer"), Damage=5.0, Angle=66, KBG=100, FKB=36, BKB=0, Size=4.5, X=-0.5, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.8, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_B, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_A, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
			ATTACK(ID=4, Part=0, Bone=hash40("hip"), Damage=5.0, Angle=70, KBG=100, FKB=50, BKB=0, Size=5.0, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.8, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_B, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_G, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
			ATTACK(ID=5, Part=0, Bone=hash40("hip"), Damage=5.0, Angle=66, KBG=100, FKB=36, BKB=0, Size=5.0, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.8, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_B, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_A, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
		}
		wait(Frames=3)
		if(is_excute){
			AttackModule::clear_all()
		}
		frame(Frame=15)
		if(is_excute){
			ATTACK(ID=0, Part=0, Bone=hash40("kneel"), Damage=7.0, Angle=361, KBG=98, FKB=0, BKB=45, Size=4.5, X=5.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_B, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
			ATTACK(ID=1, Part=0, Bone=hash40("kneel"), Damage=7.0, Angle=361, KBG=98, FKB=0, BKB=45, Size=5.0, X=-0.5, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_B, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
			ATTACK(ID=2, Part=0, Bone=hash40("hip"), Damage=7.0, Angle=361, KBG=98, FKB=0, BKB=45, Size=5.5, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_B, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
		}
		wait(Frames=3)
		if(is_excute){
			AttackModule::clear_all()
		}
		frame(Frame=29)
		if(is_excute){
			WorkModule::off_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
		}
    });
} 

#[acmd_script(
    agent = "link",
    scripts =  ["game_specialairnstart"],
    category = ACMD_GAME,
	low_priority)]
unsafe fn link_arrow_air(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		if(is_excute){
			ArticleModule::generate_article(FIGHTER_LINK_GENERATE_ARTICLE_BOWARROW, false, 0)
			ArticleModule::change_motion(FIGHTER_LINK_GENERATE_ARTICLE_BOWARROW,smash::phx::Hash40::new("n_air_start"),false,0.0)
		}
		FT_MOTION_RATE(FSM=0.722222)
		frame(Frame=18)
		FT_MOTION_RATE(FSM=1)
		if(is_excute){
			WorkModule::on_flag(Flag=FIGHTER_LINK_STATUS_BOW_FLAG_CHARGE)
		}
    });
} 
#[acmd_script(
    agent = "link",
    scripts =  ["game_specialnstart"],
    category = ACMD_GAME,
	low_priority)]
unsafe fn link_arrow(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		if(is_excute){
			ArticleModule::generate_article(FIGHTER_LINK_GENERATE_ARTICLE_BOWARROW, false, 0)
			ArticleModule::change_motion(FIGHTER_LINK_GENERATE_ARTICLE_BOWARROW,smash::phx::Hash40::new("n_start"),false,0.0)
		}
		FT_MOTION_RATE(FSM=0.722222)
		frame(Frame=18)
		FT_MOTION_RATE(FSM=1)
		if(is_excute){
			WorkModule::on_flag(Flag=FIGHTER_LINK_STATUS_BOW_FLAG_CHARGE)
		}
    });
} 

#[acmd_script( 
	agent = "link", 
	scripts = ["game_attacklw4"], 
	category = ACMD_GAME, 
	low_priority )]
unsafe fn link_dsmash(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        if(is_excute){
            ArticleModule::generate_article(FIGHTER_LINK_GENERATE_ARTICLE_BOW, false, -1);
            ArticleModule::set_visibility_whole(FIGHTER_LINK_GENERATE_ARTICLE_BOW, true, smash::app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL))
            ArticleModule::change_status_exist(FIGHTER_LINK_GENERATE_ARTICLE_BOW, *WN_LINK_BOW_STATUS_KIND_HAVE)
			ArticleModule::change_motion(FIGHTER_LINK_GENERATE_ARTICLE_BOW,smash::phx::Hash40::new("d_smash"),false,0.0)
        }
        frame(Frame=5)
        if(is_excute){
            WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD)
        }
        frame(Frame=21)
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("have"), Damage=15.2, Angle=32, KBG=80, FKB=0, BKB=45, Size=3.8, X=0.0, Y=-0.5, Z=5.5, X2=0.0, Y2=1.6, Z2=5.5, Hitlag=0.8, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=10, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_PUNCH)
			ATTACK(ID=1, Part=0, Bone=hash40("have"), Damage=15.2, Angle=32, KBG=80, FKB=0, BKB=45, Size=3.4, X=0.0, Y=3.5, Z=-0.5, X2=0.0, Y2=3.5, Z2=12.5, Hitlag=0.8, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=10, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_G, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_PUNCH)
			ATTACK(ID=2, Part=0, Bone=hash40("have"), Damage=14.2, Angle=32, KBG=74, FKB=0, BKB=45, Size=3.4, X=0.0, Y=3.5, Z=-7.5, X2=0.0, Y2=3.5, Z2=19.5, Hitlag=0.6, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=10, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_G, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_PUNCH)
		}
		frame(Frame=28)
        if(is_excute){
            AttackModule::clear_all()
        }
        frame(Frame=38)
        if(is_excute){
            //Before the FaF, do this
            ArticleModule::change_status_exist(FIGHTER_LINK_GENERATE_ARTICLE_BOW, *WN_LINK_BOW_STATUS_KIND_BACK)
        }
    });
}
#[acmd_script( 
	agent = "link", 
	scripts = ["effect_attacklw4"], 
	category = ACMD_EFFECT, 
	low_priority )]
unsafe fn link_dsmash_eff(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		if(is_excute){
			EFFECT(hash40("sys_smash_flash"), hash40("bow"), 2, 10, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true)
		}
		frame(Frame=13)
		if(is_excute){
			LANDING_EFFECT(hash40("sys_landing_smoke_s"), hash40("top"), -0.9, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false)
		}
        frame(Frame=21)
		if(is_excute){
			LANDING_EFFECT(hash40("sys_down_smoke"), hash40("top"), 6, 0, 0, 0, 0, 0, 1.2, 0, 0, 0, 0, 0, 0, false)
			EFFECT(hash40("sys_crown"), hash40("top"), 6, 0, -3, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false)
			QUAKE(CAMERA_QUAKE_KIND_M)
			LAST_EFFECT_SET_ALPHA(0.5)
		}
		frame(Frame=56)
		if(is_excute){
			LANDING_EFFECT(hash40("sys_landing_smoke_s"), hash40("top"), -0.9, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false)
		}
    });
}
#[acmd_script( 
	agent = "link", 
	scripts = ["sound_attacklw4"], 
	category = ACMD_SOUND, 
	low_priority )]
unsafe fn link_dsmash_snd(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        frame(Frame=13)
		if(is_excute){
			PLAY_SE(hash40("vc_link_attack05"))
		}
		frame(Frame=21)
		if(is_excute){
			PLAY_LANDING_SE(hash40("se_link_attackair_l01"))
		}
    });
}
#[acmd_script(
    agent = "link",
    script =  "expression_attacklw4",
    category = ACMD_EXPRESSION,
    low_priority)]
unsafe fn link_dsmash_expr(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        if(is_excute){
            VisibilityModule::set_int64(hash40("shield") as i64, hash40("shield_back") as i64)
            VisibilityModule::set_int64(hash40("sword") as i64, hash40("sword_back") as i64)
            ItemModule::set_have_item_visibility(false, 0)
        }
        frame(Frame=6)
        if(is_excute){
            VisibilityModule::set_int64(hash40("shield") as i64, hash40("shield_back") as i64)
            VisibilityModule::set_int64(hash40("sword") as i64, hash40("sword_back") as i64)
            ItemModule::set_have_item_visibility(false, 0)
        }
        frame(Frame=40)
        if(is_excute){
            VisibilityModule::set_int64(hash40("shield") as i64, hash40("shield_normal") as i64)
            VisibilityModule::set_int64(hash40("sword") as i64, hash40("sword_normal") as i64)
            ItemModule::set_have_item_visibility(true, 0)
        }
    });
}    
#[acmd_script(
    agent = "link",
    script =  "expression_attacklw4charge",
    category = ACMD_EXPRESSION,
    low_priority)]
unsafe fn link_dsmash_charge_expr(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        if(is_excute){
            VisibilityModule::set_int64(hash40("shield") as i64, hash40("shield_back") as i64)
            VisibilityModule::set_int64(hash40("sword") as i64, hash40("sword_back") as i64)
            ItemModule::set_have_item_visibility(false, 0)
        }
    });
}  

#[acmd_script( 
	agent = "link", 
	scripts = ["game_attackairf"], 
	category = ACMD_GAME, 
	low_priority )]
unsafe fn link_fair(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        if(is_excute){
            ArticleModule::generate_article(FIGHTER_LINK_GENERATE_ARTICLE_BOW, false, -1);
            ArticleModule::set_visibility_whole(FIGHTER_LINK_GENERATE_ARTICLE_BOW, true, smash::app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL))
            ArticleModule::change_status_exist(FIGHTER_LINK_GENERATE_ARTICLE_BOW, *WN_LINK_BOW_STATUS_KIND_HAVE)
			ArticleModule::change_motion(FIGHTER_LINK_GENERATE_ARTICLE_BOW,smash::phx::Hash40::new("f_air"),false,0.0)
        }
		frame(Frame=3)
		if(is_excute){
			WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
		}
		frame(Frame=13)
		if(is_excute){
			ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=7.5, Angle=361, KBG=90, FKB=0, BKB=40, Size=4.0, X=0.0, Y=12.5, Z=4.5, X2=0.0, Y2=12.5, Z2=8.5, Hitlag=0.8, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_sting"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
			ATTACK(ID=2, Part=0, Bone=hash40("have"), Damage=13.5, Angle=361, KBG=90, FKB=0, BKB=40, Size=2.3, X=0.0, Y=12.5, Z=22.0, X2=0.0, Y2=12.5, Z2=28.5, Hitlag=1.3, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_sting"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
			ATTACK(ID=1, Part=0, Bone=hash40("have"), Damage=10.5, Angle=361, KBG=90, FKB=0, BKB=40, Size=2.5, X=0.0, Y=12.5, Z=12.5, X2=0.0, Y2=12.5, Z2=20.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_sting"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
		}
		frame(Frame=15)
		if(is_excute){
			ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=5.0, Angle=361, KBG=90, FKB=0, BKB=40, Size=3.5, X=0.0, Y=12.5, Z=4.5, X2=0.0, Y2=12.5, Z2=8.5, Hitlag=0.8, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
			ATTACK(ID=2, Part=0, Bone=hash40("have"), Damage=7.5, Angle=361, KBG=90, FKB=0, BKB=40, Size=1.8, X=0.0, Y=12.5, Z=22.0, X2=0.0, Y2=12.5, Z2=28.5, Hitlag=0.8, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
			ATTACK(ID=1, Part=0, Bone=hash40("have"), Damage=6.5, Angle=361, KBG=90, FKB=0, BKB=40, Size=2.0, X=0.0, Y=12.5, Z=12.5, X2=0.0, Y2=12.5, Z2=20.0, Hitlag=0.8, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
		}
		frame(Frame=19)
		if(is_excute){
			AttackModule::clear_all()
		}
        frame(Frame=25)
        if(is_excute){
            //Before the FaF, do this
            ArticleModule::change_status_exist(FIGHTER_LINK_GENERATE_ARTICLE_BOW, *WN_LINK_BOW_STATUS_KIND_BACK)
        }
		frame(Frame=39)
		if(is_excute){
			WorkModule::off_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
		}
    });
}
#[acmd_script( 
	agent = "link", 
	scripts = ["effect_attackairf"], 
	category = ACMD_EFFECT, 
	low_priority )]
unsafe fn link_fair_eff(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		frame(Frame=13)
		if(is_excute){
			EFFECT_FOLLOW(hash40("sys_attack_speedline"), hash40("top"), 0.0, 11.5, 11.0, 0, 0, 0, 0.65, 0, 0, 0, 0, 0, 0, true)
			LAST_PARTICLE_SET_COLOR(0.4, 0.6, 1)
			LAST_EFFECT_SET_RATE(0.8)
			EFFECT(hash40("sys_smash_flash"), hash40("top"), 0.0, 12.5, 30.0, 0, 0, 0, 0.35, 0, 0, 0, 0, 0, 0, true)
		}
    });
}
#[acmd_script( 
	agent = "link", 
	scripts = ["sound_attackairf"], 
	category = ACMD_SOUND, 
	low_priority )]
unsafe fn link_fair_snd(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		frame(Frame=13)
		if(is_excute){
			PLAY_SE(hash40("se_link_swing_l"))
		}
    });
}
#[acmd_script(
    agent = "link",
    script =  "expression_attackairf",
    category = ACMD_EXPRESSION,
    low_priority)]
unsafe fn link_fair_expr(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        if(is_excute){
            VisibilityModule::set_int64(hash40("shield") as i64, hash40("shield_back") as i64)
            VisibilityModule::set_int64(hash40("sword") as i64, hash40("sword_back") as i64)
            ItemModule::set_have_item_visibility(false, 0)
        }
        frame(Frame=6)
        if(is_excute){
            VisibilityModule::set_int64(hash40("shield") as i64, hash40("shield_back") as i64)
            VisibilityModule::set_int64(hash40("sword") as i64, hash40("sword_back") as i64)
            ItemModule::set_have_item_visibility(false, 0)
        }
        frame(Frame=40)
        if(is_excute){
            VisibilityModule::set_int64(hash40("shield") as i64, hash40("shield_normal") as i64)
            VisibilityModule::set_int64(hash40("sword") as i64, hash40("sword_normal") as i64)
            ItemModule::set_have_item_visibility(true, 0)
        }
    });
}

#[acmd_script( 
	agent = "link", 
	scripts = ["game_attacks3"], 
	category = ACMD_GAME, 
	low_priority )]
unsafe fn link_ftilt(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		frame(Frame=3)
        if(is_excute){
            ArticleModule::generate_article(FIGHTER_LINK_GENERATE_ARTICLE_BOW, false, -1);
            ArticleModule::set_visibility_whole(FIGHTER_LINK_GENERATE_ARTICLE_BOW, true, smash::app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL))
            ArticleModule::change_status_exist(FIGHTER_LINK_GENERATE_ARTICLE_BOW, *WN_LINK_BOW_STATUS_KIND_HAVE)
			ArticleModule::change_motion(FIGHTER_LINK_GENERATE_ARTICLE_BOW,smash::phx::Hash40::new("f_air"),false,0.0)
        }
		frame(Frame=13)
		if(is_excute){
			ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=7.5, Angle=361, KBG=86, FKB=0, BKB=39, Size=4.0, X=0.0, Y=8.0, Z=6.5, X2=0.0, Y2=8.0, Z2=8.0, Hitlag=0.8, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_sting"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
			ATTACK(ID=2, Part=0, Bone=hash40("have"), Damage=13.0, Angle=361, KBG=86, FKB=0, BKB=39, Size=2.3, X=0.0, Y=8.0, Z=19.0, X2=0.0, Y2=8.0, Z2=25.0, Hitlag=1.2, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_sting"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
			ATTACK(ID=1, Part=0, Bone=hash40("have"), Damage=10.0, Angle=361, KBG=86, FKB=0, BKB=39, Size=2.5, X=0.0, Y=8.0, Z=9.5, X2=0.0, Y2=8.0, Z2=17.5, Hitlag=0.8, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_sting"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
		}
		frame(Frame=17)
		if(is_excute){
			AttackModule::clear_all()
		}
		frame(Frame=35)
        if(is_excute){
            //Before the FaF, do this
            ArticleModule::change_status_exist(FIGHTER_LINK_GENERATE_ARTICLE_BOW, *WN_LINK_BOW_STATUS_KIND_BACK)
        }
    });
}
#[acmd_script( 
	agent = "link", 
	scripts = ["effect_attacks3"], 
	category = ACMD_EFFECT, 
	low_priority )]
unsafe fn link_ftilt_eff(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		frame(Frame=13)
		if(is_excute){
			EFFECT_FOLLOW(hash40("sys_attack_speedline"), hash40("top"), 0.0, 8.0, 9.5, 0, 0, 0, 0.65, 0, 0, 0, 0, 0, 0, true)
			LAST_PARTICLE_SET_COLOR(0.4, 0.6, 1)
			LAST_EFFECT_SET_RATE(0.8)
			EFFECT(hash40("sys_smash_flash"), hash40("top"), 0.0, 8.0, 26.0, 0, 0, 0, 0.35, 0, 0, 0, 0, 0, 0, true)
			FOOT_EFFECT(hash40("sys_run_smoke"), hash40("top"), -6.5, 0.0, -1.0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false)
		}
    });
}
#[acmd_script( 
	agent = "link", 
	scripts = ["sound_attacks3"], 
	category = ACMD_SOUND, 
	low_priority )]
unsafe fn link_ftilt_snd(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		frame(Frame=13)
		if(is_excute){
			PLAY_SE(hash40("se_link_swing_l"))
			PLAY_SEQUENCE(hash40("seq_link_rnd_attack"))
		}
    });
}
#[acmd_script(
    agent = "link",
    script =  "expression_attacks3",
    category = ACMD_EXPRESSION,
    low_priority)]
unsafe fn link_ftilt_expr(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		frame(Frame=3)
        if(is_excute){
            VisibilityModule::set_int64(hash40("shield") as i64, hash40("shield_back") as i64)
            VisibilityModule::set_int64(hash40("sword") as i64, hash40("sword_back") as i64)
            ItemModule::set_have_item_visibility(false, 0)
        }
        frame(Frame=6)
        if(is_excute){
            VisibilityModule::set_int64(hash40("shield") as i64, hash40("shield_back") as i64)
            VisibilityModule::set_int64(hash40("sword") as i64, hash40("sword_back") as i64)
            ItemModule::set_have_item_visibility(false, 0)
        }
        frame(Frame=35)
        if(is_excute){
            VisibilityModule::set_int64(hash40("shield") as i64, hash40("shield_normal") as i64)
            VisibilityModule::set_int64(hash40("sword") as i64, hash40("sword_normal") as i64)
            ItemModule::set_have_item_visibility(true, 0)
        }
    });
}

#[acmd_script( 
	agent = "link", 
	scripts = ["game_attackairhi"], 
	category = ACMD_GAME, 
	low_priority )]
unsafe fn link_uair(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        if(is_excute){
            ArticleModule::generate_article(FIGHTER_LINK_GENERATE_ARTICLE_BOW, false, -1);
            ArticleModule::set_visibility_whole(FIGHTER_LINK_GENERATE_ARTICLE_BOW, true, smash::app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL))
            ArticleModule::change_status_exist(FIGHTER_LINK_GENERATE_ARTICLE_BOW, *WN_LINK_BOW_STATUS_KIND_HAVE)
			ArticleModule::change_motion(FIGHTER_LINK_GENERATE_ARTICLE_BOW,smash::phx::Hash40::new("u_air"),false,0.0)
        }
		frame(Frame=3)
		if(is_excute){
			WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
		}
		frame(Frame=4)
		if(is_excute){
			ATTACK(ID=1, Part=0, Bone=hash40("armr"), Damage=7.6, Angle=80, KBG=110, FKB=0, BKB=40, Size=4.2, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_BODY)
			ATTACK(ID=2, Part=0, Bone=hash40("havel"), Damage=7.6, Angle=80, KBG=110, FKB=0, BKB=40, Size=3.0, X=0.0, Y=0.0, Z=1.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_BODY)
			ATTACK(ID=3, Part=0, Bone=hash40("havel"), Damage=7.6, Angle=80, KBG=110, FKB=0, BKB=40, Size=3.9, X=0.0, Y=-1.0, Z=6.5, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_BODY)
		}
		frame(Frame=13)
		if(is_excute){
			AttackModule::clear_all()
		}
        frame(Frame=20)
        if(is_excute){
            //Before the FaF, do this
            ArticleModule::change_status_exist(FIGHTER_LINK_GENERATE_ARTICLE_BOW, *WN_LINK_BOW_STATUS_KIND_BACK)
        }
		frame(Frame=40)
		if(is_excute){
			WorkModule::off_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
		}
    });
}
#[acmd_script( 
	agent = "link", 
	scripts = ["effect_attackairhi"], 
	category = ACMD_EFFECT, 
	low_priority )]
unsafe fn link_uair_eff(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		frame(Frame=4)
		if(is_excute){
			EFFECT_FOLLOW_FLIP_ALPHA(hash40("sys_attack_arc"), hash40("sys_attack_arc"), hash40("top"), -1.5, 11.0, 0.5, 0, 90, 60, 1.5, true, EF_FLIP_YZ, 0.7)
			LAST_PARTICLE_SET_COLOR(2.04/2.55, 1.62/2.55, 1.1/2.55)
			LAST_EFFECT_SET_RATE(1.1)
		}
    });
}
#[acmd_script( 
	agent = "link", 
	scripts = ["sound_attackairhi"], 
	category = ACMD_SOUND, 
	low_priority )]
unsafe fn link_uair_snd(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		frame(Frame=7)
		if(is_excute){
			PLAY_SEQUENCE(hash40("seq_link_rnd_attack"))
			PLAY_SE(hash40("se_link_swing_l"))
		}
    });
}
#[acmd_script(
    agent = "link",
    script =  "expression_attackairhi",
    category = ACMD_EXPRESSION,
    low_priority)]
unsafe fn link_uair_expr(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        if(is_excute){
            VisibilityModule::set_int64(hash40("shield") as i64, hash40("shield_back") as i64)
            VisibilityModule::set_int64(hash40("sword") as i64, hash40("sword_back") as i64)
            ItemModule::set_have_item_visibility(false, 0)
        }
        frame(Frame=6)
        if(is_excute){
            VisibilityModule::set_int64(hash40("shield") as i64, hash40("shield_back") as i64)
            VisibilityModule::set_int64(hash40("sword") as i64, hash40("sword_back") as i64)
            ItemModule::set_have_item_visibility(false, 0)
        }
        frame(Frame=20)
        if(is_excute){
            VisibilityModule::set_int64(hash40("shield") as i64, hash40("shield_normal") as i64)
            VisibilityModule::set_int64(hash40("sword") as i64, hash40("sword_normal") as i64)
            ItemModule::set_have_item_visibility(true, 0)
        }
    });
}

#[acmd_script(
    agent = "link",
    script =  "game_attackairn",
    category = ACMD_GAME,
	low_priority)]
unsafe fn link_nair(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		frame(Frame=4)
		if(is_excute){
			FT_MOTION_RATE(FSM=0.66)
			WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
		}
		frame(Frame=7)
		if(is_excute){
			FT_MOTION_RATE(FSM=1)
			ATTACK(ID=0, Part=0, Bone=hash40("kneer"), Damage=11.0, Angle=361, KBG=100, FKB=0, BKB=22, Size=4.0, X=3.5, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
			ATTACK(ID=1, Part=0, Bone=hash40("kneer"), Damage=11.0, Angle=361, KBG=100, FKB=0, BKB=22, Size=4.0, X=-2.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
			ATTACK(ID=2, Part=0, Bone=hash40("kneel"), Damage=11.0, Angle=361, KBG=100, FKB=0, BKB=22, Size=2.5, X=1.5, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
		}
		wait(Frames=2)
		if(is_excute){
			ATTACK(ID=0, Part=0, Bone=hash40("kneer"), Damage=6.0, Angle=361, KBG=100, FKB=0, BKB=15, Size=3.5, X=3.5, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
			ATTACK(ID=1, Part=0, Bone=hash40("kneer"), Damage=6.0, Angle=361, KBG=100, FKB=0, BKB=15, Size=3.5, X=-2.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
			ATTACK(ID=2, Part=0, Bone=hash40("kneel"), Damage=6.0, Angle=361, KBG=100, FKB=0, BKB=15, Size=2.0, X=1.5, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
		}
    });
} 	
#[acmd_script(
    agent = "link",
    scripts =  ["game_speciallwblast", "game_specialairlwblast"],
    category = ACMD_GAME,
	low_priority)]
unsafe fn link_downb(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
	if StatusModule::status_kind(fighter.module_accessor) == *FIGHTER_LINK_STATUS_KIND_SPECIAL_LW_BLAST {
		acmd!(lua_state, {
			if(is_excute){
				WorkModule::on_flag(Flag=FIGHTER_LINK_STATUS_WORK_ID_FLAG_BOMB_BLAST)
			}
		});
	} else {
		acmd!(lua_state, {
			frame(Frame=11)
			if(is_excute){
				ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=12.0, Angle=80, KBG=97, FKB=0, BKB=5, Size=6.5, X=0.0, Y=10.0, Z=20.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_paralyze"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_ELEC, Type=ATTACK_REGION_ENERGY)
				ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=8.0, Angle=80, KBG=90, FKB=0, BKB=40, Size=11.5, X=0.0, Y=10.0, Z=20.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.8, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_elec"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_ELEC, Type=ATTACK_REGION_ENERGY)
				ATK_SET_SHIELD_SETOFF_MUL(ID=0, ShieldstunMul=0.1)
				ATK_SET_SHIELD_SETOFF_MUL(ID=1, ShieldstunMul=0.1)
			}
			frame(Frame=16)
			if(is_excute){
				AttackModule::clear_all()
			}
		});
	};
}		
#[acmd_script(
    agent = "link",
    scripts =  ["effect_speciallwblast", "effect_specialairlwblast"],
    category = ACMD_EFFECT,
	low_priority)]
unsafe fn link_downb_eff(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
	if StatusModule::status_kind(fighter.module_accessor) != *FIGHTER_LINK_STATUS_KIND_SPECIAL_LW_BLAST {
			if macros::is_excute(fighter) {
				macros::EFFECT_OFF_KIND(fighter, Hash40::new("sys_smash_flash"), false, true);
			}
			frame(fighter.lua_state_agent, 10.0);
			if macros::is_excute(fighter) {
				let eff: u32 = EffectModule::req_follow(fighter.module_accessor, smash::phx::Hash40::new("link_swordbeam_hit"), smash::phx::Hash40::new("top"), &SHOOT, &NONE, 0.85, true, 0, 0, 0, 0, 0, true, true) as u32;
				EffectModule::set_rgb(fighter.module_accessor, eff, 6.95, 1.5, 0.06);
				EffectModule::set_rate(fighter.module_accessor, eff, 0.65);
			};
	};
}	
#[acmd_script(
    agent = "link",
    scripts =  ["sound_speciallwblast", "sound_specialairlwblast"],
    category = ACMD_SOUND,
	low_priority)]
unsafe fn link_downb_snd(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
	if StatusModule::status_kind(fighter.module_accessor) != *FIGHTER_LINK_STATUS_KIND_SPECIAL_LW_BLAST {
		acmd!(lua_state, {
			frame(Frame=10)
			if(is_excute){
				PLAY_SEQUENCE(hash40("seq_link_rnd_attack"))
				PLAY_STATUS(hash40("se_link_smash_s01"))
				PLAY_STATUS(hash40("se_link_smash_s01"))
				PLAY_STATUS(hash40("se_link_smash_s01"))
			}
		});
	};
}	
#[acmd_script(
    agent = "link_boomerang",
    script =  "game_fly",
    category = ACMD_GAME,
	low_priority)]
unsafe fn linkerang(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		if(is_excute){
			ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=4.0, Angle=70, KBG=40, FKB=0, BKB=80, Size=4.0, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_SPEED, SetWeight=false, ShieldDamage=-4, Trip=0.0, Rehit=0, Reflectable=true, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_OBJECT)
			AttackModule::enable_safe_pos()
		}
		frame(Frame=10)
		if(is_excute){
			ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=3.0, Angle=65, KBG=40, FKB=0, BKB=60, Size=3.6, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_SPEED, SetWeight=false, ShieldDamage=-3, Trip=0.0, Rehit=0, Reflectable=true, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_OBJECT)
		}
    });
}

#[fighter_frame( agent = FIGHTER_KIND_LINK )]
fn link_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent); 
        let status_kind = smash::app::lua_bind::StatusModule::status_kind(boma);
        let ENTRY_ID = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
        //Make sure to put all statuses that use the weapon in here
        if ![*FIGHTER_STATUS_KIND_ATTACK_S3, *FIGHTER_STATUS_KIND_ATTACK_LW4_START, *FIGHTER_STATUS_KIND_ATTACK_LW4, *FIGHTER_STATUS_KIND_ATTACK_LW4_HOLD, *FIGHTER_STATUS_KIND_SPECIAL_N, *FIGHTER_STATUS_KIND_ATTACK_AIR].contains(&status_kind) {
            ArticleModule::change_status_exist(boma, *FIGHTER_LINK_GENERATE_ARTICLE_BOW, *WN_LINK_BOW_STATUS_KIND_BACK);
        };
    }
}

pub fn install() {
    smashline::install_acmd_scripts!(
		link_dtilt,
		link_nair,
		link_bair,
		link_fair,
		link_fair_expr,
		link_fair_snd,
		link_fair_eff,
		link_ftilt,
		link_ftilt_snd,
		link_ftilt_eff,
		link_ftilt_expr,
		link_uair,
		link_uair_snd,
		link_uair_eff,
		link_uair_expr,
		link_arrow_air,
		link_arrow,
		link_dsmash,
		link_dsmash_eff,
		link_dsmash_snd,
		link_dsmash_expr,
		link_dsmash_charge_expr,
		link_utilt,
		link_jab,
		link_jab2,
		linkerang,
		link_downb,
		link_downb_eff,
		link_downb_snd,
		link_landing
    );
	smashline::install_agent_frames!(link_frame);
}
