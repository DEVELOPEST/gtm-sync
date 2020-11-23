#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

#[path = "git/git.rs"] mod git;
#[path = "config/config.rs"] mod config;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

fn main() {
    let loc = "./example_config.toml".to_string();
    let loaded_cfg = config::load(&loc);
    println!("{}", &loaded_cfg.target_host);
    let repo_to_clone = loaded_cfg.repositories.get(0).unwrap();
    let repo = git::clone_or_open(&repo_to_clone).unwrap();
    let _res = git::fetch(&repo, &repo_to_clone);
    let _commits = git::read_commits(&repo);
    //rocket::ignite().mount("/", routes![index]).launch();
}