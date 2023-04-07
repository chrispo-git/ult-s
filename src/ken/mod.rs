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
use crate::util::*;


static mut KEN_MAX_METER : i32 = 44;
static mut PPP: i32 = 3;
static mut KEN_SUPER : [i32; 8] = [0; 8];
static mut KEN_IS_EX : [bool; 8] = [false; 8];
static mut HAS_ADDED : [bool; 8] = [false; 8];
static mut KEN_FX_TIMER : [i32; 8] = [0; 8];
static mut IS_SUPER : [bool; 8] = [false; 8];
static mut EX_DOWNB : [bool; 8] = [false; 8];
static mut HANDS :  smash::phx::Vector3f =  smash::phx::Vector3f { x: 0.0, y: 0.0, z: 0.0 };
static mut FEET :  smash::phx::Vector3f =  smash::phx::Vector3f { x: 1.0, y: 0.0, z: 0.0 };

#[acmd_script(
    agent = "ken",
    script =  "game_attacklw4",
    category = ACMD_GAME,
	low_priority)]
unsafe fn ken_dsmash(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		frame(Frame=3)
		if(is_excute){
			WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD)
		}
		frame(Frame=5)
		if(is_excute){
			ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=16.0, Angle=25, KBG=47, FKB=0, BKB=50, Size=3.6, X=0.0, Y=2.5, Z=12.0, X2=0.0, Y2=3.0, Z2=8.0, Hitlag=1.5, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=-8, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KEN_KICK, Type=ATTACK_REGION_KICK)
			ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=12.0, Angle=25, KBG=50, FKB=0, BKB=50, Size=2.5, X=0.0, Y=3.0, Z=2.5, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.5, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=-6, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KEN_KICK, Type=ATTACK_REGION_KICK)
			ATTACK(ID=2, Part=0, Bone=hash40("top"), Damage=12.0, Angle=25, KBG=50, FKB=0, BKB=50, Size=2.5, X=0.0, Y=3.0, Z=2.5, X2=0.0, Y2=3.0, Z2=0.0, Hitlag=1.5, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=-6, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KEN_KICK, Type=ATTACK_REGION_KICK)
		}
		wait(Frames=2)
		if(is_excute){
			AttackModule::clear_all()
		}
    });
}		
#[acmd_script(
    agent = "ken",
    script =  "game_attack13",
    category = ACMD_GAME,
	low_priority)]
unsafe fn ken_jab3(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		if(is_excute){
			WorkModule::on_flag(Flag=FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL)
			WorkModule::on_flag(Flag=FIGHTER_RYU_STATUS_ATTACK_FLAG_HIT_CANCEL)
		}
		frame(Frame=8)
		if(is_excute){
			ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=5.0, Angle=45, KBG=50, FKB=0, BKB=70, Size=3.0, X=0.0, Y=11.0, Z=11.5, X2=0.0, Y2=11.0, Z2=3.0, Hitlag=1.5, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KEN_PUNCH, Type=ATTACK_REGION_PUNCH)
			ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=5.0, Angle=45, KBG=50, FKB=0, BKB=70, Size=1.8, X=0.0, Y=8.0, Z=8.0, X2=0.0, Y2=8.0, Z2=3.0, Hitlag=1.5, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KEN_PUNCH, Type=ATTACK_REGION_PUNCH)
		}
		wait(Frames=3)
		if(is_excute){
			AttackModule::clear_all()
			WorkModule::off_flag(Flag=FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL)
			WorkModule::off_flag(Flag=FIGHTER_RYU_STATUS_ATTACK_FLAG_HIT_CANCEL)
		}
    });
}	

