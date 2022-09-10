use smash::hash40;
use smash::lib::lua_const::*;
use smash::lua2cpp::*;
use smash::app::lua_bind::*;
use smash_script::*;
use smashline::*;
#[acmd_script(
    agent = "bayonetta",
    script =  "game_attackairf",
    category = ACMD_GAME,
	low_priority)]
unsafe fn bayo_fair1(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		frame(Frame=1)
		if(is_excute){
			rust{notify_event_msc_cmd!(fighter, 0x2d51fcdb09u64, *FIGHTER_BAYONETTA_SHOOTING_SLOT_L_ARM, true, false, false, 10, 3, 10, 0, true);}
		}
		frame(Frame=3)
		if(is_excute){
			WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
		}
		frame(Frame=7)
		if(is_excute){
			ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=4.0, Angle=50, KBG=10, FKB=0, BKB=65, Size=2.8, X=0.0, Y=9.2, Z=3.2, X2=0.0, Y2=9.2, Z2=7.5, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_PUNCH)
			ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=4.0, Angle=28, KBG=10, FKB=0, BKB=55, Size=2.8, X=0.0, Y=14.8, Z=3.2, X2=0.0, Y2=14.8, Z2=7.5, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_PUNCH)
			ATTACK(ID=2, Part=0, Bone=hash40("top"), Damage=4.0, Angle=70, KBG=10, FKB=0, BKB=55, Size=5.8, X=0.0, Y=12.0, Z=5.0, X2=0.0, Y2=12.0, Z2=11.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_G, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_PUNCH)
			ATTACK(ID=3, Part=0, Bone=hash40("top"), Damage=4.0, Angle=45, KBG=10, FKB=0, BKB=45, Size=5.8, X=0.0, Y=12.0, Z=5.0, X2=0.0, Y2=12.0, Z2=11.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_A, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_PUNCH)
			AttackModule::set_add_reaction_frame(ID=0, Frames=2.0, Unk=false)
			AttackModule::set_add_reaction_frame(ID=1, Frames=2.0, Unk=false)
			AttackModule::set_add_reaction_frame(ID=2, Frames=2.0, Unk=false)
			AttackModule::set_add_reaction_frame(ID=3, Frames=2.0, Unk=false)
		}
		wait(Frames=3)
		if(is_excute){
			AttackModule::clear_all()
		}
		frame(Frame=11)
		if(is_excute){
			WorkModule::on_flag(Flag=FIGHTER_BAYONETTA_INSTANCE_WORK_ID_FLAG_SHOOTING_ACTION)
			WorkModule::on_flag(Flag=FIGHTER_BAYONETTA_INSTANCE_WORK_ID_FLAG_SHOOTING_MOTION_STOP)
			WorkModule::on_flag(Flag=FIGHTER_BAYONETTA_INSTANCE_WORK_ID_FLAG_SHOOTING_CHECK_END)
		}
		wait(Frames=1)
		if(is_excute){
			WorkModule::on_flag(Flag=FIGHTER_BAYONETTA_STATUS_ATTACK_AIR_F_FLAG_ENABLE_COMBO)
		}
		frame(Frame=30)
		if(is_excute){
			WorkModule::off_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
		}
    });
}
#[acmd_script(
    agent = "bayonetta",
    script =  "game_attackairf2",
    category = ACMD_GAME,
	low_priority)]
