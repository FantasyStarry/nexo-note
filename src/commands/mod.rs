pub mod archive;
pub mod completions;
pub mod config_cmd;
pub mod create;
pub mod delete;
pub mod edit;
pub mod init;
pub mod list;
pub mod search;
pub mod stats;
pub mod tag;
pub mod view;

use crate::cli::{Cli, Commands, ConfigCommands, TagCommands};
use crate::config::Config;
use crate::storage::repo::Repo;
use anyhow::Result;

/// Build a Repo instance from CLI args and global config.
pub fn open_repo(cli: &Cli) -> Result<Repo> {
    let config = Config::load()?;
    let notes_dir = config.resolve_notes_dir(cli.notes_dir.as_deref());
    let repo = Repo::new(notes_dir, config.categories.clone());
    repo.init()?;
    Ok(repo)
}

/// Dispatch a parsed CLI command to the appropriate handler.
pub fn run(cli: Cli) -> Result<()> {
    match cli.command {
        Commands::Init { git } => init::run(&cli, git),
        Commands::Create {
            ref title,
            ref category,
            ref tags,
            ref source_url,
            ref extra,
        } => create::run(&cli, title, category, tags.clone(), source_url.clone(), extra.clone()),
        Commands::Edit { ref id, ref editor } => edit::run(&cli, id, editor.clone()),
        Commands::View { ref id } => view::run(&cli, id),
        Commands::Ls {
            ref category,
            ref tag,
            ref status,
            limit,
            ref since,
        } => list::run(
            &cli,
            category.clone(),
            tag.clone(),
            status.clone(),
            limit,
            since.clone(),
        ),
        Commands::Search {
            ref keyword,
            ref tags,
        } => search::run(&cli, keyword, tags.clone()),
        Commands::Archive { ref id } => archive::run(&cli, id),
        Commands::Rm { ref id, force } => delete::run(&cli, id, force),
        Commands::Tag { ref command } => match command {
            TagCommands::Ls => tag::list::run(&cli),
            TagCommands::Mv { old, new } => tag::rename::run(&cli, &old, &new),
            TagCommands::Suggest { id } => tag::suggest::run(&cli, &id),
        },
        Commands::Config { command } => match command {
            ConfigCommands::Set { key, value } => config_cmd::set(&key, &value),
            ConfigCommands::Get { key } => config_cmd::get(&key),
            ConfigCommands::List => config_cmd::list(),
        },
        Commands::Stats => stats::run(&cli),
        Commands::Completions { shell } => completions::run(&shell),
    }
}
