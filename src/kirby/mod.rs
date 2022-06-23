use smash::hash40;
use smash::app::lua_bind::*;
use smash::lib::lua_const::*;
use smash::lua2cpp::*;
use smash::lua2cpp::*;
use smashline::*;
use smash_script::*;
use crate::util::*;

static mut UPB_ANGLE : [i32; 8] = [1; 8];
//0 - Inwards
//1 - Middle
//2 - Outwards
static mut IS_FINAL : [bool; 8] = [false; 8];
#[acmd_script(
    agent = "kirby",
    script =  "game_attacklw3",
    category = ACMD_GAME)]
unsafe fn kirby_dtilt(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		frame(Frame=4)
		if(is_excute){
			ATTACK(ID=0, Part=0, Bone=hash40("toer"), Damage=6.0, Angle=361, KBG=80, FKB=0, BKB=30, Size=3.7, X=4.3, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=1.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
			ATTACK(ID=1, Part=0, Bone=hash40("toer"), Damage=6.0, Angle=361, KBG=80, FKB=0, BKB=30, Size=3.7, X=-2.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=1.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
		}
		wait(Frames=3)
		if(is_excute){
			AttackModule::clear_all()
		}
    });
}
#[acmd_script(
    agent = "kirby",
    script =  "game_attackairf",
    category = ACMD_GAME)]
unsafe fn kirby_fair(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		FT_MOTION_RATE(FSM=0.7)
		frame(Frame=10)
		FT_MOTION_RATE(FSM=1)
		if(is_excute){
			WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
			ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=4.0, Angle=53, KBG=39, FKB=0, BKB=43, Size=5.0, X=0.0, Y=4.0, Z=6.5, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.8, SDI=0.5, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
			ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=4.0, Angle=84, KBG=38, FKB=0, BKB=35, Size=4.4, X=0.0, Y=3.5, Z=13.0, X2=0.0, Y2=4.2, Z2=6.5, Hitlag=0.8, SDI=0.5, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
			AttackModule::set_add_reaction_frame(ID=0, Frames=5.0, Unk=false)
			AttackModule::set_add_reaction_frame(ID=1, Frames=5.0, Unk=false)
		}
		wait(Frames=2)
		if(is_excute){
			AttackModule::clear_all()
		}
		frame(Frame=17)
		if(is_excute){
			ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=4.0, Angle=367, KBG=30, FKB=0, BKB=28, Size=4.5, X=0.0, Y=2.8, Z=13.0, X2=0.0, Y2=4.1, Z2=7.0, Hitlag=0.8, SDI=0.5, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
			AttackModule::set_add_reaction_frame(ID=0, Frames=5.0, Unk=false)
		}
		wait(Frames=2)
		if(is_excute){
			AttackModule::clear_all()
		}
		frame(Frame=25)
		if(is_excute){
			ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=6.0, Angle=361, KBG=148, FKB=0, BKB=24, Size=5.1, X=0.0, Y=3.0, Z=13.0, X2=0.0, Y2=4.2, Z2=7.0, Hitlag=1.2, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
		}
		wait(Frames=3)
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
    agent = "kirby",
    scripts =  ["game_specialhi", "game_specialairhi"],
    category = ACMD_GAME)]
unsafe fn kirby_upb1(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		frame(Frame=5)
		if(is_excute){
			rust {
				let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent); 
				StatusModule::change_status_request_from_script(boma, *FIGHTER_KIRBY_STATUS_KIND_SPECIAL_HI2, true);
			}
		}
    });
}	
#[acmd_script(
    agent = "kirby",
    scripts =  ["sound_specialhi4"],
    category = ACMD_SOUND)]
unsafe fn kirby_upb4_sound(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
}		
#[acmd_script(
    agent = "kirby",
    scripts =  ["expression_specialhi4"],
    category = ACMD_EXPRESSION)]
unsafe fn kirby_upb4_exp(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
}		
#[acmd_script(
    agent = "kirby",
    script =  "game_specialhi4",
    category = ACMD_GAME)]
