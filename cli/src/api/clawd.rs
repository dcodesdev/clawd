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

#[derive(Deserialize)]
pub struct ListResponse {
    pub skills: Vec<SkillResponse>,
    pub total: u32,
    pub page: u32,
    pub limit: u32,
    pub total_pages: u32,
}

#[derive(Deserialize)]
pub struct SkillResponse {
    pub id: String,
    pub title: String,
    pub description: String,
    pub category: String,
    #[serde(default)]
    pub tags: Vec<String>,
    pub author: Option<Author>,
    pub download_count: i64,
    pub rating: f64,
}

#[derive(Deserialize)]
pub struct Author {
    pub name: String,
    pub github: String,
    pub url: Option<String>,
    pub avatar: Option<String>,
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

    pub async fn list_skills(&self, page: u32, limit: u32) -> Result<ListResponse, ClawdError> {
        let url = format!("{}/api/skills?page={}&limit={}", self.base_url, page, limit);

        let response = self.client.get(&url).send().await?;

        match response.status() {
            StatusCode::OK => Ok(response.json().await?),
            StatusCode::TOO_MANY_REQUESTS => Err(ClawdError::RateLimitExceeded),
            status => Err(ClawdError::InvalidResponse(format!("HTTP {}", status))),
        }
    }
}
