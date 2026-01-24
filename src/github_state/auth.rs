// src/github_state/auth.rs
use octocrab::Octocrab;
use crate::github_state::AuthUser;

pub async fn whoami() -> octocrab::Result<AuthUser> {
    // Initialising token
    let token = std::env::var("GITHUB_TOKEN")
        .expect("GITHUB_TOKEN env is required");

    // Intialising Octocrab for token
    let crab = Octocrab::builder()
        .personal_token(token)
        .build()?;

    // Authorisation check
    let me: octocrab::models::UserProfile = 
        crab.get("/user", None::<&()>).await?;

    Ok(AuthUser {
            login: me.login,
            name: me.name.unwrap_or_else(|| "(no name)".to_string()),
    })
} 