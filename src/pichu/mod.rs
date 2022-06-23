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
use smash::phx::Vector2f;
static mut LAG_INCREASE : [bool; 8] = [false; 8];
static mut RECHARGE_TIMER : [i32; 8] = [0; 8];
static RECHARGE_MAX : i32 = 90;


#[acmd_script(
    agent = "pichu",
    script =  "game_attacks3",
    category = ACMD_GAME)]
unsafe fn pichu_ftilt(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		frame(Frame=5)
		if(is_excute){
			FT_ADD_DAMAGE(SelfDamage=1.6)
			ATTACK(ID=0, Part=0, Bone=hash40("footl"), Damage=8.0, Angle=361, KBG=155, FKB=0, BKB=5, Size=3.5, X=0.0, Y=0.0, Z=0.0, X2=0.0, Y2=-4.5, Z2=0.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=1.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_elec"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_ELEC, Type=ATTACK_REGION_KICK)
		}
		frame(Frame=9)
		if(is_excute){
			ATTACK(ID=1, Part=0, Bone=hash40("footr"), Damage=8.0, Angle=361, KBG=155, FKB=0, BKB=5, Size=3.0, X=0.0, Y=0.0, Z=0.0, X2=0.0, Y2=-3.7, Z2=0.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=1.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_elec"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_ELEC, Type=ATTACK_REGION_KICK)
		}
		frame(Frame=10)
		if(is_excute){
			AttackModule::clear(ID=0, false)
		}
		frame(Frame=13)
		if(is_excute){
			AttackModule::clear_all()
		}
    });
}	
#[acmd_script(
    agent = "pichu",
    script =  "game_attackairlw",
    category = ACMD_GAME)]
unsafe fn pichu_dair(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		FT_MOTION_RATE(FSM=0.5)
		if(is_excute){
			WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
		}
		frame(Frame=10)
		FT_MOTION_RATE(FSM=1)
		frame(Frame=14)
		if(is_excute){
		FT_ADD_DAMAGE(SelfDamage=1.5)
			HIT_NODE(hash40("mimir1"), HIT_STATUS_XLU)
			HIT_NODE(hash40("mimil1"), HIT_STATUS_XLU)
			ATTACK(ID=0, Part=0, Bone=hash40("neck"), Damage=4.5, Angle=367, KBG=100, FKB=40, BKB=0, Size=4.5, X=4.5, Y=-1.3, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.5, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=2, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_elec"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_ELEC, Type=ATTACK_REGION_HEAD)
		}
		frame(Frame=18)
		if(is_excute){
			AttackModule::clear_all()
			ATTACK(ID=0, Part=0, Bone=hash40("neck"), Damage=6.0, Angle=27, KBG=140, FKB=0, BKB=20, Size=5.4, X=4.5, Y=0.5, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.5, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_elec"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_ELEC, Type=ATTACK_REGION_HEAD)
			ATTACK(ID=1, Part=0, Bone=hash40("hip"), Damage=6.0, Angle=27, KBG=140, FKB=0, BKB=20, Size=3.5, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.5, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_elec"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_ELEC, Type=ATTACK_REGION_HEAD)
		}
		frame(Frame=23)
		if(is_excute){
			AttackModule::clear_all()
			HIT_NODE(hash40("mimir1"), HIT_STATUS_NORMAL)
			HIT_NODE(hash40("mimil1"), HIT_STATUS_NORMAL)
		}
		frame(Frame=42)
		if(is_excute){
			WorkModule::off_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
		}
    });
}	
#[acmd_script(
    agent = "pichu",
    script =  "game_attackdash",
    category = ACMD_GAME)]
unsafe fn pichu_da(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		frame(Frame=6)
		if(is_excute){
			ATTACK(ID=0, Part=0, Bone=hash40("neck"), Damage=8.0, Angle=65, KBG=80, FKB=0, BKB=60, Size=4.6, X=3.0, Y=-1.5, Z=1.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_HEAD)
			ATTACK(ID=1, Part=0, Bone=hash40("hip"), Damage=8.0, Angle=65, KBG=80, FKB=0, BKB=60, Size=3.0, X=0.7, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_HEAD)
		}
		frame(Frame=10)
		if(is_excute){
			ATTACK(ID=0, Part=0, Bone=hash40("neck"), Damage=6.0, Angle=90, KBG=80, FKB=0, BKB=60, Size=3.0, X=3.0, Y=-1.5, Z=1.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_HEAD)
			ATTACK(ID=1, Part=0, Bone=hash40("hip"), Damage=6.0, Angle=90, KBG=80, FKB=0, BKB=60, Size=2.5, X=0.7, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_HEAD)
		}
		frame(Frame=16)
		if(is_excute){
			AttackModule::clear_all()
		}
    });
}	
#[acmd_script(
    agent = "pichu",
    script =  "game_attack11",
    category = ACMD_GAME)]
