use thiserror::Error;

#[derive(Error, Debug)]
pub enum ClawdError {
    #[error("Invalid skill ID format: {0}. Expected format: author/skill-name")]
    InvalidSkillId(String),

    #[error("Skill not found: {0}")]
    SkillNotFound(String),

    #[error("API rate limit exceeded. Try again later.")]
    RateLimitExceeded,

    #[error("Network error: {0}")]
    NetworkError(#[from] reqwest::Error),

    #[error("File system error: {0}")]
    FileSystemError(#[from] std::io::Error),

    #[error("GitHub API error: {0}")]
    GitHubError(String),

    #[error("Invalid response: {0}")]
    InvalidResponse(String),
}
