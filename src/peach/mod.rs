use smash::hash40;
use smash::lib::lua_const::*;
use smash::app::lua_bind::*;
use smash::lua2cpp::*;
use smashline::*;
use smash_script::*;
use smash::phx::Hash40;
use smash::app::ArticleOperationTarget;
use smash::lib::L2CValue;
use smash::app::*;
use crate::util::*;
//0 - None
//1 - Punch (pummel)
//2 - Charge Forwards (fthrow)
//3 - Jump up (uthrow)
//4 - Jump up then spike (dthrow)
static mut SPECIAL_N_TYPE : [i32; 8] = [0; 8];


// A Once-Per-Fighter-Frame that only applies to Mario. Neat!
#[fighter_frame( agent = FIGHTER_KIND_PEACH )]
fn peach_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        println!("It'sa me, Mario, wahoooooooo!");
        let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent); 
		let status_kind = smash::app::lua_bind::StatusModule::status_kind(boma);
		let ENTRY_ID = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
		let motion_kind = MotionModule::motion_kind(boma);
		let frame = MotionModule::frame(boma);
		let stick_x = ControlModule::get_stick_x(boma) * PostureModule::lr(boma);
		let stick_y = ControlModule::get_stick_y(boma);
		if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_N {
			/*if frame >= 8.0 && frame <= 40.0 {
				if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_SPECIAL) {
					/*if stick_x >= 0.65 {
						SPECIAL_N_TYPE[ENTRY_ID] = 2;
					} else if stick_y >= 0.5 {
						SPECIAL_N_TYPE[ENTRY_ID] = 3;
					} else if stick_y <= -0.5 {
						SPECIAL_N_TYPE[ENTRY_ID] = 4;
					} else {
						SPECIAL_N_TYPE[ENTRY_ID] = 1;
					};*/
					StatusModule::change_status_request_from_script(boma, *FIGHTER_PEACH_STATUS_KIND_SPECIAL_N_HIT, true);
				};
			};*/
			if StatusModule::is_situation_changed(boma) {
				ArticleModule::remove_exist(boma, *FIGHTER_PEACH_GENERATE_ARTICLE_KINOPIO,smash::app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
			};
			if frame >= 40.0 {
				CancelModule::enable_cancel(boma);
			};
			if frame >= 36.0 {
				StatusModule::set_keep_situation_air(boma, false);
			} else {
				StatusModule::set_keep_situation_air(boma, true);
			};
		};
		if status_kind == *FIGHTER_PEACH_STATUS_KIND_SPECIAL_N_HIT {
			/*if frame < 2.0 {
				if SPECIAL_N_TYPE[ENTRY_ID] == 1 {
					ArticleModule::change_motion(boma, *FIGHTER_PEACH_GENERATE_ARTICLE_KINOPIO,smash::phx::Hash40::new("catch_attack"),false,0.0);
				};
				if SPECIAL_N_TYPE[ENTRY_ID] == 2 {
					ArticleModule::change_motion(boma, *FIGHTER_PEACH_GENERATE_ARTICLE_KINOPIO,smash::phx::Hash40::new("throw_f"),false,0.0);
				};
				if SPECIAL_N_TYPE[ENTRY_ID] == 3 {
					ArticleModule::change_motion(boma, *FIGHTER_PEACH_GENERATE_ARTICLE_KINOPIO,smash::phx::Hash40::new("throw_hi"),false,0.0);
				};
				if SPECIAL_N_TYPE[ENTRY_ID] == 4 {
					ArticleModule::change_motion(boma, *FIGHTER_PEACH_GENERATE_ARTICLE_KINOPIO,smash::phx::Hash40::new("throw_lw"),false,0.0);
				};
			};
			if SPECIAL_N_TYPE[ENTRY_ID] == 1 && frame >= 20.0 {
				ArticleModule::remove_exist(boma, *FIGHTER_PEACH_GENERATE_ARTICLE_KINOPIO,smash::app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
				CancelModule::enable_cancel(boma);
			};
			if SPECIAL_N_TYPE[ENTRY_ID] == 2 && frame >= 32.0 {
				ArticleModule::remove_exist(boma, *FIGHTER_PEACH_GENERATE_ARTICLE_KINOPIO,smash::app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
				CancelModule::enable_cancel(boma);
			};
			if SPECIAL_N_TYPE[ENTRY_ID] == 3 && frame >= 33.0 {
				ArticleModule::remove_exist(boma, *FIGHTER_PEACH_GENERATE_ARTICLE_KINOPIO,smash::app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
				CancelModule::enable_cancel(boma);
			};
			if SPECIAL_N_TYPE[ENTRY_ID] == 4 && frame >= 43.0 {
				ArticleModule::remove_exist(boma, *FIGHTER_PEACH_GENERATE_ARTICLE_KINOPIO,smash::app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
				CancelModule::enable_cancel(boma);
			};*/
		};
		/*if ![*FIGHTER_STATUS_KIND_SPECIAL_N, *FIGHTER_PEACH_STATUS_KIND_SPECIAL_N_HIT].contains(&status_kind) {
			SPECIAL_N_TYPE[ENTRY_ID] = 0;
		};*/
		
    }
}
#[acmd_script(
    agent = "peach",
    scripts =  ["game_specialn", "game_specialairn"],
    category = ACMD_GAME)]
