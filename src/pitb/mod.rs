use smash::hash40;
use smash::phx::Hash40;
use smash::lib::lua_const::*;
use smash::app::*;
use smash::app::lua_bind::*;
use smash::lua2cpp::{L2CFighterCommon, L2CAgentBase};
use smashline::*;
use smash_script::*;

use crate::util::*;
//Float Stuff
static mut CHECK_FLOAT : [i32; 8] = [0; 8];
static mut CHECK_FLOAT_MAX : i32 = 14; //Frames where jump needs to be held to start floating
static mut FLOAT_FALLSPEED : f32 = 0.365;
static mut HITLAG_MULT : f32 = 0.45;


#[acmd_script(
    agent = "pitb",
    script =  "game_attackairhi",
    category = ACMD_GAME,
	low_priority)]
unsafe fn dpit_uair(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		frame(Frame=1)
		FT_MOTION_RATE(FSM=0.67)
		frame(Frame=4)
		if(is_excute){
			WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
		}
		frame(Frame=9)
		FT_MOTION_RATE(FSM=1)
		if(is_excute){
			ATTACK(ID=0, Part=0, Bone=hash40("kneer"), Damage=9.0, Angle=80, KBG=40, FKB=0, BKB=90, Size=4.0, X=6.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
			ATTACK(ID=1, Part=0, Bone=hash40("kneer"), Damage=9.0, Angle=80, KBG=40, FKB=0, BKB=90, Size=4.0, X=0.5, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
			ATTACK(ID=2, Part=0, Bone=hash40("hip"), Damage=9.0, Angle=80, KBG=40, FKB=0, BKB=90, Size=4.0, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
		}
		wait(Frames=5)
		if(is_excute){
			AttackModule::clear_all()
		}
		frame(Frame=22)
		if(is_excute){
			WorkModule::off_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
		}
	});
}
#[acmd_script(
    agent = "pitb",
    script =  "effect_attackairhi",
    category = ACMD_EFFECT,
	low_priority)]
unsafe fn dpit_uair_eff(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		frame(Frame=9)
		if(is_excute){
			EFFECT_FOLLOW(hash40("sys_attack_arc_d"), hash40("top"), 0, 11, 0, 180, -70, 90, 1, true)
			LAST_EFFECT_SET_RATE(2)
		}
		frame(Frame=11)
		if(is_excute){
			EFFECT_FOLLOW(hash40("sys_attack_arc_d"), hash40("top"), 0, 11, 0, 180, -140, 90, 1, true)
			LAST_EFFECT_SET_RATE(2)
		}
	});
}
#[acmd_script(
    agent = "pitb",
    script =  "sound_attackairhi",
    category = ACMD_SOUND,
	low_priority)]
unsafe fn dpit_uair_snd(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		frame(Frame=8)
		if(is_excute){
			PLAY_SE(hash40("se_common_punch_kick_swing_s"))
		}
	});
}
#[acmd_script(
    agent = "pitb",
    script =  "game_attackairf",
    category = ACMD_GAME,
	low_priority)]
unsafe fn dpit_fair(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		FT_MOTION_RATE(FSM=0.5)
		if(is_excute){
			WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
		}
		frame(Frame=8)
		FT_MOTION_RATE(FSM=1)
		frame(Frame=11)
		FT_MOTION_RATE(FSM=0.375)
		if(is_excute){
			ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=6.0, Angle=55, KBG=138, FKB=0, BKB=25, Size=4.2, X=0.0, Y=5.0, Z=11.0, X2=0.0, Y2=5.0, Z2=19.0, Hitlag=1.1, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_PALUTENA)
		}
		frame(Frames=19)
		FT_MOTION_RATE(FSM=1)
		frame(Frames=20)
		if(is_excute){
			AttackModule::clear_all()
		}
		frame(Frame=28)
		if(is_excute){
			WorkModule::off_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
		}
		frame(Frame=42)
		if(is_excute){
			CancelModule::enable_cancel()
		}
    });
}
#[acmd_script(
    agent = "pitb",
    script =  "game_attack12",
    category = ACMD_GAME,
	low_priority)]
