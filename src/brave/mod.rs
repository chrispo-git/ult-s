use smash::hash40;
use smash::lib::lua_const::*;
use smash::lua2cpp::*;
use smashline::*;
use smash_script::*;
use smash::app::lua_bind::*;
use smash::phx::Vector2f;
use crate::util::*;
#[acmd_script(
    agent = "brave",
    script =  "game_attackairb",
    category = ACMD_GAME,
	low_priority)]
unsafe fn hero_bair(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		frame(Frame=3)
		FT_MOTION_RATE(FSM=0.5)
		if(is_excute){
			WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
		}
		wait(Frames=8)
		FT_MOTION_RATE(FSM=1)
		frame(Frame=18)
		if(is_excute){
			ATTACK(ID=0, Part=0, Bone=hash40("armr"), Damage=12.0, Angle=36, KBG=96, FKB=0, BKB=32, Size=2.5, X=1.0, Y=1.0, Z=-1.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_B, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
			ATTACK(ID=1, Part=0, Bone=hash40("sword1"), Damage=14.0, Angle=36, KBG=89, FKB=0, BKB=32, Size=3.5, X=9.0, Y=0.0, Z=-2.5, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_B, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
			ATTACK(ID=2, Part=0, Bone=hash40("sword1"), Damage=14.0, Angle=36, KBG=89, FKB=0, BKB=32, Size=4.0, X=2.5, Y=0.0, Z=-2.75, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_B, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
		}
		wait(Frames=3)
		if(is_excute){
			AttackModule::clear_all()
		}
		frame(Frame=47)
		if(is_excute){
			WorkModule::off_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
		}
    });
}	
#[acmd_script(
    agent = "brave",
    script =  "game_attackairhi",
    category = ACMD_GAME,
	low_priority)]
unsafe fn hero_uair(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {	
		FT_MOTION_RATE(FSM=1.5)
		frame(Frame=2)
		if(is_excute){
			WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
		}
		frame(Frame=4)
		FT_MOTION_RATE(FSM=1)
		if(is_excute){
			ATTACK(ID=0, Part=0, Bone=hash40("legr"), Damage=7.0, Angle=73, KBG=106, FKB=0, BKB=49, Size=4.0, X=1.4, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
			ATTACK(ID=1, Part=0, Bone=hash40("kneer"), Damage=7.0, Angle=73, KBG=106, FKB=0, BKB=49, Size=5.0, X=3.4, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
		}
		wait(Frames=5)
		if(is_excute){
			AttackModule::clear_all()
		}
		frame(Frame=19)
		if(is_excute){
			WorkModule::off_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
		}
    });
}	
#[fighter_frame_callback]
pub fn hero(fighter : &mut L2CFighterCommon) {
    unsafe {
        let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent); 
		let fighter_kind = smash::app::utility::get_kind(boma);
		let status_kind = smash::app::lua_bind::StatusModule::status_kind(boma);
		if fighter_kind == *FIGHTER_KIND_BRAVE{
			if [*FIGHTER_BRAVE_STATUS_KIND_SPECIAL_LW_START, *FIGHTER_BRAVE_STATUS_KIND_SPECIAL_LW_SELECT, *FIGHTER_BRAVE_STATUS_KIND_SPECIAL_LW_CANCEL].contains(&status_kind) {
				if (ControlModule::get_command_flag_cat(boma, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_DASH) != 0 {
					if StatusModule::situation_kind(boma) != *SITUATION_KIND_AIR {
						StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_DASH, true);
					};
				};
			};
			if MotionModule::motion_kind(boma) == hash40("attack_s3_s2") && MotionModule::frame(boma) < 2.0 && ControlModule::get_stick_y(boma) > 0.5 {
				/*if ray_check_pos(boma, 3.0, 0.0, false) == 0 {
					let pos = smash::phx::Vector3f { x: PostureModule::pos_x(boma)+3.0, y:  PostureModule::pos_y(boma), z: 0.0 };
					PostureModule::set_pos(boma, &pos);
					PostureModule::init_pos(boma, &pos, true, true);
				}*/
				let speed = smash::phx::Vector3f { x: 0.75, y: 0.0, z: 0.0 };
				KineticModule::add_speed(boma, &speed);
				StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_HI3, true);
			};
			if MotionModule::motion_kind(boma) == hash40("attack_s3_s2") && MotionModule::frame(boma) < 2.0 && ControlModule::get_stick_y(boma) < -0.5 {
				StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_LW4, true);
			};
			if MotionModule::motion_kind(boma) == hash40("attack_s3_s2") && MotionModule::frame(boma) < 2.0 && ControlModule::get_stick_x(boma)*PostureModule::lr(boma) < -0.5 {
				StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_DASH, true);
			};
		};
	};
}
		
pub fn install() {
    smashline::install_acmd_scripts!(
		hero_bair,
		hero_uair
    );
	smashline::install_agent_frame_callbacks!(hero);
}
