use serde::{Serialize};
use crate::gtm::gtm::Commit;

#[derive(Serialize)]
pub struct BoolResponseDto {
    pub success: bool,
    pub message: Option<String>,
}

#[derive(Serialize)]
pub struct Repo {
    pub commits: Vec<Commit>,
}