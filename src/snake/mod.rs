//Every time I make an edit to this mod I'm gonna leave a complaint about you still using ACMD-Skyline instead of Smashline until it's updated
use {
	crate::util::*,
	smash::{
		app::{
			ArticleOperationTarget,
			lua_bind::*,
		},
		hash40,
		lib::{
			L2CValue,
			lua_const::*,
		},
		lua2cpp::*,
		phx::Hash40
	},
	smash_script::*,
	smashline::*,
};

static mut STATIC_MUT : [i32; 8] = [6; 8];

#[acmd_script( agent = "snake", script = "game_attackdashlightthrow", category = ACMD_GAME, low_priority)]
unsafe fn snake_dash_attack_throw(fighter: &mut L2CAgentBase) 
{
	let lua_state = fighter.lua_state_agent;
	acmd!(lua_state, {
		frame(Frame=12)
		if(is_excute){
			StatusModule::change_status_request_from_script(*FIGHTER_STATUS_KIND_ITEM_THROW_DASH, false);
		}
	});
}

#[acmd_script( agent = "snake", script = "game_attacks4", category = ACMD_GAME, low_priority)]
unsafe fn snake_fsmash(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		frame(Frame=5)
		if(is_excute){
			ArticleModule::generate_article(FIGHTER_SNAKE_GENERATE_ARTICLE_RPG7, false, 0)
			ArticleModule::change_motion(FIGHTER_SNAKE_GENERATE_ARTICLE_RPG7,smash::phx::Hash40::new("start"),false,0.0)
		}
		FT_MOTION_RATE(FSM=0.3)
		wait(Frames=20)
		FT_MOTION_RATE(FSM=1)
		frame(Frame=39)
		if(is_excute){
			WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD)
		}
		frame(Frame=41)
		if(is_excute){
			ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=18.0, Angle=361, KBG=75, FKB=0, BKB=56, Size=3.0, X=0.0, Y=8.0, Z=8.0, X2=0.0, Y2=8.0, Z2=20.0, Hitlag=0.7, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=2, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_fire"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_BOMB)
			ATTACK(ID=1, Part=0, Bone=hash40("top"),Damage=10.0, Angle=361, KBG=75, FKB=0, BKB=56, Size=3.0, X=0.0, Y=8.0, Z=8.0, X2=0.0, Y2=8.0, Z2=60.0, Hitlag=0.4, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=-4, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_BOMB)
			ATTACK(ID=2, Part=0, Bone=hash40("top"), Damage=5.0, Angle=50, KBG=75, FKB=0, BKB=10, Size=3.0, X=0.0, Y=8.0, Z=8.0, X2=0.0, Y2=8.0, Z2=120.0, Hitlag=0.2, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=-10, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_GA_d, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_BOMB)
			ATTACK(ID=3, Part=0, Bone=hash40("top"), Damage=5.0, Angle=270, KBG=75, FKB=0, BKB=10, Size=3.0, X=0.0, Y=8.0, Z=8.0, X2=0.0, Y2=8.0, Z2=120.0, Hitlag=0.2, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=-10, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_BOMB)
		}
		wait(Frames=3)
		if(is_excute){
			AttackModule::clear_all()
		}
		frame(Frame=68)
		if(is_excute){
			ArticleModule::remove_exist(FIGHTER_SNAKE_GENERATE_ARTICLE_RPG7,smash::app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL))
		}
    });
}

#[acmd_script( agent = "snake", script = "effect_attacks4", category = ACMD_EFFECT, low_priority)]
unsafe fn snake_fsmash_eff(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		frame(Frame=5)
		if(is_excute){
			EFFECT(hash40("sys_smash_flash"), hash40("top"), -4, 13, 8, 0, 0, 0, 0.7, 0, 0, 0, 0, 0, 0, true)
		}
		frame(Frame=41)
		if(is_excute){
			/*EFFECT(hash40("sys_bomb_b"), hash40("top"), 0, 8, 9, 180, 0, 0, 0.4, 0, 0, 0, 0, 0, 0, true)
			LAST_EFFECT_SET_RATE(0.7)*/
			EFFECT(hash40("snake_remote_missile"), hash40("top"), 0, 8.2, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false)
			EFFECT(hash40("sys_smash_flash"), hash40("top"), 0, 8.2, 8, 0, 0, 0, 1.3, 0, 0, 0, 0, 0, 0, true)
		}
    });
}

#[fighter_frame( agent = FIGHTER_KIND_SNAKE )]
fn snake_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent); 
		let status_kind = smash::app::lua_bind::StatusModule::status_kind(boma);
		let motion_kind = MotionModule::motion_kind(boma);
		let frame = MotionModule::frame(boma);
		let stick_y = ControlModule::get_stick_y(boma);
		let ENTRY_ID = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
		if ItemModule::is_have_item(boma, 0) {
            WorkModule::unable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_THROW_DASH);
        }
		if motion_kind == hash40("attack_dash")
        && frame >= 12.0 {
            if ItemModule::is_have_item(boma, 0) {
				if !ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_ATTACK) {
					MotionModule::change_motion(boma, Hash40::new("attack_dash_item_light_throw"), 0.0, 1.0, false, 0.0, false, false);
					AttackModule::clear_all(boma);
				}
            }
        }
        if motion_kind == hash40("attack_dash_item_light_throw") {
            if fighter.global_table[0x16].get_i32() != *SITUATION_KIND_AIR
            && stick_y <= -0.6875 {
                if GroundModule::is_passable_ground(boma) {
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_PASS, true);
                }
            }
        }
		if WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_HIT_STOP_ATTACK_SUSPEND_FRAME) >= 1 {
			ArticleModule::set_visibility_whole(boma, *FIGHTER_SNAKE_GENERATE_ARTICLE_RPG7, false, smash::app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
		} 
		else {
			ArticleModule::set_visibility_whole(boma, *FIGHTER_SNAKE_GENERATE_ARTICLE_RPG7, true, smash::app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
		};
    }
}

pub fn install() {
	smashline::install_acmd_scripts!(snake_dash_attack_throw, snake_fsmash, snake_fsmash_eff);
	smashline::install_agent_frames!(snake_frame);
}