use rocket_contrib::json::{JsonValue};

use crate::server::service;

#[get("/")]
pub fn index() -> &'static str {
    "Hello, world!"
}

#[get("/repo")]
pub fn repo() -> JsonValue {
    let repo = service::get_repo();
    rocket_contrib::json!(&repo)
}