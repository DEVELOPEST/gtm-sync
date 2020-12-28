use crate::config::config;
use crate::config::repository::generate_credentials_from_clone_url;
use crate::dto::response::{RepoDto, RepoWrapperDto};
use crate::gtm::git;

pub async fn sync_all() -> i32 {
    let cfg = config::load(&config::CONFIG_PATH);
    let client = reqwest::Client::new();
    let mut synced_count = 0;

    for repo in &cfg.repositories {
        let git_repo = git::clone_or_open(&repo).unwrap();
        let _res = git::fetch(&git_repo, &repo);
        let commits = git::read_commits(&git_repo).unwrap();
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
            synced_count += 1;
        } // TODO: Handle error
    }

    return synced_count;
}

pub fn sync_single() -> bool {
    false
}

fn generate_repo_sync_url(target_host: &String) -> String {
    return format!("{}/api/repositories", target_host)
}