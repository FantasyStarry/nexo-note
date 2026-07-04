//! `nexo init-mcp` command - Initialize MCP Server configuration for AI agents
//!
//! This command detects installed AI agents on the system and configures
//! the nexo MCP Server for each of them.

use std::fs;
use std::path::{Path, PathBuf};

use crate::cli::InitMcpArgs;
use anyhow::{Context, Result};

/// Supported AI agents and their MCP configuration paths
#[derive(Debug, Clone)]
struct AgentConfig {
    /// Agent name (for display)
    name: &'static str,
    /// Configuration file path (global)
    global_config_path: Option<PathBuf>,
    /// Configuration file path (project-level)
    project_config_path: Option<PathBuf>,
    /// Whether the agent is installed
    is_installed: bool,
}

/// Get the current executable path
fn get_executable_path() -> Result<PathBuf> {
    // Try to get the current executable path
    let exe_path = std::env::current_exe()
        .context("Failed to get current executable path")?;
    
    Ok(exe_path)
}

/// Detect installed AI agents
fn detect_agents() -> Vec<AgentConfig> {
    let mut agents = Vec::new();
    let home_dir = dirs::home_dir();
    
    // WorkBuddy
    if let Some(home) = &home_dir {
        let workbuddy_config = home.join(".workbuddy/mcp.json");
        agents.push(AgentConfig {
            name: "WorkBuddy",
            global_config_path: Some(workbuddy_config),
            project_config_path: None,
            is_installed: home.join(".workbuddy").exists(),
        });
    }
    
    // Claude Code
    if let Some(home) = &home_dir {
        let claude_config = home.join(".claude/mcp.json");
        let claude_json = home.join(".claude.json");
        agents.push(AgentConfig {
            name: "Claude Code",
            global_config_path: Some(claude_config),
            project_config_path: Some(PathBuf::from(".claude/mcp.json")),
            is_installed: claude_json.exists() || home.join(".claude").exists(),
        });
    }
    
    // Cursor
    if let Some(home) = &home_dir {
        let cursor_config = home.join(".cursor/mcp.json");
        agents.push(AgentConfig {
            name: "Cursor",
            global_config_path: Some(cursor_config),
            project_config_path: Some(PathBuf::from(".cursor/mcp.json")),
            is_installed: home.join(".cursor").exists(),
        });
    }
    
    // Codex (assuming similar structure)
    if let Some(home) = &home_dir {
        let codex_config = home.join(".codex/mcp.json");
        agents.push(AgentConfig {
            name: "Codex",
            global_config_path: Some(codex_config),
            project_config_path: Some(PathBuf::from(".codex/mcp.json")),
            is_installed: home.join(".codex").exists(),
        });
    }
    
    // Windsurf
    if let Some(home) = &home_dir {
        let windsurf_config = home.join(".windsurf/mcp.json");
        agents.push(AgentConfig {
            name: "Windsurf",
            global_config_path: Some(windsurf_config),
            project_config_path: Some(PathBuf::from(".windsurf/mcp.json")),
            is_installed: home.join(".windsurf").exists(),
        });
    }
    
    agents
}

/// Create MCP server configuration JSON
fn create_mcp_config(_executable_path: &Path) -> serde_json::Value {
    // Use "nexo" instead of full path to avoid encoding issues
    // This assumes nexo is in PATH
    serde_json::json!({
        "mcpServers": {
            "nexo-note": {
                "command": "nexo",
                "args": ["serve"]
            }
        }
    })
}

/// Merge new MCP config with existing config
fn merge_config(existing: &mut serde_json::Value, new: serde_json::Value) {
    if let Some(servers) = new.get("mcpServers") {
        if let Some(obj) = existing.as_object_mut() {
            if !obj.contains_key("mcpServers") {
                obj.insert("mcpServers".to_string(), serde_json::json!({}));
            }
            if let Some(mcp_servers) = obj.get_mut("mcpServers") {
                if let Some(servers_obj) = servers.as_object() {
                    for (key, value) in servers_obj {
                        mcp_servers[key] = value.clone();
                    }
                }
            }
        }
    }
}

/// Save configuration to file
fn save_config(config_path: &Path, config: &serde_json::Value) -> Result<()> {
    // Create parent directory if it doesn't exist
    if let Some(parent) = config_path.parent() {
        fs::create_dir_all(parent)
            .with_context(|| format!("Failed to create directory: {}", parent.display()))?;
    }
    
    // Backup existing config
    if config_path.exists() {
        let backup_path = config_path.with_extension("json.bak");
        fs::copy(config_path, &backup_path)
            .with_context(|| format!("Failed to backup config to: {}", backup_path.display()))?;
        println!("  ✓ Backed up existing config to: {}", backup_path.display());
    }
    
    // Write new config
    let file = fs::File::create(config_path)
        .with_context(|| format!("Failed to create config file: {}", config_path.display()))?;
    serde_json::to_writer_pretty(file, config)
        .with_context(|| format!("Failed to write config to: {}", config_path.display()))?;
    
    Ok(())
}

