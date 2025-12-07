use crate::config::InstallScope;
use crate::error::ClawdError;
use dialoguer::{Confirm, Select};
use std::path::Path;

pub fn prompt_scope() -> Result<InstallScope, ClawdError> {
    let options = vec![
        "User (~/.claude/skills) - Available to all projects",
        "Project (./.claude/skills) - Only this project",
    ];

    let selection = Select::new()
        .with_prompt("Select installation scope")
        .items(&options)
        .default(0)
        .interact()
        .map_err(|e| ClawdError::PromptError(e.to_string()))?;

    match selection {
        0 => Ok(InstallScope::User),
        1 => Ok(InstallScope::Project),
        _ => unreachable!(),
    }
}

pub fn prompt_overwrite(path: &Path) -> Result<bool, ClawdError> {
    Confirm::new()
        .with_prompt(format!("Skill already exists at {:?}. Overwrite?", path))
        .default(false)
        .interact()
        .map_err(|e| ClawdError::PromptError(e.to_string()))
}
