# nexo-note

A local markdown-based notes CLI for your knowledge base. 一个本地 Markdown 笔记知识库 CLI 工具，支持分类、标签、搜索、版本管理，可被 agent 通过 MCP Server 调用。

## 特性

- 纯本地 Markdown + YAML frontmatter 存储
- 分类目录 + 日期层级组织
- 标签管理与重命名
- 全文搜索
- JSON 输出，方便 agent 解析
- 笔记库 Git 版本管理
- 单条笔记历史版本
- **MCP Server 支持** - 可被 AI Agent（WorkBuddy, Claude Code, Cursor, Codex, Windsurf, CodeBuddy, Trae 等）直接调用，并自动发现系统中遵循 `~/.agent/mcp.json` + `mcpServers` 约定的已安装助手
- SQLite 数据库存储元数据，快速查询

## 安装

### 方式 1：npm install（推荐）

需要 Node.js。

```bash
npm install -g nexo-note
```

### 方式 2：cargo install

需要 Rust 工具链。

```bash
cargo install --git https://github.com/FantasyStarry/nexo-note
```

### 方式 3：npx

需要 Node.js。

```bash
npx nexo-note --help
```

### 方式 4：GitHub Release

从 [GitHub Releases](https://github.com/FantasyStarry/nexo-note/releases) 下载对应平台的二进制文件，解压后放到 PATH 中。

## 配置 MCP Server（推荐）

nexo 支持 MCP Server，可以被 AI Agent（WorkBuddy, Claude Code, Cursor, Codex, Windsurf, CodeBuddy, Trae 等）直接调用。`init-mcp` 会配置已知助手，并自动扫描 home、`%APPDATA%`、`%LOCALAPPDATA%`、`~/.config` 下所有符合 `~/.agent/mcp.json` + `mcpServers` 约定的配置文件，从而发现未知的新助手。

### 自动配置

运行以下命令自动检测并配置所有已安装的 AI Agent：

```bash
nexo init-mcp --all
```

或者配置特定的 Agent：

```bash
nexo init-mcp --agent workbuddy
nexo init-mcp --agent claude
nexo init-mcp --agent cursor
```

### 手动配置

在 Agent 的 MCP 配置文件中添加：

```json
{
  "mcpServers": {
    "nexo-note": {
      "command": "nexo",
      "args": ["serve"]
    }
  }
}
```

配置文件位置：
- WorkBuddy: `~/.workbuddy/mcp.json`
- Claude Code: `~/.claude/mcp.json`
- Cursor: `~/.cursor/mcp.json`
- Codex: `~/.codex/mcp.json`
- Windsurf: `~/.windsurf/mcp.json`
- CodeBuddy: `~/.codebuddy/mcp.json`
- Trae: `~/.trae/mcp.json`

除上述已知助手外，`nexo init-mcp` 还会自动发现任意遵循 `~/.agent/mcp.json` + `mcpServers` 约定的配置文件，无需为新的 AI Agent 修改 nexo 源码。

### 诊断

运行以下命令检查 MCP Server 配置：

```bash
nexo doctor
```

## 快速开始

```bash
# 创建一条问题笔记
nexo create "Docker 容器启动失败" -c issues -t "docker,debug"

# 或者使用短别名
nn create "Docker 容器启动失败" -c issues -t "docker,debug"

# 列出笔记
nexo ls

# 查看笔记
nexo view issues-20260704-001

# 搜索
nexo search docker

# 统计
nexo stats
```

## 初始化笔记库（带 Git 版本管理）

```bash
nexo init --git
```

这会在当前目录创建笔记库结构，并初始化 Git 仓库。之后每次修改都会自动提交。

## 配置

全局配置文件位于 `~/.nexo/config.toml`。

```bash
nexo config set notes_dir D:\notes
nexo config set editor code
nexo config list
```

配置优先级：命令行参数 > 环境变量 > 配置文件 > 默认值。

## 目录结构

```
nexo-notes/
├── notes/
│   ├── issues/2026/07/issues-20260704-001.md
│   ├── articles/2026/07/articles-20260704-001.md
│   └── ...
├── archive/
├── .nexo/
│   ├── config.toml
│   └── history/
│       └── issues-20260704-001/
│           └── 20260704143000.md
└── .git/
```

## Web UI

`nexo ui` 启动本地 Web 界面（Next.js），提供比命令行更顺手的浏览与维护体验。整体采用三栏布局与暖色纸张 + 青绿强调的编辑风视觉（含明暗主题）。

### 功能

- **三栏布局**：侧边栏（分类 / 标签 / 搜索） + 笔记列表 + 笔记详情，桌面端点击列表就地打开详情，移动端自动路由跳转
- **分类与标签筛选**：侧边栏选择分类或标签后，筛选上下文在详情页与笔记链页之间持久保留（Notion 风格）
- **双视图切换**：列表头部可在「列表 / 表格」之间切换；表格视图以「标题 / 分类 / 标签 / 时间」列式呈现同一分类下的笔记，类 Notion 数据库，点击行打开详情
- **笔记链页面**：每篇笔记可进入专属的 `/thread/[id]` 页面，以垂直树状图展示与其相关的笔记脉络；点击链中节点就地重新聚焦该节点的链
- **已有笔记关联**：早期未自动关联的孤立笔记可用 `nexo relink` 批量挂接到各自日期的日志；也可用 `nexo link <id> --parent <pid>` 把某篇笔记手动挂到指定父笔记下
- **中文界面**：分类标签与主要文案均已中文化

### 本地运行

```bash
cd ui
npm install
npm run dev
```

## 开发

```bash
# 编译
cargo build

# 测试
cargo test

# 运行
nexo --help
```

## 命令清单

| 命令 | 说明 |
|------|------|
| `nexo init [--git]` | 初始化笔记库 |
| `nexo init-mcp [--all] [--agent] [--project]` | 配置 MCP Server for AI Agents |
| `nexo doctor` | 检查 MCP Server 配置 |
| `nexo serve` | 启动 MCP Server（for AI agent integration） |
| `nexo ui` | 启动本地 Web 界面（Next.js 仪表盘） |
| `nexo migrate` | 迁移现有 .md 笔记到 SQLite 数据库 |
| `nexo create <title> -c <cat> [-t tags] [-s url]` | 创建笔记 |
| `nexo edit <id> [-e editor]` | 编辑笔记 |
| `nexo view <id>` | 查看笔记 |
| `nexo ls [-c] [-t] [-s] [--limit]` | 列出笔记 |
| `nexo search <keyword> [-t tags]` | 搜索笔记 |
| `nexo thread <id>` | 查看某条笔记的完整笔记链 |
| `nexo link <id> [--parent <pid>]` | 将已有笔记挂接到链中（设置父节点）；不传 `--parent` 则关联到其创建日期的日志 |
| `nexo relink [--dry-run]` | 批量关联所有无父节点的孤立笔记到各自日期的日志 |
| `nexo archive <id>` | 归档笔记 |
| `nexo rm <id> [-f]` | 删除/归档笔记 |
| `nexo tag ls` | 列出标签 |
| `nexo tag mv <old> <new>` | 重命名标签 |
| `nexo tag suggest <id>` | 推荐标签 |
| `nexo config set/get/list` | 配置管理 |
| `nexo stats` | 统计信息 |
| `nexo completions <shell>` | 生成 shell 补全 |
