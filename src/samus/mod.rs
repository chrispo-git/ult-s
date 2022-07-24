use smash::hash40;
use smash::lib::lua_const::*;
use smash::app::lua_bind::*;
use smash::lua2cpp::*;
use smashline::*;
use smash_script::*;
#[acmd_script(
    agent = "samus",
    script =  "game_attack11",
    category = ACMD_GAME)]
unsafe fn samus_jab(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		if(is_excute){
			FT_MOTION_RATE(FSM=0.5)
		}
		frame(Frame=2)
		if(is_excute){
			FT_MOTION_RATE(FSM=1)
			ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=3.0, Angle=75, KBG=10, FKB=0, BKB=40, Size=2.0, X=0.0, Y=10.0, Z=6.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.8, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_PUNCH)
			ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=3.0, Angle=75, KBG=10, FKB=0, BKB=40, Size=2.3, X=0.0, Y=10.0, Z=8.8, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.8, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_PUNCH)
			ATTACK(ID=2, Part=0, Bone=hash40("top"), Damage=3.0, Angle=75, KBG=10, FKB=0, BKB=40, Size=2.5, X=0.0, Y=10.0, Z=12.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.8, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_PUNCH)
			AttackModule::set_add_reaction_frame(ID=0, Frames=5.0, Unk=false)
			AttackModule::set_add_reaction_frame(ID=1, Frames=5.0, Unk=false)
			AttackModule::set_add_reaction_frame(ID=2, Frames=5.0, Unk=false)
		}
		wait(Frames=3)
		if(is_excute){
			AttackModule::clear_all()
		}
		frame(Frame=6)
        if(is_excute){
            WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO)
		}
    });
}
#[acmd_script(
    agent = "samus",
    script =  "game_attacklw3",
    category = ACMD_GAME)]
unsafe fn samus_dtilt(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		frame(Frame=7)
		if(is_excute){
			ATTACK(ID=0, Part=0, Bone=hash40("kneer"), Damage=4.0, Angle=361, KBG=100, FKB=40, BKB=0, Size=4.6, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=-8, Trip=-1.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_G, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
			ATTACK(ID=1, Part=0, Bone=hash40("footr"), Damage=4.0, Angle=361, KBG=100, FKB=40, BKB=0, Size=4.6, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=-8, Trip=-1.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_G, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
			ATTACK(ID=2, Part=0, Bone=hash40("kneer"), Damage=4.0, Angle=90, KBG=100, FKB=40, BKB=0, Size=2.6, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.2, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_G, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
			ATTACK(ID=3, Part=0, Bone=hash40("footr"), Damage=4.0, Angle=90, KBG=100, FKB=40, BKB=0, Size=2.6, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.2, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_G, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
			AttackModule::set_attack_height_all(smash::app::AttackHeight(*ATTACK_HEIGHT_LOW), false)
			ATK_SET_SHIELD_SETOFF_MUL_arg3(ID1=0, ID2=1, ShieldstunMul=0.2)
			ATK_SET_SHIELD_SETOFF_MUL_arg3(ID1=2, ID2=3, ShieldstunMul=0.2)
			AttackModule::set_add_reaction_frame(ID=0, Frames=-7.0, Unk=false)
			AttackModule::set_add_reaction_frame(ID=1, Frames=-7.0, Unk=false)
		}
		frame(Frame=18)
		if(is_excute){
			AttackModule::clear_all()
		}
    });
}
#[acmd_script(
    agent = "samus",
    script =  "effect_attacklw3",
    category = ACMD_EFFECT)]
unsafe fn samus_dtilt_eff(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		frame(Frame=5)
		if(is_excute){
			FOOT_EFFECT(hash40("sys_dash_smoke"), hash40("top"), -5, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false)
		}
    });
}
#[acmd_script(
    agent = "samus",
    script =  "expression_attacklw3",
    category = ACMD_EXPRESSION)]
unsafe fn samus_dtilt_expr(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
}
#[acmd_script(
    agent = "samus",
    script =  "sound_attacklw3",
    category = ACMD_SOUND)]
unsafe fn samus_dtilt_snd(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		frame(Frame=6)
		if(is_excute){
			PLAY_SE(hash40("se_samus_swing_m"))
		}
    });
}
#[acmd_script(
    agent = "samus",
    script =  "game_attacklw4",
    category = ACMD_GAME)]
