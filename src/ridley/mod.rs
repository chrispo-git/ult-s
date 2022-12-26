use smash::app::sv_animcmd::*;
use smash::phx::Hash40;
use smash::app::lua_bind::*;
use smash::lib::lua_const::*;
use smash::app::utility::get_kind;
use smash::hash40;
use smash::lua2cpp::*;
use smashline::*;
use smash_script::*;
use smash::lib::{L2CValue, L2CAgent};
use std::mem;
use smash::app::*;
use smash::phx::Vector3f;
use crate::util::*;

static mut HAS_DOUBLE_UPB : [bool; 8] = [false; 8];
static mut SPEED_DOUBLE_UPB : [bool; 8] = [false; 8];


#[fighter_frame_callback]
pub fn ridley(fighter : &mut L2CFighterCommon) {
    unsafe {
        let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent); 
		let status_kind = smash::app::lua_bind::StatusModule::status_kind(boma);
		let ENTRY_ID = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
		let mut stick_x = ControlModule::get_stick_x(boma);
		let stick_y = ControlModule::get_stick_y(boma);
		stick_x *= PostureModule::lr(boma);
		let fighter_kind = smash::app::utility::get_kind(boma);
		if fighter_kind == *FIGHTER_KIND_RIDLEY {
			if MotionModule::motion_kind(boma) == hash40("attack_air_lw") {
				if (33..35).contains(&(MotionModule::frame(boma) as i32)) {
					let mut is_bounce = false;
					for i in 0..12 {
						for f in 0..16 {
							if ray_check_pos(boma, (f as f32)-8.0, (i as f32)*-1.0 - 10.0, true) == 1 {
								is_bounce = true;
							};
						};
					};
					if  is_bounce || AttackModule::is_infliction_status(boma, *COLLISION_CATEGORY_MASK_ALL) {
						macros::SET_SPEED_EX(fighter, 0.9, 2.2, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
					};
				};
			};
			if StatusModule::situation_kind(boma) != *SITUATION_KIND_AIR && ![*FIGHTER_RIDLEY_STATUS_KIND_SPECIAL_HI_CHARGE_B, *FIGHTER_RIDLEY_STATUS_KIND_SPECIAL_HI_CHARGE_F, *FIGHTER_RIDLEY_STATUS_KIND_SPECIAL_HI_CHARGE_HI].contains(&status_kind) {
				HAS_DOUBLE_UPB[ENTRY_ID] = false;
			};
			/*if [hash40("special_s_start")].contains(&MotionModule::motion_kind(boma)) {
				if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_CATCH) {
					StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_CATCH_DASH, true);
				} else if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_ATTACK){
					StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_DASH, true);
				};
			};*/
			if [*FIGHTER_RIDLEY_STATUS_KIND_SPECIAL_N_SHOOT, *FIGHTER_RIDLEY_STATUS_KIND_SPECIAL_N_CHARGE].contains(&status_kind) {
				if status_kind == *FIGHTER_RIDLEY_STATUS_KIND_SPECIAL_N_SHOOT {
					if StatusModule::is_situation_changed(boma) {
						StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_LANDING, true);
					};
				};
				if StatusModule::situation_kind(boma) == *SITUATION_KIND_AIR {
					let cat2 = ControlModule::get_command_flag_cat(boma, 1);
					if (cat2 & *FIGHTER_PAD_CMD_CAT2_FLAG_FALL_JUMP) != 0 && stick_y < -0.66 && SPEED_Y[ENTRY_ID] <= 0.0 {
						WorkModule::set_flag(boma, true, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_DIVE);
					}
				};
			};
			if [hash40("special_s_start"), hash40("special_air_s_start")].contains(&MotionModule::motion_kind(boma)) {
				if MotionModule::frame(boma) < 24.0 {
					MotionModule::set_rate(boma, 1.5);
				} else {
					MotionModule::set_rate(boma, 1.0);
				};
			};
			if [hash40("special_air_s_start")].contains(&MotionModule::motion_kind(boma)) {
				if GroundModule::is_wall_touch_line(boma, *GROUND_TOUCH_FLAG_SIDE as u32) && MotionModule::frame(boma) > 27.0 && MotionModule::frame(boma) < 35.0 && stick_x < -0.7 {
					StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_PASSIVE_WALL_JUMP, true);
				};
			};
			if status_kind == *FIGHTER_RIDLEY_STATUS_KIND_SPECIAL_S_CATCH {
				macros::SET_SPEED_EX(fighter, 1.5, 0.75, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
				MotionModule::set_rate(boma, 2.0);
				StatusModule::set_keep_situation_air(boma, true);
			};
			if status_kind == *FIGHTER_RIDLEY_STATUS_KIND_SPECIAL_S_FALL {
				macros::SET_SPEED_EX(fighter, 1.25, 0.4, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
				StatusModule::set_keep_situation_air(boma, true);
			};
			if [*FIGHTER_RIDLEY_STATUS_KIND_SPECIAL_HI_CHARGE_B, *FIGHTER_RIDLEY_STATUS_KIND_SPECIAL_HI_CHARGE_F, *FIGHTER_RIDLEY_STATUS_KIND_SPECIAL_HI_CHARGE_HI].contains(&status_kind) &&  WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_HIT_STOP_ATTACK_SUSPEND_FRAME) < 1 && AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT) {
				if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_ATTACK){
					StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_AIR, false);
				};
				if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_SPECIAL) && HAS_DOUBLE_UPB[ENTRY_ID] == false {
					HAS_DOUBLE_UPB[ENTRY_ID] = true;
					SPEED_DOUBLE_UPB[ENTRY_ID] = true;
					StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_SPECIAL_HI, false);
				};
			};
			if [*FIGHTER_STATUS_KIND_SPECIAL_HI, *FIGHTER_RIDLEY_STATUS_KIND_SPECIAL_HI_HOVER].contains(&status_kind) && SPEED_DOUBLE_UPB[ENTRY_ID] == true {
				if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_HI && MotionModule::frame(boma) > 3.0 {
					StatusModule::change_status_request_from_script(boma, *FIGHTER_RIDLEY_STATUS_KIND_SPECIAL_HI_HOVER, false);
				};
				if status_kind == *FIGHTER_RIDLEY_STATUS_KIND_SPECIAL_HI_HOVER {
					MotionModule::set_rate(boma, 8.0);
				};
			} else {
				SPEED_DOUBLE_UPB[ENTRY_ID] = false;
			};
		};
	};
}
#[acmd_script(
    agent = "ridley",
    script =  "game_attackairhi",
    category = ACMD_GAME,
	low_priority)]
