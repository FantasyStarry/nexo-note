mod cli;
mod commands;
mod config;
mod mcp;
mod models;
mod output;
mod storage;

use clap::Parser;
use output::{print, ApiResponse, ApiError};

fn main() {
    let args = cli::Cli::parse();
    let json = args.json;
    if let Err(e) = commands::run(args) {
        handle_error(e, json);
    }
}

fn handle_error(e: anyhow::Error, json: bool) {
    if json {
        let root_cause = e.root_cause().to_string();
        let suggestion = if root_cause.contains("invalid category") {
            Some("允许的分类: issues, articles, ideas, projects".to_string())
        } else {
            None
        };
        let code = guess_error_code(&root_cause);
        let response = ApiResponse::<String> {
            success: false,
            data: None,
            message: None,
            error: Some(ApiError {
                code,
                message: root_cause,
                suggestion,
            }),
        };
        let _ = print(&response, true);
    } else {
        eprintln!("Error: {}", e);
    }
    std::process::exit(1);
}

fn guess_error_code(message: &str) -> String {
    if message.contains("invalid category") {
        "INVALID_CATEGORY".to_string()
    } else if message.contains("not found") {
        "NOT_FOUND".to_string()
    } else if message.contains("Permission denied") {
        "PERMISSION_DENIED".to_string()
    } else {
        "UNKNOWN_ERROR".to_string()
    }
}
