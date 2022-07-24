use smash::app::lua_bind::*;
use smash::lib::lua_const::*;
use smash::app::utility::get_kind;
use smash::hash40;
use smash::phx::Hash40;
use smash::lua2cpp::*;
use smashline::*;
use smash_script::*;
use crate::util::*;
static mut FLOAT : [i32; 8] = [0; 8];
static mut X : [f32; 8] = [0.0; 8];
static mut Y : [f32; 8] = [0.0; 8];
static mut FLOAT_MAX : i32 = 90;
static mut X_MAX : f32 = 1.155;
static mut X_ACCEL_ADD : f32 = 0.06;
static mut X_ACCEL_MUL : f32 = 0.12;
static mut Y_MAX : f32 = 0.8;
static mut Y_ACCEL_ADD : f32 = 0.06;
static mut Y_ACCEL_MUL : f32 = 0.06;

//New Utilt
#[acmd_script(
    agent = "ganon",
    script =  "game_attackhi3",
    category = ACMD_GAME)]
unsafe fn ganon_utilt(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		/*frame(Frame=11)
		if(is_excute){
			ATTACK(ID=0, Part=0, Bone=hash40("kneel"), Damage=3.0, Angle=368, KBG=100, FKB=0, BKB=0, Size=10.0, X=7.0, Y=-1.0, Z=0.5, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=true, ShieldDamage=0, Trip=-1.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
			ATTACK(ID=2, Part=0, Bone=hash40("legl"), Damage=3.0, Angle=368, KBG=100, FKB=0, BKB=0, Size=4.0, X=5.0, Y=0.0, Z=0.5, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=true, ShieldDamage=0, Trip=-1.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
			ATTACK(ID=3, Part=0, Bone=hash40("legl"), Damage=3.0, Angle=368, KBG=100, FKB=0, BKB=0, Size=3.0, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=true, ShieldDamage=0, Trip=-1.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
			rust {
				let hit = smash::phx::Vector2f { x: 10.0, y: 4.0 };
				AttackModule::set_vec_target_pos(fighter.module_accessor, 3, Hash40::new("top"), &hit, 0, false);
				AttackModule::set_vec_target_pos(fighter.module_accessor, 3, Hash40::new("top"), &hit, 1, false);
				AttackModule::set_vec_target_pos(fighter.module_accessor, 3, Hash40::new("top"), &hit, 2, false);
			}
		}*/
		frame(Frame=12)
		if(is_excute){
			ATTACK(ID=0, Part=0, Bone=hash40("kneel"), Damage=3.0, Angle=285, KBG=100, FKB=80, BKB=0, Size=10.0, X=7.0, Y=-1.0, Z=0.5, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=true, ShieldDamage=0, Trip=-1.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_A, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
			ATTACK(ID=2, Part=0, Bone=hash40("legl"), Damage=3.0, Angle=285, KBG=100, FKB=80, BKB=0, Size=4.0, X=5.0, Y=0.0, Z=0.5, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=true, ShieldDamage=0, Trip=-1.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_A, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
			ATTACK(ID=3, Part=0, Bone=hash40("legl"), Damage=3.0, Angle=285, KBG=100, FKB=80, BKB=0, Size=3.0, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=true, ShieldDamage=0, Trip=-1.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_A, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
			ATTACK(ID=4, Part=0, Bone=hash40("kneel"), Damage=3.0, Angle=285, KBG=100, FKB=60, BKB=0, Size=10.0, X=7.0, Y=-1.0, Z=0.5, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=true, ShieldDamage=0, Trip=-1.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_G, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
			ATTACK(ID=5, Part=0, Bone=hash40("legl"), Damage=3.0, Angle=285, KBG=100, FKB=60, BKB=0, Size=4.0, X=5.0, Y=0.0, Z=0.5, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=true, ShieldDamage=0, Trip=-1.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_G, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
			ATTACK(ID=6, Part=0, Bone=hash40("legl"), Damage=3.0, Angle=285, KBG=100, FKB=60, BKB=0, Size=3.0, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=true, ShieldDamage=0, Trip=-1.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_G, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
		}
		frame(Frame=15)
		if(is_excute){
			AttackModule::clear_all()
			ATTACK(ID=0, Part=0, Bone=hash40("legl"), Damage=13.0, Angle=270, KBG=80, FKB=0, BKB=60, Size=5.0, X=3.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_G, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_fire"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_FIRE, Type=ATTACK_REGION_KICK)
			ATTACK(ID=1, Part=0, Bone=hash40("legl"), Damage=13.0, Angle=270, KBG=80, FKB=0, BKB=60, Size=5.0, X=9.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_G, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_fire"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_FIRE, Type=ATTACK_REGION_KICK)
			ATTACK(ID=2, Part=0, Bone=hash40("legl"), Damage=13.0, Angle=361, KBG=80, FKB=0, BKB=60, Size=5.0, X=3.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_A, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_fire"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_FIRE, Type=ATTACK_REGION_KICK)
			ATTACK(ID=3, Part=0, Bone=hash40("legl"), Damage=13.0, Angle=361, KBG=80, FKB=0, BKB=60, Size=5.0, X=9.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_A, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_fire"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_FIRE, Type=ATTACK_REGION_KICK)
			ATTACK(ID=4, Part=0, Bone=hash40("top"), Damage=13.0, Angle=90, KBG=80, FKB=0, BKB=60, Size=6.0, X=0.0, Y=4.0, Z=16.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_fire"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_FIRE, Type=ATTACK_REGION_KICK)
			ATTACK(ID=5, Part=0, Bone=hash40("top"), Damage=13.0, Angle=90, KBG=80, FKB=0, BKB=60, Size=6.0, X=0.0, Y=7.0, Z=16.0, X2=0.0, Y2=22.0, Z2=16.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_fire"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_FIRE, Type=ATTACK_REGION_KICK)
			ATK_SET_SHIELD_SETOFF_MUL(ID=0, ShieldstunMul=0.5)
			ATK_SET_SHIELD_SETOFF_MUL(ID=1, ShieldstunMul=0.5)
			ATK_SET_SHIELD_SETOFF_MUL(ID=2, ShieldstunMul=0.5)
			ATK_SET_SHIELD_SETOFF_MUL(ID=3, ShieldstunMul=0.5)
			ATK_SET_SHIELD_SETOFF_MUL(ID=4, ShieldstunMul=0.5)
			ATK_SET_SHIELD_SETOFF_MUL(ID=5, ShieldstunMul=0.5)
		}
		frame(Frame=16)
		FT_MOTION_RATE(FSM=1.35)
		frame(Frame=18)
		if(is_excute){
			AttackModule::clear_all()
		}
		frame(Frame=32)
		FT_MOTION_RATE(FSM=0.75)
		if(is_excute){
			CancelModule::enable_cancel()
		}
    });
}
#[acmd_script(
    agent = "ganon",
    script =  "effect_attackhi3",
    category = ACMD_EFFECT)]
unsafe fn ganon_utilt_eff(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		frame(Frame=13)
		if(is_excute){
			EFFECT_FOLLOW(hash40("sys_attack_arc_d"), hash40("top"), 0, 16, -0.5, 0, 0, -90, 1.4, true)
			LAST_EFFECT_SET_RATE(1.4)
		}
		frame(Frame=15)
		if(is_excute){
			EFFECT(hash40("sys_bomb_b"), hash40("top"), 16, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false)
		}
		frame(Frame=16)
		if(is_excute){
			EFFECT(hash40("sys_crown"), hash40("top"), 16, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false)
			LANDING_EFFECT(hash40("null"), hash40("top"), 16, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false)
			LANDING_EFFECT(hash40("sys_h_smoke_b"), hash40("top"), -6, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false)
		}
    });
}		
#[acmd_script(
    agent = "ganon",
    script =  "expression_attackhi3",
    category = ACMD_EXPRESSION)]
unsafe fn ganon_utilt_expr(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		frame(Frame=15)
		if(is_excute){
			QUAKE(CAMERA_QUAKE_KIND_M)
		}
    });
}	
#[acmd_script(
    agent = "ganon",
    script =  "sound_attackhi3",
    category = ACMD_SOUND)]
