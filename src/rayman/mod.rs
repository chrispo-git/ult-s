use smash::app::sv_animcmd::*;
use smash::phx::{Hash40, Vector2f, Vector3f};
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
use crate::util::*;
use std::collections::HashMap;

mod status;
mod frame; 
mod acmd;
			
static mut CURRENT_PIKMIN : [i32; 8] = [0; 8];
static mut SET_UPB_FREEFALL: [bool; 8] = [false; 8];
static mut IS_SLIDE_MOVE: [bool; 8] = [false; 8];
static mut PULL_DISTANCE: [i32; 8] = [0; 8];
static mut DO_WALLJUMP_FORCE: [bool; 8] = [false; 8];
static mut HAS_DEADED: [bool; 8] = [false; 8];
static mut WAS_SLIDE: [bool; 8] = [false; 8];
static mut FINAL_DURATION : [i32; 8] = [0; 8];
static mut CAPTURE_TIME : [i32; 8] = [0; 8];
static mut X : [f32; 8] = [0.0; 8];
static mut Y : [f32; 8] = [0.0; 8];
static mut X_MAX : f32 = 1.155;
static mut X_ACCEL_ADD : f32 = 0.06;
static mut X_ACCEL_MUL : f32 = 0.12;
static mut Y_MAX : f32 = 1.155;
static mut Y_ACCEL_ADD : f32 = 0.06;
static mut Y_ACCEL_MUL : f32 = 0.12;

pub(crate) unsafe fn attack_vc(fighter: &mut L2CAgentBase) -> () {
	let rand_val = smash::app::sv_math::rand(hash40("fighter"), 6);
	match rand_val {
		0 => macros::PLAY_SE(fighter, Hash40::new("se_pikmin_attackair_l01")),
		1 => macros::PLAY_SE(fighter, Hash40::new("se_pikmin_attackair_h01")),
		2 => macros::PLAY_SE(fighter, Hash40::new("se_pikmin_attackair_f01")),
        3 => macros::PLAY_SE(fighter, Hash40::new("se_pikmin_attackdash02")),
		_ => println!("rayman is silent"),
	}
}
pub(crate) unsafe fn dmg_vc(fighter: &mut L2CAgentBase) -> () {
	macros::STOP_SE(fighter, Hash40::new("se_pikmin_attackhard_h01"));
	macros::STOP_SE(fighter, Hash40::new("se_pikmin_attackhard_h02"));
	macros::STOP_SE(fighter, Hash40::new("se_pikmin_attackhard_h03"));
    macros::STOP_SE(fighter, Hash40::new("se_pikmin_attackhard_l01"));
    macros::STOP_SE(fighter, Hash40::new("se_pikmin_attackhard_s01"));
	let rand_val = smash::app::sv_math::rand(hash40("fighter"), 5);
	match rand_val {
		0 => macros::PLAY_SE(fighter, Hash40::new("se_pikmin_attackhard_h01")),
		1 => macros::PLAY_SE(fighter, Hash40::new("se_pikmin_attackhard_h02")),
        2 => macros::PLAY_SE(fighter, Hash40::new("se_pikmin_attackhard_h03")),
        3 => macros::PLAY_SE(fighter, Hash40::new("se_pikmin_attackhard_l01")),
		_ => macros::PLAY_SE(fighter, Hash40::new("se_pikmin_attackhard_s01")),
	}
}
pub(crate) unsafe fn dmg_fly_vc(fighter: &mut L2CAgentBase) -> () {
	let rand_val = smash::app::sv_math::rand(hash40("fighter"), 3);
	macros::STOP_SE(fighter, Hash40::new("se_pikmin_attackdash01"));
	macros::STOP_SE(fighter, Hash40::new("se_pikmin_attackair_n03"));
	macros::STOP_SE(fighter, Hash40::new("se_pikmin_attackair_n02"));
	match rand_val {
		0 => macros::PLAY_SE(fighter, Hash40::new("se_pikmin_attackdash01")),
		1 => macros::PLAY_SE(fighter, Hash40::new("se_pikmin_attackair_n03")),
		_ => macros::PLAY_SE(fighter, Hash40::new("se_pikmin_attackair_n02")),
	}
}
pub(crate) unsafe fn jump_vc(fighter: &mut L2CAgentBase) -> () {
	let rand_val = smash::app::sv_math::rand(hash40("fighter"), 2);
	macros::STOP_SE(fighter, Hash40::new("se_pikmin_attackdash02"));
	match rand_val {
		0 => macros::PLAY_SE(fighter, Hash40::new("se_pikmin_attackdash02")),
        _ => println!("rayman is silent"),
	}
}
pub(crate) unsafe fn jump_aerial_vc(fighter: &mut L2CAgentBase) -> () {
	let rand_val = smash::app::sv_math::rand(hash40("fighter"), 4);
	macros::STOP_SE(fighter, Hash40::new("se_pikmin_attackhard_s02"));
	match rand_val {
		0 => macros::PLAY_SE(fighter, Hash40::new("se_pikmin_attackhard_s02")),
		_ => println!("rayman is silent"),
	}
}

