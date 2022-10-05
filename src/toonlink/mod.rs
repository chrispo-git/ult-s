use smash::hash40;
use smash::lib::lua_const::*;
use smash::app::lua_bind::*;
use smash::lua2cpp::*;
use smashline::*;
use smash_script::*;
use smash::phx::{Hash40, Vector2f};
use crate::util::*;

#[acmd_script(
    agent = "toonlink",
    script =  "game_dash",
    category = ACMD_GAME,
	low_priority)]
unsafe fn tink_dash(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		frame(Frame=10)
		if(is_excute){
			WorkModule::enable_transition_term(FIGHTER_STATUS_TRANSITION_TERM_ID_DASH_TO_RUN)
		}
    });
}	
#[acmd_script(
    agent = "toonlink",
    script =  "game_turndash",
    category = ACMD_GAME,
	low_priority)]
unsafe fn tink_dashback(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		frame(Frame=3)
		if(is_excute){
			WorkModule::on_flag(Flag=FIGHTER_STATUS_DASH_FLAG_TURN_DASH)
		}
		frame(Frame=12)
		if(is_excute){
			WorkModule::enable_transition_term(FIGHTER_STATUS_TRANSITION_TERM_ID_DASH_TO_RUN)
		}
    });
}
#[acmd_script(
    agent = "toonlink",
    script =  "game_attack11",
    category = ACMD_GAME,
	low_priority)]
unsafe fn tink_jab1(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		frame(Frame=1)
		FT_MOTION_RATE(FSM=0.5)
		frame(Frame=6)
		FT_MOTION_RATE(FSM=1)
		if(is_excute){
			ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=2.0, Angle=361, KBG=25, FKB=0, BKB=25, Size=1.8, X=0.0, Y=6.0, Z=8.24, X2=0.0, Y2=6.0, Z2=6.0, Hitlag=1.5, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
			ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=2.0, Angle=361, KBG=25, FKB=0, BKB=25, Size=1.8, X=0.0, Y=6.0, Z=10.48, X2=0.0, Y2=6.0, Z2=6.0, Hitlag=1.5, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
			ATTACK(ID=2, Part=0, Bone=hash40("top"), Damage=2.0, Angle=180, KBG=15, FKB=0, BKB=20, Size=1.8, X=0.0, Y=6.0, Z=14.96, X2=0.0, Y2=6.0, Z2=6.0, Hitlag=1.5, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_FIGHTER, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
			ATTACK(ID=3, Part=0, Bone=hash40("top"), Damage=2.0, Angle=361, KBG=15, FKB=0, BKB=20, Size=1.8, X=0.0, Y=6.0, Z=14.96, X2=0.0, Y2=6.0, Z2=6.0, Hitlag=1.5, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
		}
		wait(Frames=2)
		if(is_excute){
			AttackModule::clear_all()
			WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO)
		}
		frame(Frame=14)
		if(is_excute){
			WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_FLAG_ENABLE_NO_HIT_COMBO)
		}
    });
}	
#[acmd_script(
    agent = "toonlink",
    script =  "game_attack12",
    category = ACMD_GAME,
	low_priority)]
unsafe fn tink_jab2(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		frame(Frame=6)
		if(is_excute){
			ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=2.0, Angle=361, KBG=25, FKB=0, BKB=25, Size=2.5, X=0.0, Y=6.0, Z=8.12, X2=0.0, Y2=6.0, Z2=7.0, Hitlag=1.2, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
			ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=2.0, Angle=361, KBG=25, FKB=0, BKB=25, Size=3.2, X=0.0, Y=6.0, Z=11.48, X2=0.0, Y2=6.0, Z2=7.0, Hitlag=1.2, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
			ATTACK(ID=2, Part=0, Bone=hash40("top"), Damage=2.0, Angle=180, KBG=15, FKB=0, BKB=20, Size=3.2, X=0.0, Y=6.0, Z=15.4, X2=0.0, Y2=6.0, Z2=7.0, Hitlag=1.2, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_FIGHTER, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
			ATTACK(ID=3, Part=0, Bone=hash40("top"), Damage=2.0, Angle=361, KBG=15, FKB=0, BKB=20, Size=3.2, X=0.0, Y=6.0, Z=15.4, X2=0.0, Y2=6.0, Z2=7.0, Hitlag=1.2, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
		}
		wait(Frames=2)
		if(is_excute){
			AttackModule::clear_all()
			WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO)
		}
		frame(Frame=14)
		if(is_excute){
			WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_FLAG_ENABLE_NO_HIT_COMBO)
		}
    });
}	
#[acmd_script(
    agent = "toonlink",
    script =  "game_attack13",
    category = ACMD_GAME,
	low_priority)]