unsafe fn dpit_jab2(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		frame(Frame=5)
		if(is_excute){
			ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=2.0, Angle=361, KBG=20, FKB=0, BKB=28, Size=2.5, X=0.0, Y=8.0, Z=10.5, X2=0.0, Y2=8.0, Z2=10.0, Hitlag=1.2, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_PALUTENA)
			ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=2.0, Angle=361, KBG=20, FKB=0, BKB=28, Size=3.2, X=0.0, Y=8.0, Z=13.6, X2=0.0, Y2=8.0, Z2=10.0, Hitlag=1.2, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_PALUTENA)
			ATTACK(ID=2, Part=0, Bone=hash40("top"), Damage=2.0, Angle=180, KBG=15, FKB=0, BKB=20, Size=3.2, X=0.0, Y=8.0, Z=16.8, X2=0.0, Y2=8.0, Z2=10.0, Hitlag=1.2, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_FIGHTER, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_PALUTENA)
			ATTACK(ID=3, Part=0, Bone=hash40("top"), Damage=2.0, Angle=361, KBG=15, FKB=0, BKB=20, Size=3.2, X=0.0, Y=8.0, Z=16.8, X2=0.0, Y2=8.0, Z2=10.0, Hitlag=1.2, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_PALUTENA)
			AttackModule::set_add_reaction_frame(ID=0, Frames=2.0, Unk=false)
			AttackModule::set_add_reaction_frame(ID=1, Frames=2.0, Unk=false)
			AttackModule::set_add_reaction_frame(ID=2, Frames=2.0, Unk=false)
			AttackModule::set_add_reaction_frame(ID=3, Frames=2.0, Unk=false)
		}
		wait(Frames=2)
		if(is_excute){
			AttackModule::clear_all()
			WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO)
		}
		frame(Frame=16)
		if(is_excute){
			WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_FLAG_ENABLE_NO_HIT_COMBO)
		}
    });
}	
#[acmd_script(
    agent = "pitb",
    script =  "game_attacklw3",
    category = ACMD_GAME,
	low_priority)]
unsafe fn dpit_dtilt(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		frame(Frame=6)
		if(is_excute){
			ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=5.5, Angle=20, KBG=97, FKB=0, BKB=35, Size=2.5, X=0.0, Y=1.5, Z=17.5, X2=0.0, Y2=5.0, Z2=7.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.2, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_PALUTENA)
		}
		wait(Frames=2)
		if(is_excute){
			AttackModule::clear_all()
		}
		FT_MOTION_RATE(FSM=0.8)
    });
}	
#[acmd_script(
    agent = "pitb",
    script =  "game_attackdash",
    category = ACMD_GAME,
	low_priority)]
unsafe fn dpit_da(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		FT_MOTION_RATE(FSM=1.2857)
		frame(Frame=7)
		FT_MOTION_RATE(FSM=1)
		if(is_excute){
			ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=11.0, Angle=60, KBG=74, FKB=0, BKB=80, Size=3.5, X=0.0, Y=4.5, Z=16.0, X2=0.0, Y2=7.0, Z2=7.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_PALUTENA)
			ATK_SET_SHIELD_SETOFF_MUL(ID=0, ShieldstunMul=1.37)
		}
		wait(Frames=3)
		if(is_excute){
			AttackModule::clear_all()
		}
    });
}	
#[acmd_script(
    agent = "pitb",
    script =  "game_attacks4",
    category = ACMD_GAME,
	low_priority)]
unsafe fn dpit_fsmash(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		FT_MOTION_RATE(FSM=1.5)
		frame(Frame=4)
		if(is_excute){
			WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD)
		}
		frame(Frame=10)
		FT_MOTION_RATE(FSM=1)
		if(is_excute){
			ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=7.5, Angle=100, KBG=100, FKB=20, BKB=0, Size=6.0, X=0.0, Y=7.0, Z=12.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_G, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_PALUTENA)
			ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=7.5, Angle=70, KBG=100, FKB=20, BKB=0, Size=6.0, X=0.0, Y=7.0, Z=12.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_A, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_PALUTENA)
			ATTACK(ID=2, Part=0, Bone=hash40("top"), Damage=7.5, Angle=35, KBG=100, FKB=53, BKB=0, Size=6.0, X=0.0, Y=7.0, Z=12.0, X2=0.0, Y2=7.0, Z2=5.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_G, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_PALUTENA)
			ATTACK(ID=3, Part=0, Bone=hash40("top"), Damage=7.5, Angle=0, KBG=100, FKB=15, BKB=0, Size=6.0, X=0.0, Y=7.0, Z=12.0, X2=0.0, Y2=7.0, Z2=5.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_A, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_PALUTENA)
		}
		frame(Frame=11)
		if(is_excute){
			AttackModule::clear_all()
		}
		frame(Frame=21)
		if(is_excute){
			ATTACK(ID=0, Part=1, Bone=hash40("top"), Damage=7.5, Angle=361, KBG=122, FKB=0, BKB=42, Size=6.0, X=0.0, Y=7.5, Z=13.5, X2=0.0, Y2=7.5, Z2=6.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_PALUTENA)
		}
		frame(Frame=23)
		if(is_excute){
			AttackModule::clear_all()
		}
    });
}
#[acmd_script(
    agent = "pitb",
    script =  "game_attackairlw",
    category = ACMD_GAME,
	low_priority)]
