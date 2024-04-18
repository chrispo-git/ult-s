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
use smash::phx::Vector3f;
use crate::util::*;
use super::*;

pub fn install() {
    Agent::new("roy")
    .on_line(Main, roy)
    .install();
}

unsafe extern "C" fn roy(fighter : &mut L2CFighterCommon) {
    unsafe {
        let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent); 
		let status_kind = smash::app::lua_bind::StatusModule::status_kind(boma);
		let ENTRY_ID = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
		let fighter_kind = smash::app::utility::get_kind(boma);
		let speed = 5.8;
		if fighter_kind == *FIGHTER_KIND_ROY && is_default(boma) {
			/*if [hash40("attack_s3_s")].contains(&MotionModule::motion_kind(boma)) {
				if MotionModule::frame(boma) > 9.0 && MotionModule::frame(boma) < 20.0 {
					if MotionModule::frame(boma) < 12.0 || AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_ALL) == false {
						if MotionModule::frame(boma) < 19.0 {
							macros::ATTACK(fighter, /*ID*/ 3, /*Part*/ 1, /*Bone*/ Hash40::new("top"), /*Damage*/ 4.0, /*Angle*/ 361, /*KBG*/ 20, /*FKB*/ 0, /*BKB*/ 40, /*Size*/ 4.0, /*X*/ 0.0, /*Y*/ 10.0, /*Z*/ ((MotionModule::frame(boma)-9.0)*speed)+12.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.7, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ -7, /*Trip*/ -1.0, /*Rehit*/ 0, /*Reflectable*/ true, /*Absorbable*/ true, /*Flinchless*/ false, /*DisableHitlag*/ true, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_fire"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_FIRE, /*Type*/ *ATTACK_REGION_NONE);
						} else {
							AttackModule::clear_all(boma);
						};
						if MotionModule::frame(boma) as i32 % 2 == 0{
							let distance1 = smash::phx::Vector3f { x: 0.0, y: 10.0, z: ((MotionModule::frame(boma)-9.0)*speed)+12.0 };
							let distance2 = smash::phx::Vector3f { x: 0.0, y: 10.0, z: ((MotionModule::frame(boma)-9.0)*speed)+7.0 };
							let empty = smash::phx::Vector3f { x: 0.0, y: 0.0, z: 0.0 };
							let projectile: u32 = EffectModule::req_follow(boma, smash::phx::Hash40::new("sys_fireflower_shot"), smash::phx::Hash40::new("top"), &distance1, &empty, 1.0, true, 0, 0, 0, 0, 0, true, true) as u32;
							EffectModule::set_rgb(boma, projectile, 1.25, 0.5, 0.5);
							let projectile2: u32 = EffectModule::req_follow(boma, smash::phx::Hash40::new("sys_fireflower_shot"), smash::phx::Hash40::new("top"), &distance2, &empty, 1.0, true, 0, 0, 0, 0, 0, true, true) as u32;
							EffectModule::set_rgb(boma, projectile2, 1.25, 0.5, 0.5);
						};
					};
				} else {
					EffectModule::kill_kind(boma, smash::phx::Hash40::new("sys_fireflower_shot"), false, false);
				};
			};*/
			if [hash40("attack_dash")].contains(&MotionModule::motion_kind(boma)) {
				if MotionModule::frame(boma) >= 36.0 {
					StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_SQUAT_RV, true);
				};
				if MotionModule::frame(boma) >= 22.0 && AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_ALL) == true {
					MotionModule::change_motion(boma, smash::phx::Hash40::new("attack_dash_2"), 0.0, 1.0, false, 0.0, false, false);
				};
			};
		};
	};
}