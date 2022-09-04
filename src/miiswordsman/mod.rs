use smash::hash40;
use smash::lib::lua_const::*;
use smash::app::lua_bind::*;
use smash::lua2cpp::*;
use smashline::*;
use smash_script::*;
use smash::app::sv_animcmd::frame;
use smash::phx::{Hash40, Vector2f};
use smash::app::ItemKind;
use smash::app::sv_animcmd::*;


static mut COUNTER_STORE: [bool; 8] = [false; 8];
static mut CUSTOM_BOMB: [bool; 8] = [false; 8];
static mut BOMB_TIME: [i32; 8] = [0; 8];

#[acmd_script(
    agent = "miiswordsman",
    script =  "game_catch",
    category = ACMD_GAME,
	low_priority)]
unsafe fn sword_grab(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		frame(Frame=5)
		if(is_excute){
			GrabModule::set_rebound(CanCatchRebound=true)
		}
		frame(Frame=6)
		if(is_excute){
			CATCH(ID=0, Bone=hash40("top"), Size=3.3, X=0.0, Y=6.6, Z=4.0, X2=0.0, Y2=6.6, Z2=11.7, Status=FIGHTER_STATUS_KIND_CAPTURE_PULLED, Ground_or_Air=COLLISION_SITUATION_MASK_G)
			CATCH(ID=1, Bone=hash40("top"), Size=1.65, X=0.0, Y=6.6, Z=2.35, X2=0.0, Y2=6.6, Z2=13.35, Status=FIGHTER_STATUS_KIND_CAPTURE_PULLED, Ground_or_Air=COLLISION_SITUATION_MASK_A)
		}
		rust{
			macros::game_CaptureCutCommon(fighter);
		}
		wait(Frames=2)
		if(is_excute){
			sv_module_access::grab(MA_MSC_CMD_GRAB_CLEAR_ALL)
			WorkModule::on_flag(Flag=FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT)
			GrabModule::set_rebound(CanCatchRebound=false)
		}
    });
}		
#[acmd_script(
    agent = "miiswordsman_lightshuriken",
    script =  "game_fly",
    category = ACMD_GAME,
	low_priority)]
unsafe fn sword_shuriken(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		if(is_excute){
			ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=4.0, Angle=45, KBG=45, FKB=0, BKB=65, Size=2.0, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_SPEED, SetWeight=false, ShieldDamage=-1, Trip=0.0, Rehit=0, Reflectable=true, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_OBJECT)
			AttackModule::enable_safe_pos()
		}
		wait(Frames=1)
		if(is_excute){
			ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=4.0, Angle=45, KBG=45, FKB=0, BKB=65, Size=2.0, X=0.0, Y=0.0, Z=0.0, X2=0.0, Y2=0.0, Z2=-3.5, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_SPEED, SetWeight=false, ShieldDamage=-1, Trip=0.0, Rehit=0, Reflectable=true, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_OBJECT)
		}
		frame(Frame=8)
		if(is_excute){
			ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=4.0, Angle=361, KBG=45, FKB=0, BKB=65, Size=2.0, X=0.0, Y=0.0, Z=0.0, X2=0.0, Y2=0.0, Z2=-2.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_SPEED, SetWeight=false, ShieldDamage=-1.5, Trip=0.0, Rehit=0, Reflectable=true, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_OBJECT)
		}
		frame(Frame=16)
		if(is_excute){
			ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=4.0, Angle=361, KBG=45, FKB=0, BKB=65, Size=2.0, X=0.0, Y=0.0, Z=0.0, X2=0.0, Y2=0.0, Z2=-2.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_SPEED, SetWeight=false, ShieldDamage=-2.5, Trip=0.0, Rehit=0, Reflectable=true, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_OBJECT)
		}
		frame(Frame=24)
		if(is_excute){
			ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=6.5, Angle=361, KBG=45, FKB=0, BKB=65, Size=2.0, X=0.0, Y=0.0, Z=0.0, X2=0.0, Y2=0.0, Z2=-2.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_SPEED, SetWeight=false, ShieldDamage=-3, Trip=0.0, Rehit=0, Reflectable=true, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_OBJECT)
		}
    });
}	
#[acmd_script(
    agent = "miiswordsman_tornadoshot",
    script =  "game_fly",
    category = ACMD_GAME,
	low_priority)]
