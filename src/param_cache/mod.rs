use once_cell::sync::Lazy;
use parking_lot::Mutex;
use std::collections::HashMap;

pub fn update_float_2(kind: i32, slots: Vec<i32>, param: (u64, u64, f32)) {
    param_config::update_float_2(kind, slots, param);
}

pub fn update_int_2(kind: i32, slots: Vec<i32>, param: (u64, u64, i32)) {
    param_config::update_int_2(kind, slots, param);
}

static GLOBAL_MULTIPLIERS: Lazy<Mutex<HashMap<(u64, u64), f32>>> = Lazy::new(|| Mutex::new(HashMap::new()));

pub fn set_global_multiplier(type_hash: u64, name_hash: u64, multiplier: f32) {
    GLOBAL_MULTIPLIERS.lock().insert((type_hash, name_hash), multiplier);
}

static HOOK_INT_OFFSET: usize = 0x4e53a0;
static HOOK_FLOAT_OFFSET: usize = 0x4e53e0;
static HOOK_INT64_OFFSET: usize = 0x4e53b0;

#[skyline::hook(offset = HOOK_INT_OFFSET)]
unsafe fn ults_get_param_int_hook(module: u64, param_type: u64, param_hash: u64) -> i32 {
    let original_value = original!()(module, param_type, param_hash);
    match GLOBAL_MULTIPLIERS.lock().get(&(param_type, param_hash)).copied() {
        Some(mult) => (original_value as f32 * mult).round() as i32,
        None => original_value,
    }
}

#[skyline::hook(offset = HOOK_FLOAT_OFFSET)]
unsafe fn ults_get_param_float_hook(module: u64, param_type: u64, param_hash: u64) -> f32 {
    let original_value = original!()(module, param_type, param_hash);
    match GLOBAL_MULTIPLIERS.lock().get(&(param_type, param_hash)).copied() {
        Some(mult) => original_value * mult,
        None => original_value,
    }
}

#[skyline::hook(offset = HOOK_INT64_OFFSET)]
unsafe fn ults_get_param_int64_hook(module: u64, param_type: u64, param_hash: u64) -> i64 {
    let original_value = original!()(module, param_type, param_hash);
    match GLOBAL_MULTIPLIERS.lock().get(&(param_type, param_hash)).copied() {
        Some(mult) => (original_value as f32 * mult).round() as i64,
        None => original_value,
    }
}

pub fn install() {
    skyline::install_hooks!(ults_get_param_int_hook, ults_get_param_float_hook, ults_get_param_int64_hook);
}
