// src/local_state/discover.rs
use std::{
    fs,
    path::{Path, PathBuf},
};
use walkdir::WalkDir;

pub fn discover_git_repos(root: &Path, max_depth: usize) -> Vec<PathBuf> {
    let mut repos = Vec::new();

    let paths = fs::read_dir(root).expect("Failed to read root directory");

    for path in paths {
        let path = path.expect("Invalid directory entry").path();

        for entry in WalkDir::new(&path)
            .max_depth(max_depth)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            if entry.file_type().is_dir() {
                let folder = entry.path();
                let git_folder_path = folder.join(".git");

                if fs::metadata(&git_folder_path).is_ok() {
                    repos.push(folder.to_path_buf());
                }
            }
        }
    }

    repos
}