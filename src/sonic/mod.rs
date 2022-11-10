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
static mut LIGHTSPEED :  smash::phx::Vector3f =  smash::phx::Vector3f { x: 0.0, y: 6.5, z: 0.0 };
static mut LIGHTSPEED_ROT :  smash::phx::Vector3f =  smash::phx::Vector3f { x: 0.0, y: 180.0, z: 0.0 };
use smash::phx::Vector2f;
static mut BAN_SIDEB : [bool; 8] = [false; 8];

#[acmd_script(
    agent = "sonic",
    script =  "effect_attackhi3",
    category = ACMD_EFFECT,
	low_priority)]
unsafe fn sonic_utilt_eff(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		frame(Frame=5)
		if(is_excute){
			EFFECT_FOLLOW_FLIP(hash40("sys_attack_arc_d"), hash40("sys_attack_arc_d"), hash40("top"), 0, 13, 2.5, 95, -28.5, 131, 1.05, true, EF_FLIP_YZ)
			LAST_EFFECT_SET_RATE(2)
			LAST_EFFECT_SET_COLOR(1.5, 0.0, 0.0)
		}
		frame(Frame=12)
		if(is_excute){
			EFFECT_FOLLOW_FLIP(hash40("sys_attack_arc_d"), hash40("sys_attack_arc_d"), hash40("top"), 0, 17.5, 0.5, 80, 24, 180, 0.95, true, EF_FLIP_YZ)
			LAST_EFFECT_SET_RATE(2)
			LAST_EFFECT_SET_COLOR(1.5, 0.0, 0.0)
		}
		frame(Frame=22)
		if(is_excute){
			LANDING_EFFECT(hash40("sys_landing_smoke_s"), hash40("top"), 0, 0, 0, 0, 0, 0, 0.7, 0, 0, 0, 0, 0, 0, false)
		}
    });
}	
#[acmd_script(
    agent = "sonic",
    script =  "effect_attacks3",
    category = ACMD_EFFECT,
	low_priority)]
unsafe fn sonic_ftilt_eff(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		frame(Frame=4)
		if(is_excute){
			LANDING_EFFECT(hash40("sys_atk_smoke"), hash40("top"), 0, 0, 0, 0, 0, 0, 0.5, 0, 0, 0, 0, 0, 0, false)
		}
		frame(Frame=7)
		if(is_excute){
			EFFECT_FOLLOW_FLIP(hash40("sys_attack_speedline"), hash40("sys_attack_speedline"), hash40("top"), 0, 1, -3, -22, 0, 0, 1.1, true, EF_FLIP_YZ)
			LAST_EFFECT_SET_COLOR(3, 0.0, 0.0)
		}
		frame(Frame=8)
		if(is_excute){
			EFFECT(hash40("sys_attack_impact"), hash40("top"), 0, 9, 15.5, 0, 0, 0, 1.1, 0, 0, 0, 0, 0, 360, true)
		}
    });
}	
#[acmd_script(
    agent = "sonic",
    script =  "effect_throwf",
    category = ACMD_EFFECT,
	low_priority)]
unsafe fn sonic_fthrow_eff(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		frame(Frame=8)
		if(is_excute){
			EFFECT_FOLLOW_NO_STOP_FLIP(hash40("sys_attack_arc"), hash40("sys_attack_arc_d"), hash40("top"), -2, 13, 4, 6, -70, 80, 0.9, true, EF_FLIP_YZ)
			LAST_EFFECT_SET_COLOR(3, 0.0, 0.0)
			LAST_EFFECT_SET_RATE(1.5)
		}
		frame(Frame=12)
		if(is_excute){
			EFFECT_FLIP(hash40("sys_smash_flash_s"), hash40("sys_smash_flash_s"), hash40("throw"), 3, 2, 3, 0, 0, 0, 1.5, 0, 0, 0, 0, 0, 0, true, EF_FLIP_YZ)
		}
    });
}	
#[acmd_script(
    agent = "sonic",
    script =  "effect_throwb",
    category = ACMD_EFFECT,
	low_priority)]
unsafe fn sonic_bthrow_eff(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		frame(Frame=11)
		if(is_excute){
			LANDING_EFFECT(hash40("sys_landing_smoke_s"), hash40("top"), 0, 0, 0, 0, 0, 0, 0.6, 0, 0, 0, 0, 0, 0, true)
		}
		frame(Frame=17)
		if(is_excute){
			EFFECT_FOLLOW_FLIP_ALPHA(hash40("sys_spin_wind"), hash40("sys_spin_wind"), hash40("trans"), 0, 16, 0, 0, 0, 90, 0.65, true, EF_FLIP_YZ, 0.6)
		}
		frame(Frame=21)
		if(is_excute){
			EFFECT_FOLLOW_FLIP_ALPHA(hash40("sys_spin_wind"), hash40("sys_spin_wind"), hash40("trans"), 0, 16, 0, 0, 180, 90, 1, true, EF_FLIP_YZ, 0.6)
		}
		frame(Frame=41)
		if(is_excute){
			EFFECT_FOLLOW_NO_STOP_FLIP(hash40("sys_attack_arc"), hash40("sys_attack_arc_d"), hash40("top"), -2, 5, -1.5, 6, 110, 80, 0.9, true, EF_FLIP_YZ)
			LAST_EFFECT_SET_COLOR(3, 0.0, 0.0)
			LAST_EFFECT_SET_RATE(1.6)
			EFFECT_FLIP(hash40("sys_smash_flash_s"), hash40("sys_smash_flash_s"), hash40("throw"), 3, 3, 10, 0, 0, 0, 1.5, 0, 0, 0, 0, 0, 0, true, EF_FLIP_YZ)
		}
		frame(Frame=41)
		if(is_excute){
			LANDING_EFFECT(hash40("sys_landing_smoke"), hash40("top"), 0, 0, -3, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false)
		}
		frame(Frame=45)
		if(is_excute){
			EFFECT_OFF_KIND(hash40("sys_attack_arc"), false, true)
		}
    });
}	
#[acmd_script(
    agent = "sonic",
    script =  "effect_attacks3lw",
    category = ACMD_EFFECT,
	low_priority)]
unsafe fn sonic_ftiltlw_eff(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		frame(Frame=4)
		if(is_excute){
			LANDING_EFFECT(hash40("sys_atk_smoke"), hash40("top"), 0, 0, 0, 0, 0, 0, 0.5, 0, 0, 0, 0, 0, 0, false)
		}
		frame(Frame=7)
		if(is_excute){
			EFFECT_FOLLOW_FLIP(hash40("sys_attack_speedline"), hash40("sys_attack_speedline"), hash40("top"), 0, 3.5, -4, -5, 0, 0, 1.1, true, EF_FLIP_YZ)
			LAST_EFFECT_SET_COLOR(3, 0.0, 0.0)
		}
		frame(Frame=8)
		if(is_excute){
			EFFECT(hash40("sys_attack_impact"), hash40("top"), 0, 6, 15.5, 0, 0, 0, 1.1, 0, 0, 0, 0, 0, 360, true)
		}
    });
}	
#[acmd_script(
    agent = "sonic",
    script =  "effect_attacks3hi",
    category = ACMD_EFFECT,
	low_priority)]
