use serde_derive::Deserialize;
use std::fs;
use std::cell::UnsafeCell;


static CONFIG: ConfigCell = ConfigCell(UnsafeCell::new(None));
struct ConfigCell(UnsafeCell<Option<Config>>);

unsafe impl Sync for ConfigCell {}
const CONFIG_PATH: &str = "sd:/ultimate/ult-s/config.toml";

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    pub general: General,

    pub movement: Movement,

    pub attacks: Attacks,

    pub defense: Defense,

    pub stats: Stats,

    pub special: Special,
} 

#[derive(Debug, Deserialize, Clone)]
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

#[derive(Debug, Deserialize, Clone)]
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

#[derive(Debug, Deserialize, Clone)]
pub struct Attacks {
    pub jab_cancel: u8,
    pub dacus: u8,
    pub ac: u8,
    pub jcg: u8,
    pub grab: u8,
    pub lcancel: u8,
    pub special_cancel: u8,
}

#[derive(Debug, Deserialize, Clone)]
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

#[derive(Debug, Deserialize, Clone)]
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


#[derive(Debug, Deserialize, Clone)]
pub struct Special {
    pub cancel_to_taunt: u8,
    pub parry_reflect: u8,
    pub taunt_cancel: u8,
    pub no_special_fall: u8,
    pub random_trip: u8,
    pub backdash_roll: u8,
}


pub fn load() -> Result<Config, String> {
    let contents = fs::read_to_string(CONFIG_PATH)
        .map_err(|e| format!("Failed to read config: {e}"))?;

    toml::from_str(&contents) 
        .map_err(|e| format!("Failed to parse config: {e}"))
}
pub fn init() -> Result<(), String> {
    println!("===== CONFIG INIT =====");
    let config = load()?;


    println!("===== CONFIG LOADED =====");

    println!("[General]");
    println!("sh_macro: {}", config.general.sh_macro);
    println!("tap_buffer: {}", config.general.tap_buffer);
    println!("hold_buffer: {}", config.general.hold_buffer);
    println!("balloon: {}", config.general.balloon);
    println!("hitstun: {}", config.general.hitstun);
    println!("hitlag: {}", config.general.hitlag);
    println!("ledges: {}", config.general.ledges);
    println!("respawn_anim: {}", config.general.respawn_anim);

    println!("[Movement]");
    println!("dash: {}", config.movement.dash);
    println!("pivots: {}", config.movement.pivots);
    println!("llpc: {}", config.movement.llpc);
    println!("jump_accel: {}", config.movement.jump_accel);
    println!("hitfall: {}", config.movement.hitfall);
    println!("agt: {}", config.movement.agt);
    println!("djc: {}", config.movement.djc);
    println!("footstool: {}", config.movement.footstool);
    println!("edge_cancel: {}", config.movement.edge_cancel);
    println!("jump_cancel: {}", config.movement.jump_cancel);

    println!("[Attacks]");
    println!("jab_cancel: {}", config.attacks.jab_cancel);
    println!("dacus: {}", config.attacks.dacus);
    println!("ac: {}", config.attacks.ac);
    println!("jcg: {}", config.attacks.jcg);
    println!("grab: {}", config.attacks.grab);
    println!("lcancel: {}", config.attacks.lcancel);
    println!("special_cancel: {}", config.attacks.special_cancel);

    println!("[Defense]");
    println!("shield: {}", config.defense.shield);
    println!("shield_health: {}", config.defense.shield_health);
    println!("shieldstun: {}", config.defense.shieldstun);
    println!("airdodge: {}", config.defense.airdodge);
    println!("dodge_stale: {}", config.defense.dodge_stale);
    println!("shield_drop: {}", config.defense.shield_drop);
    println!("di: {}", config.defense.di);
    println!("sdi: {}", config.defense.sdi);
    println!("cc: {}", config.defense.cc);
    println!("hitstun_cancel: {}", config.defense.hitstun_cancel);
    println!("vertical_gravity: {}", config.defense.vertical_gravity);
    println!("no_tumble_di: {}", config.defense.no_tumble_di);
    println!("drift_di: {}", config.defense.drift_di);

    println!("[Stats]");
    println!("ll: {}", config.stats.ll);
    println!("groundspeed: {}", config.stats.groundspeed);
    println!("airspeed: {}", config.stats.airspeed);
    println!("airaccel: {}", config.stats.airaccel);
    println!("traction: {}", config.stats.traction);
    println!("fallspeed: {}", config.stats.fallspeed);
    println!("jump: {}", config.stats.jump);
    println!("gravity: {}", config.stats.gravity);
    println!("weight: {}", config.stats.weight);

    println!("[Special]");
    println!("cancel_to_taunt: {}", config.special.cancel_to_taunt);
    println!("parry_reflect: {}", config.special.parry_reflect);
    println!("taunt_cancel: {}", config.special.taunt_cancel);
    println!("no_special_fall: {}", config.special.no_special_fall);
    println!("random_trip: {}", config.special.random_trip);
    println!("backdash_roll: {}", config.special.backdash_roll);

    println!("===== END CONFIG =====");

    *unsafe { &mut *CONFIG.0.get() } = Some(config);

    println!("CONFIG STORED");

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