//RYU SUPER
#[fighter_frame_callback]
pub fn supers(fighter : &mut L2CFighterCommon) {
    unsafe {
        let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);  
		let fighter_kind = smash::app::utility::get_kind(boma);
		let status_kind = smash::app::lua_bind::StatusModule::status_kind(boma);
		let lua_state = fighter.lua_state_agent;
		let color_id = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);
		let motion_kind = MotionModule::motion_kind(boma);
		let frame = MotionModule::frame(boma);
		let cancel_frame = FighterMotionModuleImpl::get_cancel_frame(boma,smash::phx::Hash40::new_raw(MotionModule::motion_kind(boma)),false) as f32;
		let mut ENTRY_ID = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
		if fighter_kind == *FIGHTER_KIND_KEN {
			let meter_half = KEN_MAX_METER as f32 * 0.5;
			KEN_FX_TIMER[ENTRY_ID] += 1;
			if smash::app::smashball::is_training_mode() == true {
				if status_kind == *FIGHTER_STATUS_KIND_APPEAL {
					KEN_SUPER[ENTRY_ID] = KEN_MAX_METER;
					CancelModule::enable_cancel(boma);
				};
			};
			if KEN_FX_TIMER[ENTRY_ID] == 6 {
				let meter_1: u32 = EffectModule::req_follow(boma, smash::phx::Hash40::new("sys_damage_elec"), smash::phx::Hash40::new("haver"), &HANDS, &HANDS, 0.2, true, 0, 0, 0, 0, 0, true, true) as u32;
				let meter_2: u32 = EffectModule::req_follow(boma, smash::phx::Hash40::new("sys_damage_elec"), smash::phx::Hash40::new("havel"), &HANDS, &HANDS, 0.2, true, 0, 0, 0, 0, 0, true, true) as u32;
				let meter_3: u32 = EffectModule::req_follow(boma, smash::phx::Hash40::new("sys_damage_elec"), smash::phx::Hash40::new("footr"), &FEET, &FEET, 0.25, true, 0, 0, 0, 0, 0, true, true) as u32;
				let meter_4: u32 = EffectModule::req_follow(boma, smash::phx::Hash40::new("sys_damage_elec"), smash::phx::Hash40::new("footl"), &FEET, &FEET, 0.25, true, 0, 0, 0, 0, 0, true, true) as u32;
				if KEN_SUPER[ENTRY_ID] < KEN_MAX_METER && KEN_SUPER[ENTRY_ID] >= meter_half as i32 {
					EffectModule::set_rgb(boma, meter_1, 5.0, 2.75, 0.0);
                    EffectModule::set_rgb(boma, meter_2, 5.0, 2.75, 0.0);
					EffectModule::set_visible(boma, meter_1, true);
					EffectModule::set_visible(boma, meter_2, true);
					EffectModule::set_rgb(boma, meter_3, 5.0, 2.75, 0.0);
                    EffectModule::set_rgb(boma, meter_4, 5.0, 2.75, 0.0);
					EffectModule::set_visible(boma, meter_3, true);
					EffectModule::set_visible(boma, meter_4, true);
					if color_id == 7 {
						EffectModule::set_rgb(boma, meter_3, 0.0, 7.75, 0.0);
						EffectModule::set_rgb(boma, meter_4, 0.0, 7.75, 0.0);
						EffectModule::set_rgb(boma, meter_1, 0.0, 7.75, 0.0);
						EffectModule::set_rgb(boma, meter_2, 0.0, 7.75, 0.0);	
					};
				} else if KEN_SUPER[ENTRY_ID] >= KEN_MAX_METER {
					EffectModule::set_rgb(boma, meter_1, 0.0, 5.0, 5.0);
                    EffectModule::set_rgb(boma, meter_2, 0.0, 5.0, 5.0);
					EffectModule::set_visible(boma, meter_1, true);
					EffectModule::set_visible(boma, meter_2, true);
					EffectModule::set_rgb(boma, meter_3, 0.0, 5.0, 5.0);
                    EffectModule::set_rgb(boma, meter_4, 0.0, 5.0, 5.0);
					EffectModule::set_visible(boma, meter_3, true);
					EffectModule::set_visible(boma, meter_4, true);
					if color_id == 7 {
						EffectModule::set_rgb(boma, meter_3, 10.0, 0.0, 0.0);
						EffectModule::set_rgb(boma, meter_4, 10.0, 0.0, 0.0);
						EffectModule::set_rgb(boma, meter_1, 10.0, 0.0, 0.0);
						EffectModule::set_rgb(boma, meter_2, 10.0, 0.0, 0.0);	
					};
				} else {
					EffectModule::set_visible(boma, meter_1, false);
					EffectModule::set_visible(boma, meter_2, false);
					EffectModule::set_visible(boma, meter_3, false);
					EffectModule::set_visible(boma, meter_4, false)
				}
				KEN_FX_TIMER[ENTRY_ID] = 0;
			};
			if [*FIGHTER_STATUS_KIND_ENTRY, *FIGHTER_STATUS_KIND_WIN].contains(&status_kind) {
				println!("Reset!");
				KEN_SUPER[ENTRY_ID] = 0;
				KEN_FX_TIMER[ENTRY_ID] = 0;
				KEN_IS_EX[ENTRY_ID] = false;
			};
			if !AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_ALL) {
				HAS_ADDED[ENTRY_ID] = false;
			};
			if AttackModule::is_infliction(boma, *COLLISION_KIND_MASK_ALL) && status_kind != *FIGHTER_STATUS_KIND_CATCH_ATTACK && !HAS_ADDED[ENTRY_ID] {
				KEN_SUPER[ENTRY_ID] += 3;
				println!("attacks! {} ", KEN_SUPER[ENTRY_ID]);
				HAS_ADDED[ENTRY_ID] = true;
			};
			if KEN_SUPER[ENTRY_ID] >= KEN_MAX_METER{
				KEN_SUPER[ENTRY_ID] = KEN_MAX_METER;
			};
			if status_kind == *FIGHTER_RYU_STATUS_KIND_SPECIAL_LW_STEP_F || status_kind == *FIGHTER_RYU_STATUS_KIND_SPECIAL_LW_STEP_B {
				if EX_DOWNB[ENTRY_ID] {
					if MotionModule::frame(boma) < 2.0 {
						HitModule::set_whole(boma, smash::app::HitStatus(*HIT_STATUS_XLU), 0);
					};
					if MotionModule::frame(boma) > 3.0 {
						HitModule::set_whole(boma, smash::app::HitStatus(*HIT_STATUS_NORMAL), 0);
					};
					if MotionModule::frame(boma) > 10.0 {
						CancelModule::enable_cancel(boma);
					};
				};
			} else {
				EX_DOWNB[ENTRY_ID] = false;
			};
			if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_LW {
				let mut stick_x = ControlModule::get_stick_x(boma) ;
				stick_x *= PostureModule::lr(boma);
				let is_eligble_meter = (4..25).contains(&(MotionModule::frame(boma) as i32)) && KEN_SUPER[ENTRY_ID] >= meter_half as i32;
				
				if MotionModule::frame(boma) > 25.0 || is_eligble_meter {
					if stick_x >= 0.665 {
						if is_eligble_meter {
							KEN_SUPER[ENTRY_ID] -= meter_half as i32;
							println!("meter spent! {} ", KEN_SUPER[ENTRY_ID]);
							EX_DOWNB[ENTRY_ID] = true;
						}; 
						StatusModule::change_status_request_from_script(boma, *FIGHTER_RYU_STATUS_KIND_SPECIAL_LW_STEP_F, true);
					} else if stick_x <= -0.665 {
						if is_eligble_meter {
							KEN_SUPER[ENTRY_ID] -= meter_half as i32;
							println!("meter spent! {} ", KEN_SUPER[ENTRY_ID]);
							EX_DOWNB[ENTRY_ID] = true;
						};
						StatusModule::change_status_request_from_script(boma, *FIGHTER_RYU_STATUS_KIND_SPECIAL_LW_STEP_B, true);
					};
				};
			};
			if motion_kind == hash40("attack_near_w") && AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT) && !is_hitlag(boma) {
				if (ControlModule::get_command_flag_cat(boma, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_HI3) != 0 {
					StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_HI3, true);
				} else if (ControlModule::get_command_flag_cat(boma, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_LW3) != 0 {
					StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_LW3, true);
				}else if (ControlModule::get_command_flag_cat(boma, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_S3) != 0 {
					StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_DASH, true);
				};
			};
			if [hash40("attack_s3_s_s"), hash40("attack_s3_s_w"), hash40("attack_lw3_s"), hash40("attack_hi3_s")].contains(&motion_kind) && AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_ALL) && !is_hitlag(boma) && KEN_SUPER[ENTRY_ID] >= meter_half as i32 && cancel_frame-frame > 9.0{
				let mut stick_x = ControlModule::get_stick_x(boma) ;
				stick_x *= PostureModule::lr(boma);
				if stick_x >= 0.665 && (ControlModule::get_command_flag_cat(boma, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_DASH) != 0 {
					KEN_SUPER[ENTRY_ID] -= meter_half as i32;
					println!("meter spent! {} ", KEN_SUPER[ENTRY_ID]);
					EX_DOWNB[ENTRY_ID] = true;
					StatusModule::change_status_request_from_script(boma, *FIGHTER_RYU_STATUS_KIND_SPECIAL_LW_STEP_F, true);
				} else if stick_x <= -0.665 {
					println!("meter spent! {} ", KEN_SUPER[ENTRY_ID]);
					KEN_SUPER[ENTRY_ID] -= meter_half as i32;
					EX_DOWNB[ENTRY_ID] = true;
					StatusModule::change_status_request_from_script(boma, *FIGHTER_RYU_STATUS_KIND_SPECIAL_LW_STEP_B, true);
				};
			};

			//EX
			if KEN_SUPER[ENTRY_ID] >= meter_half as i32 && !AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_ALL) {
				if [*FIGHTER_STATUS_KIND_SPECIAL_HI].contains(&status_kind) {
					if [hash40("special_hi"), hash40("special_air_hi")].contains(&motion_kind) && MotionModule::frame(boma) < 5.0 {
						if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_ATTACK) && ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_SPECIAL) {
							KEN_SUPER[ENTRY_ID] -= meter_half as i32;
							println!("meter spent! {} ", KEN_SUPER[ENTRY_ID]);
							MotionModule::change_motion(boma, Hash40::new("special_hi_ex"), -1.0, 1.0, false, 0.0, false, false);
						};
					};
				};
			};

			//Focus
			if [*FIGHTER_STATUS_KIND_SPECIAL_HI, *FIGHTER_RYU_STATUS_KIND_SPECIAL_HI_COMMAND].contains(&status_kind)  && MotionModule::frame(boma) < 8.0 && (ControlModule::get_command_flag_cat(boma, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_LW) != 0 && KEN_SUPER[ENTRY_ID] >= meter_half as i32  {
				StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_SPECIAL_LW, true);
			};
			if [*FIGHTER_STATUS_KIND_SPECIAL_N, *FIGHTER_RYU_STATUS_KIND_SPECIAL_N_COMMAND].contains(&status_kind) && MotionModule::frame(boma) < 14.0 && (ControlModule::get_command_flag_cat(boma, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_LW) != 0 && KEN_SUPER[ENTRY_ID] >= meter_half as i32  {
				StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_SPECIAL_LW, true);
			};
			if [*FIGHTER_STATUS_KIND_SPECIAL_S, *FIGHTER_RYU_STATUS_KIND_SPECIAL_S_COMMAND, *FIGHTER_RYU_STATUS_KIND_SPECIAL_S_LOOP, *FIGHTER_RYU_STATUS_KIND_SPECIAL_S_END].contains(&status_kind) && StatusModule::is_situation_changed(boma)  {
				StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_LANDING, true);
			};
		};
	};
}
#[acmd_script(
    agent = "ryu_shinkuhadoken",
    script =  "game_move",
    category = ACMD_GAME,
	low_priority)]
