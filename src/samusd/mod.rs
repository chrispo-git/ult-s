use smash::hash40;
use smash::lib::lua_const::*;
use smash::app::lua_bind::*;
use smash::lua2cpp::*;
use smashline::*;
use smash_script::*;
use smash::phx::Hash40;
use smash::app::ArticleOperationTarget;
use smash::lib::L2CValue;

use crate::util::*;

static mut HOLD : [i32; 8] = [0; 8];
static mut IS_HOLD : [bool; 8] = [false; 8];
static mut END : [bool; 8] = [false; 8];
static mut HOLD_MAX : i32 = 240;
static mut COOLDOWN : [i32; 8] = [0; 8];
static mut IS_ALLOWED : [bool; 8] = [true; 8];
static mut HOLD_COOLDOWN : i32 = 120;
static mut HANDS :  smash::phx::Vector3f =  smash::phx::Vector3f { x: 0.0, y: 0.0, z: 0.0 };


#[fighter_frame( agent = FIGHTER_KIND_SAMUSD )]
fn samusd_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent); 
		let status_kind = smash::app::lua_bind::StatusModule::status_kind(boma);
		let motion_kind = MotionModule::motion_kind(boma);
		let frame = MotionModule::frame(boma);
		let ENTRY_ID = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
		let situation_kind = StatusModule::situation_kind(boma);
		if smash::app::sv_information::is_ready_go() == false {
			HOLD[ENTRY_ID] = 0;
			IS_HOLD[ENTRY_ID] = false;
			COOLDOWN[ENTRY_ID] = 0;
			IS_ALLOWED[ENTRY_ID] = true;
			CAN_SIDEB[ENTRY_ID] = 0;
		};
		if IS_HOLD[ENTRY_ID] == true && ControlModule::check_button_off(boma, *CONTROL_PAD_BUTTON_SPECIAL) {
			IS_HOLD[ENTRY_ID] = false;
		};
		if IS_HOLD[ENTRY_ID] == true && ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_SPECIAL) {
			DamageModule::add_damage(boma, 0.075, 0);
			HOLD[ENTRY_ID] += 1;
		};
		if HOLD[ENTRY_ID] >= HOLD_MAX {
			IS_HOLD[ENTRY_ID] = false;
			HOLD[ENTRY_ID] = 0;
		};
		if END[ENTRY_ID] == true {
			COOLDOWN[ENTRY_ID] = HOLD_COOLDOWN;
			ArticleModule::remove_exist(boma, *FIGHTER_SAMUSD_GENERATE_ARTICLE_MISSILE,smash::app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
			END[ENTRY_ID] = false;
		};
		if COOLDOWN[ENTRY_ID] > 0 {
			COOLDOWN[ENTRY_ID] -= 1;
		};
		if ArticleModule::is_exist(boma, *WEAPON_KIND_SAMUSD_MISSILE) == false && IS_ALLOWED[ENTRY_ID] == false && COOLDOWN[ENTRY_ID] == 0 && END[ENTRY_ID] == false {
			END[ENTRY_ID] = true;
		};
		if COOLDOWN[ENTRY_ID] > 0 &&  COOLDOWN[ENTRY_ID] < 5{
				let m1: u32 = EffectModule::req_follow(boma, smash::phx::Hash40::new("sys_smash_flash"), smash::phx::Hash40::new("haver"), &HANDS, &HANDS, 0.325, true, 0, 0, 0, 0, 0, true, true) as u32;
				let m2: u32 = EffectModule::req_follow(boma, smash::phx::Hash40::new("sys_smash_flash"), smash::phx::Hash40::new("havel"), &HANDS, &HANDS, 0.325, true, 0, 0, 0, 0, 0, true, true) as u32;
				let m3: u32 = EffectModule::req_follow(boma, smash::phx::Hash40::new("sys_smash_flash"), smash::phx::Hash40::new("footr"), &HANDS, &HANDS, 0.325, true, 0, 0, 0, 0, 0, true, true) as u32;
				let m4: u32 = EffectModule::req_follow(boma, smash::phx::Hash40::new("sys_smash_flash"), smash::phx::Hash40::new("footl"), &HANDS, &HANDS, 0.325, true, 0, 0, 0, 0, 0, true, true) as u32;
				EffectModule::set_rgb(boma, m1, 0.0, 1.0, 3.8);
                EffectModule::set_rgb(boma, m2, 0.0, 1.0, 3.8);
				EffectModule::set_rgb(boma, m3, 0.0, 1.0, 3.8);
                EffectModule::set_rgb(boma, m4, 0.0,1.0, 3.8);
				COOLDOWN[ENTRY_ID] = 0;
				IS_ALLOWED[ENTRY_ID] = true;
		};
		if  IS_ALLOWED[ENTRY_ID] == false {
			CAN_SIDEB[ENTRY_ID] = 1;
		} else {
			CAN_SIDEB[ENTRY_ID] = 0;
		};
		if status_kind == *FIGHTER_SAMUS_STATUS_KIND_SPECIAL_S1A || status_kind == *FIGHTER_SAMUS_STATUS_KIND_SPECIAL_S1G {
			if MotionModule::frame(boma) >= 18.0 && MotionModule::frame(boma) <= 20.0 {
				IS_HOLD[ENTRY_ID] = true;
			};
			if MotionModule::frame(boma) > 25.0 {
				IS_ALLOWED[ENTRY_ID] = false;
				CancelModule::enable_cancel(boma);
			};
			MotionModule::set_rate(fighter.module_accessor, 1.5);
		};
		/*
		if IS_ALLOWED[ENTRY_ID] == false {
			if status_kind == *FIGHTER_SAMUS_STATUS_KIND_SPECIAL_S2A || status_kind == *FIGHTER_SAMUS_STATUS_KIND_SPECIAL_S1A {
					StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_FALL, true);
			};
			if status_kind == *FIGHTER_SAMUS_STATUS_KIND_SPECIAL_S2G || status_kind == *FIGHTER_SAMUS_STATUS_KIND_SPECIAL_S1G {
					StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_WAIT, true);
			};
		};*/
		if status_kind == *FIGHTER_SAMUS_STATUS_KIND_SPECIAL_S2A{
			StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_SAMUS_STATUS_KIND_SPECIAL_S1A, true);
		};
		if status_kind == *FIGHTER_SAMUS_STATUS_KIND_SPECIAL_S2G{
			StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_SAMUS_STATUS_KIND_SPECIAL_S1G, true);
		};
		if IS_HOLD[ENTRY_ID] == false {
			macros::STOP_SE(fighter, Hash40::new("se_samusd_win03_02"));
		} else if HOLD[ENTRY_ID] % 70 == 0{
			macros::PLAY_SE(fighter, Hash40::new("se_samusd_win03_02"));
		};
		if motion_kind == hash40("attack_12") {
			StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_WAIT, false);
		};
		if motion_kind == hash40("attack_lw4") {
			if frame >= 70.0 {
					MotionModule::change_motion(boma, Hash40::new("wait_4"), 0.0, 1.0, false, 0.0, false, false);
					StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_WAIT, false);
			};
		};
		if motion_kind == hash40("wait_4") {
			CancelModule::enable_cancel(boma);
		};
		//Teleport!
		if [hash40("special_lw"), hash40("special_air_lw")].contains(&motion_kind) {
				if frame > 12.0 && frame < 14.0 {
					macros::EFFECT_FOLLOW(fighter, Hash40::new("sys_smash_flash"), Hash40::new("hip"), -2, 0, 0, 0, 0, 0, 2.0, true);
					macros::PLAY_SE(fighter, Hash40::new("se_common_spirits_critical_l_tail"));
				};
				if StatusModule::is_situation_changed(boma) && situation_kind == *SITUATION_KIND_GROUND {
					StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_LANDING, true);
				};
				if frame > 5.0 && frame < 7.0 {
					macros::EFFECT_FOLLOW(fighter, Hash40::new("samusd_win3_aura"), Hash40::new("hip"), -2, 0, 0, 0, 0, 0, 2.5, true);
					macros::EFFECT_FOLLOW(fighter, Hash40::new("samusd_win3_aura"), Hash40::new("colonellm"), 2, 0, 0.5, 0, 0, 0, 2, true);
					macros::EFFECT_FOLLOW(fighter, Hash40::new("samusd_win3_aura"), Hash40::new("kneer"), 0, 0, -0.5, 0, 0, 0, 1.7, true);
					macros::EFFECT_FOLLOW(fighter, Hash40::new("samusd_win3_aura"), Hash40::new("footr"), 0, 0, 0, 0, 0, 0, 2.1, true);
					macros::EFFECT_FOLLOW(fighter, Hash40::new("samusd_win3_aura"), Hash40::new("armr"), 0, 0, 0, 0, 0, 0, 1.9, true);
					macros::EFFECT_FOLLOW(fighter, Hash40::new("samusd_win3_aura"), Hash40::new("handr"), 0, 0, 0, 0, 0, 0, 2, true);
					macros::EFFECT_FOLLOW(fighter, Hash40::new("samusd_win3_aura"), Hash40::new("colonells"), 2, 0, 0.5, 0, 0, 0, 2, true);
					macros::EFFECT_FOLLOW(fighter, Hash40::new("samusd_win3_aura"), Hash40::new("kneel"), 0, 0, -0.5, 0, 0, 0, 1.7, true);
					macros::EFFECT_FOLLOW(fighter, Hash40::new("samusd_win3_aura"), Hash40::new("footl"), 0, 0, 0, 0, 0, 0, 2.1, true);
					macros::EFFECT_FOLLOW(fighter, Hash40::new("samusd_win3_aura"), Hash40::new("arml"), 0, 0, 0, 0, 0, 0, 1.9, true);
					macros::EFFECT_FOLLOW(fighter, Hash40::new("samusd_win3_aura"), Hash40::new("handl"), 0, 0, 0, 0, 0, 0, 2, true);
				};
				if frame > 24.0 && frame < 47.0 {
					HitModule::set_whole(boma, smash::app::HitStatus(*HIT_STATUS_XLU), 0);
					VisibilityModule::set_whole(boma, false);
					JostleModule::set_status(boma, false);	
					if KineticModule::get_kinetic_type(boma) != *FIGHTER_KINETIC_TYPE_MOTION_AIR {
						KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_MOTION_AIR);
					};
					macros::SET_SPEED_EX(fighter, 0.27, 0.05, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_NONE);
				};
				if frame > 47.0 || (frame > 30.0 && ControlModule::check_button_off(boma, *CONTROL_PAD_BUTTON_SPECIAL)){
					KineticModule::clear_speed_all(boma);
					macros::EFFECT_FOLLOW(fighter, Hash40::new("sys_smash_flash"), Hash40::new("hip"), -2, 0, 0, 0, 0, 0, 2.0, true);
					HitModule::set_whole(boma, smash::app::HitStatus(*HIT_STATUS_NORMAL), 0);
					VisibilityModule::set_whole(boma, true);
					JostleModule::set_status(boma, true);	
					MotionModule::change_motion(boma, Hash40::new("special_air_lw_end"), 0.0, 1.0, false, 0.0, false, false);
				};
				if situation_kind == *SITUATION_KIND_AIR {
					StatusModule::set_keep_situation_air(boma, true);
				} else {
					StatusModule::set_situation_kind(boma, smash::app::SituationKind(*SITUATION_KIND_AIR), true);
				};
		};
		if [hash40("special_lw_end"), hash40("special_air_lw_end")].contains(&motion_kind) {
			StatusModule::set_keep_situation_air(boma, true);
			if KineticModule::get_kinetic_type(boma) == *FIGHTER_KINETIC_TYPE_MOTION_AIR {
				KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_FALL);
			};
			if frame < 2.0 {
				macros::PLAY_SE(fighter, Hash40::new("se_common_spirits_critical_m_tail"));
			};
			if StatusModule::is_situation_changed(boma){
				StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_LANDING, true);
			};
			if frame > 29.0 {
				macros::EFFECT_OFF_KIND(fighter, Hash40::new("samusd_win3_aura"), false, true);
				if ray_check_pos(boma, 0.0, -3.0, true) == 0 {
					StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_FALL, false);
				} else {
					StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_WAIT, false);
				};
			};
		};
		if [*FIGHTER_STATUS_KIND_JUMP, *FIGHTER_STATUS_KIND_JUMP_AERIAL].contains(&status_kind) {
			VisibilityModule::set_whole(boma, true);
		};
    }
}
#[weapon_frame( agent = WEAPON_KIND_SAMUSD_MISSILE )]
fn missile_frame(weapon: &mut L2CFighterBase) {
    unsafe {
        let otarget_id = WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER) as u32;
        let boma = smash::app::sv_battle_object::module_accessor(otarget_id);
		let ENTRY_ID = WorkModule::get_int(&mut *boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
        if smash::app::utility::get_kind(&mut *boma) == *FIGHTER_KIND_SAMUSD {
			if MotionModule::frame(weapon.module_accessor) >= 5.0 && MotionModule::motion_kind(weapon.module_accessor) == hash40("homing") {
				if IS_HOLD[ENTRY_ID] == true {
					MotionModule::set_rate(weapon.module_accessor, 0.001);
					println!("hold");
				} else {
					MotionModule::set_rate(weapon.module_accessor, 1.0);
				};
				if MotionModule::frame(weapon.module_accessor) >= 39.0 && (ModelModule::scale(weapon.module_accessor) > 0.001 || PostureModule::scale(weapon.module_accessor) > 0.001){
					END[ENTRY_ID] = true;
					ModelModule::set_scale(weapon.module_accessor, 0.0001);
					PostureModule::set_scale(weapon.module_accessor, 0.0001, false);
					WorkModule::set_int(weapon.module_accessor, 1, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
				};
			};
		};
    }
}
#[status_script(agent = "samusd_missile", status = WEAPON_SAMUS_MISSILE_STATUS_KIND_HOMING, condition = LUA_SCRIPT_STATUS_FUNC_EXEC_STATUS)]
unsafe fn missile_exec(fighter: &mut L2CFighterBase) -> L2CValue {
	0.into()
}	

#[acmd_script(
    agent = "samusd",
    script =  "game_attackairn",
    category = ACMD_GAME,
	low_priority)]
