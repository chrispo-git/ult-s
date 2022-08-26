use smash::hash40;
use smash::lib::lua_const::*;
use smash::app::lua_bind::*;
use smash::lua2cpp::*;
use smash::lua2cpp::*;
use smashline::*;
use smash_script::*;	
use smash::phx::Vector2f;
use smash::phx::Hash40;
		
use crate::util::*;
static mut IS_THUNDER : [bool; 8] = [false; 8];

		
#[acmd_script(
    agent = "master",
    script =  "game_attacklw3",
    category = ACMD_GAME)]
unsafe fn byleth_dtilt(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		FT_MOTION_RATE(FSM=0.583333)
		if(is_excute){
			ArticleModule::generate_article(FIGHTER_MASTER_GENERATE_ARTICLE_SWORD, false, 0)
			ArticleModule::change_motion(FIGHTER_MASTER_GENERATE_ARTICLE_SWORD,smash::phx::Hash40::new("attack_lw3"),false,0.0)
		}
		frame(Frame=13)
		FT_MOTION_RATE(FSM=1)
		if(is_excute){
			ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=8.0, Angle=93, KBG=55, FKB=0, BKB=67, Size=2.8, X=0.0, Y=2.8, Z=10.0, X2=0.0, Y2=2.8, Z2=12.5, Hitlag=0.8, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_THRU, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
			ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=8.0, Angle=96, KBG=55, FKB=0, BKB=67, Size=2.8, X=0.0, Y=2.8, Z=17.0, X2=0.0, Y2=2.8, Z2=25.5, Hitlag=0.8, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_THRU, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
		}
		frame(Frame=16)
		if(is_excute){
			AttackModule::clear_all()
		}
		frame(Frame=59)
		if(is_excute){
			ArticleModule::remove_exist(FIGHTER_MASTER_GENERATE_ARTICLE_SWORD,smash::app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL))
		}
    });
}				
#[acmd_script(
    agent = "master",
    script =  "game_landingairn",
    category = ACMD_GAME)]
unsafe fn byleth_land_nair(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		if(is_excute){	
			ArticleModule::remove_exist(FIGHTER_MASTER_GENERATE_ARTICLE_SWORD,smash::app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL))
			ArticleModule::remove_exist(FIGHTER_MASTER_GENERATE_ARTICLE_BOW,smash::app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL))
		}
	});
}		
#[acmd_script(
    agent = "master",
    script =  "game_attackairb",
    category = ACMD_GAME)]
unsafe fn byleth_bair(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		if(is_excute){
			ArticleModule::generate_article(FIGHTER_MASTER_GENERATE_ARTICLE_SPEAR, false, 0)
		}
		frame(Frame=6)
		if(is_excute){
			WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
		}
		frame(Frame=13)
		if(is_excute){
			ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=10.0, Angle=46, KBG=60, FKB=0, BKB=80, Size=2.3, X=0.0, Y=9.5, Z=-4.0, X2=0.0, Y2=9.5, Z2=-12.0, Hitlag=0.7, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_B, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_OBJECT)
			ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=15.0, Angle=28, KBG=75, FKB=0, BKB=53, Size=3.5, X=0.0, Y=9.5, Z=-17.0, X2=0.0, Y2=9.5, Z2=-24.0, Hitlag=1.4, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_B, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_sting"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_OBJECT)
		}
		frame(Frame=15)
		if(is_excute){
			ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=5.0, Angle=361, KBG=75, FKB=0, BKB=50, Size=2.0, X=0.0, Y=10.0, Z=-4.0, X2=0.0, Y2=10.0, Z2=-12.0, Hitlag=0.8, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_B, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_OBJECT)
			ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=7.5, Angle=361, KBG=75, FKB=0, BKB=50, Size=3.0, X=0.0, Y=10.0, Z=-17.0, X2=0.0, Y2=10.0, Z2=-23.0, Hitlag=0.8, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_B, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_sting"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_OBJECT)
		}
		frame(Frame=16)
		if(is_excute){
			ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=5.0, Angle=361, KBG=75, FKB=0, BKB=50, Size=1.769, X=0.0, Y=10.0, Z=-3.0, X2=0.0, Y2=10.0, Z2=-10.0, Hitlag=0.8, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_B, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_OBJECT)
			ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=7.5, Angle=361, KBG=75, FKB=0, BKB=50, Size=2.692, X=0.0, Y=10.0, Z=-14.0, X2=0.0, Y2=10.0, Z2=-21.0, Hitlag=0.8, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_B, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_sting"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_OBJECT)
		}
		frame(Frame=18)
		if(is_excute){
			AttackModule::clear_all()
		}
		frame(Frame=40)
		if(is_excute){
			WorkModule::off_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
		}
		frame(Frame=54)
		if(is_excute){
			ArticleModule::remove_exist(FIGHTER_MASTER_GENERATE_ARTICLE_SPEAR,smash::app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL))
		}
	});
}			
#[acmd_script(
    agent = "master",
    script =  "game_attackairf",
    category = ACMD_GAME)]
