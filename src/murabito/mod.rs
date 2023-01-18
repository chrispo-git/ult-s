use smash::hash40;
use smash::lib::lua_const::*;
use smash::app::lua_bind::*;
use smash::lua2cpp::*;
use smashline::*;
use smash_script::*;
use smash::phx::Hash40;

#[acmd_script(
    agent = "murabito",
    script =  "game_attack11",
    category = ACMD_GAME,
	low_priority)]
unsafe fn toad_jab1(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		frame(Frame=2)
		if(is_excute){
			ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=2.0, Angle=361, KBG=25, FKB=0, BKB=20, Size=2.5, X=0.0, Y=5.5, Z=6.5, X2=0.0, Y2=5.5, Z2=7.5, Hitlag=1.5, SDI=0.8, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_PUNCH)
			ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=2.0, Angle=180, KBG=10, FKB=0, BKB=15, Size=3.0, X=-1.5, Y=5.5, Z=11.0, X2=-1.5, Y2=4.5, Z2=11.0, Hitlag=1.5, SDI=0.8, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_FIGHTER, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_PUNCH)
			ATTACK(ID=2, Part=0, Bone=hash40("top"), Damage=2.0, Angle=180, KBG=10, FKB=0, BKB=15, Size=3.0, X=1.5, Y=5.5, Z=11.0, X2=1.5, Y2=4.5, Z2=11.0, Hitlag=1.5, SDI=0.8, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_FIGHTER, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_PUNCH)
			ATTACK(ID=3, Part=0, Bone=hash40("top"), Damage=2.0, Angle=361, KBG=10, FKB=0, BKB=15, Size=3.0, X=0.0, Y=5.5, Z=11.0, X2=0.0, Y2=4.5, Z2=11.0, Hitlag=1.5, SDI=0.8, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_PUNCH)
		}
		wait(Frames=2)
		if(is_excute){
			AttackModule::clear_all()
			WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO)
		}
		frame(Frame=7)
		if(is_excute){
			WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_FLAG_ENABLE_NO_HIT_COMBO)
		}
    });
}	
#[acmd_script(
    agent = "murabito",
    script =  "effect_attack11",
    category = ACMD_EFFECT,
	low_priority)]
unsafe fn toad_jab1_eff(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		frame(Frame=2)
		if(is_excute){
			EFFECT(hash40("sys_attack_impact"), hash40("top"), 11.0, 5.5, 0, 0, 0, 0, 1, 0, 1, 1, 0, 0, 360, false)
			LAST_EFFECT_SET_ALPHA(0.7)
		}
    });
}	
#[acmd_script(
    agent = "murabito",
    script =  "game_attack12",
    category = ACMD_GAME,
	low_priority)]
unsafe fn toad_jab2(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		frame(Frame=2)
		if(is_excute){
			ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=2.0, Angle=361, KBG=25, FKB=0, BKB=20, Size=2.5, X=0.0, Y=5.5, Z=6.5, X2=0.0, Y2=5.5, Z2=7.5, Hitlag=1.5, SDI=0.8, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_PUNCH)
			ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=2.0, Angle=361, KBG=30, FKB=0, BKB=20, Size=3.0, X=-1.5, Y=5.5, Z=11.0, X2=-1.5, Y2=4.5, Z2=11.0, Hitlag=1.5, SDI=0.8, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_FIGHTER, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_PUNCH)
			ATTACK(ID=2, Part=0, Bone=hash40("top"), Damage=2.0, Angle=361, KBG=30, FKB=0, BKB=20, Size=3.0, X=1.5, Y=5.5, Z=11.0, X2=1.5, Y2=4.5, Z2=11.0, Hitlag=1.5, SDI=0.8, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_FIGHTER, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_PUNCH)
			ATTACK(ID=3, Part=0, Bone=hash40("top"), Damage=2.0, Angle=361, KBG=30, FKB=0, BKB=20, Size=3.0, X=0.0, Y=5.5, Z=11.0, X2=0.0, Y2=4.5, Z2=11.0, Hitlag=1.5, SDI=0.8, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_PUNCH)
			AttackModule::set_add_reaction_frame(ID=0, Frames=7.0, Unk=false)
			AttackModule::set_add_reaction_frame(ID=1, Frames=7.0, Unk=false)
			AttackModule::set_add_reaction_frame(ID=2, Frames=7.0, Unk=false)
			AttackModule::set_add_reaction_frame(ID=3, Frames=7.0, Unk=false)
		}
		wait(Frames=2)
		if(is_excute){
			AttackModule::clear_all()
		}
		frame(Frame=7)
		if(is_excute){
			WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO)
			WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_FLAG_ENABLE_NO_HIT_COMBO)
		}
    });
}	
#[acmd_script(
    agent = "murabito",
    script =  "sound_attack11",
    category = ACMD_SOUND,
	low_priority)]
