use lazy_static::lazy_static;
use regex::Regex;
use serde::{Deserialize, Serialize};

lazy_static! {
    static ref PATH_FROM_URL_REGEX: Regex =
        Regex::new(r#"(git@|https://)([a-zA-Z0-9.]+)[:/]([a-zA-Z0-9-]+)/([a-zA-Z0-9-]+)\.git"#).unwrap();
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Repository {
    pub url: String,
    pub path: String,
    pub ssh_private_key: Option<String>,
    pub ssh_public_key: Option<String>,
    pub ssh_user: Option<String>,
    pub ssh_passphrase: Option<String>,
}

// pub fn generate_path_from_git_url(url: &String, base_path: &String) -> String {
//     let (provider, user, repo) = generate_credentials_from_clone_url(url);
//     return format!("{}/{}/{}/{}", base_path.trim_end_matches("/"), provider, user, repo);
// }

pub fn generate_credentials_from_clone_url(url: &String) -> (String, String, String) {
    let caps = PATH_FROM_URL_REGEX.captures(url).unwrap();
    return (caps.get(2).map_or("provider".to_string(), |m| m.as_str().to_string()),
            caps.get(3).map_or("user".to_string(), |m| m.as_str().to_string()),
            caps.get(4).map_or("repo".to_string(), |m| m.as_str().to_string()));
}

impl Repository {}