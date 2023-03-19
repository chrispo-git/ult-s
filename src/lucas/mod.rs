use smash::hash40;
use smash::lib::lua_const::*;
use smash::lua2cpp::*;
use smash::app::lua_bind::*;
use smashline::*;
use smash_script::*;
use crate::util::*;
#[acmd_script(
    agent = "lucas",
    scripts =  ["game_speciallwend", "game_specialairlwend"],
    category = ACMD_GAME,
	low_priority)]
unsafe fn lucas_downb(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		frame(Frame=1)
		if(is_excute){
			ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=8.0, Angle=75, KBG=98, FKB=0, BKB=40, Size=3.5, X=0.0, Y=6.3, Z=7.0, X2=0.0, Y2=6.3, Z2=9.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_elec"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_ELEC, Type=ATTACK_REGION_PSI)
		}
		wait(Frames=1)
		if(is_excute){
			AttackModule::clear_all()
		}
    });
}
#[acmd_script(
    agent = "lucas",
    script =  "game_throwlw",
    category = ACMD_GAME,
	low_priority)]
unsafe fn lucas_dthrow(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		if(is_excute){
			ATTACK_ABS(Kind=FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, ID=0, Damage=6.5, Angle=80, KBG=51, FKB=0, BKB=78, Hitlag=1.0, Unk=1.0, FacingRestrict=ATTACK_LR_CHECK_F, Unk=0.0, Unk=true, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_NONE, Type=ATTACK_REGION_THROW)
			ATTACK_ABS(Kind=FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, ID=0, Damage=3.0, Angle=361, KBG=100, FKB=0, BKB=60, Hitlag=0.0, Unk=1.0, FacingRestrict=ATTACK_LR_CHECK_F, Unk=0.0, Unk=true, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_NONE, Type=ATTACK_REGION_THROW)
		}
		frame(Frame=41)
		if(is_excute){
			ATK_HIT_ABS(FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, hash40("throw"), WorkModule::get_int64(module_accessor,*FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT), WorkModule::get_int64(module_accessor,*FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP), WorkModule::get_int64(module_accessor,*FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO))
			AttackModule::clear_all()
		}
    });
}
#[acmd_script(
    agent = "lucas",
    script =  "game_attackairn",
    category = ACMD_GAME,
	low_priority)]
unsafe fn lucas_nair(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		FT_MOTION_RATE(FSM=0.7142857142857143)
		if(is_excute){
			WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
		}
		frame(Frame=7)
		FT_MOTION_RATE(FSM=1)
		if(is_excute){
			ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=2.0, Angle=366, KBG=70, FKB=0, BKB=20, Size=6.9, X=0.0, Y=5.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.44, SDI=2.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=5, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_elec"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_MAGIC, Type=ATTACK_REGION_PSI)
		}
		frame(Frame=22)
		if(is_excute){
			AttackModule::clear_all()
		}
		frame(Frame=26)
		if(is_excute){
			ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=4.0, Angle=60, KBG=140, FKB=0, BKB=40, Size=10.2, X=0.0, Y=3.7, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.2, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_elec"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_MAGIC, Type=ATTACK_REGION_PSI)
		}
		wait(Frames=1)
		if(is_excute){
			AttackModule::clear_all()
		}
		frame(Frame=37)
		if(is_excute){
			WorkModule::off_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
		}
    });
}		
#[acmd_script(
    agent = "lucas",
    script =  "game_attackairf",
    category = ACMD_GAME,
	low_priority)]
