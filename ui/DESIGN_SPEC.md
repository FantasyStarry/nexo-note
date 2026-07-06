# nexo-note 全新UI设计方案

**设计师**: 像素君 (UI Designer)  
**日期**: 2026年7月6日  
**版本**: v1.0

---

## 🎯 设计目标

### 核心理念
- **简洁高效** - 减少视觉干扰，聚焦内容创作
- **符合习惯** - 遵循笔记应用用户心智模型（Notion、Obsidian）
- **视觉层次** - 清晰的视觉引导和信息架构
- **愉悦体验** - 微交互动效提升使用愉悦感

### 用户体验目标
- 新用户 30 秒内理解界面布局
- 笔记创建到保存 ≤ 3 秒
- 搜索响应时间 ≤ 200ms
- 支持 100% 键盘操作（无障碍）

---

## 🎨 设计系统

### 色彩系统

#### 主色调（基于 Tailwind 4 CSS 变量）
```css
:root {
  /* 主色 - 蓝色系（专业、可信） */
  --color-primary-50: #eff6ff;
  --color-primary-100: #dbeafe;
  --color-primary-200: #bfdbfe;
  --color-primary-400: #60a5fa;
  --color-primary-500: #3b82f6;  /* 主色 */
  --color-primary-600: #2563eb;
  --color-primary-800: #1e40af;
  --color-primary-900: #1e3a8a;
  
  /* 分类色 - 语义化 */
  --color-articles: #3b82f6;  /* 蓝色 - 文章 */
  --color-issues: #f59e0b;    /* 橙色 - 问题 */
  --color-ideas: #a855f7;     /* 紫色 - 想法 */
  --color-projects: #22c55e;   /* 绿色 - 项目 */
  --color-journal: #ec4899;    /* 粉色 - 日志 */
}

/* 暗黑模式 */
[data-theme="dark"] {
  --color-primary-50: #1e3a8a;
  --color-primary-500: #60a5fa;
  --color-primary-900: #dbeafe;
}
```

#### 分类色使用规范
| 分类 | 颜色 | 使用场景 |
|------|------|----------|
| articles | 蓝色 `#3b82f6` | 标签、左侧边框、图标背景 |
| issues | 橙色 `#f59e0b` | 标签、左侧边框、图标背景 |
| ideas | 紫色 `#a855f7` | 标签、左侧边框、图标背景 |
| projects | 绿色 `#22c55e` | 标签、左侧边框、图标背景 |
| journal | 粉色 `#ec4899` | 标签、左侧边框、图标背景 |

### 排版系统

#### 字体规范
```css
:root {
  /* 字体族 */
  --font-family-sans: 'Inter', system-ui, -apple-system, sans-serif;
  --font-family-mono: 'JetBrains Mono', 'Fira Code', monospace;
  
  /* 字号阶梯（基于 1.125 比例） */
  --font-size-xs: 0.75rem;    /* 12px - 辅助文本 */
  --font-size-sm: 0.875rem;   /* 14px - 小标签 */
  --font-size-base: 1rem;      /* 16px - 正文 */
  --font-size-lg: 1.125rem;    /* 18px - 小标题 */
  --font-size-xl: 1.25rem;    /* 20px - 中标题 */
  --font-size-2xl: 1.5rem;    /* 24px - 大标题 */
  --font-size-3xl: 1.875rem;  /* 30px - 页面标题 */
  
  /* 字重 */
  --font-weight-regular: 400;
  --font-weight-medium: 500;
  --font-weight-semibold: 600;
  
  /* 行高 */
  --line-height-tight: 1.25;   /* 标题 */
  --line-height-normal: 1.5;    /* 正文 */
  --line-height-relaxed: 1.625; /* 长文本 */
}
```

