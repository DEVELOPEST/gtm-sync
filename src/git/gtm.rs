use git2::{Oid};
use std::fmt;
use serde::export::Formatter;

pub struct Commit {
    hash: String,
    author_email: String,
    message: String,
    time: i64,
    files: Vec<File>
}

pub struct File {
    path: String,
    time: i64,
    // added_lines: i32,
    // deleted_lines: i32,
    // changed_lines: i32,
}

pub fn parse_commit(git_commit: &git2::Commit, note_oids: &[Oid]) -> Result<Commit, git2::Error> {
    let mut commit = Commit {
        hash: git_commit.id().to_string(),
        author_email: git_commit.author().to_string(),
        message: git_commit.message().unwrap().to_string(),
        time: git_commit.time().seconds(), // todo: validate
        files: vec![]
    };

    for oid in note_oids {
        let message = oid.to_string();
        commit.files.push(File {
            path: message,
            time: 0,
        });
    }

    return Ok(commit);
}

impl fmt::Display for Commit {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let _ = writeln!(f, "Commit: {}", self.hash);
        let _ = writeln!(f, "Author: {}", self.author_email);
        let _ = writeln!(f, "Time {}", self.time);
        let _ = writeln!(f, "{}", self.message);
        let _ = writeln!(f);

        for file in &self.files {
            let _ = writeln!(f, "{}", &file);
        }
        writeln!(f)
    }
}

impl fmt::Display for File {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{} : {}", self.time, self.path)
    }
}