unsafe fn tink_jab3(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		frame(Frame=6)
		if(is_excute){
			ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=4.0, Angle=30, KBG=70, FKB=0, BKB=65, Size=3.5, X=0.0, Y=6.0, Z=8.0, X2=0.0, Y2=6.0, Z2=8.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_sting"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
			ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=4.0, Angle=30, KBG=70, FKB=0, BKB=65, Size=4.5, X=0.0, Y=6.0, Z=17.52, X2=0.0, Y2=6.0, Z2=8.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_sting"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
		}
		wait(Frames=2)
		if(is_excute){
			AttackModule::clear_all()
		}
    });
}	
#[acmd_script(
    agent = "toonlink",
    script =  "game_attackdash",
    category = ACMD_GAME,
	low_priority)]
unsafe fn tink_da(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		FT_MOTION_RATE(FSM=0.7)
		frame(Frame=8)
		FT_MOTION_RATE(FSM=1.1)
		if(is_excute){
			ATTACK(ID=3, Part=0, Bone=hash40("sword2"), Damage=8.0, Angle=65, KBG=70, FKB=0, BKB=70, Size=4.2, X=6.16, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.2, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.3, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
			ATTACK(ID=0, Part=0, Bone=hash40("sword2"), Damage=8.0, Angle=65, KBG=70, FKB=0, BKB=70, Size=4.0, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.2, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.3, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
			ATTACK(ID=1, Part=0, Bone=hash40("arml"), Damage=6.0, Angle=65, KBG=70, FKB=0, BKB=70, Size=3.5, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_THRU, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.3, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
		}
		wait(Frames=2)
		FT_MOTION_RATE(FSM=0.619)
		if(is_excute){
			AttackModule::clear_all()
		}
		frame(Frame=43)
		FT_MOTION_RATE(FSM=1)
    });
}	
#[acmd_script(
    agent = "toonlink",
    script =  "game_attacklw3",
    category = ACMD_GAME,
	low_priority)]
unsafe fn tink_dtilt(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		frame(Frame=9)
		if(is_excute){
			ATTACK(ID=0, Part=0, Bone=hash40("sword2"), Damage=7.0, Angle=75, KBG=88, FKB=0, BKB=30, Size=2.5, X=6.72, Y=0.0, Z=-1.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.5, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
			ATTACK(ID=1, Part=0, Bone=hash40("sword2"), Damage=7.0, Angle=75, KBG=88, FKB=0, BKB=30, Size=3.0, X=1.0, Y=0.0, Z=-1.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.5, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
			ATTACK(ID=2, Part=0, Bone=hash40("arml"), Damage=7.0, Angle=75, KBG=88, FKB=0, BKB=30, Size=3.0, X=1.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.5, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
		}
		wait(Frames=2)
		if(is_excute){
			AttackModule::clear_all()
		}
    });
}	
#[acmd_script(
    agent = "toonlink",
    script =  "game_attacks3",
    category = ACMD_GAME,
	low_priority)]
unsafe fn tink_ftilt(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		frame(Frame=9)
		if(is_excute){
			ATTACK(ID=0, Part=0, Bone=hash40("sword2"), Damage=9.0, Angle=38, KBG=83, FKB=0, BKB=45, Size=4.3, X=5.6, Y=0.0, Z=-2.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
			ATTACK(ID=1, Part=0, Bone=hash40("sword2"), Damage=9.0, Angle=20, KBG=83, FKB=0, BKB=45, Size=4.0, X=1.0, Y=0.0, Z=-2.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
			ATTACK(ID=2, Part=0, Bone=hash40("arml"), Damage=9.0, Angle=20, KBG=83, FKB=0, BKB=45, Size=3.0, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
		}
		frame(Frame=10)
		if(is_excute){
			ATTACK(ID=0, Part=0, Bone=hash40("sword2"), Damage=9.0, Angle=361, KBG=83, FKB=0, BKB=45, Size=4.3, X=5.6, Y=0.0, Z=-2.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
			ATTACK(ID=1, Part=0, Bone=hash40("sword2"), Damage=9.0, Angle=20, KBG=83, FKB=0, BKB=45, Size=4.0, X=1.0, Y=0.0, Z=-2.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
		}
		frame(Frame=14)
		if(is_excute){
			AttackModule::clear_all()
		}
    });
}	
#[acmd_script(
    agent = "toonlink",
    script =  "game_attackhi3",
    category = ACMD_GAME,
	low_priority)]
unsafe fn tink_utilt(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		frame(Frame=8)
		if(is_excute){
			ATTACK(ID=0, Part=0, Bone=hash40("sword2"), Damage=5.0, Angle=95, KBG=140, FKB=0, BKB=30, Size=3.8, X=5.6, Y=0.0, Z=-1.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
			ATTACK(ID=1, Part=0, Bone=hash40("sword2"), Damage=5.0, Angle=85, KBG=140, FKB=0, BKB=30, Size=4.4, X=1.12, Y=0.0, Z=-1.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
			ATTACK(ID=2, Part=0, Bone=hash40("arml"), Damage=5.0, Angle=105, KBG=140, FKB=0, BKB=30, Size=3.0, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
		}
		frame(Frame=13)
		if(is_excute){
			AttackModule::clear_all()
		}
		FT_MOTION_RATE(FSM=0.5384615384615385)
    });
}	
#[acmd_script( agent = "toonlink_bow", scripts = ["game_nstart", "game_n", "game_nend", "game_nairstart", "game_nair", "game_nairend"], category = ACMD_GAME, low_priority )]
unsafe fn tink_hammer(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        LinkModule::set_model_constraint_pos_ort(fighter.module_accessor, *LINK_NO_ARTICLE, Hash40::new("have"), Hash40::new("havel"), *CONSTRAINT_FLAG_ORIENTATION as u32 | *CONSTRAINT_FLAG_POSITION as u32, false);
    }
}
#[acmd_script(
    agent = "toonlink",
    script =  "game_specialairhi",
    category = ACMD_GAME,
	low_priority)]