unsafe fn dpit_dair(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		frame(Frame=7)
		if(is_excute){
			WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
		}
		frame(Frame=10)
		if(is_excute){
			ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=10.0, Angle=60, KBG=80, FKB=0, BKB=40, Size=4.0, X=0.0, Y=-4.0, Z=0.0, X2=0.0, Y2=-6.0, Z2=0.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_G, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_PALUTENA)
			ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=10.0, Angle=60, KBG=80, FKB=0, BKB=40, Size=4.0, X=0.0, Y=-4.0, Z=0.0, X2=0.0, Y2=-6.0, Z2=0.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_A, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_PALUTENA)
			ATTACK(ID=2, Part=0, Bone=hash40("top"), Damage=10.0, Angle=60, KBG=80, FKB=0, BKB=40, Size=6.0, X=0.0, Y=0.0, Z=-8.8, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_PALUTENA)
			AttackModule::set_add_reaction_frame(ID=1, Frames=2.0, Unk=false)
		}
		wait(Frames=1)
		if(is_excute){
			ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=10.0, Angle=80, KBG=80, FKB=0, BKB=50, Size=6.0, X=0.0, Y=-4.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_PALUTENA)
			ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=10.0, Angle=70, KBG=80, FKB=0, BKB=40, Size=6.0, X=0.0, Y=0.0, Z=8.8, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_PALUTENA)
			AttackModule::set_add_reaction_frame(ID=1, Frames=2.0, Unk=false)
		}
		wait(Frames=2)
		if(is_excute){
			AttackModule::clear_all()
		}
		frame(Frame=36)
		if(is_excute){
			WorkModule::off_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
		}
    });
}	
#[acmd_script(
    agent = "pitb",
    script =  "game_attackairn",
    category = ACMD_GAME,
	low_priority)]
