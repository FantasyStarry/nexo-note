use crate::cli::Cli;
use crate::commands::open_repo;
use crate::models::tag;
use crate::output::{print, ApiResponse, NoteSummary};
use anyhow::Result;

/// Search notes by keyword, optionally filtered by tags.
pub fn run(cli: &Cli, keyword: &str, tags: Option<String>) -> Result<()> {
    let repo = open_repo(cli)?;
    let tag_list = tags.map(|t| tag::parse_tags(&t)).unwrap_or_default();
    let notes = repo.search_notes(keyword, &tag_list)?;

    let summaries: Vec<NoteSummary> = notes
        .into_iter()
        .map(|n| NoteSummary {
            id: n.frontmatter.id,
            date: n.frontmatter.created_at.format("%Y-%m-%d").to_string(),
            status: n.frontmatter.status,
            tags: n.frontmatter.tags,
            title: n.frontmatter.title,
        })
        .collect();

    print(&ApiResponse::ok(summaries), cli.json)?;
    Ok(())
}
