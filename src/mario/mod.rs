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

static mut SIDEB : [bool; 8] = [false; 8];
static mut SPIN : [bool; 8] = [false; 8];
static mut SPIN_EFF : [i32; 8] = [0; 8];
static mut SPIN1 :  smash::phx::Vector3f =  smash::phx::Vector3f { x: 0.0, y: 8.25, z: 0.0 };
static mut SPIN2 :  smash::phx::Vector3f =  smash::phx::Vector3f { x: 0.0, y: 8.2, z: 0.0 };
static mut SPIN3 :  smash::phx::Vector3f =  smash::phx::Vector3f { x: 0.0, y: 8.15, z: 0.0 };
static mut SPIN4 :  smash::phx::Vector3f =  smash::phx::Vector3f { x: 0.0, y: 8.3, z: 0.0 };
static mut SPIN5 :  smash::phx::Vector3f =  smash::phx::Vector3f { x: 0.0, y: 8.1, z: 0.0 };
static mut STAR1 :  smash::phx::Vector3f =  smash::phx::Vector3f { x: 0.0, y: 8.25, z: 0.0 };
static mut STAR2 :  smash::phx::Vector3f =  smash::phx::Vector3f { x: 0.0, y: 8.25, z: 9.0 };
static mut STAR3 :  smash::phx::Vector3f =  smash::phx::Vector3f { x: 0.0, y: 8.25, z: -9.0 };
static mut STAR4 :  smash::phx::Vector3f =  smash::phx::Vector3f { x: 0.0, y: 8.25, z: 4.5 };
static mut STAR5 :  smash::phx::Vector3f =  smash::phx::Vector3f { x: 0.0, y: 8.25, z: -4.5 };
static mut NOSPIN :  smash::phx::Vector3f =  smash::phx::Vector3f { x: 0.0, y: 0.0, z: 0.0 };

#[acmd_script(
    agent = "mario",
    script =  "game_attacklw3",
    category = ACMD_GAME)]
unsafe fn mario_dtilt(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		frame(Frame=5)
		if(is_excute){
			ATTACK(ID=0, Part=0, Bone=hash40("kneel"), Damage=7.0, Angle=90, KBG=80, FKB=0, BKB=50, Size=3.2, X=-1.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.4, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
			ATTACK(ID=1, Part=0, Bone=hash40("toel"), Damage=5.0, Angle=90, KBG=80, FKB=0, BKB=50, Size=4.2, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.4, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
			AttackModule::set_add_reaction_frame(ID=0, Frames=5.0, Unk=false)
			AttackModule::set_add_reaction_frame(ID=1, Frames=5.0, Unk=false)
		}
		wait(Frames=3)
		if(is_excute){
			AttackModule::clear_all()
		}
    });
}
#[acmd_script(
    agent = "mario",
    script =  "game_attackairhi",
    category = ACMD_GAME)]
unsafe fn mario_uair(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		frame(Frame=2)
		if(is_excute){
			WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
		}
		frame(Frame=5)
		if(is_excute){
			ATTACK(ID=0, Part=0, Bone=hash40("legr"), Damage=7.0, Angle=75, KBG=140, FKB=0, BKB=12, Size=4.4, X=1.2, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
			ATTACK(ID=1, Part=0, Bone=hash40("kneer"), Damage=7.0, Angle=75, KBG=140, FKB=0, BKB=12, Size=5.5, X=3.8, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
		}
		frame(Frame=9)
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
    agent = "mario",
    script =  "game_specials",
    category = ACMD_GAME)]
unsafe fn mario_ground_sideb(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		frame(Frame=9)
		if(is_excute){
			JostleModule::set_status(false)
			ATTACK(ID=0, Part=0, Bone=hash40("hip"), Damage=2.0, Angle=53, KBG=25, FKB=0, BKB=40, Size=5.0, X=0.0, Y=8.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.75, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=true, ShieldDamage=-2, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_KAMEHIT, Type=ATTACK_REGION_BODY)
		}
		frame(Frame=53)
		FT_MOTION_RATE(FSM=0.8)
		if(is_excute){
			JostleModule::set_status(true)
			AttackModule::clear_all()
		}
		frame(Frame=70) 
		if(is_excute){
			CancelModule::enable_cancel()
		}
    });
}			
#[acmd_script(
    agent = "mario",
    script =  "effect_specialairs",
    category = ACMD_EFFECT)]
