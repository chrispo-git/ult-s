use once_cell::sync::Lazy;
use parking_lot::Mutex;
use std::collections::HashMap;

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
    let key = (kind, slots.clone(), param.0, param.1);
    let multiplier = param.2;
    let cached = BASELINES.lock().get(&key).copied();

    match cached {
        Some(ParamValue::Float(base)) => {
            let new_value = base * multiplier;
            BASELINES.lock().insert(key, ParamValue::Float(new_value));
            param_config::update_float_2(kind, slots, (param.0, param.1, new_value));
        }
        Some(ParamValue::Int(base)) => {
            let new_value = (base as f32 * multiplier).round() as i32;
            BASELINES.lock().insert(key, ParamValue::Int(new_value));
            param_config::update_int_2(kind, slots, (param.0, param.1, new_value));
        }
        None => {
            param_config::update_attribute_mul_2(kind, slots, param);
        }
    }
}
