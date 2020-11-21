#![deny(warnings)]

use git2::build::{CheckoutBuilder, RepoBuilder};
use git2::{FetchOptions, RemoteCallbacks, Repository, Error};
use std::path::{Path};

pub fn clone(url: &String, path: &String) -> Result<Repository, Error> {

    let cb = RemoteCallbacks::new();
    // cb.certificate_check(|cc| {
    //     true
    // });

    let co = CheckoutBuilder::new();
    // co.progress(|path, cur, total| {
    //
    // });

    let mut fo = FetchOptions::new();
    fo.remote_callbacks(cb);

    return  RepoBuilder::new()
        .fetch_options(fo)
        .with_checkout(co)
        .clone(&url, Path::new(&path));
}

pub fn fetch(repo: &Repository) {
    let _b = repo.remote_add_fetch("origin","+refs/notes/gtm-data:refs/notes/gtm-data").unwrap();
    let mut remote = repo.find_remote("origin").unwrap();
    let gtm_ref= ["main".to_string(), "refs/notes/gtm-data".to_string()];
    remote.fetch(&gtm_ref, None, None).unwrap();
    remote.disconnect().unwrap();

    //let mut notes_remote = repo.find_remote("gtm").unwrap();
    //notes_remote.fetch(&gtm_ref, None, None).unwrap();
    let a = repo.notes(Option::from("refs/notes/gtm-data")).unwrap();
    println!("{}", a.count());
}