unsafe fn byleth_fair(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		if(is_excute){
			ArticleModule::generate_article(FIGHTER_MASTER_GENERATE_ARTICLE_SPEAR, false, 0)
		}
		frame(Frame=3)
		if(is_excute){
			WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
		}
		frame(Frame=12)
		if(is_excute){
			ATTACK(ID=0, Part=0, Bone=hash40("haver"), Damage=8.5, Angle=55, KBG=100, FKB=0, BKB=52, Size=3.0, X=-0.5, Y=4.0, Z=-0.5, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.8, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_OBJECT)
			ATTACK(ID=1, Part=0, Bone=hash40("haver"), Damage=8.5, Angle=55, KBG=100, FKB=0, BKB=52, Size=2.4, X=-0.5, Y=9.2, Z=-0.5, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.8, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_OBJECT)
			ATTACK(ID=2, Part=0, Bone=hash40("haver"), Damage=8.5, Angle=55, KBG=100, FKB=0, BKB=52, Size=2.4, X=-0.5, Y=13.5, Z=-0.5, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.8, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_OBJECT)
			ATTACK(ID=3, Part=0, Bone=hash40("haver"), Damage=12.75, Angle=361, KBG=84, FKB=0, BKB=52, Size=3.4, X=-0.5, Y=19.0, Z=-0.5, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.3, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_OBJECT)
			ATTACK(ID=4, Part=0, Bone=hash40("haver"), Damage=12.75, Angle=361, KBG=84, FKB=0, BKB=52, Size=3.0, X=-0.5, Y=25.0, Z=-0.5, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.3, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_OBJECT)
		}
		frame(Frame=13)
		if(is_excute){
			ATTACK(ID=5, Part=0, Bone=hash40("top"), Damage=8.5, Angle=55, KBG=100, FKB=0, BKB=52, Size=2.0, X=0.0, Y=5.0, Z=5.0, X2=0.0, Y2=5.0, Z2=13.0, Hitlag=0.8, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_OBJECT)
			ATTACK(ID=6, Part=0, Bone=hash40("top"), Damage=12.75, Angle=361, KBG=84, FKB=0, BKB=52, Size=2.5, X=0.0, Y=5.0, Z=16.5, X2=0.0, Y2=5.0, Z2=22.0, Hitlag=1.3, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_OBJECT)
		}
		frame(Frame=14)
		if(is_excute){
			AttackModule::clear_all()
		}
		frame(Frame=34)
		if(is_excute){
			WorkModule::off_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
		}
		frame(Frame=53)
		if(is_excute){
			ArticleModule::remove_exist(FIGHTER_MASTER_GENERATE_ARTICLE_SPEAR,smash::app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL))
		}
	});
}			
			
#[acmd_script(
    agent = "master",
    script =  "game_attackairn",
    category = ACMD_GAME)]
unsafe fn byleth_nair(fighter: &mut L2CAgentBase) {
	let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);  
	let mut is_close = true;
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		frame(Frame=3)
		if(is_excute){
			WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
		}
		frame(Frame=5)
		if(is_excute){
			ArticleModule::generate_article(FIGHTER_MASTER_GENERATE_ARTICLE_BOW, false, 0)
			ArticleModule::change_motion(FIGHTER_MASTER_GENERATE_ARTICLE_BOW,smash::phx::Hash40::new("attack_air_n"),false,0.0)
			ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=0.001, Angle=0, KBG=100, FKB=1, BKB=0, Size=11.5, X=0.0, Y=9.0, Z=2.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.6, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=true, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_NONE, Type=ATTACK_REGION_OBJECT)
		}
		frame(Frame=6)
		if(is_excute){
			AttackModule::clear_all()
			ATTACK(ID=2, Part=0, Bone=hash40("haver"), Damage=2.5, Angle=120, KBG=100, FKB=45, BKB=0, Size=3.5, X=-4.0, Y=-2.8, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.6, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=7, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_G, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_OBJECT)
			ATTACK(ID=1, Part=0, Bone=hash40("haver"), Damage=2.5, Angle=367, KBG=100, FKB=40, BKB=0, Size=3.5, X=-4.0, Y=-2.8, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.6, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=7, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_A, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_OBJECT)
			ATTACK(ID=0, Part=0, Bone=hash40("haver"), Damage=2.5, Angle=94, KBG=100, FKB=45, BKB=0, Size=2.7, X=-4.0, Y=1.3, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.6, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=7, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_G, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_OBJECT)
			ATTACK(ID=3, Part=0, Bone=hash40("haver"), Damage=2.5, Angle=367, KBG=100, FKB=40, BKB=0, Size=2.7, X=-4.0, Y=1.3, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.6, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=7, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_A, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_OBJECT)
			ATTACK(ID=4, Part=0, Bone=hash40("top"), Damage=2.5, Angle=80, KBG=100, FKB=35, BKB=0, Size=3.0, X=0.0, Y=9.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.6, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=7, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_OBJECT)
		}
		frame(Frame=27)
		if(is_excute){
			AttackModule::clear_all()
		}
		frame(Frame=28)
		if(is_excute){
			ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=4.0, Angle=51, KBG=130, FKB=0, BKB=44, Size=10.0, X=0.0, Y=9.0, Z=2.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.4, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_OBJECT)
		}
		frame(Frame=29)
		if(is_excute){
			AttackModule::clear_all()
		}
		frame(Frame=34)
		if(is_excute){
			WorkModule::off_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
		}
		frame(Frame=53)
		if(is_excute){
			ArticleModule::remove_exist(FIGHTER_MASTER_GENERATE_ARTICLE_BOW,smash::app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL))
		}
	});
}	
#[acmd_script(
    agent = "master",
    script =  "sound_attackairn",
    category = ACMD_SOUND)]
unsafe fn byleth_nair_sound(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		frame(Frame=5)
		if(is_excute){
			PLAY_SEQUENCE(hash40("seq_master_rnd_attack02"))
		}
		frame(Frame=7)
		if(is_excute){
			PLAY_SE(hash40("se_master_attackair_n01"))
		}
		frame(Frame=13)
		if(is_excute){
			PLAY_SE(hash40("se_master_attackair_n02"))
		}
		frame(Frame=18)
		if(is_excute){
			PLAY_SE(hash40("se_master_attackair_n03"))
		}
		frame(Frame=23)
		if(is_excute){
			PLAY_SE(hash40("se_master_attackair_n04"))
		}
		frame(Frame=29)
		if(is_excute){
			PLAY_SE(hash40("se_master_attackair_n05"))
		}
	});
}
#[acmd_script(
    agent = "master",
    script =  "sound_attackairn2",
    category = ACMD_SOUND)]
unsafe fn byleth_nair2_sound(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		frame(Frame=8)
		if(is_excute){
			PLAY_SE(hash40("se_master_attackhard_h01"))
		}
	});
}	
#[acmd_script(
    agent = "master",
    scripts =  ["sound_specialairsf", "sound_specialsf"],
    category = ACMD_SOUND)]
