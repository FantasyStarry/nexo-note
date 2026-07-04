# nexo-note skill 示例

## 场景 1：用户想记录一个 bug

**用户**：记一下这个 bug，React useEffect 无限循环。

**Agent 操作**：

```bash
nexo tag ls --json
nexo create "React useEffect 无限循环" -c issues -t "react,hooks,debug" --json
```

**Agent 回复**：

已创建问题笔记，ID 是 `issues-20260704-001`。需要我帮你打开编辑器补充详细复现步骤吗？

---

## 场景 2：用户想找之前的 Docker 笔记

**用户**：我之前记过 Docker 相关的东西吗？

**Agent 操作**：

```bash
nexo search "docker" --json
```

**Agent 回复**：

找到 3 篇相关笔记：
- `issues-20260615-002`：Docker 容器启动失败
- `articles-20260620-001`：Docker 最佳实践
- `ideas-20260701-001`：用 Docker 部署笔记服务

---

## 场景 3：用户想改标签

**用户**：把 js 标签改成 javascript。

**Agent 操作**：

```bash
nexo tag ls --json
nexo tag mv js javascript
```

**Agent 回复**：

已将 `js` 重命名为 `javascript`，共更新了 5 篇笔记。

---

## 场景 4：用户收藏文章

**用户**：收藏这篇文章 https://example.com/rust-errors，讲的是 Rust 错误处理。

**Agent 操作**：

```bash
nexo create "Rust 错误处理最佳实践" -c articles -t "rust,error-handling" -s "https://example.com/rust-errors" --json
```

**Agent 回复**：

已收藏文章，ID 是 `articles-20260704-001`。

---

## 场景 5：用户想删除笔记

**用户**：删掉 issues-20260704-001。

**Agent 操作**：

```bash
nexo archive issues-20260704-001
```

**Agent 回复**：

已将 `issues-20260704-001` 归档。如果确定要彻底删除，请告诉我"彻底删除"。

---

## 场景 6：用户初始化笔记库

**用户**：帮我初始化一个带 Git 版本管理的笔记库。

**Agent 操作**：

```bash
nexo init --git
```

**Agent 回复**：

已在当前目录初始化笔记库，并启用 Git 版本管理。之后每次修改都会自动提交。