unsafe fn pichu_facade(fighter: &mut L2CAgentBase) {
		let lua_state = fighter.lua_state_agent;
		let ENTRY_ID = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
		let min_damage = 5.0;
		let max_damage = 13.0;
		let min_bonus = 2.0;
		let max_bonus = 13.0;
		let min_size = 5.0;
		let max_size = 8.0;
		let max_percent = 100.0;
		let min_shielddamage = -10;
		let max_shielddamage = 1;
		let mut curr_percent = DamageModule::damage(fighter.module_accessor, 0); 
		if curr_percent > max_percent {
			curr_percent = max_percent;
		};
		if RECHARGE_TIMER[ENTRY_ID] != 0 {
			curr_percent = 0.0;
		};
		let hitbox_dmg = ((curr_percent/max_percent)*(max_damage-min_damage))+min_damage;
		let hitbox_bonus = ((curr_percent/max_percent)*(max_bonus-min_bonus))+min_bonus;
		let new_hitlag = (curr_percent/max_percent)+0.5;
		let hitbox_size = ((curr_percent/max_percent)*(max_size-min_size))+min_size;
		let shielddamage = (((curr_percent/max_percent) as i32)*(max_shielddamage-min_shielddamage))+min_shielddamage;
		let mut sfx = *ATTACK_SOUND_LEVEL_S;
		if curr_percent > max_percent/3.0 {
			sfx = *ATTACK_SOUND_LEVEL_M;
		};
		if curr_percent > (max_percent/3.0)*2.0 {
			sfx = *ATTACK_SOUND_LEVEL_L;
		};
		macros::FT_MOTION_RATE(fighter, /*FSM*/ 1.5);
		frame(fighter.lua_state_agent, 3.0);
		if macros::is_excute(fighter) {
			macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ hitbox_dmg, /*Angle*/ 80, /*KBG*/ 40, /*FKB*/ 0, /*BKB*/ 60, /*Size*/ hitbox_size, /*X*/ 0.0, /*Y*/ 6.0, /*Z*/ -3.0, /*X2*/ Some(0.0), /*Y2*/ Some(6.0), /*Z2*/ Some(3.0), /*Hitlag*/ new_hitlag, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ shielddamage, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_elec"), /*SFXLevel*/ sfx, /*SFXType*/ *COLLISION_SOUND_ATTR_ELEC, /*Type*/ *ATTACK_REGION_KICK);
			AttackModule::set_add_reaction_frame(fighter.module_accessor, /*ID*/ 0, /*Frames*/ hitbox_bonus, /*Unk*/ false);
			RECHARGE_TIMER[ENTRY_ID] = RECHARGE_MAX;
			DamageModule::add_damage(fighter.module_accessor, 1.0, 0);
		};
		frame(fighter.lua_state_agent, 6.0);
		if macros::is_excute(fighter) {
			AttackModule::clear_all(fighter.module_accessor);
		};
}	
#[acmd_script(
    agent = "pichu",
    script =  "effect_attack11",
    category = ACMD_EFFECT)]
unsafe fn pichu_facade_eff(fighter: &mut L2CAgentBase) {
		let lua_state = fighter.lua_state_agent;
		let ENTRY_ID = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
		let min_size = 5.0*0.25;
		let max_size = 8.0*0.25;
		let max_percent = 130.0;
		let mut curr_percent = DamageModule::damage(fighter.module_accessor, 0); 
		if curr_percent > max_percent {
			curr_percent = max_percent;
		};
		if RECHARGE_TIMER[ENTRY_ID] != 0 {
			curr_percent = 0.0;
		};
		let eff_size = ((curr_percent/max_percent)*(max_size-min_size))+min_size;
		let mut sfx = 0;
		if curr_percent > (max_percent/3.0)*2.0 {
			sfx = *CAMERA_QUAKE_KIND_S;
		};
		frame(fighter.lua_state_agent, 2.0);
		if macros::is_excute(fighter) {
			macros::EFFECT_FOLLOW(fighter, Hash40::new("pichu_elec2"), Hash40::new("top"), 0, 6, 0, 0, 0, 0, eff_size, true);
			macros::EFFECT_FOLLOW(fighter, Hash40::new("pichu_elec2"), Hash40::new("top"), 0, 6, 0, 0, 0, 0, eff_size*1.2, true);
			if curr_percent > (max_percent/3.0)*2.0 {
				EffectModule::set_rgb_partial_last(fighter.module_accessor, 2.0, 2.0, 1.0);
			};
			macros::EFFECT_FOLLOW_NO_STOP(fighter, Hash40::new("pichu_cheek"), Hash40::new("head"), 0, 0, 0, 0, -90, -90, 1, true);
			if sfx != 0 {
				macros::QUAKE(fighter, sfx);
			};
		};
		frame(fighter.lua_state_agent, 6.0);
		if macros::is_excute(fighter) {
			if curr_percent <= max_percent/3.0 {
				macros::EFFECT_OFF_KIND(fighter, Hash40::new("pichu_elec2"), false, true);
				macros::EFFECT_OFF_KIND(fighter, Hash40::new("pichu_cheek"), false, true);
			};
		};
		frame(fighter.lua_state_agent, 8.0);
		if macros::is_excute(fighter) {
			macros::EFFECT_OFF_KIND(fighter, Hash40::new("pichu_elec2"), false, true);
			macros::EFFECT_OFF_KIND(fighter, Hash40::new("pichu_cheek"), false, true);
		};
}
#[acmd_script(
    agent = "pichu",
    script =  "sound_attack11",
    category = ACMD_SOUND)]
