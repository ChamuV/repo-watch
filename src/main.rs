use std::{env, fs, process::Command};
use git2::{Repository, StatusOptions};
use walkdir::WalkDir;

fn main() {
    let home = env::var("HOME")
        .or_else(|_| env::var("USERPROFILE"))
        .expect("Could not determine home directory");

    let paths = fs::read_dir(&home).expect("Failed to read home directory");

    for path in paths {
        let path = path.expect("Invalid directory entry").path();

        for entry in WalkDir::new(&path)
            .max_depth(2)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            if !entry.file_type().is_dir() {
                continue;
            }

            let folder = entry.path();
            let git_folder_path = folder.join(".git");

            if fs::metadata(&git_folder_path).is_err() {
                continue;
            }

            println!("\nFound Git repository: {}", folder.display());

            let repo = match Repository::open(folder) {
                Ok(r) => r,
                Err(_) => {
                    println!("Failed to open repository.");
                    continue;
                }
            };

            let mut status_options = StatusOptions::new();
            status_options.include_untracked(true);

            let statuses = match repo.statuses(Some(&mut status_options)) {
                Ok(s) => s,
                Err(_) => {
                    println!("Failed to get repository statuses.");
                    continue;
                }
            };

            let has_untracked = statuses.iter().any(|e| e.status().is_wt_new());

            let has_unstaged = statuses.iter().any(|e| {
                e.status().is_wt_modified()
                    || e.status().is_wt_deleted()
                    || e.status().is_wt_renamed()
                    || e.status().is_wt_typechange()
            });

            if !has_untracked && !has_unstaged {
                println!("Repository is clean.");
                continue; // ✅ prevents wasted commit/push
            }

            // Remote check
            if repo.find_remote("origin").is_err() {
                println!("No 'origin' remote found; skipping autopush.");
                continue;
            }

            // Timestamp (requires `chrono`)
            let now = chrono::Local::now().format("%Y-%m-%d %H:%M").to_string();
            println!("Auto-push at {now}");

            // git add -A
            let st = Command::new("git")
                .args(["-C", folder.to_str().unwrap(), "add", "-A"])
                .status()
                .expect("Failed to run git add");
            if !st.success() {
                println!("git add failed; skipping repo.");
                continue;
            }

            // git commit -m "Auto-save <time>"
            let msg = format!("Auto-save {now}");
            let st = Command::new("git")
                .args([
                    "-C",
                    folder.to_str().unwrap(),
                    "commit",
                    "-m",
                    &msg,
                ])
                .status()
                .expect("Failed to run git commit");

            if st.success() {
                println!("Committed changes.");
            } else {
                println!("Nothing to commit (or commit failed). Skipping push.");
                continue; // ✅ if nothing committed, don’t push
            }

            // git push
            let st = Command::new("git")
                .args(["-C", folder.to_str().unwrap(), "push"])
                .status()
                .expect("Failed to run git push");

            if st.success() {
                println!("Pushed successfully.");
            } else {
                println!("git push failed (possible LFS/size limit, upstream, auth, or behind).");
            }
        }
    }
}