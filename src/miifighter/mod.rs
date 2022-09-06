use smash::app::lua_bind::*;
use smash::lib::lua_const::*;
use smash::app::utility::get_kind;
use smash::hash40;
use smash::phx::Hash40;
use smash::lua2cpp::*;
use smashline::*;
use smash_script::*;
use crate::util::*;

static mut COUNTER_IS : [bool; 8] = [false; 8];
static mut BAN_DOWNB : [bool; 8] = [false; 8];
static mut ESK_CHARGE : [i32; 8] = [0; 8];
static mut ESK :  smash::phx::Vector3f =  smash::phx::Vector3f { x: 0.0, y: 0.0, z: 0.0 };

#[acmd_script(
    agent = "miifighter",
    script =  "game_specialairlw2kick",
    category = ACMD_GAME,
	low_priority)]
unsafe fn brawler_fjk(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		if(is_excute){
			SET_SPEED_EX(0, 0.8, KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN)
		}
		frame(Frame=7)
		if(is_excute){
			SET_SPEED_EX(-3, -2.5, KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN)
		}
		frame(Frame=8)
		if(is_excute){
			rust{
				if WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_CUSTOMIZE_SPECIAL_LW_NO) == 1 {
					acmd!(lua_state, {
						ATTACK(ID=0, Part=0, Bone=hash40("kneer"), Damage=10.0, Angle=35, KBG=90, FKB=0, BKB=65, Size=5.8, X=4.2, Y=0.0, Z=0.0, X2=0.0, Y2=0.0, Z2=0.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_B, SetWeight=false, ShieldDamage=5, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_A, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
						ATTACK(ID=1, Part=0, Bone=hash40("kneer"), Damage=10.0, Angle=60, KBG=90, FKB=0, BKB=65, Size=5.8, X=4.2, Y=0.0, Z=0.0, X2=0.0, Y2=0.0, Z2=0.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_B, SetWeight=false, ShieldDamage=5, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_G, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
					});
				} else if WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_CUSTOMIZE_SPECIAL_LW_NO) == 2 {
					acmd!(lua_state, {
						ATTACK(ID=0, Part=0, Bone=hash40("kneer"), Damage=7.0, Angle=65, KBG=40, FKB=0, BKB=70, Size=5.8, X=4.2, Y=0.0, Z=0.0, X2=0.0, Y2=0.0, Z2=0.0, Hitlag=1.5, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_B, SetWeight=false, ShieldDamage=5, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_A, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
						ATTACK(ID=1, Part=0, Bone=hash40("kneer"), Damage=7.0, Angle=65, KBG=40, FKB=0, BKB=70, Size=5.8, X=4.2, Y=0.0, Z=0.0, X2=0.0, Y2=0.0, Z2=0.0, Hitlag=1.5, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_B, SetWeight=false, ShieldDamage=5, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_G, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
					});
				} else {
					acmd!(lua_state, {
						AttackModule::clear_all()
						FT_MOTION_RATE(FSM=0.5)
					});
				};
			}
		}
		frame(Frame=22)
		if(is_excute){
			rust{
				if WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_CUSTOMIZE_SPECIAL_LW_NO) != 0 {
					acmd!(lua_state, {
						AttackModule::clear_all()
						FT_MOTION_RATE(FSM=2.2)
					});
				};
			}
		}
		frame(Frame=51)
		if(is_excute){
			PostureModule::reverse_lr()
		}
    });
}						

#[acmd_script(
    agent = "miifighter",
    script =  "game_attack11",
    category = ACMD_GAME,
	low_priority)]
unsafe fn brawler_jab1(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		frame(Frame=2)
		if(is_excute){
			ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=2.8, Angle=361, KBG=100, FKB=15, BKB=0, Size=1.6, X=0.0, Y=7.5, Z=4.5, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.8, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_PUNCH)
			ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=2.8, Angle=361, KBG=100, FKB=15, BKB=0, Size=1.6, X=0.0, Y=7.5, Z=7.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.8, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_PUNCH)
			ATTACK(ID=2, Part=0, Bone=hash40("top"), Damage=2.8, Angle=180, KBG=100, FKB=15, BKB=0, Size=1.8, X=0.0, Y=7.5, Z=10.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.8, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_FIGHTER, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_PUNCH)
			ATTACK(ID=3, Part=0, Bone=hash40("top"), Damage=2.8, Angle=361, KBG=100, FKB=15, BKB=0, Size=1.8, X=0.0, Y=7.5, Z=10.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.8, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_PUNCH)
			AttackModule::set_add_reaction_frame(ID=0, Frames=-1.0, Unk=false)
			AttackModule::set_add_reaction_frame(ID=1, Frames=-1.0, Unk=false)
			AttackModule::set_add_reaction_frame(ID=2, Frames=-1.0, Unk=false)
			AttackModule::set_add_reaction_frame(ID=3, Frames=-1.0, Unk=false)
		}
		wait(Frames=2)
		if(is_excute){
			AttackModule::clear_all()
			WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO)
		}
		frame(Frame=7)
		if(is_excute){
			WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_FLAG_ENABLE_RESTART)
		}
    });
}
#[acmd_script(
    agent = "miifighter",
    script =  "game_attack12",
    category = ACMD_GAME,
	low_priority)]
unsafe fn brawler_jab2(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		frame(Frame=2)
		if(is_excute){
			ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=1.0, Angle=361, KBG=100, FKB=15, BKB=0, Size=3.0, X=0.0, Y=7.5, Z=7.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.2, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_PUNCH)
			ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=1.0, Angle=180, KBG=100, FKB=15, BKB=0, Size=3.5, X=0.0, Y=7.5, Z=11.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.2, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_FIGHTER, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_PUNCH)
			ATTACK(ID=2, Part=0, Bone=hash40("top"), Damage=1.0, Angle=361, KBG=100, FKB=15, BKB=0, Size=3.5, X=0.0, Y=7.5, Z=11.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.2, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_PUNCH)
			AttackModule::set_add_reaction_frame(ID=0, Frames=-2.0, Unk=false)
			AttackModule::set_add_reaction_frame(ID=1, Frames=-2.0, Unk=false)
			AttackModule::set_add_reaction_frame(ID=2, Frames=-2.0, Unk=false)
		}
		wait(Frames=2)
		if(is_excute){
			AttackModule::clear_all()
		}
		frame(Frame=5)
		if(is_excute){
			WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_FLAG_ENABLE_100)
			WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO)
		}
    });
}		
#[acmd_script(
    agent = "miifighter",
    script =  "game_attacklw4",
    category = ACMD_GAME,
	low_priority)]
