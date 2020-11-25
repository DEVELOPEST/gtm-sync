use std::fs;
use crate::model::Config;

pub fn load(config_file: &String) -> Config {
    let content = fs::read_to_string(config_file).expect("Unable to read config!");
    return toml::from_str(&content).expect("Unable to deserialize config!");
}

pub fn save(config_file: &String, config: &Config) {
    let content = toml::to_string(config).expect("Unable to serialize config!");
    fs::write(config_file, content).expect("Unable to save config!");
}