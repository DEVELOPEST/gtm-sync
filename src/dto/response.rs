use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct BoolResponseDto {
    pub success: bool,
    pub message: Option<String>,
}