unsafe fn pichu_facade_snd(fighter: &mut L2CAgentBase) {
		let lua_state = fighter.lua_state_agent;
		let ENTRY_ID = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
		let min_size = 5.0*0.25;
		let max_size = 9.5*0.25;
		let max_percent = 130.0;
		let mut curr_percent = DamageModule::damage(fighter.module_accessor, 0); 
		if curr_percent > max_percent {
			curr_percent = max_percent;
		};
		if RECHARGE_TIMER[ENTRY_ID] != 0 {
			curr_percent = 0.0;
		};
		let mut sfx = Hash40::new("se_pichu_attackhard_s01");
		if curr_percent > max_percent/3.0 {
			sfx = Hash40::new("se_pichu_attackair_b01");
		};
		if curr_percent > (max_percent/3.0)*2.0 {
			sfx = Hash40::new("se_pichu_special_l03");
		};
		frame(fighter.lua_state_agent, 3.0);
		if macros::is_excute(fighter) {
			macros::PLAY_SEQUENCE(fighter, Hash40::new("seq_pichu_rnd_attack"));
			macros::PLAY_SE(fighter, sfx);
		};
}
#[fighter_frame( agent = FIGHTER_KIND_PICHU )]
fn pichu_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent); 
		let status_kind = smash::app::lua_bind::StatusModule::status_kind(boma);
		let motion_kind = MotionModule::motion_kind(boma);
		let ENTRY_ID = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
		let total_hitstun = WorkModule::get_float(boma, *FIGHTER_INSTANCE_WORK_ID_FLOAT_DAMAGE_REACTION_FRAME_LAST);
		let cancel_frame = FighterMotionModuleImpl::get_cancel_frame(boma,smash::phx::Hash40::new_raw(MotionModule::motion_kind(boma)),false) as f32;
		let frame = MotionModule::frame(boma);
		if [*FIGHTER_PIKACHU_STATUS_KIND_SPECIAL_HI_END].contains(&status_kind) {
			if ControlModule::check_button_off(boma, *CONTROL_PAD_BUTTON_SPECIAL) && ControlModule::check_button_off(boma, *CONTROL_PAD_BUTTON_GUARD) && ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_ATTACK) && StatusModule::situation_kind(boma) == *SITUATION_KIND_AIR {
				if GroundModule::ray_check(boma, &Vector2f{ x: PostureModule::pos_x(boma), y: PostureModule::pos_y(boma)}, &Vector2f{ x: 0.0, y: -1.0}, true) == 0 {
					StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_AIR, false);
					LAG_INCREASE[ENTRY_ID] = true;
				};
			};
		};
		if status_kind == *FIGHTER_STATUS_KIND_ATTACK_AIR && LAG_INCREASE[ENTRY_ID] == true {
			if cancel_frame-frame < 4.0 {
				StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_FALL_SPECIAL, true);
				LAG_INCREASE[ENTRY_ID] = false;
			};
		};
		if status_kind == *FIGHTER_STATUS_KIND_LANDING_ATTACK_AIR && LAG_INCREASE[ENTRY_ID] == true {
				StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL, true);
				LAG_INCREASE[ENTRY_ID] = false;
		};
		if RECHARGE_TIMER[ENTRY_ID] == 10 {
			smash::app::FighterUtil::flash_eye_info(boma);
			macros::EFFECT_FOLLOW_NO_STOP(fighter, Hash40::new("pichu_cheek"), Hash40::new("head"), 0, 0, 0, 0, -90, -90, 1, true);
		};
		if RECHARGE_TIMER[ENTRY_ID] == 1 {
			macros::EFFECT_OFF_KIND(fighter, Hash40::new("pichu_cheek"), false, true);
		};
		if RECHARGE_TIMER[ENTRY_ID] > 0 {
			RECHARGE_TIMER[ENTRY_ID] -= 1;
		};
		if ![*FIGHTER_STATUS_KIND_ATTACK_AIR, *FIGHTER_STATUS_KIND_LANDING_ATTACK_AIR, *FIGHTER_PIKACHU_STATUS_KIND_SPECIAL_HI_END].contains(&status_kind) {
			LAG_INCREASE[ENTRY_ID] = false;
		};
		if !smash::app::sv_information::is_ready_go() {
			RECHARGE_TIMER[ENTRY_ID] = 0;
		};
    }
}
		
pub fn install() {
    smashline::install_acmd_scripts!(
		pichu_ftilt,
		pichu_dair,
		pichu_da,
		pichu_facade,
		pichu_facade_eff,
		pichu_facade_snd
    );
    smashline::install_agent_frames!(
        pichu_frame
    );
}
