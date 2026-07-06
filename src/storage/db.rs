use crate::models::frontmatter::Frontmatter;
use crate::models::note::Note;
use crate::models::tag;
use crate::output::{CategoryCount, StatsData, StatusCount, TagCount};
use crate::storage::repo::NoteFilters;
use anyhow::{Context, Result};
use chrono::{DateTime, FixedOffset, Local};
use rusqlite::{Connection, OptionalExtension, params};

/// SQLite-backed database for note metadata and content.
///
/// SQLite is the **source of truth**. .md files on disk are synchronized
/// copies for human readability and external editing.
pub struct Database {
    conn: Connection,
}

impl Database {
    /// Open (or create) the database at `{root}/.nexo/notes.db`.
    pub fn open(root: &std::path::Path) -> Result<Self> {
        let db_dir = root.join(".nexo");
        std::fs::create_dir_all(&db_dir)?;
        let db_path = db_dir.join("notes.db");
        let conn = Connection::open(&db_path)
            .with_context(|| format!("failed to open database at {:?}", db_path))?;

        // Enable WAL mode for better concurrent reads.
        conn.execute_batch("PRAGMA journal_mode=WAL;")?;

        let db = Self { conn };
        db.ensure_schema()?;
        Ok(db)
    }

    /// Create tables and indexes if they don't exist.
    fn ensure_schema(&self) -> Result<()> {
        self.conn.execute_batch(
            "
            CREATE TABLE IF NOT EXISTS notes (
                id         TEXT PRIMARY KEY,
                title      TEXT NOT NULL,
                category   TEXT NOT NULL DEFAULT '',
                status     TEXT NOT NULL DEFAULT 'active',
                content    TEXT NOT NULL DEFAULT '',
                source_url TEXT NOT NULL DEFAULT '',
                file_path  TEXT NOT NULL DEFAULT '',
                parent_id  TEXT,
                created_at TEXT NOT NULL,
                updated_at TEXT NOT NULL
            );

            CREATE TABLE IF NOT EXISTS tags (
                id   INTEGER PRIMARY KEY AUTOINCREMENT,
                name TEXT NOT NULL UNIQUE
            );

            CREATE TABLE IF NOT EXISTS note_tags (
                note_id TEXT NOT NULL REFERENCES notes(id),
                tag_id  INTEGER NOT NULL REFERENCES tags(id),
                PRIMARY KEY (note_id, tag_id)
            );

            CREATE INDEX IF NOT EXISTS idx_notes_category ON notes(category);
            CREATE INDEX IF NOT EXISTS idx_notes_status  ON notes(status);
            CREATE INDEX IF NOT EXISTS idx_notes_created  ON notes(created_at);
            CREATE INDEX IF NOT EXISTS idx_notes_updated  ON notes(updated_at);
            CREATE INDEX IF NOT EXISTS idx_nt_note_id     ON note_tags(note_id);
            CREATE INDEX IF NOT EXISTS idx_nt_tag_id      ON note_tags(tag_id);
            ",
        )?;

        // Migrate existing databases that were created before the file_path column.
        let has_file_path: bool = self
            .conn
            .query_row(
                "SELECT 1 FROM pragma_table_info('notes') WHERE name = 'file_path'",
                [],
                |_| Ok(true),
            )
            .unwrap_or(false);
        if !has_file_path {
            self.conn.execute(
                "ALTER TABLE notes ADD COLUMN file_path TEXT NOT NULL DEFAULT ''",
                [],
            )?;
        }

        // Migrate databases created before the parent_id column.
        let has_parent_id: bool = self
            .conn
            .query_row(
                "SELECT 1 FROM pragma_table_info('notes') WHERE name = 'parent_id'",
                [],
                |_| Ok(true),
            )
            .unwrap_or(false);
        if !has_parent_id {
            self.conn.execute(
                "ALTER TABLE notes ADD COLUMN parent_id TEXT",
                [],
            )?;
        }

        Ok(())
    }

    // ──────────────────────────────────────
    // CRUD
    // ──────────────────────────────────────

