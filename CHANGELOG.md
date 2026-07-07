# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.6.2] - 2026-07-07 (unreleased)

### Added
- 新增 `nexo link <id> [--parent <pid>]`：将已有笔记挂接到链中（设置 `parent_id`）；不传 `--parent` 时自动关联到该笔记创建日期的日志
- 新增 `nexo relink`：批量把数据库中所有「无父节点」的非 journal 笔记关联到其创建日期的日志，解决了早期通过旧 MCP / PowerShell 写入、未自动关联的孤立笔记；支持 `--dry-run` 预览
- MCP 新增 `link_note` 工具，与 CLI `link` 行为一致，便于 AI 助手在创建后补关联

### Fixed
- 修复 MCP `create_note` 创建笔记后笔记链为空的问题：此前 MCP 创建的非 journal 笔记未像 CLI 那样自动挂接到「当日日志」作为父节点，导致 `get_thread` 只返回单条笔记、Web UI 显示「暂无笔记链」。现已与 CLI 行为对齐，未显式指定 `parent_id` 时自动关联当日日志
- 修复 `nexo init-mcp` 生成的 MCP 配置依赖 `nexo` 在 PATH 中（本地 npm 安装时往往不满足），改为写入 nexo 二进制绝对路径，确保 AI 助手开箱即可启动 MCP Server

### Changed
- `npm/install.js` 安装脚本增强：递归跟随重定向、下载完成后校验文件确为有效二进制（而非 GitHub 的 HTML 404 页面），失败时给出明确指引，避免留下损坏的二进制导致后续命令静默失败

## [0.6.1] - 2026-07-07

### Added
- 笔记链独立页面：将内联的笔记链从笔记详情中拆出，改为专属 `/thread/[id]` 页面，采用垂直树状 `ThreadGraph` 组件（圆角节点卡片 + 连接线）展示笔记关系脉络
- 分类「表格视图」：列表头部新增 列表 / 表格 视图切换，将同一分类的笔记以「标题 / 分类 / 标签 / 时间」列式呈现（类 Notion 数据库），点击行打开详情；视图偏好持久化到 `localStorage`
- UI 整体重设计：暖色纸张 + 克制青绿强调色的编辑风视觉（content-first），含明暗双主题

### Fixed
- 修复笔记链页面 `thread is not defined` 运行时报错
- 修复笔记链操作逻辑：详情页点击「笔记链」不再整页跳走并丢失左侧列表（改为保留分栏的独立页面）；点击链中节点就地重新聚焦该节点的链；侧边栏分类 / 标签 / 搜索在链页面可正常返回列表，消除导航迷失
- 修复筛选上下文：进入 / 离开链页面时携带并还原 `cat / tag / q` 参数，保持列表上下文

### Changed
- 「笔记链」入口从详情头部与日期挤在一行，改为独立的带图标按钮
- 列表与表格的选中态统一为青绿左边框 + 淡青绿底，与强调色呼应

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
