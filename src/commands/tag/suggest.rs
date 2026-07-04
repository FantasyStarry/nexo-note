use crate::cli::Cli;
use crate::commands::open_repo;
use crate::output::{print, ApiResponse};
use anyhow::{anyhow, Result};

/// Suggest tags for a note based on existing tags and content.
pub fn run(cli: &Cli, id: &str) -> Result<()> {
    let repo = open_repo(cli)?;
    let note = repo
        .find_note(id)?
        .ok_or_else(|| anyhow!("note '{}' not found", id))?;

    let all_tags = repo.all_tags()?;
    let mut suggestions = Vec::new();

    let text = format!(
        "{} {}",
        note.frontmatter.title.to_lowercase(),
        note.content.to_lowercase()
    );

    for tag in all_tags {
        if text.contains(&tag) && !note.frontmatter.tags.contains(&tag) {
            suggestions.push(tag);
        }
    }

    print(&ApiResponse::ok(suggestions), cli.json)?;
    Ok(())
}
