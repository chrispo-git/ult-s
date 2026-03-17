use smash::hash40;
use smash::lib::lua_const::*;
use smash::lua2cpp::*;
use smash::app::lua_bind::*;
use smashline::*;
use smash_script::*;
use crate::util::*;
use once_cell::sync::Lazy;

pub struct ReflectorParams {
    pub fighter_kind: i32,
    pub motions: &'static [&'static str],
    pub start_frame: f32,
    pub end_frame: f32,
    pub bone: &'static str,
    pub size: f32,
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub x2: f32,
    pub y2: f32,
    pub z2: f32,
}
impl ReflectorParams {
    #[inline]
    pub unsafe fn is_motion(&self, motion_kind : u64) -> bool {
        self.motions.iter().any(|&m| hash40(m) == motion_kind)
    }
}
static REFLECTOR_DATA: Lazy<Vec<ReflectorParams>> = Lazy::new(|| {
    vec![
        ReflectorParams {
            fighter_kind: *FIGHTER_KIND_LUIGI,
            motions: &["special_s", "special_air_s", "special_s_discharge"],
            start_frame: 0.0,
            end_frame: f32::MAX,
            bone: "neck",
            size: 1.0,
            x: 0.0,  y: 0.0,  z: 0.0,
            x2: 0.0, y2: 1.2, z2: 0.0,
        },
        ReflectorParams {
            fighter_kind: *FIGHTER_KIND_DEDEDE,
            motions: &["attack_dash"],
            start_frame: 15.0,
            end_frame: 28.0,
            bone: "hip",
            size: 12.0,
            x: -5.0,  y: 0.0,  z: 0.0,
            x2: 14.0, y2: 0.0, z2: 0.0,
        },
        ReflectorParams {
            fighter_kind: *FIGHTER_KIND_MURABITO,
            motions: &["special_n"],
            start_frame: 3.0,
            end_frame: 10.0,
            bone: "stickr",
            size: 8.0,
            x: 0.0,  y: 0.0,  z: 3.0,
            x2: 0.0, y2: 0.0, z2: 11.0,
        },
        ReflectorParams {
            fighter_kind: *FIGHTER_KIND_MIISWORDSMAN,
            motions: &["special_s1", "special_air_s1"],
            start_frame: 0.0,
            end_frame: f32::MAX,
            bone: "stickr",
            size: 8.0,
            x: 0.0,  y: 12.0,  z: 6.0,
            x2: 0.0, y2: 3.5, z2: 6.0,
        },
        ReflectorParams {
            fighter_kind: *FIGHTER_KIND_RYU,
            motions: &["special_s"],
            start_frame: 0.0,
            end_frame: f32::MAX,
            bone: "top",
            size: 6.0,
            x: 0.0,  y: 12.5,  z: -11.0,
            x2: 0.0, y2: 12.5, z2: 12.5,
        },
        ReflectorParams {
            fighter_kind: *FIGHTER_KIND_GAOGAEN,
            motions: &["attack_dash"],
            start_frame: 7.0,
            end_frame: 19.0,
            bone: "legl",
            size: 10.0,
            x: 3.5,  y: 1.2,  z: 0.0,
            x2: 0.0, y2: 12.5, z2: 12.5,
        },
        ReflectorParams {
            fighter_kind: *FIGHTER_KIND_KROOL,
            motions: &["attack_dash"],
            start_frame: 10.0,
            end_frame: 29.0,
            bone: "legl",
            size: 10.0,
            x: 3.5,  y: 1.2,  z: 0.0,
            x2: 0.0, y2: 12.5, z2: 12.5,
        },
    ]
});
pub unsafe fn lazy_warm() {
	Lazy::force(&REFLECTOR_DATA);
}
static LIFE_MUL : f32 = 0.1;
static SPEED_MUL : f32 = 0.0;
static DMG_MUL : f32 = 0.0;
static REFLECTOR_MAX : f32 = 50.0;

pub unsafe fn opff(fighter : &mut L2CFighterCommon, motion_kind : u64) {
    unsafe {
		if !is_mechanics_enabled() {
			return;
		}
		let fighter_kind = smash::app::utility::get_kind(boma(fighter));
		let active_reflector = REFLECTOR_DATA.iter().find(|r| {
			r.fighter_kind == fighter_kind && r.is_motion(motion_kind)
		});
		if let Some(r) = active_reflector {
			let frame = MotionModule::frame(fighter.module_accessor);
			if frame >= r.start_frame && frame < r.end_frame {
				shield!(fighter, *MA_MSC_CMD_REFLECTOR, *COLLISION_KIND_REFLECTOR, 0, 
					hash40(r.bone), r.size, r.x, r.y, r.z, r.x2, r.y2, r.z2, 
					/*Power*/ DMG_MUL, /*Speed*/ SPEED_MUL, /*Max Damage*/ REFLECTOR_MAX, 
					false, /*Lifetime*/ LIFE_MUL, *FIGHTER_REFLECTOR_GROUP_HOMERUNBAT
				);
			} else {
				shield!(fighter, *MA_MSC_CMD_SHIELD_OFF, *COLLISION_KIND_REFLECTOR, 0, *FIGHTER_REFLECTOR_GROUP_HOMERUNBAT);
			}
		}
	}
}
