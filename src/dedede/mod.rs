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

static mut UPB_FALL : [bool; 8] = [false; 8];

#[acmd_script(
    agent = "dedede",
    script =  "game_attackairf",
    category = ACMD_GAME,
	low_priority)]
unsafe fn d3_fair(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		FT_MOTION_RATE(FSM=0.5)
		frame(Frame=5)
		if(is_excute){
			WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
		}
		frame(Frame=6)
		FT_MOTION_RATE(FSM=1)
		frame(Frame=13)
		if(is_excute){
			ATTACK(ID=0, Part=0, Bone=hash40("hammer2"), Damage=12.0, Angle=361, KBG=98, FKB=0, BKB=15, Size=12.0, X=-2.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_dedede_hammer"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_DEDEDE, Type=ATTACK_REGION_HAMMER)
			ATTACK(ID=1, Part=0, Bone=hash40("hammer2"), Damage=12.0, Angle=361, KBG=98, FKB=0, BKB=15, Size=5.0, X=-12.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_dedede_hammer"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_DEDEDE, Type=ATTACK_REGION_HAMMER)
		}
		frame(Frame=18)
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
    agent = "dedede",
    script =  "game_attackairb",
    category = ACMD_GAME,
	low_priority)]
unsafe fn d3_bair(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		frame(Frame=2)
		if(is_excute){
			WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
		}
		frame(Frame=6)
		if(is_excute){
			ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=12.0, Angle=361, KBG=100, FKB=0, BKB=15, Size=9.0, X=0.0, Y=7.5, Z=-12.0, X2=0.0, Y2=7.5, Z2=-7.5, Hitlag=1.2, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_B, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
			ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=12.0, Angle=361, KBG=100, FKB=0, BKB=15, Size=9.0, X=0.0, Y=7.5, Z=-16.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.2, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_B, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
		}
		frame(Frame=9)
		if(is_excute){
			ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=6.0, Angle=361, KBG=70, FKB=0, BKB=50, Size=6.0, X=0.0, Y=7.5, Z=-12.0, X2=0.0, Y2=7.5, Z2=-7.5, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_B, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
			ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=6.0, Angle=361, KBG=70, FKB=0, BKB=50, Size=6.0, X=0.0, Y=7.5, Z=-16.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_B, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
		}
		frame(Frame=12)
		if(is_excute){
			AttackModule::clear_all()
		}
		frame(Frame=20)
		if(is_excute){
			WorkModule::off_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
		}
		frame(Frame=37)
		if(is_excute){
			CancelModule::enable_cancel()
		}
    });
}	
#[acmd_script(
    agent = "dedede",
    script =  "effect_attackairb",
    category = ACMD_EFFECT,
	low_priority)]
unsafe fn d3_bair_eff(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {	
	
    });
}	
#[acmd_script(
    agent = "dedede",
    script =  "expression_landingairb",
    category = ACMD_EXPRESSION,
	low_priority)]
unsafe fn d3_bair_expr(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {	
	
    });
}	
#[acmd_script(
    agent = "dedede",
    script =  "game_throwlw",
    category = ACMD_GAME,
	low_priority)]
unsafe fn d3_dthrow(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		if(is_excute){
			ATTACK_ABS(Kind=FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, ID=0, Damage=6.0, Angle=80, KBG=58, FKB=0, BKB=70, Hitlag=0.0, Unk=1.0, FacingRestrict=ATTACK_LR_CHECK_F, Unk=0.0, Unk=true, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_NONE, Type=ATTACK_REGION_THROW)
			ATTACK_ABS(Kind=FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, ID=0, Damage=3.0, Angle=361, KBG=100, FKB=0, BKB=60, Hitlag=0.0, Unk=1.0, FacingRestrict=ATTACK_LR_CHECK_F, Unk=0.0, Unk=true, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_NONE, Type=ATTACK_REGION_THROW)
		}
		frame(Frame=26)
		if(is_excute){
			ATK_HIT_ABS(FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, hash40("throw"), WorkModule::get_int64(module_accessor,*FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT), WorkModule::get_int64(module_accessor,*FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP), WorkModule::get_int64(module_accessor,*FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO))
			AttackModule::clear_all()
		}
    });
}		
		
