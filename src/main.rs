#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

use crate::config::{SyncConfig, Repository};

#[path = "git/git.rs"] mod git;
#[path = "config/config.rs"] mod config;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

fn main() {
    let cfg = SyncConfig {
        target_host: "http://our.server.ee".to_string(),
        target_port: None,
        port: Some(8765),
        repositories_base_path: "./repos".to_string(),
        repositories: vec![Repository{ url: "http://gitlab.cs.ttu.ee/taannu/iti0201-2018".to_string(), path: "taannu/a".to_string(), ssh_private_key: None, ssh_public_key: None, ssh_user: None, ssh_passphrase: None }]
    };
    let loc = "./test.toml".to_string();
    config::save(&loc, &cfg);
    let loaded_cfg = config::load(&loc);
    println!("{}", &loaded_cfg.target_host);
    let repo_to_clone: git::config::Repository = git::config::Repository{
        url: "git@gitlab.cs.ttu.ee:taannu/icd0008-2020f.git".to_string(),
        path: "./repo".to_string(),
        ssh_private_key: Option::from("/home/tavo/.ssh/id_git".to_string()),
        ssh_public_key: Option::from("/home/tavo/.ssh/id_git.pub".to_string()),
        ssh_user: Option::from("git".to_string()),
        ssh_passphrase: None
    };
    let repo = git::clone_or_open(&repo_to_clone).unwrap();
    let _res = git::fetch(&repo, &repo_to_clone);
    let _commits = git::read_commits(&repo);
    //rocket::ignite().mount("/", routes![index]).launch();
}