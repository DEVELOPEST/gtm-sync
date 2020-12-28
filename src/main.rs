#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

mod server;
mod dto;
mod gtm;
mod config;
mod sync;

fn main() {
    server::server::run();
}