unsafe fn brawler_dsmash(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		FT_MOTION_RATE(FSM=0.667)
		frame(Frame=3)
		if(is_excute){
			WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD)
		}
		frame(Frame=9)
		FT_MOTION_RATE(FSM=1)
		if(is_excute){
			ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=13.0, Angle=30, KBG=85, FKB=0, BKB=50, Size=4.0, X=0.0, Y=5.5, Z=11.0, X2=0.0, Y2=5.5, Z2=4.5, Hitlag=1.2, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_PUNCH)
			ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=13.0, Angle=30, KBG=88, FKB=0, BKB=50, Size=4.0, X=0.0, Y=5.5, Z=-11.0, X2=0.0, Y2=5.5, Z2=-6.5, Hitlag=1.2, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
		}
		wait(Frames=2)
		if(is_excute){
			AttackModule::clear_all()
		}
    });
}		
#[acmd_script(
    agent = "miifighter",
    script =  "game_attackairf",
    category = ACMD_GAME,
	low_priority)]
unsafe fn brawler_fair(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		frame(Frame=3)
		if(is_excute){
			WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
		}
        frame(Frame=8)
		if(is_excute){
			ATTACK(ID=0, Part=0, Bone=hash40("kneer"), Damage=4.0, Angle=367, KBG=10, FKB=0, BKB=39, Size=4.0, X=4.2, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=0.7, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_A, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
			ATTACK(ID=1, Part=0, Bone=hash40("kneer"), Damage=4.0, Angle=75, KBG=30, FKB=0, BKB=50, Size=4.0, X=4.2, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=0.7, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_G, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
			ATTACK(ID=2, Part=0, Bone=hash40("kneer"), Damage=4.0, Angle=367, KBG=10, FKB=0, BKB=47, Size=3.5, X=-1.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=0.7, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_A, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
			ATTACK(ID=3, Part=0, Bone=hash40("kneer"), Damage=4.0, Angle=75, KBG=30, FKB=0, BKB=50, Size=3.5, X=-1.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=0.7, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_G, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
			AttackModule::set_add_reaction_frame(ID=0, Frames=3.0, Unk=false)
			AttackModule::set_add_reaction_frame(ID=1, Frames=3.0, Unk=false)
			AttackModule::set_add_reaction_frame(ID=2, Frames=3.0, Unk=false)
			AttackModule::set_add_reaction_frame(ID=3, Frames=3.0, Unk=false)
		}
		frame(Frame=10)
		if(is_excute){
			AttackModule::clear_all()
		}
		frame(Frame=15)
		if(is_excute){
			ATTACK(ID=0, Part=0, Bone=hash40("kneel"), Damage=6.65, Angle=50, KBG=105, FKB=0, BKB=50, Size=7.5, X=5.4, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.5, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
			ATTACK(ID=1, Part=0, Bone=hash40("kneel"), Damage=6.65, Angle=50, KBG=105, FKB=0, BKB=50, Size=7.5, X=-2.5, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.5, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
		}
		frame(Frame=18)
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
    agent = "miifighter",
    scripts =  ["game_specialn3","game_specialairn3"],
    category = ACMD_GAME,
	low_priority)]
unsafe fn brawler_esk(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		frame(Frame=1)
		FT_MOTION_RATE(FSM=0.1)
		frame(Frame=10)
		FT_MOTION_RATE(FSM=1)
		frame(Frame=40)
		FT_MOTION_RATE(FSM=0.7)
		frame(Frame=48)
		if(is_excute){
			sv_module_access::damage(MA_MSC_DAMAGE_DAMAGE_NO_REACTION, DAMAGE_NO_REACTION_MODE_ALWAYS, 0)
		}
		frame(Frame=50)
		FT_MOTION_RATE(FSM=1)
		if(is_excute){
			ATTACK(ID=0, Part=0, Bone=hash40("legl"), Damage=25.0, Angle=361, KBG=60, FKB=0, BKB=50, Size=4.0, X=6.8, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.5, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_fire"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
			ATTACK(ID=1, Part=0, Bone=hash40("legl"), Damage=23.0, Angle=361, KBG=60, FKB=0, BKB=50, Size=3.0, X=-4.7, Y=0.0, Z=0.0, X2=2.2, Y2=0.0, Z2=0.0, Hitlag=1.5, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_fire"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
			rust{
				let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent); 
				let ENTRY_ID = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
				AttackModule::set_power(boma, 0, (ESK_CHARGE[ENTRY_ID] as f32 * 0.075)+10.5, false);
				AttackModule::set_power(boma, 1, (ESK_CHARGE[ENTRY_ID] as f32 * 0.065)+9.0, false);
				if ESK_CHARGE[ENTRY_ID] > 170 {
					acmd!(lua_state, {ATK_SET_SHIELD_SETOFF_MUL_arg3(ID1=0, ID2=1, ShieldstunMul=0.8)});
				};
				ESK_CHARGE[ENTRY_ID] = 0;
			}
		}
		wait(Frames=3)
		if(is_excute){
			sv_module_access::damage(MA_MSC_DAMAGE_DAMAGE_NO_REACTION, DAMAGE_NO_REACTION_MODE_NORMAL, 0)
			AttackModule::clear_all()
		}
		frame(Frame=55)
		FT_MOTION_RATE(FSM=0.714)
		frame(Frame=90)
		FT_MOTION_RATE(FSM=1)
    });
}
#[acmd_script(
    agent = "miifighter",
    scripts =  ["sound_specialn3","sound_specialairn3"],
    category = ACMD_SOUND,
	low_priority)]
unsafe fn brawler_esk_s(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		wait(Frames=44)
		if(is_excute){
			PLAY_SE(hash40("se_miifighter_special_c3_n02"))
			PLAY_SEQUENCE(hash40("seq_miifighter_rnd_special_c3_n01"))
		}
    });
}
#[acmd_script(
    agent = "miifighter",
    scripts =  ["effect_specialn3","effect_specialairn3"],
    category = ACMD_EFFECT,
	low_priority)]
unsafe fn brawler_esk_e(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		frame(Frame=30)
		if(is_excute){
			EFFECT_FOLLOW(hash40("miifighter_sidekick_flash"), hash40("toel"), 0, 0, 0, 0, 0, 0, 0.8, true)
		}
		frame(Frame=33)
		for(4 Iterations){
			if(is_excute){
				FLASH(1, 1, 0.392, 0.392)
			}
			wait(Frames=1)
			if(is_excute){
				FLASH(1, 0.392, 0, 0.353)
			}
			wait(Frames=1)
			if(is_excute){
				COL_NORMAL()
			}
			wait(Frames=1)
		}
		frame(Frame=43)
		if(is_excute){
			EFFECT_FLW_POS(hash40("miifighter_sidekick"), hash40("toel"), 0, 0, 0, 0, 0, 0, 1, true)
			EffectModule::enable_sync_init_pos_last()
		}
    });
}