unsafe fn peach_neutralb(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		if(is_excute){
			ArticleModule::generate_article(FIGHTER_PEACH_GENERATE_ARTICLE_KINOPIO, false, 0)
			ArticleModule::change_motion(FIGHTER_PEACH_GENERATE_ARTICLE_KINOPIO,smash::phx::Hash40::new("throw_f"),false,0.0)
		}
		frame(Frame=13)
		if(is_excute){
			ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=13.0, Angle=361, KBG=85, FKB=0, BKB=50, Size=7.5, X=0.0, Y=7.5, Z=3.34, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_NONE)
			HIT_NODE(hash40("top"), HIT_STATUS_INVINCIBLE)
		}
		frame(Frame=14)
		if(is_excute){
			ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=13.0, Angle=361, KBG=85, FKB=0, BKB=50, Size=7.5, X=0.0, Y=7.5, Z=5.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_NONE)
		}
		frame(Frame=15)
		if(is_excute){
			ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=13.0, Angle=361, KBG=85, FKB=0, BKB=50, Size=7.5, X=0.0, Y=7.5, Z=6.67, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_NONE)
		}
		frame(Frame=16)
		if(is_excute){
			ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=13.0, Angle=361, KBG=85, FKB=0, BKB=50, Size=7.5, X=0.0, Y=7.5, Z=7.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_NONE)
		}
		frame(Frame=22)
		if(is_excute){
			AttackModule::clear_all()
			HIT_NODE(hash40("top"), HIT_STATUS_OFF)
		}
		frame(Frame=36)
		if(is_excute){
			ArticleModule::remove_exist(FIGHTER_PEACH_GENERATE_ARTICLE_KINOPIO,smash::app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL))
		}
    });
}	
#[acmd_script(
    agent = "peach",
    scripts =  ["effect_specialn", "effect_specialairn"],
    category = ACMD_EFFECT)]
unsafe fn peach_neutralb_eff(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		frame(Frame=11)
		if(is_excute){
			FOOT_EFFECT(hash40("sys_dash_smoke"), hash40("top"), -12, 0, 0, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, false)
		}
		frame(Frame=13)
		if(is_excute){
			EFFECT_FLIP(hash40("sys_attack_speedline"), hash40("sys_attack_speedline"), hash40("top"), -6, 5, -3, 0, 0, 0, 0.9, 0, 1, 0, 0, 0, 0, true, EF_FLIP_YZ)
		}
		frame(Frame=35)
		if(is_excute){
			EFFECT(hash40("sys_erace_smoke"), hash40("top"), 6, 5, 0, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, false)
		}
    });
}

pub fn install() {
    smashline::install_agent_frames!(
        peach_frame
    );
	smashline::install_acmd_scripts!(
		peach_neutralb,
		peach_neutralb_eff
	);
}
