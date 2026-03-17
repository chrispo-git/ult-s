use std::any::{Any, TypeId};
use std::collections::HashMap;
use parking_lot::Mutex;
use once_cell::sync::Lazy;

type ComponentMap = HashMap<TypeId, Box<dyn Any + Send + Sync>>;

static PLAYER_STATE: Lazy<Mutex<[ComponentMap; 8]>> = Lazy::new(|| {
    let maps: [ComponentMap; 8] = [
        HashMap::with_capacity(100),
        HashMap::with_capacity(100),
        HashMap::with_capacity(100),
        HashMap::with_capacity(100),
        HashMap::with_capacity(100),
        HashMap::with_capacity(100),
        HashMap::with_capacity(100),
        HashMap::with_capacity(100),
    ];
    Mutex::new(maps)
});

pub fn edit_state<T, F, R>(entry_id: usize, f: F) -> R 
where 
    T: 'static + Default + Send + Sync,
    F: FnOnce(&mut T) -> R 
{
    let mut all_players = PLAYER_STATE.lock();
    let player_map = &mut all_players[entry_id % 8]; // Safety check for port bounds
    
    let component = player_map
        .entry(TypeId::of::<T>())
        .or_insert_with(|| Box::new(T::default()));
    
    let data = component.downcast_mut::<T>().expect("State Type Mismatch!");
    f(data)
}

#[macro_export]
macro_rules! with_state {
    ($entry:expr, $type:ty, $vars:ident, $body:block) => {
        $crate::state_manager::edit_state::<$type, _, _>($entry as usize, |$vars| {
            $body
        })
    };
}
#[macro_export]
macro_rules! set_state {
    ($entry:expr, $value:expr) => {
        $crate::state_manager::edit_state::<_, _, _>($entry as usize, |slot| {
            *slot = $value;
        })
    };
}
#[macro_export]
macro_rules! get_state {
    ($entry:expr, $type:ty) => {
        $crate::state_manager::edit_state::<$type, _, _>($entry as usize, |data| data.clone())
    };
}
#[macro_export]
macro_rules! warm_up_states {
    ($entry:expr, [$($t:ty),* $(,)?]) => {
        $(
            let _ = $crate::get_state!($entry, $t);
        )*
    };
}