#[acmd_script(
    agent = "miifighter",
    script =  "game_attackairhi",
    category = ACMD_GAME,
	low_priority)]
unsafe fn brawler_uair(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		frame(Frame=2)
		if(is_excute){
			WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
		}
		frame(Frame=6)
		if(is_excute){
			ATTACK(ID=0, Part=0, Bone=hash40("legr"), Damage=9.0, Angle=75, KBG=100, FKB=0, BKB=9, Size=4.0, X=3.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
			ATTACK(ID=1, Part=0, Bone=hash40("kneer"), Damage=9.0, Angle=75, KBG=100, FKB=0, BKB=9, Size=5.0, X=5.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
			AttackModule::set_add_reaction_frame(ID=0, Frames=2.0, Unk=false)
			AttackModule::set_add_reaction_frame(ID=1, Frames=2.0, Unk=false)
		}
		frame(Frame=11)
		if(is_excute){
			AttackModule::clear_all()
		}
		frame(Frame=21)
		if(is_excute){
			WorkModule::off_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
		}
    });
}
#[acmd_script(
    agent = "miifighter",
    script =  "game_speciallw1landing",
    category = ACMD_GAME,
	low_priority)]
unsafe fn brawler_hoa_land(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		FT_MOTION_RATE(FSM=1)
		if(is_excute){
			AttackModule::clear_all()
			ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=7.0, Angle=75, KBG=30, FKB=0, BKB=80, Size=4.0, X=0.0, Y=4.0, Z=5.0, X2=0.0, Y2=4.0, Z2=-6.0, Hitlag=1.1, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_HEAD)
			ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=7.0, Angle=75, KBG=30, FKB=0, BKB=80, Size=4.5, X=0.0, Y=6.5, Z=-0.5, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.1, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_A, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_HEAD)
			ATTACK(ID=2, Part=0, Bone=hash40("top"), Damage=7.0, Angle=75, KBG=30, FKB=0, BKB=80, Size=6.5, X=0.0, Y=6.5, Z=-0.5, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.1, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_HEAD)
		}
		frame(Frame=1)
		if(is_excute){
			ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=7.0, Angle=75, KBG=30, FKB=0, BKB=80, Size=4.0, X=0.0, Y=4.0, Z=5.0, X2=0.0, Y2=4.0, Z2=-6.0, Hitlag=1.1, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_HEAD)
			AttackModule::clear(ID=0, false)
			AttackModule::clear(ID=2, false)
		}
		frame(Frame=5)
		FT_MOTION_RATE(FSM=0.3)
		if(is_excute){
			AttackModule::clear_all()
		}
    });
}
#[acmd_script(
    agent = "miifighter",
    script =  "game_speciallw1loop",
    category = ACMD_GAME,
	low_priority)]
unsafe fn brawler_hoa_loop(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		if(is_excute){
			ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=1.0, Angle=270, KBG=100, FKB=60, BKB=0, Size=7.0, X=0.0, Y=5.2, Z=-0.5, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.2, SDI=0.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=1, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_HEAD)
		}
		frame(Frame=3)
		if(is_excute){
			ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=1.0, Angle=270, KBG=100, FKB=207, BKB=0, Size=7.0, X=0.0, Y=5.2, Z=-0.5, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.2, SDI=0.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=1, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_HEAD)
		}
		frame(Frame=4)
		if(is_excute){
			ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=1.0, Angle=270, KBG=100, FKB=130, BKB=0, Size=7.0, X=0.0, Y=5.2, Z=-0.5, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.2, SDI=0.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=2, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_HEAD)
		}
		frame(Frame=6)
		if(is_excute){
			ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=1.0, Angle=270, KBG=100, FKB=240, BKB=0, Size=7.0, X=0.0, Y=5.2, Z=-0.5, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.2, SDI=0.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=2, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_HEAD)
		}
    });
}
#[acmd_script(
    agent = "miifighter",
    script =  "game_speciallw1",
    category = ACMD_GAME,
	low_priority)]
unsafe fn brawler_foot(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		frame(Frame=10)
		if(is_excute){
			ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=1.2, Angle=361, KBG=100, FKB=50, BKB=0, Size=7.0, X=0.0, Y=6.5, Z=8.0, X2=0.0, Y2=6.5, Z2=9.5, Hitlag=0.6, SDI=0.4, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=2, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_G, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_rush"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
			ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=1.2, Angle=361, KBG=100, FKB=80, BKB=0, Size=7.0, X=0.0, Y=6.5, Z=0.0, X2=0.0, Y2=6.5, Z2=5.0, Hitlag=0.6, SDI=0.4, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=2, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_G, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_rush"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
			ATTACK(ID=2, Part=0, Bone=hash40("top"), Damage=1.2, Angle=0, KBG=100, FKB=50, BKB=0, Size=7.0, X=0.0, Y=6.5, Z=8.0, X2=0.0, Y2=6.5, Z2=9.5, Hitlag=0.6, SDI=0.4, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=2, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_A, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_rush"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
			ATTACK(ID=3, Part=0, Bone=hash40("top"), Damage=1.2, Angle=0, KBG=100, FKB=80, BKB=0, Size=7.0, X=0.0, Y=6.5, Z=0.0, X2=0.0, Y2=6.5, Z2=9.5, Hitlag=0.6, SDI=0.4, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=2, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_A, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_rush"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
		}
		frame(Frame=37)
		if(is_excute){
			AttackModule::clear_all()
		}
		frame(Frame=40)
		if(is_excute){
			ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=5.0, Angle=361, KBG=90, FKB=0, BKB=70, Size=9.0, X=0.0, Y=6.5, Z=8.0, X2=0.0, Y2=6.5, Z2=9.5, Hitlag=1.2, SDI=0.4, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_KICK)
		}
		frame(Frame=43)
		if(is_excute){
			AttackModule::clear_all()
		}
    });
}
#[acmd_script(
    agent = "miifighter",
    script =  "game_specialairlw1",
    category = ACMD_GAME,
	low_priority)]
