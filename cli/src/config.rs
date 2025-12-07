use crate::error::ClawdError;
use anyhow::{Context, Result};
use std::env;
use std::path::PathBuf;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InstallScope {
    User,
    Project,
}

impl InstallScope {
    pub fn from_str(s: &str) -> Result<Self, ClawdError> {
        match s.to_lowercase().as_str() {
            "user" => Ok(Self::User),
            "project" => Ok(Self::Project),
            _ => Err(ClawdError::InvalidScope(s.to_string())),
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Self::User => "user",
            Self::Project => "project",
        }
    }
}

impl std::fmt::Display for InstallScope {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

pub struct Config {
    pub api_url: String,
    pub install_base_path: PathBuf,
}

impl Config {
    pub fn new(api_url: Option<String>) -> Result<Self> {
        let api_url = api_url
            .or_else(|| env::var("CLAWD_API_URL").ok())
            .unwrap_or_else(|| "https://clawd-api.dcodes.dev".to_string());

        let home = directories::UserDirs::new().context("Could not find home directory")?;
        let install_base_path = home.home_dir().join(".claude/skills");

        Ok(Self {
            api_url,
            install_base_path,
        })
    }

    pub fn resolve_install_path(
        &self,
        skill_name: &str,
        custom_path: Option<PathBuf>,
        scope: InstallScope,
    ) -> Result<PathBuf, ClawdError> {
        if let Some(path) = custom_path {
            return Ok(path);
        }

        match scope {
            InstallScope::User => Ok(self.install_base_path.join(skill_name)),
            InstallScope::Project => Self::find_project_skills_dir()?
                .map(|p| p.join(skill_name))
                .ok_or(ClawdError::NoProjectRoot),
        }
    }

    fn find_project_skills_dir() -> Result<Option<PathBuf>, ClawdError> {
        let current_dir = std::env::current_dir().map_err(ClawdError::FileSystemError)?;

        let mut path = current_dir.as_path();
        loop {
            let claude_dir = path.join(".claude");
            if claude_dir.exists() && claude_dir.is_dir() {
                return Ok(Some(claude_dir.join("skills")));
            }

            match path.parent() {
                Some(parent) => path = parent,
                None => break,
            }
        }

        Ok(None)
    }
}
