use crate::cli::Cli;
use crate::commands::open_repo;
use crate::output::{print, ApiResponse, CategoryCount, StatsData, StatusCount, TagCount};
use anyhow::Result;
use std::collections::HashMap;

/// Show notebook statistics.
pub fn run(cli: &Cli) -> Result<()> {
    let repo = open_repo(cli)?;
    let notes = repo.list_notes(&crate::storage::repo::NoteFilters::default())?;

    let total_notes = notes.len();

    let mut category_map: HashMap<String, CategoryCount> = HashMap::new();
    let mut status_count = StatusCount::default();
    let mut tag_counts: HashMap<String, usize> = HashMap::new();
    let mut this_month = 0;

    let now = chrono::Local::now();
    let current_year_month = now.format("%Y-%m").to_string();

    for note in &notes {
        let cat = category_map
            .entry(note.frontmatter.category.clone())
            .or_insert_with(|| CategoryCount {
                category: note.frontmatter.category.clone(),
                total: 0,
                active: 0,
                archived: 0,
            });
        cat.total += 1;
        match note.frontmatter.status.as_str() {
            "active" => {
                cat.active += 1;
                status_count.active += 1;
            }
            "archived" => {
                cat.archived += 1;
                status_count.archived += 1;
            }
            "draft" => status_count.draft += 1,
            _ => status_count.other += 1,
        }

        for tag in &note.frontmatter.tags {
            *tag_counts.entry(tag.clone()).or_insert(0) += 1;
        }

        if note.frontmatter.created_at.format("%Y-%m").to_string() == current_year_month {
            this_month += 1;
        }
    }

    let mut by_category: Vec<CategoryCount> = category_map.into_values().collect();
    by_category.sort_by(|a, b| a.category.cmp(&b.category));

    let mut top_tags: Vec<TagCount> = tag_counts
        .into_iter()
        .map(|(tag, count)| TagCount { tag, count })
        .collect();
    let total_tags = top_tags.len();

    top_tags.sort_by(|a, b| b.count.cmp(&a.count).then_with(|| a.tag.cmp(&b.tag)));
    top_tags.truncate(10);

    let data = StatsData {
        total_notes,
        by_category,
        by_status: status_count,
        this_month,
        total_tags,
        top_tags,
    };

    print(&ApiResponse::ok(data), cli.json)?;
    Ok(())
}