unsafe fn byleth_sideb_snd(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		if(is_excute){
			PLAY_SEQUENCE(hash40("seq_master_rnd_attack02"))
		}
		frame(Frame=2)
		if(is_excute){
			PLAY_SE(hash40("se_master_special_s04"))
		}
		frame(Frame=25)
		if(is_excute){
			PLAY_SE(hash40("se_master_cloth_ll03"))
		}
	});
}	
#[acmd_script(
    agent = "master",
    scripts =  ["sound_specialairsfdash", "sound_specialsfdash"],
    category = ACMD_SOUND)]
unsafe fn byleth_sidebdash_snd(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		if(is_excute){
			PLAY_SEQUENCE(hash40("seq_master_rnd_attack02"))
		}
		frame(Frame=2)
		if(is_excute){
			PLAY_SE(hash40("se_master_special_s05"))
		}
		frame(Frame=25)
		if(is_excute){
			PLAY_SE(hash40("se_master_cloth_ll03"))
		}
	});
}	
#[acmd_script(
    agent = "master",
    script =  "game_attackairn2",
    category = ACMD_GAME)]
unsafe fn byleth_nair2(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		if(is_excute){
			ArticleModule::remove_exist(FIGHTER_MASTER_GENERATE_ARTICLE_SWORD,smash::app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL))
			ArticleModule::remove_exist(FIGHTER_MASTER_GENERATE_ARTICLE_BOW,smash::app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL))
		}
		frame(Frame=2)
		if(is_excute){	
			ArticleModule::remove_exist(FIGHTER_MASTER_GENERATE_ARTICLE_SWORD,smash::app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL))
			ArticleModule::remove_exist(FIGHTER_MASTER_GENERATE_ARTICLE_BOW,smash::app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL))
		}
		frame(Frame=3)
		if(is_excute){	
			ArticleModule::remove_exist(FIGHTER_MASTER_GENERATE_ARTICLE_SWORD,smash::app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL))
			ArticleModule::remove_exist(FIGHTER_MASTER_GENERATE_ARTICLE_BOW,smash::app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL))
		}
		frame(Frame=4)
		if(is_excute){	
			ArticleModule::remove_exist(FIGHTER_MASTER_GENERATE_ARTICLE_SWORD,smash::app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL))
			ArticleModule::remove_exist(FIGHTER_MASTER_GENERATE_ARTICLE_BOW,smash::app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL))
		}
		frame(Frame=5)
		if(is_excute){	
			ArticleModule::remove_exist(FIGHTER_MASTER_GENERATE_ARTICLE_SWORD,smash::app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL))
			ArticleModule::remove_exist(FIGHTER_MASTER_GENERATE_ARTICLE_BOW,smash::app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL))
		}
		frame(Frame=6)
		if(is_excute){	
			ArticleModule::remove_exist(FIGHTER_MASTER_GENERATE_ARTICLE_SWORD,smash::app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL))
			ArticleModule::remove_exist(FIGHTER_MASTER_GENERATE_ARTICLE_BOW,smash::app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL))
		}
		frame(Frame=8)
		if(is_excute){
			ATTACK(ID=0, Part=0, Bone=hash40("sword1"), Damage=10.0, Angle=80, KBG=80, FKB=0, BKB=60, Size=4.0, X=12.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
			ATTACK(ID=1, Part=0, Bone=hash40("sword1"), Damage=10.0, Angle=80, KBG=80, FKB=0, BKB=60, Size=4.0, X=7.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
			ATTACK(ID=2, Part=0, Bone=hash40("sword1"), Damage=10.0, Angle=80, KBG=80, FKB=0, BKB=60, Size=4.0, X=3.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
			AttackModule::set_add_reaction_frame(ID=0, Frames=4.0, Unk=false)
			AttackModule::set_add_reaction_frame(ID=1, Frames=4.0, Unk=false)
			AttackModule::set_add_reaction_frame(ID=2, Frames=4.0, Unk=false)
		}
		frame(Frame=22)
		FT_MOTION_RATE(FSM=1)
		if(is_excute){
			AttackModule::clear_all()
		}
		frame(Frame=40)
		if(is_excute){
			CancelModule::enable_cancel()
		}
		frame(Frame=50)
		if(is_excute){
			WorkModule::off_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
		}
	});
}		
	

#[acmd_script(
    agent = "master",
    script =  "effect_attackairn2",
    category = ACMD_EFFECT)]
unsafe fn byleth_nair2e(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		frame(Frame=7)
		if(is_excute){
			AFTER_IMAGE4_ON_arg29(0x11c86cd79du64, 0x1151658627u64, 5, hash40("sword1"), 2.3, 0, 0, hash40("sword1"), 17, 0, 0.15, true, hash40("null"), hash40("sword1"), 0, 0, 0, 0, 0, -90, 1, 0, EFFECT_AXIS_X, 0, TRAIL_BLEND_ALPHA, 101, TRAIL_CULL_NONE, 1.4, 0.1)
		}
		frame(Frame=19)
		if(is_excute){
			AFTER_IMAGE_OFF(8)
		}
	});
}		
#[acmd_script(
    agent = "master",
    script =  "effect_attackairn",
    category = ACMD_EFFECT)]
unsafe fn byleth_naire(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {	
		frame(Frame=7)
		if(is_excute){
			EFFECT_FOLLOW(0x10a0dd48ddu64, hash40("haver"), 0, 4, -1, -90, 90, 0, 1, true)
			EFFECT_FOLLOW_ALPHA(0x0fd1c652bfu64, hash40("haver"), 0, 0, 0, 0, 0, 0, 1, true, 0.6)
		}
		frame(Frame=26)
		if(is_excute){
			EFFECT_OFF_KIND(0x10a0dd48ddu64, false, true)
		}
		frame(Frame=28)
		if(is_excute){
			EFFECT(hash40("sys_smash_flash_s"), hash40("top"), -5, 5, -5, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true)
			LAST_EFFECT_SET_RATE(1.5)
		}
		frame(Frame=30)
		if(is_excute){
			EFFECT_OFF_KIND(0x0fd1c652bfu64, false, true)
		}
	});
}		
#[acmd_script(
    agent = "master",
    scripts =  ["game_specialsf", "game_specialairsf"],
    category = ACMD_GAME)]