unsafe fn ganon_utilt_snd(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		frame(Frame=13)
		if(is_excute){
			PLAY_SE(hash40("se_ganon_attackhard_h02"))
		}
		frame(Frame=15)
		if(is_excute){
			PLAY_SEQUENCE(hash40("seq_ganon_rnd_attack"))
			PLAY_SE(hash40("se_ganon_attackhard_h03"))
		}
    });
}	
#[acmd_script(
    agent = "ganon",
    script =  "game_specialn",
    category = ACMD_GAME)]
unsafe fn ganon_teleport(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		FT_MOTION_RATE(FSM=0.5)
		frame(Frame=16)
		if(is_excute){
			WHOLE_HIT(HIT_STATUS_XLU)
			VisibilityModule::set_whole(false)
			JostleModule::set_status(false)
			SET_SPEED_EX(7.2, 0, KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN)
		}
		frame(Frame=35)
		FT_MOTION_RATE(FSM=1)
		if(is_excute){
			WHOLE_HIT(HIT_STATUS_NORMAL)
			VisibilityModule::set_whole(true)
			JostleModule::set_status(true)
			SET_SPEED_EX(0.0, 0, KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN)
		}
		frame(Frame=41)
		if(is_excute){
			ATTACK(ID=0, Part=0, Bone=hash40("hip"), Damage=24.0, Angle=361, KBG=100, FKB=150, BKB=0, Size=12.0, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=true, ShieldDamage=-10, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_G, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_purple"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_FIRE, Type=ATTACK_REGION_PUNCH)
			ATTACK(ID=1, Part=0, Bone=hash40("hip"), Damage=24.0, Angle=361, KBG=100, FKB=120, BKB=0, Size=8.0, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=true, ShieldDamage=-10, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_A, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_purple"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_FIRE, Type=ATTACK_REGION_PUNCH)
		}
		frame(Frame=49)
		if(is_excute){
			AttackModule::clear_all()
		}
    });
}
#[acmd_script(
    agent = "ganon",
    script =  "effect_specialn",
    category = ACMD_EFFECT)]