unsafe fn dsamus_nair(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		frame(Frame=3)
		if(is_excute){
			WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
		}
		frame(Frame=9)
		if(is_excute){
		ATTACK(ID=0, Part=0, Bone=hash40("hip"), Damage=8.5, Angle=60, KBG=72, FKB=0, BKB=64, Size=12.5, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_elec"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_FIRE, Type=ATTACK_REGION_MAGIC)
		}
		wait(Frames=2)
		if(is_excute){
			AttackModule::clear_all()
		}
		frame(Frame=36)
		if(is_excute){
			WorkModule::off_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
		}
    });
}	
#[acmd_script(
    agent = "samusd",
    script =  "effect_attackairn",
    category = ACMD_EFFECT,
	low_priority)]
unsafe fn dsamus_nair_eff(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		frame(Frame=3)
		if(is_excute){
			EFFECT_FOLLOW(hash40("samusd_win3_aura"), hash40("arml"), 0, 0, 0, 0, 0, 0, 1.9, true)
			EFFECT_FOLLOW(hash40("samusd_win3_aura"), hash40("handl"), 0, 0, 0, 0, 0, 0, 1.9, true)
			EFFECT_FOLLOW(hash40("samusd_win3_aura"), hash40("armr"), 0, 0, 0, 0, 0, 0, 1.9, true)
			EFFECT_FOLLOW(hash40("samusd_win3_aura"), hash40("handr"), 0, 0, 0, 0, 0, 0, 1.9, true)
			EFFECT_FOLLOW(hash40("samusd_win3_aura"), hash40("shoulderl"), 0, 0, 0, 0, 0, 0, 1.9, true)
			EFFECT_FOLLOW(hash40("samusd_win3_aura"), hash40("shoulderr"), 0, 0, 0, 0, 0, 0, 1.9, true)
			EFFECT_FOLLOW(hash40("samusd_win3_aura"), hash40("clavicler"), 0, 0, 0, 0, 0, 0, 1.9, true)
			EFFECT_FOLLOW(hash40("samusd_win3_aura"), hash40("claviclel"), 0, 0, 0, 0, 0, 0, 1.9, true)
			BURN_COLOR(0.26, 0.71, 1.5, 0.7)
		}
		frame(Frame=7)
		if(is_excute){
			EFFECT_FOLLOW(hash40("sys_attack_impact"), hash40("hip"), 0, 0, 0, 0, 0, 0, 2.3, true)
			LAST_EFFECT_SET_COLOR(3.0/255.0, 194.0/255.0, 252.0/255.0)
			LAST_EFFECT_SET_RATE(0.5)
			EFFECT_FOLLOW(hash40("sys_attack_impact"), hash40("hip"), 0, 0, 0, 0, 0, 0, 2.3, true)
			LAST_EFFECT_SET_COLOR(3.0/255.0, 194.0/255.0, 252.0/255.0)
			LAST_EFFECT_SET_RATE(0.5)
		}
		frame(Frame=37)
		if(is_excute){
			EFFECT_OFF_KIND(hash40("samusd_win3_aura"), false, true)
			BURN_COLOR_FRAME(20, 1, 1, 1, 0)
			BURN_COLOR_NORMAL()
		}
    });
}	
#[acmd_script(
    agent = "samusd",
    script =  "game_attackdash",
    category = ACMD_GAME,
	low_priority)]