unsafe fn sonic_ftilthi_eff(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		frame(Frame=4)
		if(is_excute){
			LANDING_EFFECT(hash40("sys_atk_smoke"), hash40("top"), 0, 0, 0, 0, 0, 0, 0.5, 0, 0, 0, 0, 0, 0, false)
		}
		frame(Frame=7)
		if(is_excute){
			EFFECT_FOLLOW_FLIP(hash40("sys_attack_speedline"), hash40("sys_attack_speedline"), hash40("top"), 0, 0, -3, -42, 0, 0, 1.1, true, EF_FLIP_YZ)
			LAST_EFFECT_SET_COLOR(3, 0.0, 0.0)
		}
		frame(Frame=8)
		if(is_excute){
			EFFECT(hash40("sys_attack_impact"), hash40("top"), 0, 13.2, 12.2, 0, 0, 0, 1.1, 0, 0, 0, 0, 0, 360, true)
		}
    });
}	
#[acmd_script(
    agent = "sonic",
    script =  "effect_attacklw3",
    category = ACMD_EFFECT,
	low_priority)]
unsafe fn sonic_dtilt_eff(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		frame(Frame=5)
		if(is_excute){
			EFFECT_FOLLOW_FLIP(hash40("sys_attack_arc_d"), hash40("sys_attack_arc_d"), hash40("top"), -2, 2, 2, 1, 5, 5, 0.925, true, EF_FLIP_YZ)
			LAST_EFFECT_SET_RATE(1.5)
			LAST_EFFECT_SET_COLOR(1.5, 0.0, 0.0)
		}
		frame(Frame=6)
		if(is_excute){
			FOOT_EFFECT(hash40("sys_run_smoke"), hash40("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false)
		}
    });
}		
#[acmd_script(
    agent = "sonic",
    script =  "effect_attackairb",
    category = ACMD_EFFECT,
	low_priority)]
unsafe fn sonic_bair_eff(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		frame(Frame=12)
		if(is_excute){
			EFFECT_FOLLOW_FLIP(hash40("sys_attack_arc"), hash40("sys_attack_arc"), hash40("top"), 0, 7.5, -6.5, 160, 60, 15, 1, true, EF_FLIP_YZ)
			LAST_EFFECT_SET_RATE(2)
			LAST_EFFECT_SET_COLOR(1.5, 0.0, 0.0)
		}
		frame(Frame=16)
		if(is_excute){
			EFFECT_OFF_KIND(hash40("sys_attack_arc"), true, true)
		}
    });
}			
#[acmd_script(
    agent = "sonic",
    script =  "effect_attackairhi",
    category = ACMD_EFFECT,
	low_priority)]
unsafe fn sonic_uair_eff(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		frame(Frame=4)
		if(is_excute){
			EFFECT_FOLLOW(hash40("sys_attack_speedline"), hash40("top"), 0, 7.5, 2.5, -10, 0, 0, 0.5, true)
			LAST_EFFECT_SET_COLOR(3, 0.0, 0.0)
			EFFECT_FOLLOW(hash40("sys_attack_speedline"), hash40("top"), -1, 7.5, -2.5, 190, 0, 0, 0.5, true)
			LAST_EFFECT_SET_COLOR(3, 0.0, 0.0)
		}
		frame(Frame=13)
		if(is_excute){
			EFFECT_FOLLOW_NO_STOP_FLIP(hash40("sonic_atk_hi"), hash40("sonic_atk_hi"), hash40("top"), 0.8, 8.7, 0, 59, 74, 154, 1, true, EF_FLIP_YZ)
			LAST_EFFECT_SET_COLOR(3, 0.0, 0.0)
			EFFECT_FOLLOW_NO_STOP_FLIP(hash40("sonic_atk_hi"), hash40("sonic_atk_hi"), hash40("top"), -0.9, 8.7, 0, 245, -77, 22, 1, true, EF_FLIP_YZ)
			LAST_EFFECT_SET_COLOR(3, 0.0, 0.0)
		}
		frame(Frame=14)
		if(is_excute){
			EFFECT(hash40("sys_attack_impact"), hash40("top"), 0, 19.5, 0, 0, 0, 0, 1.55, 0, 0, 0, 0, 0, 360, false)
		}
    });
}		
#[acmd_script(
    agent = "sonic",
    script =  "effect_attackdash",
    category = ACMD_EFFECT,
	low_priority)]
unsafe fn sonic_da_eff(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		frame(Frame=4)
		if(is_excute){
			LANDING_EFFECT(hash40("sys_atk_smoke"), hash40("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false)
			/*EFFECT_FOLLOW_FLIP(hash40("sys_attack_line"), hash40("sys_attack_line"), hash40("top"), 1, 7, -9, -7, 0, 0, 1.2, true, EF_FLIP_YZ)
			LAST_EFFECT_SET_COLOR(1.5, 0.0, 0.0)*/
		}
		/*frame(Frame=5)
		if(is_excute){
			EFFECT(hash40("sys_attack_impact"), hash40("top"), 14, 9.5, 0, 0, 0, 0, 1.15, 0, 0, 0, 0, 0, 360, false)
		}
		frame(Frame=22)
		for(3 Iterations){
			if(is_excute){
				FOOT_EFFECT(hash40("sys_turn_smoke"), hash40("top"), 4, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false)
			}
			wait(Frames=5)
		}*/
    });
}		
#[acmd_script(
    agent = "sonic",
    script =  "effect_attack13",
    category = ACMD_EFFECT,
	low_priority)]
unsafe fn sonic_jab_eff(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		frame(Frame=3)
		if(is_excute){
			EFFECT_FOLLOW_FLIP(hash40("sys_attack_line"), hash40("sys_attack_line"), hash40("top"), -1.7, 6.5, -2, -10, 7, 0, 1, true, EF_FLIP_YZ)
			LAST_EFFECT_SET_COLOR(1.5, 0.0, 0.0)
		}
		frame(Frame=4)
		if(is_excute){
			FOOT_EFFECT(hash40("sys_run_smoke"), hash40("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false)
			EFFECT(hash40("sys_attack_impact"), hash40("top"), 13, 8.5, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 360, false)
			LAST_EFFECT_SET_ALPHA(0.7)
		}
    });
}		
#[acmd_script(
    agent = "sonic",
    script =  "effect_attacklw4",
    category = ACMD_EFFECT,
	low_priority)]
unsafe fn sonic_dsmash_eff(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		frame(Frame=1)
		if(is_excute){
			EFFECT(hash40("sys_smash_flash"), hash40("toer"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true)
		}
		frame(Frame=11)
		if(is_excute){
			EFFECT(hash40("sys_attack_line"), hash40("top"), 0, 2.5, -2, 0, 0, 0, 1.1, 0, 0, 0, 0, 0, 0, true)
			LAST_EFFECT_SET_RATE(1)
			LAST_EFFECT_SET_COLOR(1.5, 0.0, 0.0)
			EFFECT(hash40("sys_attack_line"), hash40("top"), -1, 2.5, 4, 180, 0, 0, 1.1, 0, 0, 0, 0, 0, 0, true)
			LAST_EFFECT_SET_RATE(1)
			LAST_EFFECT_SET_COLOR(1.5, 0.0, 0.0)
		}
		frame(Frame=12)
		if(is_excute){
			LANDING_EFFECT(hash40("sys_down_smoke"), hash40("top"), 0, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false)
			LAST_EFFECT_SET_RATE(1.2)
			EFFECT(hash40("sys_attack_impact"), hash40("top"), 0, 2.5, 17, 0, 0, 0, 1.5, 0, 0, 0, 0, 0, 360, true)
			EFFECT(hash40("sys_attack_impact"), hash40("top"), 0, 2.5, -15, 0, 0, 0, 1.5, 0, 0, 0, 0, 0, 360, true)
			EFFECT_FOLLOW_NO_STOP_FLIP(hash40("sys_attack_speedline"), hash40("sys_attack_speedline"), hash40("top"), 0, 2.5, -2, 0, 0, 0, 1.1, true, EF_FLIP_YZ)
			LAST_EFFECT_SET_COLOR(3, 0.0, 0.0)
			EFFECT_FOLLOW_NO_STOP_FLIP(hash40("sys_attack_speedline"), hash40("sys_attack_speedline"), hash40("top"), 0, 2.5, 4, 0, 180, 0, 1.1, true, EF_FLIP_YZ)
			LAST_EFFECT_SET_COLOR(3, 0.0, 0.0)
		}
    });
}	
#[acmd_script(
    agent = "sonic",
    script =  "effect_downattacku",
    category = ACMD_EFFECT,
	low_priority)]