unsafe fn ganon_teleport_eff(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		for(6 Iterations) {
			if(is_excute){
				EFFECT_FOLLOW(hash40("ganon_entry_aura"), hash40("emit"), 0, 0, 0, 0, 0, 0, 1, true)	
			}
			wait(Frames=1)
		}
		frame(Frame=12)
		if(is_excute){
			EFFECT(hash40("ganon_entry"), hash40("top"), 0, 12, 0, 0, 0, 0, 0.6, 0, 0, 0, 0, 0, 0, true)
			LAST_EFFECT_SET_RATE(2.5)
		}
		frame(Frame=35)
		if(is_excute){
			EFFECT(hash40("ganon_entry"), hash40("top"), 0, 12, 0, 0, 0, 0, 0.6, 0, 0, 0, 0, 0, 0, true)
			LAST_EFFECT_SET_RATE(2.5)
		}
		frame(Frame=41)
		if(is_excute){
			EFFECT_FOLLOW(hash40("ganon_majinken_start"), hash40("hip"), 0, 0, 0, 0, 0, 0, 1.75, true)
			EFFECT_FOLLOW(hash40("ganon_majinken_start"), hash40("haver"), 0, 0, 0, 0, 0, 0, 1.0, true)
			EFFECT_FOLLOW(hash40("ganon_majinken_start"), hash40("havel"), 0, 0, 0, 0, 0, 0, 1.0, true)
			EFFECT_FOLLOW(hash40("ganon_majinken_start"), hash40("footr"), 0, 0, 0, 0, 0, 0, 1.0, true)
			EFFECT_FOLLOW(hash40("ganon_majinken_start"), hash40("footl"), 0, 0, 0, 0, 0, 0, 1.0, true)
		}
    });
}
#[acmd_script(
    agent = "ganon",
    script =  "sound_specialn",
    category = ACMD_SOUND)]
unsafe fn ganon_teleport_snd(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		frame(Frame=15)
		if(is_excute){
			PLAY_SE(hash40("se_ganon_appeal_h01"))
		}
		frame(Frame=35)
		if(is_excute){
			PLAY_SE(hash40("se_common_spirits_critical_l_tail"))
		}
		frame(Frame=39)
		if(is_excute){
			PLAY_SE(hash40("vc_ganon_appeal_h01"))
			PLAY_SE(hash40("se_ganon_special_l02"))
		}
    });
}
#[acmd_script(
    agent = "ganon",
    script =  "expression_specialn",
    category = ACMD_EXPRESSION)]
unsafe fn ganon_teleport_expr(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		frame(Frame=41)
		if(is_excute){
			QUAKE(CAMERA_QUAKE_KIND_L)
			RUMBLE_HIT(hash40("rbkind_attack_critical"), 0)
		}
    });
}
#[acmd_script(
    agent = "ganon",
    script =  "game_specialnturn",
    category = ACMD_GAME)]
unsafe fn ganon_warlock(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		frame(Frame=11)
		if(is_excute){
			REVERSE_LR()
			sv_module_access::damage(MSC=MA_MSC_DAMAGE_DAMAGE_NO_REACTION, Type=DAMAGE_NO_REACTION_MODE_ALWAYS, DamageThreshold=0)
		}
		frame(Frame=70)
		if(is_excute){
			ATTACK(ID=0, Part=0, Bone=hash40("shoulderl"), Damage=37.0, Angle=361, KBG=100, FKB=0, BKB=50, Size=5.0, X=2.4, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_purple"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_PUNCH)
			ATTACK(ID=1, Part=0, Bone=hash40("bust"), Damage=37.0, Angle=361, KBG=100, FKB=0, BKB=50, Size=4.7, X=0.0, Y=1.0, Z=1.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_purple"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_PUNCH)
			ATTACK(ID=2, Part=0, Bone=hash40("arml"), Damage=37.0, Angle=361, KBG=100, FKB=0, BKB=50, Size=4.8, X=5.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_purple"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_PUNCH)
		}
		frame(Frame=74)
		if(is_excute){
			sv_module_access::damage(MSC=MA_MSC_DAMAGE_DAMAGE_NO_REACTION, Type=DAMAGE_NO_REACTION_MODE_NORMAL, DamageThreshold=0)
			AttackModule::clear_all()
		}
    });
}
#[acmd_script(
    agent = "ganon",
    script =  "game_attackairf",
    category = ACMD_GAME)]
unsafe fn ganon_fair(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		FT_MOTION_RATE(FSM=0.7143)
		frame(Frame=7)
		if(is_excute){
			WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
		}
		frame(Frame=14)
		FT_MOTION_RATE(FSM=1)
		if(is_excute){
			ATTACK(ID=0, Part=0, Bone=hash40("shoulderr"), Damage=12.0, Angle=361, KBG=85, FKB=0, BKB=40, Size=6.0, X=-1.1, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_FIRE, Type=ATTACK_REGION_PUNCH)
			ATTACK(ID=1, Part=0, Bone=hash40("armr"), Damage=13.0, Angle=361, KBG=85, FKB=0, BKB=40, Size=7.5, X=4.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.1, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_FIRE, Type=ATTACK_REGION_PUNCH)
		}
		wait(Frames=6)
		if(is_excute){
			AttackModule::clear_all()
		}
		frame(Frame=43)
		if(is_excute){
			WorkModule::off_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
		}
    });
}
#[acmd_script(
    agent = "ganon",
    script =  "effect_attackairf",
    category = ACMD_EFFECT)]