unsafe fn dsamus_da(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		frame(Frame=8)
		if(is_excute){
			ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=1.0, Angle=0, KBG=100, FKB=20, BKB=0, Size=4.0, X=0.0, Y=9.0, Z=6.0, X2=0.0, Y2=9.0, Z2=5.0, Hitlag=0.8, SDI=0.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=3, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_G, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_elec"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_ELEC, Type=ATTACK_REGION_BODY)
			ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=1.0, Angle=367, KBG=100, FKB=20, BKB=0, Size=4.0, X=0.0, Y=9.0, Z=6.0, X2=0.0, Y2=9.0, Z2=5.0, Hitlag=0.8, SDI=0.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=3, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_A, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_elec"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_ELEC, Type=ATTACK_REGION_BODY)
		}
		frame(Frame=14)
		if(is_excute){
			AttackModule::clear_all()
		}
		frame(Frame=15)
		if(is_excute){
			ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=10.0, Angle=55, KBG=90, FKB=0, BKB=80, Size=6.5, X=0.0, Y=9.0, Z=3.0, X2=0.0, Y2=9.0, Z2=2.0, Hitlag=1.2, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_elec"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_BODY)
		}
		frame(Frame=18)
		if(is_excute){
			AttackModule::clear_all()
		}
    });
}
#[acmd_script(
    agent = "samusd",
    script =  "game_attackairlw",
    category = ACMD_GAME,
	low_priority)]
unsafe fn dsamus_dair(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		frame(Frame=5)
		if(is_excute){
			WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
		}
		frame(Frame=16)
		if(is_excute){
			ATTACK(ID=0, Part=0, Bone=hash40("armr"), Damage=13.0, Angle=361, KBG=90, FKB=0, BKB=6, Size=3.0, X=0.0, Y=6.5, Z=0.5, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_PUNCH)
			ATTACK(ID=1, Part=0, Bone=hash40("arml"), Damage=13.0, Angle=361, KBG=90, FKB=0, BKB=6, Size=3.0, X=0.0, Y=6.5, Z=0.5, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_PUNCH)
			ATTACK(ID=2, Part=0, Bone=hash40("armr"), Damage=13.0, Angle=270, KBG=90, FKB=0, BKB=6, Size=6.0, X=0.0, Y=6.5, Z=0.5, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_PUNCH)
			ATTACK(ID=3, Part=0, Bone=hash40("arml"), Damage=13.0, Angle=270, KBG=90, FKB=0, BKB=6, Size=6.0, X=0.0, Y=6.5, Z=0.5, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_PUNCH)
		}
		wait(Frames=2)
		if(is_excute){
			AttackModule::clear_all()
		}
		frame(Frame=43)
		if(is_excute){
			WorkModule::off_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
		}
    });
}		
#[acmd_script(
    agent = "samusd",
    script =  "effect_attackairlw",
    category = ACMD_EFFECT,
	low_priority)]
unsafe fn dsamus_dair_eff(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		if(is_excute){
			EFFECT_FOLLOW(hash40("samusd_win3_aura"), hash40("arml"), 0, 0, 0, 0, 0, 0, 1.9, true)
			EFFECT_FOLLOW(hash40("samusd_win3_aura"), hash40("handl"), 0, 0, 0, 0, 0, 0, 1.9, true)
			EFFECT_FOLLOW(hash40("samusd_win3_aura"), hash40("armr"), 0, 0, 0, 0, 0, 0, 1.9, true)
			EFFECT_FOLLOW(hash40("samusd_win3_aura"), hash40("handr"), 0, 0, 0, 0, 0, 0, 1.9, true)
			EFFECT_FOLLOW(hash40("samusd_win3_aura"), hash40("shoulderl"), 0, 0, 0, 0, 0, 0, 1.9, true)
			EFFECT_FOLLOW(hash40("samusd_win3_aura"), hash40("shoulderr"), 0, 0, 0, 0, 0, 0, 1.9, true)
			EFFECT_FOLLOW(hash40("samusd_win3_aura"), hash40("clavicler"), 0, 0, 0, 0, 0, 0, 1.9, true)
			EFFECT_FOLLOW(hash40("samusd_win3_aura"), hash40("claviclel"), 0, 0, 0, 0, 0, 0, 1.9, true)
		}
		frame(Frame=15)
		if(is_excute){
			EFFECT_FOLLOW(hash40("sys_attack_speedline"), hash40("top"), 0, 15, 0, 90, 0, 0, 1.7, true)
			LAST_EFFECT_SET_COLOR(3.0/255.0, 194.0/255.0, 252.0/255.0)
		}
		frame(Frame=37)
		if(is_excute){
			EFFECT_OFF_KIND(hash40("samusd_win3_aura"), false, true)
		}
    });
}
#[acmd_script(
    agent = "samusd",
    script =  "effect_attackdash",
    category = ACMD_EFFECT,
	low_priority)]