unsafe fn lucas_fair(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		frame(Frame=2)
		if(is_excute){
			WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
		}
		frame(Frame=9)
		if(is_excute){
			ATTACK(ID=0, Part=0, Bone=hash40("footl"), Damage=9.0, Angle=361, KBG=100, FKB=0, BKB=10, Size=4.2, X=0.0, Y=2.9, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.8, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
			ATTACK(ID=1, Part=0, Bone=hash40("toel"), Damage=12.5, Angle=30, KBG=103, FKB=0, BKB=30, Size=4.2, X=3.0, Y=0.4, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.2, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_elec"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_MAGIC, Type=ATTACK_REGION_PSI)
		}
		frame(Frame=13)
		if(is_excute){
			AttackModule::clear_all()
		}
		frame(Frame=32)
		if(is_excute){
			CancelModule::enable_cancel()
		}
		frame(Frame=34)
		if(is_excute){
			WorkModule::off_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
		}
    });
}				
#[acmd_script(
    agent = "lucas",
    script =  "game_attackairhi",
    category = ACMD_GAME,
	low_priority)]
unsafe fn lucas_uair(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		frame(Frame=2)
		if(is_excute){
			WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
		}
		frame(Frame=7)
		if(is_excute){
			HIT_NODE(hash40("head"), HIT_STATUS_XLU)
			ATTACK(ID=0, Part=0, Bone=hash40("head"), Damage=11.0, Angle=80, KBG=95, FKB=0, BKB=20, Size=5.2, X=4.4, Y=-1.0, Z=0.0, X2=2.4, Y2=-1.0, Z2=-3.5, Hitlag=1.1, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_HEAD)
		}
		frame(Frame=11)
		if(is_excute){
			HIT_NODE(hash40("head"), HIT_STATUS_NORMAL)
			AttackModule::clear_all()
		}
		FT_MOTION_RATE(FSM=0.77)
		frame(Frame=40)
		if(is_excute){
			WorkModule::off_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
		}
		FT_MOTION_RATE(FSM=1)
    });
}					
#[acmd_script(
    agent = "lucas",
    script =  "game_attackairlw",
    category = ACMD_GAME,
	low_priority)]
unsafe fn lucas_dair(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		if(is_excute){
			WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
		}
		frame(Frame=10)
		if(is_excute){
			ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=3.5, Angle=363, KBG=10, FKB=0, BKB=42, Size=4.2, X=0.0, Y=-4.0, Z=0.3, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.66, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_elec"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_MAGIC, Type=ATTACK_REGION_PSI)
			ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=3.5, Angle=270, KBG=100, FKB=20, BKB=0, Size=3.2, X=0.0, Y=1.0, Z=0.3, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.66, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_elec"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_MAGIC, Type=ATTACK_REGION_PSI)
		}
		wait(Frames=3)
		if(is_excute){
			AttackModule::clear_all()
		}
		frame(Frame=18)
		if(is_excute){
			ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=5.0, Angle=270, KBG=110, FKB=0, BKB=10, Size=5.5, X=0.0, Y=-4.0, Z=0.3, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_elec"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_MAGIC, Type=ATTACK_REGION_PSI)
			ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=5.0, Angle=270, KBG=110, FKB=0, BKB=10, Size=3.5, X=0.0, Y=1.0, Z=0.3, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_elec"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_MAGIC, Type=ATTACK_REGION_PSI)
		}
		frame(Frame=21)
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
    agent = "lucas",
    script =  "effect_attackairlw",
    category = ACMD_EFFECT,
	low_priority)]
unsafe fn lucas_dair_eff(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		frame(Frame=1)
		if(is_excute){
			EFFECT(hash40("sys_smash_flash"), hash40("top"), 0, -4, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, true)
			LAST_EFFECT_SET_RATE(1.5)
		}
		frame(Frame=10)
		if(is_excute){
			EFFECT_FOLLOW(0x0d2b5b5277u64, hash40("kneer"), 3, 0, 0, 0, 90, 0, 0.65, true)
		}
		frame(Frame=15)
		if(is_excute){
			EFFECT_OFF_KIND(0x0d2b5b5277u64, false, false)
		}
		frame(Frame=18)
		if(is_excute){
			EFFECT_FOLLOW(0x0d2b5b5277u64, hash40("kneel"), 3, 0, 0, 0, 90, 0, 0.7, true)
		}
		frame(Frame=23)
		if(is_excute){
			EFFECT_OFF_KIND(0x0d2b5b5277u64, false, false)
		}
    });
}
#[acmd_script(
    agent = "lucas",
    script =  "sound_attackairlw",
    category = ACMD_SOUND,
	low_priority)]