unsafe fn brawler_foot_air(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		frame(Frame=8)
		if(is_excute){
			ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=4.0, Angle=90, KBG=100, FKB=65, BKB=0, Size=6.5, X=0.0, Y=6.5, Z=8.0, X2=0.0, Y2=6.5, Z2=9.5, Hitlag=1.0, SDI=0.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=true, ShieldDamage=0, Trip=-1.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_G, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
			ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=4.0, Angle=90, KBG=100, FKB=50, BKB=0, Size=6.5, X=0.0, Y=6.5, Z=8.0, X2=0.0, Y2=6.5, Z2=9.5, Hitlag=1.0, SDI=0.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=true, ShieldDamage=0, Trip=-1.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_A, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
			AttackModule::set_add_reaction_frame(ID=0, Frames=5.0, Unk=false)
			AttackModule::set_add_reaction_frame(ID=1, Frames=5.0, Unk=false)
		}
		frame(Frame=10)
		if(is_excute){
			ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=4.0, Angle=90, KBG=100, FKB=50, BKB=0, Size=6.5, X=0.0, Y=6.5, Z=8.0, X2=0.0, Y2=6.5, Z2=9.5, Hitlag=1.0, SDI=0.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=true, ShieldDamage=0, Trip=-1.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_G, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
			ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=4.0, Angle=90, KBG=100, FKB=45, BKB=0, Size=6.5, X=0.0, Y=6.5, Z=8.0, X2=0.0, Y2=6.5, Z2=9.5, Hitlag=1.0, SDI=0.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=true, ShieldDamage=0, Trip=-1.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_A, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
			AttackModule::set_add_reaction_frame(ID=0, Frames=5.0, Unk=false)
			AttackModule::set_add_reaction_frame(ID=1, Frames=5.0, Unk=false)
		}
		frame(Frame=12)
		if(is_excute){
			AttackModule::clear_all()
		}
		frame(Frame=31)
		if(is_excute){
			ATTACK(ID=0, Part=0, Bone=hash40("kneer"), Damage=12.0, Angle=280, KBG=90, FKB=0, BKB=30, Size=12.0, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.2, SDI=0.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=true, ShieldDamage=0, Trip=-1.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_KICK)
		}
		frame(Frame=32)
		if(is_excute){
			ATTACK(ID=0, Part=0, Bone=hash40("kneer"), Damage=12.0, Angle=50, KBG=90, FKB=0, BKB=30, Size=10.0, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.2, SDI=0.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=true, ShieldDamage=0, Trip=-1.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_KICK)
		}
		frame(Frame=35)
		FT_MOTION_RATE(FSM=1.65)
		if(is_excute){
			AttackModule::clear_all()
		}
		frame(Frame=45)
		FT_MOTION_RATE(FSM=1)
		frame(Frame=52)
		FT_MOTION_RATE(FSM=0.5)
    });
}
#[acmd_script(
    agent = "miifighter",
    script =  "effect_speciallw1",
    category = ACMD_EFFECT,
	low_priority)]
unsafe fn brawler_foot_eff(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent); 
    acmd!(lua_state, {
		frame(Frame=8)
		if(is_excute){
			LANDING_EFFECT(hash40("sys_atk_smoke"), hash40("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false)
		}
		frame(Frame=9)
		for(7 Iterations){
			if(is_excute){
				EFFECT_FOLLOW_FLIP(0x19e7eccbcbu64, 0x19e7eccbcbu64, hash40("top"), 0, 3, 9, 0, 0, 0, 0.45, true, EF_FLIP_YZ)
				LAST_EFFECT_SET_RATE(3)
			}
			wait(Frames=1)
			if(is_excute){
				EFFECT_FOLLOW_FLIP(0x19e7eccbcbu64, 0x19e7eccbcbu64, hash40("top"), 0, 12, 12, 0, 0, 0, 0.45, true, EF_FLIP_YZ)
				LAST_EFFECT_SET_RATE(3)
			}
			wait(Frames=1)
			if(is_excute){
				EFFECT_FOLLOW_FLIP(0x19e7eccbcbu64, 0x19e7eccbcbu64, hash40("top"), 0, 7, 15, 0, 0, 0, 0.45, true, EF_FLIP_YZ)
				LAST_EFFECT_SET_RATE(3)
			}
			wait(Frames=1)
			if(is_excute){
				EFFECT_FOLLOW_FLIP(0x19e7eccbcbu64, 0x19e7eccbcbu64, hash40("top"), 0, 10, 13, 0, 0, 0, 0.45, true, EF_FLIP_YZ)
				LAST_EFFECT_SET_RATE(3)
			}
			wait(Frames=1)
		}
		frame(Frame=39)
		if(is_excute){
			EFFECT_FOLLOW_FLIP(0x19e7eccbcbu64, 0x19e7eccbcbu64, hash40("top"), 0, 5, 13, 0, 0, 0, 0.75, true, EF_FLIP_YZ)
			LAST_EFFECT_SET_RATE(1.5)
		}
		frame(Frame=56)
		if(is_excute){
			rust{
				if ray_check_pos(boma, 0.0, -1.0, false) == 1 {
					acmd!(lua_state, {LANDING_EFFECT(hash40("sys_down_smoke"), hash40("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false)});
				}
			}
		}
    });
}
#[acmd_script(
    agent = "miifighter",
    script =  "sound_speciallw1",
    category = ACMD_SOUND,
	low_priority)]
unsafe fn brawler_foot_snd(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		frame(Frame=8)
		if(is_excute){
			PLAY_SE(hash40("se_miifighter_special_c2_n02"))
		}
		frame(Frame=9)
		for(7 Iterations){
			if(is_excute){
				PLAY_SE(hash40("se_miifighter_attack100"))
			}
			wait(Frames=1)
			if(is_excute){
				PLAY_SE(hash40("se_miifighter_attack100"))
			}
			wait(Frames=1)
			if(is_excute){
				PLAY_SE(hash40("se_miifighter_attack100"))
			}
			wait(Frames=1)
			if(is_excute){
				PLAY_SE(hash40("se_miifighter_attack100"))
			}
			wait(Frames=1)
		}
		frame(Frame=38)
		if(is_excute){
			PLAY_SE(hash40("se_miifighter_swing_ll"))
			PLAY_SEQUENCE(hash40("seq_miifighter_rnd_attack03"))
		}
    });
}
#[acmd_script(
    agent = "miifighter",
    script =  "sound_specialairlw1",
    category = ACMD_SOUND,
	low_priority)]
unsafe fn brawler_foot_air_snd(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		frame(Frame=1)
		if(is_excute){
			PLAY_SE(hash40("se_miifighter_special_l02"))
			PLAY_SEQUENCE(hash40("seq_miifighter_rnd_special_c1_l01"))
		}
		frame(Frame=27)
		if(is_excute){
			PLAY_SE(hash40("se_miifighter_smash_s01"))
		}
		wait(Frames=2)
		if(is_excute){
			PLAY_SE(hash40("se_miifighter_smash_s03"))
			PLAY_SE(hash40("vc_mii_attack08"))
		}
    });
}
#[acmd_script(
    agent = "miifighter",
    script =  "effect_specialairlw1",
    category = ACMD_EFFECT,
	low_priority)]
