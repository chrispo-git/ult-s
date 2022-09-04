use smash::hash40;
use smash::lib::lua_const::*;
use smash::lua2cpp::*;
use smashline::*;
use smash_script::*;
use smash::app::lua_bind::*;
use smash::phx::Hash40;

//Joker Gun Cancel Constants 
const NONE : i32 = 100;
const ATTACK : i32 = 101;
const ATTACK_S3 : i32 = 104;
const ATTACK_HI3 : i32 = 105;
const ATTACK_LW3 : i32 = 106;
const ATTACK_S4 : i32 = 107;
const ATTACK_HI4 : i32 = 108;
const ATTACK_LW4 : i32 = 110;
const ATTACK_AIR_N : i32 = 111;
const ATTACK_AIR_F : i32 = 112;
const ATTACK_AIR_B : i32 = 113;
const ATTACK_AIR_HI : i32 = 114;
const ATTACK_AIR_LW : i32 = 115;

//Joker Gun Cancel 
static mut GUN_C: [i32; 8] = [100; 8];
static mut IS_ARSENE: [bool; 8] = [false; 8];
static mut X: [f32; 8] = [0.0; 8];
static mut Y: [f32; 8] = [0.0; 8];

#[acmd_script(
    agent = "jack",
    scripts =  ["game_specials1", "game_specialairs1"],
    category = ACMD_GAME,
	low_priority)]
unsafe fn joker_eiha(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		frame(Frame=2)
		FT_MOTION_RATE(FSM=0.5)
		wait(Frames=4)
		FT_MOTION_RATE(FSM=1)
		frame(Frame=16)
		if(is_excute){
			ArticleModule::generate_article(FIGHTER_JACK_GENERATE_ARTICLE_FIRE, false, 0)
		}
    });
}
#[acmd_script(
    agent = "jack",
    scripts =  ["game_attackairhi"],
    category = ACMD_GAME,
	low_priority)]
