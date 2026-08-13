mod status;
mod frame;
mod acmd;
use crate::util::*;
use smash::lib::lua_const::*;
use smash::hash40;

static FIGHTER_BOMBERMAN_INSTANCE_WORK_ID_FLAG_MAKE_NEW_BOMB : i32 = 0;
static FIGHTER_BOMBERMAN_INSTANCE_WORK_ID_FLOAT_NEW_BOMB_X : i32 = 1;
static FIGHTER_BOMBERMAN_INSTANCE_WORK_ID_FLOAT_NEW_BOMB_Y : i32 = 2;
static FIGHTER_BOMBERMAN_INSTANCE_WORK_ID_INT_EXPLODE_END_TIMER : i32 = 3;
static FIGHTER_BOMBERMAN_INSTANCE_WORK_ID_INT_NEUTRALB_CHARGE : i32 = 4;
static mut NEUTRALB_MAX : i32 = 120;
static mut MIN_DISTANCE : f32 = 0.75;
static mut MAX_DISTANCE : f32 = 1.75;
static FIGHTER_BOMBERMAN_INSTANCE_WORK_ID_FLOAT_NEUTRALB_DIST : i32 = 5;
static FIGHTER_BOMBERMAN_INSTANCE_WORK_ID_FLAG_BOMB_TO_REMOVE : i32 = 6;
static mut SIDEB_CATCH : [bool; 8] = [false; 8];
static FIGHTER_BOMBERMAN_INSTANCE_WORK_ID_FLAG_FORCE_END : i32 = 7;
static FIGHTER_BOMBERMAN_INSTANCE_WORK_ID_INT_FALL_COUNT : i32 = 8;
use std::collections::HashMap;

