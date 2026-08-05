use serde_derive::Deserialize;
use std::fs;
use std::sync::{OnceLock, RwLock};


static mut CONFIG: Option<Config> = None;

const CONFIG_PATH: &str = "sd:/ultimate/ult-s/config.toml";

#[derive(Debug, Deserialize)]
pub struct Config {
    pub general: General,

    pub movement: Movement,

    pub attacks: Attacks,

    pub defense: Defense,

    pub stats: Stats,

    pub special: Special,
}

#[derive(Debug, Deserialize)]
pub struct General {
    pub sh_macro: u8,
    pub tap_buffer: u8,
    pub hold_buffer: u8,
    pub balloon: u8,
    pub hitstun: u8,
    pub hitlag: u8,
    pub ledges: u8,
    pub respawn_anim: u8,
}

#[derive(Debug, Deserialize)]
pub struct Movement {
    pub dash: u8,
    pub pivots: u8,
    pub llpc: u8,
    pub jump_accel: u8,
    pub hitfall: u8,
    pub agt: u8,
    pub djc: u8,
    pub footstool: u8,
    pub edge_cancel: u8,
    pub jump_cancel: u8,
}

#[derive(Debug, Deserialize)]
pub struct Attacks {
    pub jab_cancel: u8,
    pub dacus: u8,
    pub ac: u8,
    pub jcg: u8,
    pub grab: u8,
    pub lcancel: u8,
    pub special_cancel: u8,
}

#[derive(Debug, Deserialize)]
pub struct Defense {
    pub shield: u8,
    pub shield_health: u8,
    pub shieldstun: u8,
    pub airdodge: u8,
    pub dodge_stale: u8,
    pub shield_drop: u8,
    pub di: u8,
    pub sdi: u8,
    pub cc: u8,
    pub hitstun_cancel: u8,
    pub vertical_gravity: u8,
    pub no_tumble_di: u8,
    pub drift_di: u8,
}

#[derive(Debug, Deserialize)]
pub struct Stats {
    pub ll: u8,
    pub groundspeed: u8,
    pub airspeed: u8,
    pub airaccel: u8,
    pub traction: u8,
    pub fallspeed: u8,
    pub jump: u8,
    pub gravity: u8,
    pub weight: u8,
}


#[derive(Debug, Deserialize)]
pub struct Special {
    pub cancel_to_taunt: u8,
    pub parry_reflect: u8,
    pub taunt_cancel: u8,
    pub no_special_fall: u8,
    pub random_trip: u8,
    pub backdash_roll: u8,
}


pub fn load() -> Result<Config, String> {
    let contents = std::fs::read_to_string(CONFIG_PATH)
        .map_err(|e| format!("Failed to read config: {e}"))?;

    toml::from_str(&contents)
        .map_err(|e| format!("Failed to parse config: {e}"))
}

pub fn init() -> Result<(), String> {
    let config = load()?;

    unsafe {
        CONFIG = Some(config);
    }

    Ok(())
}

pub fn reload() -> Result<(), String> {
    let config = load()?;

    unsafe {
        CONFIG = Some(config);
    }

    Ok(())
}

pub fn get() -> &'static Config {
    unsafe {
        CONFIG
            .as_ref()
            .expect("Config has not been initialized")
    }
}