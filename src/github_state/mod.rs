// src/github_state/mod.rs
pub mod auth;
pub mod repos;

use chrono::{DateTime, Utc};

#[derive(Debug, Clone)]
pub struct AuthUser {
    pub login: String,
    pub name: String,
}

#[derive(Debug, Clone)]
pub struct RepoInfo {
    pub repo_name: String,
    pub pushed_at: Option<DateTime<Utc>>,
}

