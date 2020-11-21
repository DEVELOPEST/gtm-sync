
use serde_derive::{Serialize, Deserialize};
use std::fs;

#[derive(Serialize, Deserialize)]
pub struct SyncConfig {
    pub target_host: String,
    pub target_port: Option<u16>,
    pub port: Option<u16>,
    pub repositories: Vec<Repository>,
}

#[derive(Serialize, Deserialize)]
pub struct Repository {
    pub url: String,
}

pub fn load(config_file: &String) -> SyncConfig {
    let content = fs::read_to_string(config_file).expect("Unable to read config!");
    let config: SyncConfig = toml::from_str(&content).expect("Unable to deserialize config!");
    return config;
}

pub fn save(config_file: &String, config: &SyncConfig) {
    let content = toml::to_string(config).expect("Unable to serialize config!");
    fs::write(config_file, content).expect("Unable to save config!");
}