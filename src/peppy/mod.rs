mod status;
mod frame;
mod acmd;
use crate::util::*;
pub static mut FIGHTER_FALCO_GENERATE_ARTICLE_MISSILE: i32 = 6;
use std::convert::TryInto;
use std::collections::HashMap;
use smash::lib::lua_const::*;
use smash::hash40;

static mut AIR_SHOT : [bool; 8] = [false; 8];
static mut HAS_DOWNB : [bool; 8] = [false; 8];
static mut DO_STALL : [bool; 8] = [false; 8];
static mut SUPER_LAUNCH : [bool; 8] = [false; 8];
static mut TETHER_EFFECTS : [Vec<u32>; 8] = [vec![], vec![], vec![], vec![], vec![], vec![], vec![], vec![]];

pub fn install() {
	frame::install();
	status::install();
	acmd::install();

    the_csk_collection_api::add_chara_db_entry_info(
        the_csk_collection_api::CharacterDatabaseEntry {
                ui_chara_id: smash::hash40("ui_chara_peppy"),
                fighter_kind: the_csk_collection_api::Hash40Type::Overwrite(smash::hash40("fighter_kind_falco")), 
                fighter_kind_corps: the_csk_collection_api::Hash40Type::Overwrite(smash::hash40("fighter_kind_falco")), 
                ui_series_id: the_csk_collection_api::Hash40Type::Overwrite(smash::hash40("ui_series_starfox")), 
                fighter_type: the_csk_collection_api::Hash40Type::Overwrite(0x1353795179 /* Hash40 of fighter_type_normal */), 
                alt_chara_id: the_csk_collection_api::Hash40Type::Overwrite(0x2302D482A /* Hash40 of -1 */), 
                shop_item_tag: the_csk_collection_api::Hash40Type::Overwrite(0x5E1155EA7 /* Hash40 of sc-09 */), 
                name_id: the_csk_collection_api::StringType::Overwrite(the_csk_collection_api::CStrCSK::new("peppy")), 
                exhibit_year: the_csk_collection_api::ShortType::Overwrite(1992), 
                exhibit_day_order: the_csk_collection_api::IntType::Overwrite(13102), 
                extra_flags: the_csk_collection_api::IntType::Overwrite(0), 
                ext_skill_page_num: the_csk_collection_api::SignedByteType::Overwrite(0), 
                skill_list_order: the_csk_collection_api::SignedByteType::Overwrite(93), 
                disp_order: the_csk_collection_api::SignedByteType::Optional(Some(86)), 
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
                is_article: the_csk_collection_api::BoolType::Overwrite(false), 
                has_multiple_face: the_csk_collection_api::BoolType::Overwrite(false), 
                result_pf0: the_csk_collection_api::BoolType::Overwrite(true), 
                result_pf1: the_csk_collection_api::BoolType::Overwrite(true), 
                result_pf2: the_csk_collection_api::BoolType::Overwrite(true), 
            color_num: the_csk_collection_api::UnsignedByteType::Overwrite(get_costume_count("falco","peppy")),
            extra_index_maps: the_csk_collection_api::UnsignedByteMap::Overwrite(HashMap::from([ 
                (smash::hash40("color_start_index") /* Hash40 of color_start_index */, the_csk_collection_api::UnsignedByteType::Overwrite(get_lowest_marked_costume("falco","peppy")))
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
                (0x160ab9eb98 /* Hash40 of original_ui_chara_hash */, the_csk_collection_api::Hash40Type::Overwrite(smash::hash40("ui_chara_falco")) /* Hash40 of ui_chara_falco*/)
            ])),
            ..Default::default()
        },
    );
    the_csk_collection_api::add_chara_layout_db_entry_info(
        the_csk_collection_api::CharacterLayoutDatabaseEntry {
            ui_layout_id: smash::hash40("ui_chara_peppy_00"), // Hash40 of ui_chara_peppy_00
            clone_from_ui_layout_id: Some(smash::hash40("ui_chara_falco_00")), // Hash40 of ui_chara_falco_00
            ui_chara_id: the_csk_collection_api::Hash40Type::Overwrite(
                smash::hash40("ui_chara_peppy"), // Hash40 of ui_chara_peppy
            ),
            ..Default::default()
        },
    );
    param_config::update_float_2(*FIGHTER_KIND_FALCO, get_marked_costumes("falco","peppy"), (smash::hash40("dash_speed"), 0, 1.92));
    param_config::update_float_2(*FIGHTER_KIND_FALCO, get_marked_costumes("falco","peppy"), (smash::hash40("run_speed_max"), 0, 1.65));
    param_config::update_float_2(*FIGHTER_KIND_FALCO, get_marked_costumes("falco","peppy"), (smash::hash40("walk_speed_max"), 0, 1.05));
    param_config::update_float_2(*FIGHTER_KIND_FALCO, get_marked_costumes("falco","peppy"), (smash::hash40("ground_brake"), 0, 0.08));
    param_config::update_float_2(*FIGHTER_KIND_FALCO, get_marked_costumes("falco","peppy"), (smash::hash40("jump_speed_x_mul"), 0, 0.8));
    param_config::update_float_2(*FIGHTER_KIND_FALCO, get_marked_costumes("falco","peppy"), (smash::hash40("air_accel_x_mul"), 0, 0.04));
    param_config::update_float_2(*FIGHTER_KIND_FALCO, get_marked_costumes("falco","peppy"), (smash::hash40("air_accel_x_add"), 0, 0.01));
    param_config::update_float_2(*FIGHTER_KIND_FALCO, get_marked_costumes("falco","peppy"), (smash::hash40("air_speed_x_stable"), 0, 1.09));
    param_config::update_float_2(*FIGHTER_KIND_FALCO, get_marked_costumes("falco","peppy"), (smash::hash40("air_speed_y_stable"), 0, 1.8));
    param_config::update_float_2(*FIGHTER_KIND_FALCO, get_marked_costumes("falco","peppy"), (smash::hash40("air_accel_y"), 0, 0.14));
    param_config::update_float_2(*FIGHTER_KIND_FALCO, get_marked_costumes("falco","peppy"), (smash::hash40("dive_speed_y"), 0, 2.88));
    param_config::update_float_2(*FIGHTER_KIND_FALCO, get_marked_costumes("falco","peppy"), (smash::hash40("jump_initial_y"), 0, 17.6));
    param_config::update_float_2(*FIGHTER_KIND_FALCO, get_marked_costumes("falco","peppy"), (smash::hash40("jump_y"), 0, 32.5));
    param_config::update_float_2(*FIGHTER_KIND_FALCO, get_marked_costumes("falco","peppy"), (smash::hash40("mini_jump_y"), 0, 16.0));
    param_config::update_float_2(*FIGHTER_KIND_FALCO, get_marked_costumes("falco","peppy"), (smash::hash40("jump_aerial_y"), 0, 32.5));
    param_config::update_float_2(*FIGHTER_KIND_FALCO, get_marked_costumes("falco","peppy"), (smash::hash40("weight"), 0, 92.0));
    param_config::update_float_2(*FIGHTER_KIND_FALCO, get_marked_costumes("falco","peppy"), (smash::hash40("landing_attack_air_frame_n"), 0, 8.0));
    param_config::update_float_2(*FIGHTER_KIND_FALCO, get_marked_costumes("falco","peppy"), (smash::hash40("landing_attack_air_frame_f"), 0, 13.0));
    param_config::update_float_2(*FIGHTER_KIND_FALCO, get_marked_costumes("falco","peppy"), (smash::hash40("landing_attack_air_frame_b"), 0, 11.0));
    param_config::update_float_2(*FIGHTER_KIND_FALCO, get_marked_costumes("falco","peppy"), (smash::hash40("landing_attack_air_frame_hi"), 0, 8.0));
    param_config::update_float_2(*FIGHTER_KIND_FALCO, get_marked_costumes("falco","peppy"), (smash::hash40("landing_attack_air_frame_lw"), 0, 10.0));
    param_config::update_float_2(*FIGHTER_KIND_FALCO, get_marked_costumes("falco","peppy"), (smash::hash40("landing_frame"), 0, 4.0));
    param_config::update_float_2(*FIGHTER_KIND_FALCO, get_marked_costumes("falco","peppy"), (smash::hash40("shield_radius"), 0, 11.5));
    param_config::update_int_2(*FIGHTER_KIND_FALCO, get_marked_costumes("falco","peppy"), (smash::hash40("wall_jump_type"), 0, 1));
    param_config::update_float_2(*FIGHTER_KIND_FALCO, get_marked_costumes("falco","peppy"), (smash::hash40("param_special_hi"), smash::hash40("fire_rush_speed"), 3.9));
    param_config::update_int_2(*FIGHTER_KIND_FALCO, get_marked_costumes("falco","peppy"), (smash::hash40("param_special_hi"), smash::hash40("fire_rush_brake_frame"), 6));
    param_config::update_float_2(*FIGHTER_KIND_FALCO, get_marked_costumes("falco","peppy"), (smash::hash40("param_special_hi"), smash::hash40("fire_rush_brake"), 0.09));

}