unsafe fn brawler_foot_air_eff(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		frame(Frame=6)
		if(is_excute){
			EFFECT_FOLLOW_FLIP(0x14f9cd9ee5u64, 0x14f9cd9ee5u64, hash40("top"), 0, 7, 0, -13.5, -24.5, -330.5, 0.9, true, EF_FLIP_YZ)
			LAST_EFFECT_SET_RATE(2)
		}
		frame(Frame=25)
		if(is_excute){
			EFFECT_FOLLOW_FLIP(0x14f9cd9ee5u64, 0x14f9cd9ee5u64, hash40("top"), 3, 10, -2, 25.5, 51.8, -89.9, 0.7, true, EF_FLIP_YZ)
			LAST_EFFECT_SET_RATE(1)
		}
    });
}
	
#[acmd_script(
    agent = "miifighter",
    scripts =  ["game_specialairlw3", "game_speciallw3"],
    category = ACMD_GAME,
	low_priority)]
unsafe fn brawler_counter(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		frame(Frame=6)
		if(is_excute){
			WorkModule::on_flag(Flag=FIGHTER_MIIFIGHTER_STATUS_COUNTER_THROW_FLAG_SHIELD)
			SEARCH(0, 0, hash40("top"), 5.0, 0.0, 6.5, 3.5, 0.0, 6.5, 5.5, COLLISION_KIND_MASK_ATTACK, HIT_STATUS_MASK_NORMAL, 60, COLLISION_SITUATION_MASK_GA, COLLISION_CATEGORY_MASK_ALL, COLLISION_PART_MASK_ALL, false)
		}
		frame(Frame=8)
		FT_MOTION_RATE(FSM=0.8)
		frame(Frame=28)
		if(is_excute){
			WorkModule::off_flag(Flag=FIGHTER_MIIFIGHTER_STATUS_COUNTER_THROW_FLAG_SHIELD)
			sv_module_access::shield(MA_MSC_CMD_SHIELD_OFF, COLLISION_KIND_REFLECTOR, 0, FIGHTER_REFLECTOR_GROUP_HOMERUNBAT)
			sv_module_access::search(MA_MSC_CMD_SEARCH_SEARCH_SCH_CLR_ALL)
		}
		FT_MOTION_RATE(FSM=0.5)
		frame(Frame=30)
		FT_MOTION_RATE(FSM=0.4)
		frame(Frame=40)
		FT_MOTION_RATE(FSM=0.5)
		frame(Frame=60)
		FT_MOTION_RATE(FSM=1)
    });
}

#[acmd_script(
    agent = "miifighter",
    script =  "game_attackairb",
    category = ACMD_GAME,
	low_priority)]
unsafe fn brawler_bair(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		frame(Frame=2)
		if(is_excute){
			WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
		}
		frame(Frame=7)
		if(is_excute){
			ATTACK(ID=0, Part=0, Bone=hash40("kneer"), Damage=12.5, Angle=361, KBG=105, FKB=0, BKB=10, Size=5.7, X=6.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_B, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
			ATTACK(ID=1, Part=0, Bone=hash40("legr"), Damage=12.5, Angle=361, KBG=105, FKB=0, BKB=10, Size=5.3, X=2.6, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_B, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
		}
		wait(Frames=3)
		if(is_excute){
			AttackModule::clear_all()
		}
		frame(Frame=28)
		if(is_excute){
			WorkModule::off_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
		}
    });
}
#[acmd_script(
    agent = "miifighter",
    scripts =  ["game_specialairhi11", "game_specialhi11"],
    category = ACMD_GAME,
	low_priority)]
unsafe fn brawler_sak_start(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		FT_MOTION_RATE(FSM=0.8)
	});
}
#[acmd_script(
    agent = "miifighter",
    script =  "game_specialhi14",
    category = ACMD_GAME,
	low_priority)]
unsafe fn brawler_sak_land(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		wait(Frames=1)
		if(is_excute){
			ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=4.5, Angle=45, KBG=170, FKB=0, BKB=45, Size=6.0, X=0.0, Y=5.0, Z=11.5, X2=0.0, Y2=5.0, Z2=3.5, Hitlag=1.5, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
			ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=4.5, Angle=45, KBG=170, FKB=0, BKB=45, Size=6.0, X=0.0, Y=8.0, Z=11.5, X2=0.0, Y2=8.0, Z2=9.0, Hitlag=1.5, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
		}
		wait(Frames=1)
		if(is_excute){
			ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=4.5, Angle=45, KBG=170, FKB=0, BKB=45, Size=6.0, X=0.0, Y=5.0, Z=9.0, X2=0.0, Y2=5.0, Z2=4.0, Hitlag=1.5, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
		}
		wait(Frames=1)
		if(is_excute){
			AttackModule::clear_all()
		}
    });
}		
#[acmd_script(
    agent = "miifighter",
    script =  "game_specials1start",
    category = ACMD_GAME,
	low_priority)]
unsafe fn brawler_grounded_onslaught_start(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		FT_MOTION_RATE(FSM=0.75)
		if(is_excute){
			WorkModule::on_flag(Flag=FIGHTER_MIIFIGHTER_STATUS_WORK_ID_FLAG_100KICK_ENABLE_LANDING_MUL_SPEED_X)
		}
		frame(Frame=14)
		if(is_excute){
			WorkModule::on_flag(Flag=FIGHTER_MIIFIGHTER_STATUS_WORK_ID_FLAG_100KICK_ENABLE_MOTION_SPEED_MUL)
		}
		frame(Frame=15)
		if(is_excute){
			FighterAreaModuleImpl::enable_fix_jostle_area(2.0, 3.0)
		}
		frame(Frame=16)
		FT_MOTION_RATE(FSM=1)
		if(is_excute){
			ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=2.0, Angle=60, KBG=50, FKB=130, BKB=0, Size=4.0, X=0.0, Y=7.0, Z=3.8, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=0.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_NO_FLOOR, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KNEE)
			WorkModule::on_flag(Flag=FIGHTER_MIIFIGHTER_STATUS_WORK_ID_FLAG_100KICK_HIT_CHECK_ONOFF)
		}
		frame(Frame=31)
		if(is_excute){
			AttackModule::clear_all()
			WorkModule::off_flag(Flag=FIGHTER_MIIFIGHTER_STATUS_WORK_ID_FLAG_100KICK_HIT_CHECK_ONOFF)
		}
		frame(Frame=35)
		if(is_excute){
			FighterAreaModuleImpl::enable_fix_jostle_area(4.0, 3.0)
		}
    });
}		
		
#[acmd_script(
    agent = "miifighter",
    script =  "game_specials1end",
    category = ACMD_GAME,
	low_priority)]