unsafe fn sword_nadoshot(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		if(is_excute){
			AREA_WIND_2ND_RAD_arg9(0, 2, 0.05, 200, 1, 3, 3, 25, 30)
			ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=13.0, Angle=86, KBG=100, FKB=150, BKB=0, Size=5.0, X=0.0, Y=11.0, Z=1.2, X2=0.0, Y2=0.0, Z2=0.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_SPEED, SetWeight=false, ShieldDamage=3.5, Trip=0.0, Rehit=0, Reflectable=true, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_ENERGY)
		}
		frame(Frame=18)
		if(is_excute){
			ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=11.0, Angle=86, KBG=100, FKB=150, BKB=0, Size=5.0, X=0.0, Y=11.0, Z=1.2, X2=0.0, Y2=0.0, Z2=0.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_SPEED, SetWeight=false, ShieldDamage=2.5, Trip=0.0, Rehit=0, Reflectable=true, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_ENERGY)
		}
		frame(Frame=36)
		if(is_excute){
			ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=10.0, Angle=86, KBG=100, FKB=150, BKB=0, Size=5.0, X=0.0, Y=11.0, Z=1.2, X2=0.0, Y2=0.0, Z2=0.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_SPEED, SetWeight=false, ShieldDamage=2, Trip=0.0, Rehit=0, Reflectable=true, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_ENERGY)
		}
		frame(Frame=54)
		if(is_excute){
			ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=9.0, Angle=86, KBG=100, FKB=150, BKB=0, Size=5.0, X=0.0, Y=11.0, Z=1.2, X2=0.0, Y2=0.0, Z2=0.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_SPEED, SetWeight=false, ShieldDamage=1.5, Trip=0.0, Rehit=0, Reflectable=true, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_ENERGY)
		}
    });
}	
#[acmd_script(
    agent = "miiswordsman",
    script =  "game_specialhi3start",
    category = ACMD_GAME,
	low_priority)]
unsafe fn sword_hs_start(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		frame(Frame=1)
		FT_MOTION_RATE(FSM=0.25)
    });
}	
#[acmd_script(
    agent = "miiswordsman",
    script =  "game_throwhi",
    category = ACMD_GAME,
	low_priority)]
unsafe fn sword_uthrow(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		if(is_excute){
			ATTACK_ABS(Kind=FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, ID=0, Damage=3.0, Angle=80, KBG=100, FKB=0, BKB=100, Hitlag=0.0, Unk=1.0, FacingRestrict=ATTACK_LR_CHECK_F, Unk=0.0, Unk=true, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_NONE, Type=ATTACK_REGION_THROW)
			ATTACK_ABS(Kind=FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, ID=0, Damage=3.0, Angle=361, KBG=100, FKB=0, BKB=60, Hitlag=0.0, Unk=1.0, FacingRestrict=ATTACK_LR_CHECK_F, Unk=0.0, Unk=true, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_NONE, Type=ATTACK_REGION_THROW)
		}
		frame(Frame=13)
		if(is_excute){
			ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=2.0, Angle=361, KBG=100, FKB=0, BKB=30, Size=6.0, X=0.0, Y=21.0, Z=0.0, X2=0.0, Y2=13.0, Z2=0.0, Hitlag=1.0, SDI=0.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
			AttackModule::set_catch_only_all(true, false)
			CHECK_FINISH_CAMERA(0, 33)
		}
		frame(Frame=16)
		if(is_excute){
			AttackModule::clear_all()
			ATK_HIT_ABS(FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, hash40("throw"), WorkModule::get_int64(module_accessor,*FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT), WorkModule::get_int64(module_accessor,*FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP), WorkModule::get_int64(module_accessor,*FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO))
		}
    });
}
#[acmd_script(
    agent = "miiswordsman",
    script =  "sound_throwhi",
    category = ACMD_SOUND,
	low_priority)]
unsafe fn sword_uthrow_snd(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		frame(Frame=2)
		if(is_excute){
			PLAY_SE(hash40("se_common_throw_02"))
			PLAY_SEQUENCE(hash40("seq_miiswordsman_rnd_attack02"))
		}
    });
}
#[acmd_script(
    agent = "miiswordsman",
    script =  "expression_throwhi",
    category = ACMD_EXPRESSION,
	low_priority)]
unsafe fn sword_uthrow_expr(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		if(is_excute){
			FT_ATTACK_ABS_CAMERA_QUAKE(FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, CAMERA_QUAKE_KIND_NONE)
		}
		frame(Frame=13)
		if(is_excute){
			QUAKE(CAMERA_QUAKE_KIND_M)
		}
    });
}
#[acmd_script(
    agent = "miiswordsman",
    script =  "effect_throwhi",
    category = ACMD_EFFECT,
	low_priority)]
