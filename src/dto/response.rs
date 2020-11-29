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
    pub commits: Vec<Commit>,
}