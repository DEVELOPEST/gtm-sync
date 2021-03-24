use std::fs;

use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};

use crate::config::repository::{generate_credentials_from_clone_url, Repository};

lazy_static! {
    pub static ref CONFIG_PATH: String = "./config.toml".to_string();
}


#[derive(Serialize, Deserialize)]
pub struct Config {
    target_base_url: String,
    pub sync_base_url: Option<String>,
    pub prefix: Option<String>,
    pub access_token: Option<String>,
    pub ssh_public_key: Option<String>,
    pub ssh_private_key: Option<String>,
    pub ssh_user: Option<String>,
    pub ssh_passphrase: Option<String>,
    pub repositories_base_path: String,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub repositories: Vec<Repository>,
}

pub fn load(config_file: &String) -> Config {
    let content = fs::read_to_string(config_file).expect("Unable to read config!");
    return toml::from_str(&content).expect("Unable to deserialize config!");
}

pub fn save(config_file: &String, config: &Config) {
    let content = toml::to_string(config).expect("Unable to serialize config!");
    fs::write(config_file, content).expect("Unable to save config!");
}

impl Config {
    pub fn get_target_url(&self) -> String {
        return self.target_base_url.clone();
    }

    pub fn get_sync_url(&self) -> String {
        if self.sync_base_url.is_some() {
            return self.sync_base_url.clone().unwrap();
        }
        "http://localhost:8090/services/gtm/sync".to_string()
    }

    pub fn generate_path_from_git_url(&self, url: &String) -> String {
        let (provider, user, repo) = generate_credentials_from_clone_url(url);
        return format!("{}/{}/{}/{}", self.repositories_base_path.trim_end_matches("/"), provider, user, repo);
    }

    pub fn generate_path_from_provider_user_repo(
        &self,
        provider: &String,
        user: &String,
        repo: &String,
    ) -> String {
        return format!("{}/{}/{}/{}", self.repositories_base_path.trim_end_matches("/"), provider, user, repo);
    }
}