unsafe fn sword_uthrow_eff(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		frame(Frame=13)
		if(is_excute){
			EFFECT(hash40("sys_attack_speedline"), hash40("top"), 0, 7, -1, -90, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, true)
		}
		frame(Frame=16)
		if(is_excute){
			EFFECT(hash40("sys_smash_flash_s"), hash40("top"), 0, 28, -6, 0, 0, 0, 1.5, 0, 0, 0, 0, 0, 0, true)
		}
		frame(Frame=37)
		if(is_excute){
			LANDING_EFFECT(hash40("sys_landing_smoke"), hash40("top"), 0, 0, 0, 0, 0, 0, 0.7, 0, 0, 0, 0, 0, 0, false)
		}
    });
}
#[acmd_script(
    agent = "miiswordsman",
    scripts =  ["game_specialn1", "game_specialairn1"],
    category = ACMD_GAME,
	low_priority)]
unsafe fn sword_nado(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		frame(Frame=1)
		FT_MOTION_RATE(FSM=0.7058823529411765)
		frame(Frame=17)
		if(is_excute){
			ArticleModule::generate_article(FIGHTER_MIISWORDSMAN_GENERATE_ARTICLE_TORNADOSHOT, false, 0)
		}
		frame(Frame=18)
		FT_MOTION_RATE(FSM=1)
    });
}	
#[acmd_script(
    agent = "miiswordsman",
    scripts =  ["game_attackhi4"],
    category = ACMD_GAME,
	low_priority)]
unsafe fn sword_usmash(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		frame(Frame=5)
		if(is_excute){
			WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD)
		}
		frame(Frame=8)
		if(is_excute){
			ATTACK(ID=0, Part=0, Bone=hash40("haver"), Damage=14.0, Angle=85, KBG=97, FKB=0, BKB=35, Size=5.5, X=0.0, Y=12.0, Z=-1.2, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
			ATTACK(ID=1, Part=0, Bone=hash40("haver"), Damage=14.0, Angle=85, KBG=97, FKB=0, BKB=35, Size=5.5, X=0.0, Y=4.0, Z=-1.2, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
			ATTACK(ID=2, Part=0, Bone=hash40("shoulderr"), Damage=14.0, Angle=85, KBG=97, FKB=0, BKB=35, Size=5.5, X=1.5, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
			ATTACK(ID=3, Part=0, Bone=hash40("top"), Damage=14.0, Angle=85, KBG=97, FKB=0, BKB=35, Size=3.5, X=0.0, Y=3.0, Z=12.5, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
		}
		wait(Frames=1)
		if(is_excute){
			AttackModule::clear(ID=3, false)
		}
		frame(Frame=13)
		if(is_excute){
			AttackModule::clear_all()
		}
    });
}	
#[acmd_script(
    agent = "miiswordsman",
    scripts =  ["sound_attackhi4"],
    category = ACMD_SOUND,
	low_priority)]
unsafe fn sword_usmash_snd(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		frame(Frame=6)
		if(is_excute){
			STOP_SE(hash40("se_common_smash_start_02"))
		}
		frame(Frame=7)
		if(is_excute){
			PLAY_SEQUENCE(hash40("seq_miiswordsman_rnd_attack03"))
			PLAY_SE(hash40("se_common_smashswing_03"))
			PLAY_SE(hash40("se_miiswordsman_swing_l"))
		}
		frame(Frame=25)
		if(is_excute){
			PLAY_LANDING_SE(hash40("se_miiswordsman_landing03"))
		}
    });
}		
#[acmd_script(
    agent = "miiswordsman",
    scripts =  ["effect_attackhi4"],
    category = ACMD_EFFECT,
	low_priority)]