unsafe fn dsamus_da_eff(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		frame(Frame=3)
		if(is_excute){
			EFFECT_FOLLOW(hash40("samusd_win3_aura"), hash40("hip"), -2, 0, 0, 0, 0, 0, 2.5, true)
			EFFECT_FOLLOW(hash40("samusd_win3_aura"), hash40("colonellm"), 2, 0, 0.5, 0, 0, 0, 2, true)
			EFFECT_FOLLOW(hash40("samusd_win3_aura"), hash40("kneer"), 0, 0, -0.5, 0, 0, 0, 1.7, true)
			EFFECT_FOLLOW(hash40("samusd_win3_aura"), hash40("footr"), 0, 0, 0, 0, 0, 0, 2.1, true)
			EFFECT_FOLLOW(hash40("samusd_win3_aura"), hash40("armr"), 0, 0, 0, 0, 0, 0, 1.9, true)
			EFFECT_FOLLOW(hash40("samusd_win3_aura"), hash40("handr"), 0, 0, 0, 0, 0, 0, 2, true)
			EFFECT_FOLLOW(hash40("samusd_win3_aura"), hash40("colonells"), 2, 0, -0.5, 0, 0, 0, 2, true)
			EFFECT_FOLLOW(hash40("samusd_win3_aura"), hash40("kneel"), 0, 0, 0.5, 0, 0, 0, 1.7, true)
			EFFECT_FOLLOW(hash40("samusd_win3_aura"), hash40("footl"), 0, 0, 0, 0, 0, 0, 2.1, true)
			EFFECT_FOLLOW(hash40("samusd_win3_aura"), hash40("arml"), 0, 0, 0, 0, 0, 0, 1.9, true)
			EFFECT_FOLLOW(hash40("samusd_win3_aura"), hash40("handl"), 0, 0, 0, 0, 0, 0, 1.9, true)
			BURN_COLOR(0.26, 0.71, 1.5, 0.7)
		}
		frame(Frame=22)
		if(is_excute){
			EFFECT_OFF_KIND(hash40("samusd_win3_aura"), false, true)
			BURN_COLOR_FRAME(20, 1, 1, 1, 0)
			BURN_COLOR_NORMAL()
		}
    });
}
#[acmd_script(
    agent = "samusd",
    script =  "sound_attackdash",
    category = ACMD_SOUND,
	low_priority)]
unsafe fn dsamus_da_snd(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		frame(Frame=9)
		if(is_excute){
			PLAY_SE(hash40("se_samusd_attackdash"))
			PLAY_SE(hash40("se_samusd_appeal_s02"))
		}
    });
}
#[acmd_script(
    agent = "samusd",
    script =  "game_attacklw3",
    category = ACMD_GAME,
	low_priority)]
unsafe fn dsamus_dtilt(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		frame(Frame=6)
		if(is_excute){
			ATTACK(ID=0, Part=0, Bone=hash40("armr"), Damage=8.0, Angle=75, KBG=65, FKB=0, BKB=55, Size=3.8, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.65, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.4, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_elec"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_ELEC, Type=ATTACK_REGION_BOMB)
			ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=8.0, Angle=75, KBG=65, FKB=0, BKB=55, Size=7.2, X=0.0, Y=1.6, Z=14.4, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.65, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.4, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_elec"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_ELEC, Type=ATTACK_REGION_BOMB)
		}
		wait(Frames=3)
		if(is_excute){
			AttackModule::clear_all()
		}
    });
}	
#[acmd_script(
    agent = "samusd",
    script =  "game_attack11",
    category = ACMD_GAME,
	low_priority)]
unsafe fn dsamus_jab(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		frame(Frame=4)
		if(is_excute){
			ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=9.0, Angle=361, KBG=106, FKB=0, BKB=30, Size=6.0, X=0.0, Y=13.0, Z=6.0, X2=0.0, Y2=6.0, Z2=12.0, Hitlag=1.2, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=-3, Trip=-1.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_PUNCH)
		}
		wait(Frames=3)
		if(is_excute){
			AttackModule::clear_all()
		}
    });
}	
#[acmd_script(
    agent = "samusd",
    script =  "effect_attack11",
    category = ACMD_EFFECT,
	low_priority)]
unsafe fn dsamus_jab_eff(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		if(is_excute){
			EFFECT_FOLLOW(hash40("samusd_win3_aura"), hash40("arml"), 0, 0, 0, 0, 0, 0, 1.9, true)
			EFFECT_FOLLOW(hash40("samusd_win3_aura"), hash40("handl"), 0, 0, 0, 0, 0, 0, 1.9, true)
		}
		frame(Frame=3)
		if(is_excute){
			EFFECT_FOLLOW(hash40("sys_attack_speedline"), hash40("top"), 0, 25, 0, 70, 0, 0, 1.3, true)
			LAST_EFFECT_SET_COLOR(3.0/255.0, 194.0/255.0, 252.0/255.0)
		}
		frame(Frame=14)
		if(is_excute){
			EFFECT_OFF_KIND(hash40("samusd_win3_aura"), false, true)
		}
    });
}	
#[acmd_script(
    agent = "samusd",
    script =  "game_attackhi4",
    category = ACMD_GAME,
	low_priority)]
