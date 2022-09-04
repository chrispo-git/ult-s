use smash::hash40;
use smash::lib::lua_const::*;
use smash::app::lua_bind::*;
use smash::lua2cpp::*;
use smashline::*;
use smash_script::*;
use smash::phx::{Hash40, Vector2f};
use crate::util::*;
static mut BAN_SIDEB : [bool; 8] = [false; 8];
static mut HAS_BOUNCE : [bool; 8] = [false; 8];
static mut IS_JUMP : [bool; 8] = [false; 8];

#[acmd_script(
    agent = "wario",
    script =  "game_attackhi3",
    category = ACMD_GAME,
	low_priority)]
unsafe fn wario_utilt(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		frame(Frame=8)
		if(is_excute){
			ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=6.0, Angle=92, KBG=130, FKB=0, BKB=28, Size=5.0, X=0.0, Y=18.0, Z=5.0, X2=0.0, Y2=18.0, Z2=-5.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_PUNCH)
			ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=6.0, Angle=95, KBG=130, FKB=0, BKB=28, Size=5.0, X=0.0, Y=6.0, Z=1.5, X2=0.0, Y2=6.0, Z2=-1.5, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_PUNCH)
		}
		wait(Frames=3)
		if(is_excute){
			ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=5.0, Angle=90, KBG=120, FKB=0, BKB=20, Size=4.0, X=0.0, Y=14.5, Z=3.5, X2=0.0, Y2=14.5, Z2=-3.5, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_PUNCH)
			AttackModule::clear(ID=0, false)
		}
		frame(Frame=20)
		if(is_excute){
			AttackModule::clear_all()
		}
    });
}
#[acmd_script(
    agent = "wario",
    script =  "game_attackairf",
    category = ACMD_GAME,
	low_priority)]
unsafe fn wario_fair(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		frame(Frame=5)
		if(is_excute){
			WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
			ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=7.0, Angle=361, KBG=97, FKB=0, BKB=30, Size=5.5, X=0.0, Y=5.4, Z=11.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
			ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=7.0, Angle=361, KBG=97, FKB=0, BKB=30, Size=3.5, X=0.0, Y=3.0, Z=5.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
		}
		frame(Frame=7)
		if(is_excute){
			ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=4.5, Angle=361, KBG=80, FKB=0, BKB=15, Size=3.5, X=0.0, Y=5.0, Z=11.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
			ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=4.5, Angle=361, KBG=80, FKB=0, BKB=15, Size=2.6, X=0.0, Y=3.0, Z=5.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
		}
		frame(Frame=18)
		if(is_excute){
			AttackModule::clear_all()
		}
		frame(Frame=27)
		if(is_excute){
			WorkModule::off_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
		}
		frame(Frame=32)
		if(is_excute){
			CancelModule::enable_cancel()
		}
    });
}	
#[acmd_script(
    agent = "wario",
    scripts =  ["game_specialssearch", "game_specialairssearch"],
    category = ACMD_GAME,
	low_priority)]
unsafe fn wario_sideb(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
	let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent); 
	let ENTRY_ID = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    acmd!(lua_state, {
		frame(Frame=7)
		if(is_excute){
			sv_module_access::damage(MSC=MA_MSC_DAMAGE_DAMAGE_NO_REACTION, Type=DAMAGE_NO_REACTION_MODE_DAMAGE_POWER, DamageThreshold=11)
			ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=11.0, Angle=361, KBG=97, FKB=0, BKB=60, Size=4.0, X=0.0, Y=5.0, Z=5.8, X2=0.0, Y2=9.7, Z2=5.8, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_BODY)
			ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=11.0, Angle=361, KBG=97, FKB=0, BKB=60, Size=4.0, X=0.0, Y=6.0, Z=2.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_BODY)
		}
		frame(Frame=11)
		if(is_excute){
			sv_module_access::damage(MSC=MA_MSC_DAMAGE_DAMAGE_NO_REACTION, Type=DAMAGE_NO_REACTION_MODE_NORMAL, DamageThreshold=0)
			ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=5.0, Angle=90, KBG=97, FKB=0, BKB=60, Size=4.0, X=0.0, Y=5.0, Z=5.8, X2=0.0, Y2=9.7, Z2=5.8, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_BODY)
			ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=5.0, Angle=90, KBG=97, FKB=0, BKB=60, Size=4.0, X=0.0, Y=6.0, Z=2.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_BODY)
		}
		frame(Frame=25)
		if(is_excute){
			AttackModule::clear_all()
		}
    });
}	
#[acmd_script(
    agent = "wario",
    scripts =  ["effect_specialssearch", "effect_specialairssearch"],
    category = ACMD_EFFECT,
	low_priority)]
