use serde::{Serialize, Deserialize};
use crate::model::Repository;

#[derive(Serialize, Deserialize)]
pub struct AddRepositoryDto {
    pub url: String,
    pub ssh_private_key: Option<String>,
    pub ssh_public_key: Option<String>,
    pub ssh_user: Option<String>,
    pub ssh_passphrase: Option<String>,
}

impl AddRepositoryDto {
    pub fn to_repository(&self, f: &dyn Fn(&String) -> String) -> Repository {
        return Repository {
            url: self.url.clone(),
            path: f(&self.url.to_string()),
            ssh_private_key: self.ssh_private_key.clone(),
            ssh_public_key: self.ssh_public_key.clone(),
            ssh_user: self.ssh_user.clone(),
            ssh_passphrase: self.ssh_passphrase.clone(),
        }
    }
}
