use smash::app::lua_bind::*;
use smash::lib::lua_const::*;
use smash::app::utility::get_kind;
use smash::hash40;
use smash::lua2cpp::*;
use smashline::*;
use smash_script::*;
use smash::phx::*;
use smash::lib::{ L2CValue, L2CAgent };
use smash::phx::Vector2f;
use crate::util::*;
use once_cell::sync::Lazy;

//0 for hit_condition means it can always be jump cancelled
//Otherwise, set hit_condition to a value such as *COLLISION_KIND_MASK_HIT
//-1 for jc_start/jc_end means it will always be jump cancellable at any point
struct JumpCancelEntry {
    pub fighter_kind : i32,
    pub status_kind : i32,
    pub hit_condition : i32,
    pub jc_start : i32,
    pub jc_end : i32,
    pub slot_start: i32,
    pub slot_count : i32,
}
impl JumpCancelEntry {
    pub const fn new(kind: i32, status: i32, hit: i32, start: i32, end: i32, start_slot: i32, slots : i32) -> Self {
        Self {
            fighter_kind : kind,
            status_kind: status,
            hit_condition: hit,
            jc_start: start,
            jc_end: end,
            slot_start : start_slot,
            slot_count : slots,
        }
    }
}
static JC_LIST: Lazy<Vec<JumpCancelEntry>> = Lazy::new(|| {
    vec![
        JumpCancelEntry::new(*FIGHTER_KIND_KAMUI, *FIGHTER_KAMUI_STATUS_KIND_SPECIAL_N_HOLD, 0, -1, -1, 0, 16),
        JumpCancelEntry::new(*FIGHTER_KIND_FALCO, *FIGHTER_STATUS_KIND_SPECIAL_LW, 0, 4, 32, 0, 16), 
        JumpCancelEntry::new(*FIGHTER_KIND_FALCO, *FIGHTER_STATUS_KIND_SPECIAL_LW, 0, 4, 32, 120, 8), // Peppy
        JumpCancelEntry::new(*FIGHTER_KIND_WOLF, *FIGHTER_STATUS_KIND_SPECIAL_LW, 0, -1, -1, 0, 16),
        JumpCancelEntry::new(*FIGHTER_KIND_WOLF, *FIGHTER_WOLF_STATUS_KIND_SPECIAL_LW_END, 0, -1, -1, 0, 16),
        JumpCancelEntry::new(*FIGHTER_KIND_WOLF, *FIGHTER_WOLF_STATUS_KIND_SPECIAL_LW_HIT, 0, -1, -1, 0, 16),
        JumpCancelEntry::new(*FIGHTER_KIND_WOLF, *FIGHTER_WOLF_STATUS_KIND_SPECIAL_LW_LOOP, 0, -1, -1, 0, 16),
        JumpCancelEntry::new(*FIGHTER_KIND_FOX, *FIGHTER_STATUS_KIND_SPECIAL_LW, 0, -1, -1, 0, 16),
        JumpCancelEntry::new(*FIGHTER_KIND_FOX, *FIGHTER_WOLF_STATUS_KIND_SPECIAL_LW_END, 0, -1, -1, 0, 16),
        JumpCancelEntry::new(*FIGHTER_KIND_FOX, *FIGHTER_WOLF_STATUS_KIND_SPECIAL_LW_HIT, 0, -1, -1, 0, 16),
        JumpCancelEntry::new(*FIGHTER_KIND_FOX, *FIGHTER_WOLF_STATUS_KIND_SPECIAL_LW_LOOP, 0, -1, -1, 0, 16),
        JumpCancelEntry::new(*FIGHTER_KIND_MIIGUNNER, *FIGHTER_MIIGUNNER_STATUS_KIND_SPECIAL_LW1_END, 0, -1, -1, 0, 16),
        JumpCancelEntry::new(*FIGHTER_KIND_MIIGUNNER, *FIGHTER_MIIGUNNER_STATUS_KIND_SPECIAL_LW1_LOOP, 0, -1, -1, 0, 16),
        JumpCancelEntry::new(*FIGHTER_KIND_MIIGUNNER, *FIGHTER_MIIGUNNER_STATUS_KIND_SPECIAL_LW1_HIT, 0, -1, -1, 0, 16),
    ]
});
#[inline]
pub(crate) fn is_jc(
    boma: &mut smash::app::BattleObjectModuleAccessor,
    fighter_kind: i32,
    status_kind: i32,
    frame: f32
) -> bool {
    unsafe {
	    let fighter = fighter_kind;
        let costume = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);
        for i in JC_LIST.iter() {
            if costume < i.slot_start || costume >= (i.slot_start+i.slot_count) {
                continue;
            }
            if fighter == i.fighter_kind && status_kind == i.status_kind {
                if i.jc_start != -1 && i.jc_end != -1 {
                    if (frame as i32) < i.jc_start || (frame as i32) >= i.jc_end {
                        continue;
                    }
                }
                if i.hit_condition != 0 {
                    if AttackModule::is_infliction_status(boma, i.hit_condition) {
                        return true;
                    }
                } else {
                    return true;
                }
            }
        }
        return false;
    }
}
pub(crate) fn check_jump(boma: &mut smash::app::BattleObjectModuleAccessor) -> bool {
    unsafe {
        if ControlModule::check_button_on_trriger(boma, *CONTROL_PAD_BUTTON_JUMP) {
            return true;
        }
        if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_FLICK_JUMP) {
            if ControlModule::get_flick_y(boma) >= 3 && ControlModule::get_stick_y(boma) >= 0.7 {
                return true;
            }
        }
        if ControlModule::check_button_on_trriger(boma, *CONTROL_PAD_BUTTON_JUMP_MINI) {
            return true;
        }
        return false;
    }
}

//

pub unsafe fn opff(fighter: &mut L2CFighterCommon, status_kind : i32) {
    if !is_mechanics_enabled() {
		return;
	}
    let situation_kind = StatusModule::situation_kind(fighter.module_accessor);
    if  WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT) >=
        WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT_MAX) &&
        situation_kind == *SITUATION_KIND_AIR {
        return;
    };
    let fighter_kind = smash::app::utility::get_kind(boma(fighter));
    let frame = MotionModule::frame(fighter.module_accessor);
    if is_jc(boma(fighter), fighter_kind, status_kind, frame) && check_jump(boma(fighter)) {
        match situation_kind {
            n if n == *SITUATION_KIND_AIR => StatusModule::change_status_request_from_script(fighter.module_accessor,*FIGHTER_STATUS_KIND_JUMP_AERIAL, true),
            _ => StatusModule::change_status_request_from_script(fighter.module_accessor,*FIGHTER_STATUS_KIND_JUMP_SQUAT, true),
        };
    }
}
pub unsafe fn lazy_warm() {
	Lazy::force(&JC_LIST);
}