unsafe fn bayo_fair2(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		frame(Frame=1)
		if(is_excute){
			rust{notify_event_msc_cmd!(fighter, 0x2d51fcdb09u64, *FIGHTER_BAYONETTA_SHOOTING_SLOT_L_ARM, true, false, false, 10, 3, 10, 0, true);}
		}
		frame(Frame=3)
		if(is_excute){
			WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
		}
		frame(Frame=7)
		if(is_excute){
			ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=3.3, Angle=56, KBG=10, FKB=0, BKB=55, Size=3.7, X=0.0, Y=8.5, Z=6.0, X2=0.0, Y2=8.5, Z2=11.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_PUNCH)
			ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=3.3, Angle=44, KBG=10, FKB=0, BKB=50, Size=3.7, X=0.0, Y=15.5, Z=6.0, X2=0.0, Y2=15.5, Z2=11.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_PUNCH)
			ATTACK(ID=2, Part=0, Bone=hash40("top"), Damage=3.3, Angle=82, KBG=10, FKB=0, BKB=50, Size=7.4, X=0.0, Y=12.0, Z=8.0, X2=0.0, Y2=12.0, Z2=15.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_G, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_PUNCH)
			ATTACK(ID=3, Part=0, Bone=hash40("top"), Damage=3.3, Angle=60, KBG=10, FKB=0, BKB=40, Size=7.4, X=0.0, Y=12.0, Z=8.0, X2=0.0, Y2=12.0, Z2=15.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_A, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_PUNCH)
			AttackModule::set_add_reaction_frame(ID=0, Frames=5.0, Unk=false)
			AttackModule::set_add_reaction_frame(ID=1, Frames=5.0, Unk=false)
			AttackModule::set_add_reaction_frame(ID=2, Frames=5.0, Unk=false)
			AttackModule::set_add_reaction_frame(ID=3, Frames=5.0, Unk=false)
		}
		wait(Frames=3)
		if(is_excute){
			AttackModule::clear_all()
		}
		frame(Frame=11)
		if(is_excute){
			WorkModule::on_flag(Flag=FIGHTER_BAYONETTA_INSTANCE_WORK_ID_FLAG_SHOOTING_ACTION)
			WorkModule::on_flag(Flag=FIGHTER_BAYONETTA_INSTANCE_WORK_ID_FLAG_SHOOTING_MOTION_STOP)
			WorkModule::on_flag(Flag=FIGHTER_BAYONETTA_INSTANCE_WORK_ID_FLAG_SHOOTING_CHECK_END)
		}
		wait(Frames=1)
		if(is_excute){
			WorkModule::on_flag(Flag=FIGHTER_BAYONETTA_STATUS_ATTACK_AIR_F_FLAG_ENABLE_COMBO)
		}
		frame(Frame=32)
		if(is_excute){
			WorkModule::off_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
		}
    });
}	
//AttackModule::set_no_damage_fly_smoke_all(true, false)
#[acmd_script(
    agent = "bayonetta",
    script =  "game_specials",
    category = ACMD_GAME,
	low_priority)]
unsafe fn bayo_sideb_start(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		frame(Frame=1)
		if(is_excute){
			rust{notify_event_msc_cmd!(fighter, 0x2d51fcdb09u64, *FIGHTER_BAYONETTA_SHOOTING_SLOT_R_LEG, false, false, true, 10, 0, 20, 0, false);}
		}
		frame(Frame=15)
		if(is_excute){
			ATTACK(ID=0, Part=0, Bone=hash40("footr"), Damage=8.0, Angle=45, KBG=8, FKB=0, BKB=100, Size=4.0, X=0.0, Y=0.0, Z=0.0, X2=-8.0, Y2=0.0, Z2=0.0, Hitlag=0.9, SDI=1.3, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_NO_FLOOR, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
			AttackModule::set_no_damage_fly_smoke_all(true, false)
		}
		frame(Frame=17)
		if(is_excute){
			ATTACK(ID=0, Part=0, Bone=hash40("footr"), Damage=8.0, Angle=47, KBG=8, FKB=0, BKB=95, Size=4.0, X=0.0, Y=0.0, Z=0.0, X2=-8.0, Y2=0.0, Z2=0.0, Hitlag=0.9, SDI=1.3, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_NO_FLOOR, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
			AttackModule::set_no_damage_fly_smoke_all(true, false)
		}
		frame(Frame=19)
		if(is_excute){
			WorkModule::on_flag(Flag=FIGHTER_BAYONETTA_STATUS_WORK_ID_SPECIAL_S_FLAG_WALL_CHECK)
			ATTACK(ID=0, Part=0, Bone=hash40("footr"), Damage=7.5, Angle=50, KBG=8, FKB=0, BKB=83, Size=4.0, X=0.0, Y=0.0, Z=0.0, X2=-8.0, Y2=0.0, Z2=0.0, Hitlag=0.9, SDI=1.3, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_NO_FLOOR, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
			AttackModule::set_no_damage_fly_smoke_all(true, false)
		}
		frame(Frame=25)
		if(is_excute){
			WorkModule::on_flag(Flag=FIGHTER_BAYONETTA_INSTANCE_WORK_ID_FLAG_SHOOTING_ACTION)
			ATTACK(ID=0, Part=0, Bone=hash40("footr"), Damage=7.0, Angle=62, KBG=8, FKB=0, BKB=68, Size=4.0, X=0.0, Y=0.0, Z=0.0, X2=-8.0, Y2=0.0, Z2=0.0, Hitlag=0.9, SDI=1.3, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_NO_FLOOR, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
			AttackModule::set_no_damage_fly_smoke_all(true, false)
		}
		frame(Frame=32)
		if(is_excute){
			WorkModule::on_flag(Flag=FIGHTER_BAYONETTA_STATUS_WORK_ID_SPECIAL_S_FLAG_HOLD_END_CHECK)
			WorkModule::on_flag(Flag=FIGHTER_BAYONETTA_INSTANCE_WORK_ID_FLAG_SHOOTING_CHECK_END)
		}
		frame(Frame=40)
		if(is_excute){
			AttackModule::clear_all()
			WorkModule::off_flag(Flag=FIGHTER_BAYONETTA_STATUS_WORK_ID_SPECIAL_S_FLAG_WALL_CHECK)
		}
		frame(Frame=40)
		FT_MOTION_RATE(FSM=0.843)
    });
}
#[acmd_script(
    agent = "bayonetta",
    script =  "game_specialsholdend",
    category = ACMD_GAME,
	low_priority)]
