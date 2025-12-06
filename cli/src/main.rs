use clap::{Parser, Subcommand};
use std::path::PathBuf;

mod api;
mod config;
mod download;
mod error;

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
    List,
    /// Search for a skill
    Search { query: String },
    /// Download a skill
    Download {
        /// Skill ID in format: author/skill-name
        skill_id: String,

        /// Custom installation path (default: ~/.claude/skills/<skillname>)
        #[arg(short, long)]
        path: Option<PathBuf>,

        /// Override API URL
        #[arg(long, env = "CLAWD_API_URL")]
        api_url: Option<String>,
    },
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::List => println!("Listing skills..."),
        Commands::Search { query } => println!("Searching for: {}", query),
        Commands::Download {
            skill_id,
            path,
            api_url,
        } => {
            download::execute_download(skill_id, path, api_url).await?;
        }
    }

    Ok(())
}
