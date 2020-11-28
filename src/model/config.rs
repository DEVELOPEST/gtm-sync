use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub target_host: String,
    pub target_port: Option<u16>,
    pub port: Option<u16>,
    pub repositories_base_path: String,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub repositories: Vec<Repository>,
}

#[derive(Serialize, Deserialize)]
pub struct Repository {
    pub url: String,
    pub path: String,
    pub ssh_private_key: Option<String>,
    pub ssh_public_key: Option<String>,
    pub ssh_user: Option<String>,
    pub ssh_passphrase: Option<String>,
}