unsafe fn dpit_nair(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		frame(Frame=4)
		if(is_excute){
			WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
		}
		frame(Frame=7)
		if(is_excute){
			ATTACK(ID=0, Part=0, Bone=hash40("legr"), Damage=4.0, Angle=367, KBG=25, FKB=0, BKB=50, Size=6.0, X=3.2, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_A, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
			ATTACK(ID=1, Part=0, Bone=hash40("legr"), Damage=4.0, Angle=68, KBG=75, FKB=0, BKB=65, Size=6.0, X=3.2, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.4, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_G, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
			ATTACK(ID=2, Part=0, Bone=hash40("kneer"), Damage=4.0, Angle=367, KBG=25, FKB=0, BKB=50, Size=4.8, X=4.2, Y=-0.7, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_A, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
			ATTACK(ID=3, Part=0, Bone=hash40("kneer"), Damage=4.0, Angle=74, KBG=75, FKB=0, BKB=65, Size=4.8, X=4.2, Y=-0.7, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.4, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_G, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
			ATTACK(ID=4, Part=0, Bone=hash40("hip"), Damage=4.0, Angle=68, KBG=25, FKB=0, BKB=50, Size=3.5, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
		}
		wait(Frames=2)
		if(is_excute){
			AttackModule::clear_all()
		}
		frame(Frame=13)
		if(is_excute){
			ATTACK(ID=0, Part=0, Bone=hash40("kneel"), Damage=6.0, Angle=70, KBG=120, FKB=0, BKB=45, Size=5.2, X=4.4, Y=-0.7, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
			ATTACK(ID=1, Part=0, Bone=hash40("legl"), Damage=6.0, Angle=70, KBG=120, FKB=0, BKB=45, Size=6.0, X=3.2, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
			ATTACK(ID=2, Part=0, Bone=hash40("hip"), Damage=6.0, Angle=70, KBG=120, FKB=0, BKB=45, Size=3.8, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
		}
		wait(Frames=3)
		if(is_excute){
			AttackModule::clear_all()
		}
		frame(Frame=32)
		if(is_excute){
			WorkModule::off_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
		}
    });
}		
#[acmd_script(
    agent = "pitb",
    script =  "effect_attackairn",
    category = ACMD_EFFECT,
	low_priority)]
unsafe fn dpit_nair_eff(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		frame(Frame=6)
		if(is_excute){
			EFFECT_FOLLOW_ALPHA(hash40("sys_attack_arc"), hash40("top"), 0, 2.5, 2, -148, -141, -3, 1.1, true, 0.7)
			LAST_EFFECT_SET_RATE(1.5)
		}
		frame(Frame=12)
		if(is_excute){
			EFFECT_FOLLOW(hash40("sys_attack_line_b"), hash40("top"), 0, 6, -5, -17, 5, 0, 1.1, true)
			LAST_EFFECT_SET_RATE(2)
		}
		frame(Frame=14)
		if(is_excute){
			EFFECT_ALPHA(hash40("sys_attack_impact"), hash40("top"), 0, 11, 14, 0, 0, 0, 1, 0, 0, 0, 0, 0, 360, true, 0.5)
		}
    });
}		
#[acmd_script(
    agent = "pitb",
    script =  "sound_attackairn",
    category = ACMD_SOUND,
	low_priority)]
unsafe fn dpit_nair_snd(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		frame(Frame=6)
		if(is_excute){
			PLAY_SEQUENCE(hash40("seq_pitb_rnd_attack"))
			PLAY_SE(hash40("se_common_punch_kick_swing_l"))
		}
		frame(Frame=12)
		if(is_excute){
			PLAY_SE(hash40("se_common_punch_kick_swing_l"))
		}
    });
}		
#[acmd_script(
    agent = "pitb_bowarrow",
    script =  "game_fly",
    category = ACMD_GAME,
	low_priority)]
unsafe fn dpit_arrow(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		if(is_excute){
			ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=1.0, Angle=75, KBG=60, FKB=0, BKB=40, Size=1.3, X=0.0, Y=0.0, Z=-1.5, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_SPEED, SetWeight=false, ShieldDamage=-2.2, Trip=0.0, Rehit=0, Reflectable=true, Absorbable=true, Flinchless=false, DisableHitlag=false, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_sting"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_PALUTENA)
			AttackModule::enable_safe_pos()
		}
    });
}		
#[acmd_script(
    agent = "pitb",
    script =  "game_specialairsend",
    category = ACMD_GAME,
	low_priority)]
unsafe fn dpit_upb(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		frame(Frame=4)
		if(is_excute){
			ATTACK(ID=0, Part=0, Bone=hash40("havel"), Damage=14.0, Angle=80, KBG=90, FKB=0, BKB=50, Size=12.0, X=0.0, Y=0.0, Z=0.0, X2=0.0, Y2=0.0, Z2=0.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_THRU, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_elec"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_HEAVY, Type=ATTACK_REGION_PUNCH)
		}
		frame(Frame=6)
		if(is_excute){
			rust {
				let speed = smash::phx::Vector3f { x: 0.25, y: 4.0, z: 0.0 };
				KineticModule::add_speed(fighter.module_accessor, &speed);
			}
		}
		frame(Frame=10)
		if(is_excute){
			AttackModule::clear_all()
		}
    });
}	
#[acmd_script(
    agent = "pitb",
    script =  "effect_specialairsend",
    category = ACMD_EFFECT,
	low_priority)]