unsafe fn kirby_upb4(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
    });
}	
#[acmd_script(
    agent = "kirby",
    script =  "effect_specialairhi2",
    category = ACMD_EFFECT)]
unsafe fn kirby_air_upb2_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		if(is_excute){
			EFFECT_FOLLOW(hash40("sys_damage_fire"), hash40("haver"), 0, 3, 0.3, 0, 0, 0, 1, true)
			EFFECT_FOLLOW(hash40("sys_damage_elec"), hash40("haver"), 0, 3, 0.3, 0, 0, 0, 1, true)
		}
		frame(Frame=2)
		if(is_excute){
			EFFECT_FOLLOW_FLIP(hash40("kirby_attack_arc"), hash40("kirby_attack_arc"), hash40("top"), -20.0, 6, 0, 0, 0, 90, 1.0, true, EF_FLIP_NONE)
			LAST_EFFECT_SET_COLOR(2.016, 0.648, 1.536)
			LAST_EFFECT_SET_RATE(0.275)
		}
		frame(Frame=3)
		for(3 Iterations){
			if(is_excute){
				EFFECT_FOLLOW(hash40("sys_damage_elec"), hash40("haver"), 0, 3, 0.3, 0, 0, 0, 1, true)
				EFFECT_FOLLOW(hash40("sys_damage_fire"), hash40("haver"), 0, 3, 0.3, 0, 0, 0, 1, true)
				rust {
					let boma = fighter.module_accessor;
					let distance2 = smash::phx::Vector3f { x: 0.0, y: 6.0, z: 7.0 };
					let empty = smash::phx::Vector3f { x: 0.0, y: 0.0, z: 0.0 };
					let fire: u32 = EffectModule::req_follow(boma, smash::phx::Hash40::new("sys_fireflower_shot"), smash::phx::Hash40::new("top"), &distance2, &empty, 0.65, true, 0, 0, 0, 0, 0, true, true) as u32;
					EffectModule::set_rgb(boma, fire, 1.0, 0.25, 0.25);
					EffectModule::set_alpha(boma, fire, 0.5);
					EffectModule::set_rate(boma, fire, 0.5);
					let fire2: u32 = EffectModule::req_follow(boma, smash::phx::Hash40::new("sys_fireflower_shot"), smash::phx::Hash40::new("top"), &distance2, &empty, 0.45, true, 0, 0, 0, 0, 0, true, true) as u32;
					EffectModule::set_rgb(boma, fire2, 3.0, 0.5, 0.5);
					EffectModule::set_rate(boma, fire2, 0.5);
					EffectModule::set_rate(boma, fire, 0.75);
				}
			}
			wait(Frames=2)
		}
		frame(Frame=9)
		if(is_excute){
			EFFECT_OFF_KIND(hash40("sys_damage_fire"), false, true)
			EFFECT_OFF_KIND(hash40("sys_damage_elec"), false, true)
			EFFECT_OFF_KIND(hash40("sys_fireflower_shot"), false, true)
			EFFECT_OFF_KIND(hash40("kirby_attack_arc"), false, true)
		}
    });
}
		
#[acmd_script(
    agent = "kirby",
    script =  "game_specialairhi2",
    category = ACMD_GAME)]