unsafe fn bayo_sideb(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		frame(Frame=1)
		FT_MOTION_RATE(FSM=0.5)
		if(is_excute){
			rust{notify_event_msc_cmd!(fighter, 0x2d51fcdb09u64, *FIGHTER_BAYONETTA_SHOOTING_SLOT_R_LEG, true, false, false, 10, 3, 10, 0, true);}
			ATTACK(ID=0, Part=0, Bone=hash40("footr"), Damage=8.0, Angle=80, KBG=8, FKB=0, BKB=60, Size=4.0, X=0.0, Y=0.0, Z=0.0, X2=-8.0, Y2=0.0, Z2=0.0, Hitlag=1.0, SDI=1.3, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_NO_FLOOR, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
			WorkModule::on_flag(Flag=FIGHTER_BAYONETTA_INSTANCE_WORK_ID_FLAG_SHOOTING_ACTION)
		}
		frame(Frame=23)
		if(is_excute){
			WorkModule::on_flag(Flag=FIGHTER_BAYONETTA_INSTANCE_WORK_ID_FLAG_SHOOTING_CHECK_END)
			AttackModule::clear_all()
			WorkModule::on_flag(Flag=FIGHTER_BAYONETTA_STATUS_WORK_ID_SPECIAL_S_FLAG_END_SPECIAL_S)
		}
		frame(Frame=34)
		frame(Frame=37)
		if(is_excute){
			FT_MOTION_RATE(FSM=0.75)
			ATTACK(ID=3, Part=0, Bone=hash40("kneer"), Damage=5.0, Angle=98, KBG=65, FKB=0, BKB=64, Size=5.3, X=13.5, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.3, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_NO_FLOOR, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
			ATTACK(ID=0, Part=0, Bone=hash40("kneer"), Damage=5.0, Angle=98, KBG=65, FKB=0, BKB=64, Size=5.3, X=7.5, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.3, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_NO_FLOOR, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
			ATTACK(ID=1, Part=0, Bone=hash40("kneer"), Damage=5.0, Angle=98, KBG=65, FKB=0, BKB=64, Size=4.0, X=1.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.3, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_NO_FLOOR, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
			ATTACK(ID=2, Part=0, Bone=hash40("waist"), Damage=5.0, Angle=90, KBG=65, FKB=0, BKB=64, Size=4.5, X=0.0, Y=-0.8, Z=-1.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.3, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_NO_FLOOR, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
		}
		frame(Frame=44)
		if(is_excute){
			AttackModule::clear_all()
		}
    });
}
#[acmd_script(
    agent = "bayonetta",
    script =  "game_attack11",
    category = ACMD_GAME,
	low_priority)]
unsafe fn bayo_jab1(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		frame(Frame=1)
		if(is_excute){
			rust{notify_event_msc_cmd!(fighter, 0x2d51fcdb09u64, *FIGHTER_BAYONETTA_SHOOTING_SLOT_L_ARM, true, false, false, 10, 3, 10, 5, true);}
		}
		frame(Frame=2)
		FT_MOTION_RATE(FSM=0.25)
		frame(Frame=6)
		FT_MOTION_RATE(FSM=1)
		frame(Frame=9)
		if(is_excute){
			ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=1.4, Angle=361, KBG=8, FKB=0, BKB=36, Size=2.8, X=0.0, Y=10.0, Z=7.2, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_PUNCH)
			ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=1.4, Angle=361, KBG=8, FKB=0, BKB=36, Size=2.6, X=0.0, Y=10.0, Z=3.5, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_PUNCH)
			ATTACK(ID=2, Part=0, Bone=hash40("top"), Damage=1.4, Angle=361, KBG=8, FKB=0, BKB=32, Size=2.2, X=0.0, Y=5.5, Z=3.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_G, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_PUNCH)
			ATTACK(ID=3, Part=0, Bone=hash40("top"), Damage=1.4, Angle=180, KBG=6, FKB=0, BKB=32, Size=2.8, X=0.0, Y=10.0, Z=11.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_FIGHTER, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_PUNCH)
			ATTACK(ID=4, Part=0, Bone=hash40("top"), Damage=1.4, Angle=361, KBG=6, FKB=0, BKB=32, Size=2.8, X=0.0, Y=10.0, Z=11.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_PUNCH)
			AttackModule::set_add_reaction_frame(ID=0, Frames=8.0, Unk=false)
			AttackModule::set_add_reaction_frame(ID=1, Frames=8.0, Unk=false)
			AttackModule::set_add_reaction_frame(ID=2, Frames=8.0, Unk=false)
			AttackModule::set_add_reaction_frame(ID=3, Frames=8.0, Unk=false)
			AttackModule::set_add_reaction_frame(ID=4, Frames=8.0, Unk=false)
		}
		frame(Frame=13)
		if(is_excute){
			AttackModule::clear_all()
			WorkModule::on_flag(Flag=FIGHTER_BAYONETTA_INSTANCE_WORK_ID_FLAG_SHOOTING_ACTION)
			WorkModule::on_flag(Flag=FIGHTER_BAYONETTA_INSTANCE_WORK_ID_FLAG_SHOOTING_MOTION_STOP)
			WorkModule::on_flag(Flag=FIGHTER_BAYONETTA_INSTANCE_WORK_ID_FLAG_SHOOTING_CHECK_END)
		}
		frame(Frame=14)
		if(is_excute){
			WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO)
		}
    });
}	
	
