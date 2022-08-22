use smash::hash40;
use smash::lib::lua_const::*;
use smash::app::lua_bind::*;
use smash::lua2cpp::*;
use smashline::*;
use smash_script::*;

static mut IS_DJ: [bool; 8] = [false; 8];

#[fighter_frame_callback]
pub fn jr(fighter : &mut L2CFighterCommon) {
    unsafe {
        let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent); 
		let status_kind = smash::app::lua_bind::StatusModule::status_kind(boma);
		let ENTRY_ID = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
		let fighter_kind = smash::app::utility::get_kind(boma);
		let cancel_frame = FighterMotionModuleImpl::get_cancel_frame(boma,smash::phx::Hash40::new_raw(MotionModule::motion_kind(boma)),false) as f32;
		if fighter_kind == *FIGHTER_KIND_KOOPAJR {
			if StatusModule::situation_kind(boma) != *SITUATION_KIND_AIR {
				IS_DJ[ENTRY_ID] = false;
			};
			if IS_DJ[ENTRY_ID] == true {
				let stop_rise  = smash::phx::Vector3f { x: 1.0, y: 0.75, z: 1.0 };
				KineticModule::mul_speed(boma, &stop_rise, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
				IS_DJ[ENTRY_ID] = false;
			};
			if WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT) >= WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT_MAX) {
				if [*FIGHTER_KOOPAJR_STATUS_KIND_SPECIAL_HI_FALL, *FIGHTER_KOOPAJR_STATUS_KIND_SPECIAL_HI_SHOOT, *FIGHTER_KOOPAJR_STATUS_KIND_SPECIAL_HI_ATTACK, *FIGHTER_KOOPAJR_STATUS_KIND_SPECIAL_HI_ESCAPE, *FIGHTER_KOOPAJR_STATUS_KIND_SPECIAL_HI_DAMAGE_END].contains(&status_kind) == false {
					if [*FIGHTER_STATUS_KIND_FALL, *FIGHTER_STATUS_KIND_FALL_AERIAL, *FIGHTER_STATUS_KIND_JUMP, *FIGHTER_STATUS_KIND_JUMP_AERIAL].contains(&status_kind) || (MotionModule::frame(boma) >= cancel_frame && cancel_frame > 1.0) {
						if ControlModule::check_button_on_trriger(boma, *CONTROL_PAD_BUTTON_JUMP) || ControlModule::check_button_on_trriger(boma, *CONTROL_PAD_BUTTON_FLICK_JUMP) || ControlModule::check_button_on_trriger(boma, *CONTROL_PAD_BUTTON_JUMP_MINI) {
							StatusModule::change_status_request_from_script(boma, *FIGHTER_KOOPAJR_STATUS_KIND_SPECIAL_HI_SHOOT, true);
							IS_DJ[ENTRY_ID] = true;
						};
					};
				};
			};
			if status_kind == *FIGHTER_KOOPAJR_STATUS_KIND_SPECIAL_HI_SHOOT && MotionModule::frame(boma) > 4.0 {
				if ControlModule::check_button_on_trriger(boma, *CONTROL_PAD_BUTTON_SPECIAL) || ControlModule::check_button_on_trriger(boma, *CONTROL_PAD_BUTTON_ATTACK)  || ControlModule::check_button_on_trriger(boma, *CONTROL_PAD_BUTTON_CSTICK_ON){
						StatusModule::change_status_request_from_script(boma, *FIGHTER_KOOPAJR_STATUS_KIND_SPECIAL_HI_ATTACK, true);
				};
			};
			if status_kind == *FIGHTER_KOOPAJR_STATUS_KIND_SPECIAL_HI_LANDING {
				MotionModule::set_rate(boma, 2.0);
			};
		};
	};
}
#[acmd_script(
    agent = "koopajr",
    script =  "game_throwlw",
    category = ACMD_GAME)]
unsafe fn jr_dthrow(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
		if(is_excute){
			ATTACK_ABS(Kind=FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, ID=0, Damage=4.0, Angle=270, KBG=0, FKB=0, BKB=0, Hitlag=0.0, Unk=1.0, FacingRestrict=ATTACK_LR_CHECK_B, Unk=0.0, Unk=true, Effect=hash40("collision_attr_lay"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_THROW)
			ATTACK_ABS(Kind=FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, ID=0, Damage=3.0, Angle=361, KBG=100, FKB=0, BKB=40, Hitlag=0.0, Unk=1.0, FacingRestrict=ATTACK_LR_CHECK_B, Unk=0.0, Unk=true, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_NONE, Type=ATTACK_REGION_THROW)
		}
		frame(Frame=9)
		if(is_excute){
			ArticleModule::remove_exist(FIGHTER_KOOPAJR_GENERATE_ARTICLE_MAGICHAND,smash::app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL))
		}
		frame(Frame=16)
		if(is_excute){
			ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=1.2, Angle=361, KBG=100, FKB=30, BKB=0, Size=5.0, X=0.0, Y=4.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.5, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=5, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_OBJECT)
			AttackModule::set_catch_only_all(true, false)
		}
		frame(Frame=51)
		if(is_excute){
			ATK_HIT_ABS(FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, hash40("throw"), WorkModule::get_int64(module_accessor,*FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT), WorkModule::get_int64(module_accessor,*FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP), WorkModule::get_int64(module_accessor,*FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO))
			AttackModule::clear_all()
		}
    });
}
		
pub fn install() {
    smashline::install_agent_frame_callbacks!(jr);
    smashline::install_acmd_scripts!(
		jr_dthrow
	);
}
