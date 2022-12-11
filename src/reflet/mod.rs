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



//Grima Install
static mut IS_GRIMA : [bool; 8] = [false; 8];
static mut DMG_COUNTER : [i32; 8] = [0; 8];
static mut DMG_COUNTER_MAX : i32 = 60;
static mut DMG_ADD : f32 = 1.5;
static mut SPECIAL_HI_CURR : [i32; 8] = [0; 8]; 
static mut SPECIAL_N_CURR : [i32; 8] = [0; 8]; 
static mut SPECIAL_S_CURR : [f32; 8] = [0.0; 8]; 

//Float Stuff
static mut FLOAT : [i32; 8] = [0; 8]; //Logs Float Time
static mut START_FLOAT : [bool; 8] = [false; 8];
static mut JUMPSQUAT_FLOAT : [bool; 8] = [false; 8];
static mut CHECK_FLOAT : [i32; 8] = [0; 8];
static mut CHECK_FLOAT_MAX : i32 = 15; //Frames where jump needs to be held to start floating
static mut X : [f32; 8] = [0.0; 8]; //Logs speed
static mut Y : [f32; 8] = [0.0; 8]; //Logs speed
static mut FLOAT_MAX : i32 = 95; //Frames this bitch can float (In frames, 60 frames = 1 second)
static mut X_MAX : f32 = 1.208; //Max Horizontal movespeed
static mut X_ACCEL_ADD : f32 = 0.02; //Air Accel Add
static mut X_ACCEL_MUL : f32 = 0.09; //Air Accel Mul
static mut Y_MAX : f32 = 0.0; //Max Vertical movespeed
static mut Y_ACCEL_ADD : f32 = 0.06;
static mut Y_ACCEL_MUL : f32 = 0.06;

static mut HANDS :  smash::phx::Vector3f =  smash::phx::Vector3f { x: 0.0, y: 0.0, z: 0.0 };

#[acmd_script(
    agent = "reflet_elwind",
    script =  "game_shoot0",
    category = ACMD_GAME,
	low_priority)]
unsafe fn robin_elwind(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		if(is_excute){
			ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=7.0, Angle=255, KBG=100, FKB=0, BKB=10, Size=4.0, X=0.0, Y=0.0, Z=0.5, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=true, Absorbable=true, Flinchless=false, DisableHitlag=false, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_A, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_MAGIC)
			ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=5.0, Angle=80, KBG=30, FKB=0, BKB=100, Size=6.0, X=0.0, Y=2.0, Z=0.5, X2=0.0, Y2=-3.5, Z2=0.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=true, Absorbable=true, Flinchless=false, DisableHitlag=false, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_MAGIC)
		}
		wait(Frames=5)
		if(is_excute){
			ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=5.0, Angle=80, KBG=30, FKB=0, BKB=100, Size=7.0, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=true, Absorbable=true, Flinchless=false, DisableHitlag=false, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_MAGIC)
		}			
    });
}	

#[acmd_script(
    agent = "reflet",
    script =  "game_attacklw3",
    category = ACMD_GAME,
	low_priority)]
unsafe fn robin_dtilt(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		frame(Frame=7)
		if(is_excute){
			ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=6.0, Angle=75, KBG=35, FKB=0, BKB=70, Size=3.5, X=0.0, Y=3.0, Z=11.0, X2=0.0, Y2=5.0, Z2=6.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
		}
		frame(Frame=9)
		if(is_excute){
			AttackModule::clear_all()
		}
    });
}		
#[acmd_script(
    agent = "reflet",
    script =  "game_attacks3",
    category = ACMD_GAME,
	low_priority)]
unsafe fn robin_ftilt(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		frame(Frame=9)
		if(is_excute){
			ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=7.5, Angle=361, KBG=70, FKB=0, BKB=85, Size=5.0, X=0.0, Y=8.5, Z=14.0, X2=0.0, Y2=8.5, Z2=8.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
		}
		frame(Frame=11)
		if(is_excute){
			AttackModule::clear_all()
		}
    });
}		
#[acmd_script(
    agent = "reflet",
    scripts =  ["effect_speciallwstart", "effect_specialairlwstart"],
    category = ACMD_EFFECT,
	low_priority)]
unsafe fn robin_downb_eff(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
}		
#[acmd_script(
    agent = "reflet",
    scripts =  ["effect_speciallwend", "effect_specialairlwend"],
    category = ACMD_EFFECT,
	low_priority)]
unsafe fn robin_downb_end_eff(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
}	
#[acmd_script(
    agent = "reflet",
    scripts =  ["sound_speciallwstart", "sound_specialairlwstart"],
    category = ACMD_SOUND,
	low_priority)]
unsafe fn robin_downb_snd(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
}		
#[acmd_script(
    agent = "reflet",
    scripts =  ["game_speciallwstart", "game_specialairlwstart"],
    category = ACMD_GAME,
	low_priority)]
