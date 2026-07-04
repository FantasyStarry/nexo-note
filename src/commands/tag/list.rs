use crate::cli::Cli;
use crate::commands::open_repo;
use crate::output::{print, ApiResponse};
use anyhow::Result;

/// List all tags in the notebook.
pub fn run(cli: &Cli) -> Result<()> {
    let repo = open_repo(cli)?;
    let tags = repo.all_tags()?;
    print(&ApiResponse::ok(tags), cli.json)?;
    Ok(())
}
