#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

mod server;
mod gtm;
mod config;
mod sync;
mod repo;

fn main() {
    server::server::run();
}