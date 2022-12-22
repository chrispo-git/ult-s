use smash::hash40;
use smash::app::lua_bind::*;
use smash::lib::lua_const::*;
use smash::lua2cpp::*;
use smash::lua2cpp::*;
use smashline::*;
use smash_script::*;


static mut CHARGE_FRAMES : [i32; 8] = [0; 8];
static MAX_FRAMES : i32 = 60;

#[acmd_script(
    agent = "miigunner_rapidshot_bullet",
    script =  "game_fly",
    category = ACMD_GAME,
	low_priority)]
unsafe fn gun_laser(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		if(is_excute){
			ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=5.0, Angle=90, KBG=40, FKB=0, BKB=50, Size=1.4, X=0.0, Y=0.0, Z=0.8, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=true, Absorbable=false, Flinchless=false, DisableHitlag=true, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_MIIGUNNER_BLASTER, Type=ATTACK_REGION_ENERGY)
			AttackModule::enable_safe_pos()
		}
		wait(Frames=1)
		if(is_excute){
			ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=5.0, Angle=90, KBG=40, FKB=0, BKB=50, Size=1.4, X=0.0, Y=0.0, Z=0.8, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=true, Absorbable=false, Flinchless=false, DisableHitlag=true, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_MIIGUNNER_BLASTER, Type=ATTACK_REGION_ENERGY)
		}
		wait(Frames=6)
    });
}	
#[acmd_script(
    agent = "miigunner",
    scripts =  ["game_specialn3start", "game_specialairn3start"],
    category = ACMD_GAME,
	low_priority)]
unsafe fn gunner_nade(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		frame(Frame=2)
		FT_MOTION_RATE(FSM=0.5)
		wait(Frames=20)
		FT_MOTION_RATE(FSM=1)
		frame(Frame=28)
		FT_MOTION_RATE(FSM=2)
    });
}		
#[acmd_script(
    agent = "miigunner",
    scripts =  ["game_specialn3end", "game_specialairn3end"],
    category = ACMD_GAME,
	low_priority)]
unsafe fn gunner_nade_end(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		frame(Frame=1)
		FT_MOTION_RATE(FSM=1.435)
		if(is_excute){
			ArticleModule::generate_article_enable(FIGHTER_MIIGUNNER_GENERATE_ARTICLE_GRENADELAUNCHER, false, 0)
		}
    });
}	
#[acmd_script(
    agent = "miigunner",
    scripts =  ["game_speciallw1start", "game_specialairlw1start"],
    category = ACMD_GAME,
	low_priority)]
unsafe fn gunner_shine(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		frame(Frame=2)
		if(is_excute){
			ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=2.0, Angle=10, KBG=30, FKB=0, BKB=60, Size=11.0, X=0.0, Y=6.5, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_G, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_elec"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_ELEC, Type=ATTACK_REGION_ENERGY)
			ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=2.0, Angle=24, KBG=30, FKB=0, BKB=60, Size=11.0, X=0.0, Y=6.5, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_A, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_elec"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_ELEC, Type=ATTACK_REGION_ENERGY)
		}
    });
}		
#[acmd_script(
    agent = "miigunner",
    scripts =  ["game_speciallw3hold", "game_specialairlw3hold"],
    category = ACMD_GAME,
	low_priority)]
unsafe fn gunner_mag(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		if(is_excute){
			ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=4.0, Angle=361, KBG=100, FKB=40, BKB=0, Size=12.5, X=0.0, Y=6.5, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=10, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_MAGIC, Type=ATTACK_REGION_ENERGY)
			AttackModule::set_add_reaction_frame(ID=0, Frames=6.0, Unk=false)
		}
		wait(Frames=6)
		if(is_excute){
			AttackModule::clear_all()
		}
    });
}		
#[acmd_script(
    agent = "miigunner",
    scripts =  ["game_specials1", "game_specialairs1"],
    category = ACMD_GAME,
	low_priority)]