unsafe fn dsamus_usmash(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		frame(Frame=6)
		if(is_excute){
			WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD)
		}
		frame(Frame=8)
		if(is_excute){
			ATTACK(ID=0, Part=0, Bone=hash40("handr"), Damage=14.0, Angle=80, KBG=95, FKB=0, BKB=30, Size=5.3, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
			ATTACK(ID=1, Part=0, Bone=hash40("armr"), Damage=14.0, Angle=80, KBG=95, FKB=0, BKB=30, Size=4.3, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
			ATTACK(ID=2, Part=0, Bone=hash40("shoulderr"), Damage=14.0, Angle=80, KBG=95, FKB=0, BKB=30, Size=4.3, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
		}
		frame(Frame=13)
		if(is_excute){
			ATTACK(ID=0, Part=0, Bone=hash40("handr"), Damage=12.0, Angle=80, KBG=95, FKB=0, BKB=30, Size=5.3, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
			ATTACK(ID=1, Part=0, Bone=hash40("armr"), Damage=12.0, Angle=80, KBG=95, FKB=0, BKB=30, Size=4.3, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
			ATTACK(ID=2, Part=0, Bone=hash40("shoulderr"), Damage=12.0, Angle=80, KBG=95, FKB=0, BKB=30, Size=4.3, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
		}
		frame(Frame=18)
		if(is_excute){
			AttackModule::clear_all()
		}
    });
}	
#[acmd_script(
    agent = "samusd",
    script =  "effect_attackhi4",
    category = ACMD_EFFECT,
	low_priority)]
unsafe fn dsamus_usmash_eff(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		frame(Frame=8)
		if(is_excute){
			EFFECT_FOLLOW(hash40("sys_attack_arc_b"), hash40("top"), 1, 20, -2, 1.7, 10, 75, 1.675, true)
			LAST_EFFECT_SET_RATE(0.7)
			LAST_EFFECT_SET_COLOR( 0.1, 0.7, 3.0)
			LANDING_EFFECT(hash40("sys_v_smoke_a"), hash40("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true)
		}
		frame(Frame=35)
		if(is_excute){
			LANDING_EFFECT(hash40("sys_down_smoke"), hash40("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true)
		}
    });
}	
#[acmd_script(
    agent = "samusd",
    script =  "sound_attackhi4",
    category = ACMD_SOUND,
	low_priority)]
unsafe fn dsamus_usmash_snd(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		frame(Frame=7)
		if(is_excute){
			PLAY_SE(hash40("se_common_swing_09"))
		}
		frame(Frame=8)
		if(is_excute){
			PLAY_SE(hash40("se_samusd_swing_l"))
		}
		frame(Frame=35)
		if(is_excute){
			PLAY_SE(hash40("se_samusd_landing02"))
		}
    });
}	
#[acmd_script(
    agent = "samusd",
    script =  "game_attacklw4",
    category = ACMD_GAME,
	low_priority)]
unsafe fn dsamus_dsmash(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		FT_MOTION_RATE(FSM=1.4)
		frame(Frame=4)
		if(is_excute){
			WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD)
		}
		frame(Frame=8)
		FT_MOTION_RATE(FSM=1)
		frame(Frame=10)
		if(is_excute){
			ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=15.0, Angle=50, KBG=85, FKB=0, BKB=42, Size=7.2, X=0.0, Y=1.6, Z=18.4, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_elec"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_ELEC, Type=ATTACK_REGION_BOMB)
			ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=15.0, Angle=50, KBG=85, FKB=0, BKB=42, Size=7.2, X=0.0, Y=1.6, Z=-18.4, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_elec"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_ELEC, Type=ATTACK_REGION_BOMB)
			ATTACK(ID=2, Part=0, Bone=hash40("top"), Damage=15.0, Angle=50, KBG=85, FKB=0, BKB=42, Size=7.2, X=0.0, Y=1.6, Z=9.4, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_elec"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_ELEC, Type=ATTACK_REGION_BOMB)
			ATTACK(ID=3, Part=0, Bone=hash40("top"), Damage=15.0, Angle=50, KBG=85, FKB=0, BKB=42, Size=7.2, X=0.0, Y=1.6, Z=-9.4, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_elec"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_ELEC, Type=ATTACK_REGION_BOMB)
		}
		frame(Frame=13)
		if(is_excute){
			AttackModule::clear_all()
		}
    });
}	
#[acmd_script(
    agent = "samusd",
    script =  "effect_attacklw4",
    category = ACMD_EFFECT,
	low_priority)]
unsafe fn dsamus_dsmash_eff(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		if(is_excute){
			EFFECT_FOLLOW(hash40("samusd_win3_aura"), hash40("arml"), 0, 0, 0, 0, 0, 0, 1.9, true)
			EFFECT_FOLLOW(hash40("samusd_win3_aura"), hash40("shoulderl"), 0, 0, 0, 0, 0, 0, 1.9, true)
			EFFECT_FOLLOW(hash40("samusd_win3_aura"), hash40("claviclel"), 0, 0, 0, 0, 0, 0, 1.9, true)
			EFFECT_FOLLOW(hash40("samusd_win3_aura"), hash40("handl"), 0, 0, 0, 0, 0, 0, 1.9, true)
		}
		frame(Frame=10)
		if(is_excute){
			EFFECT(hash40("samusd_bomb_b"), hash40("top"), 0, 0, 17.4, 0, 0, 0, 0.7, 0, 0, 0, 0, 0, 0, true)
			EFFECT(hash40("samusd_bomb_b"), hash40("top"), 0, 0, -19.4, 0, 0, 0, 0.7, 0, 0, 0, 0, 0, 0, true)
			EFFECT(hash40("samusd_bomb_b"), hash40("top"), 0, 0, 8.4, 0, 0, 0, 0.7, 0, 0, 0, 0, 0, 0, true)
			EFFECT(hash40("samusd_bomb_b"), hash40("top"), 0, 0, -9.4, 0, 0, 0, 0.7, 0, 0, 0, 0, 0, 0, true)
		}
		frame(Frame=39)
		if(is_excute){
			EFFECT_OFF_KIND(hash40("samusd_win3_aura"), false, true)
		}
    });
}
#[acmd_script(
    agent = "samusd",
    script =  "sound_attacklw4",
    category = ACMD_SOUND,
	low_priority)]
unsafe fn dsamus_dsmash_snd(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		frame(Frame=5)
		if(is_excute){
			STOP_SE(hash40("se_common_smash_start"))
		}
		frame(Frame=10)
		if(is_excute){
			PLAY_SE(hash40("se_common_bomb_l"))
		}
    });
}
#[acmd_script(
    agent = "samusd",
    script =  "expression_attacklw4",
    category = ACMD_EXPRESSION,
	low_priority)]
unsafe fn dsamus_dsmash_expr(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		frame(Frame=10)
		if(is_excute){
			QUAKE(CAMERA_QUAKE_KIND_M)
		}
    });
}
#[acmd_script(
    agent = "samusd_missile",
    script =  "game_homing",
    category = ACMD_GAME,
	low_priority)]
unsafe fn dsamus_homing(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {	
		frame(Frame=9)
		if(is_excute){
			ATTACK(ID=0, Part=0, Bone=hash40("hip"), Damage=10.0, Angle=60, KBG=90, FKB=0, BKB=70, Size=9.0, X=0.0, Y=0.0, Z=0.0, X2=0.0, Y2=0.0, Z2=4.0, Hitlag=0.25, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_B, SetWeight=true, ShieldDamage=-12, Trip=0.0, Rehit=0, Reflectable=true, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_elec"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_FIRE, Type=ATTACK_REGION_BODY)
		}
		wait(Frames=6)
		if(is_excute){
			ATTACK(ID=0, Part=0, Bone=hash40("hip"), Damage=6.0, Angle=60, KBG=90, FKB=0, BKB=60, Size=9.0, X=0.0, Y=0.0, Z=0.0, X2=0.0, Y2=0.0, Z2=0.0, Hitlag=0.25, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_B, SetWeight=true, ShieldDamage=-5, Trip=0.0, Rehit=0, Reflectable=true, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_elec"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_FIRE, Type=ATTACK_REGION_BODY)
		}
		wait(Frames=6)
		if(is_excute){
			AttackModule::clear_all()
		}
    });
}	
#[acmd_script(
    agent = "samusd_missile",
    script =  "sound_homing",
    category = ACMD_SOUND,
	low_priority)]
unsafe fn sound_dsamus_homing(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {	
		frame(Frame=7)
		if(is_excute){
			PLAY_SE(hash40("se_samusd_special_n04"))
		}
    });
}		
#[acmd_script(
    agent = "samusd",
    script =  "game_attackairf",
    category = ACMD_GAME,
	low_priority)]