#[acmd_script(
    agent = "dedede",
    script =  "game_throwf",
    category = ACMD_GAME,
	low_priority)]
unsafe fn d3_fthrow(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		if(is_excute){
			ATTACK_ABS(Kind=FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, ID=0, Damage=6.0, Angle=35, KBG=92, FKB=0, BKB=70, Hitlag=0.0, Unk=1.0, FacingRestrict=ATTACK_LR_CHECK_F, Unk=0.0, Unk=true, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_NONE, Type=ATTACK_REGION_THROW)
			ATTACK_ABS(Kind=FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, ID=0, Damage=3.0, Angle=361, KBG=100, FKB=0, BKB=60, Hitlag=0.0, Unk=1.0, FacingRestrict=ATTACK_LR_CHECK_F, Unk=0.0, Unk=true, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_NONE, Type=ATTACK_REGION_THROW)
		}
		frame(Frame=12)
		if(is_excute){
			ATTACK(ID=0, Part=0, Bone=hash40("hammer1"), Damage=4.0, Angle=361, KBG=150, FKB=0, BKB=30, Size=6.0, X=17.0, Y=0.0, Z=-3.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_dedede_hammer"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_DEDEDE, Type=ATTACK_REGION_HAMMER)
			ATTACK(ID=1, Part=0, Bone=hash40("hammer1"), Damage=4.0, Angle=361, KBG=150, FKB=0, BKB=30, Size=6.0, X=17.0, Y=0.0, Z=6.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_dedede_hammer"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_DEDEDE, Type=ATTACK_REGION_HAMMER)
			ATTACK(ID=2, Part=0, Bone=hash40("hammer1"), Damage=4.0, Angle=361, KBG=150, FKB=0, BKB=30, Size=4.0, X=9.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_dedede_hammer"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_DEDEDE, Type=ATTACK_REGION_HAMMER)
			ATTACK(ID=3, Part=0, Bone=hash40("hammer1"), Damage=4.0, Angle=361, KBG=150, FKB=0, BKB=30, Size=4.0, X=3.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_dedede_hammer"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_DEDEDE, Type=ATTACK_REGION_HAMMER)
		}
		frame(Frame=14)
		if(is_excute){
			ATK_HIT_ABS(FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, hash40("throw"), WorkModule::get_int64(module_accessor,*FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT), WorkModule::get_int64(module_accessor,*FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP), WorkModule::get_int64(module_accessor,*FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO))
			AttackModule::clear_all()
		}
    });
}		
#[acmd_script(
    agent = "dedede",
    script =  "game_attackhi3",
    category = ACMD_GAME,
	low_priority)]
unsafe fn d3_utilt(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		frame(Frame=7)
		if(is_excute){
			HIT_NODE(hash40("head"), HIT_STATUS_XLU)
			HIT_NODE(hash40("shoulderl"), HIT_STATUS_XLU)
			HIT_NODE(hash40("arml"), HIT_STATUS_XLU)
			ATTACK(ID=0, Part=0, Bone=hash40("head"), Damage=11.5, Angle=96, KBG=89, FKB=0, BKB=65, Size=6.0, X=-1.5, Y=1.0, Z=1.1, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_HEAD)
			ATTACK(ID=1, Part=0, Bone=hash40("head"), Damage=9.5, Angle=96, KBG=89, FKB=0, BKB=65, Size=6.5, X=4.8, Y=0.9, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_HEAD)
		}
		frame(Frame=8)
		frame(Frame=14)
		if(is_excute){
			HIT_NODE(hash40("head"), HIT_STATUS_NORMAL)
			HIT_NODE(hash40("shoulderl"), HIT_STATUS_NORMAL)
			HIT_NODE(hash40("arml"), HIT_STATUS_NORMAL)
			AttackModule::clear_all()
		}
    });
}		
		
