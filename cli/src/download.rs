use crate::api::clawd::ClawdClient;
use crate::api::github::GitHubClient;
use crate::config::Config;
use crate::error::ClawdError;
use anyhow::Result;
use indicatif::{ProgressBar, ProgressStyle};
use std::fs;
use std::path::PathBuf;

pub async fn execute_download(
    skill_id: String,
    custom_path: Option<PathBuf>,
    api_url: Option<String>,
) -> Result<()> {
    let (author, name) = parse_skill_id(&skill_id)?;

    let config = Config::new(api_url)?;
    let install_path = config.resolve_install_path(&name, custom_path);

    println!("📦 Downloading skill: {}/{}", author, name);

    let clawd = ClawdClient::new(config.api_url);
    let download_info = clawd.get_download_info(&author, &name).await?;

    println!("📍 Source: {}/{}", download_info.repo, download_info.path);

    if install_path.exists() {
        println!(
            "⚠️  Skill already exists at {:?}, will overwrite",
            install_path
        );
        fs::remove_dir_all(&install_path)?;
    }
    fs::create_dir_all(&install_path)?;

    let github = GitHubClient::new();
    let spinner = ProgressBar::new_spinner();
    spinner.set_style(
        ProgressStyle::default_spinner()
            .template("{spinner:.green} {msg}")
            .unwrap(),
    );
    spinner.set_message("Downloading files...");

    github
        .download_directory(
            &download_info.repo,
            &download_info.path,
            download_info.git_ref.as_deref(),
            &install_path,
            &spinner,
        )
        .await?;

    spinner.finish_with_message("✅ Download complete!");
    println!("📂 Installed to: {:?}", install_path);

    Ok(())
}

fn parse_skill_id(skill_id: &str) -> Result<(String, String), ClawdError> {
    let parts: Vec<&str> = skill_id.split('/').collect();
    if parts.len() != 2 {
        return Err(ClawdError::InvalidSkillId(skill_id.to_string()));
    }

    let author = parts[0];
    let name = parts[1];

    if !is_valid_identifier(author) || !is_valid_identifier(name) {
        return Err(ClawdError::InvalidSkillId(skill_id.to_string()));
    }

    Ok((author.to_string(), name.to_string()))
}

fn is_valid_identifier(s: &str) -> bool {
    !s.is_empty()
        && s.chars()
            .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-')
}