unsafe fn dsamus_fair(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		frame(Frame=9)
		if(is_excute){
			WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
		}
		frame(Frame=11)
		FT_MOTION_RATE(FSM=0.25)
		if(is_excute){
			ATTACK(ID=0, Part=0, Bone=hash40("armr"), Damage=11.0, Angle=42, KBG=90, FKB=0, BKB=42, Size=5.5, X=2.0, Y=0.0, Z=2.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_elec"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_FIRE, Type=ATTACK_REGION_BOMB)
			ATTACK(ID=1, Part=0, Bone=hash40("armr"), Damage=11.0, Angle=42, KBG=90, FKB=0, BKB=42, Size=8.5, X=9.0, Y=0.0, Z=2.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_elec"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_FIRE, Type=ATTACK_REGION_BOMB)
		}
		frame(Frame=27)
		FT_MOTION_RATE(FSM=1)
		if(is_excute){
			AttackModule::clear_all()
		}
		frame(Frame=45)
		if(is_excute){
			WorkModule::off_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
		}
    });
}	
#[acmd_script(
    agent = "samusd",
    script =  "effect_attackairf",
    category = ACMD_EFFECT,
	low_priority)]
unsafe fn dsamus_fair_eff(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		frame(Frame=11)
		if(is_excute){
			EFFECT(hash40("samusd_atk_bomb"), hash40("armr"), 14.387, -0.341, -0.169, 0, 0, 0, 1.3, 0, 0, 0, 0, 0, 0, true)
		}
    });
}		
#[acmd_script(
    agent = "samusd",
    script =  "sound_attackairf",
    category = ACMD_EFFECT,
	low_priority)]
unsafe fn dsamus_fair_snd(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		frame(Frame=10)
		if(is_excute){
			PLAY_SE(hash40("se_samusd_smash_s01"))
		}
    });
}	
#[acmd_script(
    agent = "samusd",
    scripts =  ["effect_special", "effect_specialair"],
    category = ACMD_EFFECT,
	low_priority)]
unsafe fn dsamus_special_eff(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {	
	
    });
}		
	
#[acmd_script(
    agent = "samusd_missile",
    script =  "effect_hburst",
    category = ACMD_EFFECT,
	low_priority)]
unsafe fn eff_dsamus_burst(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {	
	
    });
}		
#[acmd_script(
    agent = "samusd_missile",
    script =  "effect_homing",
    category = ACMD_EFFECT,
	low_priority)]
unsafe fn eff_dsamus_homing(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {	
		if(is_excute){
			ModelModule::set_alpha(0.0)
		}
		wait(frames=1)
		if(is_excute){
			ModelModule::set_alpha(0.33)
		}
		wait(frames=1)
		if(is_excute){
			ModelModule::set_alpha(0.66)
		}
		wait(frames=1)
		if(is_excute){
			ModelModule::set_alpha(1.0)
		}
		wait(Frames=36)
		if(is_excute){
			ModelModule::set_alpha(0.66)
		}
		wait(frames=1)
		if(is_excute){
			ModelModule::set_alpha(0.33)
		}
		wait(frames=1)
		if(is_excute){
			ModelModule::set_alpha(0.0)
		}
    });
}		
#[acmd_script(
    agent = "samusd",
    script =  "effect_speciallw",
    category = ACMD_EFFECT,
	low_priority)]
unsafe fn eff_dsamus_downb(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {	
	
    });
}	
#[acmd_script(
    agent = "samusd",
    script =  "expression_speciallw",
    category = ACMD_EXPRESSION,
	low_priority)]
unsafe fn expr_dsamus_downb(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {	
	
    });
}		
#[acmd_script(
    agent = "samusd",
    script =  "game_speciallw",
    category = ACMD_GAME,
	low_priority)]
unsafe fn dsamus_downb(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {	
	
    });
}	
#[acmd_script(
    agent = "samusd",
    scripts =  ["sound_squat", "sound_squatrv"],
    category = ACMD_SOUND,
	low_priority)]
unsafe fn dsamus_crouch_sound(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		
    });
}
#[acmd_script(
    agent = "samusd",
    script =  "expression_catchpull",
    category = ACMD_EXPRESSION,
	low_priority)]
unsafe fn dsamus_catchpull_expr(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		if(is_excute){
			VisibilityModule::set_int64(hash40("body") as i64, hash40("body_hide_gun") as i64)
			ArticleModule::remove_exist(FIGHTER_SAMUSD_GENERATE_ARTICLE_GUN,smash::app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL))
			ArticleModule::generate_article(FIGHTER_SAMUSD_GENERATE_ARTICLE_GUN, false, 0)
			ArticleModule::change_motion(FIGHTER_SAMUSD_GENERATE_ARTICLE_GUN,smash::phx::Hash40::new("catch_pull"),false,0.0)
		}
		frame(Frame=5)
		if(is_excute){
			VisibilityModule::set_int64(hash40("body") as i64, hash40("body_normal") as i64)
			ArticleModule::remove_exist(FIGHTER_SAMUSD_GENERATE_ARTICLE_GUN,smash::app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL))
		}
    });
}	
#[acmd_script(
    agent = "samusd",
    script =  "expression_catchwait",
    category = ACMD_EXPRESSION,
	low_priority)]
unsafe fn dsamus_catchwait_expr(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		if(is_excute){
			VisibilityModule::set_int64(hash40("body") as i64, hash40("body_normal") as i64)
			ArticleModule::remove_exist(FIGHTER_SAMUSD_GENERATE_ARTICLE_GUN,smash::app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL))
		}
    });
}	
#[acmd_script(
    agent = "samusd",
    script =  "game_catchattack",
    category = ACMD_GAME,
	low_priority)]
unsafe fn dsamus_catchattack(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		frame(Frame=3)
		if(is_excute){
			ATTACK(ID=0, Part=0, Bone=hash40("havel"), Damage=1.3, Angle=361, KBG=100, FKB=30, BKB=0, Size=8.2, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.4, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_elec"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_FIRE, Type=ATTACK_REGION_PUNCH)
			AttackModule::set_catch_only_all(true, false)
		}
		wait(Frames=1)
		if(is_excute){
			AttackModule::clear_all()
		}
    });
}
#[acmd_script(
    agent = "samusd",
    script =  "effect_catchattack",
    category = ACMD_EFFECT,
	low_priority)]
unsafe fn dsamus_catchattack_eff(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		frame(Frames=3)
		if(is_excute){
			EFFECT(hash40("samusd_atk_bomb"), hash40("throw"), 0, 0, 0, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, true)
		}
    });
}	
#[acmd_script(
    agent = "samusd",
    script =  "expression_catchattack",
    category = ACMD_EXPRESSION,
	low_priority)]
unsafe fn dsamus_catchattack_expr(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		if(is_excute){
			VisibilityModule::set_int64(hash40("body") as i64, hash40("body_normal") as i64)
			ArticleModule::remove_exist(FIGHTER_SAMUSD_GENERATE_ARTICLE_GUN,smash::app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL))
		}
    });
}	
#[acmd_script(
    agent = "samusd",
    script =  "expression_catchcut",
    category = ACMD_EXPRESSION,
	low_priority)]