unsafe fn byleth_sideb(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		FT_MOTION_RATE(FSM=0.5)
		frame(Frame=10)
		FT_MOTION_RATE(FSM=1)
		if(is_excute){
			ATTACK(ID=1, Part=0, Bone=hash40("armr"), Damage=7.5, Angle=80, KBG=36, FKB=0, BKB=50, Size=5.5, X=0.0, Y=0.0, Z=-1.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
			ATTACK(ID=2, Part=0, Bone=hash40("sword1"), Damage=7.5, Angle=80, KBG=36, FKB=0, BKB=50, Size=5.5, X=3.0, Y=1.0, Z=-2.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
			ATTACK(ID=3, Part=0, Bone=hash40("sword1"), Damage=7.5, Angle=80, KBG=36, FKB=0, BKB=50, Size=5.5, X=7.0, Y=1.0, Z=-2.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
			ATTACK(ID=4, Part=0, Bone=hash40("sword1"), Damage=7.5, Angle=80, KBG=36, FKB=0, BKB=50, Size=5.5, X=13.5, Y=1.0, Z=-2.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
			AttackModule::set_add_reaction_frame(ID=1, Frames=7.0, Unk=false)
			AttackModule::set_add_reaction_frame(ID=2, Frames=7.0, Unk=false)
			AttackModule::set_add_reaction_frame(ID=3, Frames=7.0, Unk=false)
			AttackModule::set_add_reaction_frame(ID=4, Frames=7.0, Unk=false)
		}	
		frame(Frame=15)
		if(is_excute){
			AttackModule::clear_all()
		}
	});
}		
#[acmd_script(
    agent = "master",
    scripts =  ["game_specialsfdash", "game_specialairsfdash"],
    category = ACMD_GAME)]
unsafe fn byleth_sidebdash(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		FT_MOTION_RATE(FSM=0.5)
		frame(Frame=2)
		if(is_excute){
			WorkModule::on_flag(Flag=FIGHTER_MASTER_STATUS_SPECIAL_S_FLAG_SPEED_UP_REQUEST)
		}
		frame(Frame=10)
		FT_MOTION_RATE(FSM=1)
		if(is_excute){
			WorkModule::on_flag(Flag=FIGHTER_MASTER_STATUS_SPECIAL_S_FLAG_SPEED_DOWN_REQUEST)
			ATTACK(ID=1, Part=0, Bone=hash40("armr"), Damage=7.5, Angle=80, KBG=36, FKB=0, BKB=50, Size=5.5, X=0.0, Y=0.0, Z=-1.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
			ATTACK(ID=2, Part=0, Bone=hash40("sword1"), Damage=7.5, Angle=80, KBG=36, FKB=0, BKB=50, Size=5.5, X=3.0, Y=1.0, Z=-2.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
			ATTACK(ID=3, Part=0, Bone=hash40("sword1"), Damage=7.5, Angle=80, KBG=36, FKB=0, BKB=50, Size=5.5, X=7.0, Y=1.0, Z=-2.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
			ATTACK(ID=4, Part=0, Bone=hash40("sword1"), Damage=7.5, Angle=80, KBG=36, FKB=0, BKB=50, Size=5.5, X=13.5, Y=1.0, Z=-2.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
			AttackModule::set_add_reaction_frame(ID=1, Frames=7.0, Unk=false)
			AttackModule::set_add_reaction_frame(ID=2, Frames=7.0, Unk=false)
			AttackModule::set_add_reaction_frame(ID=3, Frames=7.0, Unk=false)
			AttackModule::set_add_reaction_frame(ID=4, Frames=7.0, Unk=false)
		}	
		frame(Frame=15)
		if(is_excute){
			AttackModule::clear_all()
			KineticModule::change_kinetic(FIGHTER_KINETIC_TYPE_FALL)
		}
	});
}			
#[acmd_script(
    agent = "master",
    scripts =  ["effect_specialsf", "effect_specialairsf"],
    category = ACMD_EFFECT)]
unsafe fn byleth_sidebe(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		frame(Frame=10)
		if(is_excute){
			AFTER_IMAGE4_ON_arg29(0x11c86cd79du64, 0x1151658627u64, 5, hash40("sword1"), 2.3, 0, 0, hash40("sword1"), 17, 0, 0.15, true, hash40("null"), hash40("sword1"), 0, 0, 0, 0, 0, -90, 1, 0, EFFECT_AXIS_X, 0, TRAIL_BLEND_ALPHA, 101, TRAIL_CULL_NONE, 1.4, 0.1)
			EFFECT_FOLLOW(0x118d3781c4u64, hash40("sword1"), 0, 0, 0, 0, 0, -90, 1, true)
		}
		frame(Frame=18)
		if(is_excute){
			AFTER_IMAGE_OFF(5)
		}
		frame(Frame=20)
		if(is_excute){
			EFFECT_OFF_KIND(0x118d3781c4u64, false, true)
		}
	});
}	
#[acmd_script(
    agent = "master",
    scripts =  ["effect_specialsfdash", "effect_specialairsfdash"],
    category = ACMD_EFFECT)]
unsafe fn byleth_sidebedash(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		frame(Frame=2)
		if(is_excute){
			FOOT_EFFECT(hash40("sys_dash_smoke"), hash40("top"), 0, 0, 0, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, false)
		}
		frame(Frame=10)
		if(is_excute){
			AFTER_IMAGE4_ON_arg29(0x11c86cd79du64, 0x1151658627u64, 5, hash40("sword1"), 2.3, 0, 0, hash40("sword1"), 17, 0, 0.15, true, hash40("null"), hash40("sword1"), 0, 0, 0, 0, 0, -90, 1, 0, EFFECT_AXIS_X, 0, TRAIL_BLEND_ALPHA, 101, TRAIL_CULL_NONE, 1.4, 0.1)
			EFFECT_FOLLOW(0x118d3781c4u64, hash40("sword1"), 0, 0, 0, 0, 0, -90, 1, true)
		}
		frame(Frame=18)
		if(is_excute){
			AFTER_IMAGE_OFF(5)
		}
		frame(Frame=20)
		if(is_excute){
			EFFECT_OFF_KIND(0x118d3781c4u64, false, true)
		}
	});
}		
#[acmd_script(
    agent = "master",
    scripts =  ["game_specialsstart", "game_specialairsstart"],
    category = ACMD_GAME)]