pub fn install() {
	frame::install();
	status::install();
	acmd::install();

    the_csk_collection_api::add_chara_db_entry_info(
        the_csk_collection_api::CharacterDatabaseEntry {
                ui_chara_id: smash::hash40("ui_chara_rayman"),
                fighter_kind: the_csk_collection_api::Hash40Type::Overwrite(smash::hash40("fighter_kind_pikmin")), 
                fighter_kind_corps: the_csk_collection_api::Hash40Type::Overwrite(smash::hash40("fighter_kind_pikmin")), 
                ui_series_id: the_csk_collection_api::Hash40Type::Overwrite(smash::hash40("ui_series_rayman")), 
                fighter_type: the_csk_collection_api::Hash40Type::Overwrite(0x1353795179 /* Hash40 of fighter_type_normal */), 
                alt_chara_id: the_csk_collection_api::Hash40Type::Overwrite(0x2302D482A /* Hash40 of -1 */), 
                shop_item_tag: the_csk_collection_api::Hash40Type::Overwrite(0x5E1155EA7 /* Hash40 of sc-09 */), 
                name_id: the_csk_collection_api::StringType::Overwrite(the_csk_collection_api::CStrCSK::new("rayman")), 
                exhibit_year: the_csk_collection_api::ShortType::Overwrite(1995), 
                exhibit_day_order: the_csk_collection_api::IntType::Overwrite(13102), 
                extra_flags: the_csk_collection_api::IntType::Overwrite(0), 
                ext_skill_page_num: the_csk_collection_api::SignedByteType::Overwrite(1), 
                skill_list_order: the_csk_collection_api::SignedByteType::Overwrite(84), 
                disp_order: the_csk_collection_api::SignedByteType::Optional(Some(84)), 
                save_no: the_csk_collection_api::SignedByteType::Overwrite(82), 
                chara_count: the_csk_collection_api::SignedByteType::Overwrite(1), 
                is_img_ext_skill_page0: the_csk_collection_api::BoolType::Overwrite(true), 
                is_img_ext_skill_page1: the_csk_collection_api::BoolType::Overwrite(true), 
                is_img_ext_skill_page2: the_csk_collection_api::BoolType::Overwrite(true), 
                can_select: the_csk_collection_api::BoolType::Overwrite(true), 
                is_usable_soundtest: the_csk_collection_api::BoolType::Overwrite(false), 
                is_called_pokemon: the_csk_collection_api::BoolType::Overwrite(false), 
                is_mii: the_csk_collection_api::BoolType::Overwrite(false), 
                is_boss: the_csk_collection_api::BoolType::Overwrite(false), 
                is_hidden_boss: the_csk_collection_api::BoolType::Overwrite(false), 
                is_dlc: the_csk_collection_api::BoolType::Overwrite(false), 
                is_patch: the_csk_collection_api::BoolType::Overwrite(false), 
                is_plural_message: the_csk_collection_api::BoolType::Overwrite(false), 
                is_plural_narration: the_csk_collection_api::BoolType::Overwrite(false), 
                is_article: the_csk_collection_api::BoolType::Overwrite(false), 
                has_multiple_face: the_csk_collection_api::BoolType::Overwrite(false), 
                result_pf0: the_csk_collection_api::BoolType::Overwrite(true), 
                result_pf1: the_csk_collection_api::BoolType::Overwrite(true), 
                result_pf2: the_csk_collection_api::BoolType::Overwrite(true), 
            color_num: the_csk_collection_api::UnsignedByteType::Overwrite(8),
            extra_index_maps: the_csk_collection_api::UnsignedByteMap::Overwrite(HashMap::from([
                (smash::hash40("color_start_index") /* Hash40 of color_start_index */, the_csk_collection_api::UnsignedByteType::Overwrite(120))
            ])),
            extra_hash_maps: the_csk_collection_api::Hash40Map::Overwrite(HashMap::from([
                    (0x1337FC912E /* Hash40 of characall_label_c00 */, the_csk_collection_api::Hash40Type::Overwrite(smash::hash40("vc_narration_characall_rayman"))),
                    (0x1340FBA1B8 /* Hash40 of characall_label_c01 */, the_csk_collection_api::Hash40Type::Overwrite(0x0)),
                    (0x13D9F2F002 /* Hash40 of characall_label_c02 */, the_csk_collection_api::Hash40Type::Overwrite(0x0)),
                    (0x13AEF5C094 /* Hash40 of characall_label_c03 */, the_csk_collection_api::Hash40Type::Overwrite(0x0)),
                    (0x1330915537 /* Hash40 of characall_label_c04 */, the_csk_collection_api::Hash40Type::Overwrite(0x0)),
                    (0x13479665A1 /* Hash40 of characall_label_c05 */, the_csk_collection_api::Hash40Type::Overwrite(0x0)),
                    (0x13DE9F341B /* Hash40 of characall_label_c06 */, the_csk_collection_api::Hash40Type::Overwrite(0x0)),
                    (0x13A998048D /* Hash40 of characall_label_c07 */, the_csk_collection_api::Hash40Type::Overwrite(0x0)),
                    (0x1B8B13E500 /* Hash40 of characall_label_article_c00 */, the_csk_collection_api::Hash40Type::Overwrite(0x0)),
                    (0x1BFC14D596 /* Hash40 of characall_label_article_c01 */, the_csk_collection_api::Hash40Type::Overwrite(0x0)),
                    (0x1B651D842C /* Hash40 of characall_label_article_c02 */, the_csk_collection_api::Hash40Type::Overwrite(0x0)),
                    (0x1B121AB4BA /* Hash40 of characall_label_article_c03 */, the_csk_collection_api::Hash40Type::Overwrite(0x0)),
                    (0x1B8C7E2119 /* Hash40 of characall_label_article_c04 */, the_csk_collection_api::Hash40Type::Overwrite(0x0)),
                    (0x1BFB79118F /* Hash40 of characall_label_article_c05 */, the_csk_collection_api::Hash40Type::Overwrite(0x0)),
                    (0x1B62704035 /* Hash40 of characall_label_article_c06 */, the_csk_collection_api::Hash40Type::Overwrite(0x0)),
                    (0x1B157770A3 /* Hash40 of characall_label_article_c07 */, the_csk_collection_api::Hash40Type::Overwrite(0x0)),
                (0x160ab9eb98 /* Hash40 of original_ui_chara_hash */, the_csk_collection_api::Hash40Type::Overwrite(smash::hash40("ui_chara_pikmin")) /* Hash40 of ui_chara_falco*/)
            ])),
            ..Default::default()
        },
    );
    the_csk_collection_api::add_chara_layout_db_entry_info(
        the_csk_collection_api::CharacterLayoutDatabaseEntry {
            ui_layout_id: smash::hash40("ui_chara_rayman_00"), // Hash40 of ui_chara_aaa_00
            clone_from_ui_layout_id: Some(smash::hash40("ui_chara_mario_00")), // Hash40 of ui_chara_falco_00
            ui_chara_id: the_csk_collection_api::Hash40Type::Overwrite(
                smash::hash40("ui_chara_rayman"), // Hash40 of ui_chara_aaa
            ),
            ..Default::default()
        },
    );
    the_csk_collection_api::add_bgm_db_entry_info(&the_csk_collection_api::BgmDatabaseRootEntry {
        ui_bgm_id: hash40("ui_bgm_z25_f_rayman"),
        clone_from_ui_bgm_id: Some(hash40("ui_bgm_z25_f_pikmin")),
        stream_set_id: the_csk_collection_api::Hash40Type::Overwrite(hash40("set_z25_f_rayman")),
        ..Default::default()
    });

    the_csk_collection_api::add_stream_set_entry_info(&the_csk_collection_api::StreamSetEntry { 
        stream_set_id: hash40("set_z25_f_rayman"),
        info0: the_csk_collection_api::Hash40Type::Overwrite(hash40("info_z25_f_rayman")),
        ..Default::default()
    });

    the_csk_collection_api::add_assigned_info_entry_info(&the_csk_collection_api::AssignedInfoEntry { 
        info_id: hash40("info_z25_f_rayman"),
        stream_id: the_csk_collection_api::Hash40Type::Overwrite(hash40("stream_z25_f_rayman")),
        condition: the_csk_collection_api::Hash40Type::Overwrite(hash40("sound_condition_none")),
        condition_process: the_csk_collection_api::Hash40Type::Overwrite(hash40("sound_condition_process_add")),
        change_fadeout_frame: the_csk_collection_api::IntType::Overwrite(60),
        menu_change_fadeout_frame: the_csk_collection_api::IntType::Overwrite(60),
        ..Default::default()
    });

    the_csk_collection_api::add_stream_property_entry_info(&the_csk_collection_api::StreamPropertyEntry {
        stream_id: hash40("stream_z25_f_rayman"),
        data_name0: the_csk_collection_api::StringType::Overwrite(the_csk_collection_api::CStrCSK::new("z25_f_rayman")),
        ..Default::default()
    });

    the_csk_collection_api::add_new_bgm_property_entry(&smash_bgm_property::BgmPropertyEntry {
        stream_name: hash40::Hash40::new("z25_f_rayman"),
        loop_start_ms: 0,
        loop_start_sample: 0,
        loop_end_ms: 0,
        loop_end_sample: 0,
        duration_ms: 7659,
        duration_sample: 359424 
    });

    the_csk_collection_api::set_fighter_jingle(hash40("ui_chara_rayman"), "z25_f_rayman");
}