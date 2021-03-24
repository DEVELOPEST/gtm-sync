use rocket_contrib::json::{JsonValue, Json};

use crate::repo::repo_manager;
use crate::sync::sync;
use crate::repo::repo_manager::AddRepositoryDto;
use crate::server::resource::{GithubPushWebhook, GitlabPushWebhook};
use crate::config::repository::generate_credentials_from_clone_url;

#[get("/repositories/<provider>/<user>/<repo>")]
pub fn repo(provider: String, user: String, repo: String) -> JsonValue {
    let repo = repo_manager::get_repo(&provider, &user, &repo); // TODO: How to match credentials?
    rocket_contrib::json!(&repo)
}

#[post("/repositories", data="<repo>")]
pub fn add_repo(repo: Json<AddRepositoryDto>) -> JsonValue {
    let mut rt = tokio::runtime::Runtime::new().unwrap();
    let response = rt.block_on(repo_manager::add_repo(repo.into_inner()));
    rocket_contrib::json!(&response)
}

#[get("/repositories/sync-all")]
pub fn sync_all() -> JsonValue {
    let mut rt = tokio::runtime::Runtime::new().unwrap();
    let response = rt.block_on(sync::sync_all());
    rocket_contrib::json!(&response)
}

#[get("/repositories/<provider>/<user>/<repo>/sync")]
pub fn sync_repo(provider: String, user: String, repo: String) -> JsonValue {
    let mut rt = tokio::runtime::Runtime::new().unwrap();
    let response = rt.block_on(sync::sync_repo(&provider, &user, &repo));
    rocket_contrib::json!(&response)
}

#[post("/repositories/<provider>/<user>/<repo>/sync")]
pub fn post_sync_repo(provider: String, user: String, repo: String) -> JsonValue {
    sync_repo(provider, user, repo)
}

#[post("/hooks/github/push", data="<dto>")]
pub fn sync_repo_github(dto: Json<GithubPushWebhook>) -> JsonValue {
    let dto = dto.into_inner();
    let (provider, user, repo) = generate_credentials_from_clone_url(&dto.repository.ssh_url);
    let mut rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(sync::sync_repo(&provider, &user, &repo));
    rocket_contrib::json!({})
}

#[post("/hooks/gitlab/push", data="<dto>")]
pub fn sync_repo_gitlab(dto: Json<GitlabPushWebhook>) -> JsonValue {
    let dto = dto.into_inner();
    let (provider, user, repo) = generate_credentials_from_clone_url(&dto.repository.git_ssh_url);
    let mut rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(sync::sync_repo(&provider, &user, &repo));
    rocket_contrib::json!({})
}
