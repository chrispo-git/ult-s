use smash::hash40;
use smash::phx::Hash40;
use smash::lib::lua_const::*;
use smash::app::*;
use smash::app::lua_bind::*;
use smash::lua2cpp::{L2CFighterCommon, L2CAgentBase};
use smashline::*;
use smash_script::*;
use std::os::raw::c_int;

#[fighter_frame( agent = FIGHTER_KIND_DEMON )]
fn kaz_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent); 
		let status_kind = smash::app::lua_bind::StatusModule::status_kind(boma);
		let motion_kind = MotionModule::motion_kind(boma);
		let frame = MotionModule::frame(boma);
		let ENTRY_ID = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
		if [hash40("special_air_hi"), hash40("special_hi")].contains(&motion_kind) {
			if MotionModule::frame(boma) > 2.0 {
				if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_GUARD) {
					StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ESCAPE_AIR, true);
				};
			};
			if MotionModule::frame(boma) > 14.0 {
				if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_ATTACK) {
					StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_AIR, false);
				};
			};
		};
		if [hash40("special_air_hi_start"), hash40("special_hi_start")].contains(&motion_kind) {
			MotionModule::set_rate(boma, 1.5);
		};
		//Original Endlag is 27
		//Original Startup is 10
		if status_kind == *FIGHTER_DEMON_STATUS_KIND_ATTACK_STEP_2F || status_kind == *FIGHTER_DEMON_STATUS_KIND_ATTACK_STEP_2{
			MotionModule::set_rate(boma, 1.0);
			if status_kind == *FIGHTER_DEMON_STATUS_KIND_ATTACK_STEP_2 {
				HitModule::set_status_all(boma, smash::app::HitStatus(*HIT_STATUS_NORMAL), 0);
			};
		};
		if [hash40("attack_hi3")].contains(&motion_kind) && frame > 11.0 && AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT){
			if (ControlModule::get_command_flag_cat(boma, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_HI4) != 0 {
				StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_HI4_START, true);
			};
			if (ControlModule::get_command_flag_cat(boma, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_HI) != 0 {
				StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_SPECIAL_HI, true);
			};
		};
    }
}

