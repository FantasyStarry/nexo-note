use crate::models::frontmatter::Frontmatter;
use anyhow::Result;
use serde::Serialize;
use std::path::PathBuf;

/// Unified JSON response wrapper for all CLI outputs.
#[derive(Debug, Serialize)]
pub struct ApiResponse<T: Serialize> {
    pub success: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<T>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<ApiError>,
}

impl<T: Serialize> ApiResponse<T> {
    pub fn ok(data: T) -> Self {
        Self {
            success: true,
            data: Some(data),
            message: None,
            error: None,
        }
    }

    pub fn ok_with_message(data: T, message: impl Into<String>) -> Self {
        Self {
            success: true,
            data: Some(data),
            message: Some(message.into()),
            error: None,
        }
    }

    #[allow(dead_code)]
    pub fn err(code: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            success: false,
            data: None,
            message: None,
            error: Some(ApiError {
                code: code.into(),
                message: message.into(),
                suggestion: None,
            }),
        }
    }

    #[allow(dead_code)]
    pub fn err_with_suggestion(
        code: impl Into<String>,
        message: impl Into<String>,
        suggestion: impl Into<String>,
    ) -> Self {
        Self {
            success: false,
            data: None,
            message: None,
            error: Some(ApiError {
                code: code.into(),
                message: message.into(),
                suggestion: Some(suggestion.into()),
            }),
        }
    }
}

#[derive(Debug, Serialize)]
pub struct ApiError {
    pub code: String,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suggestion: Option<String>,
}

/// Data payload returned when creating or viewing a note.
#[derive(Debug, Serialize)]
pub struct NoteData {
    pub id: String,
    pub path: PathBuf,
    pub frontmatter: Frontmatter,
    pub content: String,
}

/// Summary row returned by `nexo ls`.
#[derive(Debug, Serialize)]
pub struct NoteSummary {
    pub id: String,
    pub date: String,
    pub status: String,
    pub tags: Vec<String>,
    pub title: String,
}

/// Data payload returned by `nexo stats`.
#[derive(Debug, Serialize)]
pub struct StatsData {
    pub total_notes: usize,
    pub by_category: Vec<CategoryCount>,
    pub by_status: StatusCount,
    pub this_month: usize,
    pub total_tags: usize,
    pub top_tags: Vec<TagCount>,
}

#[derive(Debug, Serialize)]
pub struct CategoryCount {
    pub category: String,
    pub total: usize,
    pub active: usize,
    pub archived: usize,
}

#[derive(Debug, Serialize, Default)]
pub struct StatusCount {
    pub active: usize,
    pub archived: usize,
    pub draft: usize,
    pub other: usize,
}

#[derive(Debug, Serialize)]
pub struct TagCount {
    pub tag: String,
    pub count: usize,
}

/// Render an API response as either JSON or human-readable text.
pub fn print<T: Serialize + HumanText>(response: &ApiResponse<T>, json: bool) -> Result<()> {
    if json {
        println!("{}", serde_json::to_string_pretty(response)?);
    } else {
        match &response.data {
            Some(data) => println!("{}", data.human_text()),
            None => {
                if let Some(err) = &response.error {
                    eprintln!("Error [{}]: {}", err.code, err.message);
                    if let Some(suggestion) = &err.suggestion {
                        eprintln!("建议: {}", suggestion);
                    }
                } else if let Some(message) = &response.message {
                    println!("{}", message);
                }
            }
        }
    }
    Ok(())
}

/// Types that can be rendered as human-readable text.
pub trait HumanText {
    fn human_text(&self) -> String;
}

impl HumanText for NoteData {
    fn human_text(&self) -> String {
        format!(
            "ID: {}\n路径: {}\n\n{}",
            self.id,
            self.path.display(),
            crate::models::note::Note::new(self.frontmatter.clone(), self.content.clone())
        )
    }
}

impl HumanText for Vec<NoteSummary> {
    fn human_text(&self) -> String {
        if self.is_empty() {
            return "No notes found.".to_string();
        }

        // Compute column widths.
        let mut id_width = 2;
        let mut date_width = 4; // "日期" length
        let mut status_width = 4; // "状态" length
        let mut tags_width = 4; // "标签" length
        let mut title_width = 4; // "标题" length

        for note in self {
            id_width = id_width.max(note.id.len());
            date_width = date_width.max(note.date.len());
            status_width = status_width.max(note.status.len());
            let tags = format!("[{}]", note.tags.join(", "));
            tags_width = tags_width.max(tags.len());
            title_width = title_width.max(note.title.len());
        }

        let mut lines = vec![format!(
            "{:<id_width$} | {:<date_width$} | {:<status_width$} | {:<tags_width$} | {:<title_width$}",
            "ID", "日期", "状态", "标签", "标题"
        )];
        lines.push("-".repeat(lines[0].len()));

        for note in self {
            let tags = format!("[{}]", note.tags.join(", "));
            lines.push(format!(
                "{:<id_width$} | {:<date_width$} | {:<status_width$} | {:<tags_width$} | {}",
                note.id, note.date, note.status, tags, note.title
            ));
        }

        lines.join("\n")
    }
}

impl HumanText for Vec<String> {
    fn human_text(&self) -> String {
        if self.is_empty() {
            return "No tags found.".to_string();
        }
        self.join("\n")
    }
}

impl HumanText for StatsData {
    fn human_text(&self) -> String {
        let mut lines = vec![format!("总笔记: {}", self.total_notes)];

        lines.push("按分类:".to_string());
        for cat in &self.by_category {
            lines.push(format!(
                "  {}: {} (active: {}, archived: {})",
                cat.category, cat.total, cat.active, cat.archived
            ));
        }

        lines.push(format!(
            "按状态: active={}, archived={}, draft={}, other={}",
            self.by_status.active, self.by_status.archived, self.by_status.draft, self.by_status.other
        ));

        lines.push(format!("本月新增: {}", self.this_month));
        lines.push(format!("标签总数: {}", self.total_tags));

        if !self.top_tags.is_empty() {
            lines.push("Top 标签:".to_string());
            for tag in &self.top_tags {
                lines.push(format!("  {} ({})", tag.tag, tag.count));
            }
        }

        lines.join("\n")
    }
}

impl HumanText for String {
    fn human_text(&self) -> String {
        self.clone()
    }
}
