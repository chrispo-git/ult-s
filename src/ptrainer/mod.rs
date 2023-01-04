use smash::hash40;
use smash::lib::lua_const::*;
use smash::app::lua_bind::*;
use smash::lua2cpp::*;
use smashline::*;
use smash_script::*;use smash::app::lua_bind::*;

#[acmd_script(
    agent = "pfushigisou",
    scripts =  ["game_attackdash"],
    category = ACMD_GAME,
	low_priority)]
unsafe fn ivy_da(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		frame(Frame=4)
		if(is_excute){
			ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=12.0, Angle=74, KBG=57, FKB=0, BKB=80, Size=5.5, X=0.0, Y=4.3, Z=5.6, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=3, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_HEAD)
		}
		frame(Frame=5)
		if(is_excute){
			ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=12.0, Angle=74, KBG=57, FKB=0, BKB=80, Size=5.5, X=0.0, Y=4.7, Z=6.4, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=3, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_HEAD)
		}
		frame(Frame=7)
		if(is_excute){
			ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=12.0, Angle=74, KBG=57, FKB=0, BKB=80, Size=5.5, X=0.0, Y=4.9, Z=6.2, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=3, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_HEAD)
		}
		frame(Frame=8)
		if(is_excute){
			ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=12.0, Angle=74, KBG=57, FKB=0, BKB=80, Size=5.5, X=0.0, Y=4.9, Z=6.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=3, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_HEAD)
		}
		frame(Frame=10)
		if(is_excute){
			ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=12.0, Angle=74, KBG=57, FKB=0, BKB=80, Size=5.5, X=0.0, Y=4.9, Z=5.5, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=3, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_HEAD)
		}
		frame(Frame=12)
		if(is_excute){
			ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=10.0, Angle=73, KBG=50, FKB=0, BKB=45, Size=4.0, X=0.0, Y=6.0, Z=3.6, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=2, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_HEAD)
		}
		frame(Frame=16)
		if(is_excute){
			ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=10.0, Angle=73, KBG=50, FKB=0, BKB=45, Size=4.0, X=0.0, Y=6.0, Z=2.8, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=2, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_HEAD)
		}
		frame(Frame=17)
		if(is_excute){
			ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=10.0, Angle=73, KBG=50, FKB=0, BKB=45, Size=4.0, X=0.0, Y=5.7, Z=2.5, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=2, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_HEAD)
		}
		frame(Frame=19)
		if(is_excute){
			ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=10.0, Angle=73, KBG=50, FKB=0, BKB=45, Size=4.0, X=0.0, Y=5.2, Z=1.5, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=2, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_HEAD)
		}
		frame(Frame=20)
		if(is_excute){
			AttackModule::clear_all()
		}
    });
}
#[acmd_script(
    agent = "pfushigisou_leafcutter",
    scripts =  ["game_move"],
    category = ACMD_GAME,
	low_priority)]
unsafe fn ivy_sideb(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		if(is_excute){
			ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=8.0, Angle=70, KBG=50, FKB=0, BKB=54, Size=3.6, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.2, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_SPEED, SetWeight=false, ShieldDamage=-2, Trip=0.0, Rehit=17, Reflectable=true, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_OBJECT)
			AttackModule::enable_safe_pos()
		}
		frame(Frame=12)
		if(is_excute){
			ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=6.0, Angle=55, KBG=40, FKB=0, BKB=38, Size=3.3, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.2, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_SPEED, SetWeight=false, ShieldDamage=-1, Trip=0.0, Rehit=17, Reflectable=true, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_OBJECT)
		}
		frame(Frame=31)
		if(is_excute){
			ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=4.0, Angle=40, KBG=32, FKB=0, BKB=30, Size=3.0, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.2, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_SPEED, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=17, Reflectable=true, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_OBJECT)
		}
    });
}
#[acmd_script(
    agent = "pzenigame",
    scripts =  ["game_attacklw3"],
    category = ACMD_GAME,
	low_priority)]
