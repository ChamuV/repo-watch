// src/main.rs
use octocrab::Octocrab;

#[tokio::main]
async fn main() -> octocrab::Result<()> {
    // Initialising token
    let token = std::env::var("GITHUB_TOKEN")
        .expect("GITHUB_TOKEN env is required");

    // Intialising Octocrab for toke
    let crab = Octocrab::builder()
        .personal_token(token)
        .build()?;

    // Authorisation check
    let me: octocrab::models::UserProfile = crab.get("/user", None::<&()>).await?;

    let login = me.login.to_string();
    let name = me.name.unwrap_or_else(|| "(no name)".to_string());
    
    // Print message for completion
    println!("Authenticated as: {login} ({name})");
    Ok(())
}