unsafe fn sword_usmash_eff(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		frame(Frame=2)
		if(is_excute){
			EFFECT(hash40("sys_smash_flash"), hash40("top"), 7, 4.5, -4, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true)
		}
		frame(Frame=8)
		if(is_excute){
			AFTER_IMAGE4_ON_WORK_arg29(WorkModule::get_int64(fighter.module_accessor, *FIGHTER_MIISWORDSMAN_INSTANCE_WORK_ID_INT_EFT_TEX_SWORD), WorkModule::get_int64(fighter.module_accessor, *FIGHTER_MIISWORDSMAN_INSTANCE_WORK_ID_INT_EFT_TEX_SWORD_ADD), 5, hash40("haver"), 0, 0.2, 0, hash40("haver"), -0.0, 10.8, 0, true, WorkModule::get_int64(fighter.module_accessor, *FIGHTER_MIISWORDSMAN_INSTANCE_WORK_ID_INT_EFT_ID_SWORD_FLARE), hash40("haver"), 0, 0, 0, 0, 0, 0, 1, 0, EFFECT_AXIS_X, 0, TRAIL_BLEND_ALPHA, 101, TRAIL_CULL_NONE, 1.4, 0.1)
		}
		frame(Frame=13)
		if(is_excute){
			AFTER_IMAGE_OFF(3)
		}
		frame(Frame=25)
		if(is_excute){
			LANDING_EFFECT(hash40("sys_down_smoke"), hash40("top"), 0, 0, 0, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, false)
		}
    });
}	
#[acmd_script(
    agent = "miiswordsman",
    scripts =  ["game_speciallw1hit", "game_specialairlw1hit"],
    category = ACMD_GAME,
	low_priority)]
unsafe fn sword_counter(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
			FT_MOTION_RATE(FSM=0.4)
			if(is_excute){
				WorkModule::on_flag(Flag=FIGHTER_MIISWORDSMAN_STATUS_COUNTER_FLAG_GRAVITY_OFF)
			}
			frame(Frame=10)
			FT_MOTION_RATE(FSM=1.0)
			frame(Frame=21)
			if(is_excute){
				ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=8.0, Angle=80, KBG=60, FKB=0, BKB=85, Size=8.8, X=0.0, Y=8.0, Z=15.0, X2=0.0, Y2=8.0, Z2=3.0, Hitlag=2.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=hash40("no"), Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
				AttackModule::set_force_reaction(0, true, false)
				rust {
					let ENTRY_ID = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
					COUNTER_STORE[ENTRY_ID] = false;
				}
			}
			frame(Frame=23)
			if(is_excute){
				AttackModule::clear_all()
				WorkModule::on_flag(Flag=FIGHTER_MIISWORDSMAN_STATUS_COUNTER_FLAG_GRAVITY_ON)
			}
			FT_MOTION_RATE(FSM=0.65)
    });
}		
#[acmd_script(
    agent = "miiswordsman",
    script =  "game_attacklw3",
    category = ACMD_GAME,
	low_priority)]
unsafe fn sword_dtilt(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
			frame(Frame=5)
			if(is_excute){
				ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=8.0, Angle=70, KBG=80, FKB=0, BKB=50, Size=2.4, X=0.0, Y=3.3, Z=13.0, X2=0.0, Y2=2.7, Z2=20.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.3, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_sting"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
				ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=8.0, Angle=70, KBG=80, FKB=0, BKB=50, Size=2.4, X=0.0, Y=3.6, Z=8.5, X2=0.0, Y2=3.6, Z2=9.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.3, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_sting"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
				AttackModule::set_attack_height_all(smash::app::AttackHeight(*ATTACK_HEIGHT_LOW), false)
			}
			wait(Frames=2)
			if(is_excute){
				AttackModule::clear_all()
			}
    });
}	
#[acmd_script(
    agent = "miiswordsman",
    script =  "game_attacks3",
    category = ACMD_GAME,
	low_priority)]
unsafe fn sword_ftilt(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
			frame(Frame=5)
			if(is_excute){
				ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=12.0, Angle=361, KBG=96, FKB=0, BKB=20, Size=2.7, X=0.0, Y=5.4, Z=4.0, X2=0.0, Y2=5.4, Z2=16.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
			}
			wait(Frames=3)
			if(is_excute){
				AttackModule::clear_all()
			}
			frame(Frame=25)
			if(is_excute){
				CancelModule::enable_cancel()
			}
    });
}	
#[acmd_script(
    agent = "miiswordsman",
    script =  "effect_attacks3",
    category = ACMD_EFFECT,
	low_priority)]