unsafe fn dpit_upb_eff(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		frame(Frame=2)
		if(is_excute){
			EFFECT_FOLLOW(0x165c0b928du64, hash40("havel"), 0, 0, 0, 0, 0, 0, 1, true)
		}
		frame(Frame=3)
		if(is_excute){
			EFFECT_FOLLOW(0x141fcff65du64, hash40("top"), -2, 15, 2, 10, -40, 93, 1.2, true)
		}
		frame(Frame=58)
		if(is_excute){
			EFFECT_FOLLOW(0x1460d5967f, hash40("havel"), 0, 0, 2, 0, 0, 0, 1, true)
		}
    });
}	
#[acmd_script(
    agent = "pitb",
    script =  "game_specialsend",
    category = ACMD_GAME,
	low_priority)]
unsafe fn dpit_upb_ground(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		if(is_excute){
			MotionModule::change_motion(smash::phx::Hash40::new("special_air_s_end"), 0.0, 1.0, false, 0.0, false, false)
		}
    });
}	
#[acmd_script(
    agent = "pitb",
    scripts =  ["game_specialairhistart", "game_specialhistart"],
    category = ACMD_GAME,
	low_priority)]
unsafe fn dpit_upb_start(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		if(is_excute){
			StatusModule::change_status_request_from_script(FIGHTER_PIT_STATUS_KIND_SPECIAL_S_END, true)
		}
    });
}	
#[acmd_script(
    agent = "pitb",
    scripts =  ["game_specialairsstart", "game_specialsstart"],
    category = ACMD_GAME,
	low_priority)]