unsafe fn gunner_fp(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		frame(Frame=1)
		FT_MOTION_RATE(FSM=0.7142857142857143)
		frame(Frame=21)
		FT_MOTION_RATE(FSM=1)
		if(is_excute){
			ArticleModule::generate_article(FIGHTER_MIIGUNNER_GENERATE_ARTICLE_FLAMEPILLAR, false, 0)
		}
    });
}	
#[acmd_script(
    agent = "miigunner",
    script =  "game_attackairlw",
    category = ACMD_GAME,
	low_priority)]
unsafe fn gunner_dair(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		FT_MOTION_RATE(FSM=0.9)
		frame(Frame=4)
		if(is_excute){
			WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
		}
		frame(Frame=20)
		FT_MOTION_RATE(FSM=1)
		if(is_excute){
			SET_SPEED_EX(1.2, 1.25, KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN)
			ATTACK(ID=0, Part=0, Bone=hash40("handr"), Damage=10.0, Angle=270, KBG=90, FKB=0, BKB=30, Size=4.0, X=2.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_PUNCH)
			ATTACK(ID=1, Part=0, Bone=hash40("handr"), Damage=15.0, Angle=270, KBG=90, FKB=0, BKB=2, Size=5.5, X=8.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_A, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_fire"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_FIRE, Type=ATTACK_REGION_PUNCH)
			ATTACK(ID=2, Part=0, Bone=hash40("handr"), Damage=15.0, Angle=270, KBG=90, FKB=0, BKB=30, Size=5.5, X=8.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_G, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_fire"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_FIRE, Type=ATTACK_REGION_PUNCH)
		}
		wait(Frames=2)
		if(is_excute){
			ATTACK(ID=0, Part=0, Bone=hash40("handr"), Damage=8.0, Angle=270, KBG=90, FKB=0, BKB=30, Size=2.5, X=4.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_PUNCH)
			ATTACK(ID=1, Part=0, Bone=hash40("handr"), Damage=12.0, Angle=270, KBG=90, FKB=0, BKB=30, Size=4.0, X=8.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_A, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_fire"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_FIRE, Type=ATTACK_REGION_PUNCH)
			ATTACK(ID=2, Part=0, Bone=hash40("handr"), Damage=12.0, Angle=270, KBG=90, FKB=0, BKB=30, Size=4.0, X=8.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_G, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_fire"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_FIRE, Type=ATTACK_REGION_PUNCH)
		}
		wait(Frames=4)
		if(is_excute){
			AttackModule::clear_all()
		}
		frame(Frame=54)
		if(is_excute){
			WorkModule::off_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
		}
    });
}	
#[acmd_script(
    agent = "miigunner",
    script =  "game_attacklw3",
    category = ACMD_GAME,
	low_priority)]
unsafe fn gunner_dtilt(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		frame(Frame=8)
		if(is_excute){
			ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=14.0, Angle=80, KBG=88, FKB=0, BKB=40, Size=5.5, X=0.0, Y=1.6, Z=17.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.2, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_fire"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_FIRE, Type=ATTACK_REGION_BOMB)
			ATTACK(ID=1, Part=0, Bone=hash40("armr"), Damage=14.0, Angle=80, KBG=88, FKB=0, BKB=40, Size=3.0, X=5.0, Y=0.0, Z=0.0, X2=2.0, Y2=0.0, Z2=0.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.2, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_fire"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_FIRE, Type=ATTACK_REGION_BOMB)
		}
		wait(Frames=3)
		if(is_excute){
			AttackModule::clear_all()
		}
    });
}	
#[acmd_script(
    agent = "miigunner",
    script =  "game_attackairhi",
    category = ACMD_GAME,
	low_priority)]
