use smash::app::sv_animcmd::*;
use smash::phx::*;
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



static mut LAND_SIDEB_BOUNCE: [i32; 8] = [0; 8];
static mut BEFORE_SIDEB_BOUNCE: [i32; 8] = [0; 8];
static mut HAS_DOWNB: [bool; 8] = [false; 8];
static mut HAS_DEADED: [bool; 8] = [false; 8];
static mut SIDEB_RESET: [bool; 8] = [false; 8];
static mut SIDEB_END: [bool; 8] = [false; 8];
static mut SIDEB_LENGTH: [i32; 8] = [0; 8];
static mut SIDEB_DIR : [f32; 8] = [1.0; 8];
static mut BIG_TIMER: [i32; 8] = [0; 8];
pub const SIDEB_LENGTH_MAX : i32 = 53;
pub const BIG_TIMER_MAX : i32 = 1200;

pub(crate) unsafe fn attack_vc(fighter: &mut L2CAgentBase) -> () {
	let rand_val = smash::app::sv_math::rand(hash40("fighter"), 7);
	match rand_val {
		0 => macros::PLAY_SE(fighter, Hash40::new("se_murabito_attackair_b01")),
		1 => macros::PLAY_SE(fighter, Hash40::new("se_murabito_attackair_b02")),
		2 => macros::PLAY_SE(fighter, Hash40::new("se_murabito_attackair_f01")),
		3 => macros::PLAY_SE(fighter, Hash40::new("se_murabito_attackair_f02")),
		4 => macros::PLAY_SE(fighter, Hash40::new("se_murabito_attackair_h01")),
		_ => println!("toad is silent"),
	}
}
pub(crate) unsafe fn dmg_vc(fighter: &mut L2CAgentBase) -> () {
	macros::STOP_SE(fighter, Hash40::new("se_murabito_attackair_h02"));
	macros::STOP_SE(fighter, Hash40::new("se_murabito_attackair_h03"));
	macros::STOP_SE(fighter, Hash40::new("se_murabito_attackair_h01"));
	let rand_val = smash::app::sv_math::rand(hash40("fighter"), 3);
	match rand_val {
		0 => macros::PLAY_SE(fighter, Hash40::new("se_murabito_attackair_h02")),
		1 => macros::PLAY_SE(fighter, Hash40::new("se_murabito_attackair_h03")),
		_ => macros::PLAY_SE(fighter, Hash40::new("se_murabito_attackair_h01")),
	}
}
pub(crate) unsafe fn dmg_fly_vc(fighter: &mut L2CAgentBase) -> () {
	let rand_val = smash::app::sv_math::rand(hash40("fighter"), 3);
	macros::STOP_SE(fighter, Hash40::new("se_murabito_attackair_l02"));
	macros::STOP_SE(fighter, Hash40::new("se_murabito_attackair_l03"));
	macros::STOP_SE(fighter, Hash40::new("se_murabito_attackair_l01"));
	match rand_val {
		0 => macros::PLAY_SE(fighter, Hash40::new("se_murabito_attackair_l02")),
		1 => macros::PLAY_SE(fighter, Hash40::new("se_murabito_attackair_l03")),
		_ => macros::PLAY_SE(fighter, Hash40::new("se_murabito_attackair_l01")),
	}
}

