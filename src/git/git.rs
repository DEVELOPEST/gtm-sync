#![deny(warnings)]

use std::fs;
use std::path::Path;

use git2::{Error, FetchOptions, Note, RemoteCallbacks, Repository};
use git2::build::RepoBuilder;
use crate::model;

pub(crate) mod gtm;

static GTM_NOTES_REF: &str = "refs/notes/gtm-data";
static GTM_NOTES_REF_SPEC: &str = "+refs/notes/gtm-data:refs/notes/gtm-data";
static DEFAULT_ORIGIN: &str = "origin";

pub fn clone_or_open(repo_config: &model::Repository) -> Result<Repository, Error> {
    let path = Path::new(&repo_config.path);

    if path.exists() {
        let repo = Repository::open(path);
        if repo.is_ok() {
            return repo;
        }
        let _remove = fs::remove_dir_all(&path)
            .expect(&*format!("Unable to remove dir: {}", repo_config.path));
        return clone_or_open(&repo_config);
    }

    let fo = generate_fetch_options(repo_config);

    return RepoBuilder::new()
        .fetch_options(fo)
        .clone(&repo_config.url, Path::new(&repo_config.path));
}

fn generate_fetch_options(repo_config: &model::Repository) -> FetchOptions {
    let mut cb = RemoteCallbacks::new();
    let repo_config = repo_config.clone();
    cb.credentials(move |_c, _o, t| {
        if t.is_ssh_key() {
            return git2::Cred::ssh_key(
                &repo_config.ssh_user.as_ref().unwrap_or(&"git".to_string()),
                Option::from(Path::new(&repo_config.ssh_public_key.as_ref().unwrap_or(&"".to_string()))),
                &Path::new(&repo_config.ssh_private_key.as_ref().unwrap_or(&"".to_string())),
                repo_config.ssh_passphrase.as_ref().map(|x| &**x),
            )
        }
        return git2::Cred::default();
    });


    let mut fo = FetchOptions::new();
    fo.remote_callbacks(cb);
    return fo;
}


pub fn fetch(repo: &Repository, repo_config: &model::Repository) {
    let mut remote = repo.find_remote(DEFAULT_ORIGIN)
        .expect("Unable to find remote 'origin'");
    let mut ref_added = false;
    let refs = remote.fetch_refspecs().unwrap();
    for i in 0..refs.len() {
        if refs.get(i).unwrap() == GTM_NOTES_REF_SPEC {
            ref_added = true;
            break;
        }
    }
    if !ref_added {
        repo.remote_add_fetch(DEFAULT_ORIGIN, GTM_NOTES_REF_SPEC)
            .expect("Unable to add fetch ref spec for gtm-data!");
        remote = repo.find_remote(DEFAULT_ORIGIN)
            .expect("Unable to find remote 'origin'");
    }

    let mut fo = generate_fetch_options(repo_config);
    remote.fetch(&[] as &[&str], Option::from(&mut fo), None)
        .expect("Error fetching data!");
    remote.disconnect().unwrap();
}

pub fn read_commits(repo: &Repository) -> Result<Vec<gtm::Commit>, Error> {
    let mut commits: Vec<gtm::Commit> = Vec::new();
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

        let res=  gtm::parse_commit(&repo, &commit, &notes)?;
        println!("{}", &res);
        commits.push(res);
    }
    // let a = repo.notes(Option::from(GTM_NOTES_REF))
    //     .expect("Unable to find gtm-notes");
    return Result::Ok(commits);
}