#[acmd_script(
    agent = "dedede",
    script =  "game_attacks3",
    category = ACMD_GAME,
	low_priority)]
unsafe fn d3_ftilt(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		FT_MOTION_RATE(FSM=0.5)
		frame(Frame=12)
		FT_MOTION_RATE(FSM=1)
		if(is_excute){
			HIT_NODE(hash40("arml"), HIT_STATUS_XLU)
			HIT_NODE(hash40("armr"), HIT_STATUS_XLU)
			HIT_NODE(hash40("shoulderl"), HIT_STATUS_XLU)
			HIT_NODE(hash40("shoulderr"), HIT_STATUS_XLU)
			ATTACK(ID=0, Part=0, Bone=hash40("hammer1"), Damage=2.0, Angle=22, KBG=100, FKB=60, BKB=0, Size=3.0, X=-8.0, Y=0.0, Z=0.0, X2=-2.0, Y2=0.0, Z2=0.0, Hitlag=0.9, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=4, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_G, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_rush"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_DEDEDE, Type=ATTACK_REGION_HAMMER)
			ATTACK(ID=1, Part=0, Bone=hash40("hammer1"), Damage=2.0, Angle=22, KBG=100, FKB=50, BKB=0, Size=3.5, X=1.0, Y=0.0, Z=0.0, X2=8.0, Y2=0.0, Z2=0.0, Hitlag=0.9, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=4, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_rush"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_DEDEDE, Type=ATTACK_REGION_HAMMER)
			ATTACK(ID=2, Part=0, Bone=hash40("top"), Damage=2.0, Angle=100, KBG=100, FKB=30, BKB=0, Size=3.0, X=0.0, Y=4.0, Z=19.2, X2=0.0, Y2=4.0, Z2=25.8, Hitlag=0.9, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=3, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_rush"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_DEDEDE, Type=ATTACK_REGION_HAMMER)
			ATTACK(ID=3, Part=0, Bone=hash40("top"), Damage=2.0, Angle=250, KBG=100, FKB=26, BKB=0, Size=4.0, X=0.0, Y=9.5, Z=20.0, X2=0.0, Y2=9.5, Z2=25.0, Hitlag=0.9, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=3, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_rush"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_DEDEDE, Type=ATTACK_REGION_HAMMER)
		}
		frame(Frame=22)
		if(is_excute){
			AttackModule::clear_all()
		}
		frame(Frame=23)
		if(is_excute){
			ATTACK(ID=0, Part=0, Bone=hash40("hammer1"), Damage=3.0, Angle=50, KBG=190, FKB=0, BKB=30, Size=3.5, X=-8.0, Y=0.0, Z=0.0, X2=-2.0, Y2=0.0, Z2=0.0, Hitlag=2.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_G, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_dedede_hammer"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_DEDEDE, Type=ATTACK_REGION_HAMMER)
			ATTACK(ID=1, Part=0, Bone=hash40("hammer1"), Damage=3.0, Angle=50, KBG=190, FKB=0, BKB=30, Size=3.5, X=1.0, Y=0.0, Z=0.0, X2=8.0, Y2=0.0, Z2=0.0, Hitlag=2.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_dedede_hammer"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_DEDEDE, Type=ATTACK_REGION_HAMMER)
			ATTACK(ID=2, Part=0, Bone=hash40("hammer2"), Damage=3.0, Angle=50, KBG=190, FKB=0, BKB=30, Size=8.3, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=2.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_dedede_hammer"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_DEDEDE, Type=ATTACK_REGION_HAMMER)
		}
		frame(Frame=24)
		if(is_excute){
			HIT_NODE(hash40("arml"), HIT_STATUS_NORMAL)
			HIT_NODE(hash40("armr"), HIT_STATUS_NORMAL)
			HIT_NODE(hash40("shoulderl"), HIT_STATUS_NORMAL)
			HIT_NODE(hash40("shoulderr"), HIT_STATUS_NORMAL)
			AttackModule::clear_all()
		}
    });
}

#[acmd_script(
    agent = "dedede",
    script =  "game_attacklw3",
    category = ACMD_GAME,
	low_priority)]
unsafe fn d3_dtilt(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		frame(Frame=15)
		if(is_excute){
			ATTACK(ID=1, Part=0, Bone=hash40("footl"), Damage=7.0, Angle=78, KBG=80, FKB=0, BKB=45, Size=5.0, X=-1.5, Y=1.0, Z=1.1, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
			ATTACK(ID=0, Part=0, Bone=hash40("toel"), Damage=8.0, Angle=78, KBG=89, FKB=0, BKB=45, Size=6.5, X=-1.5, Y=1.0, Z=1.1, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
		
		}
		frame(Frame=20)
		if(is_excute){
			AttackModule::clear_all()
		}
    });
}
#[acmd_script(
    agent = "dedede",
    script =  "effect_attacklw3",
    category = ACMD_EFFECT,
	low_priority)]
unsafe fn d3_dtilt_eff(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {	
	
    });
}	
#[acmd_script(
    agent = "dedede",
    script =  "expression_attacklw3",
    category = ACMD_EXPRESSION,
	low_priority)]
