use crate::cli::Cli;
use crate::commands::open_repo;
use crate::output::{print, ApiResponse, HumanText};
use crate::storage::git;
use anyhow::Result;
use serde::Serialize;
use std::path::PathBuf;

#[derive(Debug, Serialize)]
pub struct InitData {
    pub path: PathBuf,
    pub git_initialized: bool,
    pub git_available: bool,
}

impl HumanText for InitData {
    fn human_text(&self) -> String {
        let mut lines = vec![format!("笔记库已初始化: {}", self.path.display())];
        if self.git_available {
            if self.git_initialized {
                lines.push("Git 版本管理已启用".to_string());
            } else {
                lines.push("Git 可用但未启用，可用 --git 参数启用".to_string());
            }
        } else {
            lines.push("未检测到 Git，跳过版本管理".to_string());
        }
        lines.join("\n")
    }
}

pub fn run(cli: &Cli, enable_git: bool) -> Result<()> {
    let repo = open_repo(cli)?;
    let git_available = git::is_git_available();
    let mut git_initialized = false;

    if enable_git && git_available {
        if !git::is_git_repo(&repo.root) {
            git::init_repo(&repo.root)?;
            git_initialized = true;
        } else {
            git_initialized = true;
        }
        git::commit_all(&repo.root, "chore: initialize nexo-note repository")?;
    }

    let data = InitData {
        path: repo.root.clone(),
        git_initialized,
        git_available,
    };

    let message = if git_initialized {
        "笔记库初始化完成，Git 版本管理已启用"
    } else {
        "笔记库初始化完成"
    };

    print(&ApiResponse::ok_with_message(data, message), cli.json)
}
