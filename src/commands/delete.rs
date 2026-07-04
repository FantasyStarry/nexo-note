use crate::cli::Cli;
use crate::commands::open_repo;
use crate::output::{print, ApiResponse};
use crate::storage::git;
use anyhow::Result;

/// Delete or archive a note.
pub fn run(cli: &Cli, id: &str, force: bool) -> Result<()> {
    if cli.dry_run {
        print(
            &ApiResponse::ok_with_message(format!("would delete note {}", id), "预览模式"),
            cli.json,
        )?;
        return Ok(());
    }

    let message = if force {
        let repo = open_repo(cli)?;
        repo.backup_note_history(id)?;
        repo.delete_note(id)?;
        git::commit_all(&repo.root, &format!("chore: delete note {}", id))?;
        format!("deleted note {}", id)
    } else {
        // Default behavior: archive instead of delete.
        super::archive::run(cli, id)?;
        return Ok(());
    };

    print(&ApiResponse::ok_with_message(message, "操作成功"), cli.json)?;
    Ok(())
}
