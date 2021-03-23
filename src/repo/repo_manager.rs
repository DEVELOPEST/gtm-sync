use serde::{Deserialize, Serialize};

use crate::config::config;
use crate::sync::sync;
use crate::config::repository::generate_credentials_from_clone_url;
use crate::config::repository::Repository;
use crate::gtm::git;
use crate::gtm::gtm::Commit;

#[derive(Serialize, Deserialize)]
pub struct AddRepositoryDto {
    pub url: String,
    pub ssh_private_key: Option<String>,
    pub ssh_public_key: Option<String>,
    pub ssh_user: Option<String>,
    pub ssh_passphrase: Option<String>,
}

#[derive(Serialize)]
pub struct AddRepoDto {
    pub success: bool,
    pub provider: Option<String>,
    pub user: Option<String>,
    pub repo: Option<String>,
    pub sync_url: Option<String>,
    pub message: Option<String>,
}

#[derive(Serialize)]
pub struct RepoDto {
    pub provider: String,
    pub user: String,
    pub repo: String,
    pub commits: Vec<Commit>,
}

#[derive(Serialize)]
pub struct RepoWrapperDto {
    pub repository: Option<RepoDto>,
    // TODO: Errors
}

impl AddRepositoryDto {
    pub fn to_repository(&self, f: &dyn Fn(&String) -> String) -> Repository {
        return Repository {
            url: self.url.clone(),
            path: f(&self.url.to_string()),
            ssh_private_key: self.ssh_private_key.clone(),
            ssh_public_key: self.ssh_public_key.clone(),
            ssh_user: self.ssh_user.clone(),
            ssh_passphrase: self.ssh_passphrase.clone(),
        }
    }
}

pub fn get_repo(provider: &String, user: &String, repo: &String) -> RepoWrapperDto {
    let cfg = config::load(&config::CONFIG_PATH);
    let repo_to_clone = cfg.repositories.iter()
        .find(|r| r.path == cfg.generate_path_from_provider_user_repo(&provider, &user, &repo));

    if repo_to_clone.is_none() {
        // TODO: Some error thingy
        return RepoWrapperDto {
            repository: None
        };
    }
    let repo_to_clone = repo_to_clone.unwrap();

    let git_repo = git::clone_or_open(&repo_to_clone, &cfg).unwrap();
    if git::fetch(&git_repo, &repo_to_clone, &cfg).is_err() {
        warn!("Error fetching git data!");
    }
    let commits = git::read_commits(&git_repo).unwrap_or(vec![]);
    let gtm_repo: RepoDto = RepoDto {
        provider: provider.clone(),
        user: user.clone(),
        repo: repo.clone(),
        commits
    };
    return RepoWrapperDto {
        repository: Option::from(gtm_repo)
    };
}

pub async fn add_repo(repo_dto: AddRepositoryDto) -> AddRepoDto {
    let mut cfg = config::load(&config::CONFIG_PATH);
    let repo = repo_dto.to_repository(&|url: &String| { cfg.generate_path_from_git_url(url) });
    let cloned_repo = git::clone_or_open(&repo, &cfg);
    if cloned_repo.is_ok() {
        let (provider, user, repository) = generate_credentials_from_clone_url(&repo.url);
        if !cfg.repositories.iter().any(|r| r.url == repo_dto.url) {
            cfg.repositories.push(repo);
            config::save(&config::CONFIG_PATH, &cfg);
        }
        sync::sync_repo(&provider, &user, &repository).await;
        let sync_url = format!("{}/repositories/{}/{}/{}/sync", cfg.get_sync_url(), provider, user, repository);
        return AddRepoDto {
            success: true,
            provider: Option::from(provider),
            user: Option::from(user),
            repo: Option::from(repository),
            sync_url: Option::from(sync_url),
            message: None,
        };
    }
    let error_msg = cloned_repo.err().map(|e| e.to_string()).unwrap_or_default();
    error!("Error adding repo: {}", &error_msg);
    return AddRepoDto {
        success: false,
        provider: None,
        user: None,
        repo: None,
        sync_url: None,
        message: Option::from(error_msg),
    };
}
