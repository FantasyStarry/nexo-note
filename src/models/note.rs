use super::frontmatter::Frontmatter;
use serde::Serialize;

/// A note consists of frontmatter metadata and markdown content.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct Note {
    pub frontmatter: Frontmatter,
    pub content: String,
}

impl Note {
    pub fn new(frontmatter: Frontmatter, content: impl Into<String>) -> Self {
        Self {
            frontmatter,
            content: content.into(),
        }
    }
}

impl std::fmt::Display for Note {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let yaml = serde_yaml::to_string(&self.frontmatter).unwrap_or_default();
        write!(f, "---\n{}---\n\n{}", yaml, self.content)
    }
}