unsafe fn kirby_air_upb2(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		frame(Frame=1)
		if(is_excute){
			FT_MOTION_RATE(FSM=2.0)
			ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=0.8, Angle=90, KBG=100, FKB=60, BKB=0, Size=4.5, X=0.0, Y=3.5, Z=4.9, X2=0.0, Y2=3.5, Z2=7.9, Hitlag=0.7, SDI=0.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=2, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_rush"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_PUNCH)
			ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=0.8, Angle=90, KBG=100, FKB=60, BKB=0, Size=4.5, X=0.0, Y=7.5, Z=4.9, X2=0.0, Y2=7.5, Z2=7.9, Hitlag=0.7, SDI=0.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=2, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_rush"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_PUNCH)
		}
		frame(Frame=2)
		if(is_excute){
			ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=0.8, Angle=367, KBG=100, FKB=50, BKB=0, Size=4.5, X=0.0, Y=3.5, Z=4.9, X2=0.0, Y2=3.5, Z2=7.9, Hitlag=0.5, SDI=0.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=2, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_rush"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_PUNCH)
			ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=0.8, Angle=367, KBG=100, FKB=50, BKB=0, Size=4.5, X=0.0, Y=7.5, Z=4.9, X2=0.0, Y2=7.5, Z2=7.9, Hitlag=0.5, SDI=0.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=2, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_rush"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_PUNCH)
		}
		frame(Frame=7)
		if(is_excute){
			AttackModule::clear_all()
			ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=9.0, Angle=83, KBG=82, FKB=0, BKB=74, Size=8.5, X=0.0, Y=-1.5, Z=4.9, X2=0.0, Y2=4.8, Z2=4.9, Hitlag=2.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_fire"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_FIRE, Type=ATTACK_REGION_PUNCH)
		}
		frame(Frame=10)
		if(is_excute){
			AttackModule::clear_all()
			rust {
				notify_event_msc_cmd!(fighter, 0x2127e37c07u64, *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
			}
		}
    });
}		
#[acmd_script(
    agent = "kirby",
    script =  "game_throwlw",
    category = ACMD_GAME)]
unsafe fn kirby_dthrow(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		if(is_excute){
			ATTACK_ABS(Kind=FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, ID=0, Damage=4.0, Angle=75, KBG=90, FKB=0, BKB=70, Hitlag=0.0, Unk=1.0, FacingRestrict=ATTACK_LR_CHECK_F, Unk=0.0, Unk=true, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_NONE, Type=ATTACK_REGION_THROW)
			ATTACK_ABS(Kind=FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, ID=0, Damage=3.0, Angle=361, KBG=100, FKB=0, BKB=60, Hitlag=0.0, Unk=1.0, FacingRestrict=ATTACK_LR_CHECK_F, Unk=0.0, Unk=true, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_NONE, Type=ATTACK_REGION_THROW)
		}
		frame(Frame=9)
		for(9 Iterations){
			if(is_excute){
				ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=1.0, Angle=270, KBG=100, FKB=10, BKB=0, Size=5.8, X=0.0, Y=4.0, Z=3.2, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_rush"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
				AttackModule::set_catch_only_all(true, false)
			}
			wait(Frames=2)
			if(is_excute){
				AttackModule::clear_all()
			}
			wait(Frames=2)
		}
		frame(Frame=56)
		if(is_excute){
			ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=1.0, Angle=90, KBG=100, FKB=0, BKB=10, Size=5.8, X=0.0, Y=4.0, Z=2.4, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
			AttackModule::set_catch_only_all(true, false)
		}
		frame(Frame=58)
		if(is_excute){
			ATK_HIT_ABS(FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, hash40("throw"), WorkModule::get_int64(module_accessor,*FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT), WorkModule::get_int64(module_accessor,*FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP), WorkModule::get_int64(module_accessor,*FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO))
			AttackModule::clear_all()
		}
		frame(Frame=73)
		if(is_excute){
			CancelModule::enable_cancel()
		}
    });
}			
#[acmd_script(
    agent = "kirby",
    script =  "sound_specialairhi2",
    category = ACMD_SOUND)]
unsafe fn kirby_upb2_sound(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		frame(Frames=1)
		if(is_excute){
			PLAY_SE(hash40("vc_kirby_attack07"))
		}
    });
}			
#[acmd_script(
    agent = "kirby",
    script =  "game_attackairb",
    category = ACMD_GAME)]
