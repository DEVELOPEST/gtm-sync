use lazy_static::lazy_static;
use regex::Regex;

use crate::config::config;
use crate::dto::request::AddRepositoryDto;
use crate::dto::response::{AddRepoDto, RepoDto};
use crate::gtm::git;

lazy_static! {
    static ref PATH_FROM_URL_REGEX: Regex =
        Regex::new(r#"(git@|https://)([a-zA-Z0-9.]+)[:/]([a-zA-Z0-9-]+)/([a-zA-Z0-9-]+)\.git"#).unwrap();

    static ref CONFIG_PATH: String = "./example_config.toml".to_string();
}

pub fn get_repo(provider: &String, user: &String, repo: &String) -> RepoDto {
    let cfg = config::load(&CONFIG_PATH);
    let repo_to_clone = cfg.repositories.iter()
        .find(|r| r.path == generate_path_from_provider_user_repo(&provider, &user, &repo, &cfg.repositories_base_path));

    if repo_to_clone.is_none() {
        // TODO: Some error thingy
        return RepoDto {
            commits: vec![]
        }
    }
    let repo_to_clone = repo_to_clone.unwrap();

    let repo = git::clone_or_open(&repo_to_clone).unwrap();
    let _res = git::fetch(&repo, &repo_to_clone);
    let commits = git::read_commits(&repo).unwrap();
    let gtm_repo: RepoDto = RepoDto {
        commits
    };
    return gtm_repo;
}

pub fn add_repo(repo_dto: AddRepositoryDto) -> AddRepoDto {
    let mut cfg = config::load(&CONFIG_PATH);
    let repo = repo_dto.to_repository(&|url: &String| { generate_path_from_git_url(url, &cfg.repositories_base_path) });
    let cloned_repo = git::clone_or_open(&repo);
    if cloned_repo.is_ok() {
        let (provider, user, repository) = get_credentials_from_clone_url(&repo.url);
        if !cfg.repositories.iter().any(|r| r.url == repo_dto.url) {
            cfg.repositories.push(repo);
            config::save(&CONFIG_PATH, &cfg);
        }
        return AddRepoDto {
            success: true,
            provider: Option::from(provider),
            user: Option::from(user),
            repo: Option::from(repository),
            message: None,
        };
    }
    return AddRepoDto {
        success: false,
        provider: None,
        user: None,
        repo: None,
        message: cloned_repo.err().map(|e| e.to_string()),
    };
}

pub fn generate_path_from_git_url(url: &String, base_path: &String) -> String {
    let (provider, user, repo) = get_credentials_from_clone_url(url);
    return format!("{}/{}/{}/{}", base_path.trim_end_matches("/"), provider, user, repo);
}

pub fn get_credentials_from_clone_url(url: &String) -> (String, String, String) {
    let caps = PATH_FROM_URL_REGEX.captures(url).unwrap();
    return (caps.get(2).map_or("provider".to_string(), |m| m.as_str().to_string()),
            caps.get(3).map_or("user".to_string(), |m| m.as_str().to_string()),
            caps.get(4).map_or("repo".to_string(), |m| m.as_str().to_string()))
}

pub fn generate_path_from_provider_user_repo(
    provider: &String,
    user: &String,
    repo: &String,
    base_path: &String,
) -> String {
    return format!("{}/{}/{}/{}", base_path.trim_end_matches("/"), provider, user, repo);
}