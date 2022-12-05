use smash::hash40;
use smash::lib::lua_const::*;
use smash::lua2cpp::*;
use smashline::*;
use smash_script::*;
use smash::app::lua_bind::*;
use crate::util::*;
use smash::lib::L2CValue;
use crate::util::CAN_ATTACK_AIR;

static mut ISA_RESHOOT_TIME: [i32; 8] = [0; 8];



#[acmd_script(
    agent = "shizue",
    script =  "game_attack11",
    category = ACMD_GAME, low_priority )]
unsafe fn isa_jab(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		if(is_excute){
			ArticleModule::generate_article(FIGHTER_SHIZUE_GENERATE_ARTICLE_PICOPICOHAMMER, false, 0)
		}
		frame(Frame=3)
		if(is_excute){
			ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=3.0, Angle=65, KBG=100, FKB=0, BKB=50, Size=2.5, X=0.0, Y=5.5, Z=5.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.8, SDI=0.5, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_OBJECT)
			ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=3.0, Angle=65, KBG=100, FKB=0, BKB=50, Size=2.5, X=0.0, Y=5.5, Z=8.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.8, SDI=0.5, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_OBJECT)
			ATTACK(ID=2, Part=0, Bone=hash40("top"), Damage=3.0, Angle=65, KBG=100, FKB=0, BKB=50, Size=2.5, X=0.0, Y=5.5, Z=11.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.8, SDI=0.5, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_OBJECT)
		}
		wait(Frames=2)
		if(is_excute){
			AttackModule::clear_all()
		}
		frame(Frame=30)
		if(is_excute){
			ArticleModule::remove_exist(FIGHTER_SHIZUE_GENERATE_ARTICLE_PICOPICOHAMMER,smash::app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL))
		}
    });
}		

#[acmd_script(
    agent = "shizue",
    script =  "game_throwhi",
    category = ACMD_GAME, low_priority )]
unsafe fn isa_uthrow(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		if(is_excute){
			ATTACK_ABS(Kind=FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, ID=0, Damage=7.5, Angle=90, KBG=120, FKB=0, BKB=50, Hitlag=0.0, Unk=1.0, FacingRestrict=ATTACK_LR_CHECK_F, Unk=0.0, Unk=true, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_NONE, Type=ATTACK_REGION_THROW)
			ATTACK_ABS(Kind=FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, ID=0, Damage=3.0, Angle=361, KBG=100, FKB=0, BKB=40, Hitlag=0.0, Unk=1.0, FacingRestrict=ATTACK_LR_CHECK_F, Unk=0.0, Unk=true, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_NONE, Type=ATTACK_REGION_THROW)
		}
		frame(Frame=20)
		if(is_excute){
			ATK_HIT_ABS(FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, hash40("throw"), WorkModule::get_int64(module_accessor,*FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT), WorkModule::get_int64(module_accessor,*FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP), WorkModule::get_int64(module_accessor,*FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO))
			AttackModule::clear_all()
		}
    });
}
#[acmd_script(
    agent = "shizue",
    script =  "game_throwf",
    category = ACMD_GAME, low_priority )]
unsafe fn isa_fthrow(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		if(is_excute){
			ATTACK_ABS(Kind=FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, ID=0, Damage=9.0, Angle=30, KBG=20, FKB=0, BKB=107, Hitlag=0.0, Unk=1.0, FacingRestrict=ATTACK_LR_CHECK_F, Unk=0.0, Unk=true, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_NONE, Type=ATTACK_REGION_THROW)
			ATTACK_ABS(Kind=FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, ID=0, Damage=3.0, Angle=361, KBG=100, FKB=0, BKB=60, Hitlag=0.0, Unk=1.0, FacingRestrict=ATTACK_LR_CHECK_F, Unk=0.0, Unk=true, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_NONE, Type=ATTACK_REGION_THROW)
		}
		frame(Frame=15)
		if(is_excute){
			ATK_HIT_ABS(FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, hash40("throw"), WorkModule::get_int64(module_accessor,*FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT), WorkModule::get_int64(module_accessor,*FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP), WorkModule::get_int64(module_accessor,*FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO))
			AttackModule::clear_all()
		}
    });
}
#[acmd_script(
    agent = "shizue_slingshot",
    scripts =  ["game_attackairf", "game_attackairb"],
    category = ACMD_GAME, low_priority )]