unsafe fn mario_air_sideb_eff(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		frame(Frame=12)
		for(5 Iterations){
			if(is_excute){
				EFFECT_FOLLOW_FLIP(hash40("sys_spin_wind"), hash40("sys_spin_wind"), hash40("hat"), 0, 0.0, 0, 0, 0, 0, 0.75, true, EF_FLIP_YZ)
				LAST_EFFECT_SET_COLOR(1.25, 1.0875, 0.3375)
				LAST_EFFECT_SET_RATE(2.2)
				EffectModule::set_alpha_last(0.5)
				EFFECT_FOLLOW_FLIP(hash40("sys_spin_wind"), hash40("sys_spin_wind"), hash40("hat"), 0, 0.0, 0, 0, 0, 0, 0.4, true, EF_FLIP_YZ)
				LAST_EFFECT_SET_COLOR(1.25, 1.0875, 0.3375)
				LAST_EFFECT_SET_RATE(2.2)
			}
			wait(frames=4)
		}
	});
}
#[acmd_script(
    agent = "mario",
    script =  "effect_specials",
    category = ACMD_EFFECT)]
unsafe fn mario_ground_sideb_eff(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		frame(Frame=10)
		for(7 Iterations){
			if(is_excute){
				FOOT_EFFECT(hash40("sys_dash_smoke"), hash40("top"), -3, 0, 0, 0, 0, 0, 0.75, 0, 0, 0, 0, 0, 0, false)
				EFFECT_FOLLOW_ALPHA(hash40("sys_spin_wind"), hash40("rot"), 0, 0, 0, 0, 0, 270, 0.6, true, 0.6)
				LAST_EFFECT_SET_RATE(2)
			}
			wait(frames=6)
		}
	});
}		
#[acmd_script(
    agent = "mario",
    script =  "sound_specials",
    category = ACMD_SOUND)]
unsafe fn mario_ground_sideb_snd(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		frame(Frame=6)
		if(is_excute){
			PLAY_SEQUENCE(hash40("seq_mario_rnd_attack"))
		}
		frame(Frame=10)
		for(4 Iterations){
			if(is_excute){
				PLAY_SE(hash40("se_mario_dash_start"))
			}
			wait(frames=6)
			if(is_excute){
				PLAY_SE(hash40("se_mario_dash_start"))
				PLAY_SE(hash40("se_mario_attackdash"))
			}
			wait(frames=6)
		}
	});
}		
#[acmd_script(
    agent = "mario",
    script =  "sound_specialairs",
    category = ACMD_SOUND)]
unsafe fn mario_air_sideb_snd(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		frame(Frame=10)
		if(is_excute){
			PLAY_SE(hash40("se_mario_appeal_s01"))
			PLAY_SEQUENCE(hash40("seq_mario_rnd_attack"))
		}
		wait(Frames=3)
		for(4 Iterations){
			if(is_excute){
				PLAY_SE(hash40("se_mario_appeal_s02"))
			}
			wait(Frames=3)
			if(is_excute){
				PLAY_SE(hash40("se_mario_appeal_s03"))
			}
			wait(Frames=3)
			if(is_excute){
				PLAY_SE(hash40("se_mario_appeal_s04"))
			}
			wait(Frames=3)
		}
		frame(Frame=51)
		if(is_excute){
			PLAY_SE(hash40("se_mario_appeal_s06"))
		}
	});
}	
#[acmd_script(
    agent = "mario",
    script =  "game_specialairs",
    category = ACMD_GAME)]