unsafe fn toad_jab1_snd(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		frame(Frame=1)
		if(is_excute){
			PLAY_SE(hash40("se_murabito_swing_s"))
		}
    });
}	
#[acmd_script(
    agent = "murabito",
    script =  "effect_attack12",
    category = ACMD_EFFECT,
	low_priority)]
unsafe fn toad_jab2_eff(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		frame(Frame=2)
		if(is_excute){
			EFFECT(hash40("sys_attack_impact"), hash40("top"), 11.0, 5.5, 0, 0, 0, 0, 1, 0, 1, 1, 0, 0, 360, false)
			LAST_EFFECT_SET_ALPHA(0.7)
		}
    });
}	
#[acmd_script(
    agent = "murabito",
    script =  "sound_attack12",
    category = ACMD_SOUND,
	low_priority)]
unsafe fn toad_jab2_snd(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		frame(Frame=1)
		if(is_excute){
			PLAY_SE(hash40("se_murabito_swing_s"))
		}
    });
}	
#[acmd_script(
    agent = "murabito",
    script =  "game_attack13",
    category = ACMD_GAME,
	low_priority)]
unsafe fn toad_jab3(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		FT_MOTION_RATE(FSM=0.6)
		wait(Frames=10)
		FT_MOTION_RATE(FSM=1)
		frame(Frame=12)
		if(is_excute){
			ATTACK(ID=0, Part=0, Bone=hash40("head"), Damage=10.0, Angle=43, KBG=83, FKB=0, BKB=57, Size=8.5, X=4.4, Y=-1.0, Z=0.0, X=4.4, Y=-1.0, Z=0.0, Hitlag=1.2, SDI=0.8, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_PUNCH)
		}
		frame(Frame=14)
		if(is_excute){
			AttackModule::clear_all()
		}
    });
}	
#[acmd_script(
    agent = "murabito",
    script =  "effect_attack13",
    category = ACMD_EFFECT,
	low_priority)]
unsafe fn toad_jab3_eff(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		frame(Frame=12)
		if(is_excute){
			EFFECT(hash40("sys_attack_impact"), hash40("head"), 4.4, -1.0, 0.0, 0, 0, 0, 2.5, 0, 1, 1, 0, 0, 360, false)
			LAST_EFFECT_SET_ALPHA(0.7)
		}
		frame(Frame=26)
		if(is_excute){
			LANDING_EFFECT(hash40("sys_down_smoke"), hash40("top"), 3, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false)
		}
    });
}	
#[acmd_script(
    agent = "murabito",
    script =  "sound_attack13",
    category = ACMD_SOUND,
	low_priority)]
unsafe fn toad_jab3_snd(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		frame(Frame=10)
		if(is_excute){
			PLAY_SE(hash40("se_murabito_swing_l"))
		}
    });
}	
#[acmd_script(
    agent = "murabito",
    script =  "game_attackairn",
    category = ACMD_GAME,
	low_priority)]