unsafe fn isa_neutralb_shoot(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		frame(Frame=12)
		if(is_excute){
			ArticleModule::generate_article(WEAPON_SHIZUE_SLINGSHOT_GENERATE_ARTICLE_BULLET, false, 0)
			rust {
				let rand_val = smash::app::sv_math::rand(hash40("fighter"), 8);
				if rand_val == 0 {
					ArticleModule::change_motion(fighter.module_accessor, *WEAPON_SHIZUE_SLINGSHOT_GENERATE_ARTICLE_BULLET,smash::phx::Hash40::new("item_1"),false,0.0);
				} else if rand_val == 1 {
					ArticleModule::change_motion(fighter.module_accessor, *WEAPON_SHIZUE_SLINGSHOT_GENERATE_ARTICLE_BULLET,smash::phx::Hash40::new("item_2"),false,0.0);
				} else if rand_val == 2 {
					ArticleModule::change_motion(fighter.module_accessor, *WEAPON_SHIZUE_SLINGSHOT_GENERATE_ARTICLE_BULLET,smash::phx::Hash40::new("item_3"),false,0.0);
				} else if rand_val == 3 {
					ArticleModule::change_motion(fighter.module_accessor, *WEAPON_SHIZUE_SLINGSHOT_GENERATE_ARTICLE_BULLET,smash::phx::Hash40::new("item_4"),false,0.0);
				} else if rand_val == 4 {
					ArticleModule::change_motion(fighter.module_accessor, *WEAPON_SHIZUE_SLINGSHOT_GENERATE_ARTICLE_BULLET,smash::phx::Hash40::new("item_5"),false,0.0);
				} else if rand_val == 5 {
					ArticleModule::change_motion(fighter.module_accessor, *WEAPON_SHIZUE_SLINGSHOT_GENERATE_ARTICLE_BULLET,smash::phx::Hash40::new("item_6"),false,0.0);
				} else if rand_val == 6 {
					ArticleModule::change_motion(fighter.module_accessor, *WEAPON_SHIZUE_SLINGSHOT_GENERATE_ARTICLE_BULLET,smash::phx::Hash40::new("item_7"),false,0.0);
				} else {
					ArticleModule::change_motion(fighter.module_accessor, *WEAPON_SHIZUE_SLINGSHOT_GENERATE_ARTICLE_BULLET,smash::phx::Hash40::new("item_8"),false,0.0);
				};
			}
			ArticleModule::shoot(WEAPON_SHIZUE_SLINGSHOT_GENERATE_ARTICLE_BULLET, ARTICLE_OPE_TARGET_LAST, false)
		}
    });
}
#[acmd_script(
    agent = "shizue_bullet",
    scripts =  ["game_shootf", "game_shootb"],
    category = ACMD_GAME, low_priority )]
unsafe fn isa_neutralb_hit(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		if(is_excute){
			ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=9.0, Angle=361, KBG=100, FKB=0, BKB=55, Size=5.0, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=-4.5, Trip=0.0, Rehit=0, Reflectable=true, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_OBJECT)
			AttackModule::enable_safe_pos()
		}
		frame(Frame=5)
		if(is_excute){
			ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=3.0, Angle=361, KBG=100, FKB=0, BKB=15, Size=5.0, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.35, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=-1.5, Trip=0.0, Rehit=0, Reflectable=true, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_OBJECT)
		}
    });
}
#[acmd_script(
    agent = "shizue",
    scripts =  ["game_specialairn"],
    category = ACMD_GAME, low_priority )]
unsafe fn isa_neutralb_air(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		if(is_excute){
			ArticleModule::generate_article(FIGHTER_SHIZUE_GENERATE_ARTICLE_SLINGSHOT, true, 0)
		}
		frame(Frame=14)
		FT_MOTION_RATE(FSM=0.625)
		frame(Frame=40)
		if(is_excute){
			ArticleModule::remove_exist(FIGHTER_SHIZUE_GENERATE_ARTICLE_SLINGSHOT,smash::app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL))
		}
    });
}
#[acmd_script(
    agent = "shizue",
    scripts =  ["game_specialn"],
    category = ACMD_GAME, low_priority )]