unsafe fn ridley_uair(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		frame(Frame=4)
		if(is_excute){
			WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
		}
		frame(Frame=5)
		if(is_excute){
			JostleModule::set_status(false)
		}
		frame(Frame=10)
		if(is_excute){
			JostleModule::set_status(true)
		}
		frame(Frame=11)
		if(is_excute){
			ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=12.0, Angle=86, KBG=62, FKB=0, BKB=70, Size=12.0, X=0.0, Y=20.5, Z=0.8, X2=0.0, Y2=15.5, Z2=0.8, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_BODY)
			ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=14.0, Angle=90, KBG=64, FKB=0, BKB=70, Size=8.5, X=0.0, Y=35.5, Z=2.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.4, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_sting"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_BODY)
		}
		wait(Frames=2)
		if(is_excute){
			AttackModule::clear(ID=0, false)
		}
		wait(Frames=1)
		if(is_excute){
			AttackModule::clear_all()
		}
		frame(Frame=35)
		if(is_excute){
			WorkModule::off_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
		}
    });
}		
#[acmd_script(
    agent = "ridley",
    script =  "game_attackairb",
    category = ACMD_GAME,
	low_priority)]
unsafe fn ridley_bair(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		if(is_excute){
			WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
		}
		frame(Frame=1)
		FT_MOTION_RATE(FSM=0.5)
		frame(Frame=8)
		FT_MOTION_RATE(FSM=1)
		frame(Frame=14)
		if(is_excute){
			ATTACK(ID=0, Part=0, Bone=hash40("legr"), Damage=16.0, Angle=361, KBG=83, FKB=0, BKB=35, Size=7.0, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_B, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
			ATTACK(ID=1, Part=0, Bone=hash40("kneer"), Damage=16.0, Angle=361, KBG=83, FKB=0, BKB=35, Size=7.0, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_B, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
			ATTACK(ID=2, Part=0, Bone=hash40("footr"), Damage=16.0, Angle=361, KBG=83, FKB=0, BKB=35, Size=8.0, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_B, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
		}
		wait(Frames=3)
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
    agent = "ridley",
    script =  "game_attackairf",
    category = ACMD_GAME,
	low_priority)]
unsafe fn ridley_fair(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		FT_MOTION_RATE(FSM=0.5)
		frame(Frame=3)
		if(is_excute){
			WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
		}
		frame(Frame=10)
		if(is_excute){
			ATTACK(ID=0, Part=0, Bone=hash40("tail8"), Damage=5.0, Angle=367, KBG=100, FKB=35, BKB=0, Size=5.0, X=6.4, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.8, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_A, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_sting"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_TAIL)
			ATTACK(ID=1, Part=0, Bone=hash40("tail8"), Damage=5.0, Angle=367, KBG=100, FKB=40, BKB=0, Size=5.0, X=6.4, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.8, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_G, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_sting"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_TAIL)
			ATTACK(ID=2, Part=0, Bone=hash40("tail8"), Damage=3.0, Angle=367, KBG=100, FKB=40, BKB=0, Size=6.5, X=-0.5, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.8, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_A, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_sting"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_TAIL)
			ATTACK(ID=3, Part=0, Bone=hash40("tail8"), Damage=3.0, Angle=367, KBG=100, FKB=45, BKB=0, Size=6.5, X=-0.5, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.8, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_G, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_sting"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_TAIL)
			ATTACK(ID=4, Part=0, Bone=hash40("top"), Damage=3.0, Angle=367, KBG=100, FKB=40, BKB=0, Size=7.2, X=0.0, Y=8.5, Z=5.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.8, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_A, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_sting"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_TAIL)
			ATTACK(ID=5, Part=0, Bone=hash40("top"), Damage=3.0, Angle=66, KBG=100, FKB=55, BKB=0, Size=7.2, X=0.0, Y=8.5, Z=5.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.8, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_G, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_sting"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_TAIL)
		}
		FT_MOTION_RATE(FSM=1)
		wait(Frames=2)
		if(is_excute){
			AttackModule::clear_all()
		}
		frame(Frame=13)
		if(is_excute){
			ATTACK(ID=0, Part=0, Bone=hash40("tail8"), Damage=5.0, Angle=367, KBG=100, FKB=35, BKB=0, Size=5.0, X=6.4, Y=-2.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.8, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_sting"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_TAIL)
			ATTACK(ID=1, Part=0, Bone=hash40("tail8"), Damage=3.0, Angle=367, KBG=100, FKB=40, BKB=0, Size=6.5, X=-0.5, Y=-2.5, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.8, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_sting"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_TAIL)
			ATTACK(ID=2, Part=0, Bone=hash40("top"), Damage=3.0, Angle=367, KBG=100, FKB=40, BKB=0, Size=7.2, X=0.0, Y=8.5, Z=5.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.8, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_sting"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_TAIL)
		}
		wait(Frames=2)
		if(is_excute){
			AttackModule::clear_all()
		}
		frame(Frame=16)
		if(is_excute){
			ATTACK(ID=0, Part=0, Bone=hash40("tail8"), Damage=7.0, Angle=361, KBG=87, FKB=0, BKB=45, Size=5.0, X=6.4, Y=2.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.2, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_sting"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_TAIL)
			ATTACK(ID=1, Part=0, Bone=hash40("tail8"), Damage=5.0, Angle=361, KBG=90, FKB=0, BKB=45, Size=6.5, X=-0.5, Y=2.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_sting"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_TAIL)
			ATTACK(ID=2, Part=0, Bone=hash40("top"), Damage=5.0, Angle=361, KBG=90, FKB=0, BKB=45, Size=7.2, X=0.0, Y=8.5, Z=5.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_sting"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_TAIL)
		}
		wait(Frames=2)
		if(is_excute){
			AttackModule::clear_all()
		}
		frame(Frame=38)
		if(is_excute){
			WorkModule::off_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
		}
    });
}		
#[acmd_script(
    agent = "ridley",
    script =  "game_specialairsfalljump",
    category = ACMD_GAME,
	low_priority)]
unsafe fn ridley_sideb_end(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		if(is_excute){
			ATTACK_ABS(Kind=FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, ID=0, Damage=7.0, Angle=88, KBG=75, FKB=0, BKB=70, Hitlag=0.0, Unk=1.0, FacingRestrict=ATTACK_LR_CHECK_F, Unk=0.0, Unk=true, Effect=hash40("collision_attr_sting"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_THROW)
			ATTACK_ABS(Kind=FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, ID=0, Damage=3.0, Angle=88, KBG=75, FKB=0, BKB=70, Hitlag=0.0, Unk=1.0, FacingRestrict=ATTACK_LR_CHECK_F, Unk=0.0, Unk=true, Effect=hash40("collision_attr_sting"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_THROW)
		}
		frame(Frame=2)
		if(is_excute){
			WorkModule::on_flag(Flag=FIGHTER_RIDLEY_STATUS_SPECIAL_S_FLAG_THROW)
			WorkModule::on_flag(Flag=FIGHTER_RIDLEY_STATUS_SPECIAL_S_FLAG_ENABLE_GRAVITY)
		}
		frame(Frame=25)
		if(is_excute){
			CancelModule::enable_cancel()
		}
		frame(Frame=38)
		if(is_excute){
			WorkModule::on_flag(Flag=FIGHTER_RIDLEY_STATUS_SPECIAL_S_FLAG_ENABLE_CONTROL_JUMP)
			sv_battle_object::notify_event_msc_cmd(0x2127e37c07u64, GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES)
		}
    });
}		
#[acmd_script(
    agent = "ridley",
    script =  "effect_specialairsfall",
    category = ACMD_EFFECT,
	low_priority)]
unsafe fn ridley_sideb_fall_eff(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
}	

#[acmd_script(
    agent = "ridley",
    script =  "game_attackairlw",
    category = ACMD_GAME,
	low_priority)]
unsafe fn ridley_dair(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		frame(Frame=8)
		FT_MOTION_RATE(FSM=0.4)
		if(is_excute){
			WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
		}
		wait(Frames=20)
		FT_MOTION_RATE(FSM=1)
		frame(Frame=33)
		if(is_excute){
			ATTACK(ID=0, Part=0, Bone=hash40("tail3"), Damage=12.0, Angle=50, KBG=95, FKB=0, BKB=45, Size=5.0, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_sting"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_TAIL)
			ATTACK(ID=1, Part=0, Bone=hash40("tail5"), Damage=12.0, Angle=50, KBG=95, FKB=0, BKB=45, Size=5.0, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_sting"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_TAIL)
			ATTACK(ID=2, Part=0, Bone=hash40("tail8"), Damage=18.0, Angle=280, KBG=100, FKB=0, BKB=20, Size=5.0, X=6.8, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.3, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_sting"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_TAIL)
			ATTACK(ID=3, Part=0, Bone=hash40("tail7"), Damage=12.0, Angle=50, KBG=95, FKB=0, BKB=45, Size=5.0, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_sting"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_TAIL)
		}
		frame(Frame=35)
		if(is_excute){
			ATTACK(ID=0, Part=0, Bone=hash40("tail3"), Damage=8.0, Angle=50, KBG=95, FKB=0, BKB=45, Size=5.0, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_sting"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_TAIL)
			ATTACK(ID=1, Part=0, Bone=hash40("tail5"), Damage=8.0, Angle=50, KBG=95, FKB=0, BKB=45, Size=5.0, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_sting"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_TAIL)
			ATTACK(ID=2, Part=0, Bone=hash40("tail8"), Damage=15.0, Angle=361, KBG=100, FKB=0, BKB=20, Size=5.0, X=6.8, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.15, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_sting"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_TAIL)
			ATTACK(ID=3, Part=0, Bone=hash40("tail7"), Damage=8.0, Angle=50, KBG=95, FKB=0, BKB=45, Size=5.0, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_sting"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_TAIL)
		}
		frame(Frame=39)
		if(is_excute){
			AttackModule::clear_all()
		}
		frame(Frame=58)
		if(is_excute){
			WorkModule::off_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
		}
    });
}		

#[acmd_script( 
	agent = "ridley", 
	script = "effect_attackairlw",
	category = ACMD_EFFECT,
	low_priority)]
unsafe fn ridley_dair_eff(fighter : &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 7.0);
    if macros::is_excute(fighter) {
        macros::EFFECT(fighter, Hash40::new("sys_smash_flash"), Hash40::new("tail8"), 4, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(fighter.lua_state_agent, 29.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW(fighter, Hash40::new("ridley_death_stab_flare"), Hash40::new("tail8"), 0, 0, 0, 0, 180, 0, 1, true);
        macros::EFFECT_FOLLOW(fighter, Hash40::new("ridley_death_stab_line"), Hash40::new("top"), 0, -7, -8, 60, 0, 0, 0.9, true);
    }
    frame(fighter.lua_state_agent, 33.0);
    if macros::is_excute(fighter) {
        macros::EFFECT(fighter, Hash40::new("sys_sp_flash"), Hash40::new("tail8"), 10, -0.0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        macros::LAST_EFFECT_SET_RATE(fighter, 1.5);
    }
    wait(fighter.lua_state_agent, 5.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_OFF_KIND(fighter, Hash40::new("ridley_death_stab_flare"), false, true);
    }
}
		
pub fn install() {
    smashline::install_acmd_scripts!(
		ridley_uair,
		ridley_fair,
		ridley_bair,
		ridley_sideb_end,
		ridley_sideb_fall_eff,
		ridley_dair,
		ridley_dair_eff
    );
	smashline::install_agent_frame_callbacks!(ridley);
}

