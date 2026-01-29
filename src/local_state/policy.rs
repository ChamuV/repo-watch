// src/local_state/policy.rs
use chrono::{DateTime, Local};

use crate::local_state::diff_stats::ChangeSummary;

pub const MIN_LINE_CHANGES: usize = 5;

pub fn should_autopush(stats: &ChangeSummary) -> bool {
    // Option B: any untracked file => autopush
    if stats.untracked > 0 {
        return true;
    }

    let total = stats.insertions + stats.deletions;
    total >= MIN_LINE_CHANGES
}

pub fn make_commit_message(stats: &ChangeSummary, now: DateTime<Local>) -> String {
    format!(
        "Autosave: {} files, +{}/-{}, {} untracked ({})",
        stats.files_changed,
        stats.insertions,
        stats.deletions,
        stats.untracked,
        now.format("%Y-%m-%d %H:%M")
    )
}