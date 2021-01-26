#![deny(warnings)]

use std::fs;
use std::path::Path;

use git2::{BranchType, Error, FetchOptions, Note, RemoteCallbacks, Repository};
use git2::build::RepoBuilder;

use crate::gtm::gtm;
use crate::config::repository;
use crate::config::config::Config;

static GTM_NOTES_REF: &str = "refs/notes/gtm-data";
static GTM_NOTES_REF_SPEC: &str = "+refs/notes/gtm-data:refs/notes/gtm-data";
static DEFAULT_ORIGIN: &str = "origin";
static ORIGIN_PREFIX: &str = "refs/remotes/origin/";
static ORIGIN_HEAD: &str = "refs/remotes/origin/HEAD";

pub fn clone_or_open(repo_config: &repository::Repository, cfg: &Config) -> Result<Repository, Error> {
    let path = Path::new(&repo_config.path);

    if path.exists() {
        let repo = Repository::open(path);
        if repo.is_ok() {
            return repo;
        }
        let _remove = fs::remove_dir_all(&path)
            .expect(&*format!("Unable to remove dir: {}", repo_config.path));
        return clone_or_open(&repo_config, &cfg);
    }

    let fo = generate_fetch_options(&repo_config, &cfg);

    return RepoBuilder::new()
        .fetch_options(fo)
        .clone(&repo_config.url, Path::new(&repo_config.path));
}

fn generate_fetch_options<'a>(repo_config: &'a repository::Repository, cfg: &'a Config) -> FetchOptions<'a> {
    let mut cb = RemoteCallbacks::new();
    let repo_config = repo_config.clone();
    cb.credentials(move |_c, _o, t| {
        if t.is_ssh_key() {
            return git2::Cred::ssh_key(
                &repo_config.ssh_user.as_ref()
                    .unwrap_or(cfg.ssh_user.as_ref().unwrap_or(&"git".to_string())),
                Option::from(Path::new(&repo_config.ssh_public_key.as_ref()
                    .unwrap_or(cfg.ssh_public_key.as_ref().unwrap_or(&"".to_string())))),
                &Path::new(&repo_config.ssh_private_key.as_ref()
                    .unwrap_or(cfg.ssh_private_key.as_ref().unwrap_or(&"".to_string()))),
                repo_config.ssh_passphrase.as_ref()
                    .or(cfg.ssh_passphrase.as_ref()).map(|x| &**x),
            )
        }
        return git2::Cred::default();
    });

    let mut fo = FetchOptions::new();
    fo.remote_callbacks(cb);
    return fo;
}

pub fn fetch(repo: &Repository, repo_config: &repository::Repository, cfg: &Config) -> Result<(), git2::Error>{
    let mut remote = repo.find_remote(DEFAULT_ORIGIN)?;
    let mut ref_added = false;
    let refs = remote.fetch_refspecs()?;
    for i in 0..refs.len() {
        if refs.get(i).unwrap() == GTM_NOTES_REF_SPEC {
            ref_added = true;
            break;
        }
    }
    if !ref_added {
        if repo.remote_add_fetch(DEFAULT_ORIGIN, GTM_NOTES_REF_SPEC).is_err() {
            println!("Unable to add fetch ref spec for gtm-data!")
        }
        remote = repo.find_remote(DEFAULT_ORIGIN)
            .expect("Unable to find remote 'origin'");
    }

    let branches = repo.branches(Option::from(BranchType::Remote)).unwrap();
    let mut fetch_refs: Vec<String> = vec![];
    for branch in branches {
        let (branch, _) = branch?;
        let refspec = branch.get()
            .name()
            .unwrap()
            .strip_prefix(ORIGIN_PREFIX)
            .unwrap();
        if refspec != "HEAD" {
            fetch_refs.push(format!("refs/heads/{}", refspec.to_string()));
        }
    }
    fetch_refs.push(GTM_NOTES_REF.parse().unwrap());

    let mut fo = generate_fetch_options(repo_config, cfg);
    remote.fetch(&fetch_refs, Option::from(&mut fo), None)?;
    remote.disconnect()?;
    Ok(())
}

pub fn read_commits(repo: &Repository) -> Result<Vec<gtm::Commit>, Error> {
    let mut commits: Vec<gtm::Commit> = Vec::new();
    let mut revwalk = repo.revwalk().expect("Unable to revwalk!");
    revwalk.set_sorting(git2::Sort::TOPOLOGICAL).expect("Unable to set revwalk sorting!");
    revwalk.set_sorting(git2::Sort::REVERSE).expect("Unable to reverse revalk sorting!");
    let branches = repo.branches(Option::from(BranchType::Remote))?;
    for branch in branches {
        let (branch, _) = branch?;
        let refspec = branch.get().name().unwrap();
        if refspec == ORIGIN_HEAD {
            continue
        }
        let _ = revwalk.push_ref(refspec);
    }

    for commit_oid in revwalk {
        let commit_oid = commit_oid?;
        let commit = repo.find_commit(commit_oid)?;
        let raw_notes = repo.notes(Option::from(GTM_NOTES_REF)).ok();
        let notes: Vec<Note> = if raw_notes.is_some() {
            raw_notes.unwrap()
                .filter_map(|n| n.ok())
                .filter(|n| n.1 == commit_oid)
                .map(|n| repo.find_note(Option::from(GTM_NOTES_REF), n.1))
                .filter_map(|r| r.ok())
                .collect()
        } else { vec![] };

        let res = gtm::parse_commit(&repo, &commit, &notes)?;
        commits.push(res);
    }
    return Result::Ok(commits);
}