unsafe fn sonic_getup_attack_eff(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		frame(Frame=14)
		if(is_excute){
			LANDING_EFFECT_FLIP(hash40("sys_whirlwind_r"), hash40("sys_whirlwind_l"), hash40("top"), -2, 0, 0, 0, 0, 0, 1.2, 0, 0, 0, 0, 0, 0, false, EF_FLIP_NONE)
		}
		frame(Frame=15)
		if(is_excute){
			EFFECT_FOLLOW_FLIP(hash40("sys_attack_arc_d"), hash40("sys_attack_arc_d"), hash40("top"), -1, 5, 8, 185, 150, 5, 1.1, true, EF_FLIP_YZ)
			LAST_EFFECT_SET_COLOR(3, 0.0, 0.0)
		}
		frame(Frame=19)
		if(is_excute){
			EFFECT_OFF_KIND(hash40("sys_attack_arc_d"), false, false)
			EFFECT_FOLLOW_FLIP(hash40("sys_attack_arc_d"), hash40("sys_attack_arc_d"), hash40("top"), 0, 5, -6.5, 185, -20, 8, 1.1, true, EF_FLIP_YZ)
			LAST_EFFECT_SET_COLOR(3, 0.0, 0.0)
		}
		frame(Frame=25)
		if(is_excute){
			EFFECT_OFF_KIND(hash40("sys_attack_arc_d"), false, false)
		}
    });
}		
#[acmd_script(
    agent = "sonic",
    script =  "game_attackhi4",
    category = ACMD_GAME,
	low_priority)]
unsafe fn sonic_usmash(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		frame(Frame=3)
		if(is_excute){
			WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD)
		}
		frame(Frame=8)
		if(is_excute){
			ATTACK(ID=0, Part=0, Bone=hash40("kneer"), Damage=14.0, Angle=80, KBG=96, FKB=0, BKB=33, Size=4.0, X=4.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.1, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
			ATTACK(ID=1, Part=0, Bone=hash40("kneer"), Damage=14.0, Angle=80, KBG=96, FKB=0, BKB=33, Size=4.0, X=-1.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.1, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
		}
		wait(Frames=5)
		if(is_excute){
			ATTACK(ID=0, Part=0, Bone=hash40("kneer"), Damage=12.0, Angle=80, KBG=93, FKB=0, BKB=32, Size=3.0, X=4.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.9, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
			ATTACK(ID=1, Part=0, Bone=hash40("kneer"), Damage=12.0, Angle=80, KBG=93, FKB=0, BKB=32, Size=3.0, X=-1.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.9, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
		}
		frame(Frame=19)
		FT_MOTION_RATE(FSM=0.775)
		if(is_excute){
			AttackModule::clear_all()
		}
		frame(Frame=31)
		FT_MOTION_RATE(FSM=1)
    });
}		
#[acmd_script(
    agent = "sonic",
    script =  "sound_attackhi4",
    category = ACMD_SOUND,
	low_priority)]
unsafe fn sonic_usmash_snd(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		frame(Frame=7)
		if(is_excute){
			PLAY_SE(hash40("se_sonic_swing_l"))
			PLAY_SEQUENCE(hash40("seq_sonic_rnd_attack"))
		}
		frame(Frame=37)
		if(is_excute){
			PLAY_SE(hash40("se_sonic_landing02"))
		}
    });
}	
#[acmd_script(
    agent = "sonic",
    script =  "expression_attackhi4",
    category = ACMD_EXPRESSION,
	low_priority)]
unsafe fn sonic_usmash_expr(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    /*acmd!(lua_state, {
		frame(Frame=7)
		if(is_excute){
			PLAY_SE(hash40("se_sonic_swing_l"))
			PLAY_SEQUENCE(hash40("seq_sonic_rnd_attack"))
		}
		frame(Frame=37)
		if(is_excute){
			PLAY_SE(hash40("se_sonic_landing02"))
		}
    });*/
}		
#[acmd_script(
    agent = "sonic",
    script =  "effect_attackhi4",
    category = ACMD_EFFECT,
	low_priority)]
unsafe fn sonic_usmash_eff(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		frame(Frame=1)
		if(is_excute){
			EFFECT(hash40("sys_smash_flash"), hash40("toer"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true)
		}
		frame(Frame=3)
		if(is_excute){
			LANDING_EFFECT(hash40("sys_down_smoke"), hash40("top"), 2, 0, -5, 0, 0, 0, 1, 12, 0, 4, 0, 0, 0, false)
		}
		frame(Frame=9)
		if(is_excute){
			EFFECT_FOLLOW_FLIP(hash40("sys_attack_arc"), hash40("sys_attack_arc"), hash40("top"), 0, 17, 1, 7, 43, 92, 1, true, EF_FLIP_YZ)
			LAST_EFFECT_SET_RATE(0.7)
			LAST_EFFECT_SET_COLOR(1.5, 0.0, 0.0)
		}
		frame(Frame=27)
		if(is_excute){
			LANDING_EFFECT(hash40("sys_landing_smoke_s"), hash40("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false)
		}
    });
}		
#[acmd_script(
    agent = "sonic",
    script =  "game_attackairf",
    category = ACMD_GAME,
	low_priority)]
