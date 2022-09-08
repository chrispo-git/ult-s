use smash::hash40;
use smash::lib::lua_const::*;
use smash::app::lua_bind::*;
use smash::lua2cpp::*;
use smashline::*;
use smash_script::*;
use smash::phx::Hash40;
use smash::app::ArticleOperationTarget;
use smash::lib::L2CValue;
static mut FIREBALL : [i32; 8] = [0; 8];
static NONE :  smash::phx::Vector3f =  smash::phx::Vector3f { x: 0.0, y: 0.0, z: 0.0 };


#[acmd_script(
    agent = "koopa",
    script =  "game_attackairlw",
    category = ACMD_GAME,
	low_priority)]
unsafe fn bowser_dair(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		frame(Frame=14)
		if(is_excute){
			WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
		}
		frame(Frame=17)
		if(is_excute){
			ATTACK(ID=0, Part=0, Bone=hash40("haver"), Damage=16.0, Angle=270, KBG=95, FKB=0, BKB=20, Size=8.5, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.5, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_A, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_PUNCH)
			ATTACK(ID=1, Part=0, Bone=hash40("haver"), Damage=16.0, Angle=361, KBG=95, FKB=0, BKB=20, Size=8.5, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.5, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_G_d, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_PUNCH)
			ATTACK(ID=2, Part=0, Bone=hash40("haver"), Damage=20.0, Angle=270, KBG=95, FKB=0, BKB=20, Size=8.5, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.5, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_G, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_PUNCH)
		}
		frame(Frames=20)
		if(is_excute){
			AttackModule::clear_all()
		}
		frame(Frames=22)
		FT_MOTION_RATE(FSM=2.0)
		wait(Frames=9)
		if(is_excute){
			WorkModule::off_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
		}
		frame(Frame=40)
		if(is_excute){
			CancelModule::enable_cancel()
		}
    });
}
#[acmd_script(
    agent = "koopa_breath",
    script =  "game_move",
    category = ACMD_GAME,
	low_priority)]
unsafe fn bowser_fireball(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		if(is_excute){
			ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=13.0, Angle=45, KBG=60, FKB=0, BKB=47, Size=4.5, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=true, Absorbable=true, Flinchless=false, DisableHitlag=false, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_fire"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_FIRE, Type=ATTACK_REGION_NONE)
			AttackModule::enable_safe_pos()
		}
    });
}
#[acmd_script(
    agent = "koopa",
    script =  "effect_attackairlw",
    category = ACMD_EFFECT,
	low_priority)]
unsafe fn bowser_eff_dair(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		frame(Frame=16)
		if(is_excute){
			EFFECT_FOLLOW_ALPHA(hash40("sys_attack_impact"), hash40("haver"), 0, 0, 0, 0, 0, 0, 2.0, true, 2)
		}
    });
}		
#[acmd_script(
    agent = "koopa",
    script =  "effect_landingairlw",
    category = ACMD_EFFECT,
	low_priority)]
unsafe fn bowser_eff_land_dair(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		if(is_excute){
			LANDING_EFFECT(hash40("sys_landing_smoke"), hash40("top"), 0, 0, 0, 0, 0, 0, 1.3, 0, 0, 0, 0, 0, 0, false)
		}
    });
}	
#[acmd_script(
    agent = "koopa",
    script =  "game_landingairlw",
    category = ACMD_GAME,
	low_priority)]
unsafe fn bowser_land_dair(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
}	
#[acmd_script(
    agent = "koopa",
    script =  "sound_landingairlw",
    category = ACMD_SOUND,
	low_priority)]
unsafe fn bowser_snd_land_dair(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		frame(Frame=1)
		if(is_excute){
			PLAY_LANDING_SE(hash40("se_koopa_landing02"))
		}
	});
}	
#[acmd_script(
    agent = "koopa",
    script =  "sound_attackairlw",
    category = ACMD_SOUND,
	low_priority)]
unsafe fn bowser_snd_dair(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		frame(Frame=14)
		if(is_excute){
			PLAY_SE(hash40("se_koopa_nailswing02"))
			PLAY_SE(hash40("vc_koopa_attack06"))
		}
		frame(Frame=16)
		if(is_excute){
			PLAY_SE(hash40("se_koopa_smash_h01"))
		}
    });
}		

