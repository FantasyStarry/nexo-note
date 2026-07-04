use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};

/// Frontmatter metadata stored at the top of each note file.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Frontmatter {
    pub id: String,
    pub title: String,
    pub category: String,
    #[serde(default)]
    pub tags: Vec<String>,
    #[serde(default = "default_status")]
    pub status: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub source_url: String,
    pub created_at: DateTime<FixedOffset>,
    pub updated_at: DateTime<FixedOffset>,
}

fn default_status() -> String {
    "active".to_string()
}

impl Frontmatter {
    /// Create a new frontmatter with the current time as created/updated.
    pub fn new(id: String, title: String, category: String) -> Self {
        let now = chrono::Local::now().fixed_offset();
        Self {
            id,
            title,
            category,
            tags: Vec::new(),
            status: default_status(),
            source_url: String::new(),
            created_at: now,
            updated_at: now,
        }
    }

    /// Update the `updated_at` timestamp to now.
    pub fn touch(&mut self) {
        self.updated_at = chrono::Local::now().fixed_offset();
    }
}
