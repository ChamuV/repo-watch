// src/local_state/autopush.rs
use std::path::Path;
use std::process::Command;

use chrono::{DateTime, Local};
use git2::Repository;

#[derive(Debug, Clone, Copy)]
pub enum AutopushOutcome {
    SkippedNoOrigin,
    AddFailed,
    CommitFailed,
    PushFailed,
    CommittedAndPushed,
}

pub fn origin_url(repo_path: &Path) -> Option<String> {
    let repo = Repository::open(repo_path).ok()?;
    let remote = repo.find_remote("origin").ok()?;
    remote.url().map(|s| s.to_string())
}

pub fn has_origin_remote(repo_path: &Path) -> bool {
    origin_url(repo_path).is_some()
}

pub fn autopush(repo_path: &Path, when: DateTime<Local>) -> AutopushOutcome {
    // If no origin, skip.
    if !has_origin_remote(repo_path) {
        return AutopushOutcome::SkippedNoOrigin;
    }

    let repo_str = match repo_path.to_str() {
        Some(s) => s,
        None => return AutopushOutcome::AddFailed, // rare non-UTF8 path
    };

    // git add -A
    let st = Command::new("git")
        .args(["-C", repo_str, "add", "-A"])
        .status();
    if st.map(|s| !s.success()).unwrap_or(true) {
        return AutopushOutcome::AddFailed;
    }

    // git commit -m "Auto-save <time>"
    let msg = format!("Auto-save {}", when.format("%Y-%m-%d %H:%M"));
    let st = Command::new("git")
        .args(["-C", repo_str, "commit", "-m", &msg])
        .status();

    // commit can fail if nothing to commit; in that case, skip push
    if st.map(|s| !s.success()).unwrap_or(true) {
        return AutopushOutcome::CommitFailed;
    }

    // git push
    let st = Command::new("git")
        .args(["-C", repo_str, "push"])
        .status();
    if st.map(|s| !s.success()).unwrap_or(true) {
        return AutopushOutcome::PushFailed;
    }

    AutopushOutcome::CommittedAndPushed
}