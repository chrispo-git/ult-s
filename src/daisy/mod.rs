use smash::app::sv_animcmd::*;
use smash::phx::{Hash40, Vector2f};
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
use crate::util::*;
static mut STATIC_MUT : [i32; 8] = [6; 8];

// A Once-Per-Fighter-Frame that only applies to Mario. Neat!
#[fighter_frame( agent = FIGHTER_KIND_DAISY )]
fn daisy_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent); 
		let status_kind = smash::app::lua_bind::StatusModule::status_kind(boma);
		let ENTRY_ID = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
		let motion_kind = MotionModule::motion_kind(boma);
		let frame = MotionModule::frame(boma);
    }
}
#[acmd_script(
    agent = "daisy",
    script =  "game_speciallw",
    category = ACMD_GAME,
	low_priority)]
unsafe fn daisy_downb(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		FT_MOTION_RATE(FSM=0.5)
		frame(Frame=14)
		FT_MOTION_RATE(FSM=1)
		if(is_excute){
			ArticleModule::generate_article(FIGHTER_DAISY_GENERATE_ARTICLE_DAIKON, false, -1)
		}
    });
}
#[acmd_script(
    agent = "daisy",
    script =  "effect_speciallw",
    category = ACMD_EFFECT,
	low_priority)]
unsafe fn daisy_downb_eff(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		frame(Frame=12)
		if(is_excute){
			EFFECT(hash40("sys_erace_smoke"), hash40("top"), 0, 8, 10, 0, 0, 0, 1.2, 0, 0, 0, 0, 0, 0, false)
		}
    });
}
#[acmd_script(
    agent = "daisy",
    script =  "game_attacklw3",
    category = ACMD_GAME,
	low_priority)]
unsafe fn daisy_dtilt(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		frame(Frame=8)
		if(is_excute){
			ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=9.0, Angle=30, KBG=40, FKB=0, BKB=50, Size=3.8, X=0.0, Y=2.5, Z=6.5, X2=0.0, Y2=2.0, Z2=10.2, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_KICK)
		}
		frame(Frame=10)
		FT_MOTION_RATE(FSM=0.5)
		if(is_excute){
			AttackModule::clear_all()
		}
		wait(Frames=6)
		FT_MOTION_RATE(FSM=1)
    });
}
#[acmd_script(
    agent = "daisy",
    script =  "game_attackairhi",
    category = ACMD_GAME,
	low_priority)]
unsafe fn daisy_uair(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		frame(Frame=5)
		if(is_excute){
			WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
		}
		frame(Frame=9)
		if(is_excute){
			ATTACK(ID=0, Part=0, Bone=hash40("havel"), Damage=11.0, Angle=77, KBG=80, FKB=0, BKB=42, Size=3.0, X=0.0, Y=6.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.9, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_OBJECT)
			ATTACK(ID=1, Part=0, Bone=hash40("havel"), Damage=11.0, Angle=77, KBG=80, FKB=0, BKB=42, Size=2.0, X=0.0, Y=2.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.9, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_OBJECT)
			ATTACK(ID=2, Part=0, Bone=hash40("havel"), Damage=11.0, Angle=77, KBG=80, FKB=0, BKB=42, Size=2.0, X=0.0, Y=-2.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.9, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_OBJECT)
		}
		frame(Frame=16)
		if(is_excute){
			AttackModule::clear_all()
		}
		frame(Frame=41)
		if(is_excute){
			WorkModule::off_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
		}
    });
}
#[acmd_script(
    agent = "daisy",
    script =  "effect_attackairhi",
    category = ACMD_EFFECT,
	low_priority)]
unsafe fn daisy_uair_eff(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		frame(Frame=8)
		if(is_excute){
			//EFFECT_FOLLOW_FLIP(hash40("daisy_smash_racket"), hash40("daisy_smash_racket_l"), hash40("top"), 4.5, 9.0, 0, 200, 180, 0, 1, true, EF_FLIP_YZ)
			EFFECT_FOLLOW(hash40("daisy_smash_racket"), hash40("top"), 0, 11, 0, 180, -70, 90, 1, true)
			LAST_EFFECT_SET_RATE(0.5)
		}
    });
}
#[acmd_script(
    agent = "daisy",
    script =  "expression_attackairhi",
    category = ACMD_EXPRESSION,
	low_priority)]
unsafe fn daisy_uair_expr(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		frame(Frame=3)
		if(is_excute){
			VisibilityModule::set_int64(hash40("smash_item") as i64, hash40("smash_item_racket") as i64)
			ItemModule::set_have_item_visibility(false, 0)
		}
		frame(Frame=27)
		if(is_excute){
			VisibilityModule::set_int64(hash40("smash_item") as i64, hash40("smash_item_none") as i64)
			ItemModule::set_have_item_visibility(true, 0)
		}
    });
}
#[acmd_script(
    agent = "daisy",
    script =  "game_attackdash",
    category = ACMD_GAME,
	low_priority)]
unsafe fn daisy_da(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		frame(Frame=9)
		if(is_excute){
			ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=8.0, Angle=361, KBG=55, FKB=0, BKB=72, Size=5.0, X=0.0, Y=8.0, Z=11.2, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.2, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_PUNCH)
			ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=8.0, Angle=361, KBG=55, FKB=0, BKB=72, Size=4.5, X=0.0, Y=8.0, Z=6.5, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.2, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_PUNCH)
		}
		wait(Frames=4)
		if(is_excute){
			AttackModule::clear_all()
		}
    });
}
#[acmd_script(
    agent = "daisy",
    script =  "effect_attackdash",
    category = ACMD_EFFECT,
	low_priority)]
unsafe fn daisy_da_eff(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		frame(Frame=5)
		if(is_excute){
			LANDING_EFFECT(hash40("sys_atk_smoke"), hash40("top"), 0, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false)
		}
		frame(Frame=7)
		if(is_excute){
			EFFECT_FOLLOW_FLIP_ALPHA(hash40("sys_attack_arc_b"), hash40("sys_attack_arc_b"), hash40("top"), 2, 7.5, 2, 11, 77, 16, 0.6, true, EF_FLIP_YZ, 0.4)
			EFFECT_FOLLOW_FLIP_ALPHA(hash40("sys_attack_arc_b"), hash40("sys_attack_arc_b"), hash40("top"), -2, 9, 2, 140, 120, -50, 0.6, true, EF_FLIP_YZ, 0.4)
		}
    });
}
#[acmd_script(
    agent = "daisy",
    script =  "game_attackairf",
    category = ACMD_GAME,
	low_priority)]
unsafe fn daisy_fair(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		frame(Frame=10)
		if(is_excute){
			WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
		}
		frame(Frame=16)
		if(is_excute){
			ATTACK(ID=0, Part=0, Bone=hash40("shoulderl"), Damage=13.0, Angle=65, KBG=72, FKB=0, BKB=50, Size=3.4, X=-0.5, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_OBJECT)
			ATTACK(ID=1, Part=0, Bone=hash40("arml"), Damage=13.0, Angle=65, KBG=72, FKB=0, BKB=50, Size=4.4, X=4.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_OBJECT)
		}
		frame(Frame=21)
		if(is_excute){
			AttackModule::clear_all()
		}
		frame(Frame=23)
		if(is_excute){
			WorkModule::off_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
		}
    });
}

pub fn install() {
	smashline::install_acmd_scripts!(
		daisy_downb,
		daisy_downb_eff,
		daisy_dtilt,
		daisy_uair,
		daisy_uair_eff,
		daisy_uair_expr,
		daisy_da,
		daisy_da_eff,
		daisy_fair
	);
}