#[fighter_frame( agent = FIGHTER_KIND_KIRBY )]
fn kirby_bowser_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
		let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);   
		let status_kind = smash::app::lua_bind::StatusModule::status_kind(boma);
		let ENTRY_ID = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize; 
		let frame = MotionModule::frame(boma);
		let end_frame = MotionModule::end_frame(boma);
		if [hash40("koopa_special_n")].contains(&MotionModule::motion_kind(boma)){
			if end_frame-frame < 5.0 {
				StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_WAIT, true);
			};
			if frame >= 19.0 {
				CancelModule::enable_cancel(boma);
			};
			MotionModule::set_rate(boma, 0.775);
		};
		if [hash40("koopa_special_air_n")].contains(&MotionModule::motion_kind(boma)){
			if end_frame-frame < 5.0 {
				StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_FALL, true);
			};
			if frame >= 19.0 {
				CancelModule::enable_cancel(boma);
			};
			MotionModule::set_rate(boma, 0.775);
		};
		if [hash40("koopa_special_n_end")].contains(&MotionModule::motion_kind(boma)){
			StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_WAIT, true);
		};
		if [hash40("koopa_special_air_n_end")].contains(&MotionModule::motion_kind(boma)){
				StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_FALL, true);
		};
		if ArticleModule::is_exist(boma, *FIGHTER_KOOPA_GENERATE_ARTICLE_BREATH) {
			FIREBALL[ENTRY_ID] += 1;
		} else {
			FIREBALL[ENTRY_ID] = 0;
		};
		macros::EFFECT_OFF_KIND(fighter, Hash40::new("koopa_breath_m_fire"), false, true);
	}
}		
#[fighter_frame( agent = FIGHTER_KIND_KOOPA )]
fn bowser_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
		let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);   
		let status_kind = smash::app::lua_bind::StatusModule::status_kind(boma);
		let ENTRY_ID = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize; 
		let frame = MotionModule::frame(boma);
		let end_frame = MotionModule::end_frame(boma);
        if [hash40("special_s_catch"), hash40("special_s_air_catch")].contains(&MotionModule::motion_kind(boma)){
			if MotionModule::frame(boma) < 6.0 {
				MotionModule::set_rate(boma, 0.6);
			} else {
				MotionModule::set_rate(boma, 1.0);
			};
		};
        if [hash40("attack_air_lw")].contains(&MotionModule::motion_kind(boma)){
			if PostureModule::lr(boma) == -1.0 {
				PostureModule::reverse_lr(boma);
				PostureModule::update_rot_y_lr(boma);
			};
		};
		if [hash40("special_n")].contains(&MotionModule::motion_kind(boma)){
			if end_frame-frame < 5.0 {
				StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_WAIT, true);
			};
			if frame >= 19.0 {
				CancelModule::enable_cancel(boma);
			};
			MotionModule::set_rate(boma, 0.775);
		};
		if [hash40("special_air_n")].contains(&MotionModule::motion_kind(boma)){
			if end_frame-frame < 5.0 {
				StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_FALL, true);
			};
			if frame >= 19.0 {
				CancelModule::enable_cancel(boma);
			};
			MotionModule::set_rate(boma, 0.775);
		};
		if [hash40("special_n_end")].contains(&MotionModule::motion_kind(boma)){
			StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_WAIT, true);
		};
		if [hash40("special_air_n_end")].contains(&MotionModule::motion_kind(boma)){
				StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_FALL, true);
		};
		if ArticleModule::is_exist(boma, *FIGHTER_KOOPA_GENERATE_ARTICLE_BREATH) {
			FIREBALL[ENTRY_ID] += 1;
		} else {
			FIREBALL[ENTRY_ID] = 0;
		};
		macros::EFFECT_OFF_KIND(fighter, Hash40::new("koopa_breath_m_fire"), false, true);
	}
}		
#[weapon_frame( agent = WEAPON_KIND_KOOPA_BREATH )]
pub fn fireball_frame(weapon : &mut L2CFighterBase) {
    unsafe {
        let otarget_id = WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER) as u32;
        let boma = smash::app::sv_battle_object::module_accessor(otarget_id);
		let ENTRY_ID = WorkModule::get_int(&mut *boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
		if FIREBALL[ENTRY_ID] % 14 == 0 {
			EffectModule::kill_kind(weapon.module_accessor, Hash40::new("koopa_breath_m_fire"), false, true);
			let f1: u32 = EffectModule::req_follow(weapon.module_accessor, smash::phx::Hash40::new("sys_fireflower_shot"), smash::phx::Hash40::new("top"), &NONE, &NONE, 0.8, true, 0, 0, 0, 0, 0, true, true) as u32;
			EffectModule::set_rgb(boma, f1, 1.5, 0.5, 0.5);
			EffectModule::req_follow(weapon.module_accessor, smash::phx::Hash40::new("koopa_breath_m_fire"), smash::phx::Hash40::new("top"), &NONE, &NONE, 0.4, true, 0, 0, 0, 0, 0, true, true) as u32;
		};
    }
}
		
pub fn install() {
    smashline::install_acmd_scripts!(
		bowser_dair,
		bowser_eff_dair,
		bowser_snd_dair,
		bowser_eff_land_dair,
		bowser_land_dair,
		bowser_snd_land_dair,
		bowser_fireball
    );
    smashline::install_agent_frames!(
        bowser_frame,
		fireball_frame,
		kirby_bowser_frame
    );
}
