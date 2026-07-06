use crate::models::frontmatter::Frontmatter;
use crate::models::note::Note;
use crate::models::tag;
use crate::storage::db::Database;
use anyhow::{Context, Result, anyhow};
use chrono::Local;

use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

/// Maximum characters in a slugified filename (excluding .md extension).
const SLUG_MAX_CHARS: usize = 80;

/// Slugify a title into a human-readable filename.
///
/// Rules:
/// - Lowercase
/// - Replace non-alphanumeric/non-CJK characters with hyphens
/// - Collapse multiple hyphens
/// - Trim leading/trailing hyphens
/// - Truncate to SLUG_MAX_CHARS
/// - If the result is empty, fall back to a short random string
fn slugify_title(title: &str) -> String {
    let slug: String = title
        .chars()
        .map(|c| {
            if c.is_alphanumeric() || c.is_ascii_whitespace() {
                c.to_ascii_lowercase()
            } else if c as u32 >= 0x4E00 && c as u32 <= 0x9FFF {
                // Keep CJK characters as-is
                c
            } else if c as u32 >= 0x3000 && c as u32 <= 0x303F {
                // CJK punctuation → hyphen
                '-'
            } else {
                '-'
            }
        })
        .collect::<String>()
        .split_whitespace()
        .collect::<Vec<&str>>()
        .join("-");

    // Collapse multiple hyphens and trim
    let slug: String = {
        let mut result = String::with_capacity(slug.len());
        let mut prev_hyphen = false;
        for c in slug.chars() {
            if c == '-' {
                if !prev_hyphen {
                    result.push('-');
                }
                prev_hyphen = true;
            } else {
                result.push(c);
                prev_hyphen = false;
            }
        }
        result.trim_matches('-').to_string()
    };

    // Truncate
    let slug = if slug.len() > SLUG_MAX_CHARS {
        slug[..SLUG_MAX_CHARS].trim_end_matches('-').to_string()
    } else {
        slug
    };

    // Fallback if empty
    if slug.is_empty() {
        use std::time::{SystemTime, UNIX_EPOCH};
        let ts = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_millis();
        format!("note-{}", ts % 100_000)
    } else {
        slug
    }
}

/// Ensure the slug is unique within the category by appending a counter if needed.
fn unique_slug_in_category(db: &Database, category: &str, base_slug: &str) -> Result<String> {
    let now = chrono::Local::now();
    let year = now.format("%Y");
    let month = now.format("%m");
    let pattern = format!("notes/{}/{}/{}/{}", category, year, month, base_slug);

    // Check if this slug is already taken
    let exists: bool = db
        .slug_exists(&pattern)
        .unwrap_or(false);

    if !exists {
        return Ok(base_slug.to_string());
    }

    // Try base_slug-2, base_slug-3, etc.
    for counter in 2..1000 {
        let candidate = format!("{}-{}", base_slug, counter);
        let candidate_pattern = format!(
            "notes/{}/{}/{}/{}",
            category, year, month, &candidate
        );
        let taken: bool = db.slug_exists(&candidate_pattern).unwrap_or(false);
        if !taken {
            return Ok(candidate);
        }
    }

    // Extreme fallback: add timestamp
    use std::time::{SystemTime, UNIX_EPOCH};
    let ts = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis();
    Ok(format!("{}-{}", base_slug, ts % 100_000))
}

/// Manages the notes directory and file operations.
///
/// SQLite is the **source of truth** for metadata and content.
/// .md files are synchronized copies for human readability and external editing.
pub struct Repo {
    pub root: PathBuf,
    pub categories: Vec<String>,
    db: Database,
}

impl Repo {
    /// Open a notes repository at the given path, initializing the database.
    pub fn new(root: PathBuf, categories: Vec<String>) -> Result<Self> {
        let db = Database::open(&root)?;
        Ok(Self {
            root,
            categories,
            db,
        })
    }

    /// Get a reference to the underlying database.
    pub fn database(&self) -> &Database {
        &self.db
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
        let backup_dir = self.root.join(".nexo").join("history").join(id);
        fs::create_dir_all(&backup_dir)?;

        let backup_path = backup_dir.join(format!("{}.md", timestamp));
        fs::copy(&path, &backup_path)
            .with_context(|| format!("failed to backup note history to {:?}", backup_path))?;
        Ok(Some(backup_path))
    }