    /// Insert or replace a note (metadata + content) in the database.
    /// Also updates the tag associations.
    pub fn upsert_note(&self, note: &Note) -> Result<()> {
        let fm = &note.frontmatter;
        let fm_tags = &fm.tags;

        // Upsert the note row.
        self.conn.execute(
            "INSERT INTO notes (id, title, category, status, content, source_url, file_path, parent_id, created_at, updated_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)
             ON CONFLICT(id) DO UPDATE SET
                title      = excluded.title,
                category   = excluded.category,
                status     = excluded.status,
                content    = excluded.content,
                source_url = excluded.source_url,
                file_path  = excluded.file_path,
                parent_id  = excluded.parent_id,
                updated_at = excluded.updated_at",
            params![
                fm.id,
                fm.title,
                fm.category,
                fm.status,
                note.content,
                fm.source_url,
                fm.file_path,
                fm.parent_id,
                fm.created_at.to_rfc3339(),
                fm.updated_at.to_rfc3339(),
            ],
        )?;

        // Sync tags: delete old, insert new.
        self.conn
            .execute("DELETE FROM note_tags WHERE note_id = ?1", params![fm.id])?;

        for t in fm_tags {
            let normalized = tag::normalize(t);
            // Ensure the tag exists, get its id.
            self.conn.execute(
                "INSERT INTO tags (name) VALUES (?1) ON CONFLICT(name) DO NOTHING",
                params![normalized],
            )?;
            let tag_id: i64 = self.conn.query_row(
                "SELECT id FROM tags WHERE name = ?1",
                params![normalized],
                |row| row.get(0),
            )?;
            self.conn.execute(
                "INSERT OR IGNORE INTO note_tags (note_id, tag_id) VALUES (?1, ?2)",
                params![fm.id, tag_id],
            )?;
        }

        Ok(())
    }

    /// Delete a note and its tag associations.
    pub fn delete_note(&self, id: &str) -> Result<()> {
        self.conn
            .execute("DELETE FROM note_tags WHERE note_id = ?1", params![id])?;
        self.conn
            .execute("DELETE FROM notes WHERE id = ?1", params![id])?;
        Ok(())
    }

    /// Update the status of a note (active / archived / draft).
    #[allow(dead_code)]
    pub fn update_status(&self, id: &str, status: &str) -> Result<()> {
        let now = Local::now().fixed_offset().to_rfc3339();
        self.conn.execute(
            "UPDATE notes SET status = ?1, updated_at = ?2 WHERE id = ?3",
            params![status, now, id],
        )?;
        Ok(())
    }

    /// Update title and content after an external edit.
    #[allow(dead_code)]
    pub fn update_content(&self, id: &str, title: &str, content: &str) -> Result<()> {
        let now = Local::now().fixed_offset().to_rfc3339();
        self.conn.execute(
            "UPDATE notes SET title = ?1, content = ?2, updated_at = ?3 WHERE id = ?4",
            params![title, content, now, id],
        )?;
        Ok(())
    }

    // ──────────────────────────────────────
    // ID Generation
    // ──────────────────────────────────────

    /// Generate the next available note ID for the given category on today's date.
    pub fn next_id(&self, category: &str) -> Result<String> {
        let today = Local::now().format("%Y%m%d").to_string();
        let prefix = format!("{}-{}-", category, today);

        let max_seq: Option<i64> = self
            .conn
            .query_row(
                "SELECT MAX(CAST(SUBSTR(id, ?1) AS INTEGER)) FROM notes WHERE id LIKE ?2",
                params![prefix.len() as i64 + 1, format!("{}%", prefix)],
                |row| row.get(0),
            )
            .ok()
            .flatten();

        let next = max_seq.unwrap_or(0) + 1;
        Ok(format!("{}{:03}", prefix, next))
    }

    // ──────────────────────────────────────
    // Query
    // ──────────────────────────────────────

