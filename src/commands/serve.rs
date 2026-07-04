use crate::cli::Cli;
use crate::commands::open_repo;

/// Start the MCP server on stdio transport.
pub fn run(cli: &Cli) -> anyhow::Result<()> {
    let repo = open_repo(cli)?;
    let rt = tokio::runtime::Runtime::new()?;
    rt.block_on(crate::mcp::server::serve(repo))?;
    Ok(())
}