unsafe fn d3_dtilt_exp(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {	
	
    });
}	

#[acmd_script(
    agent = "dedede",
    script =  "game_attackdash",
    category = ACMD_GAME,
	low_priority)]
unsafe fn d3_da(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		frame(Frame=6)
		if(is_excute){
			ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=10.0, Angle=35, KBG=85, FKB=0, BKB=38, Size=7.0, X=0.0, Y=6.5, Z=12.0, X2=0.0, Y2=6.5, Z2=6.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.4, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_BODY)
		}
		wait(Frames=2)
		if(is_excute){
			ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=7.0, Angle=35, KBG=85, FKB=0, BKB=35, Size=5.0, X=0.0, Y=5.0, Z=12.0, X2=0.0, Y2=5.0, Z2=8.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.4, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_BODY)
		}
		wait(Frames=4)
		if(is_excute){
			AttackModule::clear_all()
		}
		frame(Frame=22)
		if(is_excute){
			ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=7.0, Angle=35, KBG=75, FKB=0, BKB=60, Size=6.5, X=0.0, Y=6.5, Z=12.0, X2=0.0, Y2=6.5, Z2=6.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.4, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_BODY)
		}
		frame(Frame=28)
		if(is_excute){
			AttackModule::clear_all()
		}
    });
}
#[acmd_script(
    agent = "dedede",
    script =  "effect_attackdash",
    category = ACMD_EFFECT,
	low_priority)]
unsafe fn d3_da_eff(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {	
		frame(Frame=4)
		if(is_excute){
			EFFECT_FOLLOW(hash40("sys_spin_wind"), hash40("top"), 0, 8, 0, 0, 0, -90, 1.2, true)
		}
		frame(Frame=6)
		if(is_excute){
			FOOT_EFFECT(hash40("sys_turn_smoke"), hash40("top"), -5, 0, 0, 0, 0, 0, 1.4, 0, 0, 0, 0, 0, 0, false)
		}
		frame(Frame=13)
		if(is_excute){
			EFFECT_FOLLOW_ALPHA(hash40("sys_spin_wind_s"), hash40("top"), 0, 8, 0, 0, 0, -90, 1.4, false, 0.15)
		}
		frame(Frame=16)
		if(is_excute){
			FOOT_EFFECT(hash40("sys_turn_smoke"), hash40("top"), -3, 0, 0, 0, 0, 0, 1.3, 0, 0, 0, 0, 0, 0, false)
		}
		frame(Frame=24)
		if(is_excute){
			FOOT_EFFECT(hash40("sys_turn_smoke"), hash40("top"), -3, 0, 0, 0, 0, 0, 1.1, 0, 0, 0, 0, 0, 0, false)
		}
    });
}	
#[acmd_script(
    agent = "dedede",
    script =  "sound_attackdash",
    category = ACMD_EFFECT,
	low_priority)]