unsafe fn ryu_shinsu(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		frame(Frame=1)
		if(is_excute){
			PostureModule::set_scale(0.5, false)
			ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=0.8, Angle=366, KBG=100, FKB=90, BKB=0, Size=8.0, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.0, SDI=0.5, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=-5, Trip=0.0, Rehit=5, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=true, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_FIRE, Type=ATTACK_REGION_NONE)
			ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=0.8, Angle=366, KBG=100, FKB=75, BKB=0, Size=18.0, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.0, SDI=0.5, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=-5, Trip=0.0, Rehit=5, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=true, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_NONE, Type=ATTACK_REGION_NONE)
			AttackModule::set_add_reaction_frame(ID=0, Frames=10.0, Unk=false)
			AttackModule::set_add_reaction_frame(ID=1, Frames=10.0, Unk=false)
		}
		frame(Frame=90)
		if(is_excute){
			AttackModule::clear_all()
			ATTACK(ID=0, Part=1, Bone=hash40("top"), Damage=7.0, Angle=75, KBG=127, FKB=0, BKB=90, Size=16.0, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.5, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=-5, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_FIRE, Type=ATTACK_REGION_NONE)
		}
		wait(Frames=6)
		if(is_excute){
			AttackModule::clear_all()
		}
    });
}		
#[acmd_script(
    agent = "ken",
    script =  "game_attacknearw",
    category = ACMD_GAME,
	low_priority)]
