use crate::cli::Cli;
use crate::commands::open_repo;
use crate::models::tag;
use crate::output::{ApiResponse, NoteData, print};
use crate::storage::git;
use anyhow::Result;
use std::collections::HashMap;

/// Create a new note.
pub fn run(
    cli: &Cli,
    title: &str,
    category: &str,
    tags: Option<String>,
    source_url: Option<String>,
    content: Option<String>,
    content_file: Option<String>,
    extra: Vec<String>,
) -> Result<()> {
    let repo = open_repo(cli)?;

    let tag_list = tags.as_deref().map(tag::parse_tags).unwrap_or_default();

    let body = match (content, content_file) {
        (Some(_), Some(_)) => {
            return Err(anyhow::anyhow!(
                "cannot use both --content and --content-file"
            ));
        }
        (Some(text), None) => Some(text),
        (None, Some(path)) => Some(std::fs::read_to_string(path)?),
        (None, None) => None,
    };

    let extra_map = parse_extra(extra)?;

    let note = repo.create_note(
        title,
        category,
        tag_list,
        source_url.as_deref(),
        body.as_deref(),
        extra_map,
    )?;
    let note_id = note.frontmatter.id.clone();
    let path = repo.note_path(&note_id);

    let data = NoteData {
        id: note_id.clone(),
        path,
        frontmatter: note.frontmatter,
        content: note.content,
    };

    git::commit_all(&repo.root, &format!("feat: create note {}", note_id))?;

    print(
        &ApiResponse::ok_with_message(data, "笔记创建成功"),
        cli.json,
    )?;
    Ok(())
}

fn parse_extra(extra: Vec<String>) -> Result<HashMap<String, String>> {
    let mut map = HashMap::new();
    for item in extra {
        let parts: Vec<&str> = item.splitn(2, '=').collect();
        if parts.len() != 2 {
            return Err(anyhow::anyhow!(
                "invalid extra field '{}', expected key=value",
                item
            ));
        }
        map.insert(parts[0].to_string(), parts[1].to_string());
    }
    Ok(map)
}