unsafe fn robin_downb(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		FT_MOTION_RATE(FSM=2)
		frame(Frame=7)
		FT_MOTION_RATE(FSM=1)
		frame(Frame=14)
		FT_MOTION_RATE(FSM=1.5)
    });
}	
#[acmd_script(
    agent = "reflet",
    script =  "game_specialhi3",
    category = ACMD_GAME,
	low_priority)]
unsafe fn robin_grima_upb(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		frame(Frame=4)
		if(is_excute){
			WorkModule::on_flag(Flag=FIGHTER_REFLET_STATUS_SPECIAL_HI_FLAG_JUMP)
		}
		frame(Frame=7)
		if(is_excute){
			rust {
				if StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_AIR {
					let speed = smash::phx::Vector3f { x: 0.0, y: 0.8, z: 0.0 };
					KineticModule::add_speed(fighter.module_accessor, &speed);
				} else {
					let speed = smash::phx::Vector3f { x: 0.0, y: 0.5, z: 0.0 };
					KineticModule::add_speed(fighter.module_accessor, &speed);
				};
			}
			ATTACK(ID=0, Part=0, Bone=hash40("hip"), Damage=2.0, Angle=367, KBG=100, FKB=120, BKB=0, Size=10.5, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.75, SDI=0.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=4, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_purple"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_FIRE, Type=ATTACK_REGION_MAGIC)
		}
		frame(Frame=8)
		if(is_excute){
			AttackModule::clear_all()
			ATTACK(ID=0, Part=0, Bone=hash40("hip"), Damage=2.5, Angle=367, KBG=100, FKB=90, BKB=0, Size=6.5, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.75, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=4, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_purple"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_FIRE, Type=ATTACK_REGION_MAGIC)
			ATTACK(ID=1, Part=0, Bone=hash40("head"), Damage=2.5, Angle=367, KBG=100, FKB=90, BKB=0, Size=6.5, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.75, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=4, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_purple"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_FIRE, Type=ATTACK_REGION_MAGIC)
			ATTACK(ID=2, Part=0, Bone=hash40("kneer"), Damage=2.5, Angle=367, KBG=100, FKB=90, BKB=0, Size=3.25, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.75, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=4, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_purple"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_FIRE, Type=ATTACK_REGION_MAGIC)
			ATTACK(ID=3, Part=0, Bone=hash40("kneel"), Damage=2.5, Angle=367, KBG=100, FKB=90, BKB=0, Size=3.25, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.75, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=4, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_purple"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_FIRE, Type=ATTACK_REGION_MAGIC)
		}
		frame(Frame=12)
		if(is_excute){
			sv_battle_object::notify_event_msc_cmd(0x2127e37c07u64, GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES)
		}
		frame(Frame=27)
		if(is_excute){	
			AttackModule::clear_all()
			ATTACK(ID=0, Part=0, Bone=hash40("hip"), Damage=6.0, Angle=70, KBG=78, FKB=0, BKB=75, Size=8.5, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.2, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_purple"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_FIRE, Type=ATTACK_REGION_MAGIC)
			ATTACK(ID=1, Part=0, Bone=hash40("head"), Damage=6.0, Angle=70, KBG=78, FKB=0, BKB=75, Size=8.5, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.2, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_purple"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_FIRE, Type=ATTACK_REGION_MAGIC)
			ATTACK(ID=2, Part=0, Bone=hash40("kneer"), Damage=6.0, Angle=70, KBG=78, FKB=0, BKB=75, Size=4.25, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.2, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_purple"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_FIRE, Type=ATTACK_REGION_MAGIC)
			ATTACK(ID=3, Part=0, Bone=hash40("kneel"), Damage=6.0, Angle=70, KBG=78, FKB=0, BKB=75, Size=4.25, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.2, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_purple"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_FIRE, Type=ATTACK_REGION_MAGIC)
		}
		frame(Frame=35)
		if(is_excute){
			AttackModule::clear_all()
		}
    });
}	
#[acmd_script(
    agent = "reflet",
    script =  "effect_specialhi3",
    category = ACMD_EFFECT,
	low_priority)]
unsafe fn robin_grima_upb_eff(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    /*acmd!(lua_state, {

    });*/
}	
#[acmd_script(
    agent = "reflet",
    script =  "sound_specialhi3",
    category = ACMD_SOUND,
	low_priority)]
unsafe fn robin_grima_upb_snd(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		if(is_excute){
			PLAY_SE(hash40("se_common_stage_suddendeath_out"))
			PLAY_SE(hash40("se_common_stage_suddendeath_out"))
		}
		frame(Frame=5)
		if(is_excute){
			PLAY_SEQUENCE(hash40("seq_reflet_rnd_special_h"))
		}
    });
}
#[acmd_script(
    agent = "reflet",
    scripts =  ["game_specialnshoot", "game_specialairnshoot"],
    category = ACMD_GAME,
	low_priority)]