/// Run the init command
pub fn run(args: &InitMcpArgs) -> Result<()> {
    println!("{}", "=".repeat(60));
    println!("Initializing MCP Server for AI Agents");
    println!("{}", "=".repeat(60));
    println!();
    
    // Get executable path
    let exe_path = get_executable_path()?;
    println!("Using nexo binary: {}", exe_path.display());
    println!();
    
    // Detect agents
    let agents = detect_agents();
    
    // Filter agents based on arguments
    let filtered_agents: Vec<_> = if args.all {
        agents.into_iter().filter(|a| a.is_installed).collect()
    } else if let Some(ref agent_name) = args.agent {
        agents.into_iter()
            .filter(|a| a.name.to_lowercase().contains(&agent_name.to_lowercase()))
            .collect()
    } else {
        // Default: show all installed agents
        agents.into_iter().filter(|a| a.is_installed).collect()
    };
    
    if filtered_agents.is_empty() {
        println!("Warning: No installed AI agents detected.");
        println!("Supported agents: WorkBuddy, Claude Code, Cursor, Codex, Windsurf");
        return Ok(());
    }
    
    // Show detected agents
    println!("{}", "-".repeat(60));
    println!("Detected AI Agents");
    println!("{}", "-".repeat(60));
    for agent in &filtered_agents {
        let status = if agent.is_installed { "✓" } else { "✗" };
        println!("  {} {}", status, agent.name);
    }
    println!();
    
    // Configure each agent
    let mcp_config = create_mcp_config(&exe_path);
    
    for agent in &filtered_agents {
        if !agent.is_installed {
            println!("Warning: {} is not installed. Skipping.", agent.name);
            continue;
        }
        
        println!("{}", "-".repeat(60));
        println!("Configuring {}", agent.name);
        println!("{}", "-".repeat(60));
        
        // Global config
        if let Some(global_path) = &agent.global_config_path {
            let mut config = if global_path.exists() {
                let content = fs::read_to_string(global_path)
                    .with_context(|| format!("Failed to read config: {}", global_path.display()))?;
                serde_json::from_str(&content)
                    .with_context(|| format!("Failed to parse config: {}", global_path.display()))?
            } else {
                serde_json::json!({})
            };
            
            merge_config(&mut config, mcp_config.clone());
            save_config(global_path, &config)?;
            println!("  ✓ Global config: {}", global_path.display());
        }
        
        // Project config (if requested)
        if args.project {
            if let Some(project_path) = &agent.project_config_path {
                let current_dir = std::env::current_dir()
                    .context("Failed to get current directory")?;
                let full_path = current_dir.join(project_path);
                
                let mut config = if full_path.exists() {
                    let content = fs::read_to_string(&full_path)
                        .with_context(|| format!("Failed to read config: {}", full_path.display()))?;
                    serde_json::from_str(&content)
                        .with_context(|| format!("Failed to parse config: {}", full_path.display()))?
                } else {
                    serde_json::json!({})
                };
                
                merge_config(&mut config, mcp_config.clone());
                save_config(&full_path, &config)?;
                println!("  ✓ Project config: {}", full_path.display());
            }
        }
        
        println!();
    }
    
    println!("{}", "=".repeat(60));
    println!("Next Steps");
    println!("{}", "=".repeat(60));
    println!("1. Restart your AI agent to load the new MCP configuration");
    println!("2. Test the connection by asking your agent to list your notes");
    println!("3. For project-level configs, commit the .claude/mcp.json (or similar) to git");
    println!();
    
    Ok(())
}

/// Run doctor command to check MCP configuration
pub fn doctor() -> Result<()> {
    println!("{}", "=".repeat(60));
    println!("MCP Server Diagnostics");
    println!("{}", "=".repeat(60));
    println!();
    
    // Check if nexo serve works
    println!("{}", "-".repeat(60));
    println!("Testing MCP Server");
    println!("{}", "-".repeat(60));
    let exe_path = get_executable_path()?;
    println!("Executable: {}", exe_path.display());
    
    if !exe_path.exists() {
        anyhow::bail!("nexo binary not found!");
    }
    
    println!("✓ nexo binary found");
    println!();
    
    // Check SQLite database
    println!("{}", "-".repeat(60));
    println!("Checking Database");
    println!("{}", "-".repeat(60));
    let config = crate::config::Config::load().unwrap_or_default();
    let db_path = config.resolve_notes_dir(None).join(".nexo/notes.db");
    
    if db_path.exists() {
        println!("✓ Database found: {}", db_path.display());
    } else {
        println!("Warning: Database not found: {}", db_path.display());
        println!("Run 'nexo migrate' to create the database from existing notes");
    }
    
    println!();
    
    // Check agent configurations
    println!("{}", "-".repeat(60));
    println!("Checking Agent Configurations");
    println!("{}", "-".repeat(60));
    let agents = detect_agents();
    
    for agent in &agents {
        if !agent.is_installed {
            continue;
        }
        
        println!("{}:", agent.name);
        
        if let Some(global_path) = &agent.global_config_path {
            if global_path.exists() {
                let content = fs::read_to_string(global_path)
                    .with_context(|| format!("Failed to read config: {}", global_path.display()))?;
                let config: serde_json::Value = serde_json::from_str(&content)
                    .with_context(|| format!("Failed to parse config: {}", global_path.display()))?;
                
                if config.get("mcpServers")
                    .and_then(|s| s.get("nexo-note"))
                    .is_some()
                {
                    println!("  ✓ Global config configured: {}", global_path.display());
                } else {
                    println!("  ✗ Global config exists but nexo-note not configured: {}", global_path.display());
                }
            } else {
                println!("  ✗ Global config not found: {}", global_path.display());
            }
        }
    }
    
    println!();
    Ok(())
}