unsafe fn tink_upb_air(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		if(is_excute){
			ArticleModule::generate_article(FIGHTER_TOONLINK_GENERATE_ARTICLE_BOW, false, 0)
			ArticleModule::change_motion(FIGHTER_TOONLINK_GENERATE_ARTICLE_BOW,smash::phx::Hash40::new("air_hi"),false,0.0)
		}
		frame(Frame=4)
		if(is_excute){
			ATTACK(ID=0, Part=0, Bone=hash40("haver"), Damage=11.0, Angle=90, KBG=74, FKB=0, BKB=70, Size=7.5, X=0.0, Y=0.0, Z=-1.2, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.2, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=20, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_SWORD)
		}
		frame(Frame=9)
		if(is_excute){
			ATTACK(ID=0, Part=0, Bone=hash40("haver"), Damage=5.0, Angle=90, KBG=74, FKB=0, BKB=70, Size=4.5, X=0.0, Y=0.0, Z=-1.2, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=20, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_SWORD)
			sv_battle_object::notify_event_msc_cmd(0x2127e37c07u64, GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES)
		}
    });
}	
#[acmd_script(
    agent = "toonlink",
    script =  "expression_specialairhi",
    category = ACMD_EXPRESSION,
	low_priority)]
unsafe fn tink_upb_air_expr(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		if(is_excute){
			VisibilityModule::set_int64(hash40("shield") as i64, hash40("shield_back") as i64)
			ItemModule::set_have_item_visibility(false, 0)
		}
    });
}	
#[acmd_script(
    agent = "toonlink",
    script =  "effect_specialairhi",
    category = ACMD_EFFECT,
	low_priority)]
unsafe fn tink_upb_air_eff(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
}	
#[acmd_script(
    agent = "toonlink",
    script =  "sound_specialairhi",
    category = ACMD_SOUND,
	low_priority)]
unsafe fn tink_upb_air_snd(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
}	
#[acmd_script(
    agent = "toonlink",
    script =  "game_attacks4",
    category = ACMD_GAME,
	low_priority)]
unsafe fn tink_fsmash(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		frame(Frame=8)
		if(is_excute){
			ArticleModule::generate_article(FIGHTER_TOONLINK_GENERATE_ARTICLE_BOW, false, 0)
			ArticleModule::change_motion(FIGHTER_TOONLINK_GENERATE_ARTICLE_BOW,smash::phx::Hash40::new("attack_s4_s"),false,0.0)
		}
		frame(Frame=9)
		if(is_excute){
			WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD)
		}
		frame(Frame=17)
		if(is_excute){
			ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=16.0, Angle=40, KBG=110, FKB=0, BKB=25, Size=5.5, X=0.0, Y=9.0, Z=6.0, X2=0.0, Y2=19.0, Z2=18.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_HEAVY, Type=ATTACK_REGION_SWORD)
		}
		frame(Frame=18)
		if(is_excute){
			ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=16.0, Angle=40, KBG=110, FKB=0, BKB=25, Size=5.5, X=0.0, Y=7.0, Z=6.0, X2=0.0, Y2=15.0, Z2=18.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_HEAVY, Type=ATTACK_REGION_SWORD)
		}
		frame(Frame=19)
		if(is_excute){
			ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=16.0, Angle=40, KBG=110, FKB=0, BKB=25, Size=5.5, X=0.0, Y=3.0, Z=16.0, X2=0.0, Y2=3.0, Z2=20.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_HEAVY, Type=ATTACK_REGION_SWORD)
			ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=10.0, Angle=40, KBG=110, FKB=0, BKB=25, Size=5.5, X=0.0, Y=3.0, Z=5.0, X2=0.0, Y2=3.0, Z2=32.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_G, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_SWORD)
			QUAKE(CAMERA_QUAKE_KIND_M)
		}
		frame(Frame=21)
		if(is_excute){
			AttackModule::clear_all()
		}
		frame(Frame=35)
		if(is_excute){
			ArticleModule::remove_exist(FIGHTER_TOONLINK_GENERATE_ARTICLE_BOW,smash::app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL))
		}
    });
}	
#[acmd_script(
    agent = "toonlink",
    script =  "effect_attacks4",
    category = ACMD_EFFECT,
	low_priority)]
