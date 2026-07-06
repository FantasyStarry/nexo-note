use std::fs;
use std::path::PathBuf;
use std::process::Command;

fn cargo_bin() -> PathBuf {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("target");
    path.push("debug");
    path.push("nexo");
    path.set_extension(std::env::consts::EXE_EXTENSION);
    path
}

fn setup_repo() -> PathBuf {
    let base_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("target")
        .join("test-repos");
    fs::create_dir_all(&base_dir).unwrap();

    let unique = format!(
        "repo-{}-{}",
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    );
    let repo = base_dir.join(unique);

    // Initialize an isolated git repository for each test so git operations
    // do not leak into the project repository.
    std::process::Command::new("git")
        .arg("init")
        .arg("-q")
        .arg(&repo)
        .output()
        .expect("failed to initialize test git repo");

    repo
}

fn run(repo_dir: &PathBuf, args: &[&str]) -> (String, String, bool) {
    let output = Command::new(cargo_bin())
        .args(args)
        .arg("--notes-dir")
        .arg(repo_dir)
        .output()
        .expect("failed to execute nexo");

    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    let stderr = String::from_utf8_lossy(&output.stderr).to_string();
    (stdout, stderr, output.status.success())
}

fn extract_id(stdout: &str) -> String {
    stdout
        .lines()
        .find(|l| l.contains("\"id\""))
        .and_then(|l| l.split('"').nth(3))
        .expect("failed to extract note id")
        .to_string()
}

fn extract_path(stdout: &str) -> String {
    stdout
        .lines()
        .find(|l| l.contains("\"path\""))
        .and_then(|l| l.split('"').nth(3))
        .expect("failed to extract note path")
        .to_string()
}

#[test]
fn test_create_and_view() {
    let repo = setup_repo();
    let (stdout, stderr, success) = run(
        &repo,
        &[
            "create",
            "Integration Test Note",
            "-c",
            "issues",
            "-t",
            "test,integration",
            "--json",
        ],
    );
    assert!(
        success,
        "create failed: stdout={} stderr={}",
        stdout, stderr
    );
    assert!(stdout.contains("issues-"));

    let id = extract_id(&stdout);

    let (stdout, stderr, success) = run(&repo, &["view", &id, "--json"]);
    assert!(success, "view failed: stdout={} stderr={}", stdout, stderr);
    assert!(stdout.contains("Integration Test Note"));
}

#[test]
fn test_list_and_search() {
    let repo = setup_repo();
    run(
        &repo,
        &["create", "Searchable Note", "-c", "articles", "-t", "rust"],
    );

    let (stdout, stderr, success) = run(&repo, &["ls", "--json"]);
    assert!(success, "ls failed: stdout={} stderr={}", stdout, stderr);
    assert!(stdout.contains("Searchable Note"));

    let (stdout, stderr, success) = run(&repo, &["search", "Searchable", "--json"]);
    assert!(
        success,
        "search failed: stdout={} stderr={}",
        stdout, stderr
    );
    assert!(stdout.contains("Searchable Note"));
}

#[test]
fn test_archive() {
    let repo = setup_repo();
    let (stdout, _, _) = run(
        &repo,
        &["create", "Note to Archive", "-c", "ideas", "--json"],
    );
    let id = extract_id(&stdout);

    let (_, stderr, success) = run(&repo, &["archive", &id]);
    assert!(success, "archive failed: stderr={}", stderr);

    let (stdout, _, success) = run(&repo, &["ls", "--json"]);
    assert!(success);
    assert!(!stdout.contains(&id));
}

#[test]
fn test_content_only_format() {
    let repo = setup_repo();
    let content = "This is the markdown body.\n\n- item 1\n- item 2";
    let (stdout, stderr, success) = run(
        &repo,
        &[
            "create",
            "Content Only Note",
            "-c",
            "articles",
            "--content",
            content,
            "--json",
        ],
    );
    assert!(
        success,
        "create failed: stdout={} stderr={}",
        stdout, stderr
    );

    let id = extract_id(&stdout);
    let note_path = extract_path(&stdout);

    let _id = extract_id(&stdout);
    let file_content = fs::read_to_string(&note_path).expect("failed to read note file");
    assert!(
        !file_content.starts_with("---\n"),
        "content-only note should not start with YAML frontmatter"
    );
    assert!(
        file_content.contains(content),
        "note file should contain the provided content"
    );
}