unsafe fn robin_grima_neutralb(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
	let ENTRY_ID = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
	if IS_GRIMA[ENTRY_ID] {
			frame(fighter.lua_state_agent, 1.0);
			macros::FT_MOTION_RATE(fighter, /*FSM*/ 5.0);
			wait(fighter.lua_state_agent, 1.0);
			macros::FT_MOTION_RATE(fighter, /*FSM*/ 1.0);
			if macros::is_excute(fighter) {
				damage!(fighter, /*MSC=*/*MA_MSC_DAMAGE_DAMAGE_NO_REACTION, /*Type*/ *DAMAGE_NO_REACTION_MODE_DAMAGE_POWER, /*DamageThreshold=*/11);
			};
			frame(fighter.lua_state_agent, 11.0);
			if macros::is_excute(fighter) {
				macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 1, /*Bone*/ Hash40::new("top"), /*Damage*/ 18.0, /*Angle*/ 361, /*KBG*/ 73, /*FKB*/ 0, /*BKB*/ 65, /*Size*/ 10.5, /*X*/ 0.0, /*Y*/ 12.0, /*Z*/ 19.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.6, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ true, /*Absorbable*/ true, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect=*/Hash40::new("collision_attr_purple"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_FIRE, /*Type*/ *ATTACK_REGION_MAGIC);
			};
			frame(fighter.lua_state_agent, 18.0);
			if macros::is_excute(fighter) {
				damage!(fighter, /*MSC=*/*MA_MSC_DAMAGE_DAMAGE_NO_REACTION, /*Type*/ *DAMAGE_NO_REACTION_MODE_NORMAL, /*DamageThreshold=*/0);
				AttackModule::clear_all(fighter.module_accessor);
			};
	} else {
			frame(fighter.lua_state_agent, 1.0);
			macros::FT_MOTION_RATE(fighter, /*FSM*/ 0.7);
			frame(fighter.lua_state_agent, 11.0);
			if macros::is_excute(fighter) {
				WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_REFLET_STATUS_SPECIAL_N_SHOOT_FLAG_TRY);
			};
			macros::FT_MOTION_RATE(fighter, /*FSM*/ 1);
	};
}	
#[acmd_script(
    agent = "reflet",
    scripts =  ["effect_specialnshoot", "effect_specialairnshoot"],
    category = ACMD_EFFECT,
	low_priority)]
unsafe fn robin_grima_neutralb_eff(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
	let ENTRY_ID = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
	if IS_GRIMA[ENTRY_ID] {
			frame(fighter.lua_state_agent, 6.0);
			if macros::is_excute(fighter) {
				macros::EFFECT_FOLLOW_NO_STOP(fighter, Hash40::new("sys_flash"), Hash40::new("havel"), -1, 1, 0, 0, 0, 0, 0.45, true);
			};
			frame(fighter.lua_state_agent, 11.0);
			if macros::is_excute(fighter) {
				macros::EFFECT(fighter, Hash40::new_raw(0x11c5fdc327), Hash40::new("top"), -0.0, 12.5, 19, 0, 0, 0, 1.5, 0, 0, 0, 0, 0, 0, true);
				macros::LAST_EFFECT_SET_COLOR(fighter, 0.125, 0.0, 3.0);
				macros::QUAKE(fighter, *CAMERA_QUAKE_KIND_M);
			};
	} else {
			frame(fighter.lua_state_agent, 6.0);
			if macros::is_excute(fighter) {
				macros::EFFECT_FOLLOW_NO_STOP(fighter, Hash40::new("sys_flash"), Hash40::new("havel"), -1, 1, 0, 0, 0, 0, 0.45, true);
			};
	};
}	
#[acmd_script(
    agent = "reflet",
    scripts =  ["sound_specialnshoot", "sound_specialairnshoot"],
    category = ACMD_SOUND,
	low_priority)]
unsafe fn robin_grima_neutralb_snd(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
	let ENTRY_ID = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
	if IS_GRIMA[ENTRY_ID] {
			frame(fighter.lua_state_agent, 10.0);
			if macros::is_excute(fighter) {
				macros::PLAY_SE(fighter, Hash40::new("vc_reflet_attack07"));
				macros::PLAY_SE(fighter, Hash40::new("se_reflet_smash_l01"));
				macros::PLAY_SE(fighter, Hash40::new("se_common_bomb_ll"));
			};
	} else if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_REFLET_STATUS_SPECIAL_S_FLAG_SHOOT_OK) {
		if macros::is_excute(fighter) {
			macros::PLAY_SE(fighter, Hash40::new("se_reflet_mp_empty"));
			macros::PLAY_SEQUENCE(fighter, Hash40::new("seq_reflet_rnd_special_empty"));
		};
	};
}	
#[acmd_script(
    agent = "reflet",
    scripts =  ["game_specials", "game_specialairs"],
    category = ACMD_GAME,
	low_priority)]
