use crate::cli::Cli;
use crate::commands::open_repo;
use crate::output::{print, ApiResponse, ThreadData, ThreadNote};
use anyhow::Result;

/// Show the full thread chain for a note.
///
/// Walks up the parent chain to find the root, then lists all descendants.
pub fn run(cli: &Cli, id: &str) -> Result<()> {
    let repo = open_repo(cli)?;

    let thread = repo.get_thread(id)?;

    let total = thread.len();
    let notes: Vec<ThreadNote> = thread
        .into_iter()
        .map(|note| ThreadNote {
            id: note.frontmatter.id.clone(),
            title: note.frontmatter.title,
            category: note.frontmatter.category,
            status: note.frontmatter.status,
            tags: note.frontmatter.tags,
            depth: 0,
            parent_id: note.frontmatter.parent_id,
        })
        .collect();

    // Compute depth by building a parent map.
    let mut depth_map: std::collections::HashMap<String, usize> = std::collections::HashMap::new();
    for note in &notes {
        if note.parent_id.is_none() {
            depth_map.insert(note.id.clone(), 0);
        }
    }
    // Iteratively assign depths.
    let mut changed = true;
    while changed {
        changed = false;
        for note in &notes {
            if depth_map.contains_key(&note.id) {
                continue;
            }
            if let Some(ref pid) = note.parent_id {
                if let Some(pdepth) = depth_map.get(pid) {
                    depth_map.insert(note.id.clone(), pdepth + 1);
                    changed = true;
                }
            }
        }
    }

    let notes_with_depth: Vec<ThreadNote> = notes
        .into_iter()
        .map(|mut n| {
            n.depth = depth_map.get(&n.id).copied().unwrap_or(0);
            n
        })
        .collect();

    let data = ThreadData {
        total,
        notes: notes_with_depth,
    };

    print(&ApiResponse::ok(data), cli.json)?;
    Ok(())
}