unsafe fn squirtle_dtilt(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		frame(Frame=8)
		if(is_excute){
			ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=9.0, Angle=80, KBG=75, FKB=0, BKB=35, Size=3.0, X=0.0, Y=3.0, Z=9.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.4, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_water"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_WATER)
			ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=9.0, Angle=80, KBG=75, FKB=0, BKB=35, Size=3.6, X=0.0, Y=3.0, Z=12.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.4, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_water"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_WATER)
			ATTACK(ID=2, Part=0, Bone=hash40("top"), Damage=9.0, Angle=80, KBG=75, FKB=0, BKB=35, Size=2.1, X=0.0, Y=3.4, Z=15.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.4, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_water"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_WATER)
		}
		frame(Frame=9)
		if(is_excute){
			ATTACK(ID=2, Part=0, Bone=hash40("top"), Damage=9.0, Angle=80, KBG=75, FKB=0, BKB=35, Size=4.2, X=0.0, Y=3.4, Z=15.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.4, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_water"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_WATER)
		}
		wait(Frames=5)
		if(is_excute){
			AttackModule::clear_all()
		}
    });
}
#[acmd_script(
    agent = "plizardon",
    scripts =  ["game_attacks3", "game_attacks3hi", "game_attacks3lw"],
    category = ACMD_GAME,
	low_priority)]
unsafe fn zard_ftilt(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		frame(Frame=1)
		FT_MOTION_RATE(FSM=0.8)
		frame(Frame=11)
		FT_MOTION_RATE(FSM=1)
		if(is_excute){
			ATTACK(ID=0, Part=0, Bone=hash40("head"), Damage=10.0, Angle=30, KBG=60, FKB=0, BKB=60, Size=5.0, X=0.0, Y=0.0, Z=7.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.2, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.3, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_HEAD)
			ATTACK(ID=1, Part=0, Bone=hash40("neck"), Damage=10.0, Angle=30, KBG=60, FKB=0, BKB=60, Size=5.0, X=0.0, Y=0.0, Z=7.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.2, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.3, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_HEAD)
		}
		wait(Frames=2)
		if(is_excute){
			AttackModule::clear_all()
		}
    });
}
#[acmd_script(
    agent = "plizardon",
    scripts =  ["effect_attacks3", "effect_attacks3hi", "effect_attacks3lw"],
    category = ACMD_EFFECT,
	low_priority)]
unsafe fn zard_ftilt_eff(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		frame(Frame=6)
		if(is_excute){
			LANDING_EFFECT(hash40("sys_dash_smoke"), hash40("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false)
		}
		frame(Frame=11)
		if(is_excute){
			EFFECT_FLIP_ALPHA(hash40("sys_attack_impact"), hash40("sys_attack_impact"), hash40("head"), 0, 0, 0, 0, 0, 0, 1.3, 0, 0, 0, 0, 0, 0, true, EF_FLIP_YZ, 0.65)
		}
    });
}
#[acmd_script(
    agent = "plizardon",
    scripts =  ["sound_attacks3", "sound_attacks3hi", "sound_attacks3lw"],
    category = ACMD_SOUND,
	low_priority)]
unsafe fn zard_ftilt_snd(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		frame(Frame=8)
		if(is_excute){
			PLAY_SE(hash40("se_plizardon_attackl_l01"))
		}
    });
}
#[acmd_script(
    agent = "plizardon",
    scripts =  ["game_attacklw3"],
    category = ACMD_GAME,
	low_priority)]
unsafe fn zard_dtilt(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		frame(Frame=1)
		FT_MOTION_RATE(FSM=0.8)
		wait(Frames=10)
		FT_MOTION_RATE(FSM=1.0)
		frame(Frame=12)
		if(is_excute){
			ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=9.0, Angle=80, KBG=80, FKB=0, BKB=65, Size=4.0, X=0.0, Y=3.0, Z=19.1, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_fire"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_FIRE, Type=ATTACK_REGION_TAIL)
			ATTACK(ID=1, Part=0, Bone=hash40("fire"), Damage=9.0, Angle=80, KBG=80, FKB=0, BKB=65, Size=4.0, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_fire"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_FIRE, Type=ATTACK_REGION_TAIL)
			ATTACK(ID=2, Part=0, Bone=hash40("tail3"), Damage=7.0, Angle=361, KBG=100, FKB=0, BKB=40, Size=4.0, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_TAIL)
			ATTACK(ID=3, Part=0, Bone=hash40("top"), Damage=9.0, Angle=80, KBG=80, FKB=0, BKB=65, Size=4.0, X=0.0, Y=2.5, Z=16.7, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_fire"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_FIRE, Type=ATTACK_REGION_TAIL)
			ATTACK(ID=4, Part=0, Bone=hash40("tail2"), Damage=7.0, Angle=361, KBG=100, FKB=0, BKB=40, Size=4.0, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_TAIL)
		}
		wait(Frames=1)
		if(is_excute){
			AttackModule::clear(ID=0, false)
			ATTACK(ID=1, Part=0, Bone=hash40("fire"), Damage=9.0, Angle=80, KBG=80, FKB=0, BKB=65, Size=4.0, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_fire"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_FIRE, Type=ATTACK_REGION_TAIL)
			ATTACK(ID=2, Part=0, Bone=hash40("tail3"), Damage=7.0, Angle=361, KBG=100, FKB=0, BKB=40, Size=4.0, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_TAIL)
			ATTACK(ID=3, Part=0, Bone=hash40("tail4"), Damage=9.0, Angle=80, KBG=80, FKB=0, BKB=65, Size=4.0, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_fire"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_FIRE, Type=ATTACK_REGION_TAIL)
			ATTACK(ID=4, Part=0, Bone=hash40("tail2"), Damage=7.0, Angle=361, KBG=100, FKB=0, BKB=40, Size=4.0, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_TAIL)
		}
		wait(Frames=1)
		if(is_excute){
			AttackModule::clear_all()
		}
    });
}
#[acmd_script(
    agent = "plizardon",
    scripts =  ["effect_attacklw3"],
    category = ACMD_EFFECT,
	low_priority)]
