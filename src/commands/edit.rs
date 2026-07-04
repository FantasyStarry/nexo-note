use crate::cli::Cli;
use crate::commands::open_repo;
use crate::config::Config;
use crate::output::{print, ApiResponse, NoteData};
use crate::storage::git;
use anyhow::{anyhow, Context, Result};
use std::fs;

/// Edit a note in the system editor.
pub fn run(cli: &Cli, id: &str, editor: Option<String>) -> Result<()> {
    let repo = open_repo(cli)?;
    let path = repo.note_path(id);

    if !path.exists() {
        return Err(anyhow!("note '{}' not found", id));
    }

    // Backup current version before editing.
    repo.backup_note_history(id)?;

    let config = Config::load()?;
    let editor = config.resolve_editor(editor.as_deref());

    // Remember modification time before editing.
    let before = fs::metadata(&path)?.modified()?;

    edit::edit_file(&path)
        .with_context(|| format!("failed to open editor '{}' for {:?}", editor, path))?;

    // After editing, update updated_at if the file changed.
    let after = fs::metadata(&path)?.modified()?;
    if after > before {
        // Re-read the edited .md file and sync back to SQLite.
        let mut note = repo
            .find_note(id)?
            .ok_or_else(|| anyhow!("note '{}' not found after edit", id))?;
        note.frontmatter.touch();

        // Re-parse the file to get the latest content + frontmatter.
        if let Some(file_note) = crate::storage::repo::parse_note_file(&path)? {
            note.content = file_note.content;
            note.frontmatter.title = file_note.frontmatter.title;
        }

        repo.save_note(&note)?;
    }

    let note = repo
        .find_note(id)?
        .ok_or_else(|| anyhow!("note '{}' not found", id))?;
    let data = NoteData {
        id: id.to_string(),
        path: path.clone(),
        frontmatter: note.frontmatter,
        content: note.content,
    };

    git::commit_all(&repo.root, &format!("docs: update note {}", id))?;

    print(
        &ApiResponse::ok_with_message(data, "笔记更新成功"),
        cli.json,
    )?;
    Ok(())
}
