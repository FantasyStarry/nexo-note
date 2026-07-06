use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

/// Global CLI configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    #[serde(default = "default_notes_dir")]
    pub notes_dir: PathBuf,

    #[serde(default)]
    pub editor: Option<String>,

    #[serde(default = "default_category")]
    pub default_category: String,

    #[serde(default = "default_categories")]
    pub categories: Vec<String>,

    /// Extra key-value pairs reserved for future use.
    #[serde(flatten)]
    pub extra: HashMap<String, toml::Value>,
}

fn default_notes_dir() -> PathBuf {
    dirs::home_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join("nexo-notes")
}

fn default_category() -> String {
    "issues".to_string()
}

fn default_categories() -> Vec<String> {
    vec![
        "issues".to_string(),
        "articles".to_string(),
        "ideas".to_string(),
        "projects".to_string(),
        "journal".to_string(),
    ]
}

impl Default for Config {
    fn default() -> Self {
        Self {
            notes_dir: default_notes_dir(),
            editor: None,
            default_category: default_category(),
            categories: default_categories(),
            extra: HashMap::new(),
        }
    }
}

impl Config {
    /// Path to the global config file: `~/.nexo/config.toml`
    pub fn global_path() -> PathBuf {
        dirs::home_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join(".nexo")
            .join("config.toml")
    }

    /// Load config from the global config file, creating defaults if missing.
    pub fn load() -> Result<Self> {
        let path = Self::global_path();
        if !path.exists() {
            let config = Config::default();
            config.save()?;
            return Ok(config);
        }

        let content = fs::read_to_string(&path)
            .with_context(|| format!("failed to read config from {:?}", path))?;
        let config: Config = toml::from_str(&content)
            .with_context(|| format!("failed to parse config from {:?}", path))?;
        Ok(config)
    }

    /// Save the current config to the global config file.
    pub fn save(&self) -> Result<()> {
        let path = Self::global_path();
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        let content = toml::to_string_pretty(self)?;
        fs::write(&path, content)
            .with_context(|| format!("failed to write config to {:?}", path))?;
        Ok(())
    }

    /// Resolve the effective notes directory.
    /// Priority: cli_arg > env var > config > default
    pub fn resolve_notes_dir(&self, cli_arg: Option<&str>) -> PathBuf {
        if let Some(dir) = cli_arg {
            return PathBuf::from(dir);
        }
        if let Ok(dir) = std::env::var("NEXO_NOTES_DIR") {
            return PathBuf::from(dir);
        }
        self.notes_dir.clone()
    }

    /// Resolve the effective editor.
    /// Priority: cli_arg > config > $EDITOR > "vi"
    pub fn resolve_editor(&self, cli_arg: Option<&str>) -> String {
        if let Some(editor) = cli_arg {
            return editor.to_string();
        }
        if let Some(editor) = &self.editor {
            return editor.clone();
        }
        std::env::var("EDITOR").unwrap_or_else(|_| "vi".to_string())
    }
}