#[skyline::hook(replace = smash::app::lua_bind::AttackModule::set_add_reaction_frame_revised)]
pub unsafe fn reaction_frame_hook(boma: &mut smash::app::BattleObjectModuleAccessor, int: c_int, float: f32, arg4: bool) -> () {
	let fighter_kind = smash::app::utility::get_kind(boma);
	let status_kind = smash::app::lua_bind::StatusModule::status_kind(boma);
	if fighter_kind == *FIGHTER_KIND_DEMON {
		if status_kind == *FIGHTER_DEMON_STATUS_KIND_ATTACK_STEP_2F {
			if float as i32 ==  19 {
				original!()(boma, int, 14.0, arg4)
			} else if float as i32 ==  14 {
				original!()(boma, int, 13.0, arg4)
			} else {
				original!()(boma, int, float, arg4)
			}
		} else if status_kind == *FIGHTER_DEMON_STATUS_KIND_ATTACK_STEP_2 {
			if float as i32 ==  17 {
				original!()(boma, int, 13.0, arg4)
			} else if float as i32 ==  15 {
				original!()(boma, int, 13.0, arg4)
			} else if float as i32 ==  13 {
				original!()(boma, int, 12.0, arg4)
			} else {
				original!()(boma, int, float, arg4)
			}
		}
	} else {
		original!()(boma, int, float, arg4)
	}
}
//AttackModule::set_add_reaction_frame_revised(ID=3, Frames=13.0, Unk=false)
// Uses the acmd! macro, if you're porting directly from Rubendal's data viewer.
#[acmd_script( 
agent = "demon", 
scripts = ["game_attackstand4"], 
category = ACMD_GAME)]
unsafe fn kaz_staturesmash(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		frame(Frame=12)
		if(is_excute){
			ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=9.0, Angle=0, KBG=2, FKB=0, BKB=35, Size=2.0, X=0.0, Y=8.0, Z=8.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.2, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=2, Trip=1.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_DEMON_KICK, Type=ATTACK_REGION_KICK)
			ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=9.0, Angle=0, KBG=2, FKB=0, BKB=35, Size=3.6, X=0.0, Y=7.0, Z=9.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.2, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=2, Trip=1.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_DEMON_KICK, Type=ATTACK_REGION_KICK)
			ATTACK(ID=2, Part=0, Bone=hash40("top"), Damage=9.0, Angle=0, KBG=2, FKB=0, BKB=35, Size=3.2, X=0.0, Y=9.5, Z=1.5, X2=0.0, Y2=7.0, Z2=9.0, Hitlag=0.2, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=2, Trip=1.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_DEMON_KICK, Type=ATTACK_REGION_KICK)
			ATK_SET_SHIELD_SETOFF_MUL(ID=0, ShieldstunMul=1.7)
			ATK_SET_SHIELD_SETOFF_MUL(ID=1, ShieldstunMul=1.7)
			ATK_SET_SHIELD_SETOFF_MUL(ID=2, ShieldstunMul=1.7)
		}
		frame(Frame=14)
		if(is_excute){
			ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=9.0, Angle=0, KBG=2, FKB=0, BKB=35, Size=2.0, X=0.0, Y=6.5, Z=5.5, X2=0.0, Y2=6.4, Z2=5.5, Hitlag=0.2, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=2, Trip=1.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_DEMON_KICK, Type=ATTACK_REGION_KICK)
			ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=9.0, Angle=0, KBG=2, FKB=0, BKB=35, Size=3.6, X=0.0, Y=4.5, Z=6.5, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.2, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=2, Trip=1.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_DEMON_KICK, Type=ATTACK_REGION_KICK)
			ATTACK(ID=2, Part=0, Bone=hash40("top"), Damage=9.0, Angle=0, KBG=2, FKB=0, BKB=35, Size=3.2, X=0.0, Y=9.5, Z=1.5, X2=0.0, Y2=4.5, Z2=6.5, Hitlag=0.2, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=2, Trip=1.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_DEMON_KICK, Type=ATTACK_REGION_KICK)
			ATK_SET_SHIELD_SETOFF_MUL(ID=0, ShieldstunMul=1.7)
			ATK_SET_SHIELD_SETOFF_MUL(ID=1, ShieldstunMul=1.7)
			ATK_SET_SHIELD_SETOFF_MUL(ID=2, ShieldstunMul=1.7)
		}
		frame(Frame=15)
		if(is_excute){
			AttackModule::clear_all()
		}
    });
}	
#[acmd_script( 
agent = "demon", 
scripts = ["game_attackstand5"], 
category = ACMD_GAME)]
unsafe fn kaz_flashtornado(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {	
		frame(Frame=10)
		if(is_excute){
			ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=15.0, Angle=46, KBG=69, FKB=0, BKB=55, Size=3.0, X=0.0, Y=15.0, Z=10.5, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.35, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=2, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_DEMON_KICK, Type=ATTACK_REGION_KICK)
			ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=15.0, Angle=46, KBG=69, FKB=0, BKB=55, Size=5.2, X=0.0, Y=10.5, Z=5.5, X2=0.0, Y2=13.72, Z2=10.5, Hitlag=0.35, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=2, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_DEMON_KICK, Type=ATTACK_REGION_KICK)
		}
		frame(Frame=13)
		if(is_excute){
			AttackModule::clear_all()
		}
    });
}	
#[acmd_script( 
agent = "demon", 
scripts = ["game_attackstep2f"], 
category = ACMD_GAME)]
unsafe fn kaz_ewgf(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		if(is_excute){
			HIT_NODE(hash40("head"), HIT_STATUS_XLU)
			HIT_NODE(hash40("bust"), HIT_STATUS_XLU)
			HIT_NODE(hash40("shoulderl"), HIT_STATUS_XLU)
			HIT_NODE(hash40("shoulderr"), HIT_STATUS_XLU)
			HIT_NODE(hash40("arml"), HIT_STATUS_XLU)
			HIT_NODE(hash40("armr"), HIT_STATUS_XLU)
		}
		frame(Frame=10)
		if(is_excute){
			HIT_NODE(hash40("head"), HIT_STATUS_NORMAL)
			HIT_NODE(hash40("bust"), HIT_STATUS_NORMAL)
			HIT_NODE(hash40("shoulderl"), HIT_STATUS_NORMAL)
			HIT_NODE(hash40("arml"), HIT_STATUS_NORMAL)
			HIT_NODE(hash40("shoulderr"), HIT_STATUS_XLU)
			HIT_NODE(hash40("armr"), HIT_STATUS_XLU)
			ATTACK(ID=0, Part=0, Bone=hash40("handr"), Damage=14.5, Angle=88, KBG=5, FKB=0, BKB=102, Size=2.5, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.3, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=5, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_G, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_paralyze"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_DEMON_PUNCH01, Type=ATTACK_REGION_PUNCH)
			ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=14.5, Angle=88, KBG=5, FKB=0, BKB=102, Size=5.0, X=0.0, Y=13.0, Z=6.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.3, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=5, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_G, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_paralyze"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_DEMON_PUNCH01, Type=ATTACK_REGION_PUNCH)
			ATTACK(ID=2, Part=0, Bone=hash40("top"), Damage=14.5, Angle=88, KBG=5, FKB=0, BKB=102, Size=3.0, X=-1.0, Y=9.0, Z=3.5, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.3, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=5, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_G, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_paralyze"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_DEMON_PUNCH01, Type=ATTACK_REGION_PUNCH)
			ATTACK(ID=3, Part=0, Bone=hash40("handr"), Damage=14.0, Angle=81, KBG=5, FKB=0, BKB=92, Size=2.5, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.3, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=5, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_A, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_paralyze"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_DEMON_PUNCH01, Type=ATTACK_REGION_PUNCH)
			ATTACK(ID=4, Part=0, Bone=hash40("top"), Damage=14.0, Angle=81, KBG=5, FKB=0, BKB=92, Size=5.0, X=0.0, Y=13.0, Z=6.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.3, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=5, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_A, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_paralyze"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_DEMON_PUNCH01, Type=ATTACK_REGION_PUNCH)
			ATTACK(ID=5, Part=0, Bone=hash40("top"), Damage=14.0, Angle=81, KBG=5, FKB=0, BKB=92, Size=3.0, X=-1.0, Y=9.0, Z=3.5, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.3, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=5, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_A, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_paralyze"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_DEMON_PUNCH01, Type=ATTACK_REGION_PUNCH)
			ATK_SET_SHIELD_SETOFF_MUL_arg3(ID1=0, ID2=1.2, ShieldstunMul=5)
			ATK_SET_SHIELD_SETOFF_MUL(ID=1, ShieldstunMul=1.2)
			ATK_SET_SHIELD_SETOFF_MUL(ID=2, ShieldstunMul=1.2)
			AttackModule::set_add_reaction_frame_revised(ID=0, Frames=14.0, Unk=false)
			AttackModule::set_add_reaction_frame_revised(ID=1, Frames=14.0, Unk=false)
			AttackModule::set_add_reaction_frame_revised(ID=2, Frames=14.0, Unk=false)
			AttackModule::set_add_reaction_frame_revised(ID=3, Frames=13.0, Unk=false)
			AttackModule::set_add_reaction_frame_revised(ID=4, Frames=13.0, Unk=false)
			AttackModule::set_add_reaction_frame_revised(ID=5, Frames=13.0, Unk=false)
			AttackModule::set_attack_camera_quake_forced(0, CAMERA_QUAKE_KIND_L, false)
			AttackModule::set_attack_camera_quake_forced(1, CAMERA_QUAKE_KIND_L, false)
			AttackModule::set_attack_camera_quake_forced(2, CAMERA_QUAKE_KIND_L, false)
			AttackModule::set_attack_camera_quake_forced(3, CAMERA_QUAKE_KIND_L, false)
			AttackModule::set_attack_camera_quake_forced(4, CAMERA_QUAKE_KIND_L, false)
			AttackModule::set_attack_camera_quake_forced(5, CAMERA_QUAKE_KIND_L, false)
		}
		frame(Frame=11)
		if(is_excute){
			ATTACK(ID=0, Part=0, Bone=hash40("handr"), Damage=14.5, Angle=88, KBG=5, FKB=0, BKB=102, Size=2.5, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.3, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=5, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_G, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_paralyze"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_DEMON_PUNCH01, Type=ATTACK_REGION_PUNCH)
			ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=14.5, Angle=88, KBG=5, FKB=0, BKB=102, Size=5.0, X=0.0, Y=18.0, Z=5.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.3, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=5, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_G, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_paralyze"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_DEMON_PUNCH01, Type=ATTACK_REGION_PUNCH)
			ATTACK(ID=2, Part=0, Bone=hash40("top"), Damage=14.5, Angle=88, KBG=5, FKB=0, BKB=102, Size=3.0, X=-1.0, Y=13.0, Z=7.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.3, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=5, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_G, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_paralyze"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_DEMON_PUNCH01, Type=ATTACK_REGION_PUNCH)
			ATTACK(ID=3, Part=0, Bone=hash40("handr"), Damage=14.0, Angle=81, KBG=5, FKB=0, BKB=92, Size=2.5, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.3, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=5, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_A, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_paralyze"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_DEMON_PUNCH01, Type=ATTACK_REGION_PUNCH)
			ATTACK(ID=4, Part=0, Bone=hash40("top"), Damage=14.0, Angle=81, KBG=5, FKB=0, BKB=92, Size=5.0, X=0.0, Y=18.0, Z=5.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.3, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=5, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_A, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_paralyze"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_DEMON_PUNCH01, Type=ATTACK_REGION_PUNCH)
			ATTACK(ID=5, Part=0, Bone=hash40("top"), Damage=14.0, Angle=81, KBG=5, FKB=0, BKB=92, Size=3.0, X=-1.0, Y=13.0, Z=7.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.3, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=5, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_A, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_paralyze"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_DEMON_PUNCH01, Type=ATTACK_REGION_PUNCH)
			ATK_SET_SHIELD_SETOFF_MUL(ID=0, ShieldstunMul=1.2)
			ATK_SET_SHIELD_SETOFF_MUL(ID=1, ShieldstunMul=1.2)
			ATK_SET_SHIELD_SETOFF_MUL(ID=2, ShieldstunMul=1.2)
			AttackModule::set_add_reaction_frame_revised(ID=0, Frames=14.0, Unk=false)
			AttackModule::set_add_reaction_frame_revised(ID=1, Frames=14.0, Unk=false)
			AttackModule::set_add_reaction_frame_revised(ID=2, Frames=14.0, Unk=false)
			AttackModule::set_add_reaction_frame_revised(ID=3, Frames=13.0, Unk=false)
			AttackModule::set_add_reaction_frame_revised(ID=4, Frames=13.0, Unk=false)
			AttackModule::set_add_reaction_frame_revised(ID=5, Frames=13.0, Unk=false)
			AttackModule::set_attack_camera_quake_forced(0, CAMERA_QUAKE_KIND_L, false)
			AttackModule::set_attack_camera_quake_forced(1, CAMERA_QUAKE_KIND_L, false)
			AttackModule::set_attack_camera_quake_forced(2, CAMERA_QUAKE_KIND_L, false)
			AttackModule::set_attack_camera_quake_forced(3, CAMERA_QUAKE_KIND_L, false)
			AttackModule::set_attack_camera_quake_forced(4, CAMERA_QUAKE_KIND_L, false)
			AttackModule::set_attack_camera_quake_forced(5, CAMERA_QUAKE_KIND_L, false)
		}
		frame(Frame=12)
		if(is_excute){
			ATTACK(ID=0, Part=0, Bone=hash40("handr"), Damage=14.5, Angle=88, KBG=5, FKB=0, BKB=102, Size=2.5, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.3, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=5, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_G, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_paralyze"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_DEMON_PUNCH01, Type=ATTACK_REGION_PUNCH)
			ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=14.5, Angle=88, KBG=5, FKB=0, BKB=102, Size=5.0, X=0.0, Y=19.0, Z=5.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.3, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=5, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_G, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_paralyze"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_DEMON_PUNCH01, Type=ATTACK_REGION_PUNCH)
			ATTACK(ID=2, Part=0, Bone=hash40("top"), Damage=14.5, Angle=88, KBG=5, FKB=0, BKB=102, Size=3.0, X=0.0, Y=15.0, Z=7.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.3, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=5, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_G, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_paralyze"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_DEMON_PUNCH01, Type=ATTACK_REGION_PUNCH)
			ATTACK(ID=3, Part=0, Bone=hash40("handr"), Damage=14.0, Angle=81, KBG=5, FKB=0, BKB=92, Size=2.5, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.3, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=5, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_A, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_paralyze"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_DEMON_PUNCH01, Type=ATTACK_REGION_PUNCH)
			ATTACK(ID=4, Part=0, Bone=hash40("top"), Damage=14.0, Angle=81, KBG=5, FKB=0, BKB=92, Size=5.0, X=0.0, Y=19.0, Z=5.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.3, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=5, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_A, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_paralyze"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_DEMON_PUNCH01, Type=ATTACK_REGION_PUNCH)
			ATTACK(ID=5, Part=0, Bone=hash40("top"), Damage=14.0, Angle=81, KBG=5, FKB=0, BKB=92, Size=3.0, X=0.0, Y=15.0, Z=7.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.3, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=5, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_A, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_paralyze"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_DEMON_PUNCH01, Type=ATTACK_REGION_PUNCH)
			ATK_SET_SHIELD_SETOFF_MUL(ID=0, ShieldstunMul=1.2)
			ATK_SET_SHIELD_SETOFF_MUL(ID=1, ShieldstunMul=1.2)
			ATK_SET_SHIELD_SETOFF_MUL(ID=2, ShieldstunMul=1.2)
			AttackModule::set_add_reaction_frame_revised(ID=0, Frames=14.0, Unk=false)
			AttackModule::set_add_reaction_frame_revised(ID=1, Frames=14.0, Unk=false)
			AttackModule::set_add_reaction_frame_revised(ID=2, Frames=14.0, Unk=false)
			AttackModule::set_add_reaction_frame_revised(ID=3, Frames=13.0, Unk=false)
			AttackModule::set_add_reaction_frame_revised(ID=4, Frames=13.0, Unk=false)
			AttackModule::set_add_reaction_frame_revised(ID=5, Frames=13.0, Unk=false)
			AttackModule::set_attack_camera_quake_forced(0, CAMERA_QUAKE_KIND_L, false)
			AttackModule::set_attack_camera_quake_forced(1, CAMERA_QUAKE_KIND_L, false)
			AttackModule::set_attack_camera_quake_forced(2, CAMERA_QUAKE_KIND_L, false)
			AttackModule::set_attack_camera_quake_forced(3, CAMERA_QUAKE_KIND_L, false)
			AttackModule::set_attack_camera_quake_forced(4, CAMERA_QUAKE_KIND_L, false)
			AttackModule::set_attack_camera_quake_forced(5, CAMERA_QUAKE_KIND_L, false)
		}
		frame(Frame=14)
		if(is_excute){
			AttackModule::clear(ID=0, false)
			AttackModule::clear(ID=1, false)
			AttackModule::clear(ID=2, false)
			AttackModule::clear(ID=3, false)
			AttackModule::clear(ID=4, false)
			AttackModule::clear(ID=5, false)
		}
		frame(Frame=15)
		if(is_excute){
			AttackModule::clear(ID=6, false)
			HitModule::set_status_all(smash::app::HitStatus(*HIT_STATUS_NORMAL), 0)
		}
		FT_MOTION_RATE(FSM=0.9)
    });
}
#[acmd_script( 
agent = "demon", 
scripts = ["game_attackstep2"], 
category = ACMD_GAME)]
unsafe fn kaz_wgf(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		frame(Frame=10)
		if(is_excute){
			ATTACK(ID=0, Part=0, Bone=hash40("handr"), Damage=13.5, Angle=88, KBG=5, FKB=0, BKB=102, Size=2.5, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.3, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=4, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_G, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_DEMON_PUNCH01, Type=ATTACK_REGION_PUNCH)
			ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=13.5, Angle=88, KBG=5, FKB=0, BKB=102, Size=5.0, X=0.0, Y=13.0, Z=6.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.3, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=4, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_G, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_DEMON_PUNCH01, Type=ATTACK_REGION_PUNCH)
			ATTACK(ID=2, Part=0, Bone=hash40("top"), Damage=13.5, Angle=88, KBG=5, FKB=0, BKB=102, Size=3.0, X=-1.0, Y=9.0, Z=3.5, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.3, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=4, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_G, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_DEMON_PUNCH01, Type=ATTACK_REGION_PUNCH)
			ATTACK(ID=3, Part=0, Bone=hash40("handr"), Damage=13.0, Angle=73, KBG=5, FKB=0, BKB=90, Size=2.5, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.3, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=4, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_A, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_DEMON_PUNCH01, Type=ATTACK_REGION_PUNCH)
			ATTACK(ID=4, Part=0, Bone=hash40("top"), Damage=13.0, Angle=73, KBG=5, FKB=0, BKB=90, Size=5.0, X=0.0, Y=13.0, Z=6.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.3, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=4, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_A, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_DEMON_PUNCH01, Type=ATTACK_REGION_PUNCH)
			ATTACK(ID=5, Part=0, Bone=hash40("top"), Damage=13.0, Angle=73, KBG=5, FKB=0, BKB=90, Size=3.0, X=-1.0, Y=9.0, Z=3.5, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.3, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=4, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_A, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_DEMON_PUNCH01, Type=ATTACK_REGION_PUNCH)
			ATK_SET_SHIELD_SETOFF_MUL(ID=0, ShieldstunMul=1.2)
			ATK_SET_SHIELD_SETOFF_MUL(ID=1, ShieldstunMul=1.2)
			ATK_SET_SHIELD_SETOFF_MUL(ID=2, ShieldstunMul=1.2)
			AttackModule::set_add_reaction_frame_revised(ID=0, Frames=13.0, Unk=false)
			AttackModule::set_add_reaction_frame_revised(ID=1, Frames=13.0, Unk=false)
			AttackModule::set_add_reaction_frame_revised(ID=2, Frames=13.0, Unk=false)
			AttackModule::set_add_reaction_frame_revised(ID=3, Frames=12.0, Unk=false)
			AttackModule::set_add_reaction_frame_revised(ID=4, Frames=12.0, Unk=false)
			AttackModule::set_add_reaction_frame_revised(ID=5, Frames=12.0, Unk=false)
			AttackModule::set_attack_camera_quake_forced(0, CAMERA_QUAKE_KIND_M, false)
			AttackModule::set_attack_camera_quake_forced(1, CAMERA_QUAKE_KIND_M, false)
			AttackModule::set_attack_camera_quake_forced(2, CAMERA_QUAKE_KIND_M, false)
			AttackModule::set_attack_camera_quake_forced(3, CAMERA_QUAKE_KIND_M, false)
			AttackModule::set_attack_camera_quake_forced(4, CAMERA_QUAKE_KIND_M, false)
			AttackModule::set_attack_camera_quake_forced(5, CAMERA_QUAKE_KIND_M, false)
		}
		frame(Frame=11)
		if(is_excute){
			ATTACK(ID=0, Part=0, Bone=hash40("handr"), Damage=13.5, Angle=88, KBG=5, FKB=0, BKB=102, Size=2.5, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.3, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=4, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_G, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_DEMON_PUNCH01, Type=ATTACK_REGION_PUNCH)
			ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=13.5, Angle=88, KBG=5, FKB=0, BKB=102, Size=5.0, X=0.0, Y=18.0, Z=5.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.3, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=4, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_G, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_DEMON_PUNCH01, Type=ATTACK_REGION_PUNCH)
			ATTACK(ID=2, Part=0, Bone=hash40("top"), Damage=13.5, Angle=88, KBG=5, FKB=0, BKB=102, Size=3.0, X=-1.0, Y=13.0, Z=7.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.3, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=4, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_G, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_DEMON_PUNCH01, Type=ATTACK_REGION_PUNCH)
			ATTACK(ID=3, Part=0, Bone=hash40("handr"), Damage=13.0, Angle=73, KBG=5, FKB=0, BKB=92, Size=2.5, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.3, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=4, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_A, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_DEMON_PUNCH01, Type=ATTACK_REGION_PUNCH)
			ATTACK(ID=4, Part=0, Bone=hash40("top"), Damage=13.0, Angle=73, KBG=5, FKB=0, BKB=92, Size=5.0, X=0.0, Y=18.0, Z=5.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.3, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=4, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_A, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_DEMON_PUNCH01, Type=ATTACK_REGION_PUNCH)
			ATTACK(ID=5, Part=0, Bone=hash40("top"), Damage=13.0, Angle=73, KBG=5, FKB=0, BKB=92, Size=3.0, X=-1.0, Y=13.0, Z=7.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.3, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=4, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_A, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_DEMON_PUNCH01, Type=ATTACK_REGION_PUNCH)
			ATK_SET_SHIELD_SETOFF_MUL(ID=0, ShieldstunMul=1.2)
			ATK_SET_SHIELD_SETOFF_MUL(ID=1, ShieldstunMul=1.2)
			ATK_SET_SHIELD_SETOFF_MUL(ID=2, ShieldstunMul=1.2)
			AttackModule::set_add_reaction_frame_revised(ID=0, Frames=13.0, Unk=false)
			AttackModule::set_add_reaction_frame_revised(ID=1, Frames=13.0, Unk=false)
			AttackModule::set_add_reaction_frame_revised(ID=2, Frames=13.0, Unk=false)
			AttackModule::set_add_reaction_frame_revised(ID=3, Frames=12.0, Unk=false)
			AttackModule::set_add_reaction_frame_revised(ID=4, Frames=12.0, Unk=false)
			AttackModule::set_add_reaction_frame_revised(ID=5, Frames=12.0, Unk=false)
			AttackModule::set_attack_camera_quake_forced(0, CAMERA_QUAKE_KIND_M, false)
			AttackModule::set_attack_camera_quake_forced(1, CAMERA_QUAKE_KIND_M, false)
			AttackModule::set_attack_camera_quake_forced(2, CAMERA_QUAKE_KIND_M, false)
			AttackModule::set_attack_camera_quake_forced(3, CAMERA_QUAKE_KIND_M, false)
			AttackModule::set_attack_camera_quake_forced(4, CAMERA_QUAKE_KIND_M, false)
			AttackModule::set_attack_camera_quake_forced(5, CAMERA_QUAKE_KIND_M, false)
		}
		frame(Frame=12)
		if(is_excute){
			ATTACK(ID=0, Part=0, Bone=hash40("handr"), Damage=13.5, Angle=88, KBG=5, FKB=0, BKB=102, Size=2.5, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.3, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=4, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_G, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_DEMON_PUNCH01, Type=ATTACK_REGION_PUNCH)
			ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=13.5, Angle=88, KBG=5, FKB=0, BKB=102, Size=5.0, X=0.0, Y=19.0, Z=5.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.3, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=4, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_G, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_DEMON_PUNCH01, Type=ATTACK_REGION_PUNCH)
			ATTACK(ID=2, Part=0, Bone=hash40("top"), Damage=13.5, Angle=88, KBG=5, FKB=0, BKB=102, Size=3.0, X=0.0, Y=15.0, Z=7.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.3, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=4, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_G, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_DEMON_PUNCH01, Type=ATTACK_REGION_PUNCH)
			ATTACK(ID=3, Part=0, Bone=hash40("handr"), Damage=13.0, Angle=73, KBG=5, FKB=0, BKB=92, Size=2.5, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.3, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=4, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_A, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_DEMON_PUNCH01, Type=ATTACK_REGION_PUNCH)
			ATTACK(ID=4, Part=0, Bone=hash40("top"), Damage=13.0, Angle=73, KBG=5, FKB=0, BKB=92, Size=5.0, X=0.0, Y=19.0, Z=5.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.3, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=4, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_A, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_DEMON_PUNCH01, Type=ATTACK_REGION_PUNCH)
			ATTACK(ID=5, Part=0, Bone=hash40("top"), Damage=13.0, Angle=73, KBG=5, FKB=0, BKB=92, Size=3.0, X=0.0, Y=15.0, Z=7.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.3, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=4, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_A, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_DEMON_PUNCH01, Type=ATTACK_REGION_PUNCH)
			ATK_SET_SHIELD_SETOFF_MUL(ID=0, ShieldstunMul=1.2)
			ATK_SET_SHIELD_SETOFF_MUL(ID=1, ShieldstunMul=1.2)
			ATK_SET_SHIELD_SETOFF_MUL(ID=2, ShieldstunMul=1.2)
			AttackModule::set_add_reaction_frame_revised(ID=0, Frames=13.0, Unk=false)
			AttackModule::set_add_reaction_frame_revised(ID=1, Frames=13.0, Unk=false)
			AttackModule::set_add_reaction_frame_revised(ID=2, Frames=13.0, Unk=false)
			AttackModule::set_add_reaction_frame_revised(ID=3, Frames=12.0, Unk=false)
			AttackModule::set_add_reaction_frame_revised(ID=4, Frames=12.0, Unk=false)
			AttackModule::set_add_reaction_frame_revised(ID=5, Frames=12.0, Unk=false)
			AttackModule::set_attack_camera_quake_forced(0, CAMERA_QUAKE_KIND_M, false)
			AttackModule::set_attack_camera_quake_forced(1, CAMERA_QUAKE_KIND_M, false)
			AttackModule::set_attack_camera_quake_forced(2, CAMERA_QUAKE_KIND_M, false)
			AttackModule::set_attack_camera_quake_forced(3, CAMERA_QUAKE_KIND_M, false)
			AttackModule::set_attack_camera_quake_forced(4, CAMERA_QUAKE_KIND_M, false)
			AttackModule::set_attack_camera_quake_forced(5, CAMERA_QUAKE_KIND_M, false)
		}
		frame(Frame=14)
		FT_MOTION_RATE(FSM=0.9)
		if(is_excute){
			AttackModule::clear_all()
		}
	});
}
#[acmd_script( 
agent = "demon", 
scripts = ["game_attacks3","game_attacks3lw","game_attacks3hi"], 
category = ACMD_GAME)]
unsafe fn kaz_ftilt(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		FT_MOTION_RATE(FSM=0.667)
		frame(Frame=12)
		FT_MOTION_RATE(FSM=1)
		if(is_excute){
			ATTACK(ID=0, Part=0, Bone=hash40("footl"), Damage=12.5, Angle=361, KBG=71, FKB=0, BKB=37, Size=2.5, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.35, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=2, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_DEMON_KICK, Type=ATTACK_REGION_KICK)
			ATTACK(ID=1, Part=0, Bone=hash40("footl"), Damage=12.5, Angle=361, KBG=71, FKB=0, BKB=37, Size=4.25, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.35, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=2, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_DEMON_KICK, Type=ATTACK_REGION_KICK)
			ATTACK(ID=2, Part=0, Bone=hash40("kneel"), Damage=12.5, Angle=361, KBG=71, FKB=0, BKB=37, Size=3.5, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.35, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=2, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_DEMON_KICK, Type=ATTACK_REGION_KICK)
			ATTACK(ID=3, Part=0, Bone=hash40("legl"), Damage=12.5, Angle=361, KBG=71, FKB=0, BKB=37, Size=3.5, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.35, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=2, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_DEMON_KICK, Type=ATTACK_REGION_KICK)
		}
		frame(Frame=17)
		FT_MOTION_RATE(FSM=0.78)
		if(is_excute){
			AttackModule::clear_all()
		}
	});
}
pub fn install() {
	smashline::install_acmd_scripts!(kaz_ftilt, kaz_flashtornado, kaz_staturesmash);
    smashline::install_agent_frames!(kaz_frame);
	skyline::install_hook!(reaction_frame_hook);
}
