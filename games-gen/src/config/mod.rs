use std::fs;

use crate::models::Endpoint;

pub fn get_endpoints() -> Vec<Endpoint> {
    let config_str = fs::read_to_string("games.toml").expect("Unable to read file");
    let config: toml::Value = toml::from_str(&config_str).expect("Unable to parse TOML");
    let nintendo_3ds: Endpoint = config["nintendo_3ds"].clone().try_into().unwrap();
    let nintendo_switch: Endpoint = config["nintendo_switch"].clone().try_into().unwrap();
    let wii_u: Endpoint = config["wii_u"].clone().try_into().unwrap();
    let xbox: Endpoint = config["xbox"].clone().try_into().unwrap();
    vec![nintendo_3ds, nintendo_switch, wii_u, xbox]
}