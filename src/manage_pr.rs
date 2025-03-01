pub struct PullRequest;
use std::env;
use octocrab::{models::repos::DiffEntry, Page};
use reqwest::Client;
impl PullRequest {
    pub async fn get_pr(owner: &str, repo: &str, pr_number: u64) -> Result<Page<DiffEntry>, octocrab::Error> {
        octocrab::instance().pulls(owner, repo).list_files(pr_number).await
    }

    pub async fn post_comment_to_pr(repo: &str, pr_content: &str, pr_number: u64) -> Result<(), reqwest::Error> {
        let github_token = env::var("GITHUB_TOKEN").expect("GITHUB_TOKEN not set");
    
        let url = format!(
            "https://api.github.com/repos/{}/issues/{}/comments",
            repo, pr_number
        );
    
        let client = Client::new();
        let response = client
            .post(&url)
            .header("Authorization", format!("Bearer {}", github_token))
            .header("User-Agent", "FibBot")
            .header("Accept", "application/vnd.github.full+json")
            .json(&serde_json::json!({ "body": pr_content }))
            .send()
            .await?;
    
        if response.status().is_success() {
            println!("✅ Comment posted on pr {} successfully.", pr_number);
        } else {
            eprintln!("❌ Failed to post comment: {:?}", response.text().await?);
        }
    
        Ok(())
    }

}