unsafe fn wario_sideb_eff(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
	let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent); 
	let ENTRY_ID = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    acmd!(lua_state, {
		frame(Frame=8)
		if(is_excute){
			EFFECT_FOLLOW(0x11f62f23d0, hash40("top"), 0, 4, 16, 0, 0, 0, 1, true)
			LANDING_EFFECT(hash40("sys_atk_smoke"), hash40("top"), 9, 0, 0, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, false)
			LAST_EFFECT_SET_RATE(0.6)
		}
		frame(Frame=12)
		if(is_excute){
			FOOT_EFFECT(hash40("sys_turn_smoke"), hash40("top"), 9, 0, 0, 0, 0, 0, 1.5, 0, 0, 0, 0, 0, 0, false)
		}
		frame(Frame=17)
		if(is_excute){
			FOOT_EFFECT(hash40("sys_turn_smoke"), hash40("top"), 9, 0, 0, 0, 0, 0, 1.3, 0, 0, 0, 0, 0, 0, false)
		}
    });
}	
#[acmd_script(
    agent = "wario",
    scripts =  ["sound_specialssearch", "sound_specialairssearch"],
    category = ACMD_SOUND,
	low_priority)]
unsafe fn wario_sideb_snd(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
	let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent); 
	let ENTRY_ID = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
}	
#[acmd_script(
    agent = "wario",
    scripts =  ["sound_specialsdrive", "sound_specialsride", "sound_specialairs", "sound_specialairs", "sound_specials", "sound_specialsstart", "sound_specialairsstart"],
    category = ACMD_SOUND,
	low_priority)]
unsafe fn wario_sideb_start_snd(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
	let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent); 
	let ENTRY_ID = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    acmd!(lua_state, {
		if(is_excute){
			PLAY_SE(hash40("vc_wario_006"))
			PLAY_SE(hash40("se_wario_attackdash"))
		}
	});
}	
#[acmd_script(
    agent = "wario",
    script =  "game_catchattack",
    category = ACMD_GAME,
	low_priority)]
unsafe fn wario_pummel(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		frame(Frame=2)
		if(is_excute){
			ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=1.3, Angle=361, KBG=100, FKB=30, BKB=0, Size=5.0, X=0.0, Y=10.0, Z=7.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_coin"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_COIN, Type=ATTACK_REGION_PUNCH)
			ATTACK(ID=1, Part=1, Bone=hash40("top"), Damage=0.3, Angle=361, KBG=100, FKB=30, BKB=0, Size=5.0, X=0.0, Y=10.0, Z=7.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_PUNCH)
			AttackModule::set_catch_only_all(true, false)
		}
		wait(Frames=1)
		if(is_excute){
			AttackModule::clear_all()
		}
    });
}	
#[acmd_script(
    agent = "wario",
    script =  "game_attackdash",
    category = ACMD_GAME,
	low_priority)]