unsafe fn ganon_fair_eff(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		frame(Frame=12)
		if(is_excute){
			EFFECT_FOLLOW(hash40("sys_attack_arc"), hash40("top"), 0, 12, 6, 180, -180, 100, 1.1, false)
			LAST_EFFECT_SET_RATE(1.5)
		}
    });
}
#[acmd_script(
    agent = "ganon",
    script =  "game_attackhi4",
    category = ACMD_GAME)]
unsafe fn ganon_usmash(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		FT_MOTION_RATE(FSM=0.25)
		if(is_excute){
			ArticleModule::generate_article(FIGHTER_GANON_GENERATE_ARTICLE_SWORD, false, 0)
		}
		frame(Frame=10)
		if(is_excute){
			WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD)
		}
		frame(Frame=12)
		FT_MOTION_RATE(FSM=1)
		frame(Frame=20)
		if(is_excute){
			ATTACK(ID=0, Part=0, Bone=hash40("haver"), Damage=19.0, Angle=85, KBG=71, FKB=0, BKB=40, Size=5.0, X=0.0, Y=6.5, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
			ATTACK(ID=1, Part=0, Bone=hash40("haver"), Damage=19.0, Angle=78, KBG=71, FKB=0, BKB=40, Size=4.5, X=0.0, Y=14.5, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
			ATTACK(ID=2, Part=0, Bone=hash40("armr"), Damage=19.0, Angle=75, KBG=75, FKB=0, BKB=40, Size=4.0, X=2.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
		}
		wait(Frames=6)
		if(is_excute){
			AttackModule::clear_all()
		}
    });
}	
		
#[acmd_script(
    agent = "ganon",
    script =  "game_attack11",
    category = ACMD_GAME)]
unsafe fn ganon_jab(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		frame(Frame=3)
		FT_MOTION_RATE(FSM=0.5)
		frame(Frame=8)
		FT_MOTION_RATE(FSM=1)
		if(is_excute){
			ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=11.0, Angle=361, KBG=74, FKB=0, BKB=41, Size=4.4, X=0.0, Y=12.0, Z=11.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_elec"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_HEAVY, Type=ATTACK_REGION_PUNCH)
			ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=11.0, Angle=361, KBG=74, FKB=0, BKB=41, Size=5.0, X=0.0, Y=12.0, Z=19.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_elec"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_HEAVY, Type=ATTACK_REGION_PUNCH)
			ATTACK(ID=2, Part=0, Bone=hash40("top"), Damage=11.0, Angle=361, KBG=74, FKB=0, BKB=41, Size=3.5, X=0.0, Y=12.0, Z=7.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_HEAVY, Type=ATTACK_REGION_PUNCH)
		}
		wait(Frames=2)
		if(is_excute){
			AttackModule::clear_all()
		}
		FT_MOTION_RATE(FSM=0.9)
    });
}	
#[acmd_script(
    agent = "ganon",
    script =  "game_attacks4",
    category = ACMD_GAME)]
unsafe fn ganon_fsmash(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		if(is_excute){
			ArticleModule::generate_article(FIGHTER_GANON_GENERATE_ARTICLE_SWORD, false, 0)
		}
		frame(Frame=15)
		if(is_excute){
			WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD)
		}
		frame(Frame=29)
		if(is_excute){
			ATTACK(ID=0, Part=0, Bone=hash40("haver"), Damage=20.0, Angle=40, KBG=75, FKB=0, BKB=61, Size=4.5, X=0.0, Y=4.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
			ATTACK(ID=1, Part=0, Bone=hash40("haver"), Damage=20.0, Angle=40, KBG=75, FKB=0, BKB=61, Size=4.0, X=0.0, Y=12.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
		}
		frame(Frame=30)
		if(is_excute){
			rust {
				let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
				if AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_ALL) == false {
					acmd!({	
						ATTACK(ID=0, Part=0, Bone=hash40("haver"), Damage=20.0, Angle=40, KBG=75, FKB=0, BKB=61, Size=5.0, X=0.0, Y=6.5, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
						ATTACK(ID=1, Part=0, Bone=hash40("haver"), Damage=20.0, Angle=40, KBG=75, FKB=0, BKB=61, Size=4.5, X=0.0, Y=14.5, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
						ATTACK(ID=2, Part=0, Bone=hash40("haver"), Damage=20.0, Angle=40, KBG=75, FKB=0, BKB=61, Size=4.0, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
					});
				};
			}
		}
		wait(Frames=2)
		if(is_excute){
			AttackModule::clear_all()
		}
    });
}

#[acmd_script(
    agent = "ganon",
    script =  "game_speciallw",
    category = ACMD_GAME)]