unsafe fn isa_neutralb_ground(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		if(is_excute){
			ArticleModule::generate_article(FIGHTER_SHIZUE_GENERATE_ARTICLE_SLINGSHOT, true, 0)
		}
		frame(Frame=14)
		FT_MOTION_RATE(FSM=0.75)
		frame(Frame=40)
		if(is_excute){
			ArticleModule::remove_exist(FIGHTER_SHIZUE_GENERATE_ARTICLE_SLINGSHOT,smash::app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL))
		}
    });
}
#[acmd_script(
    agent = "shizue",
    scripts =  ["effect_specialn", "effect_specialairn"],
    category = ACMD_EFFECT, low_priority )]
unsafe fn isa_neutralb_eff(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		frame(Frame=9)
		if(is_excute){
			EFFECT(hash40("sys_attack_impact"), hash40("havel"), 0, -4, 0, 0, 0, 0, 1.3, 0, 0, 0, 0, 0, 360, true)
			LAST_EFFECT_SET_ALPHA(0.6)
		}
    });
}
#[acmd_script(
    agent = "shizue",
    scripts =  ["sound_specialn", "sound_specialairn"],
    category = ACMD_SOUND, low_priority )]
unsafe fn isa_neutralb_snd(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		frame(Frame=3)
		if(is_excute){
			PLAY_SE(hash40("se_shizue_attackair_f01"))
		}
    });
}
#[acmd_script(
    agent = "shizue_pot",
    script =  "game_attackdash",
    category = ACMD_GAME, low_priority )]
unsafe fn isa_da(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		if(is_excute){
			ATTACK(ID=0, Part=0, Bone=hash40("have"), Damage=10.0, Angle=70, KBG=50, FKB=0, BKB=65, Size=4.2, X=0.0, Y=2.5, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.6, SDI=0.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=true, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_OBJECT)
			AttackModule::enable_safe_pos()
		}
		wait(Frames=6)
		if(is_excute){
			ATTACK(ID=0, Part=0, Bone=hash40("have"), Damage=6.0, Angle=70, KBG=50, FKB=0, BKB=65, Size=3.7, X=0.0, Y=2.5, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.6, SDI=0.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=true, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_OBJECT)
		}
    });
}
#[acmd_script(
    agent = "shizue",
    script =  "game_attackairb",
    category = ACMD_GAME, low_priority )]
unsafe fn isa_bair(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		if(is_excute){
			ArticleModule::generate_article(FIGHTER_SHIZUE_GENERATE_ARTICLE_BROOM, false, 0)
			ArticleModule::change_motion(FIGHTER_SHIZUE_GENERATE_ARTICLE_BROOM,smash::phx::Hash40::new("attack_hi3"),false,0.0)
			WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
		}
		FT_MOTION_RATE(FSM=2)
		wait(Frames=2)
		FT_MOTION_RATE(FSM=0.5)
		frame(Frame=8)
		FT_MOTION_RATE(FSM=1)
		if(is_excute){
			ATTACK(ID=0, Part=0, Bone=hash40("havel"), Damage=10.5, Angle=361, KBG=76, FKB=0, BKB=52, Size=3.5, X=0.0, Y=1.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_B, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_OBJECT)
			ATTACK(ID=1, Part=0, Bone=hash40("havel"), Damage=10.5, Angle=361, KBG=76, FKB=0, BKB=52, Size=5.0, X=0.0, Y=7.5, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_B, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_OBJECT)
		}
		frame(Frame=12)
		if(is_excute){
			AttackModule::clear_all()
		}
		frame(Frame=38)
		if(is_excute){
			WorkModule::off_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
			ArticleModule::remove_exist(FIGHTER_SHIZUE_GENERATE_ARTICLE_BROOM,smash::app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL))
		}
    });
}
#[acmd_script(
    agent = "shizue",
    script =  "game_attackairlw",
    category = ACMD_GAME, low_priority )]