unsafe fn sonic_fair(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		frame(Frame=2)
		FT_MOTION_RATE(FSM=2)
		if(is_excute){
			WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
		}
		frame(Frame=5)
		FT_MOTION_RATE(FSM=1)
		if(is_excute){
			ATTACK(ID=0, Part=0, Bone=hash40("kneer"), Damage=2.0, Angle=367, KBG=25, FKB=0, BKB=70, Size=5.0, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
			ATTACK(ID=1, Part=0, Bone=hash40("kneer"), Damage=2.0, Angle=367, KBG=25, FKB=0, BKB=70, Size=5.5, X=4.5, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
		}
		wait(Frames=1)
		if(is_excute){
			AttackModule::clear_all()
		}
		frame(Frame=10)
		if(is_excute){
			ATTACK(ID=0, Part=0, Bone=hash40("kneel"), Damage=2.0, Angle=367, KBG=25, FKB=0, BKB=70, Size=5.0, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
			ATTACK(ID=1, Part=0, Bone=hash40("kneel"), Damage=2.0, Angle=367, KBG=25, FKB=0, BKB=70, Size=5.5, X=4.5, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
		}
		wait(Frames=2)
		if(is_excute){
			AttackModule::clear_all()
		}
		frame(Frame=18)
		if(is_excute){
			ATTACK(ID=0, Part=0, Bone=hash40("kneer"), Damage=6.0, Angle=361, KBG=80, FKB=0, BKB=65, Size=6.5, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.2, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
			ATTACK(ID=1, Part=0, Bone=hash40("kneer"), Damage=6.0, Angle=361, KBG=80, FKB=0, BKB=65, Size=7.5, X=4.5, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.2, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
		}
		wait(Frames=2)
		if(is_excute){
			AttackModule::clear_all()
		}
		frame(Frame=35)
		if(is_excute){
			WorkModule::off_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
		}
    });
}	
#[acmd_script(
    agent = "sonic",
    script =  "sound_attackairf",
    category = ACMD_SOUND,
	low_priority)]
unsafe fn sonic_fair_snd(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		frame(Frame=4)
		if(is_excute){
			PLAY_SE(hash40("se_sonic_swing_s"))
		}
		frame(Frame=9)
		if(is_excute){
			PLAY_SE(hash40("se_sonic_swing_s"))
		}
		frame(Frame=17)
		if(is_excute){
			PLAY_SE(hash40("se_sonic_swing_l"))
			PLAY_SEQUENCE(hash40("seq_sonic_rnd_attack"))
		}
    });
}		
#[acmd_script(
    agent = "sonic",
    script =  "effect_attackairf",
    category = ACMD_EFFECT,
	low_priority)]
unsafe fn sonic_fair_eff(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		frame(Frame=4)
		if(is_excute){
			EFFECT_FOLLOW(hash40("sys_attack_arc"), hash40("top"), 1.5, 5.7, 0.0, 195, 180, -8, 0.8, true)
			LAST_EFFECT_SET_RATE(1.6)
			LAST_EFFECT_SET_COLOR(1.5, 0.0, 0.0)
		}
		frame(Frame=7)
		if(is_excute){
			EFFECT_OFF_KIND(hash40("sys_attack_arc"), false, false)
		}
		frame(Frame=8)
		if(is_excute){
			EFFECT_FOLLOW(hash40("sys_attack_arc"), hash40("top"), 1.5, 5.7, 0.0, 195, 180, -8, 0.8, true)
			LAST_EFFECT_SET_RATE(1.6)
			LAST_EFFECT_SET_COLOR(1.5, 0.0, 0.0)
		}
		frame(Frame=17)
		if(is_excute){
			EFFECT_OFF_KIND(hash40("sys_attack_arc"), false, false)
		}
		frame(Frame=18)
		if(is_excute){
			EFFECT_FOLLOW(hash40("sys_attack_arc"), hash40("top"), 1.5, 5.7, 0.0, 195, 180, -8, 1.1, true)
			LAST_EFFECT_SET_RATE(1.7)
			LAST_EFFECT_SET_COLOR(1.5, 0.0, 0.0)
		}
    });
}		
#[acmd_script(
    agent = "sonic",
    script =  "game_attackairn",
    category = ACMD_GAME,
	low_priority)]
unsafe fn sonic_nair(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		if(is_excute){
			WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
		}
		frame(Frame=6)
		if(is_excute){
			ATTACK(ID=0, Part=0, Bone=hash40("hip"), Damage=9.0, Angle=50, KBG=80, FKB=0, BKB=30, Size=5.0, X=0.0, Y=1.5, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_BODY)
		}
		wait(Frames=4)
		if(is_excute){
			ATTACK(ID=0, Part=0, Bone=hash40("hip"), Damage=5.0, Angle=75, KBG=90, FKB=0, BKB=20, Size=4.0, X=0.0, Y=1.5, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_BODY)
		}
		frame(Frame=39)
		if(is_excute){
			AttackModule::clear_all()
		}
		frame(Frame=48)
		if(is_excute){
			WorkModule::off_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
		}
    });
}	

#[acmd_script(
    agent = "sonic",
    script =  "game_attackairb",
    category = ACMD_GAME,
	low_priority)]
unsafe fn sonic_bair(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		FT_MOTION_RATE(FSM=0.8)
		frame(Frame=5)
		if(is_excute){
			WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
		}
		frame(Frame=10)
		FT_MOTION_RATE(FSM=1)
		frame(Frame=13)
		if(is_excute){
			ATTACK(ID=0, Part=0, Bone=hash40("kneer"), Damage=12.0, Angle=361, KBG=90, FKB=0, BKB=30, Size=6.0, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.2, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_B, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
			ATTACK(ID=1, Part=0, Bone=hash40("kneer"), Damage=12.0, Angle=361, KBG=90, FKB=0, BKB=30, Size=6.5, X=4.5, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.2, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_B, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
		}
		frame(Frame=15)
		if(is_excute){
			ATTACK(ID=0, Part=0, Bone=hash40("kneer"), Damage=8.0, Angle=361, KBG=76, FKB=0, BKB=40, Size=4.5, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_B, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
			ATTACK(ID=1, Part=0, Bone=hash40("kneer"), Damage=8.0, Angle=361, KBG=76, FKB=0, BKB=40, Size=5.0, X=4.5, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_B, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
		}
		wait(Frames=5)
		if(is_excute){
			AttackModule::clear_all()
		}
		frame(Frame=33)
		if(is_excute){
			WorkModule::off_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
		}
    });
}	
#[acmd_script(
    agent = "sonic",
    script =  "game_attackairhi",
    category = ACMD_GAME,
	low_priority)]
unsafe fn sonic_uair(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		FT_MOTION_RATE(FSM=0.6)
		frame(Frame=5)
		if(is_excute){
			WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
		}
		frame(Frame=14)
		FT_MOTION_RATE(FSM=1)
		if(is_excute){
			ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=8.0, Angle=91, KBG=68, FKB=0, BKB=66, Size=7.0, X=0.0, Y=20.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.2, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
			ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=8.0, Angle=91, KBG=68, FKB=0, BKB=66, Size=6.0, X=0.0, Y=15.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.2, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
			ATTACK(ID=2, Part=0, Bone=hash40("top"), Damage=8.0, Angle=91, KBG=68, FKB=0, BKB=66, Size=4.5, X=0.0, Y=10.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.2, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
			ATTACK(ID=3, Part=0, Bone=hash40("top"), Damage=8.0, Angle=91, KBG=68, FKB=0, BKB=66, Size=4.8, X=0.0, Y=10.0, Z=0.0, X2=0.0, Y2=8.0, Z2=0.0, Hitlag=1.2, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_A, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
		}
		wait(Frames=2)
		if(is_excute){
			AttackModule::clear_all()
		}
		frame(Frame=18)
		if(is_excute){
			WorkModule::off_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
		}
    });
}	
#[acmd_script(
    agent = "sonic",
    script =  "game_attackdash",
    category = ACMD_GAME,
	low_priority)]
