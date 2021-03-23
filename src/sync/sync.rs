use reqwest::{Client, Method};
use serde::{Deserialize, Serialize};

use crate::config::config;
use crate::config::repository::{generate_credentials_from_clone_url, Repository};
use crate::gtm::git;
use crate::repo::repo_manager::{RepoDto, RepoWrapperDto};
use crate::gtm::gtm::Commit;

#[derive(Serialize)]
pub struct SyncAllResult {
    error: Option<String>,
    synced_count: i32,
}

#[derive(Serialize)]
pub struct SyncSingleResult {
    error: Option<String>,
    ok: bool,
}

#[derive(Deserialize)]
pub struct LastSyncResponse {
    // hash: String,
    timestamp: i64,
    tracked_commit_hashes: Vec<String>,
}

pub async fn sync_all() -> SyncAllResult {
    let cfg = config::load(&config::CONFIG_PATH);
    let client = reqwest::Client::new();
    let mut result = SyncAllResult { error: None, synced_count: 0 };

    let tasks: Vec<_> = cfg.repositories
        .iter()
        .map(|repo| sync_single(&repo, &cfg, &client))
        .collect();

    for task in tasks {
        let res = task.await;
        if res.is_ok() {
            if res.unwrap().status().is_success() {
                result.synced_count += 1;
            }
        } else {
            result.error = Option::from(res.err().unwrap().to_string())
        }
    }

    return result;
}

pub async fn sync_repo(provider: &String, user: &String, repo: &String) -> SyncSingleResult {
    let cfg = config::load(&config::CONFIG_PATH);
    let client = reqwest::Client::new();

    let repo_to_sync = cfg.repositories.iter()
        .find(|&r| r.path == cfg.generate_path_from_provider_user_repo(&provider, &user, &repo));
    if repo_to_sync.is_none() {
        return SyncSingleResult { error: Option::from("No matching repository found!".to_string()), ok: false }
    }
    let repo_to_sync = repo_to_sync.unwrap();
    let res = sync_single(&repo_to_sync, &cfg, &client).await;
    // TODO: Check for error in json
    if res.is_err() || !res.unwrap().status().is_success() {
        error!("Error syncing repo!");
        return SyncSingleResult { error: Option::from("Error syncing repo!".to_string()), ok: false }
    }
    return SyncSingleResult { error: None, ok: true }
}

async fn sync_single(
    repo: &Repository,
    cfg: &config::Config,
    client: &reqwest::Client,
) -> Result<reqwest::Response, reqwest::Error> {
    let git_repo = git::clone_or_open(&repo, &cfg).unwrap();
    let res = git::fetch(&git_repo, &repo, &cfg);
    if res.is_err() {
        warn!("Error fetching git data: {}", res.err().unwrap().message())
    }

    let last_sync = fetch_synced_hashes(
        &client,
        &repo,
        &cfg.get_target_url(),
        &cfg.access_token.clone().unwrap_or("".to_string()))
        .await
        .unwrap_or(LastSyncResponse {
            timestamp: -1,
            tracked_commit_hashes: vec![],
        });
    let mut commits: Vec<Commit> = git::read_commits(&git_repo).unwrap_or(vec![]);
    let commit_hashes: Vec<String> = commits.iter().map(|c| c.hash.clone()).collect();
    let (provider, user, repo) = generate_credentials_from_clone_url(&repo.url);

    let mut method = Method::POST;
    if last_sync.timestamp > 0 && last_sync.tracked_commit_hashes.iter()
            .all(|h| commit_hashes.contains(h)) {
        commits = commits.into_iter()
            .filter(|c| !last_sync.tracked_commit_hashes.contains(&c.hash))
            .collect();
        method = Method::PUT;
    }

    let gtm_repo: RepoDto = RepoDto {
        provider: provider.clone(),
        user: user.clone(),
        repo: repo.clone(),
        commits,
    };
    let dto = RepoWrapperDto {
        repository: Option::from(gtm_repo)
    };

    return client.request(method, &generate_repo_sync_url(&cfg.get_target_url()))
        .json(&dto)
        .header("API-key", cfg.access_token.clone().unwrap_or("".to_string()))
        .send()
        .await;
}

fn generate_repo_sync_url(target_host: &String) -> String {
    return format!("{}/api/repositories", target_host)
}

async fn fetch_synced_hashes(
    client: &Client,
    repo: &Repository,
    target_host: &str,
    api_key: &str,
) -> Result<LastSyncResponse, reqwest::Error> {
    let (provider, user, repo) = generate_credentials_from_clone_url(&repo.url);
    let url = format!("{}/api/commits/{}/{}/{}/hash", target_host, provider, user, repo);

    return Ok(client.get(&url)
        .header("API-key", api_key)
        .send()
        .await?
        .json::<LastSyncResponse>()
        .await?)
}
