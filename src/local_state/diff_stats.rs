// src/local_state/diff_stats.rs
use anyhow::{anyhow, Result};
use std::path::Path;
use std::process::Command;

#[derive(Debug, Clone)]
pub struct ChangeSummary {
    pub files_changed: usize,
    pub insertions: usize,
    pub deletions: usize,
    pub untracked: usize,
}

fn count_untracked(repo_path: &Path) -> Result<usize> {
    let output = Command::new("git")
        .args(["status", "--porcelain"])
        .current_dir(repo_path)
        .output()?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(anyhow!("git status --porcelain failed: {stderr}"));
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    Ok(stdout.lines().filter(|l| l.starts_with("?? ")).count())
}

pub fn get_diff_stats(repo_path: &Path) -> Result<ChangeSummary> {
    // Option B: untracked is handled separately from diff shortstat
    let untracked = count_untracked(repo_path)?;

    // `git diff --shortstat` reports working-tree changes (unstaged + staged vs HEAD)
    let output = Command::new("git")
        .args(["diff", "--shortstat"])
        .current_dir(repo_path)
        .output()?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(anyhow!("git diff --shortstat failed: {stderr}"));
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    let s = stdout.trim();

    if s.is_empty() {
        return Ok(ChangeSummary {
            files_changed: 0,
            insertions: 0,
            deletions: 0,
            untracked,
        });
    }

    let mut files_changed = 0usize;
    let mut insertions = 0usize;
    let mut deletions = 0usize;

    for part in s.split(',').map(|x| x.trim()) {
        let words: Vec<&str> = part.split_whitespace().collect();
        if words.is_empty() {
            continue;
        }

        if part.ends_with("file changed") || part.ends_with("files changed") {
            files_changed = words[0].parse()?;
        } else if part.contains("insertion") {
            insertions = words[0].parse()?;
        } else if part.contains("deletion") {
            deletions = words[0].parse()?;
        }
    }

    Ok(ChangeSummary {
        files_changed,
        insertions,
        deletions,
        untracked,
    })
}