unsafe fn mario_air_sideb(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		FT_MOTION_RATE(FSM=2.5)
		frame(Frame=4)
		FT_MOTION_RATE(FSM=0.8)
		frame(Frame=9)
		FT_MOTION_RATE(FSM=1)
		if(is_excute){
			ATTACK(ID=0, Part=0, Bone=hash40("hat"), Damage=8.0, Angle=65, KBG=100, FKB=150, BKB=0, Size=13.0, X=0.0, Y=8.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.75, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=true, ShieldDamage=-7, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_KAMEHIT, Type=ATTACK_REGION_BODY)
			AttackModule::set_add_reaction_frame(ID=0, Frames=9.0, Unk=false)
			ArticleModule::generate_article(FIGHTER_MARIO_GENERATE_ARTICLE_CAPPY, false, 0)
		}
		frame(Frame=10)
		FT_MOTION_RATE(FSM=0.75)
		if(is_excute){
			ATTACK(ID=0, Part=0, Bone=hash40("hat"), Damage=8.0, Angle=70, KBG=100, FKB=150, BKB=0, Size=9.0, X=0.0, Y=8.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.75, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=true, ShieldDamage=-7, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_KAMEHIT, Type=ATTACK_REGION_BODY)
			AttackModule::set_add_reaction_frame(ID=0, Frames=5.0, Unk=false)
			ArticleModule::generate_article(FIGHTER_MARIO_GENERATE_ARTICLE_CAPPY, false, 0)
		}
		frame(Frame=19)
		if(is_excute){
			ATTACK(ID=0, Part=0, Bone=hash40("hat"), Damage=8.0, Angle=75, KBG=100, FKB=150, BKB=0, Size=5.5, X=0.0, Y=8.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.75, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=true, ShieldDamage=-7, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_KAMEHIT, Type=ATTACK_REGION_BODY)
			AttackModule::set_add_reaction_frame(ID=0, Frames=2.0, Unk=false)
		}
		frame(Frame=20)
		FT_MOTION_RATE(FSM=0.575)
		if(is_excute){
			sv_battle_object::notify_event_msc_cmd(0x2127e37c07, GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES)
		}
		frame(Frame=29)
		if(is_excute){
			ATTACK(ID=0, Part=0, Bone=hash40("hat"), Damage=8.0, Angle=75, KBG=100, FKB=150, BKB=0, Size=4.0, X=0.0, Y=8.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.75, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=true, ShieldDamage=-7, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_KAMEHIT, Type=ATTACK_REGION_BODY)
			AttackModule::set_add_reaction_frame(ID=0, Frames=-1.0, Unk=false)
		}
		frame(Frame=35)
		FT_MOTION_RATE(FSM=1)
		frame(Frame=40)
		if(is_excute){
			AttackModule::clear_all()
		}
    });
}	
#[acmd_script(
    agent = "mario",
    script =  "game_attackairn",
    category = ACMD_GAME)]
unsafe fn mario_nair(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		frame(Frame=3)
		if(is_excute){
			WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
		}
		FT_MOTION_RATE(FSM=0.5)
		frame(Frame=11)
		FT_MOTION_RATE(FSM=1)
		frame(Frame=12)
		FT_MOTION_RATE(FSM=1.025)
		if(is_excute){
			WorkModule::on_flag(Flag=FIGHTER_MARIO_STATUS_SPECIAL_S_FLAG_SPECIAL_FALL)
			ATTACK(ID=0, Part=0, Bone=hash40("shoulderl"), Damage=9.0, Angle=361, KBG=100, FKB=0, BKB=30, Size=4.0, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.5, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_magic"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_MAGIC, Type=ATTACK_REGION_PUNCH)
			ATTACK(ID=1, Part=0, Bone=hash40("arml"), Damage=9.0, Angle=361, KBG=100, FKB=0, BKB=30, Size=4.6, X=3.4, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.5, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_magic"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_MAGIC, Type=ATTACK_REGION_PUNCH)
			ATTACK(ID=2, Part=0, Bone=hash40("shoulderr"), Damage=9.0, Angle=361, KBG=100, FKB=0, BKB=30, Size=4.0, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.5, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_magic"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_MAGIC, Type=ATTACK_REGION_PUNCH)
			ATTACK(ID=3, Part=0, Bone=hash40("armr"), Damage=9.0, Angle=361, KBG=100, FKB=0, BKB=30, Size=4.6, X=3.4, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.5, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_magic"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_MAGIC, Type=ATTACK_REGION_PUNCH)
			rust{
				let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
				let ENTRY_ID = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
				if SPIN[ENTRY_ID] == false{
					acmd!(lua_state, {
						SET_SPEED_EX(0.45, 1.75, KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN)
					});
				};
				SPIN[ENTRY_ID] = true;
			}
		}
		frame(Frame=22)
		FT_MOTION_RATE(FSM=1)
		if(is_excute){
			WorkModule::off_flag(Flag=FIGHTER_MARIO_STATUS_SPECIAL_S_FLAG_SPECIAL_FALL)
			AttackModule::clear_all()
		}
		frame(Frame=26)
		FT_MOTION_RATE(FSM=0.675)
		frame(Frame=41)
		FT_MOTION_RATE(FSM=1)
		if(is_excute){
			WorkModule::off_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
		}
    });
}
#[acmd_script(
    agent = "mario",
    script =  "effect_attackairn",
    category = ACMD_EFFECT)]
unsafe fn mario_nair_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
    });
}	
#[acmd_script(
    agent = "mario",
    script =  "sound_attackairn",
    category = ACMD_SOUND)]
unsafe fn mario_nair_sound(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		frame(Frame=11)
		if(is_excute){
			PLAY_SE(hash40("se_common_punch_kick_swing_m"))
			PLAY_SE(hash40("vc_mario_attack05"))
			PLAY_SE(hash40("se_mario_special_s01"))
			PLAY_SE(hash40("se_mario_attackair_l01"))
		}
    });
}		
#[acmd_script(
    agent = "mario",
    scripts =  ["game_specialn", "game_specialairn"],
    category = ACMD_GAME)]