unsafe fn sword_ftilt_eff(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		frame(Frame=4)
		if(is_excute){
			AFTER_IMAGE4_ON_WORK_arg29(WorkModule::get_int64(fighter.module_accessor, *FIGHTER_MIISWORDSMAN_INSTANCE_WORK_ID_INT_EFT_TEX_SWORD), WorkModule::get_int64(fighter.module_accessor, *FIGHTER_MIISWORDSMAN_INSTANCE_WORK_ID_INT_EFT_TEX_SWORD_ADD), 5, hash40("haver"), 0, 0.2, 0, hash40("haver"), -0.0, 10.8, 0, true, WorkModule::get_int64(fighter.module_accessor, *FIGHTER_MIISWORDSMAN_INSTANCE_WORK_ID_INT_EFT_ID_SWORD_FLARE), hash40("haver"), 0, 0, 0, 0, 0, 0, 1, 0, EFFECT_AXIS_X, 0, TRAIL_BLEND_ALPHA, 101, TRAIL_CULL_NONE, 1.4, 0.1)
		}
		frame(Frame=6)
		if(is_excute){
			AFTER_IMAGE_OFF(3)
		}
    });
}	
#[acmd_script(
    agent = "miiswordsman",
    script =  "sound_attacks3",
    category = ACMD_SOUND,
	low_priority)]
unsafe fn sword_ftilt_snd(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		frame(Frame=4)
		if(is_excute){
			PLAY_SEQUENCE(hash40("seq_miiswordsman_rnd_attack02"))
			PLAY_SE(hash40("se_miiswordsman_swing_m"))
		}
    });
}
#[acmd_script(
    agent = "miiswordsman",
    scripts =  ["game_specialhi1", "game_specialairhi1"],
    category = ACMD_GAME,
	low_priority)]
unsafe fn sword_ss_rise(fighter: &mut L2CAgentBase) {
		let lua_state = fighter.lua_state_agent;
		let module_accessor = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
		frame(fighter.lua_state_agent, 1.0);
		if macros::is_excute(fighter) {
			let speed = smash::phx::Vector3f { x: 0.0, y: 0.65, z: 0.0 };
			KineticModule::add_speed(module_accessor, &speed);
			acmd!(lua_state, {
				ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=4.0, Angle=92, KBG=100, FKB=96, BKB=0, Size=4.0, X=0.0, Y=7.0, Z=6.0, X2=0.0, Y2=4.0, Z2=6.0, Hitlag=1.3, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
				ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=4.0, Angle=92, KBG=100, FKB=96, BKB=0, Size=3.5, X=0.0, Y=7.0, Z=13.0, X2=0.0, Y2=4.0, Z2=13.0, Hitlag=1.3, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
				AttackModule::set_add_reaction_frame(ID=0, Frames=10.0, Unk=false)
				AttackModule::set_add_reaction_frame(ID=1, Frames=10.0, Unk=false)
			});
		}
		frame(fighter.lua_state_agent, 3.0);
		if macros::is_excute(fighter) {
			acmd!(lua_state, {
				ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=4.0, Angle=92, KBG=100, FKB=60, BKB=0, Size=4.0, X=0.0, Y=7.0, Z=6.0, X2=0.0, Y2=4.0, Z2=6.0, Hitlag=1.3, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
				ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=4.0, Angle=92, KBG=100, FKB=60, BKB=0, Size=3.5, X=0.0, Y=7.0, Z=13.0, X2=0.0, Y2=4.0, Z2=13.0, Hitlag=1.3, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
				AttackModule::set_add_reaction_frame(ID=0, Frames=10.0, Unk=false)
				AttackModule::set_add_reaction_frame(ID=1, Frames=10.0, Unk=false)
			});
		}
		frame(fighter.lua_state_agent, 4.0);
		if macros::is_excute(fighter) {
			notify_event_msc_cmd!(fighter, 0x2127e37c07u64, *GROUND_CLIFF_CHECK_KIND_ALWAYS);
		}
		frame(fighter.lua_state_agent, 6.0);
		if macros::is_excute(fighter) {
			AttackModule::clear_all(fighter.module_accessor);
		}
		frame(fighter.lua_state_agent, 30.0);
		if macros::is_excute(fighter) {
			acmd!(lua_state, {
				ATTACK(ID=0, Part=0, Bone=hash40("haver"), Damage=3.0, Angle=268, KBG=180, FKB=39, BKB=0, Size=5.0, X=0.0, Y=16.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=0.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_A, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
				ATTACK(ID=1, Part=0, Bone=hash40("haver"), Damage=3.0, Angle=361, KBG=100, FKB=20, BKB=0, Size=5.0, X=0.0, Y=16.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=0.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_G, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
			});
		}
		frame(fighter.lua_state_agent, 32.0);
		if macros::is_excute(fighter) {
			acmd!(lua_state, {
				WorkModule::on_flag(Flag=FIGHTER_MIISWORDSMAN_STATUS_WORK_ID_FLAG_ROKET_UNDER_DISABLE_CONTROL_X)
			});
		}
		frame(fighter.lua_state_agent, 45.0);
		if macros::is_excute(fighter) {
			AttackModule::clear_all(fighter.module_accessor);
		}
}	
#[acmd_script(
    agent = "miiswordsman",
    scripts =  ["game_specialairn3start", "game_specialn3start"],
    category = ACMD_GAME,
	low_priority)]