unsafe fn gunner_uair(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		frame(Frame=11)
		if(is_excute){
			WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
		}
		frame(Frame=17)
		if(is_excute){
			ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=1.8, Angle=140, KBG=100, FKB=45, BKB=0, Size=4.5, X=0.0, Y=15.0, Z=-3.0, X2=0.0, Y2=15.0, Z2=3.0, Hitlag=1.0, SDI=0.8, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=1, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_rush"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_ENERGY)
			AttackModule::set_add_reaction_frame(ID=0, Frames=6.0, Unk=false)
		}
		frame(Frame=18)
		if(is_excute){
			ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=1.8, Angle=367, KBG=100, FKB=25, BKB=0, Size=2.6, X=0.0, Y=50.0, Z=0.0, X2=0.0, Y2=15.0, Z2=0.0, Hitlag=1.0, SDI=0.8, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=3, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_A, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_rush"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_ENERGY)
			ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=1.8, Angle=96, KBG=100, FKB=25, BKB=0, Size=2.6, X=0.0, Y=50.0, Z=0.0, X2=0.0, Y2=15.0, Z2=0.0, Hitlag=1.0, SDI=0.8, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=3, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_G, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_rush"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_ENERGY)
			ATTACK(ID=2, Part=0, Bone=hash40("top"), Damage=1.8, Angle=130, KBG=100, FKB=25, BKB=0, Size=3.2, X=0.0, Y=13.0, Z=-0.8, X2=0.0, Y2=13.0, Z2=0.8, Hitlag=1.0, SDI=0.8, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=3, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_A, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_rush"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_ENERGY)
			AttackModule::set_add_reaction_frame(ID=0, Frames=6.0, Unk=false)
			AttackModule::set_add_reaction_frame(ID=1, Frames=6.0, Unk=false)
			AttackModule::set_add_reaction_frame(ID=2, Frames=6.0, Unk=false)
		}
		FT_MOTION_RATE(FSM=0.5)
		frame(Frame=22)
		FT_MOTION_RATE(FSM=1)
		frame(Frame=35)
		if(is_excute){
			AttackModule::clear_all()
		}
		frame(Frame=36)
		if(is_excute){
			ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=4.0, Angle=72, KBG=175, FKB=0, BKB=50, Size=4.0, X=0.0, Y=50.0, Z=0.0, X2=0.0, Y2=12.0, Z2=0.0, Hitlag=1.0, SDI=0.8, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_ENERGY)
		}
		frame(Frame=37)
		if(is_excute){
			AttackModule::clear_all()
		}
		frame(Frame=60)
		if(is_excute){
			WorkModule::off_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
		}
    });
}		
#[fighter_frame_callback]
pub fn gunner(fighter : &mut L2CFighterCommon) {
    unsafe {
        let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent); 
		let status_kind = smash::app::lua_bind::StatusModule::status_kind(boma);
		let ENTRY_ID = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
		let fighter_kind = smash::app::utility::get_kind(boma);
		if fighter_kind == *FIGHTER_KIND_MIIGUNNER {
			if [hash40("special_lw1_start"), hash40("special_lw1_start_l"), hash40("special_air_lw1_start"), hash40("special_air_lw1_start_l")].contains(&MotionModule::motion_kind(boma)) {
				if MotionModule::frame(boma) == 3.0 {
					AttackModule::set_size(boma, 0, 14.0);
				};
			};
			if [hash40("special_n3_start"), hash40("special_air_n3_start")].contains(&MotionModule::motion_kind(boma)) {
				if MotionModule::frame(boma) > 28.0 && MotionModule::frame(boma) < 30.0{
					if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_SPECIAL) && CHARGE_FRAMES[ENTRY_ID] < MAX_FRAMES{
						CHARGE_FRAMES[ENTRY_ID] += 1;
						MotionModule::set_rate(boma, 0.01);
					} else {
						MotionModule::set_rate(boma, 0.5);
					};
				};
			} else {
				CHARGE_FRAMES[ENTRY_ID] = 0;
			};
		};
	};
}
	
pub fn install() {
    smashline::install_acmd_scripts!(
		gun_laser,
		gunner_shine,
		gunner_mag,
		gunner_fp,
		gunner_nade,
		gunner_nade_end,
		gunner_dair,
		gunner_uair,
		gunner_dtilt
    );
	smashline::install_agent_frame_callbacks!(gunner);
}