    /// List history versions for a note.
    #[allow(dead_code)]
    pub fn note_history(&self, id: &str) -> Result<Vec<PathBuf>> {
        let history_dir = self.root.join(".nexo").join("history").join(id);
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
    /// The "journal" category is always allowed (used internally).
    pub fn validate_category(&self, category: &str) -> Result<()> {
        if category == "journal" || self.categories.iter().any(|c| c == category) {
            Ok(())
        } else {
            Err(anyhow!(
                "invalid category '{}'. supported: {}",
                category,
                self.categories.join(", ")
            ))
        }
    }

    // ──────────────────────────────────────
    // File path helpers (unchanged)
    // ──────────────────────────────────────

    /// Compute the default storage path for a note ID.
    ///
    /// Uses the deterministic `{category}/{YYYY}/{MM}/{id}.md` layout.
    fn default_note_path(&self, id: &str) -> PathBuf {
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

    /// Compute the storage path for a note ID.
    ///
    /// If the database stores a relative file_path, use it. Otherwise fall back
    /// to the deterministic `{category}/{YYYY}/{MM}/{id}.md` layout.
    pub fn note_path(&self, id: &str) -> PathBuf {
        if let Ok(Some(rel)) = self.db.file_path(id) {
            return self.root.join(rel);
        }
        self.default_note_path(id)
    }

    /// Compute the archive path for a note ID.
    ///
    /// Uses the stored file_path from SQLite (replacing `notes/` with `archive/`),
    /// or falls back to the deterministic `archive/{category}/{YYYY}/{MM}/{id}.md`.
    fn archive_path(&self, id: &str) -> PathBuf {
        // Prefer the stored file_path from DB.
        if let Ok(Some(rel)) = self.db.file_path(id) {
            // Replace the leading "notes/" with "archive/"
            if let Some(archive_rel) = rel.strip_prefix("notes/") {
                return self.root.join("archive").join(archive_rel);
            }
        }
        // Fallback: deterministic path from ID.
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

    // ──────────────────────────────────────
    // CRUD
    // ──────────────────────────────────────

    /// Create a new note in the repository (SQLite + .md file).
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

        let id = self.db.next_id(category)?;
        let mut frontmatter = Frontmatter::new(id.clone(), title.to_string(), category.to_string());
        frontmatter.tags = tags;
        if let Some(url) = source_url {
            frontmatter.source_url = url.to_string();
        }

        for (key, value) in extra {
            match key.as_str() {
                "status" => frontmatter.status = value,
                "parent_id" => frontmatter.parent_id = Some(value),
                _ => {
                    return Err(anyhow!("unsupported extra field '{}'", key));
                }
            }
        }

        let mut note = Note::new(frontmatter, content.unwrap_or(""));

        // Generate a human-readable filename from the title.
        // Format: notes/{category}/{YYYY}/{MM}/{slug}.md
        let base_slug = slugify_title(title);
        let unique_slug = unique_slug_in_category(&self.db, category, &base_slug)?;
        let now = chrono::Local::now();
        let rel_path = format!(
            "notes/{}/{}/{}/{}.md",
            category,
            now.format("%Y"),
            now.format("%m"),
            unique_slug
        );
        note.frontmatter.file_path = rel_path;

        // Write to SQLite first (source of truth).
        self.db.upsert_note(&note)?;

        // Then write the .md file (human-readable copy).
        self.save_note_file(&note)?;

        Ok(note)
    }

    /// Create a new note with a parent (linked to another note).
    pub fn create_linked_note(
        &self,
        title: &str,
        category: &str,
        tags: Vec<String>,
        source_url: Option<&str>,
        content: Option<&str>,
        parent_id: &str,
        extra: HashMap<String, String>,
    ) -> Result<Note> {
        // Verify parent exists.
        if !self.db.note_exists(parent_id)? {
            return Err(anyhow!("parent note '{}' not found", parent_id));
        }
        let mut extra = extra;
        extra.insert("parent_id".to_string(), parent_id.to_string());
        self.create_note(title, category, tags, source_url, content, extra)
    }

    /// Find a note by ID. Checks SQLite first, falls back to .md file.
    pub fn find_note(&self, id: &str) -> Result<Option<Note>> {
        // Try SQLite first.
        if let Some(mut note) = self.db.find_note(id)? {
            self.db.populate_tags(&mut note)?;
            return Ok(Some(note));
        }

        // Fallback: try to parse from .md file.
        let active_path = self.note_path(id);
        if active_path.exists() {
            return parse_note_file(&active_path);
        }
        let archive_p = self.archive_path(id);
        if archive_p.exists() {
            return parse_note_file(&archive_p);
        }

        Ok(None)
    }

    /// Save a note to its .md file, creating directories as needed.
    pub fn save_note(&self, note: &Note) -> Result<()> {
        // Update SQLite.
        self.db.upsert_note(note)?;
        // Write .md file.
        self.save_note_file(note)
    }

    fn save_note_file(&self, note: &Note) -> Result<()> {
        let path = self.note_path(&note.frontmatter.id);
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        fs::write(&path, note.to_string())
            .with_context(|| format!("failed to write note to {:?}", path))?;
        Ok(())
    }

    // ──────────────────────────────────────
    // Querying
    // ──────────────────────────────────────

    /// List notes with optional filters, using SQLite.
    pub fn list_notes(&self, filters: &NoteFilters) -> Result<Vec<Note>> {
        let mut notes = self.db.list_notes(filters)?;
        self.db.populate_tags_batch(&mut notes)?;
        Ok(notes)
    }

    /// Search notes by keyword in title or content, using SQLite.
    pub fn search_notes(&self, keyword: &str, tags: &[String]) -> Result<Vec<Note>> {
        let mut notes = self.db.search_notes(keyword, tags)?;
        self.db.populate_tags_batch(&mut notes)?;
        Ok(notes)
    }

    // ──────────────────────────────────────
    // Archive / Delete
    // ──────────────────────────────────────

    /// Archive a note: move .md file and update status in SQLite.
    pub fn archive_note(&self, id: &str) -> Result<()> {
        let source = self.note_path(id);
        if !source.exists() {
            return Err(anyhow!("note '{}' not found", id));
        }

        // Load from SQLite (source of truth), update status, and write the
        // content-only file to the archive directory.
        let mut note = self.find_note(id)?.context("note not found")?;
        note.frontmatter.status = "archived".to_string();
        note.frontmatter.touch();

        let target = self.archive_path(id);
        if let Some(parent) = target.parent() {
            fs::create_dir_all(parent)?;
        }

        fs::write(&target, note.to_string())?;
        fs::remove_file(&source)?;

        // Update SQLite.
        self.db.upsert_note(&note)?;

        Ok(())
    }

    /// Delete a note from both SQLite and file system.
    pub fn delete_note(&self, id: &str) -> Result<()> {
        // Remove from SQLite.
        self.db.delete_note(id)?;

        // Remove .md file.
        let active_path = self.note_path(id);
        if active_path.exists() {
            fs::remove_file(&active_path)?;
            return Ok(());
        }
        let archive_p = self.archive_path(id);
        if archive_p.exists() {
            fs::remove_file(&archive_p)?;
            return Ok(());
        }
        Err(anyhow!("note '{}' not found", id))
    }

    // ──────────────────────────────────────
    // Thread / Journal
    // ──────────────────────────────────────

    /// Get the full thread chain for a note.
    pub fn get_thread(&self, id: &str) -> Result<Vec<Note>> {
        let mut notes = self.db.get_thread(id)?;
        self.db.populate_tags_batch(&mut notes)?;
        Ok(notes)
    }

    /// Get or create today's journal note.
    ///
    /// If today's journal already exists, returns its ID.
    /// Otherwise creates a new journal note and returns its ID.
    pub fn ensure_today_journal(&self) -> Result<String> {
        if let Some(id) = self.db.today_journal_id()? {
            return Ok(id);
        }
        let today = chrono::Local::now().format("%Y-%m-%d").to_string();
        let title = format!("{} 日志", today);
        let note = self.create_note(&title, "journal", Vec::new(), None, None, HashMap::new())?;
        Ok(note.frontmatter.id)
    }

    // ──────────────────────────────────────
    // Tags
    // ──────────────────────────────────────

    /// Collect all tags across all notes (from SQLite).
    pub fn all_tags(&self) -> Result<Vec<String>> {
        self.db.all_tags()
    }

    /// Rename a tag in SQLite and update all .md files.
    pub fn rename_tag(&self, old: &str, new: &str) -> Result<usize> {
        let old_norm = tag::normalize(old);
        let new_norm = tag::normalize(new);
        if old_norm == new_norm {
            return Ok(0);
        }

        // Update SQLite first.
        let updated = self.db.rename_tag(&old_norm, &new_norm)?;
        if updated == 0 {
            return Ok(0);
        }

        // Now resync all affected .md files.
        let filters = NoteFilters::default();
        let notes = self.list_notes(&filters)?;
        for mut note in notes {
            let mut changed = false;
            for t in &mut note.frontmatter.tags {
                if t == &old_norm {
                    *t = new_norm.clone();
                    changed = true;
                }
            }
            if changed {
                self.save_note_file(&note)?;
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
pub fn parse_note_file(path: &Path) -> Result<Option<Note>> {
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