unsafe fn kirby_bair(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {	
		frame(Frame=7)
		if(is_excute){
			ArticleModule::generate_article(FIGHTER_KIRBY_GENERATE_ARTICLE_FINALCUTTER, false, 0)
			ArticleModule::change_motion(FIGHTER_MASTER_GENERATE_ARTICLE_SWORD,smash::phx::Hash40::new("special_hi2"),false,0.0)
			WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
		}
		frame(Frame=10)
		if(is_excute){
			ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=12.0, Angle=361, KBG=85, FKB=0, BKB=40, Size=5.0, X=0.0, Y=5.5, Z=-9.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.25, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_B, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
			ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=12.0, Angle=361, KBG=85, FKB=0, BKB=40, Size=6.0, X=0.0, Y=5.5, Z=-12.5, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.25, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_B, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
			ATTACK(ID=2, Part=0, Bone=hash40("top"), Damage=12.0, Angle=361, KBG=85, FKB=0, BKB=40, Size=6.0, X=0.0, Y=5.5, Z=-16.5, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.25, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_B, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
			ATTACK(ID=3, Part=0, Bone=hash40("top"), Damage=12.0, Angle=361, KBG=85, FKB=0, BKB=40, Size=5.0, X=0.0, Y=5.5, Z=-2.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.25, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_B, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
		}
		wait(Frames=2)
		if(is_excute){
			AttackModule::clear_all()
		}
		frame(Frame=20)
		if(is_excute){
			ArticleModule::remove_exist(FIGHTER_KIRBY_GENERATE_ARTICLE_FINALCUTTER,smash::app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL))
		}
		frame(Frame=30)
		if(is_excute){
			WorkModule::off_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
		}
    });
}		
#[acmd_script(
    agent = "kirby",
    script =  "effect_attackairb",
    category = ACMD_EFFECT)]
unsafe fn kirby_bair_eff(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {	
		frame(Frames=10)
		if(is_excute){
			EFFECT_FOLLOW_FLIP(hash40("kirby_onigoroshi_wind"), hash40("kirby_onigoroshi_wind"), hash40("top"), 1, 0, 0, 13, 180, 180, 1.5, false, EF_FLIP_YZ)
			LAST_EFFECT_SET_COLOR(0.64, 1.0, 1.0)
		}
	});
}	
#[acmd_script(
    agent = "kirby",
    script =  "sound_attackairb",
    category = ACMD_SOUND)]
unsafe fn kirby_bair_snd(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {	
		frame(Frames=7)
		if(is_excute){
			PLAY_SE(hash40("se_kirby_swing_l"))
			PLAY_SEQUENCE(hash40("seq_kirby_rnd_attack"))
		}
	});
}	
#[acmd_script(
    agent = "kirby",
    script =  "game_landingairb",
    category = ACMD_GAME)]
unsafe fn kirby_landing_bair(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {	
		if(is_excute){
			ArticleModule::remove_exist(FIGHTER_KIRBY_GENERATE_ARTICLE_FINALCUTTER,smash::app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL))
		}
    });
}	
#[acmd_script(
    agent = "kirby",
    scripts =  ["sound_specialhi", "sound_specialairhi"],
    category = ACMD_SOUND)]