unsafe fn sonic_da(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		frame(Frame=5)
		if(is_excute){
			ATTACK(ID=0, Part=0, Bone=hash40("kneel"), Damage=8.0, Angle=50, KBG=95, FKB=0, BKB=60, Size=4.8, X=5.5, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
			ATTACK(ID=1, Part=0, Bone=hash40("kneel"), Damage=8.0, Angle=50, KBG=95, FKB=0, BKB=60, Size=5.0, X=0.5, Y=0.0, Z=0.5, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
			ATK_SET_SHIELD_SETOFF_MUL_arg3(ID1=0, ID2=1, ShieldstunMul=0.75)
		}
		frame(Frame=7)
		if(is_excute){
			ATTACK(ID=0, Part=0, Bone=hash40("kneel"), Damage=5.7, Angle=80, KBG=100, FKB=0, BKB=60, Size=3.6, X=5.5, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
			ATTACK(ID=1, Part=0, Bone=hash40("kneel"), Damage=5.7, Angle=80, KBG=100, FKB=0, BKB=60, Size=4.2, X=0.5, Y=0.0, Z=0.5, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
			ATK_SET_SHIELD_SETOFF_MUL_arg3(ID1=0, ID2=1, ShieldstunMul=1.05)
		}
		frame(Frame=16)
		if(is_excute){
			AttackModule::clear_all()
		}
    });
}			
#[acmd_script(
    agent = "sonic",
    script =  "game_attacklw3",
    category = ACMD_GAME,
	low_priority)]
unsafe fn sonic_dtilt(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		frame(Frame=3)
		if(is_excute){
			JostleModule::set_status(false)
		}
		frame(Frame=6)
		if(is_excute){
			ATTACK(ID=0, Part=0, Bone=hash40("kneer"), Damage=8.0, Angle=80, KBG=100, FKB=0, BKB=50, Size=3.4, X=7.5, Y=0.0, Z=0.0, X2=0.0, Y2=0.0, Z2=0.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.2, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
			AttackModule::set_add_reaction_frame(ID=0, Frames=2.0, Unk=false)
		}
		wait(Frames=2)
		FT_MOTION_RATE(FSM=0.65)
		if(is_excute){
			AttackModule::clear_all()
			JostleModule::set_status(true)
		}
    });
}		
#[acmd_script(
    agent = "sonic",
    script =  "game_attackhi3",
    category = ACMD_GAME,
	low_priority)]
unsafe fn sonic_utilt(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		frame(Frame=6)
		if(is_excute){
			ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=2.0, Angle=96, KBG=100, FKB=100, BKB=0, Size=5.2, X=0.0, Y=8.0, Z=4.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=0.5, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_G, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
		}
		frame(Frame=7)
		if(is_excute){
			ATTACK(ID=1, Part=0, Bone=hash40("legr"), Damage=2.0, Angle=90, KBG=100, FKB=60, BKB=0, Size=3.0, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=0.5, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
			ATTACK(ID=2, Part=0, Bone=hash40("kneer"), Damage=2.0, Angle=95, KBG=100, FKB=35, BKB=0, Size=4.0, X=2.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=0.5, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
			ATTACK(ID=3, Part=0, Bone=hash40("kneer"), Damage=2.0, Angle=95, KBG=100, FKB=12, BKB=0, Size=5.0, X=7.0, Y=0.0, Z=-1.5, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=0.5, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
		}
		wait(Frames=2)
		if(is_excute){
			AttackModule::clear_all()
		}
		frame(Frame=13)
		if(is_excute){
			ATTACK(ID=0, Part=0, Bone=hash40("legl"), Damage=6.0, Angle=78, KBG=120, FKB=0, BKB=40, Size=4.0, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
			ATTACK(ID=1, Part=0, Bone=hash40("kneel"), Damage=6.0, Angle=78, KBG=120, FKB=0, BKB=40, Size=5.0, X=2.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
			ATTACK(ID=2, Part=0, Bone=hash40("kneel"), Damage=6.0, Angle=78, KBG=120, FKB=0, BKB=40, Size=7.0, X=7.0, Y=0.0, Z=1.5, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
		}
		wait(Frames=2)
		if(is_excute){
			AttackModule::clear_all()
		}
		frame(Frame=26)
		if(is_excute){
			CancelModule::enable_cancel()
		}
		frame(Frame=28)
		FT_MOTION_RATE(FSM=0.6)
		frame(Frame=38)
		FT_MOTION_RATE(FSM=1)
    });
}		
#[acmd_script(
    agent = "sonic",
    scripts =  ["game_attacks3", "game_attacks3hi", "game_attacks3lw"],
    category = ACMD_GAME,
	low_priority)]
unsafe fn sonic_ftilt(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		if(is_excute){
			JostleModule::set_status(false)
		}
		frame(Frame=8)
		if(is_excute){
			ATTACK(ID=0, Part=0, Bone=hash40("kneel"), Damage=7.0, Angle=361, KBG=115, FKB=0, BKB=30, Size=5.0, X=6.0, Y=0.0, Z=-1.6, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
			ATTACK(ID=1, Part=0, Bone=hash40("kneel"), Damage=5.0, Angle=361, KBG=115, FKB=0, BKB=30, Size=3.5, X=0.0, Y=0.0, Z=-1.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
			ATTACK(ID=2, Part=0, Bone=hash40("legl"), Damage=5.0, Angle=361, KBG=115, FKB=0, BKB=30, Size=3.5, X=-1.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
		}
		wait(Frames=4)
		if(is_excute){
			AttackModule::clear_all()
		}
		frame(Frame=26)
		if(is_excute){
			CancelModule::enable_cancel()
			JostleModule::set_status(true)
		}
    });
}	
#[acmd_script(
    agent = "sonic",
    script =  "game_attack11",
    category = ACMD_GAME,
	low_priority)]
unsafe fn sonic_jab1(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		frame(Frame=3)
		if(is_excute){
			ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=2.0, Angle=361, KBG=20, FKB=0, BKB=20, Size=1.4, X=0.0, Y=7.0, Z=6.7, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_PUNCH)
			ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=2.0, Angle=361, KBG=20, FKB=0, BKB=20, Size=1.4, X=0.0, Y=7.0, Z=9.2, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_PUNCH)
			ATTACK(ID=2, Part=0, Bone=hash40("top"), Damage=2.0, Angle=180, KBG=15, FKB=0, BKB=20, Size=1.6, X=0.0, Y=7.0, Z=12.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_FIGHTER, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_PUNCH)
			ATTACK(ID=3, Part=0, Bone=hash40("top"), Damage=2.0, Angle=361, KBG=15, FKB=0, BKB=20, Size=1.6, X=0.0, Y=7.0, Z=12.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_PUNCH)
			AttackModule::set_add_reaction_frame(ID=0, Frames=8.0, Unk=false)
			AttackModule::set_add_reaction_frame(ID=1, Frames=8.0, Unk=false)
			AttackModule::set_add_reaction_frame(ID=2, Frames=8.0, Unk=false)
			AttackModule::set_add_reaction_frame(ID=3, Frames=8.0, Unk=false)
		}
		wait(Frames=1)
		if(is_excute){
		AttackModule::clear_all()
		}
		frame(Frame=6)
		if(is_excute){
		WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO)
		}
		frame(Frame=10)
		if(is_excute){
		WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_FLAG_ENABLE_RESTART)
		}
    });
}	
#[acmd_script(
    agent = "sonic",
    script =  "game_attack12",
    category = ACMD_GAME,
	low_priority)]
unsafe fn sonic_jab2(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		frame(Frame=2)
		if(is_excute){
			ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=1.5, Angle=361, KBG=100, FKB=20, BKB=0, Size=2.3, X=0.0, Y=7.0, Z=4.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_PUNCH)
			ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=1.5, Angle=361, KBG=100, FKB=20, BKB=0, Size=2.3, X=0.0, Y=7.0, Z=8.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_PUNCH)
			ATTACK(ID=2, Part=0, Bone=hash40("top"), Damage=1.5, Angle=180, KBG=100, FKB=40, BKB=0, Size=2.8, X=0.0, Y=7.0, Z=13.5, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_PUNCH)
		}
		wait(Frames=1)
		if(is_excute){
			AttackModule::clear_all()
		}
		frame(Frame=6)
		if(is_excute){
			WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_FLAG_ENABLE_100)
			WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO)
		}
    });
}	
#[acmd_script(
    agent = "sonic",
    script =  "game_attack100",
    category = ACMD_GAME,
	low_priority)]