unsafe fn mario_fireball(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		frame(Frame=14)
		if(is_excute){
			ArticleModule::generate_article(FIGHTER_MARIO_GENERATE_ARTICLE_FIREBALL, false, 0)
		}
    });
}	
#[fighter_frame_callback]
pub fn mario_frame(fighter : &mut L2CFighterCommon) {
    unsafe {
        let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
		let fighter_kind = smash::app::utility::get_kind(boma);
		let ENTRY_ID = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
		let status_kind = smash::app::lua_bind::StatusModule::status_kind(boma);
		if fighter_kind == *FIGHTER_KIND_MARIO {
			if [*FIGHTER_STATUS_KIND_SPECIAL_HI, *FIGHTER_STATUS_KIND_SPECIAL_S].contains(&status_kind) == false {
				ArticleModule::remove_exist(boma, *FIGHTER_MARIO_GENERATE_ARTICLE_CAPPY,smash::app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
			};
			if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_S {
				if StatusModule::situation_kind(boma) == *SITUATION_KIND_GROUND {
					if MotionModule::frame(boma) > 9.0 && MotionModule::frame(boma) < 73.0 && AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_SHIELD) == false {
						if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_JUMP) || ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_FLICK_JUMP) {
							StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_JUMP_SQUAT, true);
						};
					};
					if KineticModule::get_kinetic_type(boma) != *FIGHTER_KINETIC_TYPE_MOTION {
						KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_MOTION);
					};
				} else {
					SIDEB[ENTRY_ID] = true;
					if KineticModule::get_kinetic_type(boma) != *FIGHTER_KINETIC_TYPE_MOTION_AIR {
						KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_MOTION_AIR);
					};
					if MotionModule::frame(boma) < 23.0 {
						if MotionModule::frame(boma) < 2.0 {
							KineticModule::clear_speed_all(boma);
							let speed = smash::phx::Vector3f { x: 1.5, y: 1.5, z: 0.0 };
							KineticModule::add_speed(boma, &speed);
						} else {
							KineticModule::clear_speed_all(boma);
							let speed = smash::phx::Vector3f { x: 0.5, y: 0.5, z: 0.0 };
							KineticModule::add_speed(boma, &speed);
						};
					};
					if MotionModule::frame(boma) < 46.0 {
						StatusModule::set_keep_situation_air(boma, true);
					} else {
						StatusModule::set_keep_situation_air(boma, false);
					};
					if MotionModule::frame(boma) > 74.0 {
						CancelModule::enable_cancel(boma);
					};
					if MotionModule::frame(boma) > 56.0 && (ControlModule::get_command_flag_cat(boma, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_HI) != 0{
						StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_SPECIAL_HI, true);
					};
				};
			};
			if StatusModule::situation_kind(boma) != *SITUATION_KIND_AIR {
				SPIN[ENTRY_ID] = false;
				SIDEB[ENTRY_ID] = false;
			};
			if [hash40("attack_air_n")].contains(&MotionModule::motion_kind(boma)) && MotionModule::frame(boma) > 9.0 && MotionModule::frame(boma) < 22.0 && StopModule::is_stop(boma) == false {
				if SPIN_EFF[ENTRY_ID] == 0{
					let handbg1: u32 = EffectModule::req_follow(boma, smash::phx::Hash40::new("sys_spin_wind"), smash::phx::Hash40::new("top"), &SPIN2, &NOSPIN, 1.1, true, 0, 0, 0, 0, 0, true, true) as u32;
					let handbg2: u32 = EffectModule::req_follow(boma, smash::phx::Hash40::new("sys_spin_wind"), smash::phx::Hash40::new("top"), &SPIN3, &NOSPIN, 1.1, true, 0, 0, 0, 0, 0, true, true) as u32;
					let handbg3: u32 = EffectModule::req_follow(boma, smash::phx::Hash40::new("sys_spin_wind"), smash::phx::Hash40::new("top"), &SPIN4, &NOSPIN, 1.1, true, 0, 0, 0, 0, 0, true, true) as u32;
					let handbg4: u32 = EffectModule::req_follow(boma, smash::phx::Hash40::new("sys_spin_wind"), smash::phx::Hash40::new("top"), &SPIN5, &NOSPIN, 1.1, true, 0, 0, 0, 0, 0, true, true) as u32;
					let hand1: u32 = EffectModule::req_follow(boma, smash::phx::Hash40::new("sys_spin_wind"), smash::phx::Hash40::new("top"), &SPIN1, &NOSPIN, 1.1, true, 0, 0, 0, 0, 0, true, true) as u32;
					let star1: u32 = EffectModule::req_follow(boma, smash::phx::Hash40::new("sys_starrod_splash"), smash::phx::Hash40::new("top"), &STAR1, &NOSPIN, 0.5, true, 0, 0, 0, 0, 0, true, true) as u32;
					let star2: u32 = EffectModule::req_follow(boma, smash::phx::Hash40::new("sys_starrod_splash"), smash::phx::Hash40::new("top"), &STAR2, &NOSPIN, 0.5, true, 0, 0, 0, 0, 0, true, true) as u32;
					let star3: u32 = EffectModule::req_follow(boma, smash::phx::Hash40::new("sys_starrod_splash"), smash::phx::Hash40::new("top"), &STAR3, &NOSPIN, 0.5, true, 0, 0, 0, 0, 0, true, true) as u32;
					let star4: u32 = EffectModule::req_follow(boma, smash::phx::Hash40::new("sys_starrod_splash"), smash::phx::Hash40::new("handl"), &NOSPIN, &NOSPIN, 0.7, true, 0, 0, 0, 0, 0, true, true) as u32;
					let star5: u32 = EffectModule::req_follow(boma, smash::phx::Hash40::new("sys_starrod_splash"), smash::phx::Hash40::new("handr"), &NOSPIN, &NOSPIN, 0.7, true, 0, 0, 0, 0, 0, true, true) as u32;
					let star6: u32 = EffectModule::req_follow(boma, smash::phx::Hash40::new("sys_starrod_splash"), smash::phx::Hash40::new("top"), &STAR4, &NOSPIN, 0.5, true, 0, 0, 0, 0, 0, true, true) as u32;
					let star7: u32 = EffectModule::req_follow(boma, smash::phx::Hash40::new("sys_starrod_splash"), smash::phx::Hash40::new("top"), &STAR5, &NOSPIN, 0.5, true, 0, 0, 0, 0, 0, true, true) as u32;
					EffectModule::set_rgb(boma, hand1, 0.045, 0.345, 2.05);
					EffectModule::set_alpha(boma, hand1, 0.2);
					EffectModule::set_rgb(boma, handbg1, 0.045, 0.345, 2.05);
					EffectModule::set_alpha(boma, handbg1, 0.15);
					EffectModule::set_rgb(boma, handbg2, 0.045, 0.045, 2.05);
					EffectModule::set_alpha(boma, handbg2, 0.15);
					EffectModule::set_rgb(boma, handbg3, 0.045, 0.345, 2.05);
					EffectModule::set_alpha(boma, handbg3, 0.15);
					EffectModule::set_rgb(boma, handbg4, 0.045, 0.345, 2.05);
					EffectModule::set_alpha(boma, handbg4, 0.15);
					EffectModule::set_rgb(boma, star1, 1.65, 1.95, 1.85);
					EffectModule::set_rgb(boma, star2, 1.65, 1.95, 1.85);
					EffectModule::set_rgb(boma, star3, 1.65, 1.95, 1.85);
					EffectModule::set_rgb(boma, star4, 1.65, 1.95, 1.85);
					EffectModule::set_rgb(boma, star5, 1.65, 1.95, 1.85);
					EffectModule::set_rgb(boma, star6, 1.65, 1.95, 1.85);
					EffectModule::set_rgb(boma, star7, 1.65, 1.95, 1.85);
					EffectModule::set_alpha(boma, star1, 0.6);
					EffectModule::set_alpha(boma, star2, 0.6);
					EffectModule::set_alpha(boma, star3, 0.6);
					EffectModule::set_alpha(boma, star4, 0.6);
					EffectModule::set_alpha(boma, star5, 0.6);
					EffectModule::set_alpha(boma, star6, 0.6);
					EffectModule::set_alpha(boma, star7, 0.6);
				};
				SPIN_EFF[ENTRY_ID] += 1;
				if SPIN_EFF[ENTRY_ID] > 4 {
					SPIN_EFF[ENTRY_ID] = 0;
				};
			} else {
				SPIN_EFF[ENTRY_ID] = 0;
			};
		};
    }
}

pub fn install() {
    smashline::install_acmd_scripts!(
		mario_dtilt,
		mario_uair,
		mario_nair,
		mario_nair_effect,
		mario_nair_sound,
		mario_air_sideb,
		mario_air_sideb_eff,
		mario_air_sideb_snd,
		mario_ground_sideb,
		mario_ground_sideb_eff,
		mario_ground_sideb_snd,
		mario_fireball
    );
    smashline::install_agent_frame_callbacks!(
        mario_frame
    );
}
