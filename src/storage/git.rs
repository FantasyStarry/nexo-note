use anyhow::{Context, Result};
use std::path::Path;
use std::process::Command;

/// Check whether git is installed and available on PATH.
pub fn is_git_available() -> bool {
    Command::new("git")
        .arg("--version")
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false)
}

/// Check whether the given directory is inside a git repository.
pub fn is_git_repo(path: &Path) -> bool {
    path.join(".git").is_dir()
        || run_git(path, &["rev-parse", "--git-dir"]).map(|o| o.status.success()).unwrap_or(false)
}

/// Initialize a git repository at the given path.
pub fn init_repo(path: &Path) -> Result<()> {
    run_git_quiet(path, &["init"]).context("failed to initialize git repository")?;
    // Create an initial .gitignore to ignore local metadata if needed.
    let gitignore = path.join(".gitignore");
    if !gitignore.exists() {
        std::fs::write(&gitignore, ".nexo/history/\n")?;
    }
    Ok(())
}

/// Commit all changes in the repository with a message.
pub fn commit_all(path: &Path, message: &str) -> Result<()> {
    if !is_git_repo(path) {
        return Ok(());
    }

    // Stage all changes.
    let add_output = run_git(path, &["add", "."])?;
    if !add_output.status.success() {
        let stderr = String::from_utf8_lossy(&add_output.stderr);
        return Err(anyhow::anyhow!("git add failed: {}", stderr));
    }

    // Check if there is anything to commit.
    let diff_output = run_git(path, &["diff", "--cached", "--quiet"])?;
    if diff_output.status.success() {
        // Nothing staged.
        return Ok(());
    }

    let commit_output = run_git(
        path,
        &["commit", "-m", message, "--no-verify"],
    )?;
    if !commit_output.status.success() {
        let stderr = String::from_utf8_lossy(&commit_output.stderr);
        return Err(anyhow::anyhow!("git commit failed: {}", stderr));
    }

    Ok(())
}

fn run_git(path: &Path, args: &[&str]) -> Result<std::process::Output> {
    Command::new("git")
        .current_dir(path)
        .args(args)
        .output()
        .with_context(|| format!("failed to run git {:?} in {:?}", args, path))
}

fn run_git_quiet(path: &Path, args: &[&str]) -> Result<()> {
    let output = run_git(path, args)?;
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(anyhow::anyhow!("git command failed: {}", stderr));
    }
    Ok(())
}
