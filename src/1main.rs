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

    // Repositories under users name
    let my_repos = crab
        .current()
        .list_repos_for_authenticated_user()
        .type_("owner")
        .sort("updated")
        .direction("desc")
        .per_page(100)
        .send()
        .await?;

    let mut i = 0;
    for repo in my_repos {
        i += 1;
        let pushed = repo
            .pushed_at
            .map(|t| t.to_rfc3339())
            .unwrap_or_else(|| "unknown".to_string());

        println!("* {} pushed last at {}", repo.name, pushed);

        if i == 10 {
            break;
        }
    }

    
    Ok(())
}