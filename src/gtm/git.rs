#![deny(warnings)]

use std::collections::HashMap;
use std::fs;
use std::path::Path;

use git2::{BranchType, Error, FetchOptions, Note, Oid, RemoteCallbacks, Repository};
use git2::build::RepoBuilder;

use crate::gtm::gtm;
use crate::model::config;

static GTM_NOTES_REF: &str = "refs/notes/gtm-data";
static GTM_NOTES_REF_SPEC: &str = "+refs/notes/gtm-data:refs/notes/gtm-data";
static DEFAULT_ORIGIN: &str = "origin";
static ORIGIN_PREFIX: &str = "refs/remotes/origin/";
static ORIGIN_HEAD: &str = "refs/remotes/origin/HEAD";

pub fn clone_or_open(repo_config: &config::Repository) -> Result<Repository, Error> {
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

fn generate_fetch_options(repo_config: &config::Repository) -> FetchOptions {
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

pub fn fetch(repo: &Repository, repo_config: &config::Repository) {
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

    let branches = repo.branches(Option::from(BranchType::Remote)).unwrap();
    let mut fetch_refs: Vec<String> = vec![];
    for branch in branches {
        let (branch, _) = branch.unwrap();
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

    let mut fo = generate_fetch_options(repo_config);
    remote.fetch(&fetch_refs, Option::from(&mut fo), None)
        .expect("Error fetching data!");
    remote.disconnect().unwrap();
}

pub fn read_commits(repo: &Repository) -> Result<Vec<gtm::Commit>, Error> {
    let mut commits: Vec<gtm::Commit> = Vec::new();
    let mut branch_map: HashMap<Oid, String> = HashMap::new();
    let mut revwalk = repo.revwalk().expect("Unable to revwalk!");
    revwalk.set_sorting(git2::Sort::TOPOLOGICAL).expect("Unable to set revwalk sorting!");
    revwalk.set_sorting(git2::Sort::REVERSE).expect("Unable to reverse revalk sorting!");
    let branches = repo.branches(Option::from(BranchType::Remote)).unwrap();
    let default_branch = repo.head().unwrap().name().unwrap().to_string();
    for branch in branches {
        let (branch, _) = branch.unwrap();
        let refspec = branch.get().name().unwrap();
        if refspec == ORIGIN_HEAD {
            continue
        }
        branch_map.insert(
            branch.get().target().unwrap(),
            refspec.strip_prefix(ORIGIN_PREFIX).unwrap().to_string(),
        );
        let _ = revwalk.push_ref(refspec);
    }

    for commit_oid in revwalk {
        let commit_oid = commit_oid?;
        let commit = repo.find_commit(commit_oid)?;
        let mut branch = "".to_string(); // TODO: Some more intelligent choice than last wins
        for (oid, name) in &branch_map {
            if repo.merge_base(commit_oid, *oid).unwrap() == commit_oid && branch != default_branch {
                branch = name.clone();
            }
        }
        let notes: Vec<Note> = repo.notes(Option::from(GTM_NOTES_REF))?
            .map(|n| n.unwrap())
            .filter(|n| n.1 == commit_oid)
            .map(|n| repo.find_note(Option::from(GTM_NOTES_REF), n.1).unwrap())
            .collect();

        let res = gtm::parse_commit(&repo, &commit, &notes, branch)?;
        commits.push(res);
    }
    return Result::Ok(commits);
}