static FIGHTER_BOMBERMAN_INSTANCE_WORK_ID_FLAG_EXPLODE : i32 = 9;
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
                skill_list_order: the_csk_collection_api::SignedByteType::Overwrite(92), 
                disp_order: the_csk_collection_api::SignedByteType::Optional(Some(85)), 
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
            color_num: the_csk_collection_api::UnsignedByteType::Overwrite(get_costume_count("pacman","bomberman")),
            extra_index_maps: the_csk_collection_api::UnsignedByteMap::Overwrite(HashMap::from([ 
                (smash::hash40("color_start_index") /* Hash40 of color_start_index */, the_csk_collection_api::UnsignedByteType::Overwrite(get_lowest_marked_costume("pacman","bomberman")))
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
            ui_layout_id: smash::hash40("ui_chara_bomberman_00"), // Hash40 of ui_chara_aaa_00
            clone_from_ui_layout_id: Some(smash::hash40("ui_chara_pacman_00")), // Hash40 of ui_chara_falco_00
            ui_chara_id: the_csk_collection_api::Hash40Type::Overwrite(
                smash::hash40("ui_chara_bomberman"), // Hash40 of ui_chara_aaa
            ),
            ..Default::default()
        },
    );
    the_csk_collection_api::add_bgm_db_entry_info(&the_csk_collection_api::BgmDatabaseRootEntry {
        ui_bgm_id: hash40("ui_bgm_z73_f_bomberman"),
        clone_from_ui_bgm_id: Some(hash40("ui_bgm_z73_f_pacman")),
        stream_set_id: the_csk_collection_api::Hash40Type::Overwrite(hash40("set_z73_f_bomberman")),
        ..Default::default()
    });

    the_csk_collection_api::add_stream_set_entry_info(&the_csk_collection_api::StreamSetEntry { 
        stream_set_id: hash40("set_z73_f_bomberman"),
        info0: the_csk_collection_api::Hash40Type::Overwrite(hash40("info_z73_f_bomberman")),
        ..Default::default()
    });

    the_csk_collection_api::add_assigned_info_entry_info(&the_csk_collection_api::AssignedInfoEntry { 
        info_id: hash40("info_z73_f_bomberman"),
        stream_id: the_csk_collection_api::Hash40Type::Overwrite(hash40("stream_z73_f_bomberman")),
        condition: the_csk_collection_api::Hash40Type::Overwrite(hash40("sound_condition_none")),
        condition_process: the_csk_collection_api::Hash40Type::Overwrite(hash40("sound_condition_process_add")),
        change_fadeout_frame: the_csk_collection_api::IntType::Overwrite(60),
        menu_change_fadeout_frame: the_csk_collection_api::IntType::Overwrite(60),
        ..Default::default()
    });

    the_csk_collection_api::add_stream_property_entry_info(&the_csk_collection_api::StreamPropertyEntry {
        stream_id: hash40("stream_z73_f_bomberman"),
        data_name0: the_csk_collection_api::StringType::Overwrite(the_csk_collection_api::CStrCSK::new("z73_f_bomberman")),
        ..Default::default()
    });

    the_csk_collection_api::add_new_bgm_property_entry(&smash_bgm_property::BgmPropertyEntry {
        stream_name: hash40::Hash40::new("z73_f_bomberman"),
        loop_start_ms: 0,
        loop_start_sample: 0,
        loop_end_ms: 0,
        loop_end_sample: 0,
        duration_ms: 7659,
        duration_sample: 359424 
    });

    the_csk_collection_api::set_fighter_jingle(hash40("ui_chara_bomberman"), "z73_f_bomberman");


     crate::param_cache::update_float_2(*FIGHTER_KIND_PACMAN, get_marked_costumes("pacman","bomberman").into_iter().map(|x| x as i32).collect(), (smash::hash40("dash_speed"), 0, 1.92));
     crate::param_cache::update_float_2(*FIGHTER_KIND_PACMAN, get_marked_costumes("pacman","bomberman").into_iter().map(|x| x as i32).collect(), (smash::hash40("run_speed_max"), 0, 1.75));
     crate::param_cache::update_float_2(*FIGHTER_KIND_PACMAN, get_marked_costumes("pacman","bomberman").into_iter().map(|x| x as i32).collect(), (smash::hash40("walk_speed_max"), 0, 1.2));
     crate::param_cache::update_float_2(*FIGHTER_KIND_PACMAN, get_marked_costumes("pacman","bomberman").into_iter().map(|x| x as i32).collect(), (smash::hash40("ground_brake"), 0, 0.09));
     crate::param_cache::update_float_2(*FIGHTER_KIND_PACMAN, get_marked_costumes("pacman","bomberman").into_iter().map(|x| x as i32).collect(), (smash::hash40("jump_speed_x_mul"), 0, 1.1));
     crate::param_cache::update_float_2(*FIGHTER_KIND_PACMAN, get_marked_costumes("pacman","bomberman").into_iter().map(|x| x as i32).collect(), (smash::hash40("air_accel_x_mul"), 0, 0.06));
     crate::param_cache::update_float_2(*FIGHTER_KIND_PACMAN, get_marked_costumes("pacman","bomberman").into_iter().map(|x| x as i32).collect(), (smash::hash40("air_accel_x_add"), 0, 0.01));
     crate::param_cache::update_float_2(*FIGHTER_KIND_PACMAN, get_marked_costumes("pacman","bomberman").into_iter().map(|x| x as i32).collect(), (smash::hash40("air_speed_x_stable"), 0, 1.071));
     crate::param_cache::update_float_2(*FIGHTER_KIND_PACMAN, get_marked_costumes("pacman","bomberman").into_iter().map(|x| x as i32).collect(), (smash::hash40("air_speed_y_stable"), 0, 1.62));
     crate::param_cache::update_float_2(*FIGHTER_KIND_PACMAN, get_marked_costumes("pacman","bomberman").into_iter().map(|x| x as i32).collect(), (smash::hash40("air_accel_y"), 0, 0.09));
     crate::param_cache::update_float_2(*FIGHTER_KIND_PACMAN, get_marked_costumes("pacman","bomberman").into_iter().map(|x| x as i32).collect(), (smash::hash40("dive_speed_y"), 0, 2.592));
     crate::param_cache::update_float_2(*FIGHTER_KIND_PACMAN, get_marked_costumes("pacman","bomberman").into_iter().map(|x| x as i32).collect(), (smash::hash40("jump_y"), 0, 32.0));
     crate::param_cache::update_float_2(*FIGHTER_KIND_PACMAN, get_marked_costumes("pacman","bomberman").into_iter().map(|x| x as i32).collect(), (smash::hash40("mini_jump_y"), 0, 18.0));
     crate::param_cache::update_float_2(*FIGHTER_KIND_PACMAN, get_marked_costumes("pacman","bomberman").into_iter().map(|x| x as i32).collect(), (smash::hash40("jump_aerial_y"), 0, 32.0));
     crate::param_cache::update_float_2(*FIGHTER_KIND_PACMAN, get_marked_costumes("pacman","bomberman").into_iter().map(|x| x as i32).collect(), (smash::hash40("weight"), 0, 87.0));
     crate::param_cache::update_float_2(*FIGHTER_KIND_PACMAN, get_marked_costumes("pacman","bomberman").into_iter().map(|x| x as i32).collect(), (smash::hash40("landing_attack_air_frame_n"), 0, 8.0));
     crate::param_cache::update_float_2(*FIGHTER_KIND_PACMAN, get_marked_costumes("pacman","bomberman").into_iter().map(|x| x as i32).collect(), (smash::hash40("landing_attack_air_frame_f"), 0, 8.0));
     crate::param_cache::update_float_2(*FIGHTER_KIND_PACMAN, get_marked_costumes("pacman","bomberman").into_iter().map(|x| x as i32).collect(), (smash::hash40("landing_attack_air_frame_b"), 0, 9.0));
     crate::param_cache::update_float_2(*FIGHTER_KIND_PACMAN, get_marked_costumes("pacman","bomberman").into_iter().map(|x| x as i32).collect(), (smash::hash40("landing_attack_air_frame_hi"), 0, 10.0));
     crate::param_cache::update_float_2(*FIGHTER_KIND_PACMAN, get_marked_costumes("pacman","bomberman").into_iter().map(|x| x as i32).collect(), (smash::hash40("landing_attack_air_frame_lw"), 0, 9.0));
    crate::param_cache::update_int_2(*FIGHTER_KIND_PACMAN, get_marked_costumes("pacman","bomberman").into_iter().map(|x| x as i32).collect(), (smash::hash40("height"), 0, 20));
    crate::param_cache::update_int_2(*FIGHTER_KIND_PACMAN, get_marked_costumes("pacman","bomberman").into_iter().map(|x| x as i32).collect(), (smash::hash40("squat_walk_type"), 0, 0));
    crate::param_cache::update_int_2(-*WEAPON_KIND_PACMAN_FIREHYDRANT, get_marked_costumes("pacman","bomberman").into_iter().map(|x| x as i32).collect(), (smash::hash40("param_firehydrant"), smash::hash40("life"), 240));
    crate::param_cache::update_int_2(-*WEAPON_KIND_PACMAN_FIREHYDRANT, get_marked_costumes("pacman","bomberman").into_iter().map(|x| x as i32).collect(), (smash::hash40("param_firehydrant"), smash::hash40("max_shoot_num"), 0));
    crate::param_cache::update_int_2(-*WEAPON_KIND_PACMAN_FIREHYDRANT, get_marked_costumes("pacman","bomberman").into_iter().map(|x| x as i32).collect(), (smash::hash40("param_firehydrant"), smash::hash40("shoot_frame"), 999));
    crate::param_cache::update_int_2(-*WEAPON_KIND_PACMAN_FIREHYDRANT, get_marked_costumes("pacman","bomberman").into_iter().map(|x| x as i32).collect(), (smash::hash40("param_firehydrant"), smash::hash40("hp"), 1));
    crate::param_cache::update_int_2(-*WEAPON_KIND_PACMAN_FIREHYDRANT_WATER, get_marked_costumes("pacman","bomberman").into_iter().map(|x| x as i32).collect(), (smash::hash40("param_firehydrantwater"), smash::hash40("life"), 0));
     crate::param_cache::update_float_2(-*WEAPON_KIND_PACMAN_FIREHYDRANT_WATER, get_marked_costumes("pacman","bomberman").into_iter().map(|x| x as i32).collect(), (smash::hash40("param_firehydrantwater"), smash::hash40("speed"), 0.0));
} 