unsafe fn toad_nair(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		frame(Frame=4)
		if(is_excute){
			WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
			ATTACK(ID=0, Part=0, Bone=hash40("kneer"), Damage=10.0, Angle=361, KBG=100, FKB=0, BKB=10, Size=5.0, X=0.8, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_KICK)
			ATTACK(ID=1, Part=0, Bone=hash40("kneel"), Damage=10.0, Angle=361, KBG=100, FKB=0, BKB=10, Size=5.0, X=0.8, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_KICK)
		}
		wait(Frames=3)
		if(is_excute){
			ATTACK(ID=0, Part=0, Bone=hash40("legr"), Damage=7.0, Angle=361, KBG=70, FKB=0, BKB=15, Size=3.8, X=0.8, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
			ATTACK(ID=1, Part=0, Bone=hash40("kneel"), Damage=7.0, Angle=361, KBG=70, FKB=0, BKB=15, Size=3.8, X=0.8, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
		}
		frame(Frame=26)
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
    agent = "murabito",
    script =  "game_attackairb",
    category = ACMD_GAME,
	low_priority)]
unsafe fn toad_bair(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		frame(Frame=4)
		if(is_excute){
			WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
		}
		frame(Frame=6)
		if(is_excute){
			ATTACK(ID=0, Part=0, Bone=hash40("legc"), Damage=9.0, Angle=62, KBG=85, FKB=0, BKB=47, Size=6.8, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_B, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
		}
		frame(Frame=9)
		if(is_excute){
			ATTACK(ID=0, Part=0, Bone=hash40("legc"), Damage=5.0, Angle=62, KBG=85, FKB=0, BKB=50, Size=4.0, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_B, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
		}
		frame(Frame=15)
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
    agent = "murabito",
    script =  "sound_attackairb",
    category = ACMD_SOUND,
	low_priority)]
unsafe fn toad_bair_snd(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		frame(Frame=5)
		if(is_excute){
			PLAY_SE(hash40("se_murabito_swing_l"))
		}
    });
}	
#[acmd_script(
    agent = "murabito",
    script =  "effect_attackairb",
    category = ACMD_EFFECT,
	low_priority)]
unsafe fn toad_bair_eff(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		frame(Frame=12)
		if(is_excute){
			EFFECT(hash40("sys_attack_impact"), hash40("legc"), 0, 0.0, 0.0, 0, 0, 0, 1.5, 0, 1, 1, 0, 0, 0, false)
		}
    });
}	
#[acmd_script(
    agent = "murabito",
    script =  "game_attackairhi",
    category = ACMD_GAME,
	low_priority)]
unsafe fn toad_uair(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		frame(Frame=2)
		if(is_excute){
			WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
		}
		frame(Frame=6)
		if(is_excute){
			ATTACK(ID=0, Part=0, Bone=hash40("head"), Damage=12.0, Angle=77, KBG=85, FKB=0, BKB=30, Size=4.2, X=4.4, Y=-1.0, Z=0.0, X2=2.4, Y2=-1.0, Z2=-3.5, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_HEAD)
		}
		frame(Frame=12)
		if(is_excute){
			AttackModule::clear_all()
		}
		frame(Frame=25)
		if(is_excute){
			WorkModule::off_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
		}
    });
}
#[acmd_script(
    agent = "murabito",
    script =  "effect_attackairhi",
    category = ACMD_EFFECT,
	low_priority)]
unsafe fn toad_uair_eff(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		frame(Frame=6)
		if(is_excute){
			EFFECT_FOLLOW_FLIP(hash40("sys_attack_arc_d"), hash40("sys_attack_arc_d"), hash40("top"), 1.5, 7.5, 0, 0, -80, -105, 0.7, true, EF_FLIP_YZ)
		}
    });
}
#[acmd_script(
    agent = "murabito",
    script =  "sound_attackairhi",
    category = ACMD_SOUND,
	low_priority)]
unsafe fn toad_uair_snd(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		frame(Frame=5)
		if(is_excute){
			PLAY_SE(hash40("se_murabito_swing_m"))
		}
    });
}
#[acmd_script(
    agent = "murabito",
    script =  "game_attackairf",
    category = ACMD_GAME,
	low_priority)]