unsafe fn isa_dair(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		frame(Frame=1)
		FT_MOTION_RATE(FSM=0.769)
		if(is_excute){
			ArticleModule::generate_article(FIGHTER_SHIZUE_GENERATE_ARTICLE_PICOPICOHAMMER, false, 0)
		}
		frame(Frame=14)
		FT_MOTION_RATE(FSM=1)
		if(is_excute){
			WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
			ATTACK(ID=0, Part=0, Bone=hash40("havel"), Damage=10.0, Angle=270, KBG=95, FKB=0, BKB=10, Size=5.0, X=0.0, Y=1.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_OBJECT)
		}
		wait(Frames=4)
		if(is_excute){
			ATTACK(ID=0, Part=0, Bone=hash40("havel"), Damage=5.0, Angle=45, KBG=100, FKB=0, BKB=30, Size=4.5, X=0.0, Y=1.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_SHIZUE_HAMMER, Type=ATTACK_REGION_OBJECT)
		}
		frame(Frame=32)
		if(is_excute){
			AttackModule::clear_all()
		}
		frame(Frame=39)
		if(is_excute){
			WorkModule::off_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
		}
		frame(Frame=46)
		if(is_excute){
			ArticleModule::remove_exist(FIGHTER_SHIZUE_GENERATE_ARTICLE_PICOPICOHAMMER,smash::app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL))
		}
    });
}
#[acmd_script(
    agent = "shizue",
    script =  "game_landingairlw",
    category = ACMD_GAME, low_priority )]
unsafe fn isa_dair_land(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		frame(Frame=10)
		if(is_excute){
			ArticleModule::remove_exist(FIGHTER_SHIZUE_GENERATE_ARTICLE_PICOPICOHAMMER,smash::app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL))
		}
    });
}
#[acmd_script(
    agent = "shizue",
    script =  "effect_attackairb",
    category = ACMD_EFFECT, low_priority )]
unsafe fn isa_bair_eff(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		frame(Frame=8)
		if(is_excute){
			EFFECT_FOLLOW_FLIP(0x1288a621adu64, 0x1288a621adu64, hash40("top"), -3, 0, 0, 13, 180, 180, 1.6, false, EF_FLIP_YZ)
		}
    });
}
#[acmd_script(
    agent = "shizue",
    script =  "sound_attackairb",
    category = ACMD_SOUND, low_priority )]
unsafe fn isa_bair_snd(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		frame(Frame=10)
		if(is_excute){
			PLAY_SE(hash40("se_shizue_attackhard_h01"))
		}
    });
}
#[acmd_script(
    agent = "shizue",
    script =  "game_landingairb",
    category = ACMD_GAME, low_priority )]
unsafe fn isa_bairland(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		if(is_excute){
			ArticleModule::remove_exist(FIGHTER_SHIZUE_GENERATE_ARTICLE_BROOM,smash::app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL))
		}
    });
}
#[acmd_script(
    agent = "shizue",
    script =  "game_attackairf",
    category = ACMD_GAME, low_priority )]
unsafe fn isa_fair(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		if(is_excute){
			ArticleModule::generate_article(FIGHTER_SHIZUE_GENERATE_ARTICLE_CRACKER, false, 0)
			ArticleModule::change_motion(FIGHTER_SHIZUE_GENERATE_ARTICLE_CRACKER,smash::phx::Hash40::new("fire"),false,0.0)
			WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
		}
		frame(Frame=14)
		if(is_excute){
			SET_SPEED_EX(-1.2, 0.25, KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN)
			ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=14.0, Angle=361, KBG=99, FKB=0, BKB=25, Size=6.0, X=0.0, Y=6.0, Z=5.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_OBJECT)
			ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=14.0, Angle=361, KBG=99, FKB=0, BKB=25, Size=7.0, X=0.0, Y=7.0, Z=10.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_OBJECT)
		}
		frame(Frame=17)
		if(is_excute){
			ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=9.0, Angle=361, KBG=110, FKB=0, BKB=40, Size=8.57, X=0.0, Y=6.0, Z=5.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_OBJECT)
			ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=9.0, Angle=361, KBG=110, FKB=0, BKB=40, Size=10.0, X=0.0, Y=7.0, Z=10.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_OBJECT)
		}
		frame(Frame=20)
		if(is_excute){
			AttackModule::clear_all()
		}
		frame(Frame=46)
		if(is_excute){
			WorkModule::off_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
			ArticleModule::remove_exist(FIGHTER_SHIZUE_GENERATE_ARTICLE_CRACKER,smash::app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL))
		}
    });
}
#[acmd_script(
    agent = "shizue",
    script =  "sound_attackairf",
    category = ACMD_SOUND, low_priority )]
