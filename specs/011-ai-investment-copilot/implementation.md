# Leek Fund AI 投资驾驶舱首版实施说明

**版本**：0.1  
**日期**：2026-07-23  
**对应产品文档**：`specs/011-ai-investment-copilot/product.md`

## 1. 首版目标

首版将原有 AI 对话页升级为基于真实持仓快照的投资驾驶舱。产品先回答三个高频问题：

1. 今天组合发生了什么。
2. 哪些资产对组合影响最大。
3. 当前结论基于哪一份数据，哪些数据仍不完整。

本版本不让模型计算资产金额、涨跌和集中度。Rust 负责确定性计算，模型只读取固定快照并解释结果。

## 2. 已实现范围

### 2.1 今日视图

- 聚合基金、股票、加密货币和黄金持仓。
- 展示当前总市值、今日变化、涨跌覆盖率和行情覆盖率。
- 展示资产类别配置、最大影响项和 30% 单项集中度提醒。
- 显示数据缺口；行情失败时保留持仓并回退到用户录入成本金额。
- 普通进入复用五分钟内快照，手动刷新始终生成新快照。

### 2.2 组合视图

- 展示最大单项占比、前五项占比和行情完整度。
- 按当前市值展示逐项持仓、组合占比和今日影响。
- 支持从持仓或风险提示生成快捷问题。

### 2.3 固定快照问答

- 每次提问携带当前 `snapshot_id`，后端按 ID 读取不可变快照。
- 用户消息保存快照 ID 和上下文 JSON。
- 后端支持 `portfolio`、`asset_codes` 和 `report` 上下文协议；首版 UI 只开放全组合。
- 回答流包含上下文、文本块、保存状态、错误和结束事件。
- 用户可中止当前流；页面卸载时也会取消请求。
- 模型失败时，后端基于同一快照生成并保存本地确定性摘要。

### 2.4 模型连接

- 支持 OpenAI、Claude 和 OpenAI 兼容接口。
- 默认配置为本机 Ollama：`http://127.0.0.1:11434/v1`，允许不填 Key。
- 设置弹窗支持提供商、地址、模型、Key、最大输出和温度。
- 连接测试和保存均可复用同一提供商已保存的 Key。
- 配置保存后立即更新运行时，不要求重启应用。
- 配置读取接口只返回 `has_api_key`，不回传 Key 明文。

### 2.5 证据与隐私

- 证据栏展示快照时间、快照编号、资产范围、行情覆盖和数据缺口。
- 本机模型获得完整快照上下文。
- 非本机模型默认只获得占比、涨跌和贡献率，不发送精确持仓金额与总金额。
- 每次模型请求写入本地出站字段审计，不记录 Key 和消息正文。
- 本地 HTTP 服务只绑定 `127.0.0.1`，CORS 仅允许 Tauri 与本地 Vite 来源。

## 3. 数据口径

### 3.1 当前市值

`current_value = holding_quantity * current_price`

有有效份额和行情时使用当前市值；否则回退到用户录入的 `cost_amount`，并把估值依据标记为 `cost_fallback`。

### 3.2 今日变化

行情涨跌幅是相对昨收的变化率，因此：

`previous_value = current_value / (1 + change_percent)`

`daily_change_amount = current_value - previous_value`

涨跌幅缺失、行情缺失或涨跌幅小于等于 -100% 时不计算该项今日变化。

### 3.3 组合涨跌率

组合涨跌率只使用有今日变化数据的资产：

`portfolio_change_percent = covered_change / covered_previous_value`

同时展示 `daily_change_coverage_percent`，避免把无行情资产错误视作零涨跌。

### 3.4 集中度

- 最大单项占比：最大单项当前市值 / 组合当前总市值。
- 前五项占比：当前市值最大的五项之和 / 组合当前总市值。
- 首版提示阈值：最大单项占比大于等于 30%。

同一基金存在于多个分组时，首版按分组仓位分别计算；后续需要明确是否按证券代码合并。

## 4. 技术结构

### 4.1 前端

- `AiCopilotPanel`：页面编排、视图切换、聊天和取消。
- `TodayBriefing`：今日简报。
- `PortfolioOverview`：组合体检和持仓贡献。
- `EvidencePanel`：固定快照与数据质量。
- `ModelSettingsDialog`：模型配置与连接测试。
- `ai-copilot.ts`：本地 HTTP API 和 SSE 客户端。

### 4.2 Rust

- `portfolio_snapshot.rs`：多资产采集、指标计算和不可变快照。
- `llm_client.rs`：三类模型协议、SSE UTF-8 分片解析和配置持久化。
- `api/stream.rs`：固定上下文、脱敏、审计、流式回复和本地降级。
- `http_server.rs`：会话、模型设置和流式路由。
- `003_ai_copilot.sql`：快照、报告、结论、消息上下文和审计表。

### 4.3 入口

Tauri 命令：

- `get_portfolio_snapshot`
- `refresh_portfolio_snapshot`

本地 HTTP：

- `GET/POST /api/sessions`
- `GET /api/sessions/:id/messages`
- `POST /api/sessions/:id/messages/stream`
- `GET/POST /api/llm/config`
- `POST /api/llm/test`

## 5. 降级策略

- 单个行情接口失败：继续创建快照，回退成本金额并登记数据缺口。
- 模型连接失败：今日与组合视图继续可用，聊天输出同一快照的本地摘要。
- 回答保存失败：前端显示“本条回复未保存”。
- 用户停止生成：取消 SSE；已显示内容保留在当前页面，但未完成回答不保证持久化。
- 旧数据库升级：先创建 P0 表，再补消息上下文字段，最后创建字段索引。

## 6. 当前缺口

以下能力仍属于后续迭代，不能视为已完成：

- API Key 尚未接入 macOS Keychain、Windows Credential Manager 等系统安全存储，目前保存在应用数据目录的 `llm_config.json`。
- 日报、周报和报告历史只有数据库表结构，没有生成、保存、查询和重开入口。
- 前端尚未开放单资产、报告上下文选择器。
- 回答仍为自由文本，没有完整渲染 `findings/evidence/unknowns/confidence/actions` 协议。
- 没有资讯源、公告源、基准指数和宏观事件归因。
- 没有失败回答的一键重试和跨会话管理。
- 本地 HTTP 服务没有进程级鉴权令牌；同机进程仍可直接请求 `127.0.0.1:18188`。
- 未使用真实云模型凭据或本地 Ollama 完成联网端到端验证。

## 7. 验证命令

```bash
env PATH=/Users/hitol/.nvm/versions/node/v22.22.0/bin:/usr/bin:/bin npm run build
cd src-tauri && cargo test
cd src-tauri && cargo check
git diff --check
```

具体覆盖与人工验收项见 `specs/011-ai-investment-copilot/test-cases.md`。
