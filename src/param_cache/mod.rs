// param_config::update_attribute_mul(_2) multiplies the game's *original*
// vanilla param value - it reads from a completely separate `attribute_muls`
// table that the native hook applies against the baked-in value, with no
// knowledge of whatever update_float_2/update_int_2 already set for that
// same param. Confirmed against param_config's actual hook
// (FighterParamModule::get_attribute_mul, called before get_float_param):
// once ANY attribute_mul entry matches a fighter's kind - whether
// registered under that kind specifically or under FIGHTER_KIND_ALL - the
// hook returns immediately and never even calls the float-lookup function.
// So a FIGHTER_KIND_ALL attribute_mul always shadows every character's own
// float/int override for that param, no matter what those overrides
// contain or when they were registered.
//
// This module used to also check param_config::PARAM_MANAGER directly to
// pick up overrides set by other param_config-based mods. That turned out
// to be unreliable: update_float_2/update_int_2/update_attribute_mul_2 are
// all #[no_mangle] extern "C" functions, which Skyline resolves globally
// across every loaded plugin - if another plugin also links param_config,
// calls to those functions can get redirected into that plugin's own
// compiled copy, writing into a manager we never see. PARAM_MANAGER itself
// isn't #[no_mangle] though, so reading it directly always sees our own
// plugin's private, separately-compiled copy - which then looks
// permanently empty regardless of how many writes actually went through
// the (possibly redirected) functions above. So: no more reads of
// PARAM_MANAGER. Everything here is driven by our own private BASELINES
// cache instead - a plain, non-exported static that only this module's own
// update_float_2/update_int_2 calls ever populate, so it's fully reliable,
// but only for overrides that went through this module (crate::param_cache::)
// specifically, not param_config:: directly or another mod's calls.
//
// update_attribute_mul multiplies against the *current* value instead of
// vanilla:
//   - For a specific kind, it looks up our own cache for that exact
//     (kind, slots, index), falling back to param_config::update_attribute_mul_2
//     (multiplies vanilla) if we never recorded a baseline for it.
//   - For FIGHTER_KIND_ALL specifically (reload_config_values' "apply to
//     everyone" calls), a single broadcast entry would shadow every
//     character-specific override in the game, as above. Instead: every
//     kind we have a recorded baseline for gets that value patched directly
//     (multiplied, no attribute_mul registered for that kind at all);
//     every real fighter kind we don't have a baseline for
//     (FIGHTER_KIND_BASE_HEAD..=FIGHTER_KIND_BASE_APPEND_TAIL - base
//     roster, Miis, and every DLC/echo fighter, a contiguous range
//     maintained by the smash crate itself so it stays correct as fighters
//     are added) gets the attribute_mul registered on its own specific
//     kind instead of FIGHTER_KIND_ALL, so it can never shadow a kind that
//     does have a baseline.
use once_cell::sync::Lazy;
use parking_lot::Mutex;
use smash::lib::lua_const::*;
use std::collections::{HashMap, HashSet};

#[derive(Clone, Copy)]
enum ParamValue {
    Int(i32),
    Float(f32),
}

type ParamKey = (i32, Vec<i32>, u64, u64);

static BASELINES: Lazy<Mutex<HashMap<ParamKey, ParamValue>>> = Lazy::new(|| Mutex::new(HashMap::new()));

pub fn update_float_2(kind: i32, slots: Vec<i32>, param: (u64, u64, f32)) {
    BASELINES.lock().insert((kind, slots.clone(), param.0, param.1), ParamValue::Float(param.2));
    param_config::update_float_2(kind, slots, param);
}

pub fn update_int_2(kind: i32, slots: Vec<i32>, param: (u64, u64, i32)) {
    BASELINES.lock().insert((kind, slots.clone(), param.0, param.1), ParamValue::Int(param.2));
    param_config::update_int_2(kind, slots, param);
}

pub fn update_attribute_mul(kind: i32, slots: Vec<i32>, param: (u64, u64, f32)) {
    let (type_hash, name_hash, multiplier) = param;

    if kind == *FIGHTER_KIND_ALL {
        apply_multiplier_to_all_kinds(slots, type_hash, name_hash, multiplier);
        return;
    }

    let key = (kind, slots.clone(), type_hash, name_hash);
    match BASELINES.lock().get(&key).copied() {
        Some(ParamValue::Float(base)) => {
            let new_value = base * multiplier;
            BASELINES.lock().insert(key, ParamValue::Float(new_value));
            param_config::update_float_2(kind, slots, (type_hash, name_hash, new_value));
        }
        Some(ParamValue::Int(base)) => {
            let new_value = (base as f32 * multiplier).round() as i32;
            BASELINES.lock().insert(key, ParamValue::Int(new_value));
            param_config::update_int_2(kind, slots, (type_hash, name_hash, new_value));
        }
        None => {
            param_config::update_attribute_mul_2(kind, slots, (type_hash, name_hash, multiplier));
        }
    }
}

fn apply_multiplier_to_all_kinds(default_slots: Vec<i32>, type_hash: u64, name_hash: u64, multiplier: f32) {
    let overridden: Vec<(i32, Vec<i32>, ParamValue)> = BASELINES.lock().iter()
        .filter(|((kind, _, th, nh), _)| *kind != *FIGHTER_KIND_ALL && *th == type_hash && *nh == name_hash)
        .map(|((kind, slots, _, _), value)| (*kind, slots.clone(), *value))
        .collect();
    let overridden_kinds: HashSet<i32> = overridden.iter().map(|(k, _, _)| *k).collect();

    for (kind, slots, value) in overridden {
        match value {
            ParamValue::Float(base) => {
                let new_value = base * multiplier;
                BASELINES.lock().insert((kind, slots.clone(), type_hash, name_hash), ParamValue::Float(new_value));
                param_config::update_float_2(kind, slots, (type_hash, name_hash, new_value));
            }
            ParamValue::Int(base) => {
                let new_value = (base as f32 * multiplier).round() as i32;
                BASELINES.lock().insert((kind, slots.clone(), type_hash, name_hash), ParamValue::Int(new_value));
                param_config::update_int_2(kind, slots, (type_hash, name_hash, new_value));
            }
        }
    }

    // Everyone else: register the attribute_mul on their own specific kind
    // instead of FIGHTER_KIND_ALL, so it can never shadow a kind that does
    // have a baseline (registering it under FIGHTER_KIND_ALL would match
    // and win for those kinds too, via the kind-OR-ALL check in
    // get_attribute_mul, regardless of what was just patched above).
    // Explicit slot numbers rather than the -1/has_all_slots sentinel -
    // 0..=256 generously covers real costume slots (including high-numbered
    // custom ones like Toad's 120s) without depending on that mechanism.
    let default_slots: Vec<i32> = if default_slots.contains(&-1) {
        (0..=256).collect()
    } else {
        default_slots
    };
    for kind in (*FIGHTER_KIND_BASE_HEAD)..=(*FIGHTER_KIND_BASE_APPEND_TAIL) {
        if overridden_kinds.contains(&kind) {
            continue;
        }
        param_config::update_attribute_mul_2(kind, default_slots.clone(), (type_hash, name_hash, multiplier));
    }
}