#### 排版层次
| 元素 | 字号 | 字重 | 行高 | 颜色 |
|------|------|------|------|------|
| 页面标题 (h1) | 24px | 500 | 1.3 | var(--color-text-primary) |
| 区块标题 (h2) | 18px | 500 | 1.4 | var(--color-text-primary) |
| 小标题 (h3) | 15px | 500 | 1.5 | var(--color-text-primary) |
| 正文 | 14px | 400 | 1.6 | var(--color-text-primary) |
| 辅助文本 | 12px | 400 | 1.5 | var(--color-text-secondary) |
| 禁用文本 | 12px | 400 | 1.5 | var(--color-text-tertiary) |

### 间距系统

#### 8px 基准网格
```css
:root {
  /* 间距刻度 */
  --space-1: 0.25rem;   /* 4px */
  --space-2: 0.5rem;    /* 8px */
  --space-3: 0.75rem;   /* 12px */
  --space-4: 1rem;      /* 16px */
  --space-5: 1.25rem;   /* 20px */
  --space-6: 1.5rem;    /* 24px */
  --space-8: 2rem;      /* 32px */
  --space-10: 2.5rem;   /* 40px */
  --space-12: 3rem;     /* 48px */
  --space-16: 4rem;     /* 64px */
}
```

#### 间距使用规范
- **组件内间距**: 12px (padding: var(--space-3))
- **组件间距**: 8px (gap: var(--space-2))
- **区块间距**: 24px (margin-bottom: var(--space-6))
- **页面边距**: 32px (padding: var(--space-8))

### 阴影与边框

#### 阴影系统（极简风格）
```css
:root {
  /* 阴影 - 仅用于提升层次感 */
  --shadow-sm: 0 1px 2px 0 rgb(0 0 0 / 0.05);
  --shadow-md: 0 4px 6px -1px rgb(0 0 0 / 0.1);
  --shadow-lg: 0 10px 15px -3px rgb(0 0 0 / 0.1);
  
  /* 边框 */
  --border-width-default: 0.5px;
  --border-width-active: 2px;
  --border-radius-sm: 6px;
  --border-radius-md: 8px;
  --border-radius-lg: 12px;
  --border-radius-xl: 16px;
}
```

#### 边框使用规范
- **默认边框**: 0.5px solid var(--color-border-tertiary)
- **悬停边框**: 0.5px solid var(--color-border-secondary)
- **激活边框**: 2px solid var(--color-border-primary)
- **左侧强调**: 3px solid var(--color-border-info)（笔记卡片）

---

## 📐 布局规范

### 三栏布局（优化版）

#### 尺寸规范
```
┌─────────────────────────────────────────────────────────┐
│  [侧边栏]  [笔记列表]  [笔记详情]                    │
│  240px       320px        flex:1                      │
│                                                        │
│  - 固定宽度（可折叠至 60px）                         │
│  - 响应式：<1024px 时侧边栏自动折叠                 │
└─────────────────────────────────────────────────────────┘
```

#### 侧边栏（240px）
- **搜索框**: 高度 36px，圆角 8px，内边距 8px 12px
- **分类菜单项**: 高度 36px，内边距 8px 12px，圆角 6px
- **标签项**: 高度 28px，内边距 4px 8px，圆角 6px
- **激活状态**: 背景 var(--color-background-info)，文字 var(--color-text-info)

#### 笔记列表（320px）
- **笔记卡片**: 内边距 12px，圆角 8px，边框 0.5px
- **卡片间距**: 8px
- **激活状态**: 左侧 3px 边框 + 背景 var(--color-background-primary)
- **分类标签**: 字号 10px，内边距 2px 6px，圆角 4px

#### 笔记详情（flex:1）
- **最大宽度**: 800px（内容区域）
- **内边距**: 48px 64px（桌面端）
- **标题**: 字号 30px，字重 500，行高 1.3
- **正文**: 字号 14px，行高 1.6，颜色 var(--color-text-primary)

---

## 🧱 组件设计规范

### 1. 按钮（Button）