unsafe fn tink_fsmash_eff(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		frame(Frame=2)
		if(is_excute){
			EFFECT(hash40("sys_smash_flash"), hash40("havel"), 10.0, 0, 0.0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true)
		}
		frame(Frame=15)
		if(is_excute){
			LANDING_EFFECT(hash40("sys_dash_smoke"), hash40("top"), -5, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false)
		}
		frame(Frame=19)
		if(is_excute){
			EFFECT(hash40("sys_landing_smoke"), hash40("top"), 22.0, 0.0, -1.2, 0, 0, 0, 1.5, 0, 0, 0, 0, 0, 0, false)
		}
		frame(Frame=34)
		if(is_excute){
			EFFECT(hash40("sys_erace_smoke"), hash40("haver"), 0.0, 0.0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false)
			EFFECT(hash40("sys_erace_smoke"), hash40("top"), 18.0, 0.0, 0, 0, 0, 0, 1.2, 0, 0, 0, 0, 0, 0, false)
		}
    });
}	
#[acmd_script(
    agent = "toonlink",
    script =  "expression_attacks4",
    category = ACMD_EXPRESSION,
	low_priority)]
unsafe fn tink_fsmash_expr(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		if(is_excute){
			VisibilityModule::set_int64(hash40("shield") as i64, hash40("shield_back") as i64)
			VisibilityModule::set_int64(hash40("sword") as i64, hash40("sword_back") as i64)
			ItemModule::set_have_item_visibility(false, 0)
		}
		frame(Frame=10)
		if(is_excute){
			VisibilityModule::set_int64(hash40("shield") as i64, hash40("shield_back") as i64)
			VisibilityModule::set_int64(hash40("sword") as i64, hash40("sword_back") as i64)
			ItemModule::set_have_item_visibility(false, 0)
		}
		frame(Frame=62)
		if(is_excute){
			VisibilityModule::set_int64(hash40("shield") as i64, hash40("shield_normal") as i64)
			VisibilityModule::set_int64(hash40("sword") as i64, hash40("sword_normal") as i64)
			ItemModule::set_have_item_visibility(true, 0)
		}
    });
}	
#[acmd_script(
    agent = "toonlink",
    script =  "sound_attacks4",
    category = ACMD_SOUND,
	low_priority)]
unsafe fn tink_fsmash_snd(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		frame(Frame=10)
		if(is_excute){
			STOP_SE(hash40("se_common_smash_start"))
		}
		frame(Frame=16)
		if(is_excute){
			PLAY_SE(hash40("vc_toonlink_attack07"))
			PLAY_SE(hash40("se_common_smashswing_03"))
			PLAY_SE(hash40("se_toonlink_swing_ll"))
		}
		frame(Frame=19)
		if(is_excute){
			PLAY_SE(hash40("se_common_down_soil_s"))
			PLAY_LANDING_SE(hash40("se_toonlink_attackair_l01"))
		}
    });
}	
#[acmd_script(
    agent = "toonlink",
    script =  "expression_attacks4charge",
    category = ACMD_EXPRESSION,
	low_priority)]
unsafe fn tink_fsmash_charge_expr(fighter: &mut L2CAgentBase) {
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
    agent = "toonlink",
    script =  "game_attackhi4",
    category = ACMD_GAME,
	low_priority)]
