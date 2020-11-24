#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

#[path = "git/git.rs"] mod git;
#[path = "config/config.rs"] mod config;
#[path = "server/server.rs"] mod server;

fn main() {
    server::run();
}