unsafe fn ganon_downb(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		frame(Frame=10)
		if(is_excute){
			JostleModule::set_status(false)
			rust {
				println!("ganon_downb!")
			}
		}
		frame(Frame=16)
		if(is_excute){
			ATTACK(ID=0, Part=0, Bone=hash40("kneer"), Damage=14.0, Angle=45, KBG=65, FKB=0, BKB=65, Size=4.0, X=2.7, Y=0.0, Z=0.0, X2=-1.5, Y2=0.0, Z2=0.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=4, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_purple"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
			ATTACK(ID=1, Part=0, Bone=hash40("kneer"), Damage=16.0, Angle=45, KBG=65, FKB=0, BKB=65, Size=5.0, X=7.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=4, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_purple"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
			SET_SPEED_EX(-2.25, 0, KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN)
			FT_MOTION_RATE(FSM=0.575)
		}			
		wait(Frames=1)
		if(is_excute){
			WorkModule::on_flag(Flag=FIGHTER_GANON_STATUS_WORK_ID_FLAG_GANON_KICK_WALL_CHECK)
		}
		frame(Frame=36)
		if(is_excute){
			AttackModule::clear_all()
			SET_SPEED_EX(0, 0, KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN)
			JostleModule::set_status(true)
		}
    });
}	
#[acmd_script(
    agent = "ganon",
    script =  "game_specialairn",
    category = ACMD_GAME)]
unsafe fn ganon_float(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
}		
#[acmd_script(
    agent = "kirby",
    script =  "sound_ganonspecialairn",
    category = ACMD_SOUND)]
unsafe fn kirb_ganon_floats(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
}		
#[acmd_script(
    agent = "ganon",
    script =  "sound_specialairn",
    category = ACMD_SOUND)]
unsafe fn ganon_floats(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
}			
#[acmd_script(
    agent = "ganon",
    script =  "effect_specialairn",
    category = ACMD_EFFECT)]
unsafe fn ganon_floate(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
}		
#[acmd_script(
    agent = "ganon",
    script =  "game_attackairhi",
    category = ACMD_GAME)]
