use reqwest::Client;
use serde::{Deserialize, Serialize};

use crate::config::config;
use crate::config::repository::{generate_credentials_from_clone_url, Repository};
use crate::dto::response::{RepoDto, RepoWrapperDto};
use crate::gtm::git;

#[derive(Serialize)]
pub struct SyncAllResult {
    error: Option<String>,
    synced_count: i32,
}

#[derive(Deserialize)]
pub struct LastSyncResponse {
    hash: String,
    timestamp: i64,
    tracked_commit_hashes: Vec<String>,
}

pub async fn sync_all() -> SyncAllResult {
    let cfg = config::load(&config::CONFIG_PATH);
    let client = reqwest::Client::new();
    let mut result = SyncAllResult { error: None, synced_count: 0 };

    for repo in &cfg.repositories {
        let git_repo = git::clone_or_open(&repo).unwrap();
        let _res = git::fetch(&git_repo, &repo);
        let last_sync = fetch_synced_hashes(&client, &repo)
            .await
            .unwrap_or(LastSyncResponse {
                hash: "".to_string(),
                timestamp: 0,
                tracked_commit_hashes: vec![],
            });
        let commits = git::read_commits(&git_repo)
            .unwrap()
            .into_iter()
            .filter(|c| !last_sync.tracked_commit_hashes.contains(&c.hash))
            .collect();

        let (provider, user, repo) = generate_credentials_from_clone_url(&repo.url);
        let gtm_repo: RepoDto = RepoDto {
            provider: provider.clone(),
            user: user.clone(),
            repo: repo.clone(),
            sync_url: cfg.get_sync_url(),
            access_token: cfg.access_token.clone(),
            commits,
        };
        let dto = RepoWrapperDto {
            repository: Option::from(gtm_repo)
        };

        let res = client.post(&generate_repo_sync_url(&cfg.get_target_url()))
            .json(&dto)
            .send()
            .await;
        if res.is_ok() {
            result.synced_count += 1;
        } else {
            result.error = Option::from(res.err().unwrap().to_string())
        }
    }

    return result;
}

pub fn sync_single() -> bool {
    false
}

fn generate_repo_sync_url(target_host: &String) -> String {
    return format!("{}/api/repositories", target_host)
}

async fn fetch_synced_hashes(client: &Client, repo: &Repository) -> Result<LastSyncResponse, reqwest::Error> {
    let (provider, user, repo) = generate_credentials_from_clone_url(&repo.url);
    let url = format!("/commits/{}/{}/{}/hash", provider, user, repo);

    return Ok(client.get(&url)
        .send()
        .await?
        .json::<LastSyncResponse>()
        .await?)
}