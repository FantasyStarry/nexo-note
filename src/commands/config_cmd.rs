use crate::config::Config;
use crate::output::{print, ApiResponse};
use anyhow::{anyhow, Result};

/// Set a config value.
pub fn set(key: &str, value: &str) -> Result<()> {
    let mut config = Config::load()?;
    match key {
        "notes_dir" => config.notes_dir = std::path::PathBuf::from(value),
        "editor" => config.editor = Some(value.to_string()),
        "default_category" => config.default_category = value.to_string(),
        _ => return Err(anyhow!("unsupported config key '{}'", key)),
    }
    config.save()?;

    let resp = ApiResponse::ok_with_message(
        format!("{} = {}", key, value),
        "配置已保存",
    );
    print(&resp, false)?;
    Ok(())
}

/// Get a config value.
pub fn get(key: &str) -> Result<()> {
    let config = Config::load()?;
    let value = match key {
        "notes_dir" => config.notes_dir.to_string_lossy().to_string(),
        "editor" => config.editor.unwrap_or_default(),
        "default_category" => config.default_category,
        _ => return Err(anyhow!("unsupported config key '{}'", key)),
    };

    print(&ApiResponse::ok(value), false)?;
    Ok(())
}

/// List all config values.
pub fn list() -> Result<()> {
    let config = Config::load()?;
    let lines = format!(
        "notes_dir = {}\neditor = {}\ndefault_category = {}\ncategories = {}",
        config.notes_dir.to_string_lossy(),
        config.editor.unwrap_or_default(),
        config.default_category,
        config.categories.join(", ")
    );

    print(&ApiResponse::ok(lines), false)?;
    Ok(())
}
