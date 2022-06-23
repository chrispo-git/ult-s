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
    category = ACMD_GAME)]
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
    category = ACMD_GAME)]
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
    category = ACMD_GAME)]
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
    category = ACMD_GAME)]
unsafe fn sword_hs_start(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		frame(Frame=1)
		FT_MOTION_RATE(FSM=0.25)
    });
}	
#[acmd_script(
    agent = "miiswordsman",
    scripts =  ["game_specialn1", "game_specialairn1"],
    category = ACMD_GAME)]
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
    category = ACMD_GAME)]
unsafe fn sword_usmash(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		frame(Frame=1)
		FT_MOTION_RATE(FSM=0.3)
		frame(Frame=10)
		FT_MOTION_RATE(FSM=0.5)
		if(is_excute){
			WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD)
		}
		frame(Frame=16)
		FT_MOTION_RATE(FSM=1)
		frame(Frame=19)
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
		frame(Frame=22)
		if(is_excute){
			AttackModule::clear_all()
		}
    });
}	
#[acmd_script(
    agent = "miiswordsman",
    scripts =  ["sound_attackhi4"],
    category = ACMD_SOUND)]
unsafe fn sword_usmash_snd(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		frame(Frame=11)
		if(is_excute){
			STOP_SE(hash40("se_common_smash_start_02"))
		}
		frame(Frame=16)
		if(is_excute){
			PLAY_SEQUENCE(hash40("seq_miiswordsman_rnd_attack03"))
		}
		wait(Frames=2)
		if(is_excute){
			PLAY_SE(hash40("se_common_smashswing_03"))
			PLAY_SE(hash40("se_miiswordsman_swing_l"))
		}
		frame(Frame=34)
		if(is_excute){
			PLAY_LANDING_SE(hash40("se_miiswordsman_landing03"))
		}
    });
}		
#[acmd_script(
    agent = "miiswordsman",
    scripts =  ["game_speciallw1hit", "game_specialairlw1hit"],
    category = ACMD_GAME)]
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
    scripts =  ["game_specialhi1", "game_specialairhi1"],
    category = ACMD_GAME)]
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
#[fighter_frame_callback]
pub fn sword(fighter : &mut L2CFighterCommon) {
    unsafe {
        let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);  
		let fighter_kind = smash::app::utility::get_kind(boma);
		let status_kind = smash::app::lua_bind::StatusModule::status_kind(boma);
		let ENTRY_ID = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
		if fighter_kind == *FIGHTER_KIND_MIISWORDSMAN {
			if [*FIGHTER_STATUS_KIND_DEAD, *FIGHTER_STATUS_KIND_LOSE, *FIGHTER_STATUS_KIND_WIN].contains(&status_kind) || smash::app::sv_information::is_ready_go() == false{
				COUNTER_STORE[ENTRY_ID] = false;
				BOMB_TIME[ENTRY_ID] = 0;
				CUSTOM_BOMB[ENTRY_ID] = false;
			};
			if [*FIGHTER_STATUS_KIND_ATTACK_AIR, *FIGHTER_STATUS_KIND_ATTACK_LW3].contains(&status_kind) && CUSTOM_BOMB[ENTRY_ID] == true {
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
		sword_usmash_snd
    );
	smashline::install_agent_frame_callbacks!(sword);
}