unsafe fn dsamus_catchcut_expr(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		if(is_excute){
			VisibilityModule::set_int64(hash40("body") as i64, hash40("body_normal") as i64)
			ArticleModule::remove_exist(FIGHTER_SAMUSD_GENERATE_ARTICLE_GUN,smash::app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL))
		}
    });
}	
#[acmd_script(
    agent = "samusd",
    script =  "game_throwb",
    category = ACMD_GAME,
	low_priority)]
unsafe fn dsamus_bthrow(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		if(is_excute){
			ATTACK_ABS(Kind=FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, ID=0, Damage=10.0, Angle=40, KBG=55, FKB=0, BKB=60, Hitlag=0.0, Unk=1.0, FacingRestrict=ATTACK_LR_CHECK_F, Unk=0.0, Unk=true, Effect=hash40("collision_attr_elec_whip"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_NONE, Type=ATTACK_REGION_THROW)
			ATTACK_ABS(Kind=FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, ID=0, Damage=3.0, Angle=361, KBG=100, FKB=0, BKB=50, Hitlag=0.0, Unk=1.0, FacingRestrict=ATTACK_LR_CHECK_F, Unk=0.0, Unk=true, Effect=hash40("collision_attr_elec_whip"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_NONE, Type=ATTACK_REGION_THROW)
		}
		frame(Frame=9)
		if(is_excute){
			REVERSE_LR()
			ATK_HIT_ABS(FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, hash40("throw"), WorkModule::get_int64(module_accessor,*FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT), WorkModule::get_int64(module_accessor,*FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP), WorkModule::get_int64(module_accessor,*FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO))
		}
    });
}
#[acmd_script(
    agent = "samusd",
    script =  "sound_throwb",
    category = ACMD_SOUND,
	low_priority)]
unsafe fn dsamus_bthrow_snd(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		frame(Frame=9)
		if(is_excute){
			PLAY_SE(hash40("se_common_throw_02"))
		}
    });
}
#[acmd_script(
    agent = "samusd",
    script =  "effect_throwb",
    category = ACMD_EFFECT,
	low_priority)]
unsafe fn dsamus_bthrow_eff(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		frame(Frame=9)
		if(is_excute){
			LANDING_EFFECT(hash40("sys_h_smoke_a"), hash40("top"), -3, 0, 0, 0, 180, 0, 1, 0, 0, 0, 0, 0, 0, false)
		}
    });
}
#[acmd_script(
    agent = "samusd",
    script =  "expression_throwb",
    category = ACMD_EXPRESSION,
	low_priority)]
unsafe fn dsamus_bthrow_expr(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		if(is_excute){
			VisibilityModule::set_int64(hash40("body") as i64, hash40("body_normal") as i64)
			ArticleModule::remove_exist(FIGHTER_SAMUSD_GENERATE_ARTICLE_GUN,smash::app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL))
		}
    });
}	
#[acmd_script(
    agent = "samusd",
    script =  "game_throwf",
    category = ACMD_GAME,
	low_priority)]
unsafe fn dsamus_fthrow(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		if(is_excute){
			ATTACK_ABS(Kind=FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, ID=0, Damage=2.0, Angle=361, KBG=100, FKB=10, BKB=0, Hitlag=0.0, Unk=1.0, FacingRestrict=ATTACK_LR_CHECK_F, Unk=0.0, Unk=true, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_NONE, Type=ATTACK_REGION_THROW)
			ATTACK_ABS(Kind=FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, ID=0, Damage=3.0, Angle=361, KBG=100, FKB=0, BKB=40, Hitlag=0.0, Unk=1.0, FacingRestrict=ATTACK_LR_CHECK_F, Unk=0.0, Unk=true, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_NONE, Type=ATTACK_REGION_THROW)
		}
		frame(Frame=23)
		if(is_excute){
			ATK_HIT_ABS(FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, hash40("throw"), WorkModule::get_int64(module_accessor,*FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT), WorkModule::get_int64(module_accessor,*FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP), WorkModule::get_int64(module_accessor,*FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO))
		}
		frame(Frame=24)
		if(is_excute){
			ATTACK(ID=0, Part=0, Bone=hash40("havel"), Damage=10.0, Angle=45, KBG=59, FKB=0, BKB=60, Size=8.0, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.2, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_elec"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
			ATTACK(ID=1, Part=0, Bone=hash40("arml"), Damage=10.0, Angle=45, KBG=59, FKB=0, BKB=60, Size=10.0, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.2, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_elec"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
		}
		frame(Frame=26)
		if(is_excute){
			AttackModule::clear_all()
		}
    });
}
#[acmd_script(
    agent = "samusd",
    script =  "sound_throwf",
    category = ACMD_SOUND,
	low_priority)]
unsafe fn dsamus_fthrow_snd(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		frame(Frame=3)
		if(is_excute){
			PLAY_SE(hash40("se_common_throw_01"))
		}
		frame(Frame=9)
		if(is_excute){
			PLAY_SE(hash40("se_common_throw_02"))
		}
    });
}
#[acmd_script(
    agent = "samusd",
    script =  "effect_throwf",
    category = ACMD_EFFECT,
	low_priority)]
unsafe fn dsamus_fthrow_eff(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		if(is_excute){
			EFFECT_FOLLOW(hash40("samusd_win3_aura"), hash40("arml"), 0, 0, 0, 0, 0, 0, 1.9, true)
			EFFECT_FOLLOW(hash40("samusd_win3_aura"), hash40("shoulderl"), 0, 0, 0, 0, 0, 0, 1.9, true)
			EFFECT_FOLLOW(hash40("samusd_win3_aura"), hash40("claviclel"), 0, 0, 0, 0, 0, 0, 1.9, true)
			EFFECT_FOLLOW(hash40("samusd_win3_aura"), hash40("handl"), 0, 0, 0, 0, 0, 0, 1.9, true)
		}
		frame(Frame=22)
		if(is_excute){
			EFFECT_FOLLOW(hash40("sys_attack_speedline"), hash40("top"), 0, 6.5, 0, 0, 180, 0, 2.0, true)
			LAST_EFFECT_SET_COLOR(3.0/255.0, 194.0/255.0, 252.0/255.0)
		}
		frame(Frame=24)
		if(is_excute){
			EFFECT_FOLLOW(hash40("sys_attack_impact"), hash40("handl"), 0, 0, 0, 0, 0, 0, 0.8, true)
			LAST_EFFECT_SET_COLOR(3.0/255.0, 194.0/255.0, 252.0/255.0)
		}
		frame(Frame=27)
		if(is_excute){
			EFFECT_OFF_KIND(hash40("sys_attack_speedline"), false, true)
		}
		frame(Frame=35)
		if(is_excute){
			EFFECT_OFF_KIND(hash40("samusd_win3_aura"), false, true)
		}
    });
}
#[acmd_script(
    agent = "samusd",
    script =  "expression_throwf",
    category = ACMD_EXPRESSION,
	low_priority)]
unsafe fn dsamus_fthrow_expr(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		if(is_excute){
			VisibilityModule::set_int64(hash40("body") as i64, hash40("body_normal") as i64)
			ArticleModule::remove_exist(FIGHTER_SAMUSD_GENERATE_ARTICLE_GUN,smash::app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL))
		}
    });
}	
#[acmd_script(
    agent = "samusd",
    script =  "game_throwlw",
    category = ACMD_GAME,
	low_priority)]