unsafe fn dpit_sideb(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		if(is_excute){
			StatusModule::change_status_request_from_script(FIGHTER_PIT_STATUS_KIND_SPECIAL_HI_RUSH, false)
		}
    });
}	
//Pit Downb nerf
#[fighter_frame_callback]
pub fn pitoo(fighter : &mut L2CFighterCommon) {
    unsafe {
        let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent); 
		let fighter_kind = smash::app::utility::get_kind(boma);
		let ENTRY_ID = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
		let situation_kind = StatusModule::situation_kind(boma);
		let status_kind = smash::app::lua_bind::StatusModule::status_kind(boma);
		if fighter_kind == *FIGHTER_KIND_PITB {
			if  MotionModule::motion_kind(boma) == hash40("special_lw_break_l") || MotionModule::motion_kind(boma) == hash40("special_lw_break_r"){
				StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_DOWN, true);
			};
			if  MotionModule::motion_kind(boma) == hash40("special_air_lw_break_l") || MotionModule::motion_kind(boma) == hash40("special_air_lw_break_r"){
				StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_CAPTURE_JUMP, true);
			};
			if MotionModule::motion_kind(boma) == hash40("special_air_s_end") && MotionModule::frame(boma) >= 58.0 {
				StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_FALL_SPECIAL, true);
			};
			if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_HI {
				StatusModule::change_status_request_from_script(boma, *FIGHTER_PIT_STATUS_KIND_SPECIAL_S_END, true);
			};
			if status_kind == *FIGHTER_PIT_STATUS_KIND_SPECIAL_S_END {
				if situation_kind == *SITUATION_KIND_AIR {
					StatusModule::set_keep_situation_air(boma, true);
				};
			};
			if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_S {
				StatusModule::set_situation_kind(boma, smash::app::SituationKind(*SITUATION_KIND_AIR), true);
				StatusModule::set_keep_situation_air(boma, true);
				StatusModule::change_status_request_from_script(boma, *FIGHTER_PIT_STATUS_KIND_SPECIAL_HI_RUSH, false);
			};
			if situation_kind != *SITUATION_KIND_AIR {
				CAN_SIDEB[ENTRY_ID] = 0;
			};
			if status_kind == *FIGHTER_PIT_STATUS_KIND_SPECIAL_HI_RUSH {
					CAN_SIDEB[ENTRY_ID] = 1;
					if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_ATTACK) {
						StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_AIR, true);
					};
					if MotionModule::frame(boma) > 46.0 {
						StatusModule::change_status_request_from_script(boma, *FIGHTER_PIT_STATUS_KIND_SPECIAL_HI_RUSH_END, true);
					};
			};
		};
		/*if fighter_kind == *FIGHTER_KIND_KIRBY && WorkModule::get_int(boma, *FIGHTER_KIRBY_INSTANCE_WORK_ID_INT_COPY_CHARA) == *FIGHTER_KIND_PITB{
			if (status_kind == *FIGHTER_KIRBY_STATUS_KIND_PIT_SPECIAL_N_SHOOT || status_kind == *FIGHTER_KIRBY_STATUS_KIND_PITB_SPECIAL_N_SHOOT) && StatusModule::situation_kind(boma) == *SITUATION_KIND_GROUND {
				if (ControlModule::get_command_flag_cat(boma, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_DASH) != 0 {
							StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_DASH, true);
				};
			};
			if situation_kind == *SITUATION_KIND_AIR && [*FIGHTER_STATUS_KIND_ATTACK_AIR].contains(&status_kind) {
				if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_JUMP) || ControlModule::get_stick_y(boma) >= 0.5{
					CHECK_FLOAT[ENTRY_ID] += 1;
				} else {
					CHECK_FLOAT[ENTRY_ID] = 0;
				};
				if CHECK_FLOAT[ENTRY_ID] >= CHECK_FLOAT_MAX && CHECK_FLOAT[ENTRY_ID] % 5 == 0 {
					if !is_hitlag(boma) {
						let speed = smash::phx::Vector3f { x: 0.0, y: (FLOAT_FALLSPEED)/2.0, z: 0.0 };
						KineticModule::add_speed(boma, &speed);
					} else {
						let speed = smash::phx::Vector3f { x: 0.0, y: (FLOAT_FALLSPEED*HITLAG_MULT)/2.0, z: 0.0 };
						KineticModule::add_speed(boma, &speed);
					};
					macros::EFFECT(fighter, Hash40::new("pitb_feather"), Hash40::new("bust"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
				};
			} else {
				CHECK_FLOAT[ENTRY_ID] = 0;
			};
		};
		if fighter_kind == *FIGHTER_KIND_PITB {
			if  MotionModule::motion_kind(boma) == hash40("special_lw_break_l") || MotionModule::motion_kind(boma) == hash40("special_lw_break_r"){
				StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_DOWN, true);
			};
			if  MotionModule::motion_kind(boma) == hash40("special_air_lw_break_l") || MotionModule::motion_kind(boma) == hash40("special_air_lw_break_r"){
				StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_CAPTURE_JUMP, true);
			};
			if status_kind == *FIGHTER_PIT_STATUS_KIND_SPECIAL_N_SHOOT && StatusModule::situation_kind(boma) == *SITUATION_KIND_GROUND {
				if (ControlModule::get_command_flag_cat(boma, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_DASH) != 0 {
							StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_DASH, true);
				};
			};
			if situation_kind == *SITUATION_KIND_AIR && [*FIGHTER_STATUS_KIND_ATTACK_AIR].contains(&status_kind) {
				if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_JUMP) || ControlModule::get_stick_y(boma) >= 0.5{
					CHECK_FLOAT[ENTRY_ID] += 1;
				} else {
					CHECK_FLOAT[ENTRY_ID] = 0;
				};
				if CHECK_FLOAT[ENTRY_ID] >= CHECK_FLOAT_MAX && CHECK_FLOAT[ENTRY_ID] % 5 == 0 {
					if !is_hitlag(boma) {
						let speed = smash::phx::Vector3f { x: 0.0, y: FLOAT_FALLSPEED, z: 0.0 };
						KineticModule::add_speed(boma, &speed);
					} else {
						let speed = smash::phx::Vector3f { x: 0.0, y: FLOAT_FALLSPEED*HITLAG_MULT, z: 0.0 };
						KineticModule::add_speed(boma, &speed);
					};
					macros::EFFECT(fighter, Hash40::new("pitb_feather"), Hash40::new("bust"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
				};
			} else {
				CHECK_FLOAT[ENTRY_ID] = 0;
			};
		};*/
	};
}
		
		
pub fn install() {
    smashline::install_acmd_scripts!(
		dpit_fair,
		dpit_jab2,
		//dpit_dtilt,
		dpit_da,
		dpit_fsmash,
		//dpit_arrow,
		dpit_dair,
		dpit_nair,
		dpit_nair_eff,
		dpit_nair_snd,
		dpit_uair,
		dpit_uair_eff,
		dpit_uair_snd,
		dpit_upb,
		dpit_sideb,
		dpit_upb_start,
		dpit_upb_ground,
		dpit_upb_eff
    );
	smashline::install_agent_frame_callbacks!(pitoo);
}