unsafe fn robin_grima_sideb(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
	let ENTRY_ID = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
	if IS_GRIMA[ENTRY_ID] {
			macros::FT_MOTION_RATE(fighter, /*FSM*/ 0.7);
			wait(fighter.lua_state_agent, 20.0);
			macros::FT_MOTION_RATE(fighter, /*FSM*/ 1.0);
			if macros::is_excute(fighter) {	
				macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 0.8, /*Angle*/ 367, /*KBG*/ 100, /*FKB*/ 80, /*BKB*/ 0, /*Size*/ 6.0, /*X*/ 0.0, /*Y*/ 10.0, /*Z*/ 14.0, /*X2*/ Some(0.0), /*Y2*/ Some(10.0), /*Z2*/ Some(11.2), /*Hitlag*/ 0.4, /*SDI*/ 0.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 3, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_purple"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_FIRE, /*Type*/ *ATTACK_REGION_PUNCH);
			};
			wait(fighter.lua_state_agent, 20.0);
			if macros::is_excute(fighter) {	
				AttackModule::clear_all(fighter.module_accessor);
				macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 5.0, /*Angle*/ 90, /*KBG*/ 130, /*FKB*/ 0, /*BKB*/ 50, /*Size*/ 6.0, /*X*/ 0.0, /*Y*/ 10.0, /*Z*/ 14.0, /*X2*/ Some(0.0), /*Y2*/ Some(10.0), /*Z2*/ Some(11.2), /*Hitlag*/ 1.3, /*SDI*/ 0.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_purple"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_FIRE, /*Type*/ *ATTACK_REGION_PUNCH);
			};
			wait(fighter.lua_state_agent, 2.0);
			if macros::is_excute(fighter) {	
				AttackModule::clear_all(fighter.module_accessor);
			};
	} else {
		frame(fighter.lua_state_agent, 17.0);
		if macros::is_excute(fighter) {
			WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_REFLET_STATUS_SPECIAL_S_FLAG_TRY);
		};
	};
}	
#[acmd_script(
    agent = "reflet",
    scripts =  ["effect_specials", "effect_specialairs"],
    category = ACMD_EFFECT,
	low_priority)]