unsafe fn d3_da_snd(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {	
		frame(Frame=5)
		if(is_excute){
			PLAY_SE(hash40("se_dedede_attackhard_l01"))
			PLAY_SEQUENCE(hash40("seq_dedede_rnd_attack01"))
		}
    });
}	
#[acmd_script(
    agent = "dedede",
    script =  "expression_attackdash",
    category = ACMD_EXPRESSION,
	low_priority)]
unsafe fn d3_da_exp(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {	
	
    });
}	

#[acmd_script(
    agent = "dedede",
    script =  "game_specialinput",
    category = ACMD_GAME,
	low_priority)]
unsafe fn d3_special_input(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		frame(Frame=26)
		if(is_excute){
			ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=16.0, Angle=361, KBG=92, FKB=0, BKB=65, Size=7.0, X=0.0, Y=5.0, Z=11.8, X2=0.0, Y2=5.0, Z2=5.0, Hitlag=1.2, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=3, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_LL, SFXType=COLLISION_SOUND_ATTR_DEDEDE, Type=ATTACK_REGION_BODY)
		}
		frame(Frame=28)
		if(is_excute){
			ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=13.0, Angle=361, KBG=92, FKB=0, BKB=55, Size=6.0, X=0.0, Y=4.0, Z=11.8, X2=0.0, Y2=4.0, Z2=5.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=3, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_DEDEDE, Type=ATTACK_REGION_BODY)
		}
		frame(Frame=36)
		if(is_excute){
			ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=13.0, Angle=361, KBG=92, FKB=0, BKB=55, Size=4.5, X=0.0, Y=3.5, Z=10.5, X2=0.0, Y2=3.5, Z2=5.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=3, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_DEDEDE, Type=ATTACK_REGION_BODY)
		}
		frame(Frame=39)
		if(is_excute){
			ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=13.0, Angle=361, KBG=92, FKB=0, BKB=55, Size=3.0, X=0.0, Y=3.0, Z=9.5, X2=0.0, Y2=3.0, Z2=5.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=3, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_DEDEDE, Type=ATTACK_REGION_BODY)
		}
		frame(Frame=42)
		if(is_excute){
			AttackModule::clear_all()
		}
    });
}
#[acmd_script(
    agent = "dedede",
    script =  "effect_specialinput",
    category = ACMD_EFFECT,
	low_priority)]
unsafe fn d3_special_input_eff(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		frame(Frame=9)
		if(is_excute){
			LANDING_EFFECT(hash40("sys_atk_smoke"), hash40("top"), -9, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false)
		}
		frame(Frame=26)
		if(is_excute){
			LANDING_EFFECT(hash40("sys_landing_smoke"), hash40("top"), 10, 0, 0, 0, 0, 0, 1.2, 0, 0, 0, 0, 0, 0, false)
			QUAKE(CAMERA_QUAKE_KIND_M)
		}
    });
}
#[acmd_script(
    agent = "dedede",
    script =  "sound_specialinput",
    category = ACMD_SOUND,
	low_priority)]
unsafe fn d3_special_input_snd(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		frame(Frame=5)
		if(is_excute){
			PLAY_SE(hash40("se_dedede_hammer_swing_m"))
		}
		wait(Frames=1)
		if(is_excute){
			PLAY_SE(hash40("se_dedede_attackdash_02"))
		}
		frame(Frame=26)
		if(is_excute){
			PLAY_SE(hash40("se_dedede_attackdash"))
		}
		frame(Frame=65)
		if(is_excute){
			PLAY_SE(hash40("se_dedede_step_right_m"))
		}
		frame(Frame=76)
		if(is_excute){
			PLAY_SE(hash40("se_dedede_step_left_m"))
		}
    });
}

#[acmd_script(
    agent = "dedede",
    script =  "game_specialairinput",
    category = ACMD_GAME,
	low_priority)]