unsafe fn byleth_sideb_start(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		FT_MOTION_RATE(FSM=0.4)
	});
}						
#[acmd_script(
    agent = "master",
    scripts =  ["effect_specialsstart", "effect_specialairsstart"],
    category = ACMD_EFFECT)]
unsafe fn byleth_sideb_starte(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
	});
}				
#[acmd_script(
    agent = "master_arrow1",
    script =  "effect_fly",
    category = ACMD_EFFECT)]
unsafe fn byleth_arrow_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		for(10 Iterations){
			if(is_excute){
				rust {
					let scale = smash::phx::Vector3f { x: 0.75, y: 0.75, z: 0.75};
					EffectModule::set_scale_last(fighter.module_accessor, &scale);
				}
				EFFECT(hash40("sys_fireflower_shot"), hash40("top"), 0, 0, 0, 0, 0, 0, 1.25, 0, 0, 0, 0, 0, 0, true)
				EffectModule::set_rate_last(0.75)
			}
			wait(Frames=1)
		}
		wait(Frames=1)
		if(is_excute){
			rust {
				let scale = smash::phx::Vector3f { x: 0.75, y: 0.75, z: 0.75};
				EffectModule::set_scale_last(fighter.module_accessor, &scale);
			}
		}
	});
}
#[acmd_script(
    agent = "master_arrow1",
    script =  "game_fly",
    category = ACMD_GAME)]
unsafe fn byleth_arrow(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		if(is_excute){
			ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=9.0, Angle=361, KBG=70, FKB=0, BKB=20, Size=4.0, X=0.0, Y=0.0, Z=-1.5, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.8, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=5, Trip=0.0, Rehit=0, Reflectable=true, Absorbable=true, Flinchless=false, DisableHitlag=false, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_fire"), SFXLevel=ATTACK_SOUND_LEVEL_L,  SFXType=COLLISION_SOUND_ATTR_FIRE, Type=ATTACK_REGION_OBJECT)
			AttackModule::enable_safe_pos()
		}
	});
}							
#[acmd_script(
    agent = "master",
    scripts =  ["game_specialn", "game_specialairn"],
    category = ACMD_GAME)]
unsafe fn byleth_neutralb(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		FT_MOTION_RATE(FSM=1)
		if(is_excute){
			WorkModule::on_flag(Flag=FIGHTER_MASTER_STATUS_SPECIAL_N_FLAG_ENABLE_CONTROL_ENERGY)
		}
		frame(frame=9)
		if(is_excute){
			rust {
				let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);   
				let ENTRY_ID = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
				if ArticleModule::is_exist(boma, *FIGHTER_MASTER_GENERATE_ARTICLE_ARROW1) == false && IS_THUNDER[ENTRY_ID] == false {
					ArticleModule::generate_article(boma, *FIGHTER_MASTER_GENERATE_ARTICLE_ARROW1,false,0);
				};
			}
		}
		frame(frame=10)
		if(is_excute){
			rust {
				let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);   
				let ENTRY_ID = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
				if IS_THUNDER[ENTRY_ID] == false {
					ArticleModule::shoot_exist(boma, *FIGHTER_MASTER_GENERATE_ARTICLE_ARROW1, smash::app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL), false);
				} else {
					macros::ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 6.0, /*Angle*/ 270, /*KBG*/ 95, /*FKB*/ 0, /*BKB*/ 40, /*Size*/ 3.6, /*X*/ 0.0, /*Y*/ 55.0, /*Z*/ 28.0, /*X2*/ Some(0.0), /*Y2*/ Some(40.0), /*Z2*/ Some(28.0), /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 3, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ true, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_elec"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_ELEC, /*Type*/ *ATTACK_REGION_ENERGY);
					AttackModule::enable_safe_pos(boma);
					macros::ATTACK(fighter, /*ID*/ 2, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 6.0, /*Angle*/ 80, /*KBG*/ 98, /*FKB*/ 0, /*BKB*/ 40, /*Size*/ 3.6, /*X*/ 0.0, /*Y*/ 40.0, /*Z*/ 28.0, /*X2*/ Some(0.0), /*Y2*/ Some(4.0), /*Z2*/ Some(28.0), /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 3, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ true, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_elec"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_ELEC, /*Type*/ *ATTACK_REGION_ENERGY);
					AttackModule::enable_safe_pos(boma);
					macros::ATTACK(fighter, /*ID*/ 	0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 12.0, /*Angle*/ 361, /*KBG*/ 90, /*FKB*/ 0, /*BKB*/ 40, /*Size*/ 6.0, /*X*/ 0.0, /*Y*/ 5.0, /*Z*/ 28.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.5, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 3, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ true, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_fire"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_ELEC, /*Type*/ *ATTACK_REGION_ENERGY);
					AttackModule::enable_safe_pos(boma);
				};
			}
		}
		frame(frame=14)
		if(is_excute){
			AttackModule::clear_all()
			rust {
				let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);   
				let ENTRY_ID = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
				if IS_THUNDER[ENTRY_ID] == false {
					IS_THUNDER[ENTRY_ID] = true;
				} else {
					IS_THUNDER[ENTRY_ID] = false;
				};
			}
		}
	});
}		
#[acmd_script(
    agent = "master",
    scripts =  ["effect_specialn", "effect_specialairn"],
    category = ACMD_EFFECT)]
unsafe fn byleth_neutralb_eff(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		frame(frame=9)
		if(is_excute){
			rust {
				let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);  
				let ENTRY_ID = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize; 
				if IS_THUNDER[ENTRY_ID] == true {
					if StatusModule::situation_kind(boma) == *SITUATION_KIND_AIR {
						macros::EFFECT_FOLLOW(fighter, Hash40::new("sys_thunder"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 0.75, true);
					} else {
						macros::EFFECT(fighter, Hash40::new("sys_thunder"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 0.75, 0, 0, 0, 0, 0, 0, true);
					};
					macros::EFFECT(fighter, Hash40::new("sys_thunder_flash"), Hash40::new("top"), 0, 0, 25, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
				}
			}
		}
	});
}	
#[acmd_script(
    agent = "master",
    scripts =  ["sound_specialn", "sound_specialairn"],
    category = ACMD_SOUND)]
