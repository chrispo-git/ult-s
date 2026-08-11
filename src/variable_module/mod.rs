use std::collections::HashMap;
use smash::app::BattleObjectModuleAccessor;

static mut INT_VARS : Option<HashMap<(u32, i32), i32>> = None;
static mut FLOAT_VARS : Option<HashMap<(u32, i32), f32>> = None;
static mut FLAG_VARS : Option<HashMap<(u32, i32), bool>> = None;

#[inline(always)]
unsafe fn key(module_accessor: *mut BattleObjectModuleAccessor, id: i32) -> (u32, i32) {
    ((*module_accessor).battle_object_id, id)
}

pub struct VariableModule;

impl VariableModule {
    pub unsafe fn get_int(module_accessor: *mut BattleObjectModuleAccessor, id: i32) -> i32 {
        INT_VARS.get_or_insert_with(HashMap::new)
            .get(&key(module_accessor, id))
            .copied()
            .unwrap_or(0)
    }

    pub unsafe fn set_int(module_accessor: *mut BattleObjectModuleAccessor, value: i32, id: i32) {
        INT_VARS.get_or_insert_with(HashMap::new)
            .insert(key(module_accessor, id), value);
    }

    pub unsafe fn inc_int(module_accessor: *mut BattleObjectModuleAccessor, id: i32) {
        let value = Self::get_int(module_accessor, id) + 1;
        Self::set_int(module_accessor, value, id);
    }

    pub unsafe fn dec_int(module_accessor: *mut BattleObjectModuleAccessor, id: i32) {
        let value = Self::get_int(module_accessor, id) - 1;
        Self::set_int(module_accessor, value, id);
    }

    pub unsafe fn get_float(module_accessor: *mut BattleObjectModuleAccessor, id: i32) -> f32 {
        FLOAT_VARS.get_or_insert_with(HashMap::new)
            .get(&key(module_accessor, id))
            .copied()
            .unwrap_or(0.0)
    }

    pub unsafe fn set_float(module_accessor: *mut BattleObjectModuleAccessor, value: f32, id: i32) {
        FLOAT_VARS.get_or_insert_with(HashMap::new)
            .insert(key(module_accessor, id), value);
    }

    pub unsafe fn is_flag(module_accessor: *mut BattleObjectModuleAccessor, id: i32) -> bool {
        FLAG_VARS.get_or_insert_with(HashMap::new)
            .get(&key(module_accessor, id))
            .copied()
            .unwrap_or(false)
    }

    pub unsafe fn set_flag(module_accessor: *mut BattleObjectModuleAccessor, value: bool, id: i32) {
        FLAG_VARS.get_or_insert_with(HashMap::new)
            .insert(key(module_accessor, id), value);
    }

    pub unsafe fn on_flag(module_accessor: *mut BattleObjectModuleAccessor, id: i32) {
        Self::set_flag(module_accessor, true, id);
    }

    pub unsafe fn off_flag(module_accessor: *mut BattleObjectModuleAccessor, id: i32) {
        Self::set_flag(module_accessor, false, id);
    }

    pub unsafe fn clear(module_accessor: *mut BattleObjectModuleAccessor) {
        let id = (*module_accessor).battle_object_id;
        if let Some(m) = INT_VARS.as_mut() { m.retain(|k, _| k.0 != id); }
        if let Some(m) = FLOAT_VARS.as_mut() { m.retain(|k, _| k.0 != id); }
        if let Some(m) = FLAG_VARS.as_mut() { m.retain(|k, _| k.0 != id); }
    }
}
