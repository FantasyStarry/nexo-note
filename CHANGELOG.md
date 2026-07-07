# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.6.0] - 2026-07-07

### Added
- Notion 风格 UI 重构：极简美学、大量留白、柔和暖中性配色、清晰排版层级与轻量交互（细腻分割线、悬停高亮，去除厚重边框与阴影）
- 界面中文化：分类标签与主要界面文案转为中文
- 侧边栏分类实时计数与上下文相关的空状态提示，避免「选择分类后列表变空」造成的误解
- `nexo init-mcp` 新增 CodeBuddy、Trae 已知助手支持
- `nexo init-mcp` 自动发现机制：扫描 home、`%APPDATA%`、`%LOCALAPPDATA%`、`~/.config` 下符合 `~/.agent/mcp.json` + `mcpServers` 约定的配置，自动注册未知 AI 助手，无需改动 nexo 源码

### Changed
- 笔记分类图标由 emoji 改为 Lucide 图标
- README 补充支持的助手列表与自动发现说明

## [0.1.0] - 2026-07-04

### Added
- Initial release of `nexo-note`
- Create, edit, view, list, search, archive, and delete notes
- Tag management (list, rename, suggest)
- Unified JSON output for agent integration
- Local configuration with `config` command
- Notebook statistics with `stats` command
- Shell completion generation
- Git version control integration with `init --git`
- Per-note history backup before edits
- npm wrapper package for easy installation
- Trae / Claude Code skill wrapper