unsafe fn ken_prox_ftilt(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		if(is_excute){
			WorkModule::on_flag(Flag=FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL)
		}
		frame(Frame=3)
		if(is_excute){
			ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=6.0, Angle=180, KBG=100, FKB=10, BKB=0, Size=3.2, X=0.0, Y=12.5, Z=9.0, X2=0.0, Y2=7.5, Z2=9.0, Hitlag=1.0, SDI=0.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_G, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_KEN_PUNCH, Type=ATTACK_REGION_PUNCH)
			ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=6.0, Angle=75, KBG=23, FKB=0, BKB=16, Size=3.2, X=0.0, Y=12.5, Z=9.0, X2=0.0, Y2=7.5, Z2=9.0, Hitlag=1.0, SDI=0.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_A, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_KEN_PUNCH, Type=ATTACK_REGION_PUNCH)
			AttackModule::set_add_reaction_frame(ID=0, Frames=9.0, Unk=false)
		}
		frame(Frame=5)
		if(is_excute){
			AttackModule::clear_all()
			WorkModule::off_flag(Flag=FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL)
		}
		frame(Frame=8)
		if(is_excute){
			WorkModule::on_flag(Flag=FIGHTER_RYU_STATUS_ATTACK_FLAG_HIT_CANCEL)
		}
		frame(Frame=25)
		if(is_excute){
			WorkModule::off_flag(Flag=FIGHTER_RYU_STATUS_ATTACK_FLAG_HIT_CANCEL)
		}
    });
}		
#[acmd_script(
    agent = "ken",
    script =  "game_specialhiex",
    category = ACMD_GAME,
	low_priority)]