unsafe fn toad_fair(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		frame(Frame=4)
		if(is_excute){
			WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
		}
		frame(Frame=16)
		if(is_excute){
			ATTACK(ID=0, Part=0, Bone=hash40("head"), Damage=15.0, Angle=361, KBG=102, FKB=0, BKB=30, Size=6.2, X=4.4, Y=-1.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_HEAD)
		}
		frame(Frame=19)
		if(is_excute){
			AttackModule::clear_all()
		}
		frame(Frame=40)
		if(is_excute){
			WorkModule::off_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
		}
    });
}
#[acmd_script(
    agent = "murabito",
    script =  "effect_attackairf",
    category = ACMD_EFFECT,
	low_priority)]
unsafe fn toad_fair_eff(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
}
#[acmd_script(
    agent = "murabito",
    script =  "sound_attackairf",
    category = ACMD_SOUND,
	low_priority)]
unsafe fn toad_fair_snd(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		frame(Frame=15)
		if(is_excute){
			PLAY_SE(hash40("se_murabito_swing_l"))
		}
    });
}
#[acmd_script(
    agent = "murabito",
    script =  "effect_burst",
    category = ACMD_EFFECT,
	low_priority)]
unsafe fn toad_shell_burst_eff(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
}
#[acmd_script(
    agent = "murabito",
    script =  "game_attackdash",
    category = ACMD_GAME,
	low_priority)]
unsafe fn toad_da(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		frame(Frame=6)
		if(is_excute){
			ArticleModule::generate_article(FIGHTER_MURABITO_GENERATE_ARTICLE_FLOWERPOT, false, 0)
			sv_module_access::damage(MSC=MA_MSC_DAMAGE_DAMAGE_NO_REACTION, Type=DAMAGE_NO_REACTION_MODE_DAMAGE_POWER, DamageThreshold=7)
			ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=8.5, Angle=50, KBG=59, FKB=0, BKB=79, Size=6.5, X=0.0, Y=5.5, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=1, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_HEAD)
		}
		frame(Frame=10)
		if(is_excute){
			ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=6.0, Angle=60, KBG=69, FKB=0, BKB=74, Size=5.0, X=0.0, Y=5.5, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=1, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_KAMEHIT, Type=ATTACK_REGION_HEAD)
		}
		frame(Frame=30)
		if(is_excute){
			sv_module_access::damage(MSC=MA_MSC_DAMAGE_DAMAGE_NO_REACTION, Type=DAMAGE_NO_REACTION_MODE_NORMAL, DamageThreshold=0)
			AttackModule::clear_all()
			ArticleModule::remove_exist(FIGHTER_MURABITO_GENERATE_ARTICLE_FLOWERPOT,smash::app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL))
		}
    });
}
#[acmd_script(
    agent = "murabito",
    script =  "effect_attackdash",
    category = ACMD_EFFECT,
	low_priority)]
unsafe fn toad_da_eff(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		frame(Frame=6)
		if(is_excute){
			FOOT_EFFECT(hash40("sys_run_smoke"), hash40("top"), 2, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false)
		}
		frame(Frame=11)
		if(is_excute){
			FOOT_EFFECT(hash40("sys_run_smoke"), hash40("top"), 2, 0, 0, 0, 0, 0, 0.75, 0, 0, 0, 0, 0, 0, false)
		}
		frame(Frame=15)
		if(is_excute){
			FOOT_EFFECT(hash40("sys_run_smoke"), hash40("top"), 2, 0, 0, 0, 0, 0, 0.75, 0, 0, 0, 0, 0, 0, false)
		}
		frame(Frame=19)
		if(is_excute){
			FOOT_EFFECT(hash40("sys_run_smoke"), hash40("top"), 2, 0, 0, 0, 0, 0, 0.75, 0, 0, 0, 0, 0, 0, false)
		}
		frame(Frame=23)
		if(is_excute){
			FOOT_EFFECT(hash40("sys_run_smoke"), hash40("top"), 2, 0, 0, 0, 0, 0, 0.75, 0, 0, 0, 0, 0, 0, false)
		}
		frame(Frame=30)
		if(is_excute){
			LANDING_EFFECT(hash40("sys_down_smoke"), hash40("top"), 3, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false)
			EFFECT(hash40("sys_erace_smoke"), hash40("top"), 0, 5.5, 0, 0, 0, 0, 1.0, 0, 0, 0, 0, 0, 0, false)
		}
    });
}
#[acmd_script(
    agent = "murabito",
    script =  "sound_attackdash",
    category = ACMD_SOUND,
	low_priority)]
