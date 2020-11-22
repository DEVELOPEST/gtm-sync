#![deny(warnings)]

use git2::build::{RepoBuilder};
use git2::{FetchOptions, RemoteCallbacks, Repository, Error, Commit, Note};
use std::path::{Path};
use std::fs;

mod gtm;

static GTM_NOTES_REF: &str = "refs/notes/gtm-data";
static GTM_NOTES_REF_SPEC: &str = "+refs/notes/gtm-data:refs/notes/gtm-data";
static DEFAULT_ORIGIN: &str = "origin";

pub fn clone_or_open(url: &String, repo_path: &String) -> Result<Repository, Error> {

    let path = Path::new(&repo_path);

    if path.exists() {
        let repo = Repository::open(path);
        if repo.is_ok() {
            return repo;
        }
        let _remove = fs::remove_dir_all(&path)
            .expect(&*format!("Unable to remove dir: {}", repo_path));
        return clone_or_open(&url, &repo_path);
    }

    let cb = RemoteCallbacks::new();
    // cb.certificate_check(|cc| {
    //     true
    // });


    let mut fo = FetchOptions::new();
    fo.remote_callbacks(cb);

    return  RepoBuilder::new()
        .fetch_options(fo)
        .clone(&url, path);
}

pub fn fetch(repo: &Repository) {
    repo.remote_add_fetch(DEFAULT_ORIGIN,GTM_NOTES_REF_SPEC)
        .expect("Unable to add fetch ref spec for gtm-data!");
    let mut remote = repo.find_remote(DEFAULT_ORIGIN)
        .expect("Unable to find remote 'origin'");
    remote.fetch(&[] as &[&str], None, None).expect("Error fetching data!");
    remote.disconnect().unwrap();
}

pub fn read_commits(repo: &Repository) -> Result<Vec<Commit>, Error> {
    let commits : Vec<Commit> = Vec::new();
    let mut revwalk = repo.revwalk().expect("Unable to revwalk!");
    let _sorting = revwalk.set_sorting(git2::Sort::TIME);
    let _head = revwalk.push_head();
    for commit_oid in revwalk {
        let commit_oid = commit_oid?;
        let commit = repo.find_commit(commit_oid)?;
        let notes: Vec<Note> = repo.notes(Option::from(GTM_NOTES_REF))?
            .map(|n| n.unwrap())
            .filter(|n| n.1 == commit_oid)
            .map(|n| repo.find_note(Option::from(GTM_NOTES_REF), n.1).unwrap())
            .collect();

        let res=  gtm::parse_commit(&commit, &notes)?;
        println!("{}", res);
    }
    // let a = repo.notes(Option::from(GTM_NOTES_REF))
    //     .expect("Unable to find gtm-notes");
    return Result::Ok(commits);
}