unsafe fn byleth_neutralb_snd(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		frame(frame=10)
		if(is_excute){
			rust {
				let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);  
				let ENTRY_ID = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize; 
				if IS_THUNDER[ENTRY_ID] == true {
					macros::PLAY_SE(fighter, Hash40::new("se_common_smashswing_02"));
					macros::PLAY_SE(fighter, Hash40::new("se_common_down_m_02"));
					macros::PLAY_SE(fighter, Hash40::new("se_common_electric_hit_s"));
				};
				macros::PLAY_SEQUENCE(fighter, Hash40::new("seq_master_rnd_attack02"));
			}
		}
	});
}				
#[acmd_script(
    agent = "master",
    script =  "expression_specialairn",
    category = ACMD_EXPRESSION)]
unsafe fn byleth_air_neutralb_exp(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
	});
}			
	
#[acmd_script(
    agent = "master",
    scripts =  ["game_specialnstart", "game_specialairnstart"],
    category = ACMD_GAME)]
unsafe fn byleth_neutralb_start(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		if(is_excute){
			rust {
				let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent); 
				if smash::app::utility::get_kind(boma) == *FIGHTER_KIND_MASTER {
					StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_MASTER_STATUS_KIND_SPECIAL_N_SHOOT, true);
				} else {
					StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_KIRBY_STATUS_KIND_MASTER_SPECIAL_N_SHOOT, true);
				};
			}
		}
	});
}									
#[acmd_script(
    agent = "master",
    script =  "game_specialnmax",
    category = ACMD_GAME)]
unsafe fn byleth_neutralbmax(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		if(is_excute){
			CancelModule::enable_cancel()
		}
	});
}										
#[acmd_script(
    agent = "master",
    script =  "effect_specialnmax",
    category = ACMD_EFFECT)]
unsafe fn byleth_neutralbmaxe(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
	});
}											
#[acmd_script(
    agent = "master",
    script =  "sound_specialnmax",
    category = ACMD_SOUND)]
unsafe fn byleth_neutralbmaxs(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
	});
}										
#[acmd_script(
    agent = "master",
    script =  "game_specialairnmax",
    category = ACMD_GAME)]
unsafe fn byleth_neutralbmaxair(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		if(is_excute){
			CancelModule::enable_cancel()
		}
	});
}											
#[acmd_script(
    agent = "master",
    script =  "effect_specialairnmax",
    category = ACMD_EFFECT)]
unsafe fn byleth_neutralbmaxaire(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
	});
}											
#[acmd_script(
    agent = "master",
    script =  "sound_specialairnmax",
    category = ACMD_SOUND)]
unsafe fn byleth_neutralbmaxairs(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
	});
}			
#[acmd_script(
    agent = "master_axe",
    scripts =  ["game_speciallw", "game_specialairlw"],
    category = ACMD_GAME)]
unsafe fn axe_downb(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		FT_MOTION_RATE(FSM=0.04761904)
		if(is_excute){
			WorkModule::set_int(6, WEAPON_MASTER_AXE_INSTANCE_WORK_ID_INT_CRITICAL_ATTACK_ID)
		}
		frame(Frame=42)
		FT_MOTION_RATE(FSM=1)
		frame(Frame=61)
		if(is_excute){
			ATTACK(ID=0, Part=0, Bone=hash40("haver"),  Damage=16.8, Angle=361, KBG=80, FKB=0, BKB=50, Size=6.7, X=0.0, Y=14.0, Z=1.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.45, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_THRU, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_MASTER_AXE, Type=ATTACK_REGION_OBJECT)
		}
		frame(Frame=67)
		if(is_excute){
			AttackModule::clear_all()
		}
	});
}		
#[acmd_script(
    agent = "master_axe",
    scripts =  ["effect_attacklw4"],
    category = ACMD_EFFECT)]
unsafe fn axe_dsmash_eff(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		frame(Frame=1)
		if(is_excute){
			EFFECT_FOLLOW_ALPHA(hash40("master_axeflare1"), hash40("blade1"), 0, 0, 0, 0, 0, 0, 1, true, 0.58)
			EFFECT_FOLLOW_ALPHA(hash40("master_axeflare2"), hash40("blade2"), 0, 0, 0, 0, 0, 0, 1, true, 0.58)
			EFFECT_FOLLOW_ALPHA(hash40("master_axeflare3"), hash40("axe"), 0, 0, 0, 0, 0, 0, 1, true, 0.58)
			EFFECT_FOLLOW_ALPHA(hash40("master_axe_hold"), hash40("blade2"), 0, 0, 0, 0, 0, 0, 0.8, true, 0.75)
		}
		frame(Frame=5)
		if(is_excute){
			EFFECT_OFF_KIND(hash40("master_axe_hold"), false, true)
		}
		frame(Frame=15)
		if(is_excute){
			EFFECT_FOLLOW_ALPHA(0x1067610ee8u64, hash40("blade1"), 0, 0, 0, 0, 0, 0, 1, true, 0.8)
			EFFECT_FOLLOW_ALPHA(0x10fe685f52u64, hash40("blade2"), 0, 0, 0, 0, 0, 0, 1, true, 0.8)
			EFFECT_FOLLOW_ALPHA(0x10896f6fc4u64, hash40("axe"), 0, 0, 0, 0, 0, 0, 1, true, 0.8)
		}
		frame(Frame=25)
		if(is_excute){
			EFFECT_OFF_KIND(0x1067610ee8u64, true, true)
			EFFECT_OFF_KIND(0x10fe685f52u64, true, true)
			EFFECT_OFF_KIND(0x10896f6fc4u64, true, true)
			EFFECT_FOLLOW_ALPHA(0x1067610ee8u64, hash40("blade1"), 0, 0, 0, 0, 0, 0, 1, true, 0.8)
			EFFECT_FOLLOW_ALPHA(0x10fe685f52u64, hash40("blade2"), 0, 0, 0, 0, 0, 0, 1, true, 0.8)
			EFFECT_FOLLOW_ALPHA(0x10896f6fc4u64, hash40("axe"), 0, 0, 0, 0, 0, 0, 1, true, 0.8)
		}
		frame(Frame=37)
		if(is_excute){
			EFFECT_OFF_KIND(hash40("master_axeflare1"), false, true)
			EFFECT_OFF_KIND(hash40("master_axeflare2"), false, true)
			EFFECT_OFF_KIND(hash40("master_axeflare3"), false, true)
			EFFECT_OFF_KIND(0x10fe685f52u64, false, true)
			EFFECT_OFF_KIND(0x10896f6fc4u64, false, true)
			EFFECT_OFF_KIND(0x1067610ee8u64, false, true)
			EFFECT_OFF_KIND(0x10fe685f52u64, false, true)
			EFFECT_OFF_KIND(0x10896f6fc4u64, false, true)
		}
	});
}	
#[acmd_script(
    agent = "master",
    scripts =  ["effect_attacklw3"],
    category = ACMD_EFFECT)]