unsafe fn isa_fair_snd(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		frame(Frame=7)
		if(is_excute){
			STOP_SE(hash40("se_common_smash_start"))
			STOP_SE(hash40("se_shizue_smash_s02"))
		}
		wait(Frames=7)
		if(is_excute){
			PLAY_SE(hash40("se_shizue_smash_s01"))
		}
    });
}
#[acmd_script(
    agent = "shizue",
    script =  "effect_attackairf",
    category = ACMD_EFFECT, low_priority )]
unsafe fn isa_fair_eff(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		frame(Frame=7)
		if(is_excute){
			STOP_SE(hash40("se_common_smash_start"))
			STOP_SE(hash40("se_shizue_smash_s02"))
		}
		wait(Frames=7)
		if(is_excute){
			PLAY_SE(hash40("se_shizue_smash_s01"))
		}
    });
}
#[acmd_script(
    agent = "shizue",
    script =  "game_landingairf",
    category = ACMD_GAME, low_priority )]
unsafe fn isa_fairland(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		if(is_excute){
			ArticleModule::remove_exist(FIGHTER_SHIZUE_GENERATE_ARTICLE_CRACKER,smash::app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL))
		}
    });
}
#[acmd_script(
    agent = "shizue",
    scripts =  ["game_speciallwfire", "game_specialairlwfire"],
    category = ACMD_GAME, low_priority )]
unsafe fn isa_lloid_end(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let ENTRY_ID = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
	if ISA_RESHOOT_TIME[ENTRY_ID] < 1 {
		ArticleModule::shoot_exist(fighter.module_accessor, *FIGHTER_SHIZUE_GENERATE_ARTICLE_CLAYROCKET, smash::app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL), false);
		ISA_RESHOOT_TIME[ENTRY_ID] = 130;
	};
}
#[acmd_script(
    agent = "shizue",
    scripts =  ["game_speciallwset"],
    category = ACMD_GAME, low_priority )]
unsafe fn isa_lloid_set(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		frame(Frame=1)
		if(is_excute){
			JostleModule::set_status(false)
		}
		FT_MOTION_RATE(FSM=0.6)
		frame(Frame=25)
		if(is_excute){
			WorkModule::on_flag(Flag=FIGHTER_SHIZUE_STATUS_WORK_ID_SPECIAL_LW_FLAG_SET)
		}
		FT_MOTION_RATE(FSM=1)
    });
}
#[status_script(agent = "shizue", status = FIGHTER_MURABITO_STATUS_KIND_SPECIAL_N_SEARCH, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_PRE)]
unsafe fn special_n_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(
        fighter.module_accessor,
        smash::app::SituationKind(*SITUATION_KIND_NONE),
        *FIGHTER_KINETIC_TYPE_NONE,
        *GROUND_CORRECT_KIND_KEEP as u32,
		smash::app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE),
        true,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT,
        0
    );

    FighterStatusModuleImpl::set_fighter_status_data(
        fighter.module_accessor,
        false,
        *FIGHTER_TREADED_KIND_NO_REAC,
        false,
        false,
        false,
        (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_N | *FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_AIR_LASSO | *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON) as u64,
        0,
        *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_N as u32,
        0
    );
    0.into()
}

