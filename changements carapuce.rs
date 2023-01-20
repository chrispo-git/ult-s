#[acmd_script(
	agent = "pzenigame",
	 scripts = ["game_attackhi4"],
	category = ACMD_GAME, 
	low_priority)]
unsafe fn squirtle_usmash(fighter: &mut L2CAgentBase){
let lua_state =fighter.lua_state_agent;
acmd!(lua_state,{
	
	frame(Frame=6)
	if(is_excute){
		WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD)
	}
	
	frame(Frame=12)
	
	if(is_excute){
	
	
	ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=16.0, Angle=95, KBG=92, FKB=0, BKB=65, Size=7.0, X=0.0, Y=3.0, Z=-7.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_THRU, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_water"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_WATER, Type=ATTACK_REGION_WATER)
	ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=16.0, Angle=95, KBG=92, FKB=0, BKB=65, Size=7.0, X=0.0, Y=3.0, Z=7.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_THRU, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_water"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_WATER, Type=ATTACK_REGION_WATER)
	
	
	ATTACK(ID=2, Part=0, Bone=hash40("top"), Damage=16.0, Angle=95, KBG=92, FKB=0, BKB=65, Size=5.5, X=0.0, Y=10.0, Z=-7.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_THRU, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_water"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_WATER, Type=ATTACK_REGION_WATER)
	ATTACK(ID=3, Part=0, Bone=hash40("top"), Damage=16.0, Angle=95, KBG=92, FKB=0, BKB=65, Size=5.5, X=0.0, Y=10.0, Z=7.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_THRU, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_water"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_WATER, Type=ATTACK_REGION_WATER)
	ATTACK(ID=4, Part=0, Bone=hash40("top"), Damage=16.0, Angle=95, KBG=92, FKB=0, BKB=65, Size=5.5, X=0.0, Y=18.0, Z=-7.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_THRU, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_water"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_WATER, Type=ATTACK_REGION_WATER)
	ATTACK(ID=5, Part=0, Bone=hash40("top"), Damage=16.0, Angle=95, KBG=92, FKB=0, BKB=65, Size=5.5, X=0.0, Y=18.0, Z=7.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_THRU, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_water"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_WATER, Type=ATTACK_REGION_WATER)
	ATTACK(ID=6, Part=0, Bone=hash40("top"), Damage=16.0, Angle=95, KBG=92, FKB=0, BKB=65, Size=5.0, X=0.0, Y=24.0, Z=-7.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_THRU, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_water"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_WATER, Type=ATTACK_REGION_WATER)
	ATTACK(ID=7, Part=0, Bone=hash40("top"), Damage=16.0, Angle=95, KBG=92, FKB=0, BKB=65, Size=5.0, X=0.0, Y=24.0, Z=7.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_THRU, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_water"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_WATER, Type=ATTACK_REGION_WATER)
	ATTACK(ID=8, Part=0, Bone=hash40("top"), Damage=12.0, Angle=95, KBG=92, FKB=0, BKB=60, Size=5.0, X=0.0, Y=31.0, Z=-7.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_THRU, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_water"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_WATER, Type=ATTACK_REGION_WATER)
	ATTACK(ID=9, Part=0, Bone=hash40("top"), Damage=12.0, Angle=95, KBG=92, FKB=0, BKB=60, Size=5.0, X=0.0, Y=31.0, Z=7.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_THRU, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_water"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_WATER, Type=ATTACK_REGION_WATER)
	
	}
	wait(Frames=1)
	if(is_excute){
	AttackModule::clear_all()
	}
}
)
}
#[acmd_script(agent = "pzenigame", script = "effect_attackhi4", category = ACMD_EFFECT, low_priority)]
  unsafe fn squirtle_usmash_eff(fighter: &mut L2CAgentBase){
	let lua_state =fighter.lua_state_agent;
  acmd!(lua_state,{
	
frame(Frame=8)
	if(is_excute) {
	EFFECT_FOLLOW(hash40("sys_smash_flash"), hash40("top"), 0, 10, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
	frame(Frame=9)}
	if (is_excute) {
	EFFECT_FOLLOW(hash40("pzenigame_water_ripple"), hash40("top"), 0, 0, 8, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
	LAST_EFFECT_SET_RATE(1.6)
	EFFECT(hash40("pzenigame_water_ripple"), hash40("top"), 0, 0, -8, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
	LAST_EFFECT_SET_RATE(1.6)
	}
	if (is_excute) {
	LANDING_EFFECT(hash40("null"), hash40("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
	}
	if (is_excute) {
	EFFECT_FOLLOW(hash40("pzenigame_atk_hi4"), hash40("top"), 0, 0, 8, 0, 0, 0, 1, true);
	LAST_EFFECT_SET_RATE(1.1)
	EFFECT_FOLLOW(hash40("pzenigame_atk_hi4"), hash40("top"), 0, 0, -8, 0, 0, 0, 1, true);
	LAST_EFFECT_SET_RATE(1.1)
	}

})}
#[acmd_script
(agent = "pzenigame",
script = "game_attackhi3",
category = ACMD_GAME,
low_priority)]
unsafe fn squirtle_utilt(fighter: &mut L2CAgentBase){
let lua_state =fighter.lua_state_agent;
acmd!(lua_state,{
	frame(Frame=6)
	if(is_excute){
		ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=6.0, Angle=95, KBG=30, FKB=0, BKB=75, Size=2.5, X=0.0, Y=3.0, Z=9.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_THRU, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_water"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_WATER, Type=ATTACK_REGION_WATER)
		ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=6.0, Angle=95, KBG=30, FKB=0, BKB=75, Size=2.5, X=0.0, Y=10.0, Z=9.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_THRU, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_water"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_WATER, Type=ATTACK_REGION_WATER)
		ATTACK(ID=2, Part=0, Bone=hash40("top"), Damage=6.0, Angle=95, KBG=30, FKB=0, BKB=75, Size=3.0, X=0.0, Y=18.0, Z=9.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_THRU, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_water"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_WATER, Type=ATTACK_REGION_WATER)

	}
	wait(Frames=3)
	if(is_excute){
	AttackModule::clear_all()
	}



})
}
#[acmd_script(agent = "pzenigame", script = "effect_attackhi3", category = ACMD_EFFECT, low_priority)]
  unsafe fn squirtle_utilt_eff(fighter: &mut L2CAgentBase){
	let lua_state =fighter.lua_state_agent;
  acmd!(lua_state,{
	frame(Frame=4)
	if(is_excute) {
	EFFECT_FOLLOW(hash40("pzenigame_water_ripple"), hash40("top"), 0, 0, 10, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
	LAST_EFFECT_SET_RATE(1.6)
	
  }frame(Frame=5)
     if(is_excute){
		EFFECT_FOLLOW(hash40("pzenigame_atk_hi4"), hash40("top"), 0, 0, 10, 0, 0, 0, 1, true);
	LAST_EFFECT_SET_RATE(1.1)
	 } }
)
}