unsafe fn d3_special_air_input(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		frame(Frame=20)
		if(is_excute){
			ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=16.0, Angle=260, KBG=92, FKB=0, BKB=65, Size=6.0, X=0.0, Y=5.0, Z=7.5, X2=0.0, Y2=5.0, Z2=5.0, Hitlag=1.2, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=3, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_LL, SFXType=COLLISION_SOUND_ATTR_DEDEDE, Type=ATTACK_REGION_BODY)
		}
		wait(Frames=3)
		if(is_excute){
			AttackModule::clear_all()
		}
    });
}
#[acmd_script(
    agent = "dedede",
    script =  "sound_specialairinput",
    category = ACMD_SOUND,
	low_priority)]
unsafe fn d3_special_air_input_snd(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		frame(Frame=5)
		if(is_excute){
			PLAY_SE(hash40("se_dedede_hammer_swing_m"))
		}
		wait(Frames=1)
		if(is_excute){
			PLAY_SE(hash40("se_dedede_attackdash_02"))
		}
		frame(Frame=20)
		if(is_excute){
			PLAY_SE(hash40("se_dedede_attackdash"))
		}
    });
}

#[acmd_script(
    agent = "dedede",
    script =  "game_specialairinput2",
    category = ACMD_GAME,
	low_priority)]
unsafe fn d3_special_air_input2(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		frame(Frame=1)
		if(is_excute){
			ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=10.0, Angle=361, KBG=85, FKB=0, BKB=70, Size=5.5, X=0.0, Y=5.0, Z=7.5, X2=0.0, Y2=5.0, Z2=5.0, Hitlag=1.2, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=3, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_LL, SFXType=COLLISION_SOUND_ATTR_DEDEDE, Type=ATTACK_REGION_BODY)
		}
    });
}
#[acmd_script(
    agent = "dedede",
    script =  "sound_specialairinput2",
    category = ACMD_SOUND,
	low_priority)]
unsafe fn d3_special_air_input2_snd(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		frame(Frame=1)
		if(is_excute){
			PLAY_SE(hash40("se_dedede_attackdash"))
		}
    });
}

#[acmd_script(
    agent = "dedede",
    script =  "game_specialairinputlanding",
    category = ACMD_GAME,
	low_priority)]
unsafe fn d3_special_air_input_land(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		if(is_excute){
			WorkModule::on_flag(Flag=FIGHTER_STATUS_SUPER_JUMP_PUNCH_FLAG_MOVE_TRANS)
		}
		frame(Frame=1)
		if(is_excute){
			ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=5.0, Angle=361, KBG=75, FKB=0, BKB=70, Size=6.0, X=0.0, Y=5.0, Z=10.0, X2=0.0, Y2=5.0, Z2=5.0, Hitlag=1.2, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=3, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_LL, SFXType=COLLISION_SOUND_ATTR_DEDEDE, Type=ATTACK_REGION_BODY)
		}
		wait(Frames=7)
		if(is_excute){
			AttackModule::clear_all()
		}
    });
}
#[acmd_script(
    agent = "dedede",
    script =  "sound_specialairinputlanding",
    category = ACMD_SOUND,
	low_priority)]
unsafe fn d3_special_air_input_land_snd(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		frame(Frame=1)
		if(is_excute){
			PLAY_SE(hash40("se_dedede_attackdash"))
		}
    });
}
#[acmd_script(
    agent = "dedede",
    script =  "effect_specialairinputlanding",
    category = ACMD_EFFECT,
	low_priority)]
unsafe fn d3_special_air_input_land_eff(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		frame(Frame=1)
		if(is_excute){
			LANDING_EFFECT(hash40("sys_landing_smoke"), hash40("top"), 10, 0, 0, 0, 0, 0, 1.2, 0, 0, 0, 0, 0, 0, false)
			QUAKE(CAMERA_QUAKE_KIND_M)
		}
    });
}

