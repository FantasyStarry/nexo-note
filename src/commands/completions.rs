use anyhow::{anyhow, Result};
use clap::CommandFactory;
use clap_complete::{generate, Shell};
use std::io;

use crate::cli::Cli;

/// Generate shell completion scripts.
pub fn run(shell: &str) -> Result<()> {
    let shell = shell
        .parse::<Shell>()
        .map_err(|_| anyhow!("unsupported shell '{}'. supported: bash, zsh, fish, powershell, elvish", shell))?;

    let mut cmd = Cli::command();
    generate(shell, &mut cmd, "nexo", &mut io::stdout());
    Ok(())
}
