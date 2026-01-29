// src/local_state/policy.rs
use chrono::{DateTime, Local};

use crate::local_state::diff_stats::ChangeSummary;

pub const MIN_LINE_CHANGES: usize = 5;

pub fn should_autopush(stats: &ChangeSummary) -> bool {
    let total = stats.insertions + stats.deletions;
    total >= MIN_LINE_CHANGES
}

pub fn make_commit_message(stats: &ChangeSummary, now: DateTime<Local>) -> String {
    format!(
        "Autosave: {} files, +{}/-{} ({})",
        stats.files_changed,
        stats.insertions,
        stats.deletions,
        now.format("%Y-%m-%d %H:%M")
    )
}