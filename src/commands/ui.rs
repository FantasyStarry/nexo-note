use crate::cli::Cli;
use crate::commands::open_repo;
use crate::storage::repo::NoteFilters;
use axum::{
    Router,
    extract::{Path, Query, State},
    http::Method,
    response::Json,
    routing::get,
};
use serde::Deserialize;
use std::sync::Arc;
use tower_http::cors::{Any, CorsLayer};

#[derive(Clone)]
struct AppState {
    cli: Cli,
}

#[derive(Deserialize)]
struct ListQuery {
    category: Option<String>,
    tag: Option<String>,
    q: Option<String>,
    limit: Option<usize>,
}

pub fn run(cli: &Cli) -> anyhow::Result<()> {
    let port = 3456;
    eprintln!("📓 nexo-note UI starting at http://localhost:{}", port);

    let state = AppState { cli: cli.clone() };

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods([Method::GET, Method::POST])
        .allow_headers(Any);

    let app = Router::new()
        .route("/api/notes", get(list_notes))
        .route("/api/notes/{id}", get(get_note))
        .route("/api/tags", get(list_tags))
        .route("/api/stats", get(get_stats))
        .route("/api/thread/{id}", get(get_thread))
        .layer(cors)
        .with_state(Arc::new(state));

    let rt = tokio::runtime::Runtime::new()?;
    rt.block_on(async {
        let listener = tokio::net::TcpListener::bind(format!("127.0.0.1:{}", port))
            .await
            .expect("failed to bind to port");
        axum::serve(listener, app)
            .await
            .expect("server error");
    });

    Ok(())
}

async fn list_notes(
    State(state): State<Arc<AppState>>,
    Query(query): Query<ListQuery>,
) -> Json<serde_json::Value> {
    let repo = open_repo(&state.cli).ok();
    if repo.is_none() {
        return Json(serde_json::json!({ "success": false, "error": "cannot open repo" }));
    }
    let repo = repo.unwrap();
    let filters = NoteFilters {
        category: query.category,
        tag: query.tag,
        status: None,
        limit: query.limit,
        since: None,
    };

    let notes = if let Some(ref q) = query.q {
        repo.search_notes(q, &[]).unwrap_or_default()
    } else {
        repo.list_notes(&filters).unwrap_or_default()
    };

    let summaries: Vec<serde_json::Value> = notes
        .iter()
        .map(|n| {
            serde_json::json!({
                "id": n.frontmatter.id,
                "title": n.frontmatter.title,
                "category": n.frontmatter.category,
                "tags": n.frontmatter.tags,
                "status": n.frontmatter.status,
                "created_at": n.frontmatter.created_at.to_rfc3339(),
            })
        })
        .collect();

    Json(serde_json::json!({ "success": true, "data": summaries }))
}

async fn get_note(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> Json<serde_json::Value> {
    let repo = match open_repo(&state.cli) {
        Ok(r) => r,
        Err(e) => return Json(serde_json::json!({ "success": false, "error": e.to_string() })),
    };
    match repo.find_note(&id) {
        Ok(Some(note)) => Json(serde_json::json!({
            "success": true,
            "data": {
                "id": note.frontmatter.id,
                "title": note.frontmatter.title,
                "category": note.frontmatter.category,
                "tags": note.frontmatter.tags,
                "status": note.frontmatter.status,
                "content": note.content,
                "file_path": note.frontmatter.file_path,
                "parent_id": note.frontmatter.parent_id,
                "created_at": note.frontmatter.created_at.to_rfc3339(),
                "updated_at": note.frontmatter.updated_at.to_rfc3339(),
            }
        })),
        Ok(None) => Json(serde_json::json!({ "success": false, "error": "not found" })),
        Err(e) => Json(serde_json::json!({ "success": false, "error": e.to_string() })),
    }
}

async fn get_thread(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> Json<serde_json::Value> {
    let repo = match open_repo(&state.cli) {
        Ok(r) => r,
        Err(e) => return Json(serde_json::json!({ "success": false, "error": e.to_string() })),
    };
    match repo.get_thread(&id) {
        Ok(notes) => {
            let items: Vec<serde_json::Value> = notes
                .iter()
                .map(|n| {
                    serde_json::json!({
                        "id": n.frontmatter.id,
                        "title": n.frontmatter.title,
                        "category": n.frontmatter.category,
                        "tags": n.frontmatter.tags,
                        "status": n.frontmatter.status,
                        "parent_id": n.frontmatter.parent_id,
                        "created_at": n.frontmatter.created_at.to_rfc3339(),
                    })
                })
                .collect();
            Json(serde_json::json!({
                "success": true,
                "data": { "notes": items, "total": items.len() }
            }))
        }
        Err(e) => Json(serde_json::json!({ "success": false, "error": e.to_string() })),
    }
}

async fn list_tags(State(state): State<Arc<AppState>>) -> Json<serde_json::Value> {
    let repo = match open_repo(&state.cli) {
        Ok(r) => r,
        Err(e) => return Json(serde_json::json!({ "success": false, "error": e.to_string() })),
    };
    let db = repo.database();
    match db.stats() {
        Ok(stats) => {
            let tags: Vec<serde_json::Value> = stats
                .top_tags
                .iter()
                .map(|t| serde_json::json!({ "tag": t.tag, "count": t.count }))
                .collect();
            Json(serde_json::json!({ "success": true, "data": tags }))
        }
        Err(e) => Json(serde_json::json!({ "success": false, "error": e.to_string() })),
    }
}

async fn get_stats(State(state): State<Arc<AppState>>) -> Json<serde_json::Value> {
    let repo = match open_repo(&state.cli) {
        Ok(r) => r,
        Err(e) => return Json(serde_json::json!({ "success": false, "error": e.to_string() })),
    };
    let db = repo.database();
    match db.stats() {
        Ok(stats) => Json(serde_json::json!({ "success": true, "data": stats })),
        Err(e) => Json(serde_json::json!({ "success": false, "error": e.to_string() })),
    }
}
