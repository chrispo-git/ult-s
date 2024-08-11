mod hitstun;
mod dacus;
mod landing;
mod wavedash;
mod jab;
mod movement;
mod bone;
mod projectile_invuln;
mod remove_quake;
mod melee;
mod faf_change;
mod cancel;
mod training;
use smash::app::lua_bind::*;
use smash::lib::lua_const::*;
use smash::app::utility::get_kind;
use smash::hash40;
use smash::lua2cpp::*;
use smashline::*;
use smash_script::*;
use smash::phx::Hash40;
use crate::util::*;

pub static mut IS_GLOW: bool = false;
pub static mut DI_DIR: i32 = 0;
pub static mut LEDGE_OPTION: i32 = 0;
// 0 - Neutral Getup
// 1 - Ledge Attack
// 2 - Ledge Roll
// 3 - Ledge Jump
// 4 - Ledge Drop -> Double Jump
// 5 - Wait at ledge for 30 more frames
pub static mut LEDGE_OPTION_AFTER: i32 = 0;
// 0 - None
// 1 - Shield/Airdodge
// 2 - Aerial/Tilt (Have 2 lists of whether to ftilt/dtilt)
pub static mut LEDGE_DELAY : [i32; 8] = [0; 8];
pub static mut DJ_DELAY : [i32; 8] = [0; 8];
pub static mut DELAY_FRAMES: i32 = 30;
pub static mut DJ_DELAY_FRAMES: i32 = 21;


static mut IS_CALCULATING: Option<(u32, u32)> = None;

#[skyline::hook(offset = 0x402f00, inline)]
unsafe fn calculate_knockback(ctx: &skyline::hooks::InlineCtx) {
    let damage_module = *ctx.registers[19].x.as_ref();
    let our_boma = *((damage_module + 0x8) as *mut *mut smash::app::BattleObjectModuleAccessor);
    let ptr = *ctx.registers[20].x.as_ref() as *mut u8;
    let id = *(ptr.add(0x24) as *const u32);
    IS_CALCULATING = Some(((*our_boma).battle_object_id, id));
}

#[skyline::hook(offset = 0x403950, inline)]
unsafe fn process_knockback(ctx: &skyline::hooks::InlineCtx) {
    if let Some((defender, attacker)) = IS_CALCULATING {
        let boma = *ctx.registers[20].x.as_ref() as *mut smash::app::BattleObjectModuleAccessor;
        if (*boma).battle_object_id == defender {
            process_toonlinkbomb_knockback(defender, attacker);
        }
    }
}
pub unsafe extern "C" fn process_toonlinkbomb_knockback(defender: u32, attacker: u32) {
    let defender_boma = smash::app::sv_battle_object::module_accessor(defender);
    let attacker_boma = smash::app::sv_battle_object::module_accessor(attacker);
    if smash::app::utility::get_category(std::mem::transmute(defender_boma)) == *BATTLE_OBJECT_CATEGORY_ITEM {
        if (smash::app::utility::get_kind(std::mem::transmute(defender_boma)) == *ITEM_KIND_TOONLINKBOMB) {
            if smash::app::utility::get_category(std::mem::transmute(defender_boma)) == *BATTLE_OBJECT_CATEGORY_FIGHTER {
                let attacker_team_no = (TeamModule::hit_team_no(attacker_boma) as i32);
                TeamModule::set_team(defender_boma, attacker_team_no, false);
            } else {
                HitModule::set_xlu_frame_global(defender_boma, 15, 0);
            }
            StatusModule::change_status_force(defender_boma, *ITEM_STATUS_KIND_THROW, true);
        }
    }
}
pub fn install() {
    hitstun::install();
    dacus::install();
    landing::install();
    wavedash::install();
    jab::install();
    movement::install();
    bone::install();
    melee::install();
	projectile_invuln::install();
	remove_quake::install();
	faf_change::install();
	cancel::install();
    training::install();
    /*skyline::install_hooks!(
        process_knockback,
        calculate_knockback
    );*/
}