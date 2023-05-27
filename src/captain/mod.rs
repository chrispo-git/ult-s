use smash::hash40;
use smash::phx::Hash40;
use smash::lib::lua_const::*;
use smash::app::*;
use smash::app::lua_bind::*;
use smash::lua2cpp::{L2CFighterCommon, L2CAgentBase};
use smashline::*;
use smash_script::*;
use smash::app::sv_animcmd::*;
use smash::lib::L2CValue;
use crate::util::*;

pub const CMD_CAT1: i32 = 0x20; 
static mut STATIC_MUT : [i32; 8] = [6; 8];

#[fighter_frame( agent = FIGHTER_KIND_CAPTAIN )]
fn captain_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent); 
		let status_kind = smash::app::lua_bind::StatusModule::status_kind(boma);
		let ENTRY_ID = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
		let motion_kind = MotionModule::motion_kind(boma);
		//Raptor Boost Cancel Pog
		if [hash40("attack_s3_s"), hash40("attack_s3_lw"), hash40("attack_s3_hi")].contains(&motion_kind) && (AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT) || AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_SHIELD)){
            if (ControlModule::get_command_flag_cat(boma, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_S) != 0{ 
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_SPECIAL_S, true);
            };
		};
		if ![*FIGHTER_STATUS_KIND_THROW, *FIGHTER_STATUS_KIND_SPECIAL_N, *FIGHTER_CAPTAIN_STATUS_KIND_SPECIAL_N_TURN].contains(&status_kind) &&  AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_ALL) {
			let cat1 = ControlModule::get_command_flag_cat(boma, 0);
			if (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_N) != 0 && !is_hitlag(boma){
				StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_SPECIAL_N, true);
			};
		};
    };
		//down b wall jump cancel
        if motion_kind == hash40("special_air_lw") {
            let cat = fighter.global_table[CMD_CAT1].get_int() as i32;
            if ((cat & *FIGHTER_PAD_CMD_CAT1_FLAG_WALL_JUMP_LEFT) != 0 && GroundModule::get_touch_flag(boma) == *GROUND_TOUCH_FLAG_LEFT as u64) || ((cat & *FIGHTER_PAD_CMD_CAT1_FLAG_WALL_JUMP_RIGHT) != 0 && GroundModule::get_touch_flag(boma) == *GROUND_TOUCH_FLAG_RIGHT as u64) {
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_WALL_JUMP, true);
            }
        }
    }
}
#[acmd_script(
    agent = "captain",
    script =  "effect_landingheavy",
    category = ACMD_EFFECT)]
unsafe fn captain_landing(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		frame(Frame=2)
		if(is_excute){
			LANDING_EFFECT(hash40("sys_landing_smoke"), hash40("top"), 0, 0, 0, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, true)
		}
    });
}	

#[acmd_script(
    agent = "captain",
    script =  "game_attackairhi",
    category = ACMD_GAME)]
unsafe fn captain_uair(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		if(is_excute){
			WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
		}
		frame(Frame=7)
		if(is_excute){
			ATTACK(ID=0, Part=0, Bone=hash40("legl"), Damage=10.0, Angle=66, KBG=96, FKB=0, BKB=20, Size=4.5, X=3.2, Y=2.1, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
			ATTACK(ID=1, Part=0, Bone=hash40("kneel"), Damage=9.0, Angle=66, KBG=96, FKB=0, BKB=20, Size=5.0, X=6.2, Y=0.9, Z=-0.4, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
		}
		wait(Frames=6)
		if(is_excute){
			ATTACK(ID=0, Part=0, Bone=hash40("legl"), Damage=10.0, Angle=30, KBG=80, FKB=0, BKB=12, Size=4.5, X=3.2, Y=2.1, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
			ATTACK(ID=1, Part=0, Bone=hash40("kneel"), Damage=9.0, Angle=30, KBG=80, FKB=0, BKB=12, Size=5.0, X=6.2, Y=0.9, Z=-0.4, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
		}
		wait(Frames=2)
		if(is_excute){
			AttackModule::clear_all()
		}
		frame(Frame=26)
		if(is_excute){
			WorkModule::off_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
		}
    });
}	
#[acmd_script(
    agent = "captain",
    script =  "effect_attackairhi",
    category = ACMD_EFFECT)]
unsafe fn captain_uair_eff(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		frame(Frame=7)
		if(is_excute){
			EFFECT_FOLLOW(hash40("sys_attack_arc_d"), hash40("top"), 0, 16, -0.5, 0, 30, 90, 1.3, true, 1.0)
		}
		frame(Frame=11)
		if(is_excute){
			EFFECT_FOLLOW(hash40("sys_attack_arc_d"), hash40("top"), 0, 16, -0.5, 0, 110, 90, 1.2, true, 1.5)
		}
    });
}	

#[acmd_script(
    agent = "captain",
    script =  "game_attack13",
    category = ACMD_GAME)]
unsafe fn captain_jab3(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		frame(Frame=6)
		if(is_excute){
			ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=5.0, Angle=361, KBG=100, FKB=0, BKB=70, Size=5.5, X=0.0, Y=11.0, Z=12.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=2.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_elec"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KNEE)
			ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=5.0, Angle=361, KBG=100, FKB=0, BKB=70, Size=3.8, X=0.0, Y=10.0, Z=6.5, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=2.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_elec"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KNEE)
		}
		wait(Frames=2)
		if(is_excute){
			AttackModule::clear_all()
		}
    });
}	
#[acmd_script(
    agent = "captain",
    script =  "effect_attack13",
    category = ACMD_EFFECT)]
unsafe fn captain_jab3_eff(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		frame(Frame=5)
		if(is_excute){
			FOOT_EFFECT(hash40("sys_turn_smoke"), hash40("top"), 0, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, true)
		}
		frame(Frame=6)
		if(is_excute){
			EFFECT_ALPHA(hash40("sys_attack_impact"), hash40("top"), 0, 11.5, 12.8, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 360, true, 0.5)
			LAST_EFFECT_SET_RATE(1.5)
		}
    });
}	

#[status_script(agent = "captain", status = FIGHTER_STATUS_KIND_SPECIAL_LW, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_PRE)]
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
    smashline::install_agent_frames!(captain_frame);
    smashline::install_acmd_scripts!(captain_landing);
    smashline::install_acmd_scripts!(
		captain_landing,
		captain_uair,
		captain_uair_eff,
		captain_jab3,
		captain_jab3_eff
	);
	install_status_scripts!(special_lw_pre);
}