unsafe fn tink_usmash(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		frame(Frame=5)
		if(is_excute){
			WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD)
		}
		frame(Frame=11)
		if(is_excute){
			ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=13.0, Angle=100, KBG=94, FKB=0, BKB=43, Size=6.0, X=0.0, Y=6.0, Z=8.9, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_G, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
			ATTACK(ID=1, Part=0, Bone=hash40("sword2"), Damage=13.0, Angle=96, KBG=94, FKB=0, BKB=43, Size=4.0, X=6.16, Y=0.0, Z=-1.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
			ATTACK(ID=2, Part=0, Bone=hash40("arml"), Damage=13.0, Angle=96, KBG=94, FKB=0, BKB=43, Size=3.0, X=1.2, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
		}
		frame(Frame=12)
		if(is_excute){
			ATTACK(ID=0, Part=0, Bone=hash40("sword2"), Damage=13.0, Angle=96, KBG=93, FKB=0, BKB=30, Size=4.0, X=2.0, Y=0.0, Z=-1.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
		}
		frame(Frame=14)
		if(is_excute){
			ATTACK(ID=0, Part=0, Bone=hash40("sword2"), Damage=10.0, Angle=96, KBG=93, FKB=0, BKB=30, Size=4.0, X=2.0, Y=0.0, Z=-1.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
			ATTACK(ID=1, Part=0, Bone=hash40("sword2"), Damage=10.0, Angle=96, KBG=94, FKB=0, BKB=43, Size=4.0, X=6.16, Y=0.0, Z=-1.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
			ATTACK(ID=2, Part=0, Bone=hash40("arml"), Damage=10.0, Angle=96, KBG=94, FKB=0, BKB=43, Size=3.0, X=1.2, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
		}
		frame(Frame=16)
		if(is_excute){
			AttackModule::clear_all()
		}
    });
}	
#[acmd_script(
    agent = "toonlink",
    script =  "game_attacklw4",
    category = ACMD_GAME,
	low_priority)]
unsafe fn tink_dsmash(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		frame(Frame=4)
		if(is_excute){
			WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD)
		}
		frame(Frame=9)
		if(is_excute){
			ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=12.0, Angle=48, KBG=90, FKB=0, BKB=55, Size=4.0, X=0.0, Y=2.2, Z=16.96, X2=0.0, Y2=2.9, Z2=8.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
		}
		wait(Frames=2)
		if(is_excute){
			AttackModule::clear_all()
		}
		frame(Frame=17)
		if(is_excute){
			ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=13.0, Angle=42, KBG=85, FKB=0, BKB=55, Size=4.3, X=0.0, Y=4.2, Z=-16.96, X2=0.0, Y2=4.2, Z2=-5.5, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_B, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
		}
		wait(Frames=2)
		if(is_excute){
			AttackModule::clear_all()
		}
		frame(Frame=24)
		FT_MOTION_RATE(FSM=0.88)
		frame(Frame=50)
		FT_MOTION_RATE(FSM=1)
    });
}			
#[acmd_script(
    agent = "toonlink",
    script =  "game_attackairb",
    category = ACMD_GAME,
	low_priority)]
unsafe fn tink_bair(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		if(is_excute){
			WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
		}
		frame(Frame=6)
		if(is_excute){
			ATTACK(ID=0, Part=0, Bone=hash40("sword2"), Damage=8.0, Angle=60, KBG=67, FKB=0, BKB=64, Size=4.0, X=6.72, Y=0.0, Z=-1.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_B, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
			ATTACK(ID=1, Part=0, Bone=hash40("sword2"), Damage=8.0, Angle=60, KBG=67, FKB=0, BKB=64, Size=5.0, X=2.24, Y=0.0, Z=-1.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_B, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
			ATTACK(ID=2, Part=0, Bone=hash40("arml"), Damage=8.0, Angle=60, KBG=67, FKB=0, BKB=64, Size=4.0, X=1.2, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_B, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
		}
		frame(Frame=10)
		FT_MOTION_RATE(FSM=1.2)
		if(is_excute){
			AttackModule::clear_all()
		}
		frame(Frame=24)
		if(is_excute){
			WorkModule::off_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
		}
    });
}		
#[acmd_script(
    agent = "toonlink",
    script =  "game_attackairf",
    category = ACMD_GAME,
	low_priority)]
unsafe fn tink_fair(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		frame(Frame=4)
		if(is_excute){
			WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
		}
		frame(Frame=13)
		if(is_excute){
			ATTACK(ID=0, Part=0, Bone=hash40("sword2"), Damage=13.0, Angle=361, KBG=98, FKB=0, BKB=35, Size=5.0, X=4.96, Y=-2.5, Z=-2.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
			ATTACK(ID=1, Part=0, Bone=hash40("sword2"), Damage=13.0, Angle=361, KBG=98, FKB=0, BKB=35, Size=5.0, X=1.24, Y=-2.5, Z=-2.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
			ATTACK(ID=2, Part=0, Bone=hash40("arml"), Damage=13.0, Angle=361, KBG=98, FKB=0, BKB=35, Size=4.0, X=1.2, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
		}
		wait(Frames=4)
		if(is_excute){
			AttackModule::clear_all()
		}
		frame(Frame=39)
		if(is_excute){
			WorkModule::off_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
		}
    });
}		
#[acmd_script(
    agent = "toonlink",
    script =  "game_attackairn",
    category = ACMD_GAME,
	low_priority)]
