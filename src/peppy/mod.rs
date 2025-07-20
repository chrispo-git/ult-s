mod status;
mod frame;
mod acmd;
pub static mut FIGHTER_FALCO_GENERATE_ARTICLE_MISSILE: i32 = 6;
use std::convert::TryInto;

static mut AIR_SHOT : [bool; 8] = [false; 8];
static mut HAS_DOWNB : [bool; 8] = [false; 8];
static mut DO_STALL : [bool; 8] = [false; 8];
static mut SUPER_LAUNCH : [bool; 8] = [false; 8];
static mut TETHER_EFFECTS : [Vec<u32>; 8] = [vec![], vec![], vec![], vec![], vec![], vec![], vec![], vec![]];

pub fn install() {
	frame::install();
	status::install();
	acmd::install();
    // This creates a redirected ui_chara_db entry with ui_chara_mario as a base
    the_csk_collection_api::add_chara_db_entry_info(
        the_csk_collection_api::CharacterDatabaseEntry {
            ui_chara_id: 0xea1b1c420, // Hash40 of ui_chara_peppy
            clone_from_ui_chara_id: Some(0xe41749f82), // Hash40 of ui_chara_falco
            name_id: the_csk_collection_api::StringType::Overwrite(String::from("peppy")),
            color_num: the_csk_collection_api::UnsignedByteType::Overwrite(8),
            extra_index_maps: the_csk_collection_api::UnsignedByteMap::Overwrite(HashMap::from([
                (smash::hash40("color_start_index") /* Hash40 of color_start_index */, the_csk_collection_api::UnsignedByteType::Overwrite(120))
            ])),
            extra_hash_maps: the_csk_collection_api::Hash40Map::Overwrite(HashMap::from([
                    (0x1337FC912E /* Hash40 of characall_label_c00 */, the_csk_collection_api::Hash40Type::Overwrite(smash::hash40("vc_narration_characall_peppy"))),
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
                (0x160ab9eb98 /* Hash40 of original_ui_chara_hash */, the_csk_collection_api::Hash40Type::Overwrite(0xe41749f82) /* Hash40 of ui_chara_falco*/)
            ])),
            ..Default::default()
        },
    );
    the_csk_collection_api::add_chara_layout_db_entry_info(
        the_csk_collection_api::CharacterLayoutDatabaseEntry {
            ui_layout_id: 0x114a8361d1, // Hash40 of ui_chara_peppy_00
            clone_from_ui_layout_id: Some(0x11223e5a9f), // Hash40 of ui_chara_falco_00
            ui_chara_id: the_csk_collection_api::Hash40Type::Overwrite(
                0xea1b1c420, // Hash40 of ui_chara_peppy
            ),
            ..Default::default()
        },
    );
}