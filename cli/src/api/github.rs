use crate::error::ClawdError;
use indicatif::ProgressBar;
use reqwest::StatusCode;
use serde::Deserialize;
use std::fs;
use std::path::Path;

#[derive(Deserialize)]
pub struct GitHubContent {
    pub name: String,
    pub path: String,
    #[serde(rename = "type")]
    pub content_type: String,
    pub download_url: Option<String>,
}

pub struct GitHubClient {
    client: reqwest::Client,
}

impl GitHubClient {
    pub fn new() -> Self {
        Self {
            client: reqwest::Client::builder()
                .user_agent("clawd-cli")
                .build()
                .unwrap(),
        }
    }

    pub async fn list_contents(
        &self,
        repo: &str,
        path: &str,
        git_ref: Option<&str>,
    ) -> Result<Vec<GitHubContent>, ClawdError> {
        let mut url = format!("https://api.github.com/repos/{}/contents/{}", repo, path);
        if let Some(ref_val) = git_ref {
            url.push_str(&format!("?ref={}", ref_val));
        }

        let response = self
            .client
            .get(&url)
            .header("Accept", "application/vnd.github.v3+json")
            .send()
            .await?;

        match response.status() {
            StatusCode::OK => Ok(response.json().await?),
            StatusCode::FORBIDDEN => Err(ClawdError::GitHubError(
                "GitHub rate limit exceeded".to_string(),
            )),
            status => Err(ClawdError::GitHubError(format!("HTTP {}", status))),
        }
    }

    pub async fn download_file(&self, download_url: &str) -> Result<Vec<u8>, ClawdError> {
        let response = self.client.get(download_url).send().await?;
        Ok(response.bytes().await?.to_vec())
    }

    pub async fn download_directory(
        &self,
        repo: &str,
        path: &str,
        git_ref: Option<&str>,
        target_dir: &Path,
        progress: &ProgressBar,
    ) -> Result<(), ClawdError> {
        let contents = self.list_contents(repo, path, git_ref).await?;

        for item in contents {
            match item.content_type.as_str() {
                "file" => {
                    if let Some(url) = item.download_url {
                        let file_data = self.download_file(&url).await?;
                        let file_path = target_dir.join(&item.name);
                        fs::write(file_path, file_data)?;
                        progress.inc(1);
                    }
                }
                "dir" => {
                    let sub_dir = target_dir.join(&item.name);
                    fs::create_dir_all(&sub_dir)?;
                    Box::pin(
                        self.download_directory(repo, &item.path, git_ref, &sub_dir, progress),
                    )
                    .await?;
                }
                _ => {}
            }
        }

        Ok(())
    }
}