unsafe fn robin_grima_sideb_eff(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
	let ENTRY_ID = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
	if IS_GRIMA[ENTRY_ID] {
		if macros::is_excute(fighter) {
			macros::EFFECT_OFF_KIND(fighter, Hash40::new_raw(0x0dec313736), false, true);
			macros::EFFECT_FOLLOW_NO_STOP(fighter, Hash40::new_raw(0x15f3d27868), Hash40::new("top"), 0, 5, 15, 0, 0, 0, 1, true);
		};
	} else {
			frame(fighter.lua_state_agent, 10.0);
			if macros::is_excute(fighter) {
				macros::EFFECT_FOLLOW(fighter, Hash40::new_raw(0x14285d8a3e), Hash40::new("top"), -1, 22, 1.5, 0, 0, 0, 0.8, true);
			};
			frame(fighter.lua_state_agent, 15.0);
			if macros::is_excute(fighter) {
				macros::EFFECT_FOLLOW(fighter, Hash40::new_raw(0x1410f5c5b6), Hash40::new("handr"), 1, 1, 0, 0, 0, 0, 1, true);
				EffectModule::enable_sync_init_pos_last(fighter.module_accessor);
			};
			frame(fighter.lua_state_agent, 16.0);
			if macros::is_excute(fighter) {
				macros::EFFECT_FOLLOW(fighter, Hash40::new_raw(0x1410f5c5b6), Hash40::new("handl"), 1, 1, 0, 0, 0, 0, 1, true);
				EffectModule::enable_sync_init_pos_last(fighter.module_accessor);
			};
	};
}	
#[fighter_frame_callback]
pub fn robin(fighter : &mut L2CFighterCommon) {
    unsafe {
        let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);  
		let fighter_kind = smash::app::utility::get_kind(boma);
		let status_kind = smash::app::lua_bind::StatusModule::status_kind(boma);
		let situation_kind = StatusModule::situation_kind(boma);
		let cancel_frame = FighterMotionModuleImpl::get_cancel_frame(boma,smash::phx::Hash40::new_raw(MotionModule::motion_kind(boma)),false) as f32;
		let stick_x = ControlModule::get_stick_x(boma) * PostureModule::lr(boma);
		let stick_y = ControlModule::get_stick_y(boma);
		let speed_x = KineticModule::get_sum_speed_x(boma, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
		let speed_y = KineticModule::get_sum_speed_y(boma, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
		let frame = MotionModule::frame(boma);
		let end_frame = MotionModule::end_frame(boma);
		let lua_state = fighter.lua_state_agent;
		let ENTRY_ID = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
		let motion_kind = MotionModule::motion_kind(boma);
		if fighter_kind == *FIGHTER_KIND_REFLET {
			WorkModule::set_int(boma, 10, *FIGHTER_REFLET_INSTANCE_WORK_ID_INT_SPECIAL_LW_CURRENT_POINT);
			if !IS_GRIMA[ENTRY_ID] {
				SPECIAL_HI_CURR[ENTRY_ID] = WorkModule::get_int(boma, *FIGHTER_REFLET_INSTANCE_WORK_ID_INT_SPECIAL_HI_CURRENT_POINT);
				SPECIAL_N_CURR[ENTRY_ID] = WorkModule::get_int(boma, *FIGHTER_REFLET_INSTANCE_WORK_ID_INT_SPECIAL_N_CURRENT_POINT);
				SPECIAL_S_CURR[ENTRY_ID] = WorkModule::get_float(boma, *FIGHTER_REFLET_INSTANCE_WORK_ID_FLOAT_SPECIAL_S_CURRENT_POINT);
			} else {
				if SPECIAL_HI_CURR[ENTRY_ID] > 2 {
					WorkModule::set_int(boma, SPECIAL_HI_CURR[ENTRY_ID], *FIGHTER_REFLET_INSTANCE_WORK_ID_INT_SPECIAL_HI_CURRENT_POINT);
				} else {
					WorkModule::set_int(boma, 2, *FIGHTER_REFLET_INSTANCE_WORK_ID_INT_SPECIAL_HI_CURRENT_POINT);
				};
				WorkModule::set_int(boma, SPECIAL_N_CURR[ENTRY_ID], *FIGHTER_REFLET_INSTANCE_WORK_ID_INT_SPECIAL_N_CURRENT_POINT);
				WorkModule::set_float(boma, SPECIAL_S_CURR[ENTRY_ID], *FIGHTER_REFLET_INSTANCE_WORK_ID_FLOAT_SPECIAL_S_CURRENT_POINT);
			};
			if [hash40("special_hi"), hash40("special_air_hi"), hash40("special_hi_fail"), hash40("special_air_hi_fail")].contains(&motion_kind) {
				if IS_GRIMA[ENTRY_ID] == false {
					if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_GUARD) && !WorkModule::is_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_DISABLE_ESCAPE_AIR) && MotionModule::frame(boma) >= 8.0 {
						StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ESCAPE_AIR, false);
					};
				} else{
					MotionModule::change_motion(boma, smash::phx::Hash40::new("special_hi3"), 0.0, 1.0, false, 0.0, false, false);
				};
			};
			if IS_GRIMA[ENTRY_ID]{
				WorkModule::set_int(boma, 0, *FIGHTER_REFLET_INSTANCE_WORK_ID_INT_SPECIAL_N_THUNDER_KIND);
				if status_kind == *FIGHTER_REFLET_STATUS_KIND_SPECIAL_N_HOLD {
					StatusModule::change_status_request_from_script(boma, *FIGHTER_REFLET_STATUS_KIND_SPECIAL_N_SHOOT, true);
				};
			};
			/*if [hash40("special_n_start")].contains(&motion_kind) {
				if IS_GRIMA[ENTRY_ID]{
					MotionModule::change_motion(boma, smash::phx::Hash40::new("special_n"), -1.0, 1.0, false, 0.0, false, false);
				};
			};
			if [hash40("special_air_n_start")].contains(&motion_kind) {
				if IS_GRIMA[ENTRY_ID]  {
					MotionModule::change_motion(boma, smash::phx::Hash40::new("special_air_n"), -1.0, 1.0, false, 0.0, false, false);
				};
			};*/
			if [hash40("special_hi3")].contains(&motion_kind) {
				if frame as i32 % 5 == 0 {
					let a1: u32 = EffectModule::req_follow(boma, smash::phx::Hash40::new("sys_magicball_aura"), smash::phx::Hash40::new("haver"), &HANDS, &HANDS, 1.0, true, 0, 0, 0, 0, 0, true, true) as u32;
					let a2: u32 = EffectModule::req_follow(boma, smash::phx::Hash40::new("sys_magicball_aura"), smash::phx::Hash40::new("havel"), &HANDS, &HANDS, 1.0, true, 0, 0, 0, 0, 0, true, true) as u32;
					let a3: u32 = EffectModule::req_follow(boma, smash::phx::Hash40::new("sys_magicball_aura"), smash::phx::Hash40::new("footr"), &HANDS, &HANDS, 1.0, true, 0, 0, 0, 0, 0, true, true) as u32;
					let a4: u32 = EffectModule::req_follow(boma, smash::phx::Hash40::new("sys_magicball_aura"), smash::phx::Hash40::new("footl"), &HANDS, &HANDS, 1.0, true, 0, 0, 0, 0, 0, true, true) as u32;
					let a5: u32 = EffectModule::req_follow(boma, smash::phx::Hash40::new("sys_magicball_aura"), smash::phx::Hash40::new("hip"), &HANDS, &HANDS, 1.0, true, 0, 0, 0, 0, 0, true, true) as u32;
					EffectModule::set_rgb(boma, a1, 3.3, 0.4, 1.0);
					EffectModule::set_rgb(boma, a2, 3.3, 0.4, 1.0);
					EffectModule::set_rgb(boma, a3, 3.3, 0.4, 1.0);
					EffectModule::set_rgb(boma, a4, 3.3, 0.4, 1.0);
					EffectModule::set_rgb(boma, a5, 3.3, 0.4, 1.0);
				};
			};
			if [hash40("special_lw_start"), hash40("special_air_lw_start"), hash40("special_lw"), hash40("special_air_lw")].contains(&motion_kind) {
				if MotionModule::frame(boma) == 8.0 {
					if IS_GRIMA[ENTRY_ID] == true {
						IS_GRIMA[ENTRY_ID] = false;
						macros::EFFECT_FOLLOW(fighter, Hash40::new("reflet_gigafire_hold"), Hash40::new("waist"), 0, 0, 0, 0, 0, 0, 0.725, true);
						macros::LAST_EFFECT_SET_COLOR(fighter, 1.0, 1.15, 1.0);
						macros::LAST_EFFECT_SET_ALPHA(fighter, 0.75);
						macros::EFFECT_FOLLOW(fighter, Hash40::new("reflet_gigafire_hold"), Hash40::new("havel"), 0, 0, 0, 0, 0, 0, 0.2, true);
						macros::LAST_EFFECT_SET_COLOR(fighter, 1.0, 1.15, 1.0);
						macros::LAST_EFFECT_SET_ALPHA(fighter, 0.75);
						macros::EFFECT_FOLLOW(fighter, Hash40::new("reflet_gigafire_hold"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 0.2, true);
						macros::LAST_EFFECT_SET_COLOR(fighter, 1.0, 1.15, 1.0);
						macros::LAST_EFFECT_SET_ALPHA(fighter, 0.75);
					} else {
						IS_GRIMA[ENTRY_ID] = true;
						macros::FT_ADD_DAMAGE(fighter, DMG_ADD*5.0);
						macros::EFFECT_FOLLOW(fighter, Hash40::new("reflet_gigafire_hold"), Hash40::new("waist"), 0, 0, 0, 0, 0, 0, 0.725, true);
						macros::LAST_EFFECT_SET_COLOR(fighter, 0.25, 0.0, 3.0);
						macros::LAST_EFFECT_SET_ALPHA(fighter, 0.75);
						macros::EFFECT_FOLLOW(fighter, Hash40::new("reflet_gigafire_hold"), Hash40::new("havel"), 0, 0, 0, 0, 0, 0, 0.2, true);
						macros::LAST_EFFECT_SET_COLOR(fighter, 0.25, 0.0, 3.0);
						macros::LAST_EFFECT_SET_ALPHA(fighter, 0.75);
						macros::EFFECT_FOLLOW(fighter, Hash40::new("reflet_gigafire_hold"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 0.2, true);
						macros::LAST_EFFECT_SET_COLOR(fighter, 0.25, 0.0, 3.0);
						macros::LAST_EFFECT_SET_ALPHA(fighter, 0.75);
					};
					acmd!(lua_state, {PLAY_FLY_VOICE(hash40("seq_reflet_rnd_futtobi01"), hash40("seq_reflet_rnd_futtobi02"))});
					macros::PLAY_SE(fighter, Hash40::new("se_reflet_special_l01"));
					if StatusModule::is_situation_changed(boma) {
						StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_LANDING, true);
					};
				};
				if end_frame-frame < 5.0 && situation_kind != *SITUATION_KIND_AIR {
						StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_WAIT, false);
				};
				if  end_frame-frame < 3.0 && situation_kind == *SITUATION_KIND_AIR {
						StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_FALL, false);
				};
			};
			if status_kind == *FIGHTER_REFLET_STATUS_KIND_SPECIAL_LW_END && StatusModule::prev_status_kind(boma, 0) == *FIGHTER_STATUS_KIND_SPECIAL_LW {
					if situation_kind == *SITUATION_KIND_AIR {
						StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_FALL, true);
					} else {
						StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_WAIT, true);
					};
			};
			if StatusModule::situation_kind(boma) != SITUATION_KIND_AIR {
				FLOAT[ENTRY_ID] = 0;
				START_FLOAT[ENTRY_ID] = false;
				CHECK_FLOAT[ENTRY_ID] = 0;
			};
			if smash::app::sv_information::is_ready_go() == false || [*FIGHTER_STATUS_KIND_WIN, *FIGHTER_STATUS_KIND_LOSE, *FIGHTER_STATUS_KIND_DEAD].contains(&status_kind) {
				IS_GRIMA[ENTRY_ID] = false;
				DMG_COUNTER[ENTRY_ID] = DMG_COUNTER_MAX;
			};
			if IS_GRIMA[ENTRY_ID] == true {
				if DMG_COUNTER[ENTRY_ID] > 0 {
					DMG_COUNTER[ENTRY_ID] -= 1;
				} else {
					DMG_COUNTER[ENTRY_ID] = DMG_COUNTER_MAX;
					if DamageModule::damage(boma, 0) < 120.0 {
						macros::FT_ADD_DAMAGE(fighter, DMG_ADD);
					};
				};
				//Dash Speed
				if [*FIGHTER_STATUS_KIND_DASH].contains(&status_kind) {
					if MotionModule::frame(boma) == 3.0 {
							let speed = smash::phx::Vector3f { x: 0.2, y: 0.0, z: 0.0 };
							KineticModule::add_speed(boma, &speed);
					};
				};
				//Effect Code
				if DMG_COUNTER[ENTRY_ID] % 40 == 0 {
					macros::EFFECT_FOLLOW(fighter, Hash40::new("reflet_rizaia_capture"), Hash40::new("havel"), 0, 0, 0, 0, 0, 0, 0.25, true);
					macros::EFFECT_FOLLOW(fighter, Hash40::new("reflet_rizaia_capture"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 0.25, true);
					let a1: u32 = EffectModule::req_follow(boma, smash::phx::Hash40::new("sys_magicball_aura"), smash::phx::Hash40::new("waist"), &HANDS, &HANDS, 1.0, true, 0, 0, 0, 0, 0, true, true) as u32;
					EffectModule::set_rgb(boma, a1, 3.3, 0.4, 1.0);
					if FLOAT[ENTRY_ID] > 1 {
						macros::EFFECT_FOLLOW(fighter, Hash40::new("reflet_rizaia_capture"), Hash40::new("footl"), 0, 0, 0, 0, 0, 0, 0.2, true);
						macros::EFFECT_FOLLOW(fighter, Hash40::new("reflet_rizaia_capture"), Hash40::new("footr"), 0, 0, 0, 0, 0, 0, 0.2, true);
					};
				};
				//Float Code
				if FLOAT[ENTRY_ID] == 1{
					if KineticModule::get_kinetic_type(boma) == *FIGHTER_KINETIC_TYPE_MOTION_AIR && [*FIGHTER_STATUS_KIND_SPECIAL_LW, *FIGHTER_STATUS_KIND_SPECIAL_HI, *FIGHTER_STATUS_KIND_SPECIAL_S, *FIGHTER_GANON_STATUS_KIND_SPECIAL_AIR_S_CATCH, *FIGHTER_GANON_STATUS_KIND_SPECIAL_AIR_S_END].contains(&status_kind) == false {
						KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_FALL);
					};
				};
				if situation_kind == *SITUATION_KIND_AIR && (!(*FIGHTER_STATUS_KIND_DAMAGE..*FIGHTER_STATUS_KIND_DAMAGE_FALL).contains(&status_kind) && status_kind != *FIGHTER_STATUS_KIND_FALL_SPECIAL){
					if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_JUMP) {
						CHECK_FLOAT[ENTRY_ID] += 1;
					} else {
						CHECK_FLOAT[ENTRY_ID] = 0;
					};
					if (CHECK_FLOAT[ENTRY_ID] >= CHECK_FLOAT_MAX || JUMPSQUAT_FLOAT[ENTRY_ID]) && FLOAT[ENTRY_ID] == 0 {
						START_FLOAT[ENTRY_ID] = true;
					};
				};
				if status_kind == *FIGHTER_STATUS_KIND_JUMP && JUMPSQUAT_FLOAT[ENTRY_ID] {
					StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_FALL, true);
				};
				if status_kind == *FIGHTER_STATUS_KIND_JUMP_SQUAT {
					if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_JUMP) && ControlModule::get_stick_y(boma) < -0.5 {
						JUMPSQUAT_FLOAT[ENTRY_ID] = true;
					} else {
						JUMPSQUAT_FLOAT[ENTRY_ID] = false;
					};
				} else {
					JUMPSQUAT_FLOAT[ENTRY_ID] = false;
				};
				if [
					*FIGHTER_STATUS_KIND_ESCAPE_AIR, *FIGHTER_STATUS_KIND_ESCAPE_AIR_SLIDE, *FIGHTER_STATUS_KIND_SPECIAL_N, 
					*FIGHTER_STATUS_KIND_SPECIAL_S,*FIGHTER_STATUS_KIND_SPECIAL_HI, *FIGHTER_STATUS_KIND_SPECIAL_LW,
					*FIGHTER_REFLET_STATUS_KIND_SPECIAL_HI_2, *FIGHTER_REFLET_STATUS_KIND_SPECIAL_N_HOLD, *FIGHTER_REFLET_STATUS_KIND_SPECIAL_N_CANCEL,
					*FIGHTER_REFLET_STATUS_KIND_SPECIAL_N_TRON_END, *FIGHTER_REFLET_STATUS_KIND_SPECIAL_N_TRON_START, *FIGHTER_REFLET_STATUS_KIND_SPECIAL_N_JUMP_CANCEL
				].contains(&status_kind) && FLOAT[ENTRY_ID] > 1{
					FLOAT[ENTRY_ID] = 1;
				};
				if FLOAT[ENTRY_ID] > 1 {
					FLOAT[ENTRY_ID] -= 1;
					if KineticModule::get_kinetic_type(boma) != *FIGHTER_KINETIC_TYPE_MOTION_AIR {
						KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_MOTION_AIR);
					};
					if ControlModule::check_button_off(boma, *CONTROL_PAD_BUTTON_JUMP) {
						FLOAT[ENTRY_ID] = 1;
					};
					if ControlModule::check_button_on_trriger(boma, *CONTROL_PAD_BUTTON_SPECIAL) {
						FLOAT[ENTRY_ID] = 1;
					};
					let mut y_add = 0.0;
					let mut x_add = 0.0;
					if stick_x > 0.2 {
						x_add = ((stick_x-0.2)*X_ACCEL_MUL) + X_ACCEL_ADD;
						if speed_x > X_MAX || speed_x < -X_MAX{{}
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
					X[ENTRY_ID] += x_add;
					Y[ENTRY_ID] += y_add;
					macros::SET_SPEED_EX(fighter, X[ENTRY_ID], Y[ENTRY_ID], *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
				} else {
					X[ENTRY_ID] = 0.0;
					Y[ENTRY_ID] = 0.0;
				};
				if status_kind == *FIGHTER_STATUS_KIND_THROW {
					if FighterMotionModuleImpl::get_cancel_frame(boma,smash::phx::Hash40::new_raw(MotionModule::motion_kind(boma)), false)as f32 > 6.0 {
						if MotionModule::frame(boma) >= (FighterMotionModuleImpl::get_cancel_frame(boma,smash::phx::Hash40::new_raw(MotionModule::motion_kind(boma)), false)as f32 - 6.0) {
							CancelModule::enable_cancel(boma);
						};
					};
				};
				if START_FLOAT[ENTRY_ID] == true {
					FLOAT[ENTRY_ID] = FLOAT_MAX;
					START_FLOAT[ENTRY_ID] = false;
					/*if status_kind == *FIGHTER_STATUS_KIND_JUMP {
						StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_FALL, true);
					};
					if status_kind == *FIGHTER_STATUS_KIND_JUMP_AERIAL {
						StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_FALL, true);
					};*/
				};
				/*if [*FIGHTER_REFLET_STATUS_KIND_SPECIAL_N_SHOOT, *FIGHTER_REFLET_STATUS_KIND_SPECIAL_N_HOLD, *FIGHTER_REFLET_STATUS_KIND_SPECIAL_N_TRON_START, *FIGHTER_REFLET_STATUS_KIND_SPECIAL_N_TRON_HOLD, *FIGHTER_REFLET_STATUS_KIND_SPECIAL_N_TRON_END, *FIGHTER_REFLET_STATUS_KIND_SPECIAL_N_JUMP_CANCEL, *FIGHTER_REFLET_STATUS_KIND_SPECIAL_HI_2, *FIGHTER_REFLET_STATUS_KIND_SPECIAL_HI_FAIL, *FIGHTER_STATUS_KIND_SPECIAL_S].contains(&status_kind) {
					MotionModule::set_rate(boma, 0.75);
				};*/
			};
		};
    };
}
		
pub fn install() {
    smashline::install_acmd_scripts!(
		robin_dtilt,
		robin_ftilt,
		//
		robin_downb_eff,
		robin_downb_snd,
		robin_downb,
		//
		robin_elwind,
		//
		robin_grima_upb,
		robin_grima_upb_eff,
		robin_grima_upb_snd,
		//
		robin_grima_neutralb,
		robin_grima_neutralb_eff,
		robin_grima_neutralb_snd,
		//
		robin_grima_sideb,
		robin_grima_sideb_eff
    );
	smashline::install_agent_frame_callbacks!(robin);
}
