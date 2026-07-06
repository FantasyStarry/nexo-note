use crate::cli::Cli;
use crate::commands::open_repo;
use crate::config::Config;
use crate::output::{print, ApiResponse, NoteData};
use crate::storage::git;
use anyhow::{anyhow, Context, Result};
use std::fs;

/// Edit a note in the system editor.
///
/// The .md file only stores content; metadata lives in SQLite. We write the
/// current content to disk, let the user edit it, then sync the new content
/// back to the database.
pub fn run(cli: &Cli, id: &str, editor: Option<String>) -> Result<()> {
    let repo = open_repo(cli)?;

    let mut note = repo
        .find_note(id)?
        .ok_or_else(|| anyhow!("note '{}' not found", id))?;
    let path = repo.note_path(id);

    // Backup current version before editing.
    repo.backup_note_history(id)?;

    let config = Config::load()?;
    let editor = config.resolve_editor(editor.as_deref());

    // Ensure the content file exists with the latest content from SQLite.
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    fs::write(&path, &note.content)?;

    // Remember modification time before editing.
    let before = fs::metadata(&path)?.modified()?;

    edit::edit_file(&path)
        .with_context(|| format!("failed to open editor '{}' for {:?}", editor, path))?;

    // After editing, update updated_at if the file changed.
    let after = fs::metadata(&path)?.modified()?;
    if after > before {
        let new_content = fs::read_to_string(&path)?;
        note.content = new_content;
        note.frontmatter.touch();
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
