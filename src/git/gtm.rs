use std::collections::HashMap;
use std::fmt;

use git2::{DiffOptions, Note};
use lazy_static::lazy_static;
use regex::Regex;
use serde::export::Formatter;
use serde::Serialize;

lazy_static! {
    static ref NOTE_HEADER_REGEX: Regex = Regex::new("\\[ver:\\d+,total:\\d+]").unwrap();
    static ref NOTE_HEADER_VALS_REGEX: Regex = Regex::new("\\d+").unwrap();
}

#[derive(Serialize)]
pub struct Commit {
    hash: String,
    // branch: String,
    author_email: String,
    message: String,
    time: i64,
    files: Vec<File>,
}

#[derive(Serialize)]
pub struct File {
    path: String,
    time_total: i64,
    timeline: HashMap<i64, i32>,
    status: String,
    added_lines: i32,
    deleted_lines: i32,
}

pub fn parse_commit(repo: &git2::Repository, git_commit: &git2::Commit, notes: &[Note]) -> Result<Commit, git2::Error> {
    let mut commit = Commit {
        hash: git_commit.id().to_string(),
        // branch: "".to_string(),
        author_email: git_commit.author().to_string(),
        message: git_commit.message().unwrap().to_string(),
        time: git_commit.time().seconds(), // todo: validate
        files: vec![],
    };

    for note in notes {
        let message = note.message().unwrap();
        let mut files = parse_note_message(message).unwrap_or(vec![]);
        let _diff = diff_parents(files.as_mut(), git_commit, repo);
        commit.files.append(files.as_mut());
    }

    return Ok(commit);
}

fn parse_note_message(message: &str) -> Option<Vec<File>> {
    let mut version: String = "".to_string();
    let mut files: Vec<File> = Vec::new();
    let lines = message.split("\n");
    for line in lines {
        if line.trim() == "" {
            version = "".to_string();
        } else if NOTE_HEADER_REGEX.is_match(line) {
            let matches: Vec<String> = NOTE_HEADER_VALS_REGEX.find_iter(line)
                .filter_map(|d| d.as_str().parse().ok())
                .collect();
            version = matches.get(0)?.clone();
        }

        let mut file = File {
            path: "".to_string(),
            time_total: 0,
            timeline: HashMap::new(),
            status: "".to_string(),
            added_lines: 0,
            deleted_lines: 0,
        };

        if version == "1" {
            let field_groups: Vec<&str> = line.split(",").collect();
            if field_groups.len() < 3 {
                continue;
            }
            for i in 0..field_groups.len() {
                let fields: Vec<&str> = field_groups.get(i)?.split(":").collect();
                if i == 0 && fields.len() == 2 {
                    file.path = fields.get(0)?.replace("->", ":");
                    file.time_total = fields.get(1)?.parse().unwrap_or(0);
                } else if i == field_groups.len() - 1 && fields.len() == 1 {
                    file.status = fields.get(0)?.to_string();
                } else if fields.len() == 2 {
                    let epoch_timeline: i64 = fields.get(0)?.parse().unwrap_or(0);
                    let epoch_total: i32 = fields.get(1)?.parse().unwrap_or(0);
                    file.timeline.insert(epoch_timeline, epoch_total);
                }
            }
        } else {
            continue;
        }

        let mut found: bool = false;
        for mut added_file in files.iter_mut() {
            if added_file.path == file.path {
                added_file.time_total += file.time_total;
                for (epoch, secs) in &file.timeline {
                    added_file.timeline.insert(*epoch, *secs);
                }
                found = true;
            }
        }
        if !found {
            files.push(file);
        }
    }
    return Option::from(files);
}

fn diff_parents(files: &mut Vec<File>, commit: &git2::Commit, repo: &git2::Repository) -> Result<(), git2::Error> {
    if commit.parent_count() == 0 {
        // TODO: Figure out how to handle initial commit
        return Ok(());
    }

    let parent = commit.parent(0)?;
    let child_tree = commit.tree()?;
    let parent_tree = parent.tree()?;

    for mut file in files {
        if file.path.ends_with(".app") {
            continue; // Skip app events
        }
        let mut diff_options = DiffOptions::new();
        diff_options.pathspec(&file.path);
        let diff = repo.diff_tree_to_tree(
            Option::from(&parent_tree),
            Option::from(&child_tree),
            Option::from(&mut diff_options))?;
        let diff_stats = diff.stats()?;
        file.added_lines = diff_stats.insertions() as i32;
        file.deleted_lines = diff_stats.deletions() as i32;
    }

    return Ok(());
}

impl fmt::Display for Commit {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let _ = writeln!(f, "Commit: {}", self.hash);
        let _ = writeln!(f, "Author: {}", self.author_email);
        let _ = writeln!(f, "Time {}", self.time);
        let _ = writeln!(f, "{}", self.message);

        for file in &self.files {
            let _ = writeln!(f, "{}", &file);
        }
        Ok(())
    }
}

impl fmt::Display for File {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:>2}h{:>3}m{:>3}s : {:<45} +{:<4} - {:<4} {}",
               self.time_total / 3600,
               (self.time_total % 3600) / 60,
               self.time_total % 60,
               self.path,
               self.added_lines,
               self.deleted_lines,
               self.status,
        )
    }
}