unsafe fn byleth_dtilt_eff(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		frame(Frame=11)
		if(is_excute){
			EFFECT_FOLLOW(0x130d5298d9u64, hash40("top"), 0, 6, 0, 15, 0, 0, 0.95, true)
			LAST_EFFECT_SET_RATE(0.8)
		}
		frame(Frame=13)
		if(is_excute){
			EFFECT(0x14f1b6f422u64, hash40("top"), 24, 0, 0, 0, 0, 0, 0.5, 0, 0, 0, 0, 0, 0, false)
			rust{
				let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent); 
				if GroundModule::ray_check(boma, &Vector2f{ x: PostureModule::pos_x(boma)+(24.0*PostureModule::lr(boma)), y: PostureModule::pos_y(boma)}, &Vector2f{ x: 0.0, y: -1.0}, true) == 1 {
					acmd!(lua_state, {
						LANDING_EFFECT(0x15d7fa6259u64, hash40("top"), 24, 0, 0, 0, 0, 0, 0.5, 0.5, 0, 0.5, 0, 0, 0, false)
					});
				}
			}
		}
	});
}			
#[acmd_script(
    agent = "master",
    scripts =  ["game_speciallw", "game_specialairlw"],
    category = ACMD_GAME)]
unsafe fn byleth_downb(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		FT_MOTION_RATE(FSM=0.04761904)
		if(is_excute){
			WorkModule::on_flag(Flag=FIGHTER_MASTER_STATUS_SPECIAL_LW_FLAG_INHERIT_LANDING_1)
		}
		frame(Frame=42)
		FT_MOTION_RATE(FSM=1)
		frame(Frame=51)
		if(is_excute){
			WorkModule::off_flag(Flag=FIGHTER_MASTER_STATUS_SPECIAL_LW_FLAG_INHERIT_LANDING_1)
		}
		frame(Frame=60)
		if(is_excute){
			WorkModule::on_flag(Flag=FIGHTER_MASTER_STATUS_SPECIAL_LW_FLAG_FORBID_LANDING)
		}
		frame(Frame=64)
		if(is_excute){
			ArticleModule::set_flag(FIGHTER_MASTER_GENERATE_ARTICLE_AXE, true, WEAPON_PIERCE_INSTANCE_WORK_ID_FLAG_PIERCE_GROUND)
		}
		frame(Frame=65)
		if(is_excute){
			ArticleModule::set_flag(FIGHTER_MASTER_GENERATE_ARTICLE_AXE, false, WEAPON_PIERCE_INSTANCE_WORK_ID_FLAG_PIERCE_GROUND)
			WorkModule::off_flag(Flag=FIGHTER_MASTER_STATUS_SPECIAL_LW_FLAG_FORBID_LANDING)
			WorkModule::on_flag(Flag=FIGHTER_MASTER_STATUS_SPECIAL_LW_FLAG_INHERIT_LANDING_2)
		}
		frame(Frame=96)
		if(is_excute){
			WorkModule::on_flag(Flag=FIGHTER_MASTER_STATUS_SPECIAL_LW_FLAG_CONTROL_ENERGY)
			WorkModule::on_flag(Flag=FIGHTER_MASTER_STATUS_SPECIAL_LW_FLAG_REVERT_FALL_SPEED)
			CancelModule::enable_cancel()
		}
		frame(Frame=117)
		if(is_excute){
			WorkModule::off_flag(Flag=FIGHTER_MASTER_STATUS_SPECIAL_LW_FLAG_INHERIT_LANDING_2)
		}
		frame(Frame=118)
		if(is_excute){
			ArticleModule::remove_exist(FIGHTER_MASTER_GENERATE_ARTICLE_AXE,smash::app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL))
		}
	});
}			
#[acmd_script(
    agent = "master",
    script =  "game_throwhi",
    category = ACMD_GAME)]
unsafe fn byleth_uthrow(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		if(is_excute){
			ATTACK_ABS(Kind=FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, ID=0, Damage=6.0, Angle=90, KBG=120, FKB=0, BKB=50, Hitlag=0.0, Unk=1.0, FacingRestrict=ATTACK_LR_CHECK_F, Unk=0.0, Unk=true, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_NONE, Type=ATTACK_REGION_THROW)
			ATTACK_ABS(Kind=FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, ID=0, Damage=3.0, Angle=361, KBG=100, FKB=0, BKB=40, Hitlag=0.0, Unk=1.0, FacingRestrict=ATTACK_LR_CHECK_F, Unk=0.0, Unk=true, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_NONE, Type=ATTACK_REGION_THROW)
		}
		frame(Frame=15)
		if(is_excute){
			ATK_HIT_ABS(FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, hash40("throw"), WorkModule::get_int64(module_accessor,*FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT), WorkModule::get_int64(module_accessor,*FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP), WorkModule::get_int64(module_accessor,*FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO))
			AttackModule::clear_all()
		}
    });
}		
#[acmd_script(
    agent = "master",
    script =  "effect_throwhi",
    category = ACMD_EFFECT)]
