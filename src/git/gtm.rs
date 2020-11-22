use std::collections::HashMap;
use std::fmt;

use git2::Note;
use lazy_static::lazy_static;
use regex::Regex;
use serde::export::Formatter;

lazy_static! {
    static ref NOTE_HEADER_REGEX: Regex = Regex::new("\\[ver:\\d+,total:\\d+]").unwrap();
    static ref NOTE_HEADER_VALS_REGEX: Regex = Regex::new("\\d+").unwrap();
}

pub struct Commit {
    hash: String,
    author_email: String,
    message: String,
    time: i64,
    files: Vec<File>,
}

pub struct File {
    path: String,
    time_total: i64,
    timeline: HashMap<i64, i32>,
    // added_lines: i32,
    // deleted_lines: i32,
    // changed_lines: i32,
}

pub fn parse_commit(git_commit: &git2::Commit, notes: &[Note]) -> Result<Commit, git2::Error> {
    let mut commit = Commit {
        hash: git_commit.id().to_string(),
        author_email: git_commit.author().to_string(),
        message: git_commit.message().unwrap().to_string(),
        time: git_commit.time().seconds(), // todo: validate
        files: vec![],
    };

    for note in notes {
        let message = note.message().unwrap();
        commit.files.append(&mut parse_note_message(message));
    }

    return Ok(commit);
}

fn parse_note_message(message: &str) -> Vec<File> {
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
            version = matches.get(0).unwrap().clone();
        }
        if version == "1" {
            let field_groups: Vec<&str> = line.split(",").collect();
            let mut file = File {
                path: "".to_string(),
                time_total: 0,
                timeline: HashMap::new(),
            };

            if field_groups.len() < 3 {
                continue;
            }
            for i in 0..field_groups.len() {
                let fields: Vec<&str> = field_groups.get(i).unwrap().split(":").collect();
                if i == 0 && fields.len() == 2 {
                    file.path = fields.get(0).unwrap().replace("->", ":");
                    file.time_total = fields.get(1).unwrap().parse().unwrap();
                } else if i == field_groups.len() - 1 && fields.len() == 1 {
                    // todo: file status?
                } else if fields.len() == 2 {
                    let epoch_timeline: i64 = fields.get(0).unwrap().parse().unwrap();
                    let epoch_total: i32 = fields.get(1).unwrap().parse().unwrap();
                    file.timeline.insert(epoch_timeline, epoch_total);
                }
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
    }
    return files;
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
        write!(f, "{:>2}h{:>3}m{:>3}s : {}", self.time_total / 3600, (self.time_total % 3600) / 60, self.time_total % 60, self.path)
    }
}
