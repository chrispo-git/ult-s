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
		let frame = MotionModule::frame(boma);
		if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_N {
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
		
    }
}
#[fighter_frame( agent = FIGHTER_KIND_KIRBY )]
fn kirby_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        println!("It'sa me, Mario, wahoooooooo!");
        let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent); 
		let status_kind = smash::app::lua_bind::StatusModule::status_kind(boma);
		let frame = MotionModule::frame(boma);
		if status_kind == *FIGHTER_KIRBY_STATUS_KIND_PEACH_SPECIAL_N {
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
		
    }
}
#[acmd_script(
    agent = "peach",
    scripts =  ["game_specialn", "game_specialairn"],
    category = ACMD_GAME,
	low_priority)]
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
    category = ACMD_EFFECT,
	low_priority)]
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
#[acmd_script(
    agent = "kirby",
    scripts =  ["effect_peachspecialn", "effect_peachspecialairn"],
    category = ACMD_EFFECT,
	low_priority)]
unsafe fn kirby_peach_neutralb_eff(fighter: &mut L2CAgentBase) {
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
        peach_frame,
		kirby_frame
    );
	smashline::install_acmd_scripts!(
		peach_neutralb,
		peach_neutralb_eff,
		kirby_peach_neutralb_eff
	);
}
