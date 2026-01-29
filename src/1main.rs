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

// src/main.rs

use chrono::Local;

mod local_state;

use local_state::autopush::{autopush, AutopushOutcome, origin_url};
use local_state::discover::discover_git_repos;
use local_state::path_home::get_home_directory;
use local_state::repo_status::check_repo_status;

fn main() {

    let home = get_home_directory();
    let repos = discover_git_repos(&home, 3);

    for repo_path in repos {
        println!("\nFound Git repository: {}", repo_path.display());

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

        // Remote check (optional log)
        match origin_url(&repo_path) {
            Some(url) => println!("origin: {url}"),
            None => {
                println!("No 'origin' remote found; skipping autopush.");
                continue;
            }
        }

        let now = Local::now();
        println!("Auto-push at {}", now.format("%Y-%m-%d %H:%M"));

        match autopush(&repo_path, now) {
            AutopushOutcome::CommittedAndPushed => println!("Committed + pushed."),
            AutopushOutcome::PushFailed => println!("Push failed (auth/upstream/LFS/behind?)."),
            AutopushOutcome::CommitFailed => println!("Commit failed (nothing staged or hooks)."),
            AutopushOutcome::AddFailed => println!("git add failed."),
            AutopushOutcome::SkippedNoOrigin => println!("No origin remote; skipped.")
        }
    }
}
