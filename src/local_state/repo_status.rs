// src/local_state/repo_status.rs
use git2::{Repository, StatusOptions};
use std::path::Path;

#[derive(Debug, Clone)]
pub enum FileStatus {
    Untracked,
    Modified,
    Deleted,
    Renamed,
    TypeChanged,
}

pub fn check_repo_status(repo_path: &Path) -> Result<Vec<(String, FileStatus)>, git2::Error> {
    let repo = Repository::open(repo_path)?;

    let mut status_options = StatusOptions::new();
    status_options.include_untracked(true);

    let statuses = repo.statuses(Some(&mut status_options))?;

    let mut results = Vec::new();

    for entry in statuses.iter() {
        let status = entry.status();
        let path = match entry.path() {
            Some(p) => p.to_string(),
            None => continue,
        };

        if status.is_wt_new() {
            results.push((path, FileStatus::Untracked));
        } else if status.is_wt_modified() {
            results.push((path, FileStatus::Modified));
        } else if status.is_wt_deleted() {
            results.push((path, FileStatus::Deleted));
        } else if status.is_wt_renamed() {
            results.push((path, FileStatus::Renamed));
        } else if status.is_wt_typechange() {
            results.push((path, FileStatus::TypeChanged));
        }
    }

    Ok(results)
}