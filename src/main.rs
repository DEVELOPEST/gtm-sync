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
        repositories: vec![Repository{ url: "http://gitlab.cs.ttu.ee/taannu/iti0201-2018".to_string(), path: "taannu/a".to_string() },
                           Repository{ url: "http://gitlab.cs.ttu.ee/taannu/iti0201-2019".to_string(), path: "taanni/b".to_string() },
                           Repository{ url: "http://gitlab.cs.ttu.ee/taannu/iti0201-2020".to_string(), path: "taannu/c".to_string() }]
    };
    let loc = "./test.toml".to_string();
    config::save(&loc, &cfg);
    let loaded_cfg = config::load(&loc);
    println!("{}", &loaded_cfg.target_host);
    let url = "https://github.com/DEVELOPEST/gtm-sync.git".parse().unwrap();
    let path = "./repo".parse().unwrap();
    let repo = git::clone(&url, &path).unwrap();
    let _res = git::fetch(&repo);
    //rocket::ignite().mount("/", routes![index]).launch();
}