pub struct PullRequest;
use octocrab::Octocrab;
use octocrab::{models::repos::DiffEntry, Page};
impl PullRequest {
    pub async fn get_pr(owner: &str, repo: &str) -> Result<Page<DiffEntry>, octocrab::Error> {
        octocrab::instance().pulls(owner, repo).list_files(1).await
    }

    pub async fn post_comment_to_pr(
      github_token: &str,
      owner: &str,
      repo: &str,
      pr_number: u64,
      comment_body: &str,
  ) -> Result<(), Box<dyn std::error::Error>> {
      // Initialize Octocrab client
      let octocrab = Octocrab::builder()
          .personal_token(github_token.to_string())
          .build()?;
  
      // Send the comment to the pull request
      let comment = octocrab
          .issues(owner, repo)
          .create_comment(pr_number, comment_body)
          .await?;
  
      println!("Comment posted: {}", comment.id);
  
      Ok(())
  
}

}
