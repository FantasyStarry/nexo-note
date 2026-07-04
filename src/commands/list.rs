use crate::cli::Cli;
use crate::commands::open_repo;
use crate::output::{print, ApiResponse, NoteSummary};
use crate::storage::repo::NoteFilters;
use anyhow::Result;
use chrono::FixedOffset;

/// List notes with optional filters.
pub fn run(
    cli: &Cli,
    category: Option<String>,
    tag: Option<String>,
    status: Option<String>,
    limit: Option<usize>,
    since: Option<String>,
) -> Result<()> {
    let repo = open_repo(cli)?;

    let since_dt = since
        .map(|s| {
            chrono::NaiveDate::parse_from_str(&s, "%Y-%m-%d")
                .map(|d| d.and_hms_opt(0, 0, 0).unwrap())
                .map(|dt| dt.and_local_timezone(FixedOffset::east_opt(0).unwrap()).unwrap())
        })
        .transpose()?;

    let filters = NoteFilters {
        category,
        tag,
        status,
        limit,
        since: since_dt,
    };

    let notes = repo.list_notes(&filters)?;
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