#[acmd_script(
    agent = "bayonetta",
    script =  "game_attacklw3",
    category = ACMD_GAME,
	low_priority)]
unsafe fn bayo_dtilt(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		frame(Frame=1)
		if(is_excute){
			rust{notify_event_msc_cmd!(fighter, 0x2d51fcdb09u64, *FIGHTER_BAYONETTA_SHOOTING_SLOT_L_LEG, true, false, false, 10, 3, 15, 5, true);}
		}
		FT_MOTION_RATE(FSM=0.5)
		frame(Frame=12)
		FT_MOTION_RATE(FSM=0.8)
		if(is_excute){
			ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=5.0, Angle=85, KBG=55, FKB=0, BKB=70, Size=2.8, X=0.0, Y=4.5, Z=1.0, X2=0.0, Y2=4.0, Z2=3.5, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.3, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
			ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=6.0, Angle=95, KBG=55, FKB=0, BKB=70, Size=2.8, X=0.0, Y=4.5, Z=1.0, X2=0.0, Y2=3.0, Z2=11.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.3, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
		}
		wait(Frames=2)
		if(is_excute){
		AttackModule::clear_all()
		}
    });
}
#[acmd_script(
    agent = "bayonetta",
    script =  "game_attackhi3",
    category = ACMD_GAME,
	low_priority)]
unsafe fn bayo_utilt(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		frame(Frame=1)
		if(is_excute){
			rust{notify_event_msc_cmd!(fighter, 0x2d51fcdb09u64, *FIGHTER_BAYONETTA_SHOOTING_SLOT_R_ARM, true, false, false, 10, 3, 10, 5, true);}
		}
		frame(Frame=7)
		FT_MOTION_RATE(FSM=1.5)
		if(is_excute){
			ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=1.5, Angle=110, KBG=100, FKB=85, BKB=0, Size=5.0, X=0.0, Y=7.0, Z=9.5, X2=0.0, Y2=10.0, Z2=9.5, Hitlag=0.8, SDI=0.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_G, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_PUNCH)
			ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=1.5, Angle=110, KBG=100, FKB=80, BKB=0, Size=5.0, X=0.0, Y=7.0, Z=9.5, X2=0.0, Y2=10.0, Z2=9.5, Hitlag=0.8, SDI=0.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_A, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_PUNCH)
		}
		frame(Frame=9)
		if(is_excute){
			AttackModule::clear(ID=1, false)
			ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=2.0, Angle=128, KBG=27, FKB=0, BKB=65, Size=4.5, X=0.0, Y=18.0, Z=7.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.8, SDI=0.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_A, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_PUNCH)
		}
		frame(Frame=11)
		if(is_excute){
			AttackModule::clear_all()
			ATTACK(ID=0, Part=1, Bone=hash40("top"), Damage=6.0, Angle=93, KBG=60, FKB=0, BKB=55, Size=5.5, X=0.0, Y=25.5, Z=-2.0, X2=0.0, Y2=25.5, Z2=2.0, Hitlag=1.5, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_PUNCH)
			ATTACK(ID=1, Part=1, Bone=hash40("top"), Damage=6.0, Angle=93, KBG=60, FKB=0, BKB=55, Size=2.0, X=0.0, Y=20.0, Z=-6.0, X2=0.0, Y2=20.0, Z2=6.0, Hitlag=1.5, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_PUNCH)
		}
		FT_MOTION_RATE(FSM=3)
		frame(Frame=12)
		frame(Frame=13)
		FT_MOTION_RATE(FSM=0.733)
		frame(Frame=28)
		FT_MOTION_RATE(FSM=1)
    });
}

pub fn install() {
    smashline::install_acmd_scripts!(
		bayo_fair1,
		bayo_fair2,
		bayo_sideb,
		bayo_dtilt,
		bayo_utilt,
		bayo_jab1,
		bayo_sideb_start
    );
}