#[test]
fn test_readable_filename() {
    let repo = setup_repo();
    let (stdout, stderr, success) = run(
        &repo,
        &["create", "My Readable File Name Test", "-c", "articles", "--json"],
    );
    assert!(success, "create failed: stdout={} stderr={}", stdout, stderr);

    let note_path = extract_path(&stdout);
    let filename = std::path::Path::new(&note_path)
        .file_name()
        .unwrap()
        .to_string_lossy()
        .to_string();

    assert!(
        filename.contains("my-readable-file-name-test"),
        "filename should be human-readable (slugified from title), got: {}",
        filename
    );
    assert!(
        !filename.contains("articles-"),
        "filename should not contain the ID-based prefix, got: {}",
        filename
    );
}

#[test]
fn test_stats_uses_database() {
    let repo = setup_repo();
    run(
        &repo,
        &[
            "create",
            "Stats Note One",
            "-c",
            "issues",
            "-t",
            "rust,debug",
        ],
    );
    run(
        &repo,
        &["create", "Stats Note Two", "-c", "articles", "-t", "rust"],
    );

    let (stdout, stderr, success) = run(&repo, &["stats", "--json"]);
    assert!(success, "stats failed: stdout={} stderr={}", stdout, stderr);
    // total_notes = 3 because of auto-created journal note
    assert!(stdout.contains("\"total_notes\": 3"));
    assert!(stdout.contains("\"total_tags\": 2"));
    assert!(stdout.contains("rust"));
}

#[test]
fn test_note_chain() {
    let repo = setup_repo();
    let (stdout, _, success) = run(
        &repo,
        &["create", "Root Note", "-c", "issues", "--json"],
    );
    assert!(success, "root create failed: {}", stdout);
    let root_id = extract_id(&stdout);

    let (stdout, _, success) = run(
        &repo,
        &["create", "Child Note", "-c", "issues", "--link", &root_id, "--json"],
    );
    assert!(success, "child create failed: {}", stdout);
    let child_id = extract_id(&stdout);

    // Verify the child note has parent_id set.
    let (view_out, _, success) = run(&repo, &["view", &child_id, "--json"]);
    assert!(success, "view child failed: {}", view_out);
    assert!(
        view_out.contains(&root_id),
        "child note view should reference parent_id: {}",
        view_out
    );

    // Create a grandchild.
    let (gc_stdout, _, success) = run(
        &repo,
        &["create", "Grandchild Note", "-c", "issues", "--link", &child_id, "--json"],
    );
    assert!(success, "grandchild create failed: {}", gc_stdout);

    // Thread command should show all 3 notes.
    let (thread_out, _, success) = run(&repo, &["thread", &root_id, "--json"]);
    assert!(success, "thread failed: {}", thread_out);
    assert!(thread_out.contains("Root Note"), "thread should contain root");
    assert!(thread_out.contains("Child Note"), "thread should contain child");
    assert!(thread_out.contains("Grandchild Note"), "thread should contain grandchild");
}

#[test]
fn test_journal_auto_create() {
    let repo = setup_repo();
    // Creating any non-journal note should auto-create a journal entry.
    let (stdout, _, success) = run(
        &repo,
        &["create", "A regular note", "-c", "articles", "--json"],
    );
    assert!(success, "create failed: {}", stdout);

    let (ls_out, _, success) = run(&repo, &["ls", "--json"]);
    assert!(success, "ls failed: {}", ls_out);
    // We should have: the auto-created journal + the note we created
    assert!(ls_out.contains("日志"), "ls should contain the journal entry: {}", ls_out);
}