unsafe fn kirby_upb_sound(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		frame(Frames=1)
		if(is_excute){
			PLAY_SEQUENCE(hash40("seq_kirby_rnd_attack"))
		}
    });
}
#[fighter_frame( agent = FIGHTER_KIND_KIRBY )]
fn kirby_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent); 
		let ENTRY_ID = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
		let status_kind = smash::app::lua_bind::StatusModule::status_kind(boma);
		if status_kind == *FIGHTER_KIRBY_STATUS_KIND_SPECIAL_HI2 && MotionModule::frame(boma) >= 13.0 {
			StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_FALL_SPECIAL, true);
		};
		if status_kind == *FIGHTER_KIRBY_STATUS_KIND_SPECIAL_HI2 {
			if  MotionModule::frame(boma) < 2.0 {
				let stick_x = ControlModule::get_stick_x(boma) * PostureModule::lr(boma);
				if stick_x <= -0.3 {
					UPB_ANGLE[ENTRY_ID] = 0;
				} else if stick_x >= 0.45 {
					UPB_ANGLE[ENTRY_ID] = 2;
				} else {
					UPB_ANGLE[ENTRY_ID] = 1;
				};
			};
			if MotionModule::frame(boma) <= 6.0 && !is_hitlag(boma) {
				if UPB_ANGLE[ENTRY_ID] == 1 {
					//let speed = smash::phx::Vector3f { x: *(((6.0/MotionModule::frame(boma))*0.003)/6.0)*0.2)-0.03, y: 0.0, z: 0.0 };
					//KineticModule::add_speed(boma, &speed);
					macros::SET_SPEED_EX(fighter, 1.0, 0.12, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
				};
				if UPB_ANGLE[ENTRY_ID] == 2 {
					//let speed = smash::phx::Vector3f { x: ((6.0/MotionModule::frame(boma))*0.05)/6.0, y: -0.03, z: 0.0 };
					//KineticModule::add_speed(boma, &speed);
					macros::SET_SPEED_EX(fighter, 1.75, 0.08, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
				};
				if UPB_ANGLE[ENTRY_ID] == 0 {
					//let speed = smash::phx::Vector3f { x: *(((6.0/MotionModule::frame(boma))*0.003)/6.0)*0.2)-0.03, y: 0.0, z: 0.0 };
					//KineticModule::add_speed(boma, &speed);
					macros::SET_SPEED_EX(fighter, 0.0, 0.16, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
				};
			};
			if MotionModule::frame(boma) <= 10.0 && MotionModule::frame(boma) > 6.0 && !is_hitlag(boma) {
				if UPB_ANGLE[ENTRY_ID] != 0 {
					KineticModule::clear_speed_all(boma);
					if UPB_ANGLE[ENTRY_ID] == 1 {
						let speed = smash::phx::Vector3f { x: -0.1, y: 0.0, z: 0.0 };
						KineticModule::add_speed(boma, &speed);
					};
					/*let speed = smash::phx::Vector3f { x: -2.0*(0.02-((MotionModule::frame(boma)-6.0/6.0)*0.02)), y: 0.0, z: 0.0 };
					KineticModule::add_speed(boma, &speed);*/
				};
			};
		} else {
			UPB_ANGLE[ENTRY_ID] = 1;
		};
		if status_kind == *FIGHTER_KIRBY_STATUS_KIND_SPECIAL_HI3 {
			StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_FALL_SPECIAL, true);
		};
		if status_kind == *FIGHTER_KIRBY_STATUS_KIND_SPECIAL_HI4 {
			StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_LANDING, true);
		};
		if status_kind == *FIGHTER_STATUS_KIND_FINAL || status_kind == *FIGHTER_STATUS_KIND_FINAL_JUMP_END {
			IS_FINAL[ENTRY_ID] = true;
		} else {
			IS_FINAL[ENTRY_ID] = false;
		};
		if [*FIGHTER_STATUS_KIND_ATTACK_AIR].contains(&status_kind) == false {
			ArticleModule::remove_exist(boma, *FIGHTER_KIRBY_GENERATE_ARTICLE_FINALCUTTER,smash::app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
		};
		if status_kind == *FIGHTER_KIRBY_STATUS_KIND_SPECIAL_S_ATTACK &&  StatusModule::is_situation_changed(boma) {
			let frame = MotionModule::frame(boma);
			if (frame > 16.0 && frame < 25.0) || frame > 30.0 {
				StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_LANDING, true);
			};
		};
	}
}
	
pub fn install() {
    smashline::install_acmd_scripts!(
		kirby_dtilt,
		kirby_fair,
		kirby_upb1,
		kirby_air_upb2,
		kirby_air_upb2_effect,
		kirby_upb2_sound,
		kirby_upb_sound,
		kirby_upb4,
		kirby_bair,
		kirby_landing_bair,
		kirby_bair_eff,
		kirby_bair_snd,
		kirby_dthrow,
		kirby_upb4_exp,
		kirby_upb4_sound
    );
    smashline::install_agent_frames!( kirby_frame);
}
