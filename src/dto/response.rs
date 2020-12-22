use serde::{Serialize};
use crate::gtm::gtm::Commit;

#[derive(Serialize)]
pub struct AddRepoDto {
    pub success: bool,
    pub provider: Option<String>,
    pub user: Option<String>,
    pub repo: Option<String>,
    pub message: Option<String>,
}

#[derive(Serialize)]
pub struct RepoDto {
    pub provider: String,
    pub user: String,
    pub repo: String,
    pub sync_url: String,
    pub access_token: Option<String>,
    pub commits: Vec<Commit>,
}

#[derive(Serialize)]
pub struct RepoWrapperDto {
    pub repository: Option<RepoDto>,
    // TODO: Errors
}