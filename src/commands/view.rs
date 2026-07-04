use crate::cli::Cli;
use crate::commands::open_repo;
use crate::output::{print, ApiResponse, NoteData};
use anyhow::{anyhow, Result};

/// View a note by ID.
pub fn run(cli: &Cli, id: &str) -> Result<()> {
    let repo = open_repo(cli)?;
    let note = repo
        .find_note(id)?
        .ok_or_else(|| anyhow!("note '{}' not found", id))?;
    let path = repo.note_path(id);

    let data = NoteData {
        id: id.to_string(),
        path,
        frontmatter: note.frontmatter,
        content: note.content,
    };

    print(&ApiResponse::ok(data), cli.json)?;
    Ok(())
}