unsafe fn joker_uair(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent); 
	let status_kind = smash::app::lua_bind::StatusModule::status_kind(boma);
	let ENTRY_ID = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    acmd!(lua_state, {
		frame(Frame=5)
		if(is_excute){
			WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
			ATTACK(ID=0, Part=0, Bone=hash40("kneer"), Damage=0.75, Angle=80, KBG=80, FKB=0, BKB=20, Size=4.0, X=4.7, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.7, SDI=1.2, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=4, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_G, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
			ATTACK(ID=1, Part=0, Bone=hash40("kneer"), Damage=0.75, Angle=367, KBG=100, FKB=0, BKB=30, Size=4.0, X=4.7, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.7, SDI=1.2, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=4, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_A, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
		}
		frame(Frame=19)
		if(is_excute){
			AttackModule::clear_all()
		}
		frame(Frame=20)
		if(is_excute){
			ATTACK(ID=0, Part=0, Bone=hash40("kneer"), Damage=3.0, Angle=80, KBG=155, FKB=0, BKB=50, Size=4.5, X=4.8, Y=0.0, Z=0.0, X2=2.5, Y2=0.0, Z2=0.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
			rust {
				if WorkModule::is_flag(boma, *FIGHTER_JACK_INSTANCE_WORK_ID_FLAG_DOYLE) {
					acmd!(lua_state, {
						ATTACK(ID=0, Part=0, Bone=hash40("kneer"), Damage=3.0, Angle=80, KBG=155, FKB=0, BKB=50, Size=5.0, X=4.3, Y=0.0, Z=0.0, X2=1.5, Y2=0.0, Z2=0.0, Hitlag=1.7, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
						ATTACK(ID=1, Part=1, Bone=hash40("top"), Damage=5.0, Angle=80, KBG=140, FKB=0, BKB=50, Size=5.0, X=0.0, Y=20.0, Z=1.5, X2=0.0, Y2=24.0, Z2=0.0, Hitlag=1.7, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_HEAVY, Type=ATTACK_REGION_PUNCH)
					});
				}
			}
		}
		frame(Frame=23)
		if(is_excute){
			AttackModule::clear_all()
		}
    });
}		
#[acmd_script(
    agent = "jack",
    scripts =  ["game_attackairf"],
    category = ACMD_GAME,
	low_priority)]
unsafe fn joker_fair(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent); 
	let status_kind = smash::app::lua_bind::StatusModule::status_kind(boma);
	let ENTRY_ID = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    acmd!(lua_state, {
		frame(Frame=4)
		if(is_excute){
			WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
		}
		frame(Frame=7)
		if(is_excute){
			ATTACK(ID=0, Part=0, Bone=hash40("legr"), Damage=2.0, Angle=367, KBG=25, FKB=0, BKB=70, Size=4.0, X=3.2, Y=0.0, Z=0.0, X2=3.2, Y2=-0.5, Z2=1.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_A, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
			ATTACK(ID=1, Part=0, Bone=hash40("legr"), Damage=2.0, Angle=76, KBG=50, FKB=0, BKB=73, Size=4.0, X=3.2, Y=0.0, Z=0.0, X2=3.2, Y2=0.0, Z2=1.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_G, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
			ATTACK(ID=2, Part=0, Bone=hash40("kneer"), Damage=2.0, Angle=367, KBG=5, FKB=0, BKB=70, Size=4.0, X=4.2, Y=-0.7, Z=1.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_A, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
			ATTACK(ID=3, Part=0, Bone=hash40("kneer"), Damage=2.0, Angle=80, KBG=50, FKB=0, BKB=73, Size=4.0, X=4.2, Y=-0.7, Z=1.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_G, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
			ATTACK(ID=4, Part=0, Bone=hash40("top"), Damage=2.0, Angle=367, KBG=25, FKB=0, BKB=70, Size=4.0, X=0.0, Y=8.5, Z=10.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_A, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
			ATTACK(ID=5, Part=0, Bone=hash40("top"), Damage=2.0, Angle=76, KBG=50, FKB=0, BKB=73, Size=4.0, X=0.0, Y=8.5, Z=10.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_G, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
		}
		frame(Frame=8)
		if(is_excute){
			AttackModule::clear(ID=4, false)
			AttackModule::clear(ID=5, false)
		}
		frame(Frame=9)
		if(is_excute){
			AttackModule::clear_all()
		}
		frame(Frame=12)
		if(is_excute){
			ATTACK(ID=0, Part=0, Bone=hash40("kneel"), Damage=5.0, Angle=361, KBG=125, FKB=0, BKB=46, Size=3.5, X=4.4, Y=-0.7, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
			ATTACK(ID=1, Part=0, Bone=hash40("legl"), Damage=5.0, Angle=361, KBG=125, FKB=0, BKB=46, Size=3.5, X=3.2, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
			rust {
				if WorkModule::is_flag(boma, *FIGHTER_JACK_INSTANCE_WORK_ID_FLAG_DOYLE) {
					acmd!(lua_state, {
						AttackModule::clear(ID=1, false)
						ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=1.0, Angle=34, KBG=148, FKB=0, BKB=48, Size=6.0, X=0.0, Y=13.5, Z=5.0, X2=0.0, Y2=13.5, Z2=7.5, Hitlag=1.9, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_HEAVY, Type=ATTACK_REGION_KICK)
						ATTACK(ID=1, Part=1, Bone=hash40("top"), Damage=8.0, Angle=34, KBG=98, FKB=0, BKB=58, Size=6.0, X=0.0, Y=13.5, Z=5.0, X2=0.0, Y2=13.5, Z2=7.5, Hitlag=1.6, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_HEAVY, Type=ATTACK_REGION_KICK)
						ATTACK(ID=2, Part=1, Bone=hash40("top"), Damage=8.0, Angle=34, KBG=98, FKB=0, BKB=58, Size=7.0, X=0.0, Y=18.0, Z=5.0, X2=0.0, Y2=17.0, Z2=6.5, Hitlag=1.6, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_HEAVY, Type=ATTACK_REGION_KICK)
					});
				}
			}
		}
		frame(Frame=16)
		if(is_excute){
			AttackModule::clear_all()
		}
	});
}		
#[acmd_script(
    agent = "jack",
    scripts =  ["game_attackairb"],
    category = ACMD_GAME,
	low_priority)]
unsafe fn joker_bair(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent); 
	let status_kind = smash::app::lua_bind::StatusModule::status_kind(boma);
	let ENTRY_ID = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    acmd!(lua_state, {
		frame(Frame=4)
		if(is_excute){
			WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
		}
		frame(Frame=7)
		if(is_excute){
			ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=9.0, Angle=43, KBG=90, FKB=0, BKB=54, Size=2.5, X=0.0, Y=3.3, Z=-6.0, X2=0.0, Y2=10.7, Z2=-8.4, Hitlag=0.8, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_B, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
			ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=9.0, Angle=43, KBG=90, FKB=0, BKB=54, Size=2.5, X=0.0, Y=4.7, Z=-11.1, X2=0.0, Y2=11.1, Z2=-14.2, Hitlag=0.8, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_B, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
			ATTACK(ID=2, Part=0, Bone=hash40("top"), Damage=9.0, Angle=43, KBG=90, FKB=0, BKB=54, Size=3.0, X=0.0, Y=4.0, Z=-3.0, X2=0.0, Y2=8.5, Z2=-5.0, Hitlag=0.8, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_B, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
			rust {
				if WorkModule::is_flag(boma, *FIGHTER_JACK_INSTANCE_WORK_ID_FLAG_DOYLE) {
					acmd!(lua_state, {
						ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=4.0, Angle=35, KBG=102, FKB=0, BKB=51, Size=5.0, X=0.0, Y=10.5, Z=-12.5, X2=0.0, Y2=15.0, Z2=-12.5, Hitlag=1.3, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_B, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
						ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=4.0, Angle=35, KBG=102, FKB=0, BKB=51, Size=5.0, X=0.0, Y=7.5, Z=-10.0, X2=0.0, Y2=5.5, Z2=-8.5, Hitlag=1.3, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_B, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
						ATTACK(ID=2, Part=1, Bone=hash40("top"), Damage=7.0, Angle=35, KBG=100, FKB=0, BKB=58, Size=5.0, X=0.0, Y=7.5, Z=-10.0, X2=0.0, Y2=5.5, Z2=-8.5, Hitlag=1.4, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_B, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_HEAVY, Type=ATTACK_REGION_PUNCH)
						ATTACK(ID=3, Part=1, Bone=hash40("top"), Damage=7.0, Angle=35, KBG=100, FKB=0, BKB=58, Size=5.0, X=0.0, Y=10.5, Z=-12.5, X2=0.0, Y2=15.0, Z2=-12.5, Hitlag=1.4, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_B, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_HEAVY, Type=ATTACK_REGION_PUNCH)
						ATTACK(ID=4, Part=1, Bone=hash40("top"), Damage=7.0, Angle=35, KBG=100, FKB=0, BKB=58, Size=5.0, X=0.0, Y=16.5, Z=-12.0, X2=0.0, Y2=20.5, Z2=-9.0, Hitlag=1.4, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_B, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_HEAVY, Type=ATTACK_REGION_PUNCH)
					});
				}
			}
		}
		wait(Frames=1)
		if(is_excute){
			AttackModule::clear_all()
		}
		frame(Frame=30)
		if(is_excute){
			WorkModule::off_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
		}
    });
}		
		
#[acmd_script(
    agent = "jack",
    scripts =  ["game_attacks4"],
    category = ACMD_GAME,
	low_priority)]
unsafe fn joker_fsmash(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent); 
	let status_kind = smash::app::lua_bind::StatusModule::status_kind(boma);
	let ENTRY_ID = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    acmd!(lua_state, {
		frame(Frame=6)
		if(is_excute){
			WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD)
		}
		frame(Frame=16)
		if(is_excute){
			ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=14.0, Angle=361, KBG=5, FKB=0, BKB=60, Size=2.0, X=0.0, Y=10.0, Z=12.0, X2=0.0, Y2=6.0, Z2=12.0, Hitlag=0.8, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
			ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=14.0, Angle=361, KBG=5, FKB=0, BKB=60, Size=4.0, X=0.0, Y=9.5, Z=9.0, X2=0.0, Y2=8.0, Z2=9.0, Hitlag=0.8, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
			rust {
				if WorkModule::is_flag(boma, *FIGHTER_JACK_INSTANCE_WORK_ID_FLAG_DOYLE) {
					acmd!(lua_state, {
						ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=10.0, Angle=361, KBG=5, FKB=0, BKB=60, Size=2.0, X=0.0, Y=10.0, Z=12.0, X2=0.0, Y2=6.0, Z2=12.0, Hitlag=0.8, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
						ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=10.0, Angle=361, KBG=5, FKB=0, BKB=60, Size=4.0, X=0.0, Y=9.5, Z=9.0, X2=0.0, Y2=8.0, Z2=9.0, Hitlag=0.8, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
						ATTACK(ID=2, Part=1, Bone=hash40("top"), Damage=8.0, Angle=361, KBG=60, FKB=0, BKB=70, Size=3.0, X=0.0, Y=14.0, Z=15.5, X2=0.0, Y2=8.0, Z2=15.5, Hitlag=1.2, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_HEAVY, Type=ATTACK_REGION_NONE)
						ATTACK(ID=3, Part=1, Bone=hash40("top"), Damage=8.0, Angle=361, KBG=60, FKB=0, BKB=70, Size=5.0, X=0.0, Y=8.5, Z=10.0, X2=0.0, Y2=11.5, Z2=10.0, Hitlag=1.2, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_HEAVY, Type=ATTACK_REGION_NONE)
					});
				}
			}
		}
		frame(Frame=17)
		if(is_excute){
			ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=14.0, Angle=361, KBG=5, FKB=0, BKB=60, Size=2.0, X=0.0, Y=8.0, Z=12.0, X2=0.0, Y2=6.0, Z2=12.0, Hitlag=0.8, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
			ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=14.0, Angle=361, KBG=5, FKB=0, BKB=60, Size=4.0, X=0.0, Y=9.5, Z=9.0, X2=0.0, Y2=6.0, Z2=9.0, Hitlag=0.8, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
			rust {
				if WorkModule::is_flag(boma, *FIGHTER_JACK_INSTANCE_WORK_ID_FLAG_DOYLE) {
					acmd!(lua_state, {
						ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=10.0, Angle=361, KBG=5, FKB=0, BKB=60, Size=2.0, X=0.0, Y=10.0, Z=12.0, X2=0.0, Y2=6.0, Z2=12.0, Hitlag=0.8, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
						ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=10.0, Angle=361, KBG=5, FKB=0, BKB=60, Size=4.0, X=0.0, Y=9.5, Z=9.0, X2=0.0, Y2=8.0, Z2=9.0, Hitlag=0.8, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
						ATTACK(ID=2, Part=1, Bone=hash40("top"), Damage=8.0, Angle=361, KBG=60, FKB=0, BKB=70, Size=3.0, X=0.0, Y=14.0, Z=15.5, X2=0.0, Y2=8.0, Z2=15.5, Hitlag=1.2, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_HEAVY, Type=ATTACK_REGION_NONE)
						ATTACK(ID=3, Part=1, Bone=hash40("top"), Damage=8.0, Angle=361, KBG=60, FKB=0, BKB=70, Size=5.0, X=0.0, Y=8.5, Z=10.0, X2=0.0, Y2=11.5, Z2=10.0, Hitlag=1.2, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_HEAVY, Type=ATTACK_REGION_NONE)
					});
				}
			}
		}
		wait(Frames=2)
		if(is_excute){
			AttackModule::clear_all()
		}
    });
}		
#[acmd_script(
    agent = "jack",
    scripts =  ["game_specialn1_ex", "game_specialairn1_ex", "game_specialn1", "game_specialairn1", "game_specialn2", "game_specialairn2", "game_specialn3", "game_specialairn3"],
    category = ACMD_GAME,
	low_priority)]