unsafe fn sonic_rapidjab(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		for(8 Iterations){
			if(is_excute){
				ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=0.5, Angle=361, KBG=20, FKB=0, BKB=7, Size=4.0, X=0.0, Y=6.5, Z=8.0, X2=0.0, Y2=6.5, Z2=14.5, Hitlag=0.6, SDI=0.2, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_rush"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_PUNCH)
				AttackModule::set_add_reaction_frame(ID=0, Frames=6.0, Unk=false)
			}
			wait(Frames=1)
			if(is_excute){
				AttackModule::clear_all()
			}
			wait(Frames=1)
		}
    });
}	
#[acmd_script(
    agent = "sonic",
    script =  "effect_attack100",
    category = ACMD_EFFECT,
	low_priority)]
unsafe fn sonic_rapidjab_eff(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		for(2 Iterations){
			if(is_excute){
				rust {
					let y_rand = (smash::app::sv_math::rand(hash40("fighter"), 100) as f32)/10.0;
					let x_rand = (smash::app::sv_math::rand(hash40("fighter"), 100) as f32)/10.0;
					macros::EFFECT(fighter, Hash40::new("sys_attack_impact"), Hash40::new("top"), 2.5+x_rand, 1.5+y_rand, 0, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, false);
				}
			}
			wait(Frames=4)
			if(is_excute){
				rust {
					let y_rand = (smash::app::sv_math::rand(hash40("fighter"), 100) as f32)/10.0;
					let x_rand = (smash::app::sv_math::rand(hash40("fighter"), 100) as f32)/10.0;
					macros::EFFECT(fighter, Hash40::new("sys_attack_impact"), Hash40::new("top"), 2.5+x_rand, 1.5+y_rand, 0, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, false);
				}
			}
			wait(Frames=4)
			if(is_excute){
				rust {
					let y_rand = (smash::app::sv_math::rand(hash40("fighter"), 100) as f32)/10.0;
					let x_rand = (smash::app::sv_math::rand(hash40("fighter"), 100) as f32)/10.0;
					macros::EFFECT(fighter, Hash40::new("sys_attack_impact"), Hash40::new("top"), 2.5+x_rand, 1.5+y_rand, 0, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, false);
				}
			}
			wait(Frames=4)
		}
    });
}
#[acmd_script(
    agent = "sonic",
    script =  "sound_attack100",
    category = ACMD_SOUND,
	low_priority)]
unsafe fn sonic_rapidjab_snd(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		for(8 Iterations){
			if(is_excute){
				PLAY_SE(hash40("se_sonic_swing_s"))
			}
			wait(Frames=1)
		}
    });
}	
#[acmd_script(
    agent = "sonic",
    script =  "game_attack100end",
    category = ACMD_GAME,
	low_priority)]
unsafe fn sonic_rapidjabend(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		frame(Frame=4)
		if(is_excute){
			ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=2.0, Angle=361, KBG=130, FKB=0, BKB=55, Size=4.6, X=0.0, Y=7.2, Z=2.2, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=2.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
			ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=2.0, Angle=361, KBG=130, FKB=0, BKB=55, Size=4.6, X=0.0, Y=8.0, Z=6.8, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=2.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
			ATTACK(ID=2, Part=0, Bone=hash40("top"), Damage=2.0, Angle=361, KBG=130, FKB=0, BKB=55, Size=4.6, X=0.0, Y=8.5, Z=8.0, X2=0.0, Y2=8.5, Z2=12.0, Hitlag=2.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
		}
		wait(Frames=2)
		if(is_excute){
			AttackModule::clear_all()
		}
    });
}	
#[acmd_script(
    agent = "sonic",
    script =  "game_attackairlw",
    category = ACMD_GAME,
	low_priority)]
unsafe fn sonic_dair(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		frame(Frame=3)
		if(is_excute){
			WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
		}
		frame(Frame=13)
		if(is_excute){
			ATTACK(ID=0, Part=0, Bone=hash40("kneel"), Damage=11.0, Angle=300, KBG=100, FKB=0, BKB=25, Size=5.0, X=0.0, Y=0.0, Z=0.0,  X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.15, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_A, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
			ATTACK(ID=1, Part=0, Bone=hash40("footl"), Damage=11.0, Angle=300, KBG=100, FKB=0, BKB=25, Size=5.0, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.15, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_G, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
		}
		frame(Frame=19)
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
    agent = "sonic",
    script =  "effect_attackairlw",
    category = ACMD_EFFECT,
	low_priority)]
unsafe fn sonic_dair_eff(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		frame(Frame=12)
		if(is_excute){
			EFFECT_FOLLOW(hash40("sys_attack_arc"), hash40("top"), -2, 5, 0, -60, 40, -130, 0.95, true)
			LAST_EFFECT_SET_COLOR(1.5, 0.0, 0.0)
		}
		frame(Frame=19)
		if(is_excute){
			EFFECT_OFF_KIND(hash40("sys_attack_arc"), true, true)
		}
    });
}
#[acmd_script(
    agent = "sonic",
    script =  "expression_attackairlw",
    category = ACMD_EXPRESSION,
	low_priority)]
unsafe fn sonic_dair_expr(fighter: &mut L2CAgentBase) {
			let lua_state = fighter.lua_state_agent;
			if macros::is_excute(fighter) {
				ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("sonic_talkcenter"), false);
				ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("sonic_talk"), false);
				ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("sonic_talkcenterflip"), false);
				ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("sonic_talkflip"), false);
				ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("sonic_facen"), true);
			};
			frame(fighter.lua_state_agent, 42.0);
			if macros::is_excute(fighter) {
				ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("sonic_talkcenter"), false);
				ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("sonic_talk"), false);
				ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("sonic_talkcenterflip"), false);
				ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("sonic_talkflip"), false);
				ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("sonic_facen"), true);
			};
}
#[acmd_script(
    agent = "sonic",
    script =  "sound_attackairlw",
    category = ACMD_SOUND,
	low_priority)]
unsafe fn sonic_dair_snd(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		frame(Frame=12)
		if(is_excute){
			PLAY_SE(hash40("se_sonic_smash_s02"))
			PLAY_SEQUENCE(hash40("seq_sonic_rnd_attack"))
		}
    });
}
#[acmd_script(
    agent = "sonic",
    script =  "expression_landingairlw",
    category = ACMD_EXPRESSION,
	low_priority)]
unsafe fn sonic_dair_land_expr(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
}
#[acmd_script(
    agent = "sonic",
    script =  "sound_landingairlw",
    category = ACMD_SOUND,
	low_priority)]
unsafe fn sonic_dair_land_snd(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		if(is_excute){
			PLAY_LANDING_SE(hash40("se_sonic_landing02"))
		}
    });
}