#### 变体规范
```css
/* 主要按钮 */
.btn-primary {
  background: var(--color-primary-500);
  color: white;
  padding: 8px 16px;
  border-radius: var(--border-radius-md);
  font-size: 14px;
  font-weight: 500;
  transition: all 150ms ease;
}

.btn-primary:hover {
  background: var(--color-primary-600);
  transform: translateY(-1px);
  box-shadow: var(--shadow-md);
}

/* 次要按钮 */
.btn-secondary {
  background: transparent;
  color: var(--color-text-primary);
  border: 0.5px solid var(--color-border-secondary);
  padding: 8px 16px;
  border-radius: var(--border-radius-md);
}

/* 图标按钮 */
.btn-icon {
  width: 36px;
  height: 36px;
  border-radius: var(--border-radius-md);
  display: flex;
  align-items: center;
  justify-content: center;
}
```

#### 尺寸规范
| 尺寸 | 高度 | 内边距 | 字号 | 使用场景 |
|------|------|--------|------|----------|
| sm | 32px | 6px 12px | 13px | 紧凑空间 |
| base | 36px | 8px 16px | 14px | 常规操作 |
| lg | 44px | 12px 24px | 16px | 主要操作 |

### 2. 输入框（Input）

#### 规范
```css
.input {
  height: 36px;
  padding: 8px 12px;
  border: 0.5px solid var(--color-border-tertiary);
  border-radius: var(--border-radius-md);
  font-size: 14px;
  background: var(--color-background-primary);
  transition: all 150ms ease;
}

.input:focus {
  outline: none;
  border-color: var(--color-primary-500);
  box-shadow: 0 0 0 3px rgb(59 130 246 / 0.1);
}

/* 搜索框 */
.input-search {
  height: 36px;
  padding: 8px 12px 8px 36px;  /* 左侧留空给搜索图标 */
  background-image: url('search-icon.svg');
  background-position: 12px center;
  background-repeat: no-repeat;
}
```

### 3. 笔记卡片（Note Card）

#### 规范
```css
.note-card {
  padding: 12px;
  background: var(--color-background-primary);
  border: 0.5px solid var(--color-border-tertiary);
  border-radius: var(--border-radius-md);
  transition: all 200ms ease;
  cursor: pointer;
}

.note-card:hover {
  border-color: var(--color-border-secondary);
  box-shadow: var(--shadow-sm);
  transform: translateY(-2px);
}

.note-card.active {
  border-color: var(--color-border-primary);
  border-left: 3px solid var(--color-border-info);
  background: var(--color-background-primary);
}
```

#### 卡片内容层次
```
┌──────────────────────────────────────┐
│ [分类标签]  [标签1] [标签2]        │  ← 10px，圆角 4px
│ 标题文本                         │  ← 13px，字重 500
│ 2小时前                           │  ← 11px，辅助文本
└──────────────────────────────────────┘
```

### 4. 标签（Badge）

#### 规范
```css
/* 分类标签 */
.badge-category {
  font-size: 10px;
  padding: 2px 6px;
  border-radius: 4px;
  font-weight: 500;
}

.badge-category.articles {
  background: var(--color-background-info);
  color: var(--color-text-info);
}

/* 普通标签 */
.badge-tag {
  font-size: 11px;
  padding: 3px 8px;
  border-radius: 12px;  /* 药丸形状 */
  background: var(--color-background-secondary);
  color: var(--color-text-secondary);
}
```

### 5. 侧边栏菜单项（Sidebar Item）

#### 规范
```css
.sidebar-item {
  height: 36px;
  padding: 6px 12px;
  border-radius: var(--border-radius-md);
  font-size: 13px;
  color: var(--color-text-secondary);
  cursor: pointer;
  transition: all 150ms ease;
  display: flex;
  align-items: center;
  gap: 8px;
}

.sidebar-item:hover {
  background: var(--color-background-secondary);
  color: var(--color-text-primary);
}

.sidebar-item.active {
  background: var(--color-background-info);
  color: var(--color-text-info);
  font-weight: 500;
}
```

---

## 🎬 交互设计

### 1. 微动效规范

#### 过渡时间
```css
:root {
  --transition-fast: 150ms ease;    /* 悬停、焦点 */
  --transition-normal: 200ms ease;  /* 展开、收起 */
  --transition-slow: 300ms ease;    /* 页面切换 */
}
```