unsafe fn lucas_dair_snd(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		frame(Frame=1)
		if(is_excute){
			PLAY_SE(hash40("se_lucas_attackair_l05"))
		}
		wait(Frames=7)
		if(is_excute){
			PLAY_SEQUENCE(hash40("seq_lucas_rnd_attack"))
			PLAY_SE(hash40("se_lucas_attackair_l01"))
		}
		wait(Frames=8)
		if(is_excute){
			PLAY_SE(hash40("se_lucas_attackair_l02"))
		}
		wait(Frames=8)
		if(is_excute){
			PLAY_SE(hash40("se_lucas_attackair_l04"))
		}
    });
}		
#[acmd_script(
    agent = "lucas",
    script =  "game_attackairb",
    category = ACMD_GAME,
	low_priority)]
unsafe fn lucas_bair(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		frame(Frame=3)
		if(is_excute){
			WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
		}
		frame(Frame=15)
		if(is_excute){
			ATTACK(ID=0, Part=0, Bone=hash40("kneer"), Damage=9.0, Angle=361, KBG=80, FKB=0, BKB=30, Size=3.0, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_B, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_elec"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_MAGIC, Type=ATTACK_REGION_PSI)
			ATTACK(ID=1, Part=0, Bone=hash40("kneer"), Damage=12.0, Angle=280, KBG=90, FKB=0, BKB=30, Size=4.5, X=4.5, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_B, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_A, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_elec"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_MAGIC, Type=ATTACK_REGION_PSI)
			ATTACK(ID=2, Part=0, Bone=hash40("kneer"), Damage=12.0, Angle=280, KBG=90, FKB=0, BKB=30, Size=4.5, X=4.5, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_B, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_G, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_elec"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_MAGIC, Type=ATTACK_REGION_PSI)
			ATTACK(ID=3, Part=0, Bone=hash40("kneer"), Damage=7.0, Angle=361, KBG=60, FKB=0, BKB=10, Size=3.5, X=8.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_B, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_elec"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_MAGIC, Type=ATTACK_REGION_PSI)
			AttackModule::set_add_reaction_frame(ID=0, Frames=4.0, Unk=false)
			AttackModule::set_add_reaction_frame(ID=1, Frames=4.0, Unk=false)
			AttackModule::set_add_reaction_frame(ID=2, Frames=4.0, Unk=false)
			AttackModule::set_add_reaction_frame(ID=3, Frames=4.0, Unk=false)
		}
		frame(Frame=19)
		if(is_excute){
			ATTACK(ID=0, Part=0, Bone=hash40("kneer"), Damage=9.0, Angle=361, KBG=80, FKB=0, BKB=30, Size=3.0, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_PART, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_elec"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_MAGIC, Type=ATTACK_REGION_PSI)
			ATTACK(ID=2, Part=0, Bone=hash40("kneer"), Damage=12.0, Angle=361, KBG=90, FKB=0, BKB=30, Size=4.5, X=4.5, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_PART, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_G, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_elec"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_MAGIC, Type=ATTACK_REGION_PSI)
			ATTACK(ID=3, Part=0, Bone=hash40("kneer"), Damage=7.0, Angle=361, KBG=60, FKB=0, BKB=10, Size=3.5, X=8.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_PART, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_elec"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_MAGIC, Type=ATTACK_REGION_PSI)
			AttackModule::set_add_reaction_frame(ID=0, Frames=4.0, Unk=false)
			AttackModule::set_add_reaction_frame(ID=2, Frames=4.0, Unk=false)
			AttackModule::set_add_reaction_frame(ID=3, Frames=4.0, Unk=false)
		}
		frame(Frame=20)
		if(is_excute){
			AttackModule::clear(ID=1, false)
		}
		frame(Frame=23)
		if(is_excute){
			AttackModule::clear_all()
		}
		frame(Frame=34)
		if(is_excute){
			WorkModule::off_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
		}
    });
}		
#[acmd_script(
    agent = "lucas",
    scripts =  ["game_specialnstart", "game_specialairnstart"],
    category = ACMD_GAME,
	low_priority)]
unsafe fn lucas_pk_freeze(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		frame(Frame=12)
		if(is_excute){
			ATTACK(ID=0, Part=0, Bone=hash40("hip"), Damage=13.0, Angle=40, KBG=65, FKB=0, BKB=70, Size=15.0, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.2, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=5, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_FIRE, Type=ATTACK_REGION_PSI)
			QUAKE(CAMERA_QUAKE_KIND_L)
		}
		wait(Frames=1)
		if(is_excute){
			AttackModule::clear_all()
		}
    });
}		
#[acmd_script(
    agent = "lucas",
    scripts =  ["effect_specialnstart", "effect_specialairnstart"],
    category = ACMD_EFFECT,
	low_priority)]
