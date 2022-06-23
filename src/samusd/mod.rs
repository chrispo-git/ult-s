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
		let ENTRY_ID = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
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
			};
			MotionModule::set_rate(fighter.module_accessor, 1.5);
		};
		if IS_ALLOWED[ENTRY_ID] == false {
			if status_kind == *FIGHTER_SAMUS_STATUS_KIND_SPECIAL_S2A || status_kind == *FIGHTER_SAMUS_STATUS_KIND_SPECIAL_S1A {
					StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_SAMUS_STATUS_KIND_SPECIAL_N_JUMP_CANCEL, true);
			};
			if status_kind == *FIGHTER_SAMUS_STATUS_KIND_SPECIAL_S2G || status_kind == *FIGHTER_SAMUS_STATUS_KIND_SPECIAL_S1G {
					StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_WAIT, true);
			};
		};
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
    script =  "game_attack11",
    category = ACMD_GAME)]
unsafe fn dsamus_jab(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		frame(Frame=3)
		if(is_excute){
			ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=3.0, Angle=75, KBG=10, FKB=0, BKB=40, Size=2.0, X=0.0, Y=10.0, Z=6.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.8, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_PUNCH)
			ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=3.0, Angle=75, KBG=10, FKB=0, BKB=40, Size=2.3, X=0.0, Y=10.0, Z=8.8, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.8, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_PUNCH)
			ATTACK(ID=2, Part=0, Bone=hash40("top"), Damage=3.0, Angle=75, KBG=10, FKB=0, BKB=40, Size=2.5, X=0.0, Y=10.0, Z=12.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.8, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_PUNCH)
			AttackModule::set_add_reaction_frame(ID=0, Frames=3.0, Unk=false)
			AttackModule::set_add_reaction_frame(ID=1, Frames=3.0, Unk=false)
			AttackModule::set_add_reaction_frame(ID=2, Frames=3.0, Unk=false)
		}
		wait(Frames=2)
		if(is_excute){
			AttackModule::clear_all()
		}
		frame(Frame=6)
        if(is_excute){
            WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO)
		}
    });
}
#[acmd_script(
    agent = "samusd",
    script =  "game_attackairn",
    category = ACMD_GAME)]
unsafe fn dsamus_nair(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		frame(Frame=1)
		if(is_excute){
			FT_MOTION_RATE(FSM=0.625)
		}	
		frame(Frame=8)
		if(is_excute){
			FT_MOTION_RATE(FSM=1)
			WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
			ATTACK(ID=0, Part=0, Bone=hash40("kneer"), Damage=10.0, Angle=32, KBG=100, FKB=0, BKB=40, Size=6.0, X=5.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
			ATTACK(ID=1, Part=0, Bone=hash40("kneer"), Damage=10.0, Angle=32, KBG=100, FKB=0, BKB=40, Size=6.0, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
		}
		wait(Frames=4)
		if(is_excute){
			AttackModule::clear_all()
		}
		frame(Frame=14)
		if(is_excute){
			ATTACK(ID=0, Part=0, Bone=hash40("kneer"), Damage=9.0, Angle=45, KBG=100, FKB=0, BKB=40, Size=4.0, X=5.3, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
			ATTACK(ID=1, Part=0, Bone=hash40("kneer"), Damage=9.0, Angle=45, KBG=100, FKB=0, BKB=40, Size=4.0, X=0.0, Y=1.0, Z=1.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
		}
		wait(Frames=2)
		if(is_excute){
			ATTACK(ID=0, Part=0, Bone=hash40("kneer"), Damage=8.0, Angle=45, KBG=98, FKB=0, BKB=10, Size=3.6, X=5.4, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
			ATTACK(ID=1, Part=0, Bone=hash40("kneer"), Damage=8.0, Angle=45, KBG=98, FKB=0, BKB=10, Size=3.6, X=0.0, Y=1.0, Z=1.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
		}
		frame(Frame=23)
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
    agent = "samusd",
    script =  "game_attacklw3",
    category = ACMD_GAME)]
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
    agent = "samusd_missile",
    script =  "game_homing",
    category = ACMD_GAME)]
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
    category = ACMD_SOUND)]
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
    category = ACMD_GAME)]
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
    category = ACMD_EFFECT)]
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
    category = ACMD_EFFECT)]
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
    agent = "samusd_missile",
    script =  "effect_hburst",
    category = ACMD_EFFECT)]
unsafe fn eff_dsamus_burst(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {	
	
    });
}		
#[acmd_script(
    agent = "samusd_missile",
    script =  "effect_homing",
    category = ACMD_EFFECT)]
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
		
pub fn install() {
    smashline::install_acmd_scripts!(
        dsamus_jab,
		dsamus_nair,
		dsamus_dtilt,
		dsamus_homing,
		eff_dsamus_homing,
		eff_dsamus_burst,
		sound_dsamus_homing,
		dsamus_fair,
		dsamus_fair_eff,
		dsamus_fair_snd
    );
    smashline::install_agent_frames!(
        samusd_frame,
		missile_frame
    );
	smashline::install_status_scripts!(missile_exec);
}
