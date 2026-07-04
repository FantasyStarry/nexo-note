use crate::models::frontmatter::Frontmatter;
use crate::models::note::Note;
use crate::models::tag;
use anyhow::{anyhow, Context, Result};
use chrono::Local;

use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::{Path, PathBuf};

/// Manages the notes directory and file operations.
pub struct Repo {
    pub root: PathBuf,
    pub categories: Vec<String>,
}

impl Repo {
    /// Open a notes repository at the given path.
    pub fn new(root: PathBuf, categories: Vec<String>) -> Self {
        Self { root, categories }
    }

    /// Ensure the basic directory structure exists.
    pub fn init(&self) -> Result<()> {
        fs::create_dir_all(self.root.join("notes"))?;
        fs::create_dir_all(self.root.join("archive"))?;
        fs::create_dir_all(self.root.join(".nexo"))?;
        fs::create_dir_all(self.root.join(".nexo").join("history"))?;
        Ok(())
    }

    /// Backup the current version of a note before editing.
    pub fn backup_note_history(&self, id: &str) -> Result<Option<PathBuf>> {
        let path = self.note_path(id);
        if !path.exists() {
            return Ok(None);
        }

        let timestamp = Local::now().format("%Y%m%d%H%M%S").to_string();
        let backup_dir = self
            .root
            .join(".nexo")
            .join("history")
            .join(id);
        fs::create_dir_all(&backup_dir)?;

        let backup_path = backup_dir.join(format!("{}.md", timestamp));
        fs::copy(&path, &backup_path)
            .with_context(|| format!("failed to backup note history to {:?}", backup_path))?;
        Ok(Some(backup_path))
    }

    /// List history versions for a note.
    #[allow(dead_code)]
    pub fn note_history(&self, id: &str) -> Result<Vec<PathBuf>> {
        let history_dir = self
            .root
            .join(".nexo")
            .join("history")
            .join(id);
        if !history_dir.exists() {
            return Ok(Vec::new());
        }

        let mut entries: Vec<PathBuf> = Vec::new();
        for entry in fs::read_dir(&history_dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.extension().and_then(|s| s.to_str()) == Some("md") {
                entries.push(path);
            }
        }
        entries.sort();
        Ok(entries)
    }

    /// Validate that a category is allowed.
    pub fn validate_category(&self, category: &str) -> Result<()> {
        if self.categories.iter().any(|c| c == category) {
            Ok(())
        } else {
            Err(anyhow!(
                "invalid category '{}'. supported: {}",
                category,
                self.categories.join(", ")
            ))
        }
    }

    /// Create a new note in the repository.
    pub fn create_note(
        &self,
        title: &str,
        category: &str,
        tags: Vec<String>,
        source_url: Option<&str>,
        content: Option<&str>,
        extra: HashMap<String, String>,
    ) -> Result<Note> {
        self.validate_category(category)?;

        let id = self.next_id(category)?;
        let mut frontmatter = Frontmatter::new(id.clone(), title.to_string(), category.to_string());
        frontmatter.tags = tags;
        if let Some(url) = source_url {
            frontmatter.source_url = url.to_string();
        }

        // Apply extra frontmatter fields from the whitelist.
        for (key, value) in extra {
            match key.as_str() {
                "status" => frontmatter.status = value,
                _ => {
                    // Future extensible fields can be added here.
                    return Err(anyhow!("unsupported extra field '{}'", key));
                }
            }
        }

        let note = Note::new(frontmatter, content.unwrap_or(""));
        self.save_note(&note)?;
        Ok(note)
    }

