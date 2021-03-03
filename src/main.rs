#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use(info, warn, error)] extern crate log;

mod server;
mod gtm;
mod config;
mod sync;
mod repo;

fn main() {
    server::server::run();
}