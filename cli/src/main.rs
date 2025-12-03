use clap::{Parser, Subcommand};

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
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::List => println!("Listing skills..."),
        Commands::Search { query } => println!("Searching for: {}", query),
    }
}