    /// Generate the next available ID for a category on today's date.
    fn next_id(&self, category: &str) -> Result<String> {
        let today = Local::now().format("%Y%m%d").to_string();
        let pattern = format!("{}-{}-", category, today);
        let mut max_seq = 0;

        let notes_dir = self.root.join("notes").join(category);
        if notes_dir.exists() {
            for entry in fs::read_dir(&notes_dir)? {
                let entry = entry?;
                let name = entry.file_name().to_string_lossy().to_string();
                if let Some(stem) = name.strip_suffix(".md") {
                    if let Some(seq_str) = stem.strip_prefix(&pattern) {
                        if let Ok(seq) = seq_str.parse::<u32>() {
                            if seq > max_seq {
                                max_seq = seq;
                            }
                        }
                    }
                }
            }
        }

        Ok(format!("{}-{}-{:03}", category, today, max_seq + 1))
    }

    /// Compute the storage path for a note ID.
    pub fn note_path(&self, id: &str) -> PathBuf {
        // ID format: {category}-{YYYYMMDD}-{seq}
        let parts: Vec<&str> = id.split('-').collect();
        if parts.len() >= 3 {
            let category = parts[0];
            let year = &parts[1][0..4];
            let month = &parts[1][4..6];
            self.root
                .join("notes")
                .join(category)
                .join(year)
                .join(month)
                .join(format!("{}.md", id))
        } else {
            self.root.join("notes").join(format!("{}.md", id))
        }
    }

    /// Compute the archive path for a note ID.
    fn archive_path(&self, id: &str) -> PathBuf {
        let parts: Vec<&str> = id.split('-').collect();
        if parts.len() >= 3 {
            let category = parts[0];
            let year = &parts[1][0..4];
            let month = &parts[1][4..6];
            self.root
                .join("archive")
                .join(category)
                .join(year)
                .join(month)
                .join(format!("{}.md", id))
        } else {
            self.root.join("archive").join(format!("{}.md", id))
        }
    }

    /// Save a note to its storage path, creating directories as needed.
    pub fn save_note(&self, note: &Note) -> Result<()> {
        let path = self.note_path(&note.frontmatter.id);
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        fs::write(&path, note.to_string())
            .with_context(|| format!("failed to write note to {:?}", path))?;
        Ok(())
    }

    /// Find a note by ID, searching both active and archived notes.
    pub fn find_note(&self, id: &str) -> Result<Option<Note>> {
        let active_path = self.note_path(id);
        if active_path.exists() {
            return parse_note_file(&active_path);
        }
        let archive_path = self.archive_path(id);
        if archive_path.exists() {
            return parse_note_file(&archive_path);
        }
        Ok(None)
    }

    /// List all notes, optionally filtered.
    pub fn list_notes(&self, filters: &NoteFilters) -> Result<Vec<Note>> {
        let mut notes = Vec::new();
        let notes_dir = self.root.join("notes");
        if !notes_dir.exists() {
            return Ok(notes);
        }

        Self::collect_notes(&notes_dir, &mut notes)?;

        // Apply filters
        if let Some(category) = &filters.category {
            notes.retain(|n| &n.frontmatter.category == category);
        }
        if let Some(tag) = &filters.tag {
            notes.retain(|n| n.frontmatter.tags.iter().any(|t| t == tag));
        }
        if let Some(status) = &filters.status {
            notes.retain(|n| &n.frontmatter.status == status);
        }
        if let Some(since) = &filters.since {
            notes.retain(|n| n.frontmatter.created_at >= *since);
        }

        // Sort by created_at descending
        notes.sort_by(|a, b| b.frontmatter.created_at.cmp(&a.frontmatter.created_at));

        if let Some(limit) = filters.limit {
            notes.truncate(limit);
        }

        Ok(notes)
    }

