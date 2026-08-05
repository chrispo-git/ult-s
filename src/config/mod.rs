use arc_swap::ArcSwap;
use serde::Deserialize;
use std::fs;
use std::sync::{Arc, OnceLock};

const CONFIG_PATH: &str = "sd:/ultimate/ult-s/config.toml";
static CONFIG: OnceLock<ArcSwap<Config>> = OnceLock::new();

#[derive(Debug, Deserialize)]
pub struct Config {
    #[serde(rename = "General")]
    pub general: General,

    #[serde(rename = "Movement")]
    pub movement: Movement,

    #[serde(rename = "Attacks")]
    pub attacks: Attacks,

    #[serde(rename = "Defense")]
    pub defense: Defense,

    #[serde(rename = "Special")]
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
}

#[derive(Debug, Deserialize)]
pub struct Attacks {
    pub jab_cancel: u8,
    pub dacus: u8,
    pub ac: u8,
    pub jcg: u8,
    pub grab: u8,
    pub lcancel: u8,
    pub ll: u8,
    pub special_cancel: u8,
}

#[derive(Debug, Deserialize)]
pub struct Defense {
    pub shield: u8,
    pub airdodge: u8,
    pub dodge_stale: u8,
    pub shield_drop: u8,
    pub di: u8,
    pub sdi: u8,
    pub cc: u8,
    pub hitstun_cancel: u8,
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


fn load_from_file() -> Result<Config, Box<dyn std::error::Error>> {
    let contents = fs::read_to_string(CONFIG_PATH)?;
    let config = toml::from_str(&contents)?;

    Ok(config)
}

pub fn init() -> Result<(), Box<dyn std::error::Error>> {
    let config = load_from_file()?;

    CONFIG
        .set(ArcSwap::from_pointee(config))
        .map_err(|_| "Config already initialized")?;

    Ok(())
}

pub fn reload() -> Result<(), Box<dyn std::error::Error>> {
    let config = load_from_file()?;

    CONFIG
        .get()
        .ok_or("Config not initialized")?
        .store(Arc::new(config));

    Ok(())
}

pub fn get() -> Arc<Config> {
    CONFIG
        .get()
        .expect("Config not initialized")
        .load_full()
}
```