unsafe fn wario_da(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		FT_MOTION_RATE(FSM=1.28)
		frame(Frame=7)
		FT_MOTION_RATE(FSM=1)
		if(is_excute){
			ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=7.0, Angle=70, KBG=70, FKB=0, BKB=80, Size=4.5, X=0.0, Y=2.0, Z=11.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_KICK)
			ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=7.0, Angle=70, KBG=70, FKB=0, BKB=80, Size=4.0, X=0.0, Y=2.0, Z=6.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_KICK)
		}
		frame(Frame=9)
		if(is_excute){
			ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=4.0, Angle=180, KBG=100, FKB=50, BKB=0, Size=4.5, X=0.0, Y=2.0, Z=11.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.2, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=1.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_G, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
			ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=4.0, Angle=180, KBG=100, FKB=50, BKB=0, Size=4.0, X=0.0, Y=2.0, Z=6.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.2, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=1.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_G, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
			ATTACK(ID=2, Part=0, Bone=hash40("top"), Damage=4.0, Angle=20, KBG=100, FKB=50, BKB=0, Size=4.5, X=0.0, Y=2.0, Z=11.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.2, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=1.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_A, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
			ATTACK(ID=3, Part=0, Bone=hash40("top"), Damage=4.0, Angle=20, KBG=100, FKB=50, BKB=0, Size=4.0, X=0.0, Y=2.0, Z=6.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.2, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=1.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_A, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
		}
		frame(Frame=30)
		if(is_excute){
			AttackModule::clear_all()
		}
    });
}	
#[acmd_script(
    agent = "wario",
    script =  "effect_attackdash",
    category = ACMD_EFFECT,
	low_priority)]
unsafe fn wario_da_eff(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		frame(Frame=1)
		if(is_excute){
			FOOT_EFFECT(hash40("sys_run_smoke"), hash40("top"), -1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false)
		}
		frame(Frame=7)
		if(is_excute){
			LANDING_EFFECT(hash40("sys_down_smoke"), hash40("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false)
		}
    });
}	
#[acmd_script(
    agent = "wario",
    script =  "game_attacklw4",
    category = ACMD_GAME,
	low_priority)]
unsafe fn wario_dsmash(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		frame(Frame=3)
		FT_MOTION_RATE(FSM=2.5)
		frame(Frame=4)
		if(is_excute){
			WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD)
		}
		frame(Frame=5)
		FT_MOTION_RATE(FSM=1)
		frame(Frame=10)
		if(is_excute){
			ATTACK(ID=0, Part=0, Bone=hash40("haver"), Damage=16.0, Angle=361, KBG=93, FKB=0, BKB=20, Size=8.5, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_G_d, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_PUNCH)
			ATTACK(ID=1, Part=0, Bone=hash40("haver"), Damage=16.0, Angle=270, KBG=93, FKB=0, BKB=20, Size=8.5, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_PUNCH)
		}
		wait(Frames=3)
		if(is_excute){
			ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=10.0, Angle=361, KBG=30, FKB=0, BKB=80, Size=9.0, X=0.0, Y=9.0, Z=13.5, X2=0.0, Y2=9.0, Z2=-13.5, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_G, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_BODY)
			ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=10.0, Angle=361, KBG=30, FKB=0, BKB=80, Size=9.0, X=0.0, Y=9.0, Z=13.5, X2=0.0, Y2=9.0, Z2=-13.5, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_G, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_BODY)
		}
		wait(Frames=2)
		if(is_excute){
			AttackModule::clear_all()
		}
		frame(Frame=50)
		if(is_excute){
			CancelModule::enable_cancel()
		}
    });
}	
#[acmd_script(
    agent = "wario",
    script =  "effect_attacklw4",
    category = ACMD_EFFECT,
	low_priority)]