unsafe fn dsamus_dthrow(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		if(is_excute){
			ATTACK_ABS(Kind=FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, ID=0, Damage=7.0, Angle=78, KBG=72, FKB=0, BKB=70, Hitlag=0.0, Unk=1.0, FacingRestrict=ATTACK_LR_CHECK_F, Unk=0.0, Unk=true, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_NONE, Type=ATTACK_REGION_THROW)
			ATTACK_ABS(Kind=FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, ID=0, Damage=3.0, Angle=361, KBG=100, FKB=0, BKB=40, Hitlag=0.0, Unk=1.0, FacingRestrict=ATTACK_LR_CHECK_F, Unk=0.0, Unk=true, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_NONE, Type=ATTACK_REGION_THROW)
		}
		frame(Frame=15)
		if(is_excute){
			ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=3.0, Angle=361, KBG=100, FKB=0, BKB=0, Size=12.0, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.2, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_elec"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_ELEC, Type=ATTACK_REGION_KICK)
			AttackModule::set_catch_only_all(true, false)
			QUAKE(CAMERA_QUAKE_KIND_M)
		}
		frame(Frame=16)
		FT_MOTION_RATE(FSM=0.83)
		if(is_excute){
			ATK_HIT_ABS(FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, hash40("throw"), WorkModule::get_int64(module_accessor,*FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT), WorkModule::get_int64(module_accessor,*FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP), WorkModule::get_int64(module_accessor,*FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO))
			AttackModule::clear_all()
		}
    });
}
#[acmd_script(
    agent = "samusd",
    script =  "sound_throwlw",
    category = ACMD_SOUND,
	low_priority)]
unsafe fn dsamus_dthrow_snd(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		frame(Frame=3)
		if(is_excute){
			PLAY_SE(hash40("se_common_throw_01"))
		}
		frame(Frame=15)
		if(is_excute){
			PLAY_SE(hash40("se_common_bomb_l"))
		}
    });
}
#[acmd_script(
    agent = "samusd",
    script =  "effect_throwlw",
    category = ACMD_EFFECT,
	low_priority)]
unsafe fn dsamus_dthrow_eff(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		if(is_excute){
			EFFECT_FOLLOW(hash40("samusd_win3_aura"), hash40("arml"), 0, 0, 0, 0, 0, 0, 1.9, true)
			EFFECT_FOLLOW(hash40("samusd_win3_aura"), hash40("shoulderl"), 0, 0, 0, 0, 0, 0, 1.9, true)
			EFFECT_FOLLOW(hash40("samusd_win3_aura"), hash40("claviclel"), 0, 0, 0, 0, 0, 0, 1.9, true)
			EFFECT_FOLLOW(hash40("samusd_win3_aura"), hash40("handl"), 0, 0, 0, 0, 0, 0, 1.9, true)
			BURN_COLOR(0.26, 0.71, 1.5, 0.7)
		}
		frame(Frame=15)
		if(is_excute){
			BURN_COLOR(0.26, 0.71, 1.5, 0.7)
			EFFECT(hash40("samusd_bomb_b"), hash40("top"), 0, 0, 0, 0, 0, 0, 0.75, 0, 0, 0, 0, 0, 0, true)
		}
		frame(Frame=43)
		if(is_excute){
			EFFECT_OFF_KIND(hash40("samusd_win3_aura"), false, true)
		}
    });
}
#[acmd_script(
    agent = "samusd",
    script =  "expression_throwlw",
    category = ACMD_EXPRESSION,
	low_priority)]
unsafe fn dsamus_dthrow_expr(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		if(is_excute){
			VisibilityModule::set_int64(hash40("body") as i64, hash40("body_normal") as i64)
			ArticleModule::remove_exist(FIGHTER_SAMUSD_GENERATE_ARTICLE_GUN,smash::app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL))
		}
    });
}	
#[status_script(agent = "samusd", status = FIGHTER_SAMUS_STATUS_KIND_SPECIAL_AIR_LW, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_PRE)]
unsafe fn special_air_lw_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(
        fighter.module_accessor,
        smash::app::SituationKind(*SITUATION_KIND_NONE),
        *FIGHTER_KINETIC_TYPE_NONE,
        *GROUND_CORRECT_KIND_KEEP as u32,
		smash::app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE),
        true,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT,
        0
    );

    FighterStatusModuleImpl::set_fighter_status_data(
        fighter.module_accessor,
        false,
        *FIGHTER_TREADED_KIND_NO_REAC,
        false,
        false,
        false,
        (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_LW | *FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_AIR_LASSO | *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON) as u64,
        *FIGHTER_STATUS_ATTR_START_TURN as u32,
        *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_LW as u32,
        0
    );
    0.into()
}	
#[status_script(agent = "samusd", status = FIGHTER_SAMUS_STATUS_KIND_SPECIAL_GROUND_LW, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_PRE)]
unsafe fn special_lw_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(
        fighter.module_accessor,
        smash::app::SituationKind(*SITUATION_KIND_NONE),
        *FIGHTER_KINETIC_TYPE_NONE,
        *GROUND_CORRECT_KIND_KEEP as u32,
		smash::app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE),
        true,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT,
        0
    );

    FighterStatusModuleImpl::set_fighter_status_data(
        fighter.module_accessor,
        false,
        *FIGHTER_TREADED_KIND_NO_REAC,
        false,
        false,
        false,
        (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_LW | *FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_AIR_LASSO | *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON) as u64,
        *FIGHTER_STATUS_ATTR_START_TURN as u32,
        *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_LW as u32,
        0
    );
    0.into()
}	
pub fn install() {
    smashline::install_acmd_scripts!(
        dsamus_jab,
		dsamus_jab_eff,
		dsamus_nair,
		dsamus_nair_eff,
		dsamus_dtilt,
		dsamus_homing,
		eff_dsamus_homing,
		eff_dsamus_burst,
		sound_dsamus_homing,
		dsamus_fair,
		dsamus_fair_eff,
		dsamus_fair_snd,
		dsamus_da,
		dsamus_da_eff,
		dsamus_da_snd,
		dsamus_dair,
		dsamus_dair_eff,
		dsamus_downb,
		expr_dsamus_downb,
		eff_dsamus_downb,
		dsamus_dsmash,
		dsamus_dsmash_eff,
		dsamus_dsmash_snd,
		dsamus_dsmash_expr,
		dsamus_special_eff,
		dsamus_crouch_sound,
		dsamus_usmash, dsamus_usmash_eff, dsamus_usmash_snd,

		dsamus_catchpull_expr, dsamus_catchwait_expr, dsamus_catchattack_expr, dsamus_catchcut_expr,
		dsamus_catchattack, dsamus_catchattack_eff,
		dsamus_fthrow, dsamus_fthrow_expr, dsamus_fthrow_eff, dsamus_fthrow_snd,
		dsamus_dthrow, dsamus_dthrow_eff, dsamus_dthrow_expr, dsamus_dthrow_snd,
		dsamus_bthrow, dsamus_bthrow_expr, dsamus_bthrow_eff, dsamus_bthrow_snd
	);
    smashline::install_agent_frames!(
        samusd_frame,
		missile_frame  
    );
	smashline::install_status_scripts!(missile_exec, special_lw_pre, special_air_lw_pre);
}