unsafe fn joker_gun(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent); 
	let status_kind = smash::app::lua_bind::StatusModule::status_kind(boma);
	let ENTRY_ID = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    acmd!(lua_state, {
		if(is_excute){
			WorkModule::on_flag(Flag=FIGHTER_JACK_STATUS_SPECIAL_N_FLAG_UNABLE_SPECIAL_N)
		}
		frame(Frame=12)
		if(is_excute){
			rust {
				if GUN_C[ENTRY_ID] == ATTACK {
					acmd!(lua_state, {ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=5.0, Angle=361, KBG=40, FKB=0, BKB=75, Size=4.5, X=0.0, Y=11.0, Z=9.0, X2=0.0, Y2=11.0, Z2=30.0, Hitlag=0.5, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=-2, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_jack_bullet"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_JACK_SHOT, Type=ATTACK_REGION_OBJECT)});
				} else if GUN_C[ENTRY_ID] == ATTACK_S4 {
					acmd!(lua_state, {ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=5.0, Angle=120, KBG=30, FKB=0, BKB=100, Size=4.5, X=0.0, Y=11.0, Z=9.0, X2=0.0, Y2=11.0, Z2=44.0, Hitlag=0.5, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=-2, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_jack_bullet"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_JACK_SHOT, Type=ATTACK_REGION_OBJECT)});
				} else if GUN_C[ENTRY_ID] == ATTACK_LW4 {
					acmd!(lua_state, {ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=5.0, Angle=361, KBG=30, FKB=0, BKB=55, Size=4.5, X=0.0, Y=11.0, Z=9.0, X2=0.0, Y2=11.0, Z2=50.0, Hitlag=0.5, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=-2, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_jack_bullet"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_JACK_SHOT, Type=ATTACK_REGION_OBJECT)});
				}  else if GUN_C[ENTRY_ID] == ATTACK_AIR_HI {
					acmd!(lua_state, {ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=5.0, Angle=90, KBG=60, FKB=0, BKB=60, Size=14.0, X=0.0, Y=11.0, Z=9.0, X2=0.0, Y2=11.0, Z2=9.0, Hitlag=0.5, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=-2, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_jack_bullet"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_JACK_SHOT, Type=ATTACK_REGION_OBJECT)});
				} else if GUN_C[ENTRY_ID] == ATTACK_AIR_F {
					acmd!(lua_state, {ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=5.0, Angle=50, KBG=30, FKB=0, BKB=5, Size=5.5, X=0.0, Y=11.0, Z=9.0, X2=0.0, Y2=11.0, Z2=30.0, Hitlag=0.5, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=-2, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=true, DisableHitlag=false, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_jack_bullet"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_JACK_SHOT, Type=ATTACK_REGION_OBJECT)});
				} else if GUN_C[ENTRY_ID] == ATTACK_AIR_LW {
					acmd!(lua_state, {ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=5.0, Angle=55, KBG=155, FKB=0, BKB=35, Size=4.5, X=0.0, Y=11.0, Z=9.0, X2=0.0, Y2=11.0, Z2=44.0, Hitlag=0.5, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=-2, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_jack_bullet"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_JACK_SHOT, Type=ATTACK_REGION_OBJECT)});
				} else {
					acmd!(lua_state, {ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=1.0, Angle=361, KBG=30, FKB=0, BKB=5, Size=2.5, X=0.0, Y=11.0, Z=9.0, X2=0.0, Y2=11.0, Z2=50.0, Hitlag=0.5, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=-2, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_jack_bullet"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_JACK_SHOT, Type=ATTACK_REGION_OBJECT)});
				};
				GUN_C[ENTRY_ID] = NONE;
				if WorkModule::is_flag(boma, *FIGHTER_JACK_INSTANCE_WORK_ID_FLAG_DOYLE) {
					acmd!(lua_state, {AttackModule::set_add_reaction_frame(ID=0, Frames=6.0, Unk=false)});
				};
			}
			FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(0, 4, 4)
			FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(1, 4, 4)
			FT_SHOOTING_ATTACK_GROUND_CHECK_NEW_arg5(2, 4, 4, hash40("jack_gun_hit2"), hash40("se_jack_special_n02"))
		}
		wait(Frames=1)
		if(is_excute){
			AttackModule::clear_all()
		}
		wait(Frames=1)
		if(is_excute){
			rust {
				if StatusModule::situation_kind(boma) == *SITUATION_KIND_AIR {
					acmd!(lua_state, {
						WorkModule::on_flag(Flag=FIGHTER_JACK_STATUS_SPECIAL_N_FLAG_CHECK_BARRAGE_TRIGGER)
						WorkModule::on_flag(Flag=FIGHTER_JACK_STATUS_SPECIAL_N_FLAG_CHECK_BARRAGE_BUTTON_ON)
					});
				};
			}
		}
		frame(Frame=22)
		if(is_excute){
			WorkModule::on_flag(Flag=FIGHTER_JACK_STATUS_SPECIAL_N_FLAG_CHECK_NEXT_STATUS)
		}
		frame(Frame=24)
		if(is_excute){
			WorkModule::on_flag(Flag=FIGHTER_JACK_STATUS_SPECIAL_N_FLAG_UNABLE_SPECIAL_N)
		}
    });
}		
#[acmd_script(
    agent = "jack",
    scripts =  ["sound_specialn1_ex", "sound_specialairn1_ex"],
    category = ACMD_SOUND,
	low_priority)]
unsafe fn s_joker_gun(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent); 
	let status_kind = smash::app::lua_bind::StatusModule::status_kind(boma);
	let ENTRY_ID = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    acmd!(lua_state, {
		frame(Frame=12)
		if(is_excute){
			PLAY_SE(hash40("se_jack_special_n01"))
		}
    });
}		
#[acmd_script(
    agent = "jack",
    scripts =  ["effect_specialn1_ex", "effect_specialairn1_ex"],
    category = ACMD_EFFECT,
	low_priority)]
unsafe fn e_joker_gun(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent); 
	let status_kind = smash::app::lua_bind::StatusModule::status_kind(boma);
	let ENTRY_ID = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    acmd!(lua_state, {
		frame(Frame=12)
		if(is_excute){
			EFFECT(0x16b6b8d02du64, hash40("gunl"), 0, 0.8, 1.9, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true)
		}
    });
}		
#[acmd_script(
    agent = "jack",
    scripts =  ["effect_specialnescapeb_ex", "effect_specialairnescapeb_ex"],
    category = ACMD_EFFECT,
	low_priority)]
unsafe fn e_joker_gun_b(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent); 
	let status_kind = smash::app::lua_bind::StatusModule::status_kind(boma);
	let ENTRY_ID = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    acmd!(lua_state, {
		frame(Frame=22)
		if(is_excute){
			EFFECT(0x16b6b8d02du64, hash40("gunl"), 0, 0.8, 1.9, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true)
		}
    });
}	
#[acmd_script(
    agent = "jack",
    scripts =  ["sound_specialnescapeb_ex", "sound_specialairnescapeb_ex"],
    category = ACMD_SOUND,
	low_priority)]
unsafe fn s_joker_gun_b(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent); 
	let status_kind = smash::app::lua_bind::StatusModule::status_kind(boma);
	let ENTRY_ID = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    acmd!(lua_state, {
		frame(Frame=5)
		if(is_excute){
			PLAY_SE(hash40("se_jack_escape"))
			PLAY_SEQUENCE(hash40("seq_jack_rnd_special_n01"))
		}
		frame(Frame=22)
		if(is_excute){
			PLAY_SE(hash40("se_jack_special_n01"))
		}
    });
}		
		
#[acmd_script(
    agent = "jack",
    scripts =  ["game_specialnescapeb_ex", "game_specialairnescapeb_ex","game_specialnescapeb", "game_specialairnescapeb"],
    category = ACMD_GAME,
	low_priority)]
