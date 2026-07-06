use clap::{Parser, Subcommand};

#[derive(Parser, Clone)]
#[command(name = "nexo")]
#[command(alias = "nn")]
#[command(about = "A local markdown-based notes CLI for your knowledge base")]
#[command(version)]
pub struct Cli {
    /// Output results as JSON
    #[arg(long, global = true)]
    pub json: bool,

    /// Path to the notes directory
    #[arg(long, global = true)]
    pub notes_dir: Option<String>,

    /// Preview the operation without making changes
    #[arg(long, global = true)]
    pub dry_run: bool,

    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Clone)]
pub enum Commands {
    /// Initialize a new notebook repository
    Init {
        /// Initialize a git repository for version control
        #[arg(long)]
        git: bool,
    },

    /// Create a new note
    Create {
        /// Note title
        title: String,

        /// Category (e.g., issues, articles, ideas, projects)
        #[arg(short, long)]
        category: String,

        /// Comma-separated tags
        #[arg(short, long)]
        tags: Option<String>,

        /// Source URL for articles
        #[arg(short, long)]
        source_url: Option<String>,

        /// Note body content (Markdown)
        #[arg(short = 'b', long)]
        content: Option<String>,

        /// Path to a file containing the note body (Markdown)
        #[arg(long, value_name = "PATH")]
        content_file: Option<String>,

        /// Extra frontmatter fields as key=value
        #[arg(short, long)]
        extra: Vec<String>,

        /// Link this note to a parent note (creates a chain)
        #[arg(long)]
        link: Option<String>,
    },

    /// Edit a note in the default editor
    Edit {
        /// Note ID
        id: String,

        /// Editor to use (overrides config)
        #[arg(short, long)]
        editor: Option<String>,
    },

    /// View a note
    View {
        /// Note ID
        id: String,
    },

    /// List notes
    Ls {
        /// Filter by category
        #[arg(short, long)]
        category: Option<String>,

        /// Filter by tag
        #[arg(short, long)]
        tag: Option<String>,

        /// Filter by status
        #[arg(short, long)]
        status: Option<String>,

        /// Limit number of results
        #[arg(long)]
        limit: Option<usize>,

        /// Filter notes created on or after this date (YYYY-MM-DD)
        #[arg(long)]
        since: Option<String>,
    },

    /// Search notes by keyword
    Search {
        /// Search keyword
        keyword: String,

        /// Filter by comma-separated tags
        #[arg(short, long)]
        tags: Option<String>,
    },

    /// Archive a note
    Archive {
        /// Note ID
        id: String,
    },

    /// Delete a note
    Rm {
        /// Note ID
        id: String,

        /// Force deletion without archiving
        #[arg(short, long)]
        force: bool,
    },

    /// Tag management
    Tag {
        #[command(subcommand)]
        command: TagCommands,
    },

    /// Configuration management
    Config {
        #[command(subcommand)]
        command: ConfigCommands,
    },

    /// Show notebook statistics
    Stats,

    /// Migrate existing .md notes from file system into SQLite database
    Migrate,

    /// Generate shell completions
    Completions {
        /// Shell name (bash, zsh, fish, powershell)
        shell: String,
    },

    /// Start MCP server on stdio (for AI agent integration)
    Serve,

    /// Initialize MCP Server configuration for AI agents
    InitMcp(InitMcpArgs),

    /// Check MCP Server configuration and diagnose issues
    Doctor,

    /// Show the full thread chain for a note
    Thread {
        /// Note ID
        id: String,
    },

    /// Start the web UI dashboard
    Ui,
}

/// Arguments for the init-mcp command
#[derive(Parser, Debug, Clone)]
pub struct InitMcpArgs {
    /// Configure all detected AI agents
    #[arg(long)]
    pub all: bool,

    /// Configure a specific AI agent (workbuddy, claude, cursor, codex, windsurf)
    #[arg(long)]
    pub agent: Option<String>,

    /// Also create project-level configuration
    #[arg(long)]
    pub project: bool,
}

#[derive(Subcommand, Clone)]
pub enum TagCommands {
    /// List all tags
    Ls,

    /// Rename a tag across all notes
    Mv { old: String, new: String },

    /// Suggest tags for a note
    Suggest { id: String },
}

#[derive(Subcommand, Clone)]
pub enum ConfigCommands {
    /// Set a config value
    Set { key: String, value: String },

    /// Get a config value
    Get { key: String },

    /// List all config values
    List,
}
