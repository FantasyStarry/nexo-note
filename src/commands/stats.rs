use crate::cli::Cli;
use crate::commands::open_repo;
use crate::output::{print, ApiResponse, StatsData};
use anyhow::Result;

/// Show notebook statistics.
///
/// Statistics are aggregated directly in SQLite rather than loading all notes
/// into memory, which is faster for large notebooks.
pub fn run(cli: &Cli) -> Result<()> {
    let repo = open_repo(cli)?;
    let data: StatsData = repo.database().stats()?;
    print(&ApiResponse::ok(data), cli.json)?;
    Ok(())
}