#[fighter_frame(agent = FIGHTER_KIND_DEDEDE)]
pub fn dedede_frame(fighter : &mut L2CFighterCommon) {
    unsafe {
		let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent); 
		let status_kind = smash::app::lua_bind::StatusModule::status_kind(boma);
		let ENTRY_ID = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
		let fighter_kind = smash::app::utility::get_kind(boma);

		if [hash40("attack_lw3")].contains(&MotionModule::motion_kind(boma)) {
			if MotionModule::frame(boma) >= 47.0 {
				StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_WAIT, true);
			};
		};
		if [hash40("attack_dash")].contains(&MotionModule::motion_kind(boma)) {
			if MotionModule::frame(boma) >= 65.0 {
				StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_WAIT, true);
			};
		};
		if [hash40("special_input")].contains(&MotionModule::motion_kind(boma)) {
			if MotionModule::frame(boma) >= 79.0 {
				StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_WAIT, true);
			};
			if KineticModule::get_kinetic_type(boma) != *FIGHTER_KINETIC_TYPE_MOTION {
				KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_MOTION);
			};
		};
		if [hash40("special_air_input"), hash40("special_air_input_2")].contains(&MotionModule::motion_kind(boma)) {
			macros::SET_SPEED_EX(fighter,0.0, 0.0, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
			KineticModule::suspend_energy(boma, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
		};
		if [hash40("special_air_input")].contains(&MotionModule::motion_kind(boma)) {
			if MotionModule::frame(boma) >= 24.0 {
				MotionModule::change_motion(boma, smash::phx::Hash40::new("special_air_input_2"), 0.0, 1.0, false, 0.0, false, false);
			};
		};
		if [hash40("special_air_input_2")].contains(&MotionModule::motion_kind(boma)) {
			//MotionModule::set_rate(boma, 0.0);
			if MotionModule::frame(boma) >= 0.0 {
				macros::SET_SPEED_EX(fighter,2.0, -3.7, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
			};
			UPB_FALL[ENTRY_ID] = true;
			let stick_y = ControlModule::get_stick_y(boma);
			if stick_y <= -0.5 {
				GroundModule::pass_floor(boma);
				if ray_check_pos(boma, 0.0, -0.1, false) == 1 {
					//KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_MOTION);
					MotionModule::change_motion(boma, smash::phx::Hash40::new("special_air_input_landing"), 0.0, 1.0, false, 0.0, false, false);
					StatusModule::set_situation_kind(boma, smash::app::SituationKind(*SITUATION_KIND_GROUND), true);
					KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_MOTION);
				};
			}else {
				GroundModule::clear_pass_floor(boma);
				if ray_check_pos(boma, 0.0, -0.1, true) == 1 {
					//KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_MOTION);
					MotionModule::change_motion(boma, smash::phx::Hash40::new("special_air_input_landing"), 0.0, 1.0, false, 0.0, false, false);
					StatusModule::set_situation_kind(boma, smash::app::SituationKind(*SITUATION_KIND_GROUND), true);
					KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_MOTION);
				};
			};
		};
		if [hash40("special_air_input_landing")].contains(&MotionModule::motion_kind(boma)) {
			if MotionModule::frame(boma) >= 55.0 {
				StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_WAIT, true);
			};
		};
	};
}

pub fn install() {
    smashline::install_acmd_scripts!(
		d3_fair,
		d3_ftilt,
		d3_utilt,
		d3_dthrow,
		d3_fthrow,
		d3_bair,
		d3_bair_eff,
		d3_bair_expr,
		d3_dtilt,
		d3_dtilt_eff,
		d3_dtilt_exp,
		d3_da,
		d3_da_eff,
		d3_da_snd,
		d3_da_exp,
		d3_special_input,
		d3_special_input_eff,
		d3_special_input_snd,
		d3_special_air_input,
		d3_special_air_input_snd,
		d3_special_air_input2,
		d3_special_air_input2_snd,
		d3_special_air_input_land,
		d3_special_air_input_land_snd,
		d3_special_air_input_land_eff
    );
	smashline::install_agent_frames!(dedede_frame);
}