unsafe fn byleth_uthrow_eff(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		frame(Frame=26)
		if(is_excute){
			LANDING_EFFECT(hash40("sys_down_smoke"), hash40("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false)
		}
    });
}		
#[acmd_script(
    agent = "master",
    script =  "sound_throwhi",
    category = ACMD_SOUND)]
unsafe fn byleth_uthrow_snd(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		frame(Frame=3)
		if(is_excute){
			PLAY_SE(hash40("se_common_throw_01"))
		}
		wait(Frames=7)
		if(is_excute){
			PLAY_SE(hash40("se_common_throw_02"))
			PLAY_SEQUENCE(hash40("seq_master_rnd_attack01"))
		}
    });
}			
#[fighter_frame( agent = FIGHTER_KIND_MASTER )]
fn master_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent); 
		let status_kind = smash::app::lua_bind::StatusModule::status_kind(boma);
		let ENTRY_ID = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
		if MotionModule::motion_kind(boma) == hash40("special_air_s_front") || MotionModule::motion_kind(boma) == hash40("special_air_s_front_dash") || MotionModule::motion_kind(boma) == hash40("special_s_front") || MotionModule::motion_kind(boma) == hash40("special_s_front_dash") || MotionModule::motion_kind(boma) == hash40("special_s_start") || MotionModule::motion_kind(boma) == hash40("special_air_s_start") {
			ArticleModule::remove_exist(boma, *FIGHTER_MASTER_GENERATE_ARTICLE_SPEAR,smash::app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
		};
		if smash::app::sv_information::is_ready_go() == false {
			IS_THUNDER[ENTRY_ID] = false;
		};
		if MotionModule::frame(boma) > 4.0 && MotionModule::motion_kind(boma) == hash40("attack_air_n") {
			if AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_ALL) {
				println!("Close Nair triggered");
			} else {
				AttackModule::clear_all(boma);
				MotionModule::change_motion(boma, smash::phx::Hash40::new("attack_air_n2"), 6.0, 1.0, false, 0.0, false, false);
				ArticleModule::remove_exist(boma, *FIGHTER_MASTER_GENERATE_ARTICLE_BOW,smash::app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
				println!("Far Nair triggered");
			};
		}
		if [*FIGHTER_MASTER_STATUS_KIND_SPECIAL_N_HOLD, *FIGHTER_STATUS_KIND_SPECIAL_N].contains(&status_kind){
			StatusModule::change_status_request_from_script(boma, *FIGHTER_MASTER_STATUS_KIND_SPECIAL_N_SHOOT, true);
		};
		if MotionModule::motion_kind(boma) == hash40("special_s_front") || MotionModule::motion_kind(boma) == hash40("special_air_s_front")|| MotionModule::motion_kind(boma) == hash40("special_air_s_front_dash")|| MotionModule::motion_kind(boma) == hash40("special_s_front_dash"){
			if AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT) && ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_ATTACK) && ControlModule::check_button_off(boma, *CONTROL_PAD_BUTTON_SPECIAL){
				CancelModule::enable_cancel(boma);
			};
		};
		if [*FIGHTER_STATUS_KIND_SPECIAL_LW].contains(&status_kind) {
			if MotionModule::frame(boma) < 3.0 {
				if (ControlModule::get_stick_x(boma)*PostureModule::lr(boma)) < -0.2 {
					PostureModule::reverse_lr(boma);
					PostureModule::update_rot_y_lr(boma);
				};
			};
		};
		if [*FIGHTER_MASTER_STATUS_KIND_SPECIAL_N_SHOOT].contains(&status_kind) {
			if motion_duration(boma) == 2 {
				if (ControlModule::get_stick_x(boma)*PostureModule::lr(boma)) < -0.2 {
					PostureModule::reverse_lr(boma);
					PostureModule::update_rot_y_lr(boma);
					let stop_rise  = smash::phx::Vector3f { x: -1.0, y: 1.0, z: 1.0 };
					KineticModule::mul_speed(boma, &stop_rise, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
				};
			};
		};
		if [hash40("special_lw_landing_1"), hash40("special_lw_landing_2")].contains(&MotionModule::motion_kind(boma)) && motion_duration(boma) > 14 {
			CancelModule::enable_cancel(boma);
		};
		if [hash40("special_lw_hit"), hash40("special_air_lw_hit")].contains(&MotionModule::motion_kind(boma)) && motion_duration(boma) > 20 {
			if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_JUMP) {
				StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_JUMP_AERIAL, true);
			};
		};
    }
}			
#[fighter_frame( agent = FIGHTER_KIND_KIRBY )]
fn master_kirby_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent); 
		let ENTRY_ID = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
		if smash::app::sv_information::is_ready_go() == false {
			IS_THUNDER[ENTRY_ID] = false;
		};
	}
}

pub fn install() {
    smashline::install_acmd_scripts!(
		byleth_dtilt,
		byleth_dtilt_eff,
		byleth_nair,
		byleth_naire,
		byleth_nair2,
		byleth_nair2e,
		byleth_land_nair,
		byleth_sideb,
		byleth_sidebdash,
		byleth_sidebe,
		byleth_sidebedash,
		byleth_sideb_start,
		byleth_sideb_starte,
		byleth_neutralb_start,
		byleth_neutralb,
		byleth_neutralb_eff,
		byleth_neutralb_snd,
		byleth_neutralbmax,
		byleth_neutralbmaxe,
		byleth_neutralbmaxs,
		byleth_neutralbmaxair,
		byleth_neutralbmaxaire,
		byleth_neutralbmaxairs,
		byleth_arrow_effect,
		byleth_arrow,
		byleth_downb,
		axe_downb,
		axe_dsmash_eff,
		byleth_air_neutralb_exp,
		byleth_fair,
		byleth_nair2_sound,
		byleth_bair,
		byleth_nair_sound,
		byleth_sideb_snd,
		byleth_uthrow,
		byleth_uthrow_eff,
		byleth_uthrow_snd
    );
    smashline::install_agent_frames!(
        master_frame,
		master_kirby_frame
    );
}
