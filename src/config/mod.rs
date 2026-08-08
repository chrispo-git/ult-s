use serde_derive::Deserialize;
use std::fs;
use std::cell::UnsafeCell;
use std::hash::{Hash, Hasher};

static CONFIG: ConfigCell = ConfigCell(UnsafeCell::new(None));
struct ConfigCell(UnsafeCell<Option<Config>>);

unsafe impl Sync for ConfigCell {}
const CONFIG_PATH: &str = "sd:/ultimate/ult-s/config.toml";

#[derive(Debug, Deserialize, Clone, Hash)]
pub struct Config {
    pub general: General,

    pub movement: Movement,

    pub attacks: Attacks,

    pub defense: Defense,

    pub stats: Stats,

    pub special: Special,
} 

#[derive(Debug, Deserialize, Clone, Hash)]
pub struct General {
    pub sh_macro: u8,
    pub hold_buffer: u8,
    pub hitstun: u8,
    pub hitlag: u8,
    pub ledges: u8,
    pub respawn_anim: u8,
}

#[derive(Debug, Deserialize, Clone, Hash)]
pub struct Movement {
    pub dash: u8,
    pub moonwalk: u8,
    pub pivots: u8,
    pub llpc: u8,
    pub jump_accel: u8,
    pub hitfall: u8,
    pub agt: u8,
    pub djc: u8,
    pub footstool: u8,
    pub edge_cancel: u8,
    pub jump_cancel: u8,
    pub g2a: u8,
}

#[derive(Debug, Deserialize, Clone, Hash)]
pub struct Attacks {
    pub jab_cancel: u8,
    pub dacus: u8,
    pub ac: u8,
    pub jcg: u8,
    pub grab: u8,
    pub lcancel: u8,
    pub special_cancel: u8,
}

#[derive(Debug, Deserialize, Clone, Hash)]
pub struct Defense {
    pub shield: u8,
    pub shield_health: u8,
    pub shieldstun: u8,
    pub airdodge: u8,
    pub shield_drop: u8,
    pub di: u8,
    pub hitstun_cancel: u8,
    pub vertical_gravity: u8,
    pub no_tumble_di: u8,
    pub drift_di: u8,
}

#[derive(Debug, Deserialize, Clone, Hash)]
pub struct Stats {
    pub ll: u8,
    pub initial_dash: u8,
    pub runspeed: u8,
    pub walkspeed: u8,
    pub airspeed: u8,
    pub airaccel: u8,
    pub traction: u8,
    pub fallspeed: u8,
    pub jump: u8,
    pub gravity: u8,
    pub weight: u8,
}


#[derive(Debug, Deserialize, Clone, Hash)]
pub struct Special {
    pub cancel_to_taunt: u8,
    pub parry_reflect: u8,
    pub taunt_cancel: u8,
    pub no_special_fall: u8,
    pub random_trip: u8,
}

impl Config {
    pub fn to_js(&self) -> String {
        format!(
            r#"var config_map = {{
    "sh_macro": {},
    "hold_buffer": {},
    "hitstun": {},
    "hitlag": {},
    "ledges": {},
    "respawn_anim": {},
    "dash": {},
    "moonwalk": {},
    "pivots": {},
    "llpc": {},
    "jump_accel": {},
    "hitfall": {},
    "agt": {},
    "djc": {},
    "footstool": {},
    "edge_cancel": {},
    "jump_cancel": {},
    "g2a": {},
    "jab_cancel": {},
    "dacus": {},
    "ac": {},
    "jcg": {},
    "grab": {},
    "lcancel": {},
    "special_cancel": {},
    "shield": {},
    "shield_health": {},
    "shieldstun": {},
    "airdodge": {},
    "shield_drop": {},
    "di": {},
    "no_tumble_di": {},
    "drift_di": {},
    "hitstun_cancel": {},
    "vertical_gravity": {},
    "ll": {},
    "initial_dash": {},
    "runspeed": {},
    "walkspeed": {},
    "airspeed": {},
    "airaccel": {},
    "traction": {},
    "fallspeed": {},
    "jump": {},
    "gravity": {},
    "weight": {},
    "cancel_to_taunt": {},
    "parry_reflect": {},
    "taunt_cancel": {},
    "no_special_fall": {},
    "random_trip": {}
}};"#,
            self.general.sh_macro,
            self.general.hold_buffer,
            self.general.hitstun,
            self.general.hitlag,
            self.general.ledges,
            self.general.respawn_anim,
            self.movement.dash,
            self.movement.moonwalk,
            self.movement.pivots,
            self.movement.llpc,
            self.movement.jump_accel,
            self.movement.hitfall,
            self.movement.agt,
            self.movement.djc,
            self.movement.footstool,
            self.movement.edge_cancel,
            self.movement.jump_cancel,
            self.movement.g2a,
            self.attacks.jab_cancel,
            self.attacks.dacus,
            self.attacks.ac,
            self.attacks.jcg,
            self.attacks.grab,
            self.attacks.lcancel,
            self.attacks.special_cancel,
            self.defense.shield,
            self.defense.shield_health,
            self.defense.shieldstun,
            self.defense.airdodge,
            self.defense.shield_drop,
            self.defense.di,
            self.defense.no_tumble_di,
            self.defense.drift_di,
            self.defense.hitstun_cancel,
            self.defense.vertical_gravity,
            self.stats.ll,
            self.stats.initial_dash,
            self.stats.runspeed,
            self.stats.walkspeed,
            self.stats.airspeed,
            self.stats.airaccel,
            self.stats.traction,
            self.stats.fallspeed,
            self.stats.jump,
            self.stats.gravity,
            self.stats.weight,
            self.special.cancel_to_taunt,
            self.special.parry_reflect,
            self.special.taunt_cancel,
            self.special.no_special_fall,
            self.special.random_trip
        )
    }
}

pub fn load() -> Result<Config, String> {
    let contents = fs::read_to_string(CONFIG_PATH)
        .map_err(|e| format!("Failed to read config: {e}"))?;

    toml::from_str(&contents) 
        .map_err(|e| format!("Failed to parse config: {e}"))
}
pub fn init() -> Result<(), String> {
    let config = load()?;
    *unsafe { &mut *CONFIG.0.get() } = Some(config);

    Ok(())
}

pub fn reload() -> Result<(), String> {
    *unsafe { &mut *CONFIG.0.get() } = Some(load()?);
    Ok(())
}

pub fn get() -> Config {
    unsafe {
        (*CONFIG.0.get())
            .as_ref()
            .expect("Config has not been initialized")
            .clone()
    }
}