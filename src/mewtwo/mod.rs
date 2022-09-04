use smash::hash40;
use smash::app::lua_bind::*;
use smash::lib::lua_const::*;
use smash::lua2cpp::*;
use smashline::*;
use smash_script::*;
use crate::util::*;
static mut ATTACK_AIR_WINDOW : [i32; 8] = [0; 8];
static mut MAX_ATTACK_AIR_WINDOW : i32 = 15;
static mut HAS_ATTACK_AIR: [bool; 8] = [false; 8];
static mut HAS_ALREADY_TELECANCEL: [bool; 8] = [false; 8];

#[fighter_frame( agent = FIGHTER_KIND_MEWTWO )]
fn mew2_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent); 
		let status_kind = smash::app::lua_bind::StatusModule::status_kind(boma);
		let frame = MotionModule::frame(boma);
		let ENTRY_ID = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
		let cancel_frame = FighterMotionModuleImpl::get_cancel_frame(boma,smash::phx::Hash40::new_raw(MotionModule::motion_kind(boma)),false) as f32;
		let motion_kind = MotionModule::motion_kind(boma);
		if (status_kind == *FIGHTER_STATUS_KIND_FALL_SPECIAL || status_kind == *FIGHTER_MEWTWO_STATUS_KIND_SPECIAL_HI_3 )&& !HAS_ALREADY_TELECANCEL[ENTRY_ID] {
			ATTACK_AIR_WINDOW[ENTRY_ID] += 1;
		} else {
			ATTACK_AIR_WINDOW[ENTRY_ID] = 0;
		};
		if ATTACK_AIR_WINDOW[ENTRY_ID] > 3 && ATTACK_AIR_WINDOW[ENTRY_ID] < MAX_ATTACK_AIR_WINDOW && StatusModule::situation_kind(boma) == *SITUATION_KIND_AIR{
			if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_ATTACK) {
				HAS_ATTACK_AIR[ENTRY_ID] = true;
				HAS_ALREADY_TELECANCEL[ENTRY_ID] = true;
				WorkModule::set_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT_MAX, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT);
				WorkModule::set_flag(boma, true, *FIGHTER_INSTANCE_WORK_ID_FLAG_DISABLE_ESCAPE_AIR);
				StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_AIR, true);
			};
		};
		if StatusModule::situation_kind(boma) != *SITUATION_KIND_AIR {
				HAS_ALREADY_TELECANCEL[ENTRY_ID] = false;
				HAS_ATTACK_AIR[ENTRY_ID] = false;
		};
		if [*FIGHTER_STATUS_KIND_DEAD, *FIGHTER_STATUS_KIND_LOSE, *FIGHTER_STATUS_KIND_WIN].contains(&status_kind) || smash::app::sv_information::is_ready_go() == false{
			HAS_ATTACK_AIR[ENTRY_ID] = false;
			ATTACK_AIR_WINDOW[ENTRY_ID] = 0;
		};
    }
}

#[acmd_script(
    agent = "mewtwo",
    script =  "game_dash",
    category = ACMD_GAME,
	low_priority)]
unsafe fn m2_dash(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		frame(Frame=1)
		if(is_excute){
			HIT_NODE(hash40("s_tail2"), HIT_STATUS_XLU)
			HIT_NODE(hash40("s_tail3"), HIT_STATUS_XLU)
			HIT_NODE(hash40("s_tail5"), HIT_STATUS_XLU)
			HIT_NODE(hash40("s_tail4"), HIT_STATUS_XLU)
			HIT_NODE(hash40("s_tail7"), HIT_STATUS_XLU)
		}
		frame(Frame=11)
		if(is_excute){
			HIT_NODE(hash40("s_tail2"), HIT_STATUS_NORMAL)
			HIT_NODE(hash40("s_tail5"), HIT_STATUS_NORMAL)
			HIT_NODE(hash40("s_tail3"), HIT_STATUS_NORMAL)
			HIT_NODE(hash40("s_tail4"), HIT_STATUS_NORMAL)
			HIT_NODE(hash40("s_tail7"), HIT_STATUS_NORMAL)
			WorkModule::enable_transition_term(FIGHTER_STATUS_TRANSITION_TERM_ID_DASH_TO_RUN)
		}
    });
}	
#[acmd_script(
    agent = "mewtwo",
    script =  "game_turndash",
    category = ACMD_GAME,
	low_priority)]
