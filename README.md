# 📈 基金查询客户端 (Leek Fund)

一个基于 Tauri + Rust + React 构建的跨平台桌面应用，用于查询、管理中国基金信息。

## ✨ 功能特性

- 🔍 **基金搜索**: 输入6位基金代码，实时查询基金信息
- 📝 **多列表管理**: 创建、重命名、删除多个自定义基金列表
- ➕ **智能添加**: 添加基金到列表，自动去重（同一列表内）
- 💾 **数据持久化**: 所有数据自动保存到本地，应用重启后恢复
- 🎨 **现代UI**: 简洁美观的用户界面，响应式布局

## 🚀 快速开始

### 前置要求

- Rust 1.70+ ([安装 Rust](https://www.rust-lang.org/tools/install))
- Node.js 18+ ([安装 Node.js](https://nodejs.org/))
- 操作系统: macOS, Windows, 或 Linux

### 安装依赖

```bash
# 安装前端依赖
npm install

# Cargo 会自动处理 Rust 依赖
```

### 开发模式运行

```bash
npm run tauri:dev
```

这将启动前端开发服务器和 Tauri 应用窗口。前端支持热重载。

### 生产构建

```bash
npm run tauri:build
```

构建产物位置：
- **macOS**: `src-tauri/target/release/bundle/macos/`
- **Windows**: `src-tauri/target/release/bundle/msi/`
- **Linux**: `src-tauri/target/release/bundle/appimage/`

## 📚 技术栈

### 后端 (Rust)
- **Tauri 1.5**: 跨平台桌面应用框架
- **reqwest**: HTTP 客户端
- **serde/serde_json**: 序列化/反序列化
- **tokio**: 异步运行时
- **uuid**: UUID 生成
- **chrono**: 时间处理

### 前端 (React)
- **React 18**: UI 框架
- **TypeScript**: 类型安全
- **Vite**: 构建工具

## 🏗️ 项目结构

```
leek-fund/
├── src/                      # 前端源代码
│   ├── components/           # React 组件
│   ├── hooks/                # 自定义 Hooks
│   ├── types/                # TypeScript 类型
│   ├── App.tsx               # 主应用组件
│   └── main.tsx              # 入口文件
├── src-tauri/                # Rust 后端
│   ├── src/
│   │   ├── modules/          # 核心模块
│   │   │   ├── fund_api.rs   # 基金 API
│   │   │   ├── storage.rs    # 数据持久化
│   │   │   └── list_manager.rs # 列表管理
│   │   ├── models.rs         # 数据模型
│   │   ├── errors.rs         # 错误类型
│   │   ├── commands.rs       # Tauri 命令
│   │   └── main.rs           # Rust 入口
│   └── Cargo.toml            # Rust 依赖
├── specs/                    # 设计文档
└── package.json              # Node 依赖
```

## 🔧 开发指南

### 运行测试

```bash
# Rust 后端测试
cd src-tauri
cargo test

# 前端测试（如果配置）
npm test
```

### 代码格式化

```bash
# Rust
cd src-tauri
cargo fmt

# TypeScript/React
npm run format
```

### 调试

- **Rust 后端**: 使用 `println!` 或 `dbg!` 宏，输出会显示在终端
- **前端**: 使用浏览器开发工具（在 Tauri 窗口中按 `Cmd+Option+I` / `Ctrl+Shift+I`）

## 📖 API 接口

### 基金数据来源

- **基金实时数据**: `http://fundgz.1234567.com.cn/js/{code}.js`
- **基金历史走势**: `https://fund.eastmoney.com/pingzhongdata/{code}.js`
- **返回格式**: JSONP (`jsonpgz({...})`)

### Tauri 命令

- `search_fund(code)`: 搜索基金
- `get_all_lists()`: 获取所有列表
- `create_list(name)`: 创建列表
- `add_fund_to_list(list_id, fund_code)`: 添加基金
- `remove_fund_from_list(list_id, fund_code)`: 移除基金
- 更多详见 [API 契约文档](./specs/001-fund-list-management/contracts/tauri-commands.md)

## 📝 功能使用

1. **搜索基金**: 在搜索框输入6位基金代码（如 `001632`）
2. **创建列表**: 在左侧面板输入列表名称并点击"创建"
3. **添加基金**: 搜索到基金后，选择目标列表并点击"添加到列表"
4. **查看列表**: 点击左侧列表查看其中的基金详情
5. **管理列表**: 使用重命名（✏️）和删除（🗑️）按钮管理列表

## 🛠️ 故障排查

### 问题: 依赖安装失败
**解决**: 确保网络正常，尝试使用淘宝镜像：
```bash
npm config set registry https://registry.npmmirror.com
```

### 问题: Rust 编译错误
**解决**: 更新 Rust 工具链：
```bash
rustup update
```

### 问题: 数据文件损坏
**位置**: 
- macOS: `~/Library/Application Support/leek-fund/lists.json`
- Windows: `%APPDATA%/leek-fund/lists.json`
- Linux: `~/.local/share/leek-fund/lists.json`

**解决**: 删除该文件，应用会创建新的空数据

## 📄 开发文档

完整的设计和实现文档位于 `specs/001-fund-list-management/` 目录：
- [功能规格](./specs/001-fund-list-management/spec.md)
- [实现计划](./specs/001-fund-list-management/plan.md)
- [数据模型](./specs/001-fund-list-management/data-model.md)
- [任务列表](./specs/001-fund-list-management/tasks.md)

## 📊 实现进度

- [x] Phase 1: 项目初始化 (5/5 任务)
- [x] Phase 2: 数据模型与存储 (6/6 任务)
- [x] Phase 3: US1 基金搜索 (6/6 任务)
- [x] Phase 4: US2 添加到列表 (7/7 任务)
- [x] Phase 5: US3 列表管理 (8/8 任务)
- [x] Phase 6: US4 数据持久化 (3/5 任务)
- [x] Phase 7: UI 优化 (4/6 任务)

**总进度**: 39/43 任务完成 (91%)

## 🤝 贡献

欢迎贡献代码、报告问题或提出建议！

## 📜 许可证

MIT License

---

**构建信息**: 
- 🦀 Rust: 核心业务逻辑与数据处理
- ⚛️ React: 用户界面
- 🔧 Tauri: 跨平台桌面框架
