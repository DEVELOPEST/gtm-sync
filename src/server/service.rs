use lazy_static::lazy_static;
use regex::Regex;

use crate::config::config;
use crate::dto::request::AddRepositoryDto;
use crate::dto::response::{BoolResponseDto, Repo};
use crate::gtm::git;

lazy_static! {
    static ref PATH_FROM_URL_REGEX: Regex =
        Regex::new(r#"(git@|https://)([a-zA-Z0-9.]+)[:/]([a-zA-Z0-9-]+)/([a-zA-Z0-9-]+)\.git"#).unwrap();
}

pub fn get_repo(provider: &String, user: &String, repo: &String) -> Repo {
    let loc = "./example_config.toml".to_string();
    let loaded_cfg = config::load(&loc);
    let repo_to_clone = loaded_cfg.repositories.iter()
        .find(|r| r.path == generate_path_from_provider_user_repo(&provider, &user, &repo, &loaded_cfg.repositories_base_path));

    if repo_to_clone.is_none() {
        // TODO: Some error thingy
        return Repo {
            commits: vec![]
        }
    }
    let repo_to_clone = repo_to_clone.unwrap();

    let repo = git::clone_or_open(&repo_to_clone).unwrap();
    let _res = git::fetch(&repo, &repo_to_clone);
    let commits = git::read_commits(&repo).unwrap();
    let gtm_repo: Repo = Repo {
        commits
    };
    return gtm_repo;
}

pub fn add_repo(repo_dto: AddRepositoryDto) -> BoolResponseDto {
    let loc = "./example_config.toml".to_string();
    let mut loaded_cfg = config::load(&loc);
    let repo = repo_dto.to_repository(&|url: &String| { generate_path_from_git_url(url, &loaded_cfg.repositories_base_path) });
    let cloned_repo = git::clone_or_open(&repo);
    if cloned_repo.is_ok() {
       if !loaded_cfg.repositories.iter().any(|r| r.url == repo_dto.url) {
            loaded_cfg.repositories.push(repo);
            config::save(&loc, &loaded_cfg);
        }
        return BoolResponseDto {
            success: true,
            message: None,
        };
    }
    return BoolResponseDto {
        success: false,
        message: cloned_repo.err().map(|e| e.to_string()),
    };
}

pub fn generate_path_from_git_url(url: &String, base_path: &String) -> String {
    let caps = PATH_FROM_URL_REGEX.captures(url).unwrap();
    let path = format!("{}/{}/{}/{}",
                       base_path.trim_end_matches("/"),
                       caps.get(2).map_or("provider", |m| m.as_str()),
                       caps.get(3).map_or("user", |m| m.as_str()),
                       caps.get(4).map_or("repo", |m| m.as_str())
    );
    return path;
}

pub fn generate_path_from_provider_user_repo(
    provider: &String,
    user: &String,
    repo: &String,
    base_path: &String,
) -> String {
    return format!("{}/{}/{}/{}", base_path.trim_end_matches("/"),
                   provider, user, repo
    );
}