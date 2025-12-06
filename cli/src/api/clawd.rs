use crate::error::ClawdError;
use reqwest::StatusCode;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct DownloadResponse {
    pub repo: String,
    pub path: String,
    #[serde(rename = "ref")]
    pub git_ref: Option<String>,
}

pub struct ClawdClient {
    base_url: String,
    client: reqwest::Client,
}

impl ClawdClient {
    pub fn new(base_url: String) -> Self {
        Self {
            base_url,
            client: reqwest::Client::new(),
        }
    }

    pub async fn get_download_info(
        &self,
        author: &str,
        name: &str,
    ) -> Result<DownloadResponse, ClawdError> {
        let url = format!("{}/api/skills/{}/{}/download", self.base_url, author, name);

        let response = self.client.get(&url).send().await?;

        match response.status() {
            StatusCode::OK => Ok(response.json().await?),
            StatusCode::NOT_FOUND => Err(ClawdError::SkillNotFound(format!("{}/{}", author, name))),
            StatusCode::TOO_MANY_REQUESTS => Err(ClawdError::RateLimitExceeded),
            status => Err(ClawdError::InvalidResponse(format!("HTTP {}", status))),
        }
    }
}