#[fighter_frame( agent = FIGHTER_KIND_SHIZUE )]
fn shizue_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent); 
		let status_kind = smash::app::lua_bind::StatusModule::status_kind(boma);
		let ENTRY_ID = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
		let fighter_kind = smash::app::utility::get_kind(boma);
		let motion_kind = MotionModule::motion_kind(boma);
		let stick_y = ControlModule::get_stick_y(boma);
		if [*FIGHTER_STATUS_KIND_DEAD, *FIGHTER_STATUS_KIND_LOSE, *FIGHTER_STATUS_KIND_WIN].contains(&status_kind) || smash::app::sv_information::is_ready_go() == false{
			ISA_RESHOOT_TIME[ENTRY_ID] = 0;
		};
		if ISA_RESHOOT_TIME[ENTRY_ID] > 0 {
			ISA_RESHOOT_TIME[ENTRY_ID] -= 1;
		};
		if status_kind == *FIGHTER_SHIZUE_STATUS_KIND_SPECIAL_LW_FIRE {
				if motion_duration(boma) == 5 {
					if (ControlModule::get_stick_x(boma)*PostureModule::lr(boma)) < -0.2 {
						PostureModule::reverse_lr(boma);
						PostureModule::update_rot_y_lr(boma);
						let stop_rise  = smash::phx::Vector3f { x: -1.0, y: 1.0, z: 1.0 };
						KineticModule::mul_speed(boma, &stop_rise, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
					};
				};
		};
		if ![*FIGHTER_STATUS_KIND_ATTACK_AIR, *FIGHTER_STATUS_KIND_ATTACK, *FIGHTER_STATUS_KIND_LANDING_ATTACK_AIR].contains(&status_kind) {
			ArticleModule::remove_exist(boma, *FIGHTER_SHIZUE_GENERATE_ARTICLE_PICOPICOHAMMER,smash::app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
		};
		if [hash40("special_air_hi"), hash40("special_hi"), hash40("special_air_hi_wait1"), hash40("special_air_hi_wait2"), hash40("special_air_hi_flap1"), hash40("special_air_hi_flap2"), hash40("special_air_hi_flap1")].contains(&MotionModule::motion_kind(boma)) {
			if WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT) < WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT_MAX) && StatusModule::situation_kind(boma) == *SITUATION_KIND_AIR && ControlModule::check_button_on_trriger(boma, *CONTROL_PAD_BUTTON_JUMP) && ControlModule::check_button_off(boma, *CONTROL_PAD_BUTTON_SPECIAL){
				ArticleModule::remove_exist(boma, *FIGHTER_SHIZUE_GENERATE_ARTICLE_SWING,smash::app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
				ArticleModule::remove(boma, *FIGHTER_SHIZUE_GENERATE_ARTICLE_SWING,smash::app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
				StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_JUMP_AERIAL, true);
			};
			if ControlModule::check_button_on_trriger(boma, *CONTROL_PAD_BUTTON_GUARD) && ControlModule::check_button_off(boma, *CONTROL_PAD_BUTTON_SPECIAL) && !WorkModule::is_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_DISABLE_ESCAPE_AIR){
				ArticleModule::remove_exist(boma, *FIGHTER_SHIZUE_GENERATE_ARTICLE_SWING,smash::app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
				ArticleModule::remove(boma, *FIGHTER_SHIZUE_GENERATE_ARTICLE_SWING,smash::app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
				StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ESCAPE_AIR, true);
			};
		};
		if [hash40("special_n"), hash40("special_air_n")].contains(&motion_kind) {
			if MotionModule::frame(boma) >= 52.0 {
				if StatusModule::situation_kind(boma) == *SITUATION_KIND_AIR {
					StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_FALL, false);
				} else {
					StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_WAIT, false);
				};
			};
			if StatusModule::situation_kind(boma) == *SITUATION_KIND_AIR {
				let cat2 = ControlModule::get_command_flag_cat(boma, 1);
				if (cat2 & *FIGHTER_PAD_CMD_CAT2_FLAG_FALL_JUMP) != 0 && stick_y < -0.66 && SPEED_Y[ENTRY_ID] <= 0.0 {
					WorkModule::set_flag(boma, true, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_DIVE);
				}
			};
		};
		if ISA_RESHOOT_TIME[ENTRY_ID] < 1{
			if [
				*FIGHTER_STATUS_KIND_GUARD_ON, *FIGHTER_STATUS_KIND_GUARD_OFF, *FIGHTER_STATUS_KIND_GUARD_DAMAGE, *FIGHTER_STATUS_KIND_GUARD, *FIGHTER_STATUS_KIND_ATTACK, *FIGHTER_STATUS_KIND_ATTACK_DASH, 
				*FIGHTER_STATUS_KIND_ATTACK_LW3, *FIGHTER_STATUS_KIND_ATTACK_S3, *FIGHTER_STATUS_KIND_ATTACK_HI3, *FIGHTER_STATUS_KIND_ATTACK_AIR, *FIGHTER_STATUS_KIND_LANDING_ATTACK_AIR, *FIGHTER_STATUS_KIND_ATTACK_S4_START, 
				*FIGHTER_STATUS_KIND_ATTACK_S4_HOLD, *FIGHTER_STATUS_KIND_ATTACK_S4, *FIGHTER_STATUS_KIND_ATTACK_HI4_START, *FIGHTER_STATUS_KIND_ATTACK_HI4_HOLD, *FIGHTER_STATUS_KIND_ATTACK_HI4, *FIGHTER_STATUS_KIND_ATTACK_LW4_START, 
				*FIGHTER_STATUS_KIND_ATTACK_LW4_HOLD, *FIGHTER_STATUS_KIND_ATTACK_LW4, *FIGHTER_STATUS_KIND_SPECIAL_N, *FIGHTER_STATUS_KIND_SPECIAL_S, *FIGHTER_SHIZUE_STATUS_KIND_SPECIAL_S_CUT, *FIGHTER_SHIZUE_STATUS_KIND_SPECIAL_S_END,
				*FIGHTER_SHIZUE_STATUS_KIND_SPECIAL_S_HIT, *FIGHTER_SHIZUE_STATUS_KIND_SPECIAL_S_HOOK, *FIGHTER_SHIZUE_STATUS_KIND_SPECIAL_S_REEL, *FIGHTER_SHIZUE_STATUS_KIND_SPECIAL_S_START, *FIGHTER_SHIZUE_STATUS_KIND_SPECIAL_S_THROW, 
				*FIGHTER_SHIZUE_STATUS_KIND_SPECIAL_S_PICKUP, *FIGHTER_SHIZUE_STATUS_KIND_SPECIAL_S_CATCH_WAIT, *FIGHTER_STATUS_KIND_ESCAPE, *FIGHTER_STATUS_KIND_ESCAPE_AIR_SLIDE, *FIGHTER_STATUS_KIND_ESCAPE_AIR, *FIGHTER_STATUS_KIND_LANDING,
				*FIGHTER_STATUS_KIND_ESCAPE_F, *FIGHTER_STATUS_KIND_ESCAPE_B, *FIGHTER_STATUS_KIND_THROW, *FIGHTER_STATUS_KIND_CATCH, *FIGHTER_STATUS_KIND_CATCH_DASH, *FIGHTER_STATUS_KIND_CATCH_TURN, *FIGHTER_STATUS_KIND_CATCH_ATTACK
			].contains(&status_kind) {
				if ControlModule::get_stick_y(boma) <= -0.5 && ControlModule::check_button_on_trriger(boma, *CONTROL_PAD_BUTTON_SPECIAL) {
					ArticleModule::shoot_exist(boma, *FIGHTER_SHIZUE_GENERATE_ARTICLE_CLAYROCKET, smash::app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL), false);
					ISA_RESHOOT_TIME[ENTRY_ID] = 130;
				};
			};
		};
	};
}		
pub fn install() {
    smashline::install_acmd_scripts!(
		isa_uthrow,
		isa_fthrow,
		isa_jab,
		isa_lloid_end,
		isa_lloid_set,
		isa_fair,
		isa_fairland,
		isa_fair_eff,
		isa_fair_snd,
		isa_bair,
		isa_bairland,
		isa_bair_eff,
		isa_bair_snd,
		isa_da,
		isa_neutralb_air,
		isa_neutralb_ground,
		isa_neutralb_eff,
		isa_neutralb_snd,
		isa_neutralb_hit,
		isa_neutralb_shoot
		isa_dair,
		isa_dair_land
    );
    smashline::install_agent_frames!(
        shizue_frame
    );
	smashline::install_status_scripts!(special_n_pre);
}
