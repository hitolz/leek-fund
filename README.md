# 📈 Leek Fund - AI 投资助手

![Leek Fund](leekimage.png)

一个基于 Tauri + Rust + React 构建的跨平台桌面应用，集成了 AI 投资助手，支持基金、股票、加密货币、黄金的行情查询、持仓管理和智能分析。

## ✨ 功能特性

### 📊 多资产支持
- 🔍 **基金**: 搜索、实时估值、历史净值、持仓管理
- 📈 **股票**: A股/港股实时行情、持仓管理
- 🪙 **加密货币**: BTC/ETH 等主流币种实时行情
- 🥇 **黄金**: AU9999 实时行情

### 🤖 AI 投资助手
- 💬 **智能对话**: 基于 LLM 的流式对话，支持 Markdown 渲染
- 🔧 **Tool Calling**: AI 可调用 25+ 工具获取实时数据
- 📰 **新闻查询**: 东方财富快讯、按股票/主题搜索新闻
- 📝 **自然语言记账**: "买了5000块012733" 自动记录操作
- 📊 **定投回测**: 基于历史净值数据模拟定投收益
- ⚖️ **智能再平衡**: 检查持仓偏离度，给出再平衡建议
- 🔗 **关联性分析**: 计算持仓资产相关系数
- 🏗️ **一键建仓**: 根据风险偏好生成投资组合方案
- 😰 **市场情绪**: 综合黄金、加密货币等指标判断市场情绪
- 💰 **税务优化**: 分析持仓给出节税建议
- 🔍 **基金筛选**: 按收益率、风险等条件筛选基金
- 📅 **每日报告**: 自动生成每日投资报告
- 🎲 **蒙特卡洛模拟**: 模拟未来收益分布，计算亏损概率
- 💾 **每日行情保存**: 自动保存股票、加密货币、黄金每日行情

### 🎨 用户体验
- 🗂️ **会话管理**: 历史会话列表、`/new` 新建对话
- 📋 **复制按钮**: hover 消息即可复制原始内容
- 🔁 **自动刷新**: 行情数据周期性自动刷新
- 💾 **本地持久化**: SQLite 数据库，支持数据迁移和故障恢复

## 🚀 快速开始

### 前置要求