unsafe fn sword_airgrab_start(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
			FT_MOTION_RATE(FSM=1.5)
			frame(Frame=6)
			FT_MOTION_RATE(FSM=1)
			if(is_excute){
				SET_SPEED_EX(-0.0, 0.0, KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN)
				ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=1.0, Angle=368, KBG=100, FKB=20, BKB=0, Size=4.0, X=0.0, Y=6.0, Z=13.0, X2=0.0, Y2=6.0, Z2=6.0, Hitlag=0.0, SDI=0.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=hash40("no"), Trip=-1.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=true, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_FIGHTER, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_fire"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_FIRE, Type=ATTACK_REGION_NONE)
				AttackModule::set_attack_height_all(smash::app::AttackHeight(*ATTACK_HEIGHT_HIGH), false)
				rust {
					let hit = smash::phx::Vector2f { x: 8.0, y: 0.0 };
					AttackModule::set_vec_target_pos(fighter.module_accessor, 0, Hash40::new("top"), &hit, 0, false);
				}
			}
			wait(Frames=2)
			if(is_excute){
				AttackModule::clear_all()
			}
			frame(Frame=10)
			FT_MOTION_RATE(FSM=1.2333)
    });
}	
#[acmd_script(
    agent = "miiswordsman",
    scripts =  ["effect_specialairn3start", "effect_specialn3start"],
    category = ACMD_EFFECT,
	low_priority)]
unsafe fn sword_airgrab_start_eff(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		if(is_excute){
			EFFECT(hash40("sys_smash_flash"), hash40("havel"), -0.0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true)
		}
		wait(Frames=1)	
		for(10 Iterations){
			if(is_excute){
				EFFECT(hash40("sys_fireflower_shot"), hash40("havel"), -0.0, 0, 0, 0, 0, 0, 0.4, 0, 0, 0, 0, 0, 0, true)
			}
			wait(Frames=1)	
		}
    });
}	
#[acmd_script(
    agent = "miiswordsman",
    scripts =  ["effect_specialairn3loop", "effect_specialn3loop"],
    category = ACMD_EFFECT,
	low_priority)]
unsafe fn sword_airgrab_eff(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		for(50 Iterations){
			if(is_excute){
				EFFECT(hash40("sys_fireflower_shot"), hash40("havel"), -0.0, 0, 0, 0, 0, 0, 0.5, 0, 0, 0, 0, 0, 0, true)
			}
			wait(Frames=3)	
		}
    });
}	
#[acmd_script(
    agent = "miiswordsman",
    scripts =  ["effect_specialairn3end", "effect_specialn3end"],
    category = ACMD_EFFECT,
	low_priority)]
unsafe fn sword_airgrab_end_eff(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		if(is_excute){
			EFFECT(hash40("sys_damage_fire"), hash40("top"), 0, -1, 10, 0, 0, 0, 2.75, 0, 0, 0, 0, 0, 0, true)
			EFFECT_OFF_KIND(hash40("sys_fireflower_shot"), false, true)
		}
    });
}
#[acmd_script(
    agent = "miiswordsman",
    script =  "game_specialairn3loop",
    category = ACMD_GAME,
	low_priority)]
