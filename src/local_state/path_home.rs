// src/local_state/path_home.rs
use std::{env, path::PathBuf};

pub fn get_home_directory() -> PathBuf {
    let home = env::var("HOME")
        .or_else(|_| env::var("USERPROFILE"))
        .expect("Could not determine home directory");

    PathBuf::from(home)
}