unsafe fn ken_ex_shoryu(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		frame(Frame=6)
		if(is_excute){
			HitModule::set_status_all(smash::app::HitStatus(*HIT_STATUS_XLU), 0)
			WorkModule::on_flag(Flag=FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL)
			ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=12.0, Angle=48, KBG=100, FKB=77, BKB=0, Size=5.6, X=0.0, Y=14.5, Z=7.1, X2=0.0, Y2=12.5, Z2=8.1, Hitlag=1.0, SDI=0.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_fire"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KEN_PUNCH, Type=ATTACK_REGION_PUNCH)
			AttackModule::set_add_reaction_frame(ID=0, Frames=8.0, Unk=false)
		}
		frame(Frame=9)
		if(is_excute){
			ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=8.0, Angle=50, KBG=100, FKB=0, BKB=0, Size=3.6, X=0.0, Y=14.5, Z=7.1, X2=0.0, Y2=12.5, Z2=8.1, Hitlag=0.8, SDI=0.5, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_fire"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KEN_PUNCH, Type=ATTACK_REGION_PUNCH)
			WorkModule::off_flag(Flag=FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL)
			WorkModule::on_flag(Flag=FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_HI_FLAG_JUMP)
		}
		frame(Frame=13)
		if(is_excute){
			HitModule::set_status_all(smash::app::HitStatus(*HIT_STATUS_NORMAL), 0)
			AttackModule::clear_all()
		}
		frame(Frame=27)
		if(is_excute){
			ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=15.0, Angle=80, KBG=40, FKB=0, BKB=80, Size=6.6, X=0.0, Y=10.0, Z=7.6, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=2.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_fire"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KEN_PUNCH, Type=ATTACK_REGION_PUNCH)
		}
		frame(Frame=36)
		if(is_excute){
			sv_battle_object::notify_event_msc_cmd(0x2127e37c07u64, GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES)
			AttackModule::clear_all()
		}
    });
}
#[acmd_script(
    agent = "ken",
    script =  "effect_specialhiex",
    category = ACMD_EFFECT,
	low_priority)]
