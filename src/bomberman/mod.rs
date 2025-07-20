mod status;
mod frame;
mod acmd;

static mut MAKE_NEW_BOMB : [bool; 8] = [false; 8];
static mut NEW_BOMB_X : [f32; 8] = [0.0; 8];
static mut NEW_BOMB_Y : [f32; 8] = [0.0; 8];
static mut EXPLODE_END_TIMER : [i32; 8] = [0; 8];
static mut NEUTRALB_CHARGE : [i32; 8] = [0; 8];
static mut NEUTRALB_MAX : i32 = 120;
static mut MIN_DISTANCE : f32 = 0.75;
static mut MAX_DISTANCE : f32 = 1.75;
static mut NEUTRALB_DIST : [f32; 8] = [1.0; 8];
static mut BOMB_TO_REMOVE : [bool; 8] = [false; 8];
static mut SIDEB_CATCH : [bool; 8] = [false; 8];
static mut FORCE_END : [bool; 8] = [false; 8];
static mut FALL_COUNT : [i32; 8] = [0; 8];
use std::collections::HashMap;


static mut EXPLODE : [bool; 8] = [false; 8];
static mut BOMB :  smash::phx::Vector3f =  smash::phx::Vector3f { x: 0.0, y: 10.5, z: 0.0 };

pub fn install() {
	frame::install();
	status::install();
	acmd::install();

    the_csk_collection_api::add_chara_db_entry_info(
        the_csk_collection_api::CharacterDatabaseEntry {
                ui_chara_id: smash::hash40("ui_chara_bomberman"),
                fighter_kind: the_csk_collection_api::Hash40Type::Overwrite(smash::hash40("fighter_kind_pacman")), 
                fighter_kind_corps: the_csk_collection_api::Hash40Type::Overwrite(smash::hash40("fighter_kind_pacman")), 
                ui_series_id: the_csk_collection_api::Hash40Type::Overwrite(smash::hash40("ui_series_bomberman")), 
                fighter_type: the_csk_collection_api::Hash40Type::Overwrite(0x1353795179 /* Hash40 of fighter_type_normal */), 
                alt_chara_id: the_csk_collection_api::Hash40Type::Overwrite(0x2302D482A /* Hash40 of -1 */), 
                shop_item_tag: the_csk_collection_api::Hash40Type::Overwrite(0x5E1155EA7 /* Hash40 of sc-09 */), 
                name_id: the_csk_collection_api::StringType::Overwrite(the_csk_collection_api::CStrCSK::new("bomberman")), 
                exhibit_year: the_csk_collection_api::ShortType::Overwrite(1983), 
                exhibit_day_order: the_csk_collection_api::IntType::Overwrite(13102), 
                extra_flags: the_csk_collection_api::IntType::Overwrite(0), 
                ext_skill_page_num: the_csk_collection_api::SignedByteType::Overwrite(0), 
                skill_list_order: the_csk_collection_api::SignedByteType::Overwrite(83), 
                disp_order: the_csk_collection_api::SignedByteType::Optional(Some(83)), 
                save_no: the_csk_collection_api::SignedByteType::Overwrite(83), 
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
                is_article: the_csk_collection_api::BoolType::Overwrite(false), 
                unk_0x112b7bb52a: the_csk_collection_api::BoolType::Overwrite(false), 
                result_pf0: the_csk_collection_api::BoolType::Overwrite(true), 
                result_pf1: the_csk_collection_api::BoolType::Overwrite(true), 
                result_pf2: the_csk_collection_api::BoolType::Overwrite(true), 
            color_num: the_csk_collection_api::UnsignedByteType::Overwrite(8),
            extra_index_maps: the_csk_collection_api::UnsignedByteMap::Overwrite(HashMap::from([
                (smash::hash40("color_start_index") /* Hash40 of color_start_index */, the_csk_collection_api::UnsignedByteType::Overwrite(120))
            ])),
            extra_hash_maps: the_csk_collection_api::Hash40Map::Overwrite(HashMap::from([
                    (0x1337FC912E /* Hash40 of characall_label_c00 */, the_csk_collection_api::Hash40Type::Overwrite(smash::hash40("vc_narration_characall_bomberman"))),
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
                (0x160ab9eb98 /* Hash40 of original_ui_chara_hash */, the_csk_collection_api::Hash40Type::Overwrite(smash::hash40("ui_chara_pacman")) /* Hash40 of ui_chara_falco*/)
            ])),
            ..Default::default()
        },
    );
    the_csk_collection_api::add_chara_layout_db_entry_info(
        the_csk_collection_api::CharacterLayoutDatabaseEntry {
            ui_layout_id: smash::hash40("ui_chara_bomberman_00"), // Hash40 of ui_chara_peppy_00
            clone_from_ui_layout_id: Some(smash::hash40("ui_chara_pacman_00")), // Hash40 of ui_chara_falco_00
            ui_chara_id: the_csk_collection_api::Hash40Type::Overwrite(
                smash::hash40("ui_chara_bomberman"), // Hash40 of ui_chara_peppy
            ),
            ..Default::default()
        },
    );
} 