use crate::cli::Cli;
use crate::commands::open_repo;
use crate::output::{print, ApiResponse, HumanText};
use anyhow::{Context, Result};
use serde::Serialize;
use std::path::Path;

/// Migrate existing .md notes from the file system into SQLite.
pub fn run(cli: &Cli) -> Result<()> {
    let repo = open_repo(cli)?;

    // Check if we already have notes in the database.
    let existing = repo.database().note_count()?;
    if existing > 0 {
        print(
            &ApiResponse::ok_with_message(
                MigrateData {
                    imported: 0,
                    skipped: existing,
                    total: existing,
                },
                &format!("数据库已包含 {} 条笔记，跳过迁移", existing),
            ),
            cli.json,
        )?;
        return Ok(());
    }

    // Scan for .md files.
    let mut files: Vec<std::path::PathBuf> = Vec::new();
    collect_md_files(&repo.root.join("notes"), &mut files);
    collect_md_files(&repo.root.join("archive"), &mut files);

    if files.is_empty() {
        print(
            &ApiResponse::ok_with_message(
                MigrateData {
                    imported: 0,
                    skipped: 0,
                    total: 0,
                },
                "未找到可迁移的 .md 文件",
            ),
            cli.json,
        )?;
        return Ok(());
    }

    files.sort();

    let mut imported = 0usize;
    let mut skipped = 0usize;
    let mut errors: Vec<String> = Vec::new();

    for path in &files {
        match parse_and_import(&repo, path) {
            Ok(true) => imported += 1,
            Ok(false) => skipped += 1,
            Err(e) => {
                errors.push(format!("{:?}: {}", path, e));
                skipped += 1;
            }
        }
    }

    let data = MigrateData {
        imported,
        skipped,
        total: imported + skipped,
    };

    let summary = if errors.is_empty() {
        format!(
            "迁移完成：成功 {} 条，跳过 {} 条，共 {} 条",
            imported, skipped, data.total
        )
    } else {
        format!(
            "迁移完成：成功 {} 条，跳过 {} 条（包含 {} 个错误），共 {} 条\n错误详情:\n  {}",
            imported,
            skipped,
            errors.len(),
            data.total,
            errors.join("\n  ")
        )
    };

    print(&ApiResponse::ok_with_message(data, summary), cli.json)?;
    Ok(())
}

fn collect_md_files(dir: &Path, files: &mut Vec<std::path::PathBuf>) {
    if !dir.exists() {
        return;
    }
    if let Ok(entries) = std::fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                collect_md_files(&path, files);
            } else if path.extension().and_then(|s| s.to_str()) == Some("md") {
                files.push(path);
            }
        }
    }
}

fn parse_and_import(
    repo: &crate::storage::repo::Repo,
    path: &std::path::Path,
) -> Result<bool> {
    let content = std::fs::read_to_string(path)
        .with_context(|| format!("failed to read {:?}", path))?;

    if !content.starts_with("---\n") {
        // Not a valid nexo note.
        return Ok(false);
    }

    let parts: Vec<&str> = content[4..].splitn(2, "\n---").collect();
    if parts.len() != 2 {
        return Ok(false);
    }

    let frontmatter: crate::models::frontmatter::Frontmatter =
        serde_yaml::from_str(parts[0])
            .with_context(|| format!("failed to parse frontmatter in {:?}", path))?;

    let body = parts[1].trim_start_matches('\n').to_string();
    let note = crate::models::note::Note::new(frontmatter, body);

    repo.database().upsert_note(&note)?;

    // Rewrite the file as content-only. Metadata now lives in SQLite.
    std::fs::write(path, &note.content)
        .with_context(|| format!("failed to rewrite {:?} as content-only", path))?;

    Ok(true)
}

#[derive(Debug, Serialize)]
struct MigrateData {
    pub imported: usize,
    pub skipped: usize,
    pub total: usize,
}

impl HumanText for MigrateData {
    fn human_text(&self) -> String {
        format!(
            "导入: {} | 跳过: {} | 总计: {}",
            self.imported, self.skipped, self.total
        )
    }
}