    /// Find a single note by ID.
    pub fn find_note(&self, id: &str) -> Result<Option<Note>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, title, category, status, content, source_url, file_path, parent_id, created_at, updated_at
             FROM notes WHERE id = ?1",
        )?;

        let mut rows = stmt.query_map(params![id], Self::row_to_note)?;
        match rows.next() {
            Some(Ok(note)) => Ok(Some(note)),
            _ => Ok(None),
        }
    }

    /// List notes matching the given filters.
    /// By default, excludes archived notes (status = 'active') unless a status filter is provided.
    pub fn list_notes(&self, filters: &NoteFilters) -> Result<Vec<Note>> {
        let mut conditions: Vec<String> = Vec::new();
        let mut param_values: Vec<Box<dyn rusqlite::types::ToSql>> = Vec::new();

        // Default: only show active notes, unless a status filter is explicitly provided.
        if filters.status.is_none() {
            conditions.push("status = 'active'".to_string());
        }

        if let Some(category) = &filters.category {
            param_values.push(Box::new(category.clone()));
            conditions.push(format!("category = ?{}", param_values.len()));
        }
        if let Some(tag_name) = &filters.tag {
            param_values.push(Box::new(tag_name.clone()));
            conditions.push(format!(
                "id IN (SELECT nt.note_id FROM note_tags nt JOIN tags t ON nt.tag_id = t.id WHERE t.name = ?{})",
                param_values.len()
            ));
        }
        if let Some(status) = &filters.status {
            param_values.push(Box::new(status.clone()));
            conditions.push(format!("status = ?{}", param_values.len()));
        }
        if let Some(since) = &filters.since {
            let since_str = since.format("%Y-%m-%dT00:00:00").to_string();
            param_values.push(Box::new(since_str));
            conditions.push(format!("created_at >= ?{}", param_values.len()));
        }

        let where_clause = conditions.join(" AND ");
        let order = "ORDER BY created_at DESC";

        let mut sql = format!(
            "SELECT id, title, category, status, content, source_url, file_path, parent_id, created_at, updated_at
             FROM notes WHERE {} {}",
            where_clause, order
        );

        if let Some(limit) = filters.limit {
            param_values.push(Box::new(limit as i64));
            sql.push_str(&format!(" LIMIT ?{}", param_values.len()));
        }

        let mut stmt = self.conn.prepare(&sql)?;
        let params_refs: Vec<&dyn rusqlite::types::ToSql> =
            param_values.iter().map(|p| p.as_ref()).collect();
        let notes = stmt
            .query_map(params_refs.as_slice(), Self::row_to_note)?
            .collect::<std::result::Result<Vec<_>, _>>()?;

        Ok(notes)
    }

    /// Search notes by keyword in title or content.
    /// Searches all notes (including archived) unless overridden via NoteFilters.
    pub fn search_notes(&self, keyword: &str, tag_names: &[String]) -> Result<Vec<Note>> {
        let kw = format!("%{}%", keyword);
        let mut conditions = vec![
            "1=1".to_string(),
            "(title LIKE ?1 OR content LIKE ?1)".to_string(),
        ];
        let mut param_values: Vec<Box<dyn rusqlite::types::ToSql>> = vec![Box::new(kw)];

        if !tag_names.is_empty() {
            let tag_conditions: Vec<String> = tag_names
                .iter()
                .enumerate()
                .map(|(_i, t)| {
                    let idx = param_values.len() + 1;
                    param_values.push(Box::new(t.clone()));
                    format!(
                        "id IN (SELECT nt.note_id FROM note_tags nt JOIN tags tgs ON nt.tag_id = tgs.id WHERE tgs.name = ?{})",
                        idx
                    )
                })
                .collect();
            conditions.push(tag_conditions.join(" AND "));
        }

        let where_clause = conditions.join(" AND ");
        let sql = format!(
            "SELECT id, title, category, status, content, source_url, file_path, parent_id, created_at, updated_at
             FROM notes WHERE {} ORDER BY created_at DESC",
            where_clause
        );

        let mut stmt = self.conn.prepare(&sql)?;
        let params_refs: Vec<&dyn rusqlite::types::ToSql> =
            param_values.iter().map(|p| p.as_ref()).collect();
        let notes = stmt
            .query_map(params_refs.as_slice(), Self::row_to_note)?
            .collect::<std::result::Result<Vec<_>, _>>()?;

        Ok(notes)
    }

    /// Get the stored relative file path for a note, if any.
    pub fn file_path(&self, id: &str) -> Result<Option<String>> {
        let path: Option<String> = self
            .conn
            .query_row(
                "SELECT file_path FROM notes WHERE id = ?1",
                params![id],
                |row| row.get(0),
            )
            .optional()?;
        Ok(path.filter(|p| !p.is_empty()))
    }

    /// Check whether a file path pattern already exists in the database.
    ///
    /// Used to ensure unique slug-based filenames. The `path_pattern` is matched
    /// against the `file_path` column using SQL LIKE (with `%` appended).
    pub fn slug_exists(&self, path_pattern: &str) -> Result<bool> {
        let like = format!("{}%", path_pattern);
        let count: i64 = self.conn.query_row(
            "SELECT COUNT(*) FROM notes WHERE file_path LIKE ?1",
            params![like],
            |row| row.get(0),
        )?;
        Ok(count > 0)
    }

    /// Check whether a note exists by ID.
    #[allow(dead_code)]
    pub fn note_exists(&self, id: &str) -> Result<bool> {
        let count: i64 = self.conn.query_row(
            "SELECT COUNT(*) FROM notes WHERE id = ?1",
            params![id],
            |row| row.get(0),
        )?;
        Ok(count > 0)
    }

    // ──────────────────────────────────────
    // Thread / Chain
    // ──────────────────────────────────────

    /// Get the parent note ID, if any.
    pub fn parent_id(&self, id: &str) -> Result<Option<String>> {
        let result: Option<Option<String>> = self
            .conn
            .query_row(
                "SELECT parent_id FROM notes WHERE id = ?1 AND parent_id IS NOT NULL AND parent_id != ''",
                params![id],
                |row| row.get::<_, Option<String>>(0),
            )
            .optional()?;
        Ok(result.flatten())
    }

    /// Get all direct children of a note, ordered by created_at.
    pub fn get_children(&self, id: &str) -> Result<Vec<Note>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, title, category, status, content, source_url, file_path, parent_id, created_at, updated_at
             FROM notes WHERE parent_id = ?1 ORDER BY created_at ASC",
        )?;
        let notes = stmt
            .query_map(params![id], Self::row_to_note)?
            .collect::<std::result::Result<Vec<_>, _>>()?;
        Ok(notes)
    }

    /// Build the full thread chain for a note: [root, ...ancestors, note, ...descendants].
    ///
    /// Walks up the parent chain to find the root, then recursively collects all
    /// descendants in breadth-first order.
    pub fn get_thread(&self, id: &str) -> Result<Vec<Note>> {
        // Build the ancestor chain: walk from id up to root.
        let mut ancestors: Vec<Note> = Vec::new();
        let mut current_parent: Option<String> = self.parent_id(id)?;
        while let Some(pid) = current_parent {
            if let Some(note) = self.find_note(&pid)? {
                current_parent = note.frontmatter.parent_id.clone();
                ancestors.push(note);
            } else {
                break;
            }
        }
        ancestors.reverse();

        // Collect descendants (breadth-first).
        let mut descendants: Vec<Note> = Vec::new();
        let mut queue: Vec<String> = vec![id.to_string()];
        while let Some(cid) = queue.pop() {
            let children = self.get_children(&cid)?;
            for child in children {
                if child.frontmatter.id != id {
                    descendants.push(child.clone());
                    queue.push(child.frontmatter.id.clone());
                }
            }
        }

        // Re-fetch the central note to include in the result.
        let central = self.find_note(id)?.ok_or_else(|| {
            anyhow::anyhow!("note '{}' not found", id)
        })?;

        let mut result = ancestors;
        result.push(central);
        result.extend(descendants);
        Ok(result)
    }

    /// Get today's journal note ID, if one exists.
    pub fn today_journal_id(&self) -> Result<Option<String>> {
        let today = Local::now().format("%Y-%m-%d").to_string();
        let prefix = format!("journal-{}", today.replace('-', ""));
        let id: Option<String> = self
            .conn
            .query_row(
                "SELECT id FROM notes WHERE id LIKE ?1 AND category = 'journal' LIMIT 1",
                params![format!("{}%", prefix)],
                |row| row.get(0),
            )
            .optional()?;
        Ok(id)
    }

    /// Total number of notes in the database.
    pub fn note_count(&self) -> Result<usize> {
        let count: i64 = self
            .conn
            .query_row("SELECT COUNT(*) FROM notes", [], |row| row.get(0))?;
        Ok(count as usize)
    }

    // ──────────────────────────────────────
    // Tags
    // ──────────────────────────────────────

    /// Collect all unique tag names.
    pub fn all_tags(&self) -> Result<Vec<String>> {
        let mut stmt = self.conn.prepare("SELECT name FROM tags ORDER BY name")?;
        let tags: Vec<String> = stmt
            .query_map([], |row| row.get(0))?
            .collect::<std::result::Result<Vec<_>, _>>()?;
        Ok(tags)
    }

    /// Rename a tag across all notes. Returns the number of notes affected.
    pub fn rename_tag(&self, old: &str, new: &str) -> Result<usize> {
        let old = tag::normalize(old);
        let new = tag::normalize(new);
        if old == new {
            return Ok(0);
        }

        // Find or create the new tag.
        self.conn.execute(
            "INSERT INTO tags (name) VALUES (?1) ON CONFLICT(name) DO NOTHING",
            params![new],
        )?;

        let new_id: i64 =
            self.conn
                .query_row("SELECT id FROM tags WHERE name = ?1", params![new], |row| {
                    row.get(0)
                })?;

        // Count affected notes before updating.
        let count: i64 = self.conn.query_row(
            "SELECT COUNT(*) FROM note_tags WHERE tag_id = (SELECT id FROM tags WHERE name = ?1)",
            params![old],
            |row| row.get(0),
        )?;

        // Update note_tags: replace old tag with new one, deduplicate.
        self.conn.execute(
            "INSERT OR IGNORE INTO note_tags (note_id, tag_id)
             SELECT nt.note_id, ?1
             FROM note_tags nt
             JOIN tags t ON nt.tag_id = t.id
             WHERE t.name = ?2",
            params![new_id, old],
        )?;
        self.conn.execute(
            "DELETE FROM note_tags WHERE tag_id = (SELECT id FROM tags WHERE name = ?1)",
            params![old],
        )?;

        // Clean up orphan tags.
        self.conn.execute(
            "DELETE FROM tags WHERE id NOT IN (SELECT DISTINCT tag_id FROM note_tags)",
            [],
        )?;

        Ok(count as usize)
    }

    // ──────────────────────────────────────
    // Stats
    // ──────────────────────────────────────

    /// Compute notebook statistics via a single query.
    pub fn stats(&self) -> Result<StatsData> {
        let total_notes: usize = self.note_count()?;

        // By category.
        let mut stmt = self.conn.prepare(
            "SELECT category,
                    COUNT(*) AS total,
                    SUM(CASE WHEN status = 'active' THEN 1 ELSE 0 END) AS active,
                    SUM(CASE WHEN status = 'archived' THEN 1 ELSE 0 END) AS archived
             FROM notes
             GROUP BY category
             ORDER BY category",
        )?;
        let by_category: Vec<CategoryCount> = stmt
            .query_map([], |row| {
                Ok(CategoryCount {
                    category: row.get(0)?,
                    total: row.get::<_, i64>(1)? as usize,
                    active: row.get::<_, i64>(2)? as usize,
                    archived: row.get::<_, i64>(3)? as usize,
                })
            })?
            .collect::<std::result::Result<Vec<_>, _>>()?;

        // By status.
        let mut stmt = self
            .conn
            .prepare("SELECT status, COUNT(*) FROM notes GROUP BY status")?;
        let mut status_count = StatusCount::default();
        let rows = stmt.query_map([], |row| {
            Ok((row.get::<_, String>(0)?, row.get::<_, i64>(1)?))
        })?;
        for row in rows {
            let (status, count) = row?;
            match status.as_str() {
                "active" => status_count.active = count as usize,
                "archived" => status_count.archived = count as usize,
                "draft" => status_count.draft = count as usize,
                _ => status_count.other = count as usize,
            }
        }

        // This month.
        let current_month = Local::now().format("%Y-%m").to_string();
        let this_month: i64 = self.conn.query_row(
            "SELECT COUNT(*) FROM notes WHERE substr(created_at, 1, 7) = ?1",
            params![current_month],
            |row| row.get(0),
        )?;

        // Top tags.
        let mut stmt = self.conn.prepare(
            "SELECT t.name, COUNT(nt.note_id) AS cnt
             FROM tags t
             JOIN note_tags nt ON nt.tag_id = t.id
             GROUP BY t.id
             ORDER BY cnt DESC, t.name
             LIMIT 10",
        )?;
        let top_tags: Vec<TagCount> = stmt
            .query_map([], |row| {
                Ok(TagCount {
                    tag: row.get(0)?,
                    count: row.get::<_, i64>(1)? as usize,
                })
            })?
            .collect::<std::result::Result<Vec<_>, _>>()?;
        let total_tags: usize = self
            .conn
            .query_row("SELECT COUNT(*) FROM tags", [], |row| row.get(0))?;

        Ok(StatsData {
            total_notes,
            by_category,
            by_status: status_count,
            this_month: this_month as usize,
            total_tags,
            top_tags,
        })
    }

    // ──────────────────────────────────────
    // Internal helpers
    // ──────────────────────────────────────

    fn row_to_note(row: &rusqlite::Row<'_>) -> rusqlite::Result<Note> {
        let id: String = row.get(0)?;
        let title: String = row.get(1)?;
        let category: String = row.get(2)?;
        let status: String = row.get(3)?;
        let content: String = row.get(4)?;
        let source_url: String = row.get(5)?;
        let file_path: String = row.get(6)?;
        let parent_id: Option<String> = row.get(7)?;
        let created_at_str: String = row.get(8)?;
        let updated_at_str: String = row.get(9)?;

        let parse_dt = |s: &str| -> DateTime<FixedOffset> {
            DateTime::parse_from_rfc3339(s)
                .or_else(|_| {
                    chrono::NaiveDateTime::parse_from_str(s, "%Y-%m-%dT%H:%M:%S%.f%:z")
                        .map(|dt| dt.and_utc().fixed_offset())
                })
                .or_else(|_| {
                    chrono::NaiveDateTime::parse_from_str(s, "%Y-%m-%dT%H:%M:%S").map(|dt| {
                        dt.and_local_timezone(FixedOffset::east_opt(0).unwrap())
                            .unwrap()
                    })
                })
                .unwrap_or_else(|_| Local::now().fixed_offset())
        };

        let fm = Frontmatter {
            id,
            title,
            category,
            tags: Vec::new(), // loaded below
            status,
            source_url,
            file_path,
            parent_id,
            created_at: parse_dt(&created_at_str),
            updated_at: parse_dt(&updated_at_str),
        };

        Ok(Note::new(fm, content))
    }

    /// Load tags for a note from the database and attach them to the frontmatter.
    #[allow(dead_code)]
    pub fn populate_tags(&self, note: &mut Note) -> Result<()> {
        let mut stmt = self.conn.prepare(
            "SELECT t.name FROM tags t
             JOIN note_tags nt ON nt.tag_id = t.id
             WHERE nt.note_id = ?1
             ORDER BY t.name",
        )?;
        let tags: Vec<String> = stmt
            .query_map(params![note.frontmatter.id], |row| row.get(0))?
            .collect::<std::result::Result<Vec<_>, _>>()?;
        note.frontmatter.tags = tags;
        Ok(())
    }

    /// Load tags for multiple notes in one query.
    pub fn populate_tags_batch(&self, notes: &mut [Note]) -> Result<()> {
        if notes.is_empty() {
            return Ok(());
        }

        use std::collections::HashMap;

        let ids: Vec<String> = notes.iter().map(|n| n.frontmatter.id.clone()).collect();

        // Create placeholders for the IN clause.
        let placeholders: Vec<String> = (0..ids.len()).map(|i| format!("?{}", i + 1)).collect();
        let sql = format!(
            "SELECT nt.note_id, t.name FROM tags t
             JOIN note_tags nt ON nt.tag_id = t.id
             WHERE nt.note_id IN ({})
             ORDER BY nt.note_id, t.name",
            placeholders.join(", ")
        );

        let mut stmt = self.conn.prepare(&sql)?;
        let params_refs: Vec<&dyn rusqlite::types::ToSql> = ids
            .iter()
            .map(|id| id as &dyn rusqlite::types::ToSql)
            .collect();

        let mut tag_map: HashMap<String, Vec<String>> = HashMap::new();
        let rows = stmt.query_map(params_refs.as_slice(), |row| {
            Ok((row.get::<_, String>(0)?, row.get::<_, String>(1)?))
        })?;
        for row in rows {
            let (note_id, tag_name) = row?;
            tag_map.entry(note_id).or_default().push(tag_name);
        }

        for note in notes.iter_mut() {
            if let Some(tags) = tag_map.get(&note.frontmatter.id) {
                note.frontmatter.tags = tags.clone();
            }
        }

        Ok(())
    }
}
