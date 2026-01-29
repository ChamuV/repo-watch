// src/local_state/skip_path.rs
use std::path::Path;

pub fn should_skip_path(path: &Path) -> bool {
    const SKIP_DIR_NAMES: &[&str] = &[".cache", ".venv", "node_modules", "target", ".git"];

    path.components().any(|c| {
        let s = c.as_os_str().to_string_lossy();
        SKIP_DIR_NAMES.contains(&s.as_ref())
    })
}