- Rust 1.70+ ([安装 Rust](https://www.rust-lang.org/tools/install))
- Node.js 18+ ([安装 Node.js](https://nodejs.org/))
- 操作系统: macOS, Windows, 或 Linux

### 安装依赖

```bash
npm install
```

### 开发模式运行

```bash
npm run tauri:dev
```

### 生产构建

```bash
npm run tauri:build
```

## 🛠️ AI 配置

AI 助手需要配置 LLM 服务。支持以下 Provider：

| Provider | 说明 |
|----------|------|
| `openai_compatible` | OpenAI 兼容 API（默认，支持 Ollama、OneAPI 等） |
| `openai` | OpenAI 官方 API |
| `claude` | Anthropic Claude API |
| `claude_compatible` | Claude 兼容 API |

配置文件位置：`~/.leek/llm_config.json`

```json
{
  "provider": "openai_compatible",
  "api_key": "your-api-key",
  "base_url": "https://api.openai.com/v1",
  "model": "gpt-4",
  "max_tokens": 4096,
  "temperature": 0.3
}
```

也可在应用内的模型设置对话框中配置。

## 📚 技术栈

### 后端 (Rust)
- **Tauri 1.5**: 跨平台桌面应用框架
- **reqwest**: HTTP 客户端
- **serde/serde_json**: 序列化/反序列化
- **tokio**: 异步运行时
- **SQLx + SQLite**: 本地数据库

### 前端 (React)
- **React 18**: UI 框架
- **TypeScript**: 类型安全
- **Vite**: 构建工具
- **react-markdown**: Markdown 渲染
- **SSE 流式处理**: AI 对话实时回复

## 🏗️ 项目结构

```
leek-fund/
├── src/                          # 前端源代码
│   ├── components/
│   │   ├── ai/                   # AI 投资助手组件
│   │   │   ├── ChatWorkspace.tsx # 聊天工作区
│   │   │   ├── SessionDrawer.tsx # 会话历史抽屉
│   │   │   └── ...
│   │   ├── StockPanel.tsx        # 股票面板
│   │   └── CryptoPanel.tsx       # 加密货币面板
│   ├── services/
│   │   └── ai-copilot.ts         # AI 服务（HTTP/SSE）
│   └── types/
│       └── ai-copilot.ts         # 类型定义
├── src-tauri/                    # Rust 后端
│   ├── src/
│   │   ├── api/                  # HTTP API
│   │   │   ├── stream.rs         # SSE 流式处理
│   │   │   ├── session.rs        # 会话管理
│   │   │   └── message.rs        # 消息管理
│   │   ├── services/
│   │   │   ├── llm_client.rs     # LLM 客户端（支持 OpenAI/Claude）
│   │   │   ├── chat_agent.rs     # AI Agent（Tool Calling 循环）
│   │   │   └── tool_registry.rs  # 工具注册与执行
│   │   ├── modules/
│   │   │   ├── fund_api.rs       # 基金 API
│   │   │   ├── stock_api.rs      # 股票 API
│   │   │   ├── crypto_api.rs     # 加密货币 API
│   │   │   ├── gold_api.rs       # 黄金 API
│   │   │   ├── news_api.rs       # 新闻 API
│   │   │   └── portfolio_snapshot.rs # 组合快照
│   │   └── migrations/           # 数据库迁移
│   │       ├── 001_init.sql
│   │       ├── 002_group_fund_positions.sql
│   │       ├── 003_ai_copilot.sql
│   │       ├── 004_operations.sql
│   │       └── 005_daily_history.sql
│   └── Cargo.toml
└── package.json
```

## 🗄️ 数据存储

数据库位置：`~/.leek/lists.sqlite`

### 主要数据表

| 表名 | 说明 |
|------|------|
| `funds` | 基金列表 |
| `fund_nav_daily` | 基金每日净值历史 |
| `group_fund_positions` | 基金持仓 |
| `stock_holdings` | 股票持仓 |
| `crypto_holdings` | 加密货币/黄金持仓 |
| `stock_daily_quotes` | 股票每日行情历史 |
| `crypto_daily_quotes` | 加密货币/黄金每日行情历史 |
| `daily_portfolio_snapshot` | 每日持仓快照 |
| `operations` | 投资操作记录 |
| `sessions` | AI 会话 |
| `session_chat_messages` | AI 聊天消息 |
| `portfolio_snapshots` | 组合快照 |

## 📖 AI 工具列表

AI 助手可调用以下工具：

| 工具 | 说明 |
|------|------|
| `get_stock_quote` | 获取股票实时行情 |
| `search_stock` | 搜索股票 |
| `get_fund_info` | 获取基金实时估值 |
| `get_fund_detail` | 获取基金详情 |
| `get_local_fund_profile` | 查询本地基金档案 |
| `get_crypto_quote` | 获取加密货币行情 |
| `get_gold_quote` | 获取黄金行情 |
| `get_financial_news` | 获取财经新闻（支持关键词筛选） |
| `get_stock_news` | 获取股票相关新闻 |
| `query_wencai` | 同花顺问财自然语言查询 |
| `record_operation` | 记录投资操作 |
| `query_operations` | 查询操作记录 |
| `get_operations_summary` | 操作汇总统计 |
| `backtest_dca` | 定投回测 |
| `suggest_rebalance` | 智能再平衡建议 |
| `analyze_correlation` | 关联性分析 |
| `build_portfolio` | 一键建仓方案 |
| `get_market_sentiment` | 市场情绪指标 |
| `tax_optimization` | 税务优化建议 |
| `screen_funds` | 基金筛选 |
| `generate_daily_report` | 每日投资报告 |
| `monte_carlo_simulation` | 蒙特卡洛模拟 |
| `get_portfolio_holdings` | 查看当前持仓 |
| `query_local_db` | 查询本地数据库 |
| `save_daily_snapshot` | 保存每日行情快照 |

## 🔧 开发

```bash
# 运行测试
cd src-tauri && cargo test

# 代码格式化
cd src-tauri && cargo fmt
```

## 📜 许可证

MIT License

---

**构建信息**: 
- 🦀 Rust: 核心业务逻辑与数据处理
- ⚛️ React: 用户界面
- 🔧 Tauri: 跨平台桌面框架
- 🤖 AI: Tool Calling + Agent Loop
