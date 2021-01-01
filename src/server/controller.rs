use rocket_contrib::json::{JsonValue, Json};

use crate::server::service;
use crate::dto::request::AddRepositoryDto;
use crate::sync::sync::sync_all;

#[get("/")]
pub fn index() -> &'static str {
    "Hello, world!"
}

#[get("/repository/<provider>/<user>/<repo>")]
pub fn repo(provider: String, user: String, repo: String) -> JsonValue {
    let repo = service::get_repo(&provider, &user, &repo); // TODO: How to match credentials?
    rocket_contrib::json!(&repo)
}

#[post("/repository", data="<repo>")]
pub fn add_repo(repo: Json<AddRepositoryDto>) -> JsonValue {
    let response = service::add_repo(repo.into_inner());
    rocket_contrib::json!(&response)
}

#[get("/sync")]
pub fn sync() -> JsonValue {
    let mut rt = tokio::runtime::Runtime::new().unwrap();
    let response = rt.block_on(sync_all());
    rocket_contrib::json!(&response)
}