unsafe fn toad_da_snd(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		frame(Frame=5)
		for(5 Iterations){
			if(is_excute){
				PLAY_SE(hash40("se_common_step_sand"))
			}
			wait(Frames=1)
			if(is_excute){
				PLAY_SE(hash40("se_common_step_snow"))
				PLAY_SE(hash40("se_common_step_sand"))
			}
			wait(Frames=1)
			if(is_excute){
				PLAY_SE(hash40("se_common_step_snow"))
				PLAY_SE(hash40("se_common_step_sand"))
				PLAY_SE(hash40("se_common_step_sand"))
			}
			wait(Frames=1)
			if(is_excute){
				PLAY_SE(hash40("se_common_step_sand"))
				PLAY_SE(hash40("se_common_step_sand"))
			}
			wait(Frames=1)
			if(is_excute){
				PLAY_SE(hash40("se_common_step_snow"))
				PLAY_SE(hash40("se_common_step_snow"))
			}
			wait(Frames=1)
		}
    });
}
#[acmd_script(
    agent = "murabito",
    script =  "game_attackhi3",
    category = ACMD_GAME,
	low_priority)]
unsafe fn toad_utilt(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		frame(Frame=6)
		if(is_excute){
			ATTACK(ID=0, Part=0, Bone=hash40("head"), Damage=10.0, Angle=90, KBG=100, FKB=0, BKB=50, Size=6.5, X=4.4, Y=-1.0, Z=0.0, X=LUA_VOID, Y=LUA_VOID, Z=LUA_VOID, Hitlag=1.0, SDI=0.8, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_PUNCH)
		}
		frame(Frame=13)
		if(is_excute){
			AttackModule::clear_all()
		}
    });
}	
#[acmd_script(
    agent = "murabito",
    script =  "sound_attackhi3",
    category = ACMD_SOUND,
	low_priority)]
unsafe fn toad_utilt_snd(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		frame(Frame=5)
		if(is_excute){
			PLAY_SE(hash40("se_murabito_swing_s"))
		}
    });
}
#[acmd_script(
    agent = "murabito",
    script =  "effect_attackhi3",
    category = ACMD_EFFECT,
	low_priority)]
unsafe fn toad_utilt_eff(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		frame(Frame=6)
		if(is_excute){
			EFFECT_FOLLOW(hash40("sys_jump_smoke"), hash40("top"), 0, 0, 0, 0, 0, 0, 1, false)
		}
		frame(Frame=24)
		if(is_excute){
			LANDING_EFFECT(hash40("sys_down_smoke"), hash40("top"), 3, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false)
		}
    });
}	
#[acmd_script(
    agent = "murabito",
    script =  "game_attackhi4",
    category = ACMD_GAME,
	low_priority)]
unsafe fn toad_usmash(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		frame(Frame=4)
		if(is_excute){
			WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD)
		}
		frame(Frame=5)
		if(is_excute){
			ArticleModule::generate_article(FIGHTER_MURABITO_GENERATE_ARTICLE_SLINGSHOT, true, 0)
		}
		frame(Frame=6)
		if(is_excute){
			ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=1.0, Angle=100, KBG=100, FKB=125, BKB=0, Size=3.5, X=0.0, Y=6.5, Z=8.0, X=LUA_VOID, Y=LUA_VOID, Z=LUA_VOID, Hitlag=1.0, SDI=0.8, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_PUNCH)
		}
		frame(Frame=7)
		if(is_excute){
			AttackModule::clear_all()
		}
		frame(Frame=11)
		if(is_excute){
			ATTACK(ID=0, Part=0, Bone=hash40("havel"), Damage=17.0, Angle=82, KBG=78, FKB=0, BKB=58, Size=10.0, X=0.0, Y=0.0, Z=0.0, X=LUA_VOID, Y=LUA_VOID, Z=LUA_VOID, Hitlag=1.0, SDI=0.8, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_PUNCH)
		}
		frame(Frame=13)
		if(is_excute){
			AttackModule::clear_all()
		}
		frame(Frame=14)
		if(is_excute){
			ArticleModule::remove_exist(FIGHTER_MURABITO_GENERATE_ARTICLE_SLINGSHOT,smash::app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL))
		}
    });
}	
#[acmd_script(
    agent = "murabito",
    script =  "effect_attackhi4",
    category = ACMD_EFFECT,
	low_priority)]