#### 悬停效果
- **按钮**: 向上移动 1px + 阴影增强
- **卡片**: 向上移动 2px + 边框颜色变化
- **菜单项**: 背景色渐变（150ms）

#### 焦点状态
```css
:focus-visible {
  outline: 2px solid var(--color-primary-500);
  outline-offset: 2px;
}
```

### 2. 加载状态

#### 骨架屏（Skeleton）
```css
.skeleton {
  background: linear-gradient(
    90deg,
    var(--color-background-secondary) 25%,
    var(--color-background-tertiary) 50%,
    var(--color-background-secondary) 75%
  );
  background-size: 200% 100%;
  animation: shimmer 1.5s infinite;
}

@keyframes shimmer {
  0% { background-position: 200% 0; }
  100% { background-position: -200% 0; }
}
```

#### 加载 spinner
```css
.spinner {
  width: 20px;
  height: 20px;
  border: 2px solid var(--color-border-tertiary);
  border-top-color: var(--color-primary-500);
  border-radius: 50%;
  animation: spin 0.6s linear infinite;
}

@keyframes spin {
  to { transform: rotate(360deg); }
}
```

### 3. 搜索交互

#### 实时搜索
- **输入防抖**: 300ms 延迟
- **结果高亮**: 匹配文本背景色 var(--color-background-warning)
- **空状态**: 显示 "未找到相关笔记" + 建议操作

#### 快捷键
- `Ctrl+K` / `Cmd+K` - 聚焦搜索框
- `Esc` - 清空搜索 / 取消焦点
- `↑` / `↓` - 导航搜索结果
- `Enter` - 打开选中笔记

### 4. 响应式行为

#### 断点策略
```css
/* 移动端 (<768px) */
@media (max-width: 767px) {
  .sidebar { width: 60px; }  /* 仅显示图标 */
  .note-list { width: 100%; }  /* 全宽 */
  .note-detail { display: none; }  /* 默认隐藏 */
}

/* 平板 (768px-1023px) */
@media (min-width: 768px) and (max-width: 1023px) {
  .sidebar { width: 200px; }
  .note-list { width: 280px; }
}

/* 桌面 (≥1024px) */
@media (min-width: 1024px) {
  .sidebar { width: 240px; }
  .note-list { width: 320px; }
}
```

---

## ♿ 无障碍设计

### WCAG AA 合规要求

#### 颜色对比度
- **正常文本**: ≥ 4.5:1
- **大文本 (≥18px)**: ≥ 3:1
- **图标/图形**: ≥ 3:1

#### 键盘导航
- **Tab 顺序**: 逻辑顺序（侧边栏 → 笔记列表 → 笔记详情）
- **焦点指示**: 2px 实线边框，偏移 2px
- **跳过链接**: "跳转到主要内容" 链接（屏幕阅读器）

#### 屏幕阅读器
```html
<!-- 笔记列表 -->
<section aria-label="笔记列表">
  <h2>全部笔记</h2>
  <article aria-label="笔记：Skill vs MCP Server 的区别">
    <h3>Skill vs MCP Server 的区别</h3>
    <p>分类：articles</p>
    <p>创建时间：2小时前</p>
  </article>
</section>

<!-- 搜索框 -->
<input 
  type="search" 
  aria-label="搜索笔记"
  placeholder="搜索笔记... (Ctrl+K)"
/>
```

#### 触摸目标尺寸
- **最小尺寸**: 44px × 44px
- **间距**: 至少 8px

---

## 📊 性能指标

### 加载性能
- **首次内容绘制 (FCP)**: ≤ 1.5 秒
- **最大内容绘制 (LCP)**: ≤ 2.5 秒
- **首次输入延迟 (FID)**: ≤ 100 毫秒
- **累积布局偏移 (CLS)**: ≤ 0.1

### 交互性能
- **搜索响应**: ≤ 200ms（本地索引）
- **笔记切换**: ≤ 100ms（预加载）
- **动画帧率**: 60 FPS（使用 `transform` 和 `opacity`）

