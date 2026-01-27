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
            if entry.file_type().is_dir() {
                let folder = entry.path();
                let git_folder_path = folder.join(".git");

                if fs::metadata(&git_folder_path).is_ok() {
                    println!("Found Git repository: {}", folder.display());

                    // Check repository status
                    let repo = Repository::open(folder)
                        .expect("Failed to open repository");

                    let mut status_options = StatusOptions::new();
                    status_options.include_untracked(true);

                    let statuses = repo
                        .statuses(Some(&mut status_options))
                        .expect("Failed to get repository statuses");

                    let has_untracked = statuses
                        .iter()
                        .any(|entry| entry.status().is_wt_new());

                    let has_unstaged = statuses
                        .iter()
                        .any(|entry| {
                            entry.status().is_wt_modified()
                                || entry.status().is_wt_deleted()
                                || entry.status().is_wt_renamed()
                                || entry.status().is_wt_typechange()
                        });

                    if has_untracked {
                        println!("Repository has untracked files.");
                        println!("Untracked files:");
                        for entry in statuses.iter().filter(|e| e.status().is_wt_new()) {
                            if let Some(path) = entry.path() {
                                println!(" - {}", path);
                            }
                        }
                    }

                    if has_unstaged {
                        println!("Repository has unstaged changes.");
                        println!("Unstaged changes:");
                        for entry in statuses.iter().filter(|e| {
                            e.status().is_wt_modified()
                                || e.status().is_wt_deleted()
                                || e.status().is_wt_renamed()
                                || e.status().is_wt_typechange()
                        }) {
                            if let Some(path) = entry.path() {
                                println!(" - {}", path);
                            }
                        }
                    }

                    if !has_unstaged && !has_untracked {
                        println!("Repository is clean.");
                    }

                    // Check if remote exists
                    let origin_ok = repo.find_remote("origin");

                    if origin_ok.is_err() {
                        println!("No 'origin' remote found; skipping autopush.");
                        continue;
                    }

                    // git add -A
                    let st = Command::new("git")
                        .args(["-C", folder.to_str().unwrap(), "add", "-A"])
                        .status()
                        .expect("Failed to run git add");

                    if !st.success() {
                        println!("git add failed; skipping repo.");
                        continue;
                    }

                    // git commit -m "Auto-save"
                    let st = Command::new("git")
                        .args([
                            "-C",
                            folder.to_str().unwrap(),
                            "commit",
                            "-m",
                            "Auto-save",
                        ])
                        .status()
                        .expect("Failed to run git commit");

                    if st.success() {
                        println!("Committed changes.");
                    } else {
                        println!(
                            "Nothing to commit (or commit failed). Continuing to push anyway..."
                        );
                    }

                    // git push
                    let st = Command::new("git")
                        .args(["-C", folder.to_str().unwrap(), "push"])
                        .status()
                        .expect("Failed to run git push");

                    if st.success() {
                        println!("Pushed successfully.");
                    } else {
                        println!(
                            "git push failed (maybe no upstream set, auth, or behind remote)."
                        );
                    }
                }
            }
        }
    }
}