unsafe fn tink_nair(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		frame(Frame=3)
		if(is_excute){
			WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
		}
		frame(Frame=6)
		if(is_excute){
			ATTACK(ID=0, Part=0, Bone=hash40("sword2"), Damage=8.5, Angle=361, KBG=100, FKB=0, BKB=20, Size=4.0, X=4.816, Y=0.0, Z=-1.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.3, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
			ATTACK(ID=1, Part=0, Bone=hash40("sword2"), Damage=8.5, Angle=361, KBG=100, FKB=0, BKB=20, Size=4.5, X=0.0, Y=0.0, Z=-1.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.3, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
			ATTACK(ID=2, Part=0, Bone=hash40("arml"), Damage=8.5, Angle=361, KBG=100, FKB=0, BKB=20, Size=4.5, X=-0.5, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.3, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
			ATTACK(ID=3, Part=0, Bone=hash40("colonellm"), Damage=8.5, Angle=361, KBG=100, FKB=0, BKB=20, Size=2.5, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.3, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
		}
		frame(Frame=8)
		if(is_excute){
			AttackModule::clear_all()
		}
		frame(Frame=13)
		if(is_excute){
			ATTACK(ID=0, Part=0, Bone=hash40("sword2"), Damage=7.0, Angle=361, KBG=100, FKB=0, BKB=20, Size=4.0, X=4.816, Y=0.0, Z=-1.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.3, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
			ATTACK(ID=1, Part=0, Bone=hash40("sword2"), Damage=7.0, Angle=361, KBG=100, FKB=0, BKB=20, Size=4.5, X=0.0, Y=0.0, Z=-1.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.3, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
			ATTACK(ID=2, Part=0, Bone=hash40("arml"), Damage=7.0, Angle=361, KBG=100, FKB=0, BKB=20, Size=4.5, X=-0.5, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.3, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
			ATTACK(ID=3, Part=0, Bone=hash40("colonellm"), Damage=7.0, Angle=361, KBG=100, FKB=0, BKB=20, Size=2.5, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.3, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
		}
		frame(Frame=15)
		if(is_excute){
			AttackModule::clear_all()
		}
		FT_MOTION_RATE(FSM=0.8148148148148)
		frame(Frame=41)
		if(is_excute){
			WorkModule::off_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
		}
    });
}

#[acmd_script(
    agent = "toonlink",
    script =  "game_attackairhi",
    category = ACMD_GAME,
	low_priority)]
unsafe fn tink_uair(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		frame(Frame=6)
		if(is_excute){
			WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
		}
		frame(Frame=11)
		if(is_excute){
			ATTACK(ID=0, Part=0, Bone=hash40("sword2"), Damage=1.5, Angle=367, KBG=100, FKB=40, BKB=0, Size=4.6, X=5.6, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.6, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=4, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_sting"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
			ATTACK(ID=1, Part=0, Bone=hash40("sword2"), Damage=1.5, Angle=367, KBG=100, FKB=40, BKB=0, Size=5.2, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.6, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=4, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_sting"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
			ATTACK(ID=2, Part=0, Bone=hash40("arml"), Damage=1.5, Angle=367, KBG=100, FKB=40, BKB=0, Size=3.5, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.6, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=4, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_sting"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
			AttackModule::set_add_reaction_frame(ID=0, Frames=6.0, Unk=false)
			AttackModule::set_add_reaction_frame(ID=1, Frames=6.0, Unk=false)
			AttackModule::set_add_reaction_frame(ID=2, Frames=6.0, Unk=false)
		}
		frame(Frame=28)
		if(is_excute){
			AttackModule::clear_all()
			ATTACK(ID=0, Part=0, Bone=hash40("sword2"), Damage=10.0, Angle=80, KBG=95, FKB=0, BKB=25, Size=6.6, X=5.6, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.1, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_sting"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
			ATTACK(ID=1, Part=0, Bone=hash40("sword2"), Damage=10.0, Angle=80, KBG=95, FKB=0, BKB=25, Size=6.2, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.1, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_sting"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
			ATTACK(ID=2, Part=0, Bone=hash40("arml"), Damage=10.0, Angle=80, KBG=95, FKB=0, BKB=25, Size=5.5, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.1, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_sting"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
		}
		frame(Frame=30)
		if(is_excute){
			ATTACK(ID=0, Part=0, Bone=hash40("sword2"), Damage=5.0, Angle=80, KBG=95, FKB=0, BKB=25, Size=6.6, X=5.6, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.8, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_sting"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
			ATTACK(ID=1, Part=0, Bone=hash40("sword2"), Damage=5.0, Angle=80, KBG=95, FKB=0, BKB=25, Size=6.2, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.8, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_sting"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
			ATTACK(ID=2, Part=0, Bone=hash40("arml"), Damage=5.0, Angle=80, KBG=95, FKB=0, BKB=25, Size=5.5, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.8, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_sting"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
		}
		frame(Frame=41)
		if(is_excute){
			AttackModule::clear_all()
		}
		frame(Frame=47)
		if(is_excute){
			WorkModule::off_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
		}
    });
}
#[acmd_script(
    agent = "toonlink",
    script =  "effect_attackairhi",
    category = ACMD_EFFECT,
	low_priority)]
unsafe fn tink_uair_eff(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
			frame(Frame=10)
			if(is_excute){
				EFFECT_FOLLOW(hash40("toonlink_sword"), hash40("sword1"), 0, 0, 0, 0, 0, 0, 1, true)
			}
			frame(Frame=11)
			if(is_excute){
				EFFECT(hash40("sys_smash_flash"), hash40("sword1"), 9.5, 0, 0, 0, 0, 0, 0.75, 0, 0, 0, 0, 0, 0, true)
				LAST_EFFECT_SET_RATE(1.5)
				EFFECT_FOLLOW(hash40("sys_spin_wind"), hash40("sword2"), 0, 0, 0, 0, 0, 90, 0.5, true)
				EFFECT_FOLLOW(hash40("sys_spin_wind"), hash40("sword2"), 1.0, 0, 0, 0, 0, 90, 0.45, true)
				EFFECT_FOLLOW(hash40("sys_spin_wind"), hash40("sword2"), 3.0, 0, 0, 0, 0, 90, 0.35, true)
				EFFECT_FOLLOW(hash40("sys_spin_wind"), hash40("sword2"), 4.0, 0, 0, 0, 0, 90, 0.3, true)
				EFFECT_FOLLOW(hash40("sys_spin_wind"), hash40("sword2"), 6.0, 0, 0, 0, 0, 90, 0.2, true)
				EFFECT_FOLLOW(hash40("sys_spin_wind"), hash40("sword2"), 8.0, 0, 0, 0, 0, 90, 0.2, true)
			}
			frame(Frame=16)
			if(is_excute){
				EFFECT_FOLLOW(hash40("sys_spin_wind"), hash40("sword2"), 0, 0, 0, 0, 0, 90, 0.5, true)
				EFFECT_FOLLOW(hash40("sys_spin_wind"), hash40("sword2"), 1.0, 0, 0, 0, 0, 90, 0.45, true)
				EFFECT_FOLLOW(hash40("sys_spin_wind"), hash40("sword2"), 3.0, 0, 0, 0, 0, 90, 0.35, true)
				EFFECT_FOLLOW(hash40("sys_spin_wind"), hash40("sword2"), 4.0, 0, 0, 0, 0, 90, 0.3, true)
				EFFECT_FOLLOW(hash40("sys_spin_wind"), hash40("sword2"), 6.0, 0, 0, 0, 0, 90, 0.2, true)
				EFFECT_FOLLOW(hash40("sys_spin_wind"), hash40("sword2"), 8.0, 0, 0, 0, 0, 90, 0.2, true)
			}
			frame(Frame=21)
			if(is_excute){
				EFFECT_FOLLOW(hash40("sys_spin_wind"), hash40("sword2"), 0, 0, 0, 0, 0, 90, 0.5, true)
				EFFECT_FOLLOW(hash40("sys_spin_wind"), hash40("sword2"), 1.0, 0, 0, 0, 0, 90, 0.45, true)
				EFFECT_FOLLOW(hash40("sys_spin_wind"), hash40("sword2"), 3.0, 0, 0, 0, 0, 90, 0.35, true)
				EFFECT_FOLLOW(hash40("sys_spin_wind"), hash40("sword2"), 4.0, 0, 0, 0, 0, 90, 0.3, true)
				EFFECT_FOLLOW(hash40("sys_spin_wind"), hash40("sword2"), 6.0, 0, 0, 0, 0, 90, 0.2, true)
				EFFECT_FOLLOW(hash40("sys_spin_wind"), hash40("sword2"), 8.0, 0, 0, 0, 0, 90, 0.2, true)
			}
			frame(Frame=24)
			if(is_excute){
				EFFECT_FOLLOW(hash40("sys_spin_wind"), hash40("sword2"), 0, 0, 0, 0, 0, 90, 0.5, true)
				EFFECT_FOLLOW(hash40("sys_spin_wind"), hash40("sword2"), 1.0, 0, 0, 0, 0, 90, 0.45, true)
				EFFECT_FOLLOW(hash40("sys_spin_wind"), hash40("sword2"), 3.0, 0, 0, 0, 0, 90, 0.35, true)
				EFFECT_FOLLOW(hash40("sys_spin_wind"), hash40("sword2"), 4.0, 0, 0, 0, 0, 90, 0.3, true)
				EFFECT_FOLLOW(hash40("sys_spin_wind"), hash40("sword2"), 6.0, 0, 0, 0, 0, 90, 0.2, true)
				EFFECT_FOLLOW(hash40("sys_spin_wind"), hash40("sword2"), 8.0, 0, 0, 0, 0, 90, 0.2, true)
			}
			frame(Frame=28)
			if(is_excute){
				EFFECT_FOLLOW(hash40("sys_spin_wind"), hash40("sword2"), 0, 0, 0, 0, 0, 90, 0.5, true)
				EFFECT_FOLLOW(hash40("sys_spin_wind"), hash40("sword2"), 1.0, 0, 0, 0, 0, 90, 0.45, true)
				EFFECT_FOLLOW(hash40("sys_spin_wind"), hash40("sword2"), 3.0, 0, 0, 0, 0, 90, 0.35, true)
				EFFECT_FOLLOW(hash40("sys_spin_wind"), hash40("sword2"), 4.0, 0, 0, 0, 0, 90, 0.3, true)
				EFFECT_FOLLOW(hash40("sys_spin_wind"), hash40("sword2"), 6.0, 0, 0, 0, 0, 90, 0.2, true)
				EFFECT_FOLLOW(hash40("sys_spin_wind"), hash40("sword2"), 8.0, 0, 0, 0, 0, 90, 0.2, true)
			}
			frame(Frame=43)
			if(is_excute){
				EFFECT_OFF_KIND(hash40("toonlink_sword"), false, false)
			}
    });
}		
#[acmd_script(
    agent = "toonlink",
    script =  "sound_attackairhi",
    category = ACMD_SOUND,
	low_priority)]
unsafe fn tink_uair_snd(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		frame(Frame=10)
		if(is_excute){
			PLAY_SEQUENCE(hash40("seq_toonlink_rnd_attack"))
			PLAY_SE(hash40("se_toonlink_appeal_swing_s"))
		}
		frame(Frame=15)
		if(is_excute){
			PLAY_SE(hash40("se_toonlink_appeal_swing_s"))
		}
		frame(Frame=20)
		if(is_excute){
			PLAY_SE(hash40("se_toonlink_appeal_swing_s"))
		}
		frame(Frame=25)
		if(is_excute){
			PLAY_SE(hash40("se_toonlink_appeal_swing_s"))
		}
		frame(Frame=28)
		if(is_excute){
			PLAY_SE(hash40("se_toonlink_appeal_swing_m"))
		}
    });
}		
#[acmd_script(
    agent = "toonlink",
    scripts =  ["game_speciallw","game_specialairlw"],
    category = ACMD_GAME,
	low_priority)]
unsafe fn tink_bomb(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		frame(Frame=17)
		if(is_excute){
			WorkModule::on_flag(Flag=FIGHTER_LINK_STATUS_WORK_ID_FLAG_BOMB_GENERATE_LINKBOMB)
		}
    });
}	
#[fighter_frame( agent = FIGHTER_KIND_TOONLINK )]
fn tink_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent); 
		let status_kind = smash::app::lua_bind::StatusModule::status_kind(boma);
		let motion_kind = MotionModule::motion_kind(boma);
		let frame = MotionModule::frame(boma);
		let end_frame = MotionModule::end_frame(boma);
		let ENTRY_ID = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
		if ![*FIGHTER_STATUS_KIND_ATTACK_S4, *FIGHTER_STATUS_KIND_ATTACK_S4_START, *FIGHTER_STATUS_KIND_ATTACK_S4_HOLD, *FIGHTER_STATUS_KIND_SPECIAL_HI, *FIGHTER_STATUS_KIND_SPECIAL_N].contains(&status_kind) {
			ArticleModule::remove_exist(boma, *FIGHTER_TOONLINK_GENERATE_ARTICLE_BOW,smash::app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
		};
		if [hash40("special_air_hi")].contains(&motion_kind) {
			if end_frame-frame < 3.0 {
				MotionModule::change_motion(boma, smash::phx::Hash40::new("special_air_hi"), 12.0, 1.0, false, 0.0, false, false);
			};
			if frame == 3.0 {
				macros::PLAY_SEQUENCE(fighter, Hash40::new("seq_toonlink_rnd_attack"));
				macros::PLAY_SE(fighter, Hash40::new("se_toonlink_005"));
			};
			if frame > 2.0 && frame < 12.0 && (frame as i32 % 3 == 0) {
				macros::EFFECT(fighter, Hash40::new("sys_attack_speedline"),  Hash40::new("top"), 0, 3, 0, 90, 0, 0, 0.85, 0, 0, 0, 0, 0, 0, true);
			};
			if SPEED_Y[ENTRY_ID] <= 0.0 {
				if (ControlModule::get_command_flag_cat(boma, 1) & *FIGHTER_PAD_CMD_CAT2_FLAG_FALL_JUMP) != 0 {
					StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_FALL_SPECIAL, true);
				};
				AttackModule::clear_all(boma);
			};
		};
	}
}
		
pub fn install() {
    smashline::install_acmd_scripts!(
		tink_jab1,
		tink_jab2,
		tink_jab3,
		tink_da,
		//
		tink_dtilt,
		tink_ftilt,
		tink_utilt,
		//
		tink_dsmash,
		tink_fsmash,
		tink_fsmash_eff,
		tink_fsmash_expr,
		tink_fsmash_charge_expr,
		tink_fsmash_snd,
		tink_usmash,
		//
		tink_nair,
		tink_fair,
		tink_bair,
		tink_uair,
		tink_uair_eff,
		tink_uair_snd,
		//
		tink_bomb,
		//
		tink_dash,
		tink_dashback,
		//
		tink_hammer,
		//
		tink_upb_air,
		tink_upb_air_expr,
		tink_upb_air_eff,
		tink_upb_air_snd
    );
	smashline::install_agent_frames!(tink_frame);
}
