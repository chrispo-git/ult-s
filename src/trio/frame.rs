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
    Agent::new("mario_trio")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .on_line(Main, trio_frame)
    .install();
}
unsafe extern "C" fn trio_frame(weapon: &mut L2CFighterBase) {
    unsafe {
        let otarget_id = WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER) as u32;
        let boma = smash::app::sv_battle_object::module_accessor(otarget_id);
		let ENTRY_ID = WorkModule::get_int(&mut *boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
        let owner_status = StatusModule::status_kind(&mut *boma);
        let owner_motion = MotionModule::motion_kind(&mut *boma);
        let owner_rate = MotionModule::rate(&mut *boma);
        let owner_frame = MotionModule::frame(&mut *boma);
		let status_kind = StatusModule::status_kind(weapon.module_accessor);
        let motion_kind = MotionModule::motion_kind(weapon.module_accessor);
        let motion_rate = MotionModule::rate(weapon.module_accessor);
        if smash::app::utility::get_kind(&mut *boma) == *FIGHTER_KIND_MARIO {
            if motion_kind != owner_motion {
                MotionModule::change_motion(weapon.module_accessor, Hash40::new_raw(owner_motion), 0.0, 1.0, false, 0.0, false, false);
                if owner_rate != 0.0 {
                    MotionModule::set_rate(weapon.module_accessor, owner_rate);
					MotionModule::set_frame_sync_anim_cmd(weapon.module_accessor, owner_frame, true, true, false);
                }
            } else {
                if owner_rate != 0.0 && owner_rate != motion_rate {
                    MotionModule::set_rate(weapon.module_accessor, owner_rate);
                }
            }

            if FOLLOW_MAIN[ENTRY_ID] {
				let pos = smash::phx::Vector3f { x:PostureModule::pos_x(&mut *boma), y: PostureModule::pos_y(&mut *boma), z: 0.0 };
				PostureModule::set_pos(weapon.module_accessor, &pos);
				PostureModule::init_pos(weapon.module_accessor, &pos, true, true);
            } else {
                TRIO_POS_X[ENTRY_ID] = PostureModule::pos_x(weapon.module_accessor);
                TRIO_POS_Y[ENTRY_ID] = PostureModule::pos_y(weapon.module_accessor);
            }

			let tether_pos = &mut Vector3f{ x: 0.0, y: 0.0, z: 0.0 };
			ModelModule::joint_global_position(boma, Hash40::new("throw"), tether_pos, false);
		};
    }
}


pub(crate) unsafe fn hurtbox_handler(weapon: &mut L2CFighterBase, otarget_id: u32, bone: Hash40,
            //x1: f32, y1: f32, z1: f32, x2: f32, y2: f32, z2: f32, size: f32
             size: f32
) -> () {
    let otarget_id = WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER) as u32;
    let boma = smash::app::sv_battle_object::module_accessor(otarget_id);

	let owner = &mut Vector3f{ x: 0.0, y: 0.0, z: 0.0 };
	ModelModule::joint_global_position(&mut *boma,bone, owner, false);

	let child = &mut Vector3f{ x: 0.0, y: 0.0, z: 0.0 };
	ModelModule::joint_global_position(weapon.module_accessor,bone, owner, false);


    let mut combined = Vector3f{x: child.x - owner.x, y: child.y - owner.y, z: child.z - owner.z};
    FighterUtil::set_hit_data(&mut *boma,0,0,&vec3,&vec3,0.0,Hash40::new("top"),CollisionPart(*COLLISION_PART_BODY),HitHeight(*HIT_HEIGHT_CENTER),HitStatus(*HIT_STATUS_NORMAL),CollisionShapeType(*COLLISION_SHAPE_TYPE_CAPSULE));
    
}