unsafe fn ganon_uair(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		if(is_excute){
			WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
		}
		frame(Frame=8)
		if(is_excute){
			ATTACK(ID=0, Part=0, Bone=hash40("legl"), Damage=13.0, Angle=361, KBG=100, FKB=0, BKB=35, Size=4.8, X=3.2, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
			ATTACK(ID=1, Part=0, Bone=hash40("kneel"), Damage=12.0, Angle=361, KBG=100, FKB=0, BKB=35, Size=5.8, X=6.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
		}
		wait(Frames=3)
		if(is_excute){
			ATTACK(ID=0, Part=0, Bone=hash40("legl"), Damage=12.0, Angle=30, KBG=80, FKB=0, BKB=30, Size=4.8, X=3.2, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
			ATTACK(ID=1, Part=0, Bone=hash40("kneel"), Damage=10.0, Angle=30, KBG=80, FKB=0, BKB=30, Size=5.8, X=6.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
		}
		wait(Frames=3)
		if(is_excute){
			ATTACK(ID=0, Part=0, Bone=hash40("legl"), Damage=8.0, Angle=0, KBG=70, FKB=0, BKB=20, Size=4.8, X=3.2, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_B, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
			ATTACK(ID=1, Part=0, Bone=hash40("kneel"), Damage=6.0, Angle=0, KBG=70, FKB=0, BKB=20, Size=5.8, X=6.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_B, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
		}
		wait(Frames=5)
		if(is_excute){
			AttackModule::clear_all()
		}
		frame(Frame=25)
		if(is_excute){
			WorkModule::off_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
		}
    });
}		
#[fighter_frame_callback]
pub fn ganon_float(fighter : &mut L2CFighterCommon) {
    unsafe {
        let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent); 
		let status_kind = smash::app::lua_bind::StatusModule::status_kind(boma);
		let fighter_kind = smash::app::utility::get_kind(boma);
		let lua_state = fighter.lua_state_agent;
		let stick_x = ControlModule::get_stick_x(boma) * PostureModule::lr(boma);
		let stick_y = ControlModule::get_stick_y(boma);
		let speed_x = KineticModule::get_sum_speed_x(boma, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
		let speed_y = KineticModule::get_sum_speed_y(boma, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
		let ENTRY_ID = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
		if fighter_kind == *FIGHTER_KIND_GANON {
			if StatusModule::situation_kind(boma) != SITUATION_KIND_AIR || smash::app::sv_information::is_ready_go() == false || [*FIGHTER_STATUS_KIND_WIN, *FIGHTER_STATUS_KIND_LOSE, *FIGHTER_STATUS_KIND_DEAD].contains(&status_kind) {
				FLOAT[ENTRY_ID] = 0;
			};
			if [hash40("special_air_n")].contains(&MotionModule::motion_kind(boma)) {
					StatusModule::set_situation_kind(boma, smash::app::SituationKind(*SITUATION_KIND_AIR), true);
					StatusModule::set_keep_situation_air(boma, true);
			};
			if [hash40("appeal_hi_l"), hash40("appeal_hi_r")].contains(&MotionModule::motion_kind(boma)) {
					PostureModule::reverse_lr(boma);
					PostureModule::update_rot_y_lr(boma);
					StatusModule::change_status_request_from_script(boma, *FIGHTER_GANON_STATUS_KIND_SPECIAL_N_TURN, true);
			};
			if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_LW {
				if MotionModule::frame(boma) > 16.0 && ControlModule::check_button_on_trriger(boma, *CONTROL_PAD_BUTTON_SPECIAL) {
					StatusModule::change_status_request_from_script(boma, *FIGHTER_GANON_STATUS_KIND_SPECIAL_LW_END, true);
				};
			};
			if FLOAT[ENTRY_ID] == 1{
				CAN_NEUTRALB[ENTRY_ID] = 1;
			} else {
				CAN_NEUTRALB[ENTRY_ID] = 0;
			};
			if FLOAT[ENTRY_ID] == 1{
				if KineticModule::get_kinetic_type(boma) == *FIGHTER_KINETIC_TYPE_MOTION_AIR && [*FIGHTER_STATUS_KIND_SPECIAL_LW, *FIGHTER_STATUS_KIND_SPECIAL_HI, *FIGHTER_STATUS_KIND_SPECIAL_S, *FIGHTER_GANON_STATUS_KIND_SPECIAL_AIR_S_CATCH, *FIGHTER_GANON_STATUS_KIND_SPECIAL_AIR_S_END].contains(&status_kind) == false {
					KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_FALL);
				};
				if MotionModule::motion_kind(boma) == hash40("special_air_n") {
					StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_FALL, true);
				};
			};
			if [*FIGHTER_STATUS_KIND_ESCAPE_AIR, *FIGHTER_STATUS_KIND_ESCAPE_AIR_SLIDE, *FIGHTER_STATUS_KIND_JUMP_AERIAL].contains(&status_kind) && FLOAT[ENTRY_ID] > 1{
				FLOAT[ENTRY_ID] = 1;
			};
			if [*FIGHTER_STATUS_KIND_SPECIAL_LW, *FIGHTER_STATUS_KIND_SPECIAL_HI, *FIGHTER_STATUS_KIND_SPECIAL_S, *FIGHTER_GANON_STATUS_KIND_SPECIAL_AIR_S_CATCH, *FIGHTER_GANON_STATUS_KIND_SPECIAL_AIR_S_END].contains(&status_kind){
				FLOAT[ENTRY_ID] = 0;
			};
			if FLOAT[ENTRY_ID] > 1{
				if FLOAT[ENTRY_ID] % 30 == 0 {
					macros::EFFECT_FOLLOW(fighter, Hash40::new("ganon_rekkikyaku"), Hash40::new("kneer"), 12, -1.5, 0, 0, 0, 0, 0.5, true);
					macros::EFFECT_FOLLOW(fighter, Hash40::new("ganon_rekkikyaku"), Hash40::new("kneel"), 12, -1.5, 0, 0, 0, 0, 0.5, true);
				};
				if [*FIGHTER_STATUS_KIND_FALL, *FIGHTER_STATUS_KIND_FALL_AERIAL].contains(&status_kind) && MotionModule::motion_kind(boma) != hash40("special_air_n") {
					MotionModule::change_motion(boma, smash::phx::Hash40::new("special_air_n"), 1.0, 1.0, false, 0.0, false, false);
				};
				FLOAT[ENTRY_ID] -= 1;
				if KineticModule::get_kinetic_type(boma) != *FIGHTER_KINETIC_TYPE_MOTION_AIR {
					KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_MOTION_AIR);
				};
				if ControlModule::check_button_off(boma, *CONTROL_PAD_BUTTON_SPECIAL){
					FLOAT[ENTRY_ID] = 1;
				};
				let mut y_add = 0.0;
				let mut x_add = 0.0;
				if stick_x > 0.2 {
					x_add = ((stick_x-0.2)*X_ACCEL_MUL) + X_ACCEL_ADD;
					if speed_x > X_MAX || speed_x < -X_MAX{
						x_add = 0.0;
					};
				};
				if stick_x < -0.2 {
					x_add = ((stick_x+0.2)*X_ACCEL_MUL) + X_ACCEL_ADD;
					if speed_x > X_MAX || speed_x < -X_MAX{
						x_add = 0.0;
					};
				};
				if stick_y > 0.2 {
					y_add = ((stick_y-0.2)*Y_ACCEL_MUL) + Y_ACCEL_ADD;
					if speed_y > Y_MAX || speed_y < -Y_MAX{
						y_add = 0.0;
					};
				};
				if stick_y < -0.2 {
					y_add = ((stick_y+0.2)*Y_ACCEL_MUL) + Y_ACCEL_ADD;
					if speed_y > Y_MAX || speed_y < -Y_MAX{
						y_add = 0.0;
					};
				};
				if stick_x > -0.2 && stick_x < 0.2 && stick_y > -0.2 && stick_y < 0.2 {
					if speed_y > 0.0 {
						y_add = -Y_ACCEL_MUL - Y_ACCEL_ADD;
					} else if speed_y < 0.0{
						y_add = Y_ACCEL_MUL + Y_ACCEL_ADD;
					};
					let mut x_add = 0.0;
					if speed_x > 0.0 {
						x_add = -X_ACCEL_MUL - X_ACCEL_ADD;
					} else if speed_x < 0.0{
						x_add = X_ACCEL_MUL + X_ACCEL_ADD;
					};
				};
				x_add = (stick_x)*X_ACCEL_MUL;
				y_add = (stick_y)*X_ACCEL_MUL;
				if x_add > 0.0 && X[ENTRY_ID] > X_MAX {
					x_add = 0.0;
				};
				if x_add < 0.0 && X[ENTRY_ID] < X_MAX*-1.0 {
					x_add = 0.0;
				};
				if y_add > 0.0 && Y[ENTRY_ID] > Y_MAX {
					y_add = 0.0;
				};
				if y_add < 0.0 && Y[ENTRY_ID] < Y_MAX*-1.0 {
					y_add = 0.0;
				};
				println!("x{}, y{}", X[ENTRY_ID], Y[ENTRY_ID]);
				println!("x_add{}, y_add{}", x_add, y_add);
				let speed = smash::phx::Vector3f { x: x_add, y: y_add, z: 0.0 };
				KineticModule::add_speed(boma, &speed);
				X[ENTRY_ID] += x_add;
				Y[ENTRY_ID] += y_add;
			} else {
				X[ENTRY_ID] = 0.0;
				Y[ENTRY_ID] = 0.0;
			};
			if [hash40("special_air_n")].contains(&MotionModule::motion_kind(boma)) {
				if MotionModule::frame(boma) == 2.0 && FLOAT[ENTRY_ID] == 0 {
					FLOAT[ENTRY_ID] = FLOAT_MAX;
					macros::PLAY_SE(fighter, Hash40::new("se_ganon_special_l01"));
				};
				if FLOAT[ENTRY_ID] == 1 {
					StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_FALL_AERIAL, true);
				};
				CancelModule::enable_cancel(boma);
			};
		};
		if fighter_kind == *FIGHTER_KIND_KIRBY {
			if StatusModule::situation_kind(boma) != SITUATION_KIND_AIR || smash::app::sv_information::is_ready_go() == false || [*FIGHTER_STATUS_KIND_WIN, *FIGHTER_STATUS_KIND_LOSE, *FIGHTER_STATUS_KIND_DEAD].contains(&status_kind) {
				FLOAT[ENTRY_ID] = 0;
			};
			if [hash40("ganon_special_air_n")].contains(&MotionModule::motion_kind(boma)) {
					StatusModule::set_situation_kind(boma, smash::app::SituationKind(*SITUATION_KIND_AIR), true);
					StatusModule::set_keep_situation_air(boma, true);
			};
			if FLOAT[ENTRY_ID] == 1{
				if KineticModule::get_kinetic_type(boma) == *FIGHTER_KINETIC_TYPE_MOTION_AIR && [*FIGHTER_STATUS_KIND_SPECIAL_LW, *FIGHTER_STATUS_KIND_SPECIAL_HI, *FIGHTER_STATUS_KIND_SPECIAL_S, *FIGHTER_GANON_STATUS_KIND_SPECIAL_AIR_S_CATCH, *FIGHTER_GANON_STATUS_KIND_SPECIAL_AIR_S_END].contains(&status_kind) == false {
					KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_FALL);
				};
				if MotionModule::motion_kind(boma) == hash40("ganon_special_air_n") {
					StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_FALL, true);
				};
			};
			if [*FIGHTER_STATUS_KIND_ESCAPE_AIR, *FIGHTER_STATUS_KIND_ESCAPE_AIR_SLIDE, *FIGHTER_STATUS_KIND_JUMP_AERIAL].contains(&status_kind) && FLOAT[ENTRY_ID] > 1{
				FLOAT[ENTRY_ID] = 1;
			};
			if [*FIGHTER_STATUS_KIND_SPECIAL_LW, *FIGHTER_STATUS_KIND_SPECIAL_HI, *FIGHTER_STATUS_KIND_SPECIAL_S, *FIGHTER_GANON_STATUS_KIND_SPECIAL_AIR_S_CATCH, *FIGHTER_GANON_STATUS_KIND_SPECIAL_AIR_S_END].contains(&status_kind){
				FLOAT[ENTRY_ID] = 0;
			};
			if FLOAT[ENTRY_ID] > 1{
				if FLOAT[ENTRY_ID] % 30 == 0 {
					macros::EFFECT_FOLLOW(fighter, Hash40::new("ganon_rekkikyaku"), Hash40::new("haver"), 0, 3, 0, 0, 0, 0, 0.22, true);
					macros::EFFECT_FOLLOW(fighter, Hash40::new("ganon_rekkikyaku"), Hash40::new("havel"), 0, 3, 0, 0, 0, 0, 0.22, true);
				};
				if [*FIGHTER_STATUS_KIND_FALL, *FIGHTER_STATUS_KIND_FALL_AERIAL].contains(&status_kind) && MotionModule::motion_kind(boma) != hash40("ganon_special_air_n") {
					MotionModule::change_motion(boma, smash::phx::Hash40::new("ganon_special_air_n"), 1.0, 1.0, false, 0.0, false, false);
				};
				FLOAT[ENTRY_ID] -= 1;
				if KineticModule::get_kinetic_type(boma) != *FIGHTER_KINETIC_TYPE_MOTION_AIR {
					KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_MOTION_AIR);
				};
				if ControlModule::check_button_off(boma, *CONTROL_PAD_BUTTON_SPECIAL){
					FLOAT[ENTRY_ID] = 1;
				};
				let mut y_add = 0.0;
				let mut x_add = 0.0;
				if stick_x > 0.2 {
					x_add = ((stick_x-0.2)*X_ACCEL_MUL) + X_ACCEL_ADD;
					if speed_x > X_MAX || speed_x < -X_MAX{
						x_add = 0.0;
					};
				};
				if stick_x < -0.2 {
					x_add = ((stick_x+0.2)*X_ACCEL_MUL) + X_ACCEL_ADD;
					if speed_x > X_MAX || speed_x < -X_MAX{
						x_add = 0.0;
					};
				};
				if stick_y > 0.2 {
					y_add = ((stick_y-0.2)*Y_ACCEL_MUL) + Y_ACCEL_ADD;
					if speed_y > Y_MAX || speed_y < -Y_MAX{
						y_add = 0.0;
					};
				};
				if stick_y < -0.2 {
					y_add = ((stick_y+0.2)*Y_ACCEL_MUL) + Y_ACCEL_ADD;
					if speed_y > Y_MAX || speed_y < -Y_MAX{
						y_add = 0.0;
					};
				};
				if stick_x > -0.2 && stick_x < 0.2 && stick_y > -0.2 && stick_y < 0.2 {
					if speed_y > 0.0 {
						y_add = -Y_ACCEL_MUL - Y_ACCEL_ADD;
					} else if speed_y < 0.0{
						y_add = Y_ACCEL_MUL + Y_ACCEL_ADD;
					};
					let mut x_add = 0.0;
					if speed_x > 0.0 {
						x_add = -X_ACCEL_MUL - X_ACCEL_ADD;
					} else if speed_x < 0.0{
						x_add = X_ACCEL_MUL + X_ACCEL_ADD;
					};
				};
				x_add = (stick_x)*X_ACCEL_MUL;
				y_add = (stick_y)*X_ACCEL_MUL;
				if x_add > 0.0 && X[ENTRY_ID] > X_MAX {
					x_add = 0.0;
				};
				if x_add < 0.0 && X[ENTRY_ID] < X_MAX*-1.0 {
					x_add = 0.0;
				};
				if y_add > 0.0 && Y[ENTRY_ID] > Y_MAX {
					y_add = 0.0;
				};
				if y_add < 0.0 && Y[ENTRY_ID] < Y_MAX*-1.0 {
					y_add = 0.0;
				};
				println!("x{}, y{}", X[ENTRY_ID], Y[ENTRY_ID]);
				println!("x_add{}, y_add{}", x_add, y_add);
				let speed = smash::phx::Vector3f { x: x_add, y: y_add, z: 0.0 };
				KineticModule::add_speed(boma, &speed);
				X[ENTRY_ID] += x_add;
				Y[ENTRY_ID] += y_add;
			} else {
				X[ENTRY_ID] = 0.0;
				Y[ENTRY_ID] = 0.0;
			};
			if [hash40("ganon_special_air_n")].contains(&MotionModule::motion_kind(boma)) {
				if MotionModule::frame(boma) == 0.3 && FLOAT[ENTRY_ID] == 0 {
					FLOAT[ENTRY_ID] = FLOAT_MAX;
					macros::PLAY_SE(fighter, Hash40::new("se_ganon_special_l01"));
				};
				if FLOAT[ENTRY_ID] == 1 {
					StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_FALL_AERIAL, true);
				};
				CancelModule::enable_cancel(boma);
				MotionModule::set_rate(boma, 0.05);
			};
		};
	};
}
#[acmd_script(
    agent = "ganon",
    script =  "expression_specialairn",
    category = ACMD_EXPRESSION)]
unsafe fn ganon_float_expr(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
}		
		
pub fn install() {
    smashline::install_acmd_scripts!(
		ganon_fair,
		ganon_jab,
		ganon_usmash,
		ganon_fsmash,
		ganon_downb,
		ganon_float,
		ganon_floats,
		ganon_float_expr,
		kirb_ganon_floats,
		ganon_floate,
		ganon_uair,
		ganon_utilt,
		ganon_utilt_eff,
		ganon_utilt_snd,
		ganon_utilt_expr,
		ganon_fair_eff,
		ganon_teleport,
		ganon_teleport_eff,
		ganon_teleport_snd,
		ganon_teleport_expr,
		ganon_warlock
    );
    smashline::install_agent_frame_callbacks!(ganon_float);
}
