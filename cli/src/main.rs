use clap::{Parser, Subcommand};
use std::path::PathBuf;

mod api;
mod config;
mod download;
mod error;
mod list;
mod prompts;
mod upgrade;

#[derive(Parser)]
#[command(name = "clawd")]
#[command(about = "CLI for Clawd Skills", version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// List available skills
    List {
        /// Page number (default: 1)
        #[arg(short, long, default_value_t = 1)]
        page: u32,

        /// Items per page (default: 20, max: 100)
        #[arg(short, long, default_value_t = 20)]
        limit: u32,

        /// Override API URL
        #[arg(long, env = "CLAWD_API_URL")]
        api_url: Option<String>,
    },
    /// Search for a skill
    Search { query: String },
    /// Download a skill
    Download {
        /// Skill ID in format: author/skill-name
        skill_id: String,

        /// Installation scope: "user" (~/.claude/skills) or "project" (./.claude/skills)
        #[arg(short, long, value_name = "SCOPE")]
        scope: Option<String>,

        /// Force overwrite without confirmation
        #[arg(short, long)]
        force: bool,

        /// Custom installation path (overrides scope)
        #[arg(short, long)]
        path: Option<PathBuf>,

        /// Override API URL
        #[arg(long, env = "CLAWD_API_URL")]
        api_url: Option<String>,
    },
    /// Upgrade clawd to the latest version
    Upgrade {
        /// Force reinstall even if already on latest version
        #[arg(short, long)]
        force: bool,
    },
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::List {
            page,
            limit,
            api_url,
        } => {
            list::execute_list(page, limit, api_url).await?;
        }
        Commands::Search { query } => println!("Searching for: {}", query),
        Commands::Download {
            skill_id,
            scope,
            force,
            path,
            api_url,
        } => {
            download::execute_download(skill_id, scope, force, path, api_url).await?;
        }
        Commands::Upgrade { force } => {
            upgrade::execute_upgrade(force).await?;
        }
    }

    Ok(())
}