unsafe fn ken_ex_shoryu_eff(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		frame(Frame=4)
		if(is_excute){
			FOOT_EFFECT(hash40("sys_v_smoke_a"), hash40("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false)
			LAST_EFFECT_SET_RATE(0.7)
		}
		frame(Frame=5)
		if(is_excute){
			BURN_COLOR(2.47, 1.42, 0.05, 1.2)
		}
		frame(Frame=10)
		if(is_excute){
			BURN_COLOR(2.47, 1.42, 0.05, 1.2)
		}
		frame(Frame=15)
		if(is_excute){
			BURN_COLOR(2.47, 1.42, 0.05, 1.2)
		}
		frame(Frame=22)
		if(is_excute){
			LANDING_EFFECT(hash40("sys_down_smoke"), hash40("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false)
			BURN_COLOR(2.47, 1.42, 0.05, 1.2)
		}
		frame(Frame=27)
		if(is_excute){
			EFFECT_FOLLOW_NO_STOP(hash40("ken_syoryuken_fire"), hash40("handr"), 0, 0, 0, 0, 0, 0, 1, false)
			EffectModule::enable_sync_init_pos_last()
			BURN_COLOR(2.47, 1.42, 0.05, 1.2)
			rust {
				if PostureModule::lr(fighter.module_accessor) < 0.0 {
					macros::EFFECT_FOLLOW(fighter, Hash40::new("ken_syoryuken_firearc"), Hash40::new("trans"), 3, 2, 2, 5, 0, 5, 1, false);
            		macros::EFFECT_FOLLOW(fighter, Hash40::new("ken_syoryuken_firearc2"), Hash40::new("trans"), 3, 2, 2, 5, 0, 5, 1, false);
				} else {
					macros::EFFECT_FOLLOW(fighter, Hash40::new("ken_syoryuken_firearc"), Hash40::new("trans"), -3, 2, 2, 5, 0, -5, 1, false);
            		macros::EFFECT_FOLLOW(fighter, Hash40::new("ken_syoryuken_firearc2"), Hash40::new("trans"), -3, 2, 2, 5, 0, -5, 1, false);
            		EffectModule::enable_sync_init_pos_last(fighter.module_accessor);
				}
			}
		}
		frame(Frame=31)
		if(is_excute){
			EFFECT_DETACH_KIND(hash40("ken_syoryuken_firearc"), -1)
			BURN_COLOR_FRAME(20, 1, 1, 1, 0)
			BURN_COLOR_NORMAL()
		}
		frame(Frame=43)
		if(is_excute){
			EFFECT_OFF_KIND(hash40("ken_syoryuken_fire"), false, true)
		}
    });
}
#[acmd_script(
    agent = "ken",
    script =  "sound_specialhiex",
    category = ACMD_SOUND,
	low_priority)]
unsafe fn ken_ex_shoryu_snd(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		frame(Frame=2)
		if(is_excute){
			PLAY_SE(hash40("vc_ken_special_h01"))
		}
		frame(Frame=6)
		if(is_excute){
			PLAY_SE(hash40("se_ken_special_h03"))
		}
		frame(Frame=22)
		if(is_excute){
			PLAY_LANDING_SE(hash40("se_ken_landing03"))
		}
		frame(Frame=27)
		if(is_excute){
			PLAY_SE(hash40("se_ken_special_h01"))
		}
		frame(Frame=33)
		if(is_excute){
			PLAY_SE(hash40("se_ken_special_h02"))
		}
    });
}
pub fn install() {
    smashline::install_acmd_scripts!(
		ken_dsmash,
		ken_jab3,
		ken_ex_shoryu,
		ken_ex_shoryu_eff,
		ken_ex_shoryu_snd
    );
    smashline::install_agent_frame_callbacks!(supers);
}
