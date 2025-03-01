pub struct PullRequest;
use std::env;
use octocrab::{models::repos::DiffEntry, Page};
use reqwest::Client;
impl PullRequest {
    pub async fn get_pr(owner: &str, repo: &str) -> Result<Page<DiffEntry>, octocrab::Error> {
        octocrab::instance().pulls(owner, repo).list_files(1).await
    }

    pub async fn post_comment_to_pr(pr_content: &str) -> Result<(), reqwest::Error> {
        let repo = env::var("GITHUB_REPOSITORY").expect("GITHUB_REPOSITORY not set");
        let pr_number = env::var("PR_NUMBER")
            .expect("PR_NUMBER not set")
            .parse::<u32>()
            .expect("Invalid PR_NUMBER");
        println!("pr number: {}", pr_number);
    
        let github_token = env::var("GITHUB_TOKEN").expect("GITHUB_TOKEN not set");
    
        let url = format!(
            "https://api.github.com/repos/{}/issues/{}/comments",
            repo, pr_number
        );
    
        let client = Client::new();
        let response = client
            .post(&url)
            .header("Authorization", format!("{}", github_token))
            .header("User-Agent", "FibBot")
            .header("Accept", "application/vnd.github.full+json")
            .json(&serde_json::json!({ "body": pr_content }))
            .send()
            .await?;
    
        if response.status().is_success() {
            println!("✅ Comment posted successfully.");
        } else {
            eprintln!("❌ Failed to post comment: {:?}", response.text().await?);
        }
    
        Ok(())
    }

}
