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

    #[error("Invalid scope: {0}. Expected 'user' or 'project'")]
    InvalidScope(String),

    #[error("No project root found. Could not locate .claude directory in current or parent directories.")]
    NoProjectRoot,

    #[error("Interactive prompt error: {0}")]
    PromptError(String),
}