pub fn install() {
	frame::install();
	status::install();
	acmd::install();
    the_csk_collection_api::add_chara_db_entry_info(
        the_csk_collection_api::CharacterDatabaseEntry {
                ui_chara_id: smash::hash40("ui_chara_toad"),
                fighter_kind: the_csk_collection_api::Hash40Type::Overwrite(smash::hash40("fighter_kind_murabito")), 
                fighter_kind_corps: the_csk_collection_api::Hash40Type::Overwrite(smash::hash40("fighter_kind_murabito")), 
                ui_series_id: the_csk_collection_api::Hash40Type::Overwrite(smash::hash40("ui_series_mario")), 
                fighter_type: the_csk_collection_api::Hash40Type::Overwrite(0x1353795179 /* Hash40 of fighter_type_normal */), 
                alt_chara_id: the_csk_collection_api::Hash40Type::Overwrite(0x2302D482A /* Hash40 of -1 */), 
                shop_item_tag: the_csk_collection_api::Hash40Type::Overwrite(0x5E1155EA7 /* Hash40 of sc-09 */), 
                name_id: the_csk_collection_api::StringType::Overwrite(the_csk_collection_api::CStrCSK::new("toad")), 
                exhibit_year: the_csk_collection_api::ShortType::Overwrite(1985), 
                exhibit_day_order: the_csk_collection_api::IntType::Overwrite(13102), 
                extra_flags: the_csk_collection_api::IntType::Overwrite(0), 
                ext_skill_page_num: the_csk_collection_api::SignedByteType::Overwrite(0), 
                skill_list_order: the_csk_collection_api::SignedByteType::Overwrite(83), 
                disp_order: the_csk_collection_api::SignedByteType::Optional(Some(83)), 
                save_no: the_csk_collection_api::SignedByteType::Overwrite(82),  
                chara_count: the_csk_collection_api::SignedByteType::Overwrite(1), 
                is_img_ext_skill_page0: the_csk_collection_api::BoolType::Overwrite(false), 
                is_img_ext_skill_page1: the_csk_collection_api::BoolType::Overwrite(false), 
                is_img_ext_skill_page2: the_csk_collection_api::BoolType::Overwrite(false), 
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
                is_article: the_csk_collection_api::BoolType::Overwrite(true), 
                has_multiple_face: the_csk_collection_api::BoolType::Overwrite(false), 
                result_pf0: the_csk_collection_api::BoolType::Overwrite(true), 
                result_pf1: the_csk_collection_api::BoolType::Overwrite(true), 
                result_pf2: the_csk_collection_api::BoolType::Overwrite(true), 
            color_num: the_csk_collection_api::UnsignedByteType::Overwrite(8),
            extra_index_maps: the_csk_collection_api::UnsignedByteMap::Overwrite(HashMap::from([
                    (0x915C075DE /* Hash40 of c00_index */, the_csk_collection_api::UnsignedByteType::Overwrite(0)), 
                    (0x9B3B77E6A /* Hash40 of c01_index */, the_csk_collection_api::UnsignedByteType::Overwrite(0)), 
                    (0x9825F64F7 /* Hash40 of c02_index */, the_csk_collection_api::UnsignedByteType::Overwrite(0)), 
                    (0x924286F43 /* Hash40 of c03_index */, the_csk_collection_api::UnsignedByteType::Overwrite(0)), 
                    (0x9E18F51CD /* Hash40 of c04_index */, the_csk_collection_api::UnsignedByteType::Overwrite(0)), 
                    (0x947F85A79 /* Hash40 of c05_index */, the_csk_collection_api::UnsignedByteType::Overwrite(5)), 
                    (0x9761040E4 /* Hash40 of c06_index */, the_csk_collection_api::UnsignedByteType::Overwrite(6)), 
                    (0x9D0674B50 /* Hash40 of c07_index */, the_csk_collection_api::UnsignedByteType::Overwrite(7)), 
                    (0x9E48F9289 /* Hash40 of n00_index */, the_csk_collection_api::UnsignedByteType::Overwrite(0)), 
                    (0x942F8993D /* Hash40 of n01_index */, the_csk_collection_api::UnsignedByteType::Overwrite(0)), 
                    (0x9731083A0 /* Hash40 of n02_index */, the_csk_collection_api::UnsignedByteType::Overwrite(0)), 
                    (0x9D5678814 /* Hash40 of n03_index */, the_csk_collection_api::UnsignedByteType::Overwrite(0)), 
                    (0x910C0B69A /* Hash40 of n04_index */, the_csk_collection_api::UnsignedByteType::Overwrite(0)), 
                    (0x9B6B7BD2E /* Hash40 of n05_index */, the_csk_collection_api::UnsignedByteType::Overwrite(5)), 
                    (0x9875FA7B3 /* Hash40 of n06_index */, the_csk_collection_api::UnsignedByteType::Overwrite(6)), 
                    (0x92128AC07 /* Hash40 of n07_index */, the_csk_collection_api::UnsignedByteType::Overwrite(7)), 
                (smash::hash40("color_start_index") /* Hash40 of color_start_index */, the_csk_collection_api::UnsignedByteType::Overwrite(120))
            ])),
            extra_hash_maps: the_csk_collection_api::Hash40Map::Overwrite(HashMap::from([
                    (0x1337FC912E /* Hash40 of characall_label_c00 */, the_csk_collection_api::Hash40Type::Overwrite(smash::hash40("vc_narration_characall_toad"))),
                    (0x1340FBA1B8 /* Hash40 of characall_label_c01 */, the_csk_collection_api::Hash40Type::Overwrite(smash::hash40("vc_narration_characall_toad"))),
                    (0x13D9F2F002 /* Hash40 of characall_label_c02 */, the_csk_collection_api::Hash40Type::Overwrite(smash::hash40("vc_narration_characall_toad"))),
                    (0x13AEF5C094 /* Hash40 of characall_label_c03 */, the_csk_collection_api::Hash40Type::Overwrite(smash::hash40("vc_narration_characall_toad"))),
                    (0x1330915537 /* Hash40 of characall_label_c04 */, the_csk_collection_api::Hash40Type::Overwrite(smash::hash40("vc_narration_characall_toad"))),
                    (0x13479665A1 /* Hash40 of characall_label_c05 */, the_csk_collection_api::Hash40Type::Overwrite(smash::hash40("vc_narration_characall_toadette"))),
                    (0x13DE9F341B /* Hash40 of characall_label_c06 */, the_csk_collection_api::Hash40Type::Overwrite(smash::hash40("vc_narration_characall_toadcaptain"))),
                    (0x13A998048D /* Hash40 of characall_label_c07 */, the_csk_collection_api::Hash40Type::Overwrite(smash::hash40("vc_narration_characall_toadsworth"))),
                    (0x1B8B13E500 /* Hash40 of characall_label_article_c00 */, the_csk_collection_api::Hash40Type::Overwrite(smash::hash40("vc_narration_characall_toad"))),
                    (0x1BFC14D596 /* Hash40 of characall_label_article_c01 */, the_csk_collection_api::Hash40Type::Overwrite(smash::hash40("vc_narration_characall_toad"))),
                    (0x1B651D842C /* Hash40 of characall_label_article_c02 */, the_csk_collection_api::Hash40Type::Overwrite(smash::hash40("vc_narration_characall_toad"))),
                    (0x1B121AB4BA /* Hash40 of characall_label_article_c03 */, the_csk_collection_api::Hash40Type::Overwrite(smash::hash40("vc_narration_characall_toad"))),
                    (0x1B8C7E2119 /* Hash40 of characall_label_article_c04 */, the_csk_collection_api::Hash40Type::Overwrite(smash::hash40("vc_narration_characall_toad"))),
                    (0x1BFB79118F /* Hash40 of characall_label_article_c05 */, the_csk_collection_api::Hash40Type::Overwrite(smash::hash40("vc_narration_characall_toadette"))),
                    (0x1B62704035 /* Hash40 of characall_label_article_c06 */, the_csk_collection_api::Hash40Type::Overwrite(smash::hash40("vc_narration_characall_toadcaptain"))),
                    (0x1B157770A3 /* Hash40 of characall_label_article_c07 */, the_csk_collection_api::Hash40Type::Overwrite(smash::hash40("vc_narration_characall_toadsworth"))),
                (0x160ab9eb98 /* Hash40 of original_ui_chara_hash */, the_csk_collection_api::Hash40Type::Overwrite(smash::hash40("ui_chara_murabito")) /* Hash40 of ui_chara_falco*/)
            ])),
            ..Default::default()
        },
    );
    the_csk_collection_api::add_chara_layout_db_entry_info(
            the_csk_collection_api::CharacterLayoutDatabaseEntry {
                ui_layout_id: smash::hash40("ui_chara_toad_00"), // Hash40 of ui_chara_aaa_00
                clone_from_ui_layout_id: Some(smash::hash40("ui_chara_murabito_00")), // Hash40 of ui_chara_falco_00
                ui_chara_id: the_csk_collection_api::Hash40Type::Overwrite(
                    smash::hash40("ui_chara_toad"), // Hash40 of ui_chara_aaa
                ),
                ..Default::default()
            },
    );
    the_csk_collection_api::add_bgm_db_entry_info(&the_csk_collection_api::BgmDatabaseRootEntry {
        ui_bgm_id: hash40("ui_bgm_z80_f_toad"),
        clone_from_ui_bgm_id: Some(hash40("ui_bgm_z66_f_murabito")),
        stream_set_id: the_csk_collection_api::Hash40Type::Overwrite(hash40("set_z80_f_toad")),
        ..Default::default()
    });

    the_csk_collection_api::add_stream_set_entry_info(&the_csk_collection_api::StreamSetEntry { 
        stream_set_id: hash40("set_z80_f_toad"),
        info0: the_csk_collection_api::Hash40Type::Overwrite(hash40("info_z80_f_toad")),
        ..Default::default()
    });

    the_csk_collection_api::add_assigned_info_entry_info(&the_csk_collection_api::AssignedInfoEntry { 
        info_id: hash40("info_z80_f_toad"),
        stream_id: the_csk_collection_api::Hash40Type::Overwrite(hash40("stream_z80_f_toad")),
        condition: the_csk_collection_api::Hash40Type::Overwrite(hash40("sound_condition_none")),
        condition_process: the_csk_collection_api::Hash40Type::Overwrite(hash40("sound_condition_process_add")),
        change_fadeout_frame: the_csk_collection_api::IntType::Overwrite(60),
        menu_change_fadeout_frame: the_csk_collection_api::IntType::Overwrite(60),
        ..Default::default()
    });

    the_csk_collection_api::add_stream_property_entry_info(&the_csk_collection_api::StreamPropertyEntry {
        stream_id: hash40("stream_z80_f_herobrine"),
        data_name0: the_csk_collection_api::StringType::Overwrite(the_csk_collection_api::CStrCSK::new("z80_f_toad")),
        ..Default::default()
    });

    the_csk_collection_api::add_new_bgm_property_entry(&smash_bgm_property::BgmPropertyEntry {
        stream_name: hash40::Hash40::new("z80_f_toad"),
        loop_start_ms: 0,
        loop_start_sample: 0,
        loop_end_ms: 0,
        loop_end_sample: 0,
        duration_ms: 7659,
        duration_sample: 359424 
    });

    the_csk_collection_api::set_fighter_jingle(hash40("ui_chara_toad"), "z80_f_toad");
} 
