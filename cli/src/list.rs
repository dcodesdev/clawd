use crate::api::clawd::ClawdClient;
use crate::config::Config;
use crate::error::ClawdError;

pub async fn execute_list(
    page: u32,
    limit: u32,
    api_url: Option<String>,
) -> Result<(), ClawdError> {
    let config = Config::new(api_url).map_err(|e| ClawdError::InvalidResponse(e.to_string()))?;
    let client = ClawdClient::new(config.api_url);

    let response = client.list_skills(page, limit).await?;

    if response.skills.is_empty() {
        println!("No skills found.");
        return Ok(());
    }

    // Print header
    println!(
        "{:<30} {:<20} {:<15} {:>10} {:>8}",
        "ID", "TITLE", "CATEGORY", "DOWNLOADS", "RATING"
    );
    println!("{}", "-".repeat(87));

    // Print skills
    for skill in &response.skills {
        let rating = if skill.rating > 0.0 {
            format!("{:.1}★", skill.rating)
        } else {
            "-".to_string()
        };

        println!(
            "{:<30} {:<20} {:<15} {:>10} {:>8}",
            truncate(&skill.id, 29),
            truncate(&skill.title, 19),
            truncate(&skill.category, 14),
            format_number(skill.download_count),
            rating
        );
    }

    // Print pagination info
    println!();
    println!(
        "Page {} of {} ({} total skills)",
        response.page, response.total_pages, response.total
    );

    Ok(())
}

fn truncate(s: &str, max: usize) -> String {
    if s.len() > max {
        format!("{}…", &s[..max - 1])
    } else {
        s.to_string()
    }
}

fn format_number(n: i64) -> String {
    if n >= 1_000_000 {
        format!("{:.1}M", n as f64 / 1_000_000.0)
    } else if n >= 1_000 {
        format!("{:.1}K", n as f64 / 1_000.0)
    } else {
        n.to_string()
    }
}