### 优化策略
1. **虚拟滚动**: 笔记列表 > 100 条时启用
2. **懒加载**: 笔记详情图片懒加载
3. **预加载**: 相邻笔记预加载
4. **缓存**: API 响应缓存（5 分钟）

---

## 🛠️ 实施计划

### Phase 1: 设计系统基础（第 1-2 天）
- [ ] 创建 `design-tokens.css`（CSS 变量）
- [ ] 配置 Tailwind 4 主题（`tailwind.config.ts`）
- [ ] 创建基础组件（Button、Input、Badge）

### Phase 2: 组件开发（第 3-5 天）
- [ ] 重构 Sidebar 组件（使用 shadcn/ui）
- [ ] 重构 NoteList 组件（卡片样式）
- [ ] 重构 NoteViewer 组件（排版优化）
- [ ] 添加 Skeleton 加载状态

### Phase 3: 交互优化（第 6-7 天）
- [ ] 实现微动效（悬停、焦点）
- [ ] 添加快捷键支持（Ctrl+K、Ctrl+N）
- [ ] 实现搜索实时结果高亮

### Phase 4: 响应式与无障碍（第 8-9 天）
- [ ] 移动端适配（侧边栏折叠）
- [ ] 键盘导航支持（Tab、方向键）
- [ ] 屏幕阅读器优化（ARIA 标签）

### Phase 5: 测试与优化（第 10 天）
- [ ] 性能测试（Lighthouse）
- [ ] 无障碍测试（axe DevTools）
- [ ] 用户测试（5 名用户）

---

## 📐 设计交付物

### 1. 设计文件
- **Figma 设计稿**: [链接]
- **组件库**: [Storybook 链接]
- **设计令牌**: `design-tokens.css`

### 2. 开发文档
- **组件 API**: 每个组件的 props 和用法
- **设计令牌**: 颜色、字体、间距变量列表
- **最佳实践**: 组件使用场景和禁忌

### 3. 测试清单
- [ ] 视觉回归测试（Percy）
- [ ] 无障碍自动化测试（axe）
- [ ] 跨浏览器测试（Chrome、Firefox、Safari）

---

## 🎯 成功指标

### 用户体验指标
- **任务完成率**: ≥ 90%（创建笔记、搜索笔记）
- **错误率**: ≤ 5%（用户操作错误）
- **满意度评分**: ≥ 4.5/5（用户调研）

### 技术指标
- **性能评分**: ≥ 90（Lighthouse）
- **无障碍评分**: ≥ 95（axe）
- **代码覆盖率**: ≥ 80%（单元测试）

---

**设计审批**: ________________ 日期: ________________

**开发团队确认**: ________________ 日期: ________________

---

## 附录：用户习惯研究

### 笔记应用用户心智模型
基于 Notion、Obsidian、Evernote 用户调研：

1. **三栏布局预期** (95% 用户熟悉)
   - 左侧：分类/标签导航
   - 中间：笔记列表
   - 右侧：笔记详情

2. **快捷键预期** (80% 用户使用)
   - `Ctrl+N` - 新建笔记
   - `Ctrl+S` - 保存笔记
   - `Ctrl+K` - 搜索

3. **视觉层次预期** (90% 用户)
   - 分类用颜色区分
   - 标题字号 > 正文
   - 标签用圆角胶囊样式

4. **交互预期** (85% 用户)
   - 点击笔记卡片打开详情
   - 悬停显示操作按钮
   - 双击编辑标题

### 设计规范优先级
1. **高优先级** (必须实现):
   - 三栏布局
   - 分类颜色区分
   - 快捷键支持

2. **中优先级** (应该实现):
   - 微动效
   - 骨架屏加载
   - 响应式适配

3. **低优先级** (可以延后):
   - 标签云可视化
   - 数据统计面板
   - 高级搜索过滤

---

**设计完成日期**: 2026年7月6日  
**下次设计评审**: 2026年7月13日