unsafe fn brawler_grounded_onslaught(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		frame(Frame=3)
		if(is_excute){
			ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=0.5, Angle=70, KBG=100, FKB=11, BKB=0, Size=7.0, X=0.0, Y=8.5, Z=8.5, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.7, SDI=0.0, Clang_Rebound=ATTACK_SETOFF_KIND_THRU, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=3, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_G, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_rush"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
			ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=0.5, Angle=70, KBG=100, FKB=11, BKB=0, Size=7.0, X=0.0, Y=8.5, Z=8.5, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.7, SDI=0.0, Clang_Rebound=ATTACK_SETOFF_KIND_THRU, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=3, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_A, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_rush"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
		}
		frame(Frame=5)
		if(is_excute){
			AttackModule::clear_all()
		}
		frame(Frame=7)
		if(is_excute){
			ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=0.5, Angle=367, KBG=100, FKB=16, BKB=0, Size=7.5, X=0.0, Y=9.0, Z=9.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.7, SDI=0.0, Clang_Rebound=ATTACK_SETOFF_KIND_THRU, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=2, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_rush"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_PUNCH)
		}
		frame(Frame=10)
		if(is_excute){
			AttackModule::clear_all()
		}
		frame(Frame=12)
		if(is_excute){
			ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=0.5, Angle=367, KBG=100, FKB=50, BKB=0, Size=6.5, X=0.0, Y=9.0, Z=9.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.7, SDI=0.0, Clang_Rebound=ATTACK_SETOFF_KIND_THRU, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=3, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_rush"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
		}
		frame(Frame=13)
		if(is_excute){
			AttackModule::clear_all()
		}
		frame(Frame=14)
		if(is_excute){
			ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=0.5, Angle=367, KBG=100, FKB=50, BKB=0, Size=6.5, X=0.0, Y=9.0, Z=9.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.7, SDI=0.0, Clang_Rebound=ATTACK_SETOFF_KIND_THRU, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=2, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_rush"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
		}
		frame(Frame=16)
		if(is_excute){
			AttackModule::clear_all()
			WorkModule::on_flag(Flag=FIGHTER_MIIFIGHTER_STATUS_WORK_ID_FLAG_100KICK_DISABLE_OPPONENT_PASSIVE)
		}
		frame(Frame=17)
		if(is_excute){
			ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=1.5, Angle=270, KBG=80, FKB=44, BKB=40, Size=6.5, X=0.0, Y=8.0, Z=8.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=0.0, Clang_Rebound=ATTACK_SETOFF_KIND_THRU, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=3, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_rush"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_PUNCH)
		}
		frame(Frame=19)
		if(is_excute){
			AttackModule::clear_all()
		}
		frame(Frame=22)
		if(is_excute){
			WorkModule::off_flag(Flag=FIGHTER_MIIFIGHTER_STATUS_WORK_ID_FLAG_100KICK_DISABLE_OPPONENT_PASSIVE)
		}
		frame(Frame=27)
		if(is_excute){
			ATTACK(ID=0, Part=0, Bone=hash40("toer"), Damage=7.0, Angle=80, KBG=110, FKB=0, BKB=58, Size=7.0, X=0.0, Y=0.0, Z=0.0, X2=-6.0, Y2=-2.0, Z2=1.0, Hitlag=1.5, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
		}
		frame(Frame=29)
		FT_MOTION_RATE(FSM=0.458)
		if(is_excute){
			AttackModule::clear_all()
		}
		frame(Frame=53)
		FT_MOTION_RATE(FSM=1)
    });
}
#[acmd_script(
    agent = "miifighter_ironball",
    script =  "game_fly",
    category = ACMD_GAME,
	low_priority)]
unsafe fn brawler_shotput(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		if(is_excute){
			ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=15.0, Angle=47, KBG=58, FKB=0, BKB=50, Size=2.2, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.5, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_THRU, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=-3, Trip=-1.0, Rehit=0, Reflectable=true, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_OBJECT)
			AttackModule::enable_safe_pos()
		}
    });
}	
#[acmd_script(
    agent = "miifighter",
    scripts =  ["game_specialairlw1end", "game_speciallw1end"],
    category = ACMD_GAME,
	low_priority)]
unsafe fn brawler_hoa_end(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		frame(Frame=12)
		if(is_excute){
			ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=9.5, Angle=75, KBG=109, FKB=0, BKB=65, Size=5.5, X=0.0, Y=7.5, Z=6.0, X2=0.0, Y2=17.0, Z2=6.0, Hitlag=1.8, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_PUNCH)
		}
		wait(Frames=1)
		if(is_excute){
			AttackModule::clear_all()
		}
		frame(Frame=18)
		FT_MOTION_RATE(FSM=0.75)
		frame(Frame=60)
		FT_MOTION_RATE(FSM=1)
    });
}	
#[acmd_script(
    agent = "miifighter_ironball",
    script =  "effect_fly",
    category = ACMD_EFFECT,
	low_priority)]
