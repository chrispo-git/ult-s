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
mod command;
mod cancel;
mod training;

pub static mut IS_GLOW: bool = false;
pub static mut DI_DIR: i32 = 0;

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
    let defender_boma = &mut *(*util::get_battle_object_from_id(defender)).module_accessor;
    let attacker_boma = &mut *(*util::get_battle_object_from_id(attacker)).module_accessor;
    if defender_boma.is_item() {
        if (defender_boma.kind() == *ITEM_KIND_TOONLINKBOMB) {
            if attacker_boma.is_fighter() {
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
	command::install();
	cancel::install();
    training::install();
    skyline::install_hooks!(
        process_knockback,
        calculate_knockback
    );
}