unsafe fn wario_dsmash_eff(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		frame(Frame=2)
		if(is_excute){
			EFFECT(hash40("sys_smash_flash"), hash40("handr"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true)
		}
		frame(Frame=10)
		if(is_excute){
			EFFECT(hash40("sys_crown"), hash40("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true)
			LAST_EFFECT_SET_RATE(0.8)
			LANDING_EFFECT(hash40("sys_landing_smoke"), hash40("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true)
			LAST_EFFECT_SET_RATE(0.8)
			QUAKE(CAMERA_QUAKE_KIND_M)
		}		
    });
}	
#[acmd_script(
    agent = "wario",
    script =  "sound_attacklw4",
    category = ACMD_SOUND,
	low_priority)]
unsafe fn wario_dsmash_snd(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		frame(Frame=5)
		if(is_excute){
			STOP_SE(hash40("se_common_smash_start"))
		}
		frame(Frame=9)
		if(is_excute){
			PLAY_SE(hash40("vc_wario_attack07"))
			PLAY_SE(hash40("se_wario_smash_s01"))
		}
		frame(Frame=10)
		if(is_excute){
			PLAY_SE(hash40("se_common_throw_02"))
		}
    });
}	
// A Once-Per-Fighter-Frame that only applies to Mario. Neat!
#[fighter_frame( agent = FIGHTER_KIND_WARIO )]
fn wario_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        println!("It'sa me, Mario, wahoooooooo!");
        let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent); 
		let status_kind = smash::app::lua_bind::StatusModule::status_kind(boma);
		let ENTRY_ID = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
		let motion_kind = MotionModule::motion_kind(boma);
		let frame = MotionModule::frame(boma);
		let situation_kind = StatusModule::situation_kind(boma);
		let is_near_ground = GroundModule::ray_check(boma, &Vector2f{ x: PostureModule::pos_x(boma), y: PostureModule::pos_y(boma)}, &Vector2f{ x: 0.0, y: -1.0}, true);
		println!("is near ground {}", is_near_ground);
		if ![*FIGHTER_STATUS_KIND_ENTRY, *FIGHTER_STATUS_KIND_WIN].contains(&status_kind) && smash::app::sv_information::is_ready_go() {
			ArticleModule::remove_exist(boma, *FIGHTER_WARIO_GENERATE_ARTICLE_WARIOBIKE,smash::app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
		};
		if situation_kind != *SITUATION_KIND_AIR && MotionModule::motion_kind(boma) != hash40("special_s_search"){
			BAN_SIDEB[ENTRY_ID] = false;
			HAS_BOUNCE[ENTRY_ID] = false;
		};
		if BAN_SIDEB[ENTRY_ID] == true {
			CAN_SIDEB[ENTRY_ID] = 1;
		} else {
			CAN_SIDEB[ENTRY_ID] = 0;
		};
		if MotionModule::motion_kind(boma) == hash40("special_s_search") && StatusModule::situation_kind(boma) == *SITUATION_KIND_AIR && status_kind != *FIGHTER_STATUS_KIND_ATTACK_DASH{
			BAN_SIDEB[ENTRY_ID] = true;
			if MotionModule::frame(boma) >= 20.0 {
				if is_near_ground == 0 {
					if MotionModule::frame(boma) >= 70.0 {
						StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_FALL, true);
					};
				} else if MotionModule::frame(boma) > 31.0 {
						if MotionModule::frame(boma) < 37.0 {
							if !HAS_BOUNCE[ENTRY_ID] {
								StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_WAIT, true);
							};
						} else {
							StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_LANDING, true);
						};
				};
				if is_near_ground == 1 {
					MotionModule::set_rate(boma, 0.5);
				} else {
					MotionModule::set_rate(boma, 1.0);
				};
			};
			if !HAS_BOUNCE[ENTRY_ID] && MotionModule::frame(boma) < 24.0 {
				//macros::SET_SPEED_EX(fighter, -0.3, -0.4, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
				let dummy = 0;
			} else {
				macros::COL_NORMAL(fighter);
			};
			if is_near_ground == 1 && !HAS_BOUNCE[ENTRY_ID]  && !IS_JUMP[ENTRY_ID]  && MotionModule::frame(boma) < 20.0 {
				if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_JUMP) || ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_JUMP_MINI) { //|| ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_FLICK_JUMP) {
					WorkModule::on_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_NO_SPEED_OPERATION_CHK);
					macros::SET_SPEED_EX(fighter, 3.0, 2.5, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
					WorkModule::off_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_NO_SPEED_OPERATION_CHK);
					KineticModule::suspend_energy(boma, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
					macros::EFFECT_FOLLOW(fighter, Hash40::new("sys_jump_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, false);
					macros::PLAY_SE(fighter, Hash40::new("se_wario_jump01"));
					IS_JUMP[ENTRY_ID] = true;
				};
			};
			if AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_ALL)  || GroundModule::is_wall_touch_line(boma, *GROUND_TOUCH_FLAG_SIDE as u32){
				if !AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT) && !HAS_BOUNCE[ENTRY_ID] && !IS_JUMP[ENTRY_ID]{
					macros::PLAY_SE(fighter, Hash40::new("se_wario_landing01"));
					WorkModule::on_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_NO_SPEED_OPERATION_CHK);
					macros::SET_SPEED_EX(fighter, -0.5, 2.0, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
					WorkModule::off_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_NO_SPEED_OPERATION_CHK);
					KineticModule::suspend_energy(boma, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
					macros::QUAKE(fighter, *CAMERA_QUAKE_KIND_M);
				};
				if AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT) && !HAS_BOUNCE[ENTRY_ID] {
					macros::PLAY_SE(fighter, Hash40::new("se_wario_landing01"));
					macros::SET_SPEED_EX(fighter, -0.5, 2.0, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
					macros::QUAKE(fighter, *CAMERA_QUAKE_KIND_M);
				};
				HAS_BOUNCE[ENTRY_ID] = true;
			}
			if HAS_BOUNCE[ENTRY_ID] &&  WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_HIT_STOP_ATTACK_SUSPEND_FRAME) < 1 {
				if MotionModule::frame(boma) < 25.0 {
					MotionModule::change_motion(boma, smash::phx::Hash40::new("special_s_search"), 25.0, 1.0, false, 0.0, false, false);
				} else {
					MotionModule::set_rate(boma, 0.1);
					let stop_rise  = smash::phx::Vector3f { x: 0.0, y: 1.0, z: 1.0 };
					KineticModule::mul_speed(boma, &stop_rise, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
				};
				if MotionModule::frame(boma) >= 26.0 && AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT) {
					StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_FALL, true);
				};
				if MotionModule::frame(boma) >= 28.5 {
					StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_FALL, true);
				};
			};
		} else {
			IS_JUMP[ENTRY_ID] = false;
		};
		if MotionModule::motion_kind(boma) == hash40("special_s_search") && StatusModule::situation_kind(boma) == *SITUATION_KIND_GROUND {
			if HAS_BOUNCE[ENTRY_ID] &&  WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_HIT_STOP_ATTACK_SUSPEND_FRAME) < 1 {
				if MotionModule::frame(boma) < 25.0 {
					MotionModule::change_motion(boma, smash::phx::Hash40::new("special_s_search"), 25.0, 1.0, false, 0.0, false, false);
				} else {
					MotionModule::set_rate(boma, 0.2);
				};
			};
		};
		if [*FIGHTER_WARIO_STATUS_KIND_SPECIAL_S_START, *FIGHTER_WARIO_STATUS_KIND_SPECIAL_S_SEARCH, *FIGHTER_WARIO_STATUS_KIND_SPECIAL_S_RIDE, *FIGHTER_WARIO_STATUS_KIND_SPECIAL_S_WHEELIE, *FIGHTER_WARIO_STATUS_KIND_SPECIAL_S_DOWN, *FIGHTER_WARIO_STATUS_KIND_SPECIAL_S_DRIVE, *FIGHTER_STATUS_KIND_SPECIAL_S].contains(&status_kind) {
			if MotionModule::motion_kind(boma) != hash40("special_s_search") {
				MotionModule::change_motion_inherit_frame(boma, smash::phx::Hash40::new("special_s_search"), 0.0, 1.0, 0.0, false, false);
			};
			if MotionModule::frame(boma) < 48.0 {
				StatusModule::set_situation_kind(boma, smash::app::SituationKind(*SITUATION_KIND_AIR), true);
				StatusModule::set_keep_situation_air(boma, true);
			} else if is_near_ground == 1{
				StatusModule::set_keep_situation_air(boma, false);
				StatusModule::set_situation_kind(boma, smash::app::SituationKind(*SITUATION_KIND_GROUND), true);
			};
			/*if StatusModule::is_situation_changed(boma) {
				StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_LANDING, true);
			};*/	
			if [5.0, 20.0].contains(&MotionModule::frame(boma)){
				macros::COL_NORMAL(fighter);
				macros::FLASH(fighter, 1.0, 0.0, 0.0, 0.5);
			} else if [10.0, 25.0].contains(&MotionModule::frame(boma)){
				macros::COL_NORMAL(fighter);
				macros::FLASH(fighter, 0.0, 0.0, 1.0, 0.5);
			} else if [15.0].contains(&MotionModule::frame(boma)){
				macros::COL_NORMAL(fighter);
				macros::FLASH(fighter, 1.0, 1.0, 0.0, 0.5);
			} else if MotionModule::frame(boma) > 25.0{
				macros::COL_NORMAL(fighter);
			};
			if StatusModule::situation_kind(boma) == *SITUATION_KIND_AIR {
					if !HAS_BOUNCE[ENTRY_ID] && !IS_JUMP[ENTRY_ID] {
						if KineticModule::get_kinetic_type(boma) != *FIGHTER_KINETIC_TYPE_MOTION_AIR{
							KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_MOTION_AIR);
						};
					} else {
						if KineticModule::get_kinetic_type(boma) != *FIGHTER_KINETIC_TYPE_FALL{
							KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_FALL);
						};
					};
					if !HAS_BOUNCE[ENTRY_ID] && !IS_JUMP[ENTRY_ID] && frame >= 42.0 {
						if KineticModule::get_kinetic_type(boma) != *FIGHTER_KINETIC_TYPE_FALL{
							KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_FALL);
						};
					};
			} else {
					if KineticModule::get_kinetic_type(boma) != *FIGHTER_KINETIC_TYPE_MOTION {
						KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_MOTION);
					};
			};
		};
		/*ArticleModule::remove_exist(boma, *FIGHTER_WARIO_GENERATE_ARTICLE_WARIOBIKE,smash::app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
		if [*FIGHTER_STATUS_KIND_SPECIAL_S, *FIGHTER_WARIO_STATUS_KIND_SPECIAL_S_BUMP, *FIGHTER_WARIO_STATUS_KIND_SPECIAL_S_DOWN, *FIGHTER_WARIO_STATUS_KIND_SPECIAL_S_RIDE, *FIGHTER_WARIO_STATUS_KIND_SPECIAL_S_DRIVE, *FIGHTER_WARIO_STATUS_KIND_SPECIAL_S_START, *FIGHTER_WARIO_STATUS_KIND_SPECIAL_S_APPEAL, *FIGHTER_WARIO_STATUS_KIND_SPECIAL_S_ESCAPE, *FIGHTER_WARIO_STATUS_KIND_SPECIAL_S_WHEELIE, *FIGHTER_WARIO_STATUS_KIND_SPECIAL_S_TURN_END, *FIGHTER_WARIO_STATUS_KIND_SPECIAL_S_TURN_LOOP, *FIGHTER_WARIO_STATUS_KIND_SPECIAL_S_THROWN_OUT, *FIGHTER_WARIO_STATUS_KIND_SPECIAL_S_TURN_START, *FIGHTER_WARIO_STATUS_KIND_SPECIAL_S_ESCAPE_START].contains(&status_kind) {
			StatusModule::change_status_request_from_script(boma, *FIGHTER_WARIO_STATUS_KIND_SPECIAL_S_SEARCH, true);
		};
		if status_kind == *FIGHTER_WARIO_STATUS_KIND_SPECIAL_S_SEARCH {
			if frame >= 69.0 {
				if situation_kind == *SITUATION_KIND_AIR {
					StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_FALL, true);
				};
			};
			if frame >= 57.0 {
				if situation_kind == *SITUATION_KIND_GROUND {
					StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_WAIT, true);
				};
			};
			/*if MotionModule::frame(boma) < 38.0 {
				StatusModule::set_situation_kind(boma, smash::app::SituationKind(*SITUATION_KIND_AIR), true);
				StatusModule::set_keep_situation_air(boma, true);
			} else if is_near_ground == 1{
				StatusModule::set_keep_situation_air(boma, false);
				StatusModule::set_situation_kind(boma, smash::app::SituationKind(*SITUATION_KIND_GROUND), true);
			};*/
			if situation_kind == *SITUATION_KIND_GROUND && motion_kind != hash40("special_s_search"){
				MotionModule::change_motion_inherit_frame(boma, smash::phx::Hash40::new("special_s_search"), 0.0, 1.0, 0.0, false, false);
			};
			if situation_kind == *SITUATION_KIND_AIR && motion_kind != hash40("special_s_search") && frame < 2.0{
				MotionModule::change_motion_inherit_frame(boma, smash::phx::Hash40::new("special_s_search"), 0.0, 1.0, 0.0, false, false);
			};
			/*if situation_kind == *SITUATION_KIND_AIR {
				StatusModule::set_keep_situation_air(boma, true);
			};*/
			if situation_kind == *SITUATION_KIND_AIR && motion_kind != hash40("special_s_search") && frame >= 2.0{
				StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_FALL, true);
			};
			/*if StatusModule::is_situation_changed(boma) {
				StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_FALL, true);
			};*/
			if frame < 7.0 && frame > 5.0 {
				if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_SPECIAL) && ADD_DAMAGE[ENTRY_ID] <= 40{
					ADD_DAMAGE[ENTRY_ID] += 1;
					MotionModule::set_rate(boma, 0.0225);
				} else {
					MotionModule::set_rate(boma, 1.0);
				};
			} else {
				MotionModule::set_rate(boma, 1.0);
				if frame > 20.0 {
					ADD_DAMAGE[ENTRY_ID] = 0;
				};
			};
			if frame < 2.0 {
					macros::SET_SPEED_EX(fighter, 0.0, -0.5, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
			};
			if frame < 38.0{ 
					if !AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_ALL) { 
						if KineticModule::get_kinetic_type(boma) != *FIGHTER_KINETIC_TYPE_MOTION_AIR{
							KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_MOTION_AIR);
						};
					} else {
						if KineticModule::get_kinetic_type(boma) != *FIGHTER_KINETIC_TYPE_FALL{
							KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_FALL);
						};
					};
					if frame >= 9.0 && frame < 37.0{
						macros::SET_SPEED_EX(fighter, 1.5 + (ADD_DAMAGE[ENTRY_ID] as f32/40.0)*1.5, 0, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
					};
					if frame == 37.0{
						macros::SET_SPEED_EX(fighter, 0.1, -0.2, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
					};
			} else {
					if KineticModule::get_kinetic_type(boma) != *FIGHTER_KINETIC_TYPE_FALL {
						KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_FALL);
					};
			};
		} else {
			ADD_DAMAGE[ENTRY_ID] = 0;
		};*/
    }
}
#[weapon_frame( agent = WEAPON_KIND_WARIO_WARIOBIKE )]
fn bike_frame(weapon: &mut L2CFighterBase) {
    unsafe {
        let otarget_id = WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER) as u32;
        let boma = smash::app::sv_battle_object::module_accessor(otarget_id);
		let ENTRY_ID = WorkModule::get_int(&mut *boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
		let status_kind = StatusModule::status_kind(weapon.module_accessor);
        if smash::app::utility::get_kind(&mut *boma) == *FIGHTER_KIND_WARIO {
			if !smash::app::sv_information::is_ready_go() {
				ModelModule::set_scale(weapon.module_accessor, 1.0);
			} else{
				ModelModule::set_scale(weapon.module_accessor, 0.0000001);
			};
		};
    }
}
pub fn install() {
    smashline::install_acmd_scripts!(
		wario_dsmash,
		wario_fair,
		wario_utilt,
		wario_pummel,
		wario_da,
		wario_da_eff,
		wario_dsmash_eff,
		wario_dsmash_snd,
		wario_sideb,
		wario_sideb_eff,
		wario_sideb_snd,
		wario_sideb_start_snd
    );
    smashline::install_agent_frames!(
        wario_frame,
		bike_frame
    );
}
