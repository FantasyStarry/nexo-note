use crate::cli::Cli;
use crate::commands::open_repo;
use crate::output::{print, ApiResponse};
use crate::storage::git;
use anyhow::Result;

/// Rename a tag across all notes.
pub fn run(cli: &Cli, old: &str, new: &str) -> Result<()> {
    if cli.dry_run {
        print(
            &ApiResponse::ok_with_message(
                format!("would rename tag '{}' to '{}'", old, new),
                "预览模式",
            ),
            cli.json,
        )?;
        return Ok(());
    }

    let repo = open_repo(cli)?;
    let updated = repo.rename_tag(old, new)?;

    if updated > 0 {
        git::commit_all(
            &repo.root,
            &format!("chore: rename tag '{}' to '{}'", old, new),
        )?;
    }

    print(
        &ApiResponse::ok_with_message(
            format!("renamed tag '{}' to '{}' in {} note(s)", old, new, updated),
            "标签重命名成功",
        ),
        cli.json,
    )?;
    Ok(())
}