unsafe fn zard_dtilt_eff(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		frame(Frame=11)
		if(is_excute){
			EFFECT_FOLLOW_NO_STOP(0x1229ac4d89u64, hash40("fire"), 0, 0, 0, 0, 0, 0, 1, true)
		}
		frame(Frame=12)
		if(is_excute){
			LANDING_EFFECT(hash40("sys_dash_smoke"), hash40("top"), -5, 0, -3, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false)
		}
		frame(Frame=19)
		if(is_excute){
			EFFECT_OFF_KIND(0x1229ac4d89u64, false, false)
		}
    });
}
#[acmd_script(
    agent = "plizardon",
    scripts =  ["sound_attacklw3"],
    category = ACMD_SOUND,
	low_priority)]
unsafe fn zard_dtilt_snd(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		frame(Frame=11)
		if(is_excute){
			PLAY_SEQUENCE(hash40("seq_plizardon_rnd_attack"))
			PLAY_SE(hash40("se_plizardon_attackl_s01"))
		}
    });
}
#[fighter_frame_callback]
pub fn pt(fighter : &mut L2CFighterCommon) {
    unsafe {
        let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent); 
		let status_kind = smash::app::lua_bind::StatusModule::status_kind(boma);
		let lua_state = fighter.lua_state_agent;
		let fighter_kind = smash::app::utility::get_kind(boma);
		if fighter_kind == *FIGHTER_KIND_PFUSHIGISOU {
			if [hash40("attack_dash")].contains(&MotionModule::motion_kind(boma)) && MotionModule::frame(boma) >= 4.0 && MotionModule::frame(boma) < 12.0 {
				acmd!(lua_state, {
					sv_module_access::shield(MA_MSC_CMD_REFLECTOR, COLLISION_KIND_REFLECTOR, 0, hash40("top"), 7.0, 0.0, 4.0, 5.0, 0.0, 4.0, 5.0, 0.0, 0.0, 500, false, 0.0, FIGHTER_REFLECTOR_GROUP_HOMERUNBAT)
				});
			};
			if [hash40("attack_air_hi")].contains(&MotionModule::motion_kind(boma)) {
				if MotionModule::frame(boma) < 12.0 {
					MotionModule::set_rate(boma, 0.7059);
				} else {
					MotionModule::set_rate(boma, 1.0);
				};
			};
		};
		if fighter_kind == *FIGHTER_KIND_PLIZARDON {
			if [hash40("special_hi"), hash40("special_air_hi")].contains(&MotionModule::motion_kind(boma)) {
				if MotionModule::frame(boma) < 6.0 {
					MotionModule::set_rate(boma, 3.0);
				} else {
					MotionModule::set_rate(boma, 1.0);
				};
			};
		};
	};
}	

pub fn install() {
    smashline::install_acmd_scripts!(
		//Squirtle
		squirtle_dtilt,
		//Ivysaur
		ivy_da,
		ivy_sideb,
		//Charizard
		zard_dtilt,
		zard_dtilt_eff,
		zard_dtilt_snd,
		zard_ftilt,
		zard_ftilt_eff,
		zard_ftilt_snd
    );
    smashline::install_agent_frame_callbacks!(pt);
}