unsafe fn joker_gun_b(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent); 
	let status_kind = smash::app::lua_bind::StatusModule::status_kind(boma);
	let ENTRY_ID = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    acmd!(lua_state, {
		frame(Frame=4)
		if(is_excute){
			JostleModule::set_status(false)
		}
		frame(Frame=17)
		if(is_excute){
			JostleModule::set_status(true)
			WorkModule::on_flag(Flag=FIGHTER_JACK_STATUS_SPECIAL_N_FLAG_UNABLE_SPECIAL_N)
		}
		frame(Frame=22)
		if(is_excute){
			rust {
				if GUN_C[ENTRY_ID] == ATTACK_S3 {
					acmd!(lua_state, {ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=5.0, Angle=68, KBG=70, FKB=0, BKB=80, Size=4.5, X=0.0, Y=11.0, Z=9.0, X2=0.0, Y2=11.0, Z2=30.0, Hitlag=0.5, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=-2, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_jack_bullet"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_JACK_SHOT, Type=ATTACK_REGION_OBJECT)});
				}else if GUN_C[ENTRY_ID] == ATTACK_AIR_B {
					StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_FALL, true);
				} else {
					acmd!(lua_state, {ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=1.0, Angle=50, KBG=30, FKB=0, BKB=5, Size=2.5, X=0.0, Y=11.0, Z=9.0, X2=0.0, Y2=11.0, Z2=50.0, Hitlag=0.5, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=-2, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=true, DisableHitlag=false, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_jack_bullet"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_JACK_SHOT, Type=ATTACK_REGION_OBJECT)});
				};
				if WorkModule::is_flag(boma, *FIGHTER_JACK_INSTANCE_WORK_ID_FLAG_DOYLE) {
					acmd!(lua_state, {AttackModule::set_add_reaction_frame(ID=0, Frames=6.0, Unk=false)});
				};
				GUN_C[ENTRY_ID] = NONE;
			}
			FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(0, 4, 4)
			FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(1, 4, 4)
			FT_SHOOTING_ATTACK_GROUND_CHECK_NEW_arg5(2, 4, 4, hash40("jack_gun_hit2"), hash40("se_jack_special_n02"))
		}
		wait(Frames=1)
		if(is_excute){
			AttackModule::clear_all()
		}
    });
}			
#[acmd_script(
    agent = "jack",
    scripts =  ["game_specials2"],
    category = ACMD_GAME,
	low_priority)]
unsafe fn joker_eiagon(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		frame(Frame=1)
		FT_MOTION_RATE(FSM=1.4375)
		frame(Frame=18)
		FT_MOTION_RATE(FSM=1)
    });
}	
			
#[acmd_script(
    agent = "jack",
    scripts =  ["game_specialairs2"],
    category = ACMD_GAME,
	low_priority)]