unsafe fn lucas_pk_freeze_eff(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		frame(Frame=2)
		if(is_excute){
			EFFECT_FLW_POS(0x0fc6a66d24u64, hash40("top"), 0, 9, 0, 0, 0, 0, 1, true)
		}
		frame(Frame=12)
		if(is_excute){
			EFFECT_OFF_KIND(0x0fc6a66d24u64, false, false)
			EFFECT(hash40("lucas_pkfr_bullet_ed"), hash40("hip"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false)
			LAST_EFFECT_SET_RATE(0.5)
			EFFECT(hash40("lucas_pkfr_bomb_max"), hash40("hip"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false)
		}
    });
}
#[acmd_script(
    agent = "lucas",
    scripts =  ["sound_specialnstart", "sound_specialairnstart"],
    category = ACMD_SOUND,
	low_priority)]
unsafe fn lucas_pk_freeze_snd(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		frame(Frame=11)
		if(is_excute){
			PLAY_SE(hash40("vc_lucas_attack04"))
			PLAY_SE_REMAIN(hash40("se_lucas_special_n04_ll"))
		}
    });
}
	
#[fighter_frame( agent = FIGHTER_KIND_LUCAS )]
fn lucas_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent); 
		let status_kind = smash::app::lua_bind::StatusModule::status_kind(boma);
		let frame = MotionModule::frame(boma);
		let end_frame = MotionModule::end_frame(boma) as f32;
		let motion_kind = MotionModule::motion_kind(boma);
		let situation_kind = StatusModule::situation_kind(boma);
		let ENTRY_ID = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
		if [hash40("special_air_n_start"), hash40("special_n_start")].contains(&motion_kind) {
			CAN_NEUTRALB[ENTRY_ID] = 1;
			if end_frame - frame < 3.0 {
				if situation_kind == *SITUATION_KIND_AIR {
					StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_FALL, false);
				} else {
					StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_WAIT, false);
				};
			};
		};
		if situation_kind != *SITUATION_KIND_AIR {
			CAN_NEUTRALB[ENTRY_ID] = 0;
		};
    }
}
		
pub fn install() {
    smashline::install_acmd_scripts!(
		lucas_nair,
		lucas_fair,
		lucas_dair,
		lucas_dair_eff,
		lucas_dair_snd,
		lucas_bair,
		lucas_uair,
		lucas_dthrow,
		lucas_downb,
		lucas_pk_freeze,
		lucas_pk_freeze_eff, 
		lucas_pk_freeze_snd
    );
    smashline::install_agent_frames!(
        lucas_frame
    );
}