unsafe fn toad_usmash_eff(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		frame(Frame=13)
		if(is_excute){
			EFFECT(0x12de01e4e5u64, hash40("havel"), 0, 0, 0, 0, 0, 0, 0.75, 0, 0, 0, 0, 0, 0, false)
			EFFECT(0x12de01e4e5u64, hash40("havel"), 0, 0, 0, 270, 0, 0, 0.75, 0, 0, 0, 0, 0, 0, false)
		}
		frame(Frame=48)
		if(is_excute){
			LANDING_EFFECT(hash40("sys_down_smoke"), hash40("top"), 3, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false)
		}
    });
}	
#[acmd_script(
    agent = "murabito",
    script =  "sound_attackhi4",
    category = ACMD_SOUND,
	low_priority)]
unsafe fn toad_usmash_snd(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		frame(Frame=6)
		if(is_excute){
			PLAY_SE(hash40("se_common_step_jump"))
		}
		frame(Frame=13)
		if(is_excute){
			PLAY_SE(hash40("se_common_smashswing_01"))
			PLAY_SE(hash40("se_common_pitin_move_00"))
			PLAY_SE(hash40("se_common_landing_brick"))
		}
		frame(Frame=47)
		if(is_excute){
			PLAY_SE(hash40("se_common_landing_soil"))
		}
    });
}	
#[acmd_script(
    agent = "murabito_slingshot",
    scripts =  ["game_attackairf", "game_attackairb"],
    category = ACMD_GAME,
	low_priority)]
unsafe fn toad_usmash_coin(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		frame(Frame=2)
		if(is_excute){
			ArticleModule::generate_article(WEAPON_MURABITO_SLINGSHOT_GENERATE_ARTICLE_BULLET, true, 0)
			ArticleModule::shoot(WEAPON_MURABITO_SLINGSHOT_GENERATE_ARTICLE_BULLET, smash::app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_LAST), false)
		}
    });
}	
#[acmd_script(
    agent = "murabito_slingshot",
    scripts =  ["sound_attackairf", "sound_attackairb"],
    category = ACMD_SOUND,
	low_priority)]
unsafe fn toad_usmash_coin_snd(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
}	
#[acmd_script(
    agent = "murabito_slingshot",
    scripts =  ["effect_attackairf", "effect_attackairb"],
    category = ACMD_EFFECT,
	low_priority)]
unsafe fn toad_usmash_coin_eff(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
}	
#[acmd_script(
    agent = "murabito_bullet",
    scripts =  ["game_shootf", "game_shootb"],
    category = ACMD_GAME,
	low_priority)]
unsafe fn toad_usmash_coin_bullet(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		frame(Frame=5)
		if(is_excute){
			ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=5.0, Angle=90, KBG=100, FKB=0, BKB=55, Size=5.0, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.4, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=-3.5, Trip=0.0, Rehit=0, Reflectable=true, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_COIN, Type=ATTACK_REGION_OBJECT)
			AttackModule::enable_safe_pos()
		}
		wait(Frames=3)
		if(is_excute){
			ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=3.0, Angle=90, KBG=100, FKB=0, BKB=10, Size=1.2, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.4, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=-2, Trip=0.0, Rehit=0, Reflectable=true, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_COIN, Type=ATTACK_REGION_OBJECT)
		}
		wait(Frames=6)
		if(is_excute){
			ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=1.5, Angle=90, KBG=100, FKB=0, BKB=10, Size=1.2, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.4, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=-1, Trip=0.0, Rehit=0, Reflectable=true, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_COIN, Type=ATTACK_REGION_OBJECT)
		}
    });
}	
#[acmd_script(
    agent = "murabito_bullet",
    scripts =  ["sound_shootf", "sound_shootb"],
    category = ACMD_SOUND,
	low_priority)]
