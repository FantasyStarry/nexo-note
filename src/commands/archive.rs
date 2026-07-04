use crate::cli::Cli;
use crate::commands::open_repo;
use crate::output::{print, ApiResponse};
use crate::storage::git;
use anyhow::Result;

/// Archive a note by ID.
pub fn run(cli: &Cli, id: &str) -> Result<()> {
    if cli.dry_run {
        print(
            &ApiResponse::ok_with_message(format!("would archive note {}", id), "预览模式"),
            cli.json,
        )?;
        return Ok(());
    }

    let repo = open_repo(cli)?;
    repo.backup_note_history(id)?;
    repo.archive_note(id)?;

    git::commit_all(&repo.root, &format!("chore: archive note {}", id))?;

    print(
        &ApiResponse::ok_with_message(format!("archived note {}", id), "归档成功"),
        cli.json,
    )?;
    Ok(())
}