unsafe fn m2_turn_dash(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		frame(Frame=1)
		if(is_excute){
			HIT_NODE(hash40("s_tail2"), HIT_STATUS_XLU)
			HIT_NODE(hash40("s_tail3"), HIT_STATUS_XLU)
			HIT_NODE(hash40("s_tail5"), HIT_STATUS_XLU)
			HIT_NODE(hash40("s_tail4"), HIT_STATUS_XLU)
			HIT_NODE(hash40("s_tail7"), HIT_STATUS_XLU)
		}
		frame(Frame=3)
		if(is_excute){
			WorkModule::on_flag(Flag=FIGHTER_STATUS_DASH_FLAG_TURN_DASH)
		}
		frame(Frame=13)
		if(is_excute){
			HIT_NODE(hash40("s_tail2"), HIT_STATUS_NORMAL)
			HIT_NODE(hash40("s_tail3"), HIT_STATUS_NORMAL)
			HIT_NODE(hash40("s_tail5"), HIT_STATUS_NORMAL)
			HIT_NODE(hash40("s_tail4"), HIT_STATUS_NORMAL)
			HIT_NODE(hash40("s_tail7"), HIT_STATUS_NORMAL)
			WorkModule::enable_transition_term(FIGHTER_STATUS_TRANSITION_TERM_ID_DASH_TO_RUN)
		}
    });
}	
#[acmd_script(
    agent = "mewtwo",
    script =  "game_attackairb",
    category = ACMD_GAME,
	low_priority)]
unsafe fn m2_bair(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		frame(Frame=3)
		if(is_excute){
			HIT_NODE(hash40("s_tail2"), HIT_STATUS_XLU)
			HIT_NODE(hash40("s_tail3"), HIT_STATUS_XLU)
			HIT_NODE(hash40("s_tail5"), HIT_STATUS_XLU)
			HIT_NODE(hash40("s_tail4"), HIT_STATUS_XLU)
			HIT_NODE(hash40("s_tail7"), HIT_STATUS_XLU)
			WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
		}
		frame(Frame=13)
		if(is_excute){
			ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=13.0, Angle=361, KBG=101, FKB=0, BKB=20, Size=5.5, X=0.0, Y=12.0, Z=-7.7, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_B, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_TAIL)
			ATTACK(ID=1, Part=0, Bone=hash40("s_tail5"), Damage=12.0, Angle=361, KBG=90, FKB=0, BKB=20, Size=4.5, X=1.0, Y=0.0, Z=-2.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_B, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_TAIL)
			ATTACK(ID=2, Part=0, Bone=hash40("s_tail7"), Damage=11.0, Angle=361, KBG=90, FKB=0, BKB=20, Size=3.4, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_B, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_TAIL)
		}
		frame(Frame=18)
		if(is_excute){
			AttackModule::clear_all()
		}
		frame(Frame=38)
		if(is_excute){
			HIT_NODE(hash40("s_tail2"), HIT_STATUS_NORMAL)
			HIT_NODE(hash40("s_tail5"), HIT_STATUS_NORMAL)
			HIT_NODE(hash40("s_tail4"), HIT_STATUS_NORMAL)
			HIT_NODE(hash40("s_tail3"), HIT_STATUS_NORMAL)
			HIT_NODE(hash40("s_tail7"), HIT_STATUS_NORMAL)
			WorkModule::off_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
		}
    });
}		
#[acmd_script(
    agent = "mewtwo",
    script =  "game_attackairhi",
    category = ACMD_GAME,
	low_priority)]
unsafe fn m2_uair(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		frame(Frame=3)
		if(is_excute){
			HIT_NODE(hash40("s_tail2"), HIT_STATUS_XLU)
			HIT_NODE(hash40("s_tail3"), HIT_STATUS_XLU)
			HIT_NODE(hash40("s_tail5"), HIT_STATUS_XLU)
			HIT_NODE(hash40("s_tail4"), HIT_STATUS_XLU)
			HIT_NODE(hash40("s_tail7"), HIT_STATUS_XLU)
			WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
		}
		frame(Frame=10)
		if(is_excute){
			ATTACK(ID=0, Part=0, Bone=hash40("s_tail3"), Damage=12.0, Angle=72, KBG=98, FKB=0, BKB=40, Size=5.0, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_TAIL)
			ATTACK(ID=1, Part=0, Bone=hash40("s_tail5"), Damage=11.0, Angle=65, KBG=92, FKB=0, BKB=40, Size=4.5, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_TAIL)
			ATTACK(ID=2, Part=0, Bone=hash40("s_tail7"), Damage=10.0, Angle=55, KBG=92, FKB=0, BKB=40, Size=4.0, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_TAIL)
		}
		wait(Frames=5)
		if(is_excute){
			AttackModule::clear_all()
		}
		frame(Frame=36)
		if(is_excute){
			HIT_NODE(hash40("s_tail2"), HIT_STATUS_NORMAL)
			HIT_NODE(hash40("s_tail5"), HIT_STATUS_NORMAL)
			HIT_NODE(hash40("s_tail4"), HIT_STATUS_NORMAL)
			HIT_NODE(hash40("s_tail3"), HIT_STATUS_NORMAL)
			HIT_NODE(hash40("s_tail7"), HIT_STATUS_NORMAL)
			WorkModule::off_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
		}
    });
}		
		
		
pub fn install() {
    smashline::install_acmd_scripts!(
		m2_bair,
		m2_dash,
		m2_turn_dash,
		m2_uair
    );
    smashline::install_agent_frames!(
        mew2_frame
    );
}