#[fighter_frame_callback]
pub fn sonic(fighter : &mut L2CFighterCommon) {
    unsafe {
        let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent); 
		let status_kind = smash::app::lua_bind::StatusModule::status_kind(boma);
		let lua_state = fighter.lua_state_agent;
		let ENTRY_ID = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
		let fighter_kind = smash::app::utility::get_kind(boma);
		let motion_kind = MotionModule::motion_kind(boma);
		let situation_kind = StatusModule::situation_kind(boma);
		let mut stick_x = ControlModule::get_stick_x(boma) ;
		stick_x *= PostureModule::lr(boma);
		if fighter_kind == *FIGHTER_KIND_SONIC {
			if motion_kind == hash40("attack_100") {
				if MotionModule::frame(fighter.module_accessor) >= 16.0 {
					if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_ATTACK) {
						MotionModule::change_motion(boma, Hash40::new("attack_100"), 0.0, 1.0, false, 0.0, false, false);
					} else {
						MotionModule::change_motion(boma, Hash40::new("attack_100_end"), 2.0, 1.0, false, 0.0, false, false);
					};
				};
			};
			if situation_kind != *SITUATION_KIND_AIR || (*FIGHTER_STATUS_KIND_DAMAGE..*FIGHTER_STATUS_KIND_DAMAGE_FALL).contains(&status_kind){
				BAN_SIDEB[ENTRY_ID] = false;
			};
			if BAN_SIDEB[ENTRY_ID] == true {
					CAN_SIDEB[ENTRY_ID] = 1;
			} else {
					CAN_SIDEB[ENTRY_ID] = 0;
			};
			if [hash40("special_s_start"), hash40("special_air_s_start")].contains(&MotionModule::motion_kind(boma)) {
				/*println!("cliff check: {}", GroundModule::cliff_check(boma));
				println!("is near cliff: {}", GroundModule::is_near_cliff(boma, PostureModule::pos_x(boma), PostureModule::pos_y(boma)));
				println!("can entry cliff: {}", );*/
				if GroundModule::can_entry_cliff(boma) == 1 && WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_CLIFF_COUNT) < 7 && WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_CLIFF_NO_CATCH_FRAME) < 1 {
					fighter.change_status(FIGHTER_STATUS_KIND_CLIFF_CATCH_MOVE.into(),false.into());
					macros::EFFECT_OFF_KIND(fighter, Hash40::new("sys_attack_speedline"), false, true);
				};
				/*WorkModule::unable_transition_term_forbid(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_JUMP_START);
				WorkModule::unable_transition_term_forbid(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_SQUAT);
				WorkModule::unable_transition_term_forbid(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_SQUAT_BUTTON);*/
				if StatusModule::situation_kind(boma) == *SITUATION_KIND_GROUND {
					/*if MotionModule::frame(boma) >= 24.0 && AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT){
						if MotionModule::frame(boma) >= 43.0 {
							StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_LANDING, true);
						};
					};*/
					if MotionModule::frame(boma) >= 54.0 && MotionModule::frame(boma) < 75.0  {
						MotionModule::set_rate(boma, 2.0);
					} else {
						MotionModule::set_rate(boma, 1.0);
					};
					if MotionModule::frame(boma) >= 85.0 {
						StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_WAIT, true);
					};
				} else {
					/*if MotionModule::frame(boma) >= 24.0 && AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT){
						MotionModule::set_rate(boma, 2.0);
					};*/
					if MotionModule::frame(boma) >= 54.0 {
						MotionModule::set_rate(boma, 2.0);
					};
					if MotionModule::frame(boma) >= 60.0 {
						CancelModule::enable_cancel(boma);
					};
					if MotionModule::frame(boma) >= 70.0 {
						StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_FALL, true);
					};
				};
				if GroundModule::is_wall_touch_line(boma, *GROUND_TOUCH_FLAG_SIDE as u32) && MotionModule::frame(boma) < 28.0 && stick_x < -0.7 {
					StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_PASSIVE_WALL_JUMP, true);
					macros::EFFECT_OFF_KIND(fighter, Hash40::new("sys_attack_speedline"), false, true);
				};
				if KineticModule::get_kinetic_type(boma) != *FIGHTER_KINETIC_TYPE_MOTION_AIR {
					KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_MOTION_AIR);
				};
				
				if StatusModule::situation_kind(boma) != *SITUATION_KIND_GROUND && MotionModule::motion_kind(boma) == hash40("special_s_start") {
					if MotionModule::frame(boma) < 34.0 {
						MotionModule::change_motion(boma, smash::phx::Hash40::new("special_air_s_start"), MotionModule::frame(boma)+1.0, 1.0, false, 0.0, false, false);
					} else {
						StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_FALL, true);
					};
				};
				if StatusModule::situation_kind(boma) != *SITUATION_KIND_AIR && MotionModule::motion_kind(boma) == hash40("special_air_s_start") {
					if MotionModule::frame(boma) < 42.0 {
						MotionModule::change_motion(boma, smash::phx::Hash40::new("special_s_start"), MotionModule::frame(boma)+1.0, 1.0, false, 0.0, false, false);
					} else {
						StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_LANDING, true);
					};
				};
				if MotionModule::frame(boma) == 14.0 {
					let lightspeed: u32 = EffectModule::req_follow(boma, smash::phx::Hash40::new("sys_attack_speedline"), smash::phx::Hash40::new("top"), &LIGHTSPEED, &LIGHTSPEED_ROT, 2.2, true, 0, 0, 0, 0, 0, true, true) as u32;
					EffectModule::set_rgb(boma, lightspeed, 0.2, 0.4, 10.0);
					EffectModule::set_rate(boma, lightspeed, 0.2);
					acmd!(lua_state, {
						STOP_SE(hash40("se_sonic_smash_h01"))
						PLAY_SE(hash40("vc_sonic_attack05"))
						PLAY_SE(hash40("se_sonic_swing_m"))
						PLAY_SE(hash40("se_sonic_swing_l"))
						PLAY_SE(hash40("se_sonic_attackair_l01"))
					});
				};
				if MotionModule::frame(boma) == 1.0 {
					acmd!(lua_state, {
						PLAY_SE(hash40("se_sonic_smash_h01"))
					});
				};
				notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS);
				notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07 as u64), *GROUND_CLIFF_CHECK_KIND_ALWAYS);
				notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07u64), *GROUND_CLIFF_CHECK_KIND_ALWAYS);
				notify_event_msc_cmd!(fighter, 0x2127e37c07 as u64, *GROUND_CLIFF_CHECK_KIND_ALWAYS);
				notify_event_msc_cmd!(fighter, 0x2127e37c07u64, *GROUND_CLIFF_CHECK_KIND_ALWAYS);
				BAN_SIDEB[ENTRY_ID] = true;
				/*if StatusModule::situation_kind(boma) == *SITUATION_KIND_GROUND {
					MotionModule::set_rate(boma, 0.6075);
				} else {
					MotionModule::set_rate(boma, 0.43875);
				};
				if MotionModule::frame(boma) >= 4.65 && MotionModule::frame(boma) < 6.0{
					VisibilityModule::set_whole(boma, false);
					JostleModule::set_status(boma, false);
					HitModule::set_whole(boma, smash::app::HitStatus(*HIT_STATUS_XLU), 0);
					if StatusModule::situation_kind(boma) == *SITUATION_KIND_GROUND {
						if GroundModule::ray_check(fighter.module_accessor, &Vector2f{ x: PostureModule::pos_x(boma), y: PostureModule::pos_y(boma)}, &Vector2f{ x: 20.0*PostureModule::lr(boma), y: 0.0}, false) == 0 {
							let pos = smash::phx::Vector3f { x: PostureModule::pos_x(boma)+(20.0*PostureModule::lr(boma)), y: PostureModule::pos_y(boma), z: 0.0 };
							PostureModule::set_pos(boma, &pos);
							PostureModule::init_pos(boma, &pos, true, true);
						};
					} else {
						if GroundModule::ray_check(fighter.module_accessor, &Vector2f{ x: PostureModule::pos_x(boma), y: PostureModule::pos_y(boma)}, &Vector2f{ x: 10.0*PostureModule::lr(boma), y: 0.0}, false) == 0 {
							let pos = smash::phx::Vector3f { x: PostureModule::pos_x(boma)+(10.0*PostureModule::lr(boma)), y: PostureModule::pos_y(boma), z: 0.0 };
							PostureModule::set_pos(boma, &pos);
							PostureModule::init_pos(boma, &pos, true, true);
						};
					};
				} else {
					VisibilityModule::set_whole(boma, true);
					JostleModule::set_status(boma, true);
					HitModule::set_whole(boma, smash::app::HitStatus(*HIT_STATUS_NORMAL), 0);
				};
				if MotionModule::frame(boma) >= 4.909086446285048 && MotionModule::frame(boma) < 5.72726752066589 {
					if StatusModule::situation_kind(boma) == *SITUATION_KIND_GROUND { 
						let speed = smash::phx::Vector3f { x: 35.0, y: 0.0, z: 0.0 };
						KineticModule::add_speed(boma, &speed);
					} else {
						let speed = smash::phx::Vector3f { x: 25.0, y: 1.0, z: 0.0 };
						KineticModule::add_speed(boma, &speed);
					};
				};
				if MotionModule::frame(boma) >= 6.0 {
					if StatusModule::situation_kind(boma) == *SITUATION_KIND_GROUND {
						if PostureModule::lr(boma) == 1.0 {
							MotionModule::change_motion(boma, smash::phx::Hash40::new("run_brake_r"), 0.0, 1.0, false, 0.0, false, false);
						} else {
							MotionModule::change_motion(boma, smash::phx::Hash40::new("run_brake_l"), 0.0, 1.0, false, 0.0, false, false);
						};
					} else {
						if PostureModule::lr(boma) == 1.0 {
							MotionModule::change_motion(boma, smash::phx::Hash40::new("run_brake_r"), 0.0, 1.0, false, 0.0, false, false);
						} else {
							MotionModule::change_motion(boma, smash::phx::Hash40::new("run_brake_l"), 0.0, 1.0, false, 0.0, false, false);
						};
					};
					let speed = smash::phx::Vector3f { x: 5.0, y: 0.0, z: 0.0 };
					KineticModule::add_speed(boma, &speed);
					let lightspeed: u32 = EffectModule::req_follow(boma, smash::phx::Hash40::new("sys_attack_speedline"), smash::phx::Hash40::new("top"), &LIGHTSPEED, &LIGHTSPEED_ROT, 2.5, true, 0, 0, 0, 0, 0, true, true) as u32;
					EffectModule::set_rgb(boma, lightspeed, 0.2, 0.4, 10.0);
					EffectModule::set_rate(boma, lightspeed, 0.25);
					acmd!(lua_state, {
						STOP_SE(hash40("se_sonic_smash_h01"))
						PLAY_SE(hash40("vc_sonic_attack05"))
						PLAY_SE(hash40("se_sonic_swing_m"))
						PLAY_SE(hash40("se_sonic_swing_l"))
						PLAY_SE(hash40("se_sonic_attackair_l01"))
					});
				};*/
			};
		};
	};
}
#[acmd_script(
    agent = "sonic",
    script =  "effect_specialsstart",
    category = ACMD_EFFECT,
	low_priority)]
