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
    // git::main();
    let cfg = SyncConfig {
        target_host: "http://our.server.ee".to_string(),
        target_port: None,
        port: Some(8765),
        repositories: vec![Repository{ url: "http://gitlab.cs.ttu.ee/taannu/iti0201-2018".to_string() }]
    };
    let loc = "./test.toml".to_string();
    config::save(&loc, &cfg);
    rocket::ignite().mount("/", routes![index]).launch();
}