unsafe fn sword_airgrab(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
			if(is_excute){
				AttackModule::clear_all()
				SET_SPEED_EX(0.8, 1.5, KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN)
				ATTACK(ID=0, Part=0, Bone=hash40("havel"), Damage=0.2, Angle=368, KBG=100, FKB=70, BKB=0, Size=7.0, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.0, SDI=0.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=-1.0, Rehit=1, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=true, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_nomal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_NONE, Type=ATTACK_REGION_NONE)
				ATTACK(ID=0, Part=0, Bone=hash40("havel"), Damage=0.2, Angle=60, KBG=100, FKB=30, BKB=0, Size=7.0, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.0, SDI=0.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=-1.0, Rehit=1, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=true, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_nomal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_NONE, Type=ATTACK_REGION_NONE)
				AttackModule::set_attack_height_all(smash::app::AttackHeight(*ATTACK_HEIGHT_HIGH), false)
				rust {
					let hit = smash::phx::Vector2f { x: 8.0, y: 0.0 };
					AttackModule::set_vec_target_pos(fighter.module_accessor, 0, Hash40::new("top"), &hit, 0, false);
				}
			}
			frame(Frame=7)
			if(is_excute){
				SET_SPEED_EX(1.75, -2.625, KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN)
			}
    });
}	
#[acmd_script(
    agent = "miiswordsman",
    script =  "game_specialairn3end",
    category = ACMD_GAME,
	low_priority)]
unsafe fn sword_airgrab_end(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
			if(is_excute){
				SET_SPEED_EX(-0.8, 0.7, KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN)
				AttackModule::clear_all()
				ATTACK(ID=0, Part=0, Bone=hash40("havel"), Damage=10.0, Angle=50, KBG=80, FKB=0, BKB=40, Size=9.0, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=0.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=-1.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_fire"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_FIRE, Type=ATTACK_REGION_NONE)
			}
			wait(Frames=6)
			if(is_excute){
				AttackModule::clear_all()
			}
    });
}	
#[acmd_script(
    agent = "miiswordsman",
    script =  "sound_specialairn3end",
    category = ACMD_SOUND,
	low_priority)]
unsafe fn sword_airgrab_end_snd(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
			if(is_excute){
				PLAY_SEQUENCE(hash40("seq_miiswordsman_rnd_attack03"))
				PLAY_SE(hash40("se_miiswordsman_special_c3_n02"))
			}
    });
}
#[acmd_script(
    agent = "miiswordsman",
    script =  "expression_specialairn3end",
    category = ACMD_EXPRESSION,
	low_priority)]
