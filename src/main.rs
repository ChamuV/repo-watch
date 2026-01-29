// src/main.rs
use chrono::Local;

mod local_state;

use local_state::autopush::{autopush, origin_url, AutopushOutcome};
use local_state::diff_stats::get_diff_stats;
use local_state::discover::discover_git_repos;
use local_state::path_home::get_home_directory;
use local_state::policy::{make_commit_message, should_autopush, MIN_LINE_CHANGES};
use local_state::repo_status::check_repo_status;

fn main() {
    let home = get_home_directory();
    let repos = discover_git_repos(&home, 3);

    for repo_path in repos {
        println!("\nFound Git repository: {}", repo_path.display());

        // Status 
        let changes = match check_repo_status(&repo_path) {
            Ok(v) => v,
            Err(e) => {
                println!("Failed to read repo status: {e}");
                continue;
            }
        };

        if changes.is_empty() {
            println!("Repository is clean.");
            continue;
        }

        println!("Changes:");
        for (path, st) in &changes {
            println!(" - {:?}: {}", st, path);
        }

        // Optional remote check log
        match origin_url(&repo_path) {
            Some(url) => println!("origin: {url}"),
            None => {
                println!("No 'origin' remote found; skipping autopush.");
                continue;
            }
        }

        let stats = match get_diff_stats(&repo_path) {
            Ok(s) => s,
            Err(e) => {
                println!("Failed to compute diff stats: {e}");
                continue;
            }
        };

        println!(
            "Diff stats: {} files, +{}/-{}",
            stats.files_changed, stats.insertions, stats.deletions
        );

        if !should_autopush(&stats) {
            println!(
                "Skipping (below threshold {} lines): {} files, +{}/-{}",
                MIN_LINE_CHANGES, stats.files_changed, stats.insertions, stats.deletions
            );
            continue;
        }

        let now = Local::now();
        let commit_msg = make_commit_message(&stats, now);

        println!("Auto-push at {}", now.format("%Y-%m-%d %H:%M"));
        println!("Commit message: {commit_msg}");

        match autopush(&repo_path, &commit_msg) {
            AutopushOutcome::CommittedAndPushed => println!("Committed + pushed."),
            AutopushOutcome::PushFailed => println!("Push failed (auth/upstream/LFS/behind?)."),
            AutopushOutcome::CommitFailed => println!("Commit failed (nothing staged or hooks)."),
            AutopushOutcome::AddFailed => println!("git add failed."),
            AutopushOutcome::SkippedNoOrigin => println!("No origin remote; skipped."),
        }
    }
}