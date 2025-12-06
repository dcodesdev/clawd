use anyhow::{Context, Result};
use std::env;
use std::path::PathBuf;

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

    pub fn resolve_install_path(&self, skill_name: &str, custom_path: Option<PathBuf>) -> PathBuf {
        custom_path.unwrap_or_else(|| self.install_base_path.join(skill_name))
    }
}