    fn collect_notes(dir: &Path, notes: &mut Vec<Note>) -> Result<()> {
        if !dir.exists() {
            return Ok(());
        }
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                Self::collect_notes(&path, notes)?;
            } else if path.extension().and_then(|s| s.to_str()) == Some("md") {
                if let Some(note) = parse_note_file(&path)? {
                    notes.push(note);
                }
            }
        }
        Ok(())
    }

    /// Search notes by keyword in title, content, or tags.
    pub fn search_notes(&self, keyword: &str, tags: &[String]) -> Result<Vec<Note>> {
        let filters = NoteFilters::default();
        let mut notes = self.list_notes(&filters)?;
        let keyword_lower = keyword.to_lowercase();

    notes.retain(|n| {
            let title_match = n.frontmatter.title.to_lowercase().contains(&keyword_lower);
            let content_match = n.content.to_lowercase().contains(&keyword_lower);
            let tag_match = n.frontmatter.tags.iter().any(|t| t.to_lowercase() == keyword_lower);
            title_match || content_match || tag_match
        });

        if !tags.is_empty() {
            notes.retain(|n| tags.iter().all(|t| n.frontmatter.tags.contains(t)));
        }

        Ok(notes)
    }

    /// Archive a note: move to archive directory and mark status archived.
    pub fn archive_note(&self, id: &str) -> Result<()> {
        let source = self.note_path(id);
        if !source.exists() {
            return Err(anyhow!("note '{}' not found", id));
        }

        let target = self.archive_path(id);
        if let Some(parent) = target.parent() {
            fs::create_dir_all(parent)?;
        }

        let mut note = parse_note_file(&source)?.context("note not found")?;
        note.frontmatter.status = "archived".to_string();
        note.frontmatter.touch();

        fs::write(&target, note.to_string())?;
        fs::remove_file(&source)?;
        Ok(())
    }

    /// Delete a note permanently.
    pub fn delete_note(&self, id: &str) -> Result<()> {
        let active_path = self.note_path(id);
        if active_path.exists() {
            fs::remove_file(&active_path)?;
            return Ok(());
        }
        let archive_path = self.archive_path(id);
        if archive_path.exists() {
            fs::remove_file(&archive_path)?;
            return Ok(());
        }
        Err(anyhow!("note '{}' not found", id))
    }

    /// Collect all tags across all notes.
    pub fn all_tags(&self) -> Result<Vec<String>> {
        let filters = NoteFilters::default();
        let notes = self.list_notes(&filters)?;
        let mut tags: HashSet<String> = HashSet::new();
        for note in notes {
            for tag in note.frontmatter.tags {
                tags.insert(tag);
            }
        }
        let mut result: Vec<String> = tags.into_iter().collect();
        result.sort();
        Ok(result)
    }

    /// Rename a tag across all notes and return the number of updated notes.
    pub fn rename_tag(&self, old: &str, new: &str) -> Result<usize> {
        let old = tag::normalize(old);
        let new = tag::normalize(new);
        if old == new {
            return Ok(0);
        }

        let filters = NoteFilters::default();
        let notes = self.list_notes(&filters)?;
        let mut updated = 0;

        for mut note in notes {
            let mut changed = false;
            for tag in &mut note.frontmatter.tags {
                if tag == &old {
                    *tag = new.clone();
                    changed = true;
                }
            }
            if changed {
                note.frontmatter.touch();
                self.save_note(&note)?;
                updated += 1;
            }
        }

        Ok(updated)
    }

}

#[derive(Default)]
pub struct NoteFilters {
    pub category: Option<String>,
    pub tag: Option<String>,
    pub status: Option<String>,
    pub limit: Option<usize>,
    pub since: Option<chrono::DateTime<chrono::FixedOffset>>,
}

/// Parse a markdown file into a Note, splitting YAML frontmatter from content.
fn parse_note_file(path: &Path) -> Result<Option<Note>> {
    let content = fs::read_to_string(path)?;
    if !content.starts_with("---\n") {
        return Ok(None);
    }

    let parts: Vec<&str> = content[4..].splitn(2, "\n---").collect();
    if parts.len() != 2 {
        return Ok(None);
    }

    let frontmatter: Frontmatter = serde_yaml::from_str(parts[0])
        .with_context(|| format!("failed to parse frontmatter in {:?}", path))?;
    let body = parts[1].trim_start_matches('\n').to_string();

    Ok(Some(Note::new(frontmatter, body)))
}