unsafe fn brawler_shotput_eff(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
    });
}	
#[fighter_frame( agent = FIGHTER_KIND_MIIFIGHTER )]
fn brawler_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent); 
		let status_kind = smash::app::lua_bind::StatusModule::status_kind(boma);
		let fighter_kind = smash::app::utility::get_kind(boma);
		let lua_state = fighter.lua_state_agent;
		let ENTRY_ID = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
		let remaining_hitstun = WorkModule::get_float(boma, *FIGHTER_INSTANCE_WORK_ID_FLOAT_DAMAGE_REACTION_FRAME);
		let total_hitstun = WorkModule::get_float(boma, *FIGHTER_INSTANCE_WORK_ID_FLOAT_DAMAGE_REACTION_FRAME_LAST);	
		let frame = MotionModule::frame(boma);
		let situation_kind = StatusModule::situation_kind(boma);
		let cat1 = ControlModule::get_command_flag_cat(boma, 0);
		let kinetic_type = KineticModule::get_kinetic_type(boma);
		let end_frame = MotionModule::end_frame(boma);
		let last_frame_cancel = (end_frame-frame)-2.0;
		let stick_x = ControlModule::get_stick_x(boma) * PostureModule::lr(boma);
		if fighter_kind == *FIGHTER_KIND_MIIFIGHTER {
			
			//HOA replacement
			if [hash40("special_lw1_loop")].contains(&MotionModule::motion_kind(boma)) {
				StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_FALL, true);
				KineticModule::clear_speed_all(boma);
			};
			if [hash40("special_lw1")].contains(&MotionModule::motion_kind(boma)) && frame >= 79.0 {
				StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_WAIT, true);
			};
			if [hash40("special_lw1")].contains(&MotionModule::motion_kind(boma)) && frame >= 55.0 {
				if ray_check_pos(boma, 0.0, -1.0, false) == 0 {
					StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_FALL, true);
				};
			};
			if [hash40("special_air_lw1")].contains(&MotionModule::motion_kind(boma))  {
				if MotionModule::frame(boma) >= 60.0 {
					if KineticModule::get_kinetic_type(boma) != *FIGHTER_KINETIC_TYPE_FALL {
						KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_FALL);
					};
					CancelModule::enable_cancel(boma);
				} else {
					if KineticModule::get_kinetic_type(boma) != *FIGHTER_KINETIC_TYPE_MOTION_AIR {
						KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_MOTION_AIR);
					};
				};
				if frame >= 61.0 {
					StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_FALL, true);
					KineticModule::clear_speed_all(boma);
				};
				BAN_DOWNB[ENTRY_ID] = true;
			};
			if situation_kind != *SITUATION_KIND_AIR {
				BAN_DOWNB[ENTRY_ID] = false;
			};
			if WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_CUSTOMIZE_SPECIAL_LW_NO) == 0 {
				if BAN_DOWNB[ENTRY_ID] == true {
					CAN_DOWNB[ENTRY_ID] = 1;
				} else {
					CAN_DOWNB[ENTRY_ID] = 0;
				};
			};
			//HOA Replaced with Airdash
			/*
			if status_kind == *FIGHTER_MIIFIGHTER_STATUS_KIND_SPECIAL_LW1_AIR {
				BAN_DOWNB[ENTRY_ID] = true;
				DOWNB_COOLDOWN[ENTRY_ID] = 30;
				StatusModule::set_keep_situation_air(boma, true);
				if frame >= last_frame_cancel {
					StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_FALL, true);
				};
				if frame >= 8.0 {
					CancelModule::enable_cancel(boma);
				};
			};
			if DOWNB_COOLDOWN[ENTRY_ID] > 0 {
				DOWNB_COOLDOWN[ENTRY_ID] -= 1;
			};
			if status_kind == *FIGHTER_MIIFIGHTER_STATUS_KIND_SPECIAL_LW1_GROUND {
				if frame >= 2.0 {
					StatusModule::change_status_request_from_script(boma, *FIGHTER_MIIFIGHTER_STATUS_KIND_SPECIAL_LW1_AIR, true);
				};
			};
			if [hash40("special_lw1_loop")].contains(&MotionModule::motion_kind(boma)) {
				StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_FALL, true);
			};
			if situation_kind != *SITUATION_KIND_AIR {
				BAN_DOWNB[ENTRY_ID] = false;
			};
			if BAN_DOWNB[ENTRY_ID] == true || situation_kind == *SITUATION_KIND_GROUND || DOWNB_COOLDOWN[ENTRY_ID] > 0 {
				CAN_DOWNB[ENTRY_ID] = 1;
			} else {
				CAN_DOWNB[ENTRY_ID] = 0;
			};
			*/
			//HOA Special Kick 
			if status_kind == *FIGHTER_MIIFIGHTER_STATUS_KIND_SPECIAL_LW2_KICK_LANDING && WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_CUSTOMIZE_SPECIAL_LW_NO) == 0{
				if MotionModule::frame(boma) > 12.0 {
					CancelModule::enable_cancel(boma);
				};
			};
			//CT Special Kick Bounce
			if status_kind == *FIGHTER_MIIFIGHTER_STATUS_KIND_SPECIAL_LW2_KICK && WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_CUSTOMIZE_SPECIAL_LW_NO) == 2 && AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT){
				StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_FALL_AERIAL, true);
				KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_JUMP);
			};
			//CT Reflect
			if [hash40("special_lw3"), hash40("special_air_lw3")].contains(&MotionModule::motion_kind(boma)) && SearchModule::is_inflict(boma) {
				MotionModule::change_motion(boma, smash::phx::Hash40::new("throw_f"), 12.0, 1.0, false, 0.0, false, false);
				acmd!(lua_state, {
					sv_module_access::shield(MA_MSC_CMD_REFLECTOR, COLLISION_KIND_REFLECTOR, 0, hash40("top"), 5.0, 0.0, 6.5, 3.5, 0.0, 6.5, 5.5, 1.4, 1.5, 50, false, 0.5, FIGHTER_REFLECTOR_GROUP_HOMERUNBAT)
				});
				println!("reflection!");
				COUNTER_IS[ENTRY_ID] = true;
			};
			if MotionModule::motion_kind(boma) == hash40("throw_f") && MotionModule::frame(boma) > 20.0 {
				acmd!(lua_state, {
					sv_module_access::search(MA_MSC_CMD_SEARCH_SEARCH_SCH_CLR_ALL)
					COL_NORMAL()
				});
			};
			if MotionModule::motion_kind(boma) == hash40("special_lw3") && COUNTER_IS[ENTRY_ID] == true {
				MotionModule::change_motion(boma, smash::phx::Hash40::new("throw_f"), 12.0, 1.0, false, 0.0, false, false);
				COUNTER_IS[ENTRY_ID] = false;
				acmd!(lua_state, {
					sv_module_access::search(MA_MSC_CMD_SEARCH_SEARCH_SCH_CLR_ALL)
					COL_NORMAL()
				});
			};
			if status_kind != *FIGHTER_STATUS_KIND_THROW && MotionModule::motion_kind(boma) == hash40("throw_f"){
				StatusModule::set_keep_situation_air(boma, true);
			};
			if [hash40("special_lw3"), hash40("special_air_lw3"), hash40("throw_f")].contains(&MotionModule::motion_kind(boma)) == false {
				COUNTER_IS[ENTRY_ID] = false;
			};
			
			//Onslaught
			if MotionModule::motion_kind(boma) == hash40("special_air_s1_start") && MotionModule::frame(boma) >50.0 {
				StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_FALL, true);
			};
			if [hash40("special_air_s1"), hash40("special_air_s1_end")].contains(&MotionModule::motion_kind(boma))  && MotionModule::frame(boma) > 40.0 {
				if MotionModule::frame(boma) > 51.0 {
					StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_FALL, true);
				} else {
					CancelModule::enable_cancel(boma);
				};
			};
			if [hash40("special_s1"), hash40("special_s1_end")].contains(&MotionModule::motion_kind(boma))  && MotionModule::frame(boma) > 53.0 {
				CancelModule::enable_cancel(boma);
			};
			
			//ESK
			if ESK_CHARGE[ENTRY_ID] > 0 && total_hitstun > 0.0 {
				if StatusModule::prev_status_kind(boma, 0) == *FIGHTER_STATUS_KIND_SPECIAL_N {
					ESK_CHARGE[ENTRY_ID] = 0;
				};
			};
			if smash::app::sv_information::is_ready_go() == false || [*FIGHTER_STATUS_KIND_WIN, *FIGHTER_STATUS_KIND_LOSE, *FIGHTER_STATUS_KIND_DEAD].contains(&status_kind) {
				ESK_CHARGE[ENTRY_ID] = 0;
			};
			if [hash40("special_n3"), hash40("special_air_n3")].contains(&MotionModule::motion_kind(boma)) {
				if motion_duration(boma) == 5 {
					if (ControlModule::get_stick_x(boma)*PostureModule::lr(boma)) < -0.2 {
						PostureModule::reverse_lr(boma);
						PostureModule::update_rot_y_lr(boma);
						let stop_rise  = smash::phx::Vector3f { x: -1.0, y: 1.0, z: 1.0 };
						KineticModule::mul_speed(boma, &stop_rise, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
					};
				};
				if MotionModule::frame(boma) > 12.0 && MotionModule::frame(boma) < 20.0 {
					if ESK_CHARGE[ENTRY_ID] % 18 == 0 || MotionModule::frame(boma) == 13.0 {
						EffectModule::req_follow(boma, smash::phx::Hash40::new_raw(0x198abfaca9), smash::phx::Hash40::new("toel"), &ESK, &ESK, 1.0, true, 0, 0, 0, 0, 0, true, true);
					};
					if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_GUARD) {
						if StatusModule::situation_kind(boma) == *SITUATION_KIND_GROUND {
							EffectModule::kill_kind(boma, smash::phx::Hash40::new_raw(0x198abfaca9), false, false);
							macros::STOP_SE(fighter, smash::phx::Hash40::new("se_miifighter_special_c3_n01"));
							StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_GUARD_ON, true);
						} else if WorkModule::is_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_DISABLE_ESCAPE_AIR) == false{
							EffectModule::kill_kind(boma, smash::phx::Hash40::new_raw(0x198abfaca9), false, false);
							macros::STOP_SE(fighter, smash::phx::Hash40::new("se_miifighter_special_c3_n01"));
							StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ESCAPE_AIR, true);
						};
					};
					if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_JUMP) {
						if StatusModule::situation_kind(boma) == *SITUATION_KIND_GROUND {
							macros::STOP_SE(fighter, smash::phx::Hash40::new("se_miifighter_special_c3_n01"));
							EffectModule::kill_kind(boma, smash::phx::Hash40::new_raw(0x198abfaca9), false, false);
							StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_JUMP_SQUAT, true);
						} else if WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT) < WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT_MAX) {
							EffectModule::kill_kind(boma, smash::phx::Hash40::new_raw(0x198abfaca9), false, false);
							macros::STOP_SE(fighter, smash::phx::Hash40::new("se_miifighter_special_c3_n01"));
							StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_JUMP_AERIAL, true);
						};
					};
					if motion_duration(boma) % 60 == 0 || MotionModule::frame(boma) == 13.0 && ControlModule::check_button_off(boma, *CONTROL_PAD_BUTTON_JUMP) && ControlModule::check_button_off(boma, *CONTROL_PAD_BUTTON_GUARD){
						macros::PLAY_SE(fighter, smash::phx::Hash40::new("se_miifighter_special_c3_n01"));
					};
					let cat1 = ControlModule::get_command_flag_cat(boma, 0);
					if (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_ANY) != 0 || ESK_CHARGE[ENTRY_ID] >= 186 {
						EffectModule::kill_kind(boma, smash::phx::Hash40::new_raw(0x198abfaca9), false, false);
						macros::STOP_SE(fighter, smash::phx::Hash40::new("se_miifighter_special_c3_n01"));
						if StatusModule::situation_kind(boma) == *SITUATION_KIND_GROUND {
							macros::SET_SPEED_EX(fighter, 1.0, 0.25, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
							MotionModule::change_motion(boma, smash::phx::Hash40::new("special_n3"), 40.0, 1.0, false, 0.0, false, false);
						} else {
							macros::SET_SPEED_EX(fighter, 1.4, 0.45, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
							MotionModule::change_motion(boma, smash::phx::Hash40::new("special_air_n3"), 40.0, 1.0, false, 0.0, false, false);
						};
					};
				};
				if MotionModule::frame(boma) > 19.0 && MotionModule::frame(boma) < 39.0 {
					if StatusModule::situation_kind(boma) == *SITUATION_KIND_GROUND {
						MotionModule::change_motion(boma, smash::phx::Hash40::new("special_n3"), 16.0, 1.0, false, 0.0, false, false);
					} else {
						MotionModule::change_motion(boma, smash::phx::Hash40::new("special_air_n3"), 16.0, 1.0, false, 0.0, false, false);
					};
				};
				if ESK_CHARGE[ENTRY_ID] == 185 {
					ESK_CHARGE[ENTRY_ID] += 1;
					smash::app::FighterUtil::flash_eye_info(boma);
					EffectModule::req_follow(boma, smash::phx::Hash40::new("sys_smash_flash"), smash::phx::Hash40::new("footl"), &ESK, &ESK, 0.5, true, 0, 0, 0, 0, 0, true, true) as u32;
					if StatusModule::situation_kind(boma) == *SITUATION_KIND_GROUND {
						macros::STOP_SE(fighter, smash::phx::Hash40::new("se_miifighter_special_c3_n01"));
						EffectModule::kill_kind(boma, smash::phx::Hash40::new_raw(0x198abfaca9), false, false);
						StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_WAIT, true);
					} else {
						EffectModule::kill_kind(boma, smash::phx::Hash40::new_raw(0x198abfaca9), false, false);
						macros::STOP_SE(fighter, smash::phx::Hash40::new("se_miifighter_special_c3_n01"));
						StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_FALL, true);
					};
				};
				if ESK_CHARGE[ENTRY_ID] < 187 {
					ESK_CHARGE[ENTRY_ID] += 1;
				};
			};
		};
    }
}
#[acmd_script(
    agent = "miifighter",
    scripts =  ["effect_specialairlw2kick"],
    category = ACMD_EFFECT,
	low_priority)]
unsafe fn brawler_eff_fjk(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		frame(Frame=8)
		if(is_excute){
			rust{
				if WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_CUSTOMIZE_SPECIAL_LW_NO) != 0 {
					acmd!(lua_state, {EFFECT_FOLLOW(0x18c4d672abu64, hash40("top"), 0, 0, -5, 140, 0, 0, 1.3, true)});
				};
			}
		}
    });
}		
		
		
pub fn install() {
    smashline::install_acmd_scripts!(
        brawler_fair,
		brawler_uair,
		brawler_bair,
		brawler_counter,
		brawler_esk,
		brawler_esk_s,
		brawler_esk_e,
		brawler_fjk,
		brawler_eff_fjk,
		brawler_dsmash,
		brawler_jab1,
		brawler_jab2,
		brawler_sak_start,
		brawler_sak_land,
		brawler_shotput,
		brawler_grounded_onslaught,
		brawler_grounded_onslaught_start,
		brawler_foot,
		brawler_foot_eff,
		brawler_foot_air,
		brawler_foot_air_eff,
		brawler_foot_snd,
		brawler_foot_air_snd
    );
    smashline::install_agent_frames!(
        brawler_frame
    );
}