unsafe fn samus_dsmash(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		frame(Frame=1)
		FT_MOTION_RATE(FSM=0.5)
		frame(Frame=4)
		if(is_excute){
			WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD)
		}
		frame(Frame=8)
		FT_MOTION_RATE(FSM=1)
		if(is_excute){
			ATTACK(ID=0, Part=0, Bone=hash40("kneer"), Damage=10.0, Angle=30, KBG=70, FKB=0, BKB=70, Size=4.2, X=8.0, Y=-0.5, Z=0.0, X2=-2.0, Y2=0.0, Z2=0.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.3, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
		}
		wait(Frames=3)
		if(is_excute){
			AttackModule::clear_all()
		}
		frame(Frame=17)
		if(is_excute){
			ATTACK(ID=0, Part=0, Bone=hash40("kneer"), Damage=12.0, Angle=30, KBG=68, FKB=0, BKB=70, Size=4.6, X=8.0, Y=-0.5, Z=0.0, X2=-2.0, Y2=0.0, Z2=0.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.3, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
		}
		wait(Frames=2)
		if(is_excute){
			AttackModule::clear_all()
		}
    });
}
#[acmd_script(
    agent = "samus",
    script =  "game_specialhi",
    category = ACMD_GAME)]
unsafe fn samus_upb(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		FT_MOTION_RATE(FSM=1.5)
		if(is_excute){
			WorkModule::on_flag(Flag=FIGHTER_SAMUS_STATUS_SPECIAL_HI_FLAG_DISABLE_LR)
		}
		frame(Frame=4)
		FT_MOTION_RATE(FSM=1)
		if(is_excute){
			WorkModule::off_flag(Flag=FIGHTER_SAMUS_STATUS_SPECIAL_HI_FLAG_DISABLE_LR)
			WorkModule::on_flag(Flag=FIGHTER_SAMUS_STATUS_SPECIAL_HI_FLAG_ACC_X)
			ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=3.0, Angle=92, KBG=100, FKB=200, BKB=0, Size=3.0, X=0.0, Y=0.0, Z=4.75, X2=0.0, Y2=0.0, Z2=4.75, Hitlag=0.5, SDI=1.5, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_rush"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_SAMUS_SCREW, Type=ATTACK_REGION_BODY)
			ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=3.0, Angle=105, KBG=100, FKB=200, BKB=0, Size=3.0, X=0.0, Y=0.0, Z=-3.0, X2=0.0, Y2=0.0, Z2=-3.0, Hitlag=0.5, SDI=1.5, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_rush"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_SAMUS_SCREW, Type=ATTACK_REGION_BODY)
			ATTACK(ID=2, Part=0, Bone=hash40("top"), Damage=3.0, Angle=92, KBG=100, FKB=200, BKB=0, Size=3.0, X=0.0, Y=9.0, Z=4.75, X2=0.0, Y2=9.0, Z2=4.75, Hitlag=0.5, SDI=1.5, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_rush"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_SAMUS_SCREW, Type=ATTACK_REGION_BODY)
			ATTACK(ID=3, Part=0, Bone=hash40("top"), Damage=3.0, Angle=105, KBG=100, FKB=200, BKB=0, Size=3.0, X=0.0, Y=9.0, Z=-3.0, X2=0.0, Y2=9.0, Z2=-3.0, Hitlag=0.5, SDI=1.5, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_rush"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_SAMUS_SCREW, Type=ATTACK_REGION_BODY)
		}
		wait(Frames=3)
		if(is_excute){
			AttackModule::clear_all()
			ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=1.0, Angle=92, KBG=100, FKB=180, BKB=0, Size=3.0, X=0.0, Y=2.0, Z=4.75, X2=0.0, Y2=2.0, Z2=4.75, Hitlag=0.5, SDI=1.5, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=3, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_rush"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_SAMUS_SCREW, Type=ATTACK_REGION_BODY)
			ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=1.0, Angle=98, KBG=100, FKB=180, BKB=0, Size=3.0, X=0.0, Y=2.0, Z=-3.0, X2=0.0, Y2=2.0, Z2=-3.0, Hitlag=0.5, SDI=1.5, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=3, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_rush"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_SAMUS_SCREW, Type=ATTACK_REGION_BODY)
			ATTACK(ID=2, Part=0, Bone=hash40("top"), Damage=1.0, Angle=92, KBG=100, FKB=80, BKB=0, Size=3.0, X=0.0, Y=9.0, Z=4.75, X2=0.0, Y2=9.0, Z2=4.75, Hitlag=0.5, SDI=1.5, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=3, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_rush"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_SAMUS_SCREW, Type=ATTACK_REGION_BODY)
			ATTACK(ID=3, Part=0, Bone=hash40("top"), Damage=1.0, Angle=98, KBG=100, FKB=80, BKB=0, Size=3.0, X=0.0, Y=9.0, Z=-3.0, X2=0.0, Y2=9.0, Z2=-3.0, Hitlag=0.5, SDI=1.5, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=3, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_rush"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_SAMUS_SCREW, Type=ATTACK_REGION_BODY)
		}
		frame(Frame=12)
		if(is_excute){
			AttackModule::clear_all()
			WorkModule::on_flag(Flag=FIGHTER_SAMUS_STATUS_SPECIAL_HI_FLAG_DISABLE_LR)
			ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=1.0, Angle=84, KBG=100, FKB=100, BKB=0, Size=3.0, X=0.0, Y=2.0, Z=4.75, X2=0.0, Y2=2.0, Z2=4.75, Hitlag=0.5, SDI=1.5, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=3, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_rush"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_SAMUS_SCREW, Type=ATTACK_REGION_BODY)
			ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=1.0, Angle=90, KBG=100, FKB=100, BKB=0, Size=3.0, X=0.0, Y=2.0, Z=-3.0, X2=0.0, Y2=2.0, Z2=-3.0, Hitlag=0.5, SDI=1.5, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=3, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_rush"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_SAMUS_SCREW, Type=ATTACK_REGION_BODY)
			ATTACK(ID=2, Part=0, Bone=hash40("top"), Damage=1.0, Angle=84, KBG=100, FKB=40, BKB=0, Size=3.0, X=0.0, Y=9.0, Z=4.75, X2=0.0, Y2=9.0, Z2=4.75, Hitlag=0.5, SDI=1.5, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=3, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_rush"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_SAMUS_SCREW, Type=ATTACK_REGION_BODY)
			ATTACK(ID=3, Part=0, Bone=hash40("top"), Damage=1.0, Angle=90, KBG=100, FKB=40, BKB=0, Size=3.0, X=0.0, Y=9.0, Z=-3.0, X2=0.0, Y2=9.0, Z2=-3.0, Hitlag=0.5, SDI=1.5, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=3, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_rush"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_SAMUS_SCREW, Type=ATTACK_REGION_BODY)
		}
		frame(Frame=15)
		if(is_excute){
			AttackModule::clear_all()
			ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=1.0, Angle=82, KBG=100, FKB=40, BKB=0, Size=3.0, X=0.0, Y=2.0, Z=4.75, X2=0.0, Y2=2.0, Z2=4.75, Hitlag=0.5, SDI=1.5, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=3, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_rush"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_SAMUS_SCREW, Type=ATTACK_REGION_BODY)
			ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=1.0, Angle=90, KBG=100, FKB=40, BKB=0, Size=3.0, X=0.0, Y=2.0, Z=-3.0, X2=0.0, Y2=2.0, Z2=-3.0, Hitlag=0.5, SDI=1.5, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=3, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_rush"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_SAMUS_SCREW, Type=ATTACK_REGION_BODY)
			ATTACK(ID=2, Part=0, Bone=hash40("top"), Damage=1.0, Angle=82, KBG=100, FKB=20, BKB=0, Size=3.0, X=0.0, Y=9.0, Z=4.75, X2=0.0, Y2=9.0, Z2=4.75, Hitlag=0.5, SDI=1.5, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=3, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_rush"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_SAMUS_SCREW, Type=ATTACK_REGION_BODY)
			ATTACK(ID=3, Part=0, Bone=hash40("top"), Damage=1.0, Angle=90, KBG=100, FKB=20, BKB=0, Size=3.0, X=0.0, Y=9.0, Z=-3.0, X2=0.0, Y2=9.0, Z2=-3.0, Hitlag=0.5, SDI=1.5, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=3, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_rush"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_SAMUS_SCREW, Type=ATTACK_REGION_BODY)
		}
		frame(Frame=25)
		if(is_excute){
			AttackModule::clear_all()
			ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=2.0, Angle=70, KBG=190, FKB=0, BKB=56, Size=8.0, X=0.0, Y=6.5, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=2.5, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_elec"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_SAMUS_SCREW_FINISH, Type=ATTACK_REGION_BODY)
		}
		wait(Frames=2)
		if(is_excute){
			AttackModule::clear_all()
		}
    });
}
#[acmd_script(
    agent = "samus",
    script =  "game_aircatch",
    category = ACMD_GAME)]
unsafe fn samus_zair(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		if(is_excute){
			WorkModule::on_flag(Flag=FIGHTER_STATUS_AIR_LASSO_FLAG_CHECK)
		}
		wait(Frames=5)
		if(is_excute){
			WorkModule::off_flag(Flag=FIGHTER_STATUS_AIR_LASSO_FLAG_CHECK)
		}
		frame(Frame=8)
		if(is_excute){
			ATTACK(ID=0, Part=0, Bone=hash40("throw"), Damage=2.5, Angle=45, KBG=30, FKB=0, BKB=30, Size=3.0, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_B, SetWeight=false, ShieldDamage=1, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_elec"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_ELEC, Type=ATTACK_REGION_ENERGY)
		}
		frame(Frame=16)
		if(is_excute){
			ATTACK(ID=0, Part=1, Bone=hash40("throw"), Damage=4.0, Angle=70, KBG=100, FKB=0, BKB=30, Size=5.0, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_B, SetWeight=false, ShieldDamage=1, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_elec"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_ELEC, Type=ATTACK_REGION_ENERGY)
		}
		frame(Frame=20)
		if(is_excute){
			AttackModule::clear_all()
		}
    });
}		
#[acmd_script(
    agent = "samus",
    script =  "game_attackhi3",
    category = ACMD_GAME)]
unsafe fn samus_utilt(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		frame(Frame=8)
		if(is_excute){
			ATTACK(ID=0, Part=0, Bone=hash40("handr"), Damage=11.0, Angle=361, KBG=100, FKB=0, BKB=40, Size=5.3, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
			ATTACK(ID=1, Part=0, Bone=hash40("armr"), Damage=11.0, Angle=361, KBG=100, FKB=0, BKB=40, Size=4.3, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
			ATTACK(ID=2, Part=0, Bone=hash40("shoulderr"), Damage=11.0, Angle=361, KBG=100, FKB=0, BKB=40, Size=4.3, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
			AttackModule::set_attack_height_all(smash::app::AttackHeight(*ATTACK_HEIGHT_LOW), false)
			HIT_NODE(hash40("shoulderr"), HIT_STATUS_INVINCIBLE)
			HIT_NODE(hash40("armr"), HIT_STATUS_INVINCIBLE)
		}
		frame(Frame=14)
		if(is_excute){
			HIT_NODE(hash40("shoulderr"), HIT_STATUS_NORMAL)
			HIT_NODE(hash40("armr"), HIT_STATUS_NORMAL)
			AttackModule::clear_all()
		}
    });
}
#[acmd_script(
    agent = "samus",
    script =  "effect_attackhi3",
    category = ACMD_EFFECT)]
unsafe fn samus_utilt_eff(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		frame(Frame=8)
		if(is_excute){
			EFFECT_FOLLOW(hash40("sys_attack_arc_b"), hash40("top"), 1, 11, -2, 1.7, 10, 70, 1.4, true)
		}
    });
}
#[acmd_script(
    agent = "samus_missile",
    script =  "game_homing",
    category = ACMD_GAME)]
