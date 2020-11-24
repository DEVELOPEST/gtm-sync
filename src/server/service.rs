use crate::config;
use crate::git;
use crate::git::gtm;

// TODO: (Tavo) Repo wrapper
pub fn get_repo() -> Vec<gtm::Commit> {
    let loc = "./example_config.toml".to_string();
    let loaded_cfg = config::load(&loc);
    println!("{}", &loaded_cfg.target_host);
    let repo_to_clone = loaded_cfg.repositories.get(0).unwrap();
    let repo = git::clone_or_open(&repo_to_clone).unwrap();
    let _res = git::fetch(&repo, &repo_to_clone);
    let commits = git::read_commits(&repo).unwrap();
    return commits;
}