unsafe fn sonic_lightspeed_dash_eff(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
	let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent); 
}	
#[acmd_script(
    agent = "sonic",
    script =  "sound_specialsstart",
    category = ACMD_SOUND,
	low_priority)]
unsafe fn sonic_lightspeed_dash_sound(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
}	
#[acmd_script(
    agent = "sonic",
    script =  "game_specialsstart",
    category = ACMD_GAME,
	low_priority)]
unsafe fn sonic_lightspeed_dash(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		if(is_excute){
			WorkModule::enable_transition_term(FIGHTER_STATUS_TRANSITION_TERM_ID_CLIFF_CATCH)
			rust {
				notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS);
				notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07 as u64), *GROUND_CLIFF_CHECK_KIND_ALWAYS);
				notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07u64), *GROUND_CLIFF_CHECK_KIND_ALWAYS);
				notify_event_msc_cmd!(fighter, 0x2127e37c07 as u64, *GROUND_CLIFF_CHECK_KIND_ALWAYS);
				notify_event_msc_cmd!(fighter, 0x2127e37c07u64, *GROUND_CLIFF_CHECK_KIND_ALWAYS);
			}
			WorkModule::enable_transition_term(FIGHTER_STATUS_TRANSITION_TERM_ID_CLIFF_CATCH)
		}
		frame(Frame=15)
		if(is_excute){
			ATTACK(ID=0, Part=0, Bone=hash40("hip"), Damage=8.0, Angle=70, KBG=50, FKB=0, BKB=80, Size=13.0, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.2, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_THRU, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_BODY)
			WorkModule::enable_transition_term(FIGHTER_STATUS_TRANSITION_TERM_ID_CLIFF_CATCH)
			rust {
				notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS);
				notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07 as u64), *GROUND_CLIFF_CHECK_KIND_ALWAYS);
				notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07u64), *GROUND_CLIFF_CHECK_KIND_ALWAYS);
				notify_event_msc_cmd!(fighter, 0x2127e37c07 as u64, *GROUND_CLIFF_CHECK_KIND_ALWAYS);
				notify_event_msc_cmd!(fighter, 0x2127e37c07u64, *GROUND_CLIFF_CHECK_KIND_ALWAYS);
			}
			WorkModule::enable_transition_term(FIGHTER_STATUS_TRANSITION_TERM_ID_CLIFF_CATCH)
		}
		frame(Frame=23)
		if(is_excute){
			AttackModule::clear_all()
		}
    });
}	
		
pub fn install() {
    smashline::install_acmd_scripts!(
		sonic_bair,
		sonic_uair,
		sonic_dtilt,
		sonic_utilt,
		sonic_ftilt,
		sonic_da,
		sonic_lightspeed_dash,
		sonic_lightspeed_dash_eff,
		sonic_lightspeed_dash_sound,
		sonic_bair_eff,
		sonic_da_eff,
		sonic_dsmash_eff,
		sonic_dtilt_eff,
		sonic_jab_eff,
		sonic_uair_eff,
		sonic_utilt_eff,
		sonic_getup_attack_eff,
		sonic_ftilt_eff,
		sonic_ftilthi_eff,
		sonic_ftiltlw_eff,
		sonic_fthrow_eff,
		sonic_bthrow_eff,
		sonic_jab2,
		sonic_jab1,
		sonic_rapidjab,
		sonic_rapidjab_eff,
		sonic_rapidjab_snd,
		sonic_rapidjabend,
		sonic_dair,
		sonic_dair_eff,
		sonic_dair_snd,
		sonic_dair_land_expr,
		sonic_dair_land_snd,
		sonic_dair_expr,
		sonic_fair,
		sonic_fair_eff,
		sonic_fair_snd,
		sonic_usmash,
		sonic_usmash_eff,
		sonic_usmash_snd,
		sonic_usmash_expr,
		sonic_nair
    );
	smashline::install_agent_frame_callbacks!(sonic);
}
