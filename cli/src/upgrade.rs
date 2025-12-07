use anyhow::{bail, Context, Result};
use indicatif::{ProgressBar, ProgressStyle};
use serde::Deserialize;
use std::env;
use std::fs;
use std::os::unix::fs::PermissionsExt;

const REPO: &str = "dcodesdev/clawd";
const CURRENT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Debug, Deserialize)]
struct Release {
    tag_name: String,
    assets: Vec<Asset>,
}

#[derive(Debug, Deserialize)]
struct Asset {
    name: String,
    browser_download_url: String,
}

fn detect_platform() -> Result<String> {
    let os = if cfg!(target_os = "macos") {
        "darwin"
    } else if cfg!(target_os = "linux") {
        "linux"
    } else if cfg!(target_os = "windows") {
        "windows"
    } else {
        bail!("Unsupported operating system");
    };

    let arch = if cfg!(target_arch = "x86_64") {
        "amd64"
    } else if cfg!(target_arch = "aarch64") {
        "arm64"
    } else {
        bail!("Unsupported architecture");
    };

    let ext = if cfg!(target_os = "windows") {
        ".exe"
    } else {
        ""
    };

    Ok(format!("clawd-{}-{}{}", os, arch, ext))
}

async fn get_latest_release(client: &reqwest::Client) -> Result<Release> {
    let url = format!("https://api.github.com/repos/{}/releases/latest", REPO);

    let response = client
        .get(&url)
        .header("User-Agent", "clawd-cli")
        .header("Accept", "application/vnd.github.v3+json")
        .send()
        .await
        .context("Failed to fetch latest release")?;

    if !response.status().is_success() {
        bail!(
            "Failed to fetch release info: HTTP {}",
            response.status().as_u16()
        );
    }

    response
        .json::<Release>()
        .await
        .context("Failed to parse release info")
}

fn version_to_comparable(version: &str) -> String {
    version.trim_start_matches('v').to_string()
}

fn is_newer_version(current: &str, latest: &str) -> bool {
    let current = version_to_comparable(current);
    let latest = version_to_comparable(latest);

    // Simple semver comparison
    let current_parts: Vec<u32> = current.split('.').filter_map(|s| s.parse().ok()).collect();
    let latest_parts: Vec<u32> = latest.split('.').filter_map(|s| s.parse().ok()).collect();

    for i in 0..3 {
        let c = current_parts.get(i).unwrap_or(&0);
        let l = latest_parts.get(i).unwrap_or(&0);
        if l > c {
            return true;
        }
        if l < c {
            return false;
        }
    }
    false
}

pub async fn execute_upgrade(force: bool) -> Result<()> {
    let client = reqwest::Client::new();

    println!("Checking for updates...");

    let release = get_latest_release(&client).await?;
    let latest_version = version_to_comparable(&release.tag_name);
    let current_version = version_to_comparable(CURRENT_VERSION);

    println!("Current version: v{}", current_version);
    println!("Latest version:  v{}", latest_version);

    if !force && !is_newer_version(&current_version, &latest_version) {
        println!("\nYou're already on the latest version!");
        return Ok(());
    }

    if force && current_version == latest_version {
        println!("\nForce reinstalling v{}...", latest_version);
    } else {
        println!("\nUpgrading to v{}...", latest_version);
    }

    let binary_name = detect_platform()?;

    let asset = release
        .assets
        .iter()
        .find(|a| a.name == binary_name)
        .context(format!("No binary found for platform: {}", binary_name))?;

    // Download the new binary
    let pb = ProgressBar::new_spinner();
    pb.set_style(
        ProgressStyle::default_spinner()
            .template("{spinner:.green} {msg}")
            .unwrap(),
    );
    pb.set_message("Downloading...");

    let response = client
        .get(&asset.browser_download_url)
        .header("User-Agent", "clawd-cli")
        .send()
        .await
        .context("Failed to download binary")?;

    if !response.status().is_success() {
        bail!("Failed to download: HTTP {}", response.status().as_u16());
    }

    let bytes = response.bytes().await.context("Failed to read binary")?;

    pb.set_message("Installing...");

    // Get current executable path
    let current_exe = env::current_exe().context("Failed to get current executable path")?;

    // Create temp file in the same directory
    let temp_path = current_exe.with_extension("new");
    fs::write(&temp_path, &bytes).context("Failed to write new binary")?;

    // Set executable permissions on Unix
    #[cfg(unix)]
    {
        let mut perms = fs::metadata(&temp_path)?.permissions();
        perms.set_mode(0o755);
        fs::set_permissions(&temp_path, perms)?;
    }

    // Replace old binary with new one
    #[cfg(unix)]
    {
        let backup_path = current_exe.with_extension("old");
        // Move current to backup
        if let Err(e) = fs::rename(&current_exe, &backup_path) {
            // Try with elevated permissions hint
            fs::remove_file(&temp_path).ok();
            bail!("Failed to replace binary (try with sudo): {}", e);
        }
        // Move new to current
        if let Err(e) = fs::rename(&temp_path, &current_exe) {
            // Restore backup
            fs::rename(&backup_path, &current_exe).ok();
            bail!("Failed to install new binary: {}", e);
        }
        // Remove backup
        fs::remove_file(&backup_path).ok();
    }

    #[cfg(windows)]
    {
        // On Windows, we need to use a different approach since the running exe is locked
        let batch_content = format!(
            r#"@echo off
timeout /t 1 /nobreak >nul
move /y "{}" "{}"
del "%~f0"
"#,
            temp_path.display(),
            current_exe.display()
        );
        let batch_path = current_exe.with_extension("bat");
        fs::write(&batch_path, batch_content)?;
        std::process::Command::new("cmd")
            .args(["/C", "start", "/min", "", batch_path.to_str().unwrap()])
            .spawn()?;
    }

    pb.finish_with_message("Done!");

    println!("\nSuccessfully upgraded to v{}!", latest_version);
    println!("Run 'clawd --version' to verify.");

    Ok(())
}