unsafe fn joker_eiagon_air(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		frame(Frame=1)
		FT_MOTION_RATE(FSM=1.4375)
		frame(Frame=18)
		FT_MOTION_RATE(FSM=1)
		if(is_excute){
			WorkModule::on_flag(Flag=FIGHTER_JACK_STATUS_SPECIAL_S_FLAG_ENABLE_CONTROL_ENERGY)
		}
		frame(Frame=32)
		if(is_excute){
			WorkModule::on_flag(Flag=FIGHTER_JACK_STATUS_SPECIAL_S_FLAG_SET_FALL_NORMAL)
		}
    });
}
#[fighter_frame( agent = FIGHTER_KIND_KIRBY )]
fn kirby_joker_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent); 
		let status_kind = smash::app::lua_bind::StatusModule::status_kind(boma);
		let ENTRY_ID = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
		let cat1 = ControlModule::get_command_flag_cat(boma, 0);
		let motion_kind = MotionModule::motion_kind(boma);
		if WorkModule::get_int(boma, *FIGHTER_KIRBY_INSTANCE_WORK_ID_INT_COPY_CHARA) == *FIGHTER_KIND_JACK {
			if smash::app::sv_information::is_ready_go() == false || [*FIGHTER_STATUS_KIND_DAMAGE, *FIGHTER_STATUS_KIND_DAMAGE_AIR, *FIGHTER_STATUS_KIND_DAMAGE_FLY, *FIGHTER_STATUS_KIND_DAMAGE_FALL, *FIGHTER_STATUS_KIND_DAMAGE_SONG, *FIGHTER_STATUS_KIND_DAMAGE_SLEEP, *FIGHTER_STATUS_KIND_DAMAGE_FLY_ROLL, *FIGHTER_STATUS_KIND_DAMAGE_SONG_END, *FIGHTER_STATUS_KIND_DAMAGE_SLEEP_END, *FIGHTER_STATUS_KIND_DAMAGE_FLY_METEOR, *FIGHTER_STATUS_KIND_DAMAGE_SLEEP_FALL, *FIGHTER_STATUS_KIND_DAMAGE_SONG_START, *FIGHTER_STATUS_KIND_DAMAGE_SLEEP_START, *FIGHTER_STATUS_KIND_CATCH_PULL, *FIGHTER_STATUS_KIND_CATCH_JUMP, *FIGHTER_STATUS_KIND_CATCH_ATTACK, *FIGHTER_STATUS_KIND_CATCH_CUT, *FIGHTER_STATUS_KIND_CATCH_WAIT, *FIGHTER_STATUS_KIND_CATCHED_GANON, *FIGHTER_STATUS_KIND_CATCHED_RIDLEY, *FIGHTER_STATUS_KIND_CATCHED_REFLET, *FIGHTER_STATUS_KIND_CATCHED_RIDLEY, *FIGHTER_STATUS_KIND_CATCH_DASH_PULL, *FIGHTER_STATUS_KIND_CATCHED_AIR_GANON, *FIGHTER_STATUS_KIND_CATCHED_CUT_GANON, *FIGHTER_STATUS_KIND_CATCHED_AIR_END_GANON, *FIGHTER_STATUS_KIND_CATCHED_AIR_FALL_GANON, *FIGHTER_STATUS_KIND_CATCHED_PICKEL_TROLLEY, *FIGHTER_STATUS_KIND_CAPTURE_CUT, *FIGHTER_STATUS_KIND_CAPTURE_ITEM, *FIGHTER_STATUS_KIND_CAPTURE_JUMP, *FIGHTER_STATUS_KIND_CAPTURE_WAIT, *FIGHTER_STATUS_KIND_CAPTURE_YOSHI, *FIGHTER_STATUS_KIND_CAPTURE_BEETLE, *FIGHTER_STATUS_KIND_CAPTURE_DAMAGE, *FIGHTER_STATUS_KIND_CAPTURE_DRIVER, *FIGHTER_STATUS_KIND_CAPTURE_NABBIT, *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *FIGHTER_STATUS_KIND_CAPTURE_CLAPTRAP, *FIGHTER_STATUS_KIND_CAPTURE_KAWASAKI, *FIGHTER_STATUS_KIND_CAPTURE_MIMIKKYU, *FIGHTER_STATUS_KIND_CAPTURE_BEITCRANE, *FIGHTER_STATUS_KIND_CAPTURE_BLACKHOLE, *FIGHTER_STATUS_KIND_CAPTURE_JACK_WIRE, *FIGHTER_STATUS_KIND_CAPTURE_BOSSGALAGA, *FIGHTER_STATUS_KIND_CAPTURE_MASTERCORE, *FIGHTER_STATUS_KIND_CAPTURE_MASTERHAND, *FIGHTER_STATUS_KIND_CAPTURE_WAIT_YOSHI, *FIGHTER_STATUS_KIND_CAPTURE_DAMAGE_YOSHI, *FIGHTER_STATUS_KIND_CAPTURE_MASTER_SWORD, *FIGHTER_STATUS_KIND_CAPTURE_PULLED_YOSHI, *FIGHTER_STATUS_KIND_CAPTURE_WAIT_OCTOPUS, *FIGHTER_STATUS_KIND_CAPTURE_PULLED_OCTOPUS, *FIGHTER_STATUS_KIND_CAPTURE_PULLED_FISHINGROD, *FIGHTER_STATUS_KIND_BURY, *FIGHTER_STATUS_KIND_BIND, *FIGHTER_STATUS_KIND_ICE, *FIGHTER_STATUS_KIND_DOWN, *FIGHTER_STATUS_KIND_LOSE, *FIGHTER_STATUS_KIND_NONE, *FIGHTER_STATUS_KIND_SLIP, *FIGHTER_STATUS_KIND_ENTRY, *FIGHTER_STATUS_KIND_FINAL, *FIGHTER_STATUS_KIND_GLIDE, *FIGHTER_STATUS_KIND_APPEAL, *FIGHTER_STATUS_KIND_THROWN, *FIGHTER_STATUS_KIND_FURAFURA, *FIGHTER_STATUS_KIND_REBOUND, *FIGHTER_STATUS_KIND_REBIRTH, *FIGHTER_STATUS_KIND_PASSIVE, *FIGHTER_STATUS_KIND_KILLER, *FIGHTER_STATUS_KIND_ICE_JUMP, *FIGHTER_STATUS_KIND_LAY_DOWN, *FIGHTER_STATUS_KIND_PIT_FALL, *FIGHTER_STATUS_KIND_ROULETTE, *FIGHTER_STATUS_KIND_WARPSTAR, *FIGHTER_STATUS_KIND_BURY_JUMP, *FIGHTER_STATUS_KIND_BURY_WAIT, *FIGHTER_STATUS_KIND_SLIP_WAIT, *FIGHTER_STATUS_KIND_SLEEP_END, *FIGHTER_STATUS_KIND_STOP_CEIL, *FIGHTER_STATUS_KIND_STOP_WALL, *FIGHTER_STATUS_KIND_SWALLOWED, *FIGHTER_STATUS_KIND_YOSHI_EGG, *FIGHTER_STATUS_KIND_KASEY_WARP, *FIGHTER_STATUS_KIND_SLIP_STAND, *FIGHTER_STATUS_KIND_TREAD_FALL, *FIGHTER_STATUS_KIND_CLIFF_CATCH, *FIGHTER_STATUS_KIND_CLIFF_CLIMB, *FIGHTER_STATUS_KIND_CLIFF_JUMP1, *FIGHTER_STATUS_KIND_CLIFF_JUMP2, *FIGHTER_STATUS_KIND_CLIFF_JUMP3, *FIGHTER_STATUS_KIND_CLUNG_DIDDY, *FIGHTER_STATUS_KIND_CLUNG_GANON, *FIGHTER_STATUS_KIND_DEMON_DIVED, *FIGHTER_STATUS_KIND_DETACH_WALL, *FIGHTER_STATUS_KIND_BITTEN_WARIO, *FIGHTER_STATUS_KIND_CLIFF_ATTACK, *FIGHTER_STATUS_KIND_CLIFF_ESCAPE, *FIGHTER_STATUS_KIND_CLIFF_ROBBED, *FIGHTER_STATUS_KIND_DRAGOON_RIDE, *FIGHTER_STATUS_KIND_FALL_SPECIAL, *FIGHTER_STATUS_KIND_GUARD_DAMAGE, *FIGHTER_STATUS_KIND_KAMUI_PIERCE, *FIGHTER_STATUS_KIND_PASSIVE_CEIL, *FIGHTER_STATUS_KIND_PASSIVE_WALL, *FIGHTER_STATUS_KIND_REBOUND_JUMP, *FIGHTER_STATUS_KIND_REBOUND_STOP, *FIGHTER_STATUS_KIND_SLIP_STAND_B, *FIGHTER_STATUS_KIND_SLIP_STAND_F, *FIGHTER_STATUS_KIND_SMASH_APPEAL, *FIGHTER_STATUS_KIND_SUICIDE_BOMB, *FIGHTER_STATUS_KIND_TREAD_DAMAGE, *FIGHTER_STATUS_KIND_CLUNG_CAPTAIN, *FIGHTER_STATUS_KIND_DOWN_CONTINUE, *FIGHTER_STATUS_KIND_DOWN_STAND_FB, *FIGHTER_STATUS_KIND_GENESIS_SHOOT, *FIGHTER_STATUS_KIND_GIMMICK_EATEN, *FIGHTER_STATUS_KIND_GLIDE_LANDING, *FIGHTER_STATUS_KIND_ITEM_STARRING, *FIGHTER_STATUS_KIND_MEWTWO_THROWN].contains(&status_kind) {
				GUN_C[ENTRY_ID] = NONE;
			};
			if GUN_C[ENTRY_ID] != NONE && status_kind == *FIGHTER_KIRBY_STATUS_KIND_JACK_SPECIAL_N_ESCAPE {
				if MotionModule::frame(boma) < 20.0 {
					MotionModule::set_rate(boma, 2.0);
				} else {
					MotionModule::set_rate(boma, 1.0);
				};
			};
			if [*FIGHTER_KIRBY_STATUS_KIND_JACK_SPECIAL_N_BARRAGE, *FIGHTER_KIRBY_STATUS_KIND_JACK_SPECIAL_N_JUMP, *FIGHTER_KIRBY_STATUS_KIND_JACK_SPECIAL_N, *FIGHTER_KIRBY_STATUS_KIND_JACK_SPECIAL_N_ESCAPE].contains(&status_kind) && AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT){
				if (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_DASH) != 0 {
					if StatusModule::situation_kind(boma) == *SITUATION_KIND_AIR {
						StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_FALL_AERIAL, true);
					} else {
						StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_DASH, true);
					};
				};
				if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_JUMP) {
					if StatusModule::situation_kind(boma) != *SITUATION_KIND_AIR {
						StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_JUMP, true);
					} else {
						StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_FALL_AERIAL, true);
					};
				};
				if ControlModule::get_stick_y(boma) <= -0.625 {
					if StatusModule::situation_kind(boma) == *SITUATION_KIND_AIR {
						StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_FALL_AERIAL, true);
					};
				};
			};
			if AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT) && (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_ANY) != 0 && (WorkModule::is_flag(boma, *FIGHTER_JACK_INSTANCE_WORK_ID_FLAG_DOYLE) == false || AttackModule::is_infliction(boma, *COLLISION_KIND_MASK_HIT) == false){
				if status_kind == *FIGHTER_STATUS_KIND_ATTACK {
					GUN_C[ENTRY_ID] = ATTACK;
					StatusModule::change_status_request_from_script(boma, *FIGHTER_KIRBY_STATUS_KIND_JACK_SPECIAL_N, true);
				};
				if status_kind == *FIGHTER_STATUS_KIND_ATTACK_S3 {
					GUN_C[ENTRY_ID] = ATTACK_S3;
					PostureModule::reverse_lr(boma);
					PostureModule::update_rot_y_lr(boma);
					StatusModule::change_status_request_from_script(boma, *FIGHTER_KIRBY_STATUS_KIND_JACK_SPECIAL_N_ESCAPE, true);
				};
				if status_kind == *FIGHTER_STATUS_KIND_ATTACK_S4 {
					GUN_C[ENTRY_ID] = ATTACK_S4;
					StatusModule::change_status_request_from_script(boma, *FIGHTER_KIRBY_STATUS_KIND_JACK_SPECIAL_N, true);
				};
				if status_kind == *FIGHTER_STATUS_KIND_ATTACK_AIR {
					if motion_kind == hash40("attack_air_f") {
						GUN_C[ENTRY_ID] = ATTACK_AIR_F;
						StatusModule::change_status_request_from_script(boma, *FIGHTER_KIRBY_STATUS_KIND_JACK_SPECIAL_N, true);
					};
					if motion_kind == hash40("attack_air_b") {
						GUN_C[ENTRY_ID] = ATTACK_AIR_B;
						StatusModule::change_status_request_from_script(boma, *FIGHTER_KIRBY_STATUS_KIND_JACK_SPECIAL_N_ESCAPE, true);
					};
					if motion_kind == hash40("attack_air_hi") {
						GUN_C[ENTRY_ID] = ATTACK_AIR_HI;
						StatusModule::change_status_request_from_script(boma, *FIGHTER_KIRBY_STATUS_KIND_JACK_SPECIAL_N, true);
					};
				};
				println!("{}", GUN_C[ENTRY_ID]);
			};
		};
    }
}
#[fighter_frame( agent = FIGHTER_KIND_JACK )]
fn joker_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent); 
		let status_kind = smash::app::lua_bind::StatusModule::status_kind(boma);
		let ENTRY_ID = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
		let cat1 = ControlModule::get_command_flag_cat(boma, 0);
		let motion_kind = MotionModule::motion_kind(boma);
		if smash::app::sv_information::is_ready_go() == false || [*FIGHTER_STATUS_KIND_DAMAGE, *FIGHTER_STATUS_KIND_DAMAGE_AIR, *FIGHTER_STATUS_KIND_DAMAGE_FLY, *FIGHTER_STATUS_KIND_DAMAGE_FALL, *FIGHTER_STATUS_KIND_DAMAGE_SONG, *FIGHTER_STATUS_KIND_DAMAGE_SLEEP, *FIGHTER_STATUS_KIND_DAMAGE_FLY_ROLL, *FIGHTER_STATUS_KIND_DAMAGE_SONG_END, *FIGHTER_STATUS_KIND_DAMAGE_SLEEP_END, *FIGHTER_STATUS_KIND_DAMAGE_FLY_METEOR, *FIGHTER_STATUS_KIND_DAMAGE_SLEEP_FALL, *FIGHTER_STATUS_KIND_DAMAGE_SONG_START, *FIGHTER_STATUS_KIND_DAMAGE_SLEEP_START, *FIGHTER_STATUS_KIND_CATCH_PULL, *FIGHTER_STATUS_KIND_CATCH_JUMP, *FIGHTER_STATUS_KIND_CATCH_ATTACK, *FIGHTER_STATUS_KIND_CATCH_CUT, *FIGHTER_STATUS_KIND_CATCH_WAIT, *FIGHTER_STATUS_KIND_CATCHED_GANON, *FIGHTER_STATUS_KIND_CATCHED_RIDLEY, *FIGHTER_STATUS_KIND_CATCHED_REFLET, *FIGHTER_STATUS_KIND_CATCHED_RIDLEY, *FIGHTER_STATUS_KIND_CATCH_DASH_PULL, *FIGHTER_STATUS_KIND_CATCHED_AIR_GANON, *FIGHTER_STATUS_KIND_CATCHED_CUT_GANON, *FIGHTER_STATUS_KIND_CATCHED_AIR_END_GANON, *FIGHTER_STATUS_KIND_CATCHED_AIR_FALL_GANON, *FIGHTER_STATUS_KIND_CATCHED_PICKEL_TROLLEY, *FIGHTER_STATUS_KIND_CAPTURE_CUT, *FIGHTER_STATUS_KIND_CAPTURE_ITEM, *FIGHTER_STATUS_KIND_CAPTURE_JUMP, *FIGHTER_STATUS_KIND_CAPTURE_WAIT, *FIGHTER_STATUS_KIND_CAPTURE_YOSHI, *FIGHTER_STATUS_KIND_CAPTURE_BEETLE, *FIGHTER_STATUS_KIND_CAPTURE_DAMAGE, *FIGHTER_STATUS_KIND_CAPTURE_DRIVER, *FIGHTER_STATUS_KIND_CAPTURE_NABBIT, *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *FIGHTER_STATUS_KIND_CAPTURE_CLAPTRAP, *FIGHTER_STATUS_KIND_CAPTURE_KAWASAKI, *FIGHTER_STATUS_KIND_CAPTURE_MIMIKKYU, *FIGHTER_STATUS_KIND_CAPTURE_BEITCRANE, *FIGHTER_STATUS_KIND_CAPTURE_BLACKHOLE, *FIGHTER_STATUS_KIND_CAPTURE_JACK_WIRE, *FIGHTER_STATUS_KIND_CAPTURE_BOSSGALAGA, *FIGHTER_STATUS_KIND_CAPTURE_MASTERCORE, *FIGHTER_STATUS_KIND_CAPTURE_MASTERHAND, *FIGHTER_STATUS_KIND_CAPTURE_WAIT_YOSHI, *FIGHTER_STATUS_KIND_CAPTURE_DAMAGE_YOSHI, *FIGHTER_STATUS_KIND_CAPTURE_MASTER_SWORD, *FIGHTER_STATUS_KIND_CAPTURE_PULLED_YOSHI, *FIGHTER_STATUS_KIND_CAPTURE_WAIT_OCTOPUS, *FIGHTER_STATUS_KIND_CAPTURE_PULLED_OCTOPUS, *FIGHTER_STATUS_KIND_CAPTURE_PULLED_FISHINGROD, *FIGHTER_STATUS_KIND_BURY, *FIGHTER_STATUS_KIND_BIND, *FIGHTER_STATUS_KIND_ICE, *FIGHTER_STATUS_KIND_DOWN, *FIGHTER_STATUS_KIND_LOSE, *FIGHTER_STATUS_KIND_NONE, *FIGHTER_STATUS_KIND_SLIP, *FIGHTER_STATUS_KIND_ENTRY, *FIGHTER_STATUS_KIND_FINAL, *FIGHTER_STATUS_KIND_GLIDE, *FIGHTER_STATUS_KIND_APPEAL, *FIGHTER_STATUS_KIND_THROWN, *FIGHTER_STATUS_KIND_FURAFURA, *FIGHTER_STATUS_KIND_REBOUND, *FIGHTER_STATUS_KIND_REBIRTH, *FIGHTER_STATUS_KIND_PASSIVE, *FIGHTER_STATUS_KIND_KILLER, *FIGHTER_STATUS_KIND_ICE_JUMP, *FIGHTER_STATUS_KIND_LAY_DOWN, *FIGHTER_STATUS_KIND_PIT_FALL, *FIGHTER_STATUS_KIND_ROULETTE, *FIGHTER_STATUS_KIND_WARPSTAR, *FIGHTER_STATUS_KIND_BURY_JUMP, *FIGHTER_STATUS_KIND_BURY_WAIT, *FIGHTER_STATUS_KIND_SLIP_WAIT, *FIGHTER_STATUS_KIND_SLEEP_END, *FIGHTER_STATUS_KIND_STOP_CEIL, *FIGHTER_STATUS_KIND_STOP_WALL, *FIGHTER_STATUS_KIND_SWALLOWED, *FIGHTER_STATUS_KIND_YOSHI_EGG, *FIGHTER_STATUS_KIND_KASEY_WARP, *FIGHTER_STATUS_KIND_SLIP_STAND, *FIGHTER_STATUS_KIND_TREAD_FALL, *FIGHTER_STATUS_KIND_CLIFF_CATCH, *FIGHTER_STATUS_KIND_CLIFF_CLIMB, *FIGHTER_STATUS_KIND_CLIFF_JUMP1, *FIGHTER_STATUS_KIND_CLIFF_JUMP2, *FIGHTER_STATUS_KIND_CLIFF_JUMP3, *FIGHTER_STATUS_KIND_CLUNG_DIDDY, *FIGHTER_STATUS_KIND_CLUNG_GANON, *FIGHTER_STATUS_KIND_DEMON_DIVED, *FIGHTER_STATUS_KIND_DETACH_WALL, *FIGHTER_STATUS_KIND_BITTEN_WARIO, *FIGHTER_STATUS_KIND_CLIFF_ATTACK, *FIGHTER_STATUS_KIND_CLIFF_ESCAPE, *FIGHTER_STATUS_KIND_CLIFF_ROBBED, *FIGHTER_STATUS_KIND_DRAGOON_RIDE, *FIGHTER_STATUS_KIND_FALL_SPECIAL, *FIGHTER_STATUS_KIND_GUARD_DAMAGE, *FIGHTER_STATUS_KIND_KAMUI_PIERCE, *FIGHTER_STATUS_KIND_PASSIVE_CEIL, *FIGHTER_STATUS_KIND_PASSIVE_WALL, *FIGHTER_STATUS_KIND_REBOUND_JUMP, *FIGHTER_STATUS_KIND_REBOUND_STOP, *FIGHTER_STATUS_KIND_SLIP_STAND_B, *FIGHTER_STATUS_KIND_SLIP_STAND_F, *FIGHTER_STATUS_KIND_SMASH_APPEAL, *FIGHTER_STATUS_KIND_SUICIDE_BOMB, *FIGHTER_STATUS_KIND_TREAD_DAMAGE, *FIGHTER_STATUS_KIND_CLUNG_CAPTAIN, *FIGHTER_STATUS_KIND_DOWN_CONTINUE, *FIGHTER_STATUS_KIND_DOWN_STAND_FB, *FIGHTER_STATUS_KIND_GENESIS_SHOOT, *FIGHTER_STATUS_KIND_GIMMICK_EATEN, *FIGHTER_STATUS_KIND_GLIDE_LANDING, *FIGHTER_STATUS_KIND_ITEM_STARRING, *FIGHTER_STATUS_KIND_MEWTWO_THROWN].contains(&status_kind) {
			GUN_C[ENTRY_ID] = NONE;
		};
		if GUN_C[ENTRY_ID] != NONE && status_kind == *FIGHTER_JACK_STATUS_KIND_SPECIAL_N_ESCAPE {
			if MotionModule::frame(boma) < 20.0 {
				MotionModule::set_rate(boma, 2.0);
			} else {
				MotionModule::set_rate(boma, 1.0);
			};
		};
		if [*FIGHTER_JACK_STATUS_KIND_SPECIAL_N_JUMP, *FIGHTER_STATUS_KIND_SPECIAL_N, *FIGHTER_JACK_STATUS_KIND_SPECIAL_N_ESCAPE].contains(&status_kind) && AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT){
			if (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_DASH) != 0 {
				if StatusModule::situation_kind(boma) == *SITUATION_KIND_AIR {
					StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_FALL_AERIAL, true);
				} else {
					StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_DASH, true);
				};
			};
			if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_JUMP) {
				if StatusModule::situation_kind(boma) != *SITUATION_KIND_AIR {
					StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_JUMP, true);
				} else {
					StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_FALL_AERIAL, true);
				};
			};
			if ControlModule::get_stick_y(boma) <= -0.625 {
				if StatusModule::situation_kind(boma) == *SITUATION_KIND_AIR {
					StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_FALL_AERIAL, true);
				};
			};
		};
		if AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT) && (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_ANY) != 0 && (WorkModule::is_flag(boma, *FIGHTER_JACK_INSTANCE_WORK_ID_FLAG_DOYLE) == false || AttackModule::is_infliction(boma, *COLLISION_KIND_MASK_HIT) == false){
			if status_kind == *FIGHTER_STATUS_KIND_ATTACK {
				GUN_C[ENTRY_ID] = ATTACK;
				macros::EFFECT_OFF_KIND(fighter, Hash40::new_raw(0x15e858a896), true, true);
				macros::EFFECT_OFF_KIND(fighter, Hash40::new_raw(0x0a38380378), true, true);
				macros::EFFECT_OFF_KIND(fighter, Hash40::new_raw(0x0f6ac1c8de), true, true);
				macros::EFFECT_OFF_KIND(fighter, Hash40::new_raw(0x17a5cc8181), true, true);
				StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_SPECIAL_N, true);
			};
			if status_kind == *FIGHTER_STATUS_KIND_ATTACK_S3 {
				GUN_C[ENTRY_ID] = ATTACK_S3;
				PostureModule::reverse_lr(boma);
				PostureModule::update_rot_y_lr(boma);
				StatusModule::change_status_request_from_script(boma, *FIGHTER_JACK_STATUS_KIND_SPECIAL_N_ESCAPE, true);
			};
			if status_kind == *FIGHTER_STATUS_KIND_ATTACK_S4 {
				GUN_C[ENTRY_ID] = ATTACK_S4;
				StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_SPECIAL_N, true);
			};
			if status_kind == *FIGHTER_STATUS_KIND_ATTACK_AIR {
				if motion_kind == hash40("attack_air_f") {
					GUN_C[ENTRY_ID] = ATTACK_AIR_F;
					StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_SPECIAL_N, true);
				};
				if motion_kind == hash40("attack_air_b") {
					GUN_C[ENTRY_ID] = ATTACK_AIR_B;
					StatusModule::change_status_request_from_script(boma, *FIGHTER_JACK_STATUS_KIND_SPECIAL_N_ESCAPE, true);
				};
				if motion_kind == hash40("attack_air_hi") {
					GUN_C[ENTRY_ID] = ATTACK_AIR_HI;
					StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_SPECIAL_N, true);
				};
			};
			println!("{}", GUN_C[ENTRY_ID]);
		};
    }
}
pub fn install() {
    smashline::install_acmd_scripts!(
		joker_eiha,
		joker_eiagon,
		joker_eiagon_air,
		joker_gun,
		joker_gun_b,
		s_joker_gun,
		s_joker_gun_b,
		e_joker_gun,
		e_joker_gun_b,
		joker_fsmash,
		joker_bair,
		joker_fair,
		joker_uair
    );
	smashline::install_agent_frames!(joker_frame, kirby_joker_frame);
}