unsafe fn samus_homing(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		if(is_excute){
			ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=11.0, Angle=0, KBG=25, FKB=0, BKB=26, Size=2.4, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.3, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_SPEED, SetWeight=false, ShieldDamage=1, Trip=0.0, Rehit=0, Reflectable=true, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_fire"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_FIRE, Type=ATTACK_REGION_OBJECT)
			AttackModule::enable_safe_pos()
		}
    });
}
#[acmd_script(
    agent = "samus_supermissile",
    scripts =  ["game_ready", "game_straight"],
    category = ACMD_GAME)]
unsafe fn samus_super(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		if(is_excute){
			ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=15.0, Angle=65, KBG=65, FKB=0, BKB=50, Size=2.5, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.3, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_SPEED, SetWeight=false, ShieldDamage=5, Trip=0.0, Rehit=0, Reflectable=true, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_fire"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_BOMB, Type=ATTACK_REGION_OBJECT)
			AttackModule::enable_safe_pos()
		}
    });
}
#[fighter_frame( agent = FIGHTER_KIND_SAMUS )]
fn samus_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent); 
		let status_kind = smash::app::lua_bind::StatusModule::status_kind(boma);
		let ENTRY_ID = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
		let motion_kind = MotionModule::motion_kind(boma);
		let frame = MotionModule::frame(boma);
		if [hash40("attack_lw3")].contains(&motion_kind) {
			if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_JUMP) && frame > 6.0 {
				CancelModule::enable_cancel(boma);
			};
		};
    }
}
		
pub fn install() {
    smashline::install_acmd_scripts!(
        samus_jab,
		samus_dsmash,
		samus_upb,
		samus_zair,
		samus_dtilt,
		samus_dtilt_eff,
		samus_dtilt_snd,
		samus_dtilt_expr,
		samus_utilt,
		samus_utilt_eff,
		samus_homing,
		samus_super
    );
	smashline::install_agent_frames!(samus_frame);
}
