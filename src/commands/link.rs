use crate::cli::Cli;
use crate::commands::open_repo;
use crate::output::{print, ApiResponse, HumanText};
use anyhow::Result;
use serde::Serialize;

/// Result of linking a single note.
#[derive(Debug, Serialize)]
pub struct LinkData {
    pub id: String,
    pub parent_id: String,
    pub dry_run: bool,
}

impl HumanText for LinkData {
    fn human_text(&self) -> String {
        if self.dry_run {
            format!("将把 {} 关联至 {}", self.id, self.parent_id)
        } else {
            format!("已将 {} 关联至 {}", self.id, self.parent_id)
        }
    }
}

/// Result of the bulk relink operation.
#[derive(Debug, Serialize)]
pub struct RelinkData {
    pub linked: usize,
    pub dry_run: bool,
}

impl HumanText for RelinkData {
    fn human_text(&self) -> String {
        if self.dry_run {
            format!("预览：将关联 {} 条孤立笔记（未执行任何修改）", self.linked)
        } else {
            format!("已关联 {} 条孤立笔记到各自日期的日志", self.linked)
        }
    }
}

/// Link a note into a chain.
///
/// If `parent` is given, the note is attached under that specific note (the
/// parent must exist). Otherwise the note is linked to the journal of its own
/// creation date, matching the auto-link behavior of `create`.
pub fn run_link(cli: &Cli, id: &str, parent: Option<String>) -> Result<()> {
    let repo = open_repo(cli)?;
    let note = repo
        .find_note(id)?
        .ok_or_else(|| anyhow::anyhow!("note '{}' not found", id))?;

    let parent_id = if let Some(pid) = &parent {
        if !repo.database().note_exists(pid)? {
            return Err(anyhow::anyhow!("parent note '{}' not found", pid));
        }
        pid.clone()
    } else {
        let date = note.frontmatter.created_at.date_naive();
        repo.ensure_journal_for_date(date)?
    };

    if cli.dry_run {
        let data = LinkData {
            id: id.to_string(),
            parent_id,
            dry_run: true,
        };
        print(
            &ApiResponse::ok_with_message(data, "预览：未执行任何修改"),
            cli.json,
        )?;
        return Ok(());
    }

    repo.set_parent(id, Some(&parent_id))?;
    let data = LinkData {
        id: id.to_string(),
        parent_id,
        dry_run: false,
    };
    print(&ApiResponse::ok_with_message(data, "笔记关联成功"), cli.json)?;
    Ok(())
}

/// Link every orphaned (parent-less) note to the journal of its creation date.
pub fn run_relink(cli: &Cli) -> Result<()> {
    let repo = open_repo(cli)?;
    let count = repo.relink_orphans(cli.dry_run)?;
    let data = RelinkData {
        linked: count,
        dry_run: cli.dry_run,
    };
    let message = if cli.dry_run {
        "预览完成（未修改）"
    } else {
        "批量关联完成"
    };
    print(&ApiResponse::ok_with_message(data, message), cli.json)?;
    Ok(())
}