unsafe fn sword_airgrab_end_expr(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		if(is_excute){
			QUAKE(CAMERA_QUAKE_KIND_M)
		}
    });
}
#[fighter_frame_callback]
pub fn sword(fighter : &mut L2CFighterCommon) {
    unsafe {
        let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);  
		let fighter_kind = smash::app::utility::get_kind(boma);
		let status_kind = smash::app::lua_bind::StatusModule::status_kind(boma);
		let ENTRY_ID = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
		let frame = MotionModule::frame(boma);
		if fighter_kind == *FIGHTER_KIND_MIISWORDSMAN {
			if [*FIGHTER_STATUS_KIND_DEAD, *FIGHTER_STATUS_KIND_LOSE, *FIGHTER_STATUS_KIND_WIN].contains(&status_kind) || smash::app::sv_information::is_ready_go() == false{
				COUNTER_STORE[ENTRY_ID] = false;
				BOMB_TIME[ENTRY_ID] = 0;
				CUSTOM_BOMB[ENTRY_ID] = false;
			};
			/*if [*FIGHTER_STATUS_KIND_ATTACK_AIR, *FIGHTER_STATUS_KIND_ATTACK_LW3].contains(&status_kind) && CUSTOM_BOMB[ENTRY_ID] == true {
				MotionModule::change_motion(boma, smash::phx::Hash40::new("catch"), 9.0, 1.0, false, 0.0, false, false);
				MotionModule::set_rate(boma, 0.8);
				CUSTOM_BOMB[ENTRY_ID] = false;
			};
			if [hash40("special_n3_start"), hash40("special_air_n3_start")].contains(&MotionModule::motion_kind(boma)){
				if BOMB_TIME[ENTRY_ID] == 0 {
					ItemModule::have_item(fighter.module_accessor, ItemKind(*ITEM_KIND_BOMBCHU), 0, 0, false, false);
					if StatusModule::situation_kind(boma) == *SITUATION_KIND_AIR {
						StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_AIR, false);
						CUSTOM_BOMB[ENTRY_ID] = true;
					} else {
						StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_LW3, false);
						CUSTOM_BOMB[ENTRY_ID] = true;
					};
					BOMB_TIME[ENTRY_ID] = 120;
				} else {
					if StatusModule::situation_kind(boma) == *SITUATION_KIND_AIR {
						StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_AIR, false);
						CUSTOM_BOMB[ENTRY_ID] = true;
					} else {
						StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_LW3, false);
						CUSTOM_BOMB[ENTRY_ID] = true;
					};
				};
			};
			if [hash40("special_n3_end"), hash40("special_air_n3_end")].contains(&MotionModule::motion_kind(boma)){
				if MotionModule::frame(boma) < 6.0 {
					MotionModule::set_rate(boma, 4.0);
				} else {
					MotionModule::set_rate(boma, 1.0);
				};
			};*/
			if [hash40("special_air_n3_start")].contains(&MotionModule::motion_kind(boma)){
				if frame >= 36.0 {
					StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_FALL, false);
				};
				if AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT) {
					MotionModule::change_motion(boma, smash::phx::Hash40::new("special_air_n3_loop"), 0.0, 1.0, false, 0.0, false, false);
				};
			};
			if [hash40("special_n3_start")].contains(&MotionModule::motion_kind(boma)){
				if frame >= 40.0 {
					StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_WAIT, false);
				};
				if AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT) {
					StatusModule::set_keep_situation_air(boma, true);
					StatusModule::set_situation_kind(boma, smash::app::SituationKind(*SITUATION_KIND_AIR), true);
					MotionModule::change_motion(boma, smash::phx::Hash40::new("special_air_n3_loop"), 3.0, 1.0, false, 0.0, false, false);
				};
			};
			if [hash40("special_air_n3_loop")].contains(&MotionModule::motion_kind(boma)){
				StatusModule::set_keep_situation_air(boma, true);
				if (GroundModule::ray_check(boma, &Vector2f{ x: PostureModule::pos_x(boma), y: PostureModule::pos_y(boma)}, &Vector2f{ x: 0.0, y: -1.5}, true) == 1 && frame > 10.0) {
					StatusModule::change_status_request_from_script(boma, *FIGHTER_MIISWORDSMAN_STATUS_KIND_SPECIAL_N3_END, false);
				};
				if frame > 30.0 {
					StatusModule::change_status_request_from_script(boma, *FIGHTER_MIISWORDSMAN_STATUS_KIND_SPECIAL_N3_END, false);
				};
			};
			if [hash40("special_air_n3_end")].contains(&MotionModule::motion_kind(boma)){
				StatusModule::set_keep_situation_air(boma, true);
				if KineticModule::get_kinetic_type(boma) != *FIGHTER_KINETIC_TYPE_JUMP {
					KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_JUMP);
				};
				if frame >= 22.0 {
					StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_FALL, false);
				};
			};
			if BOMB_TIME[ENTRY_ID] > 0 {
				BOMB_TIME[ENTRY_ID] -= 1;
			};
			if [hash40("special_s1_hit"), hash40("special_air_s1_hit")].contains(&MotionModule::motion_kind(boma)) {
				MotionModule::set_rate(boma, 1.42857142);
			};
			if status_kind == *FIGHTER_MIISWORDSMAN_STATUS_KIND_SPECIAL_LW1_HIT && COUNTER_STORE[ENTRY_ID] == false {
				if MotionModule::frame(boma) < 12.0 && ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_GUARD) {
					if StatusModule::situation_kind(boma) == *SITUATION_KIND_AIR {
						StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_FALL_AERIAL, false);
					} else {
						StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_GUARD_OFF, false);
					};
					COUNTER_STORE[ENTRY_ID] = true;
				};
			};
			if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_LW && COUNTER_STORE[ENTRY_ID] == true{
				StatusModule::change_status_request_from_script(boma, *FIGHTER_MIISWORDSMAN_STATUS_KIND_SPECIAL_LW1_HIT, false);
			};
		};
    };
}	
	
pub fn install() {
    smashline::install_acmd_scripts!(
		sword_grab,
		sword_hs_start,
		sword_nadoshot,
		sword_shuriken,
		sword_ss_rise,
		sword_nado,
		sword_counter,
		sword_usmash,
		sword_usmash_snd,
		sword_usmash_eff,
		sword_uthrow,
		sword_uthrow_snd,
		sword_uthrow_eff,
		sword_uthrow_expr,
		sword_ftilt,
		sword_ftilt_eff,
		sword_ftilt_snd,
		sword_dtilt,
		sword_airgrab,
		sword_airgrab_start,
		sword_airgrab_end,
		sword_airgrab_eff,
		sword_airgrab_end_eff,
		sword_airgrab_start_eff,
		sword_airgrab_end_snd,
		sword_airgrab_end_expr
    );
	smashline::install_agent_frame_callbacks!(sword);
}