unsafe fn toad_usmash_coin_bullet_snd(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
}	
#[acmd_script(
    agent = "murabito_bullet",
    scripts =  ["effect_shootf", "effect_shootb"],
    category = ACMD_EFFECT,
	low_priority)]
unsafe fn toad_usmash_coin_bullet_eff(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
}	
#[fighter_frame_callback]
pub fn toad(fighter : &mut L2CFighterCommon) {
    unsafe {
        let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent); 
		let status_kind = smash::app::lua_bind::StatusModule::status_kind(boma);
		let ENTRY_ID = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
		let motion_kind = MotionModule::motion_kind(boma);
		let frame = MotionModule::frame(boma);
		let fighter_kind = smash::app::utility::get_kind(boma);
		if fighter_kind == *FIGHTER_KIND_MURABITO {
			//if [*FIGHTER_STATUS_KIND_ATTACK, *FIGHTER_STATUS_KIND_ATTACK_100, *FIGHTER_STATUS_KIND_ATTACK_AIR].contains(&status_kind) == false {
				ModelModule::set_mesh_visibility(fighter.module_accessor,Hash40::new("murabito_glove_l"),false);
				ModelModule::set_mesh_visibility(fighter.module_accessor,Hash40::new("murabito_glove_r"),false);
				ModelModule::set_mesh_visibility(fighter.module_accessor,Hash40::new("murabito_turnip_1"),false);
				ModelModule::set_mesh_visibility(fighter.module_accessor,Hash40::new("murabito_turnip_2"),false);
				ModelModule::set_mesh_visibility(fighter.module_accessor,Hash40::new("murabito_turnip_3"),false);
				ModelModule::set_mesh_visibility(fighter.module_accessor,Hash40::new("murabito_turnip_1"),false);
				ModelModule::set_mesh_visibility(fighter.module_accessor,Hash40::new("murabito_turnipFLIP_2"),false);
				ModelModule::set_mesh_visibility(fighter.module_accessor,Hash40::new("murabito_turnipFLIP_3"),false);
				ModelModule::set_mesh_visibility(fighter.module_accessor,Hash40::new("murabito_handpa_l"),true);
				ModelModule::set_mesh_visibility(fighter.module_accessor,Hash40::new("murabito_handpa_r"),true);
			//};
		};
	};
}
	
pub fn install() {
    smashline::install_acmd_scripts!(
		//Jab
		toad_jab1,
		toad_jab1_eff,
		toad_jab1_snd,
		toad_jab2,
		toad_jab2_eff,
		toad_jab2_snd,
		toad_jab3,
		toad_jab3_eff,
		toad_jab3_snd,

		//Dash Attack
		toad_da,
		toad_da_eff,
		toad_da_snd,
		toad_shell_burst_eff,

		//Tilts
		toad_utilt,
		toad_utilt_eff,
		toad_utilt_snd,

		//Smashes
		toad_usmash,
		toad_usmash_eff,
		toad_usmash_snd,
		toad_usmash_coin,
		//toad_usmash_coin_eff,
		//toad_usmash_coin_snd,
		toad_usmash_coin_bullet,
		toad_usmash_coin_bullet_eff,
		toad_usmash_coin_bullet_snd,

		//Aerials
		toad_nair,
		toad_uair,
		toad_uair_eff,
		toad_uair_snd,
		toad_fair,
		toad_fair_eff,
		toad_fair_snd,
		toad_bair,
		toad_bair_snd,
		toad_bair_eff
    );
    smashline::install_agent_frame_callbacks!(toad);
}
