use rmcp::{
    ServerHandler, ServiceExt,
    handler::server::{router::tool::ToolRouter, wrapper::Parameters},
    model::{ServerCapabilities, ServerInfo},
    schemars, tool, tool_handler, tool_router,
};
use serde::Serialize;
use std::sync::{Arc, Mutex};

use crate::models::note::Note;
use crate::storage::repo::{NoteFilters, Repo};

// ──────────────────────────────────────
// Server struct
// ──────────────────────────────────────

#[derive(Clone)]
pub struct NexoServer {
    repo: Arc<Mutex<Repo>>,
    tool_router: ToolRouter<Self>,
}

impl NexoServer {
    pub fn new(repo: Repo) -> Self {
        Self {
            repo: Arc::new(Mutex::new(repo)),
            tool_router: Self::tool_router(),
        }
    }
}

// ──────────────────────────────────────
// Helpers
// ──────────────────────────────────────

fn ok<T: Serialize>(data: &T) -> String {
    serde_json::to_string(&serde_json::json!({ "success": true, "data": data }))
        .unwrap_or_else(|_| r#"{"success":false,"error":"serialization failed"}"#.into())
}

fn err(msg: impl std::fmt::Display) -> String {
    serde_json::to_string(&serde_json::json!({ "success": false, "error": msg.to_string() }))
        .unwrap_or_else(|_| r#"{"success":false,"error":"unknown"}"#.into())
}

fn note_summary(note: &Note, file_path: Option<String>) -> serde_json::Value {
    serde_json::json!({
        "id": note.frontmatter.id,
        "title": note.frontmatter.title,
        "category": note.frontmatter.category,
        "status": note.frontmatter.status,
        "tags": note.frontmatter.tags,
        "source_url": note.frontmatter.source_url,
        "created_at": note.frontmatter.created_at.to_rfc3339(),
        "updated_at": note.frontmatter.updated_at.to_rfc3339(),
        "file_path": file_path,
    })
}

// ──────────────────────────────────────
// Tool parameter structs
// ──────────────────────────────────────

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct ListNotesParams {
    /// Filter by category (e.g., "issues", "articles", "ideas", "projects")
    pub category: Option<String>,
    /// Filter by tag name
    pub tag: Option<String>,
    /// Filter by status: "active", "archived", "draft"
    pub status: Option<String>,
    /// Maximum number of notes to return (default: 50)
    pub limit: Option<usize>,
}

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct SearchNotesParams {
    /// Search keyword (matches note title and content)
    pub keyword: String,
    /// Filter by tags (all must match)
    #[serde(default)]
    pub tags: Vec<String>,
}

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct GetNoteParams {
    /// Note ID (e.g., "issues-20260704-001")
    pub id: String,
}

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct CreateNoteParams {
    /// Note title
    pub title: String,
    /// Category (e.g., "issues", "articles", "ideas", "projects")
    pub category: String,
    /// Tags for the note
    #[serde(default)]
    pub tags: Vec<String>,
    /// Source URL if the note is derived from a web page
    pub source_url: Option<String>,
    /// Markdown content of the note
    pub content: Option<String>,
    /// Optional parent note ID to create a chain (thread)
    pub parent_id: Option<String>,
}

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct NoteIdParams {
    /// Note ID
    pub id: String,
}

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct LinkNoteParams {
    /// Note ID to link
    pub id: String,
    /// Parent note ID to link under (optional; defaults to the note's own-date journal)
    pub parent_id: Option<String>,
}

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct RenameTagParams {
    /// Old tag name
    pub old: String,
    /// New tag name
    pub new: String,
}

// ──────────────────────────────────────
// Tool implementations
// ──────────────────────────────────────

#[tool_router]
impl NexoServer {
    #[tool(
        description = "List notes with optional filters by category, tag, status, and limit. \
        Returns note summaries (id, title, category, status, tags, timestamps). \
        By default only active notes are shown; set status to 'archived' or 'all' to see more."
    )]
    fn list_notes(&self, Parameters(params): Parameters<ListNotesParams>) -> String {
        let repo = self.repo.lock().unwrap_or_else(|e| e.into_inner());
        let filters = NoteFilters {
            category: params.category,
            tag: params.tag,
            status: params.status,
            limit: Some(params.limit.unwrap_or(50)),
            since: None,
        };
        match repo.list_notes(&filters) {
            Ok(notes) => {
                let summaries: Vec<serde_json::Value> = notes
                    .iter()
                    .map(|n| {
                        note_summary(n, Some(repo.note_path(&n.frontmatter.id).display().to_string()))
                    })
                    .collect();
                ok(&summaries)
            }
            Err(e) => err(e),
        }
    }

    #[tool(
        description = "Search notes by keyword in title or content. \
        Optionally filter by tags (all tags must match). \
        Searches across both active and archived notes."
    )]
    fn search_notes(&self, Parameters(params): Parameters<SearchNotesParams>) -> String {
        let repo = self.repo.lock().unwrap_or_else(|e| e.into_inner());
        match repo.search_notes(&params.keyword, &params.tags) {
            Ok(notes) => {
                let summaries: Vec<serde_json::Value> = notes
                    .iter()
                    .map(|n| {
                        note_summary(n, Some(repo.note_path(&n.frontmatter.id).display().to_string()))
                    })
                    .collect();
                ok(&summaries)
            }
            Err(e) => err(e),
        }
    }

    #[tool(description = "Get a single note by ID, including full markdown content and metadata.")]
    fn get_note(&self, Parameters(params): Parameters<GetNoteParams>) -> String {
        let repo = self.repo.lock().unwrap_or_else(|e| e.into_inner());
        match repo.find_note(&params.id) {
            Ok(Some(note)) => {
                let data = serde_json::json!({
                    "id": note.frontmatter.id,
                    "title": note.frontmatter.title,
                    "category": note.frontmatter.category,
                    "status": note.frontmatter.status,
                    "tags": note.frontmatter.tags,
                    "source_url": note.frontmatter.source_url,
                    "created_at": note.frontmatter.created_at.to_rfc3339(),
                    "updated_at": note.frontmatter.updated_at.to_rfc3339(),
                    "content": note.content,
                    "file_path": repo.note_path(&note.frontmatter.id).display().to_string(),
                });
                ok(&data)
            }
            Ok(None) => err(format!("note '{}' not found", params.id)),
            Err(e) => err(e),
        }
    }

    #[tool(
        description = "Create a new note. Requires title and category. \
        Optionally provide tags, source_url, markdown content, and parent_id for chaining. \
        Returns the created note's ID and metadata."
    )]
    fn create_note(&self, Parameters(params): Parameters<CreateNoteParams>) -> String {
        let repo = self.repo.lock().unwrap_or_else(|e| e.into_inner());
        let mut extra = std::collections::HashMap::new();

        // Mirror the CLI `create` behavior: a non-journal note is auto-linked to
        // today's journal (its parent) unless an explicit parent_id is supplied.
        // Without this, MCP-created notes had parent_id = None, so get_thread()
        // returned a single-element chain and the Web UI showed "暂无笔记链".
        let parent_id = if let Some(pid) = &params.parent_id {
            Some(pid.clone())
        } else if params.category != "journal" {
            repo.ensure_today_journal().ok()
        } else {
            None
        };
        if let Some(pid) = &parent_id {
            extra.insert("parent_id".to_string(), pid.clone());
        }

        match repo.create_note(
            &params.title,
            &params.category,
            params.tags,
            params.source_url.as_deref(),
            params.content.as_deref(),
            extra,
        ) {
            Ok(note) => {
                let data = serde_json::json!({
                    "id": note.frontmatter.id,
                    "title": note.frontmatter.title,
                    "category": note.frontmatter.category,
                    "status": note.frontmatter.status,
                    "tags": note.frontmatter.tags,
                    "source_url": note.frontmatter.source_url,
                    "created_at": note.frontmatter.created_at.to_rfc3339(),
                    "updated_at": note.frontmatter.updated_at.to_rfc3339(),
                    "file_path": repo.note_path(&note.frontmatter.id).display().to_string(),
                });
                ok(&data)
            }
            Err(e) => err(e),
        }
    }

    #[tool(
        description = "Archive a note by ID. Moves the note to archived status \
        and relocates the .md file to the archive directory."
    )]
    fn archive_note(&self, Parameters(params): Parameters<NoteIdParams>) -> String {
        let repo = self.repo.lock().unwrap_or_else(|e| e.into_inner());
        match repo.archive_note(&params.id) {
            Ok(()) => ok(&serde_json::json!({ "id": params.id, "archived": true })),
            Err(e) => err(e),
        }
    }

    #[tool(
        description = "Delete a note by ID. Permanently removes it from \
        both SQLite and the file system. This action cannot be undone."
    )]
    fn delete_note(&self, Parameters(params): Parameters<NoteIdParams>) -> String {
        let repo = self.repo.lock().unwrap_or_else(|e| e.into_inner());
        match repo.delete_note(&params.id) {
            Ok(()) => ok(&serde_json::json!({ "id": params.id, "deleted": true })),
            Err(e) => err(e),
        }
    }

    #[tool(
        description = "Link a note into a chain by setting its parent. \
        Provide parent_id to attach it under a specific note, or omit it to link the note \
        to the journal of its own creation date. Returns the resulting parent_id."
    )]
    fn link_note(&self, Parameters(params): Parameters<LinkNoteParams>) -> String {
        let repo = self.repo.lock().unwrap_or_else(|e| e.into_inner());
        let note = match repo.find_note(&params.id) {
            Ok(Some(n)) => n,
            Ok(None) => return err(format!("note '{}' not found", params.id)),
            Err(e) => return err(e),
        };
        let parent_id = if let Some(pid) = &params.parent_id {
            if !repo.database().note_exists(pid).unwrap_or(false) {
                return err(format!("parent note '{}' not found", pid));
            }
            pid.clone()
        } else {
            let date = note.frontmatter.created_at.date_naive();
            match repo.ensure_journal_for_date(date) {
                Ok(id) => id,
                Err(e) => return err(e),
            }
        };
        match repo.set_parent(&params.id, Some(&parent_id)) {
            Ok(()) => ok(&serde_json::json!({ "id": params.id, "parent_id": parent_id })),
            Err(e) => err(e),
        }
    }

    #[tool(description = "List all unique tags across all notes.")]
    fn list_tags(&self) -> String {
        let repo = self.repo.lock().unwrap_or_else(|e| e.into_inner());
        match repo.all_tags() {
            Ok(tags) => ok(&tags),
            Err(e) => err(e),
        }
    }

    #[tool(
        description = "Rename a tag across all notes. Returns the number of notes affected. \
        Also updates the corresponding .md files."
    )]
    fn rename_tag(&self, Parameters(params): Parameters<RenameTagParams>) -> String {
        let repo = self.repo.lock().unwrap_or_else(|e| e.into_inner());
        match repo.rename_tag(&params.old, &params.new) {
            Ok(count) => {
                ok(&serde_json::json!({ "old": params.old, "new": params.new, "affected": count }))
            }
            Err(e) => err(e),
        }
    }

    #[tool(
        description = "Get notebook statistics: total notes, breakdown by category and status, \
        notes created this month, total tags, and top 10 tags by usage."
    )]
    fn get_stats(&self) -> String {
        let repo = self.repo.lock().unwrap_or_else(|e| e.into_inner());
        let db = repo.database();
        match db.stats() {
            Ok(stats) => ok(&stats),
            Err(e) => err(e),
        }
    }

    #[tool(
        description = "Get the full thread chain for a note by ID. \
        Walks up the parent chain to find the root, then lists all descendants. \
        Returns an ordered array of notes showing the thread hierarchy."
    )]
    fn get_thread(&self, Parameters(params): Parameters<NoteIdParams>) -> String {
        let repo = self.repo.lock().unwrap_or_else(|e| e.into_inner());
        match repo.get_thread(&params.id) {
            Ok(notes) => {
                let items: Vec<serde_json::Value> = notes
                    .iter()
                    .map(|n| {
                        serde_json::json!({
                            "id": n.frontmatter.id,
                            "title": n.frontmatter.title,
                            "category": n.frontmatter.category,
                            "status": n.frontmatter.status,
                            "tags": n.frontmatter.tags,
                            "parent_id": n.frontmatter.parent_id,
                            "created_at": n.frontmatter.created_at.to_rfc3339(),
                        })
                    })
                    .collect();
                ok(&serde_json::json!({ "notes": items, "total": items.len() }))
            }
            Err(e) => err(e),
        }
    }
}

// ──────────────────────────────────────
// ServerHandler impl (#[tool_handler] generates call_tool/list_tools/get_tool)
// ──────────────────────────────────────

#[tool_handler]
impl ServerHandler for NexoServer {
    fn get_info(&self) -> ServerInfo {
        ServerInfo {
            instructions: Some(
                "nexo-note: A local markdown-based notes knowledge base. \
                Use list_notes to browse, search_notes to find by keyword, \
                get_note to read full content, create_note to add new notes, \
                get_thread to view the full thread chain for a note, \
                link_note to attach a note under a parent (or its date journal)."
                    .into(),
            ),
            capabilities: ServerCapabilities::builder().enable_tools().build(),
            ..Default::default()
        }
    }
}

// ──────────────────────────────────────
// Entry point
// ──────────────────────────────────────

/// Start the MCP server on stdio transport.
pub async fn serve(repo: Repo) -> anyhow::Result<()> {
    let service = NexoServer::new(repo)
        .serve(rmcp::transport::stdio())
        .await?;
    service.waiting().await?;
    Ok(())
}
