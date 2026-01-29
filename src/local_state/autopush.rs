// src/local_state/autopush.rs
use std::path::Path;
use std::process::Command;

#[derive(Debug, Clone, Copy)]
pub enum AutopushOutcome {
    SkippedNoOrigin,
    AddFailed,
    CommitFailed,
    PushFailed,
    CommittedAndPushed,
}

pub fn origin_url(repo_path: &Path) -> Option<String> {
    let out = Command::new("git")
        .args(["remote", "get-url", "origin"])
        .current_dir(repo_path)
        .output()
        .ok()?;

    if !out.status.success() {
        return None;
    }

    let s = String::from_utf8_lossy(&out.stdout).trim().to_string();
    if s.is_empty() {
        None
    } else {
        Some(s)
    }
}

pub fn autopush(repo_path: &Path, commit_msg: &str) -> AutopushOutcome {
    if origin_url(repo_path).is_none() {
        return AutopushOutcome::SkippedNoOrigin;
    }

    // git add -A
    let add = Command::new("git")
        .args(["add", "-A"])
        .current_dir(repo_path)
        .status();

    if add.is_err() || !add.unwrap().success() {
        return AutopushOutcome::AddFailed;
    }

    // git commit -m "..."
    let commit = Command::new("git")
        .args(["commit", "-m", commit_msg])
        .current_dir(repo_path)
        .output();

    let commit = match commit {
        Ok(v) => v,
        Err(_) => return AutopushOutcome::CommitFailed,
    };

    if !commit.status.success() {
        // If hooks fail or "nothing to commit", treat as CommitFailed
        return AutopushOutcome::CommitFailed;
    }

    // git push origin
    let push = Command::new("git")
        .args(["push", "origin"])
        .current_dir(repo_path)
        .status();

    if push.is_err() || !push.unwrap().success() {
        return AutopushOutcome::PushFailed;
    }

    AutopushOutcome::CommittedAndPushed
}