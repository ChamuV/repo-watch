// src/github_state/repos.rs
use octocrab::Octocrab;
use crate::github_state::RepoInfo;

pub async fn list_my_repos() -> octocrab::Result<Vec<RepoInfo>> {
    let token = std::env::var("GITHUB_TOKEN")
        .expect("GITHUB_TOKEN env is required");

    let crab = Octocrab::builder()
        .personal_token(token)
        .build()?;

    let repo_list = crab
        .current()
        .list_repos_for_authenticated_user()
        .type_("owner")
        .per_page(100)
        .send()
        .await?;

    let repos: Vec<RepoInfo> = repo_list
        .into_iter()
        .map(|r| RepoInfo {
            repo_name: r.name,
            pushed_at: r.pushed_at,
        })
        .collect();

    Ok(repos)
}