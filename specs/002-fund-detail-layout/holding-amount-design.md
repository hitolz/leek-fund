# Design Doc / 设计文档: Fund Holding Amount in Right Detail Panel

**Feature**: Add a holding amount input in the right fund detail panel.  
**功能**：在右侧基金详情面板新增“持仓金额”输入功能。  
**Status**: Draft  
**状态**：草稿  
**Owner**: TBD  
**负责人**：待定  
**Last Updated**: 2026-01-21  
**最后更新**：2026-01-21

## 1. Background / 背景

The right column currently focuses on fund details and trend visualization; users also need a place to record their holding amount for the selected fund.  
右侧列目前聚焦基金详情与走势展示；用户还需要记录该基金的持仓金额。

This document designs a minimal, reliable holding-amount feature that fits the existing three-column flow.  
本文档设计一个最小、可靠且与三列流程匹配的持仓金额功能。

## 2. Reference & Research / 参考与调研

Reference source: https://github.com/LeekHub/leek-fund.  
参考来源：https://github.com/LeekHub/leek-fund。

本地 /tmp/leek-fund-github

Observed behavior (from `template/fund-amount.html` + `src/webview/setAmount.ts`):  
观察到的行为（来自 `template/fund-amount.html` + `src/webview/setAmount.ts`）：

- Entry point is a "设置持仓" button in the fund info header; it opens a dedicated webview to edit positions for all funds.  
  入口是基金信息头部的“设置持仓”按钮；打开独立页面批量编辑持仓。  
- Inputs are **cost price** and **shares**, not raw amount; holding amount is computed as `shares * latest net value (yestclose)`.  
  输入项为**成本价**和**持仓份额**，而非直接金额；持仓金额由 `份额 × 最新净值` 计算。  
- Shares are formatted to 2 decimals on blur; input length is capped (12 chars).  
  份额失焦时保留 2 位小数；输入长度限制为 12 位。  
- The UI shows per-fund computed amount inline and a total holding amount at the top.  
  列表中实时展示单只基金的计算金额，并在顶部显示总持仓。  
- Save persists a per-fund config map (`fundAmount`) keyed by fund code with fields like `amount`, `shares`, `unitPrice`.  
  保存后以基金代码为键持久化配置（如 `amount`、`shares`、`unitPrice`）。  

Relevance to this feature: we should keep the **computed amount** approach to avoid desync with net value and enable future P/L metrics.  
对本功能的启发：应保持“计算金额”的思路，避免与净值脱节，并为后续盈亏扩展留空间。

## 3. Goals / 目标

- Allow users to set a holding amount for the selected fund from the right detail panel.  
  允许用户在右侧详情面板为当前基金设置持仓金额。  
- Persist the holding amount locally and load it on selection.  
  持仓金额本地持久化，并在选中基金时加载。  
- Keep the interaction lightweight and consistent with the existing detail UI.  
  交互轻量并与现有详情界面保持一致。

## 4. Non-Goals / 非目标

- Portfolio performance analytics (e.g., total profit/loss) are out of scope for this iteration.  
  组合层面的收益分析（如总盈亏）不在本次范围内。  
- Multi-currency support is out of scope; default currency is CNY.  
  多币种支持不在本次范围内；默认币种为人民币。  
- Complex trade history (buy/sell lots) is out of scope.  
  复杂的买卖流水（分笔持仓）不在本次范围内。

## 5. UX Design / 交互设计

### 5.1 Entry Point / 入口

In the right detail panel, add a "Holding Amount" row near the top summary area (above the trend chart).  
在右侧详情面板顶部摘要区域新增“持仓金额”一行（在走势图上方）。

### 5.2 Interaction / 交互

Default state shows a formatted holding amount or a placeholder ("Not set").  
默认状态展示格式化持仓金额或占位文本（“未设置”）。

Clicking "Edit" expands a compact editor with **cost price** and **shares** fields; the amount is computed and displayed read-only.  
点击“编辑”展开紧凑编辑器，包含**成本价**与**持仓份额**输入；金额为只读计算结果。

Saving persists immediately and returns to display mode.  
保存后立即持久化并返回展示态。

If the user clicks away, prompt to save or discard.  
点击空白处时提示保存或放弃。

### 5.3 Validation / 校验

- Cost price: positive number, up to 4 decimals (to match fund net value precision).  
  成本价：正数，最多 4 位小数（匹配净值精度）。  
- Shares: positive number, up to 2 decimals (matches plugin behavior).  
  持仓份额：正数，最多 2 位小数（对齐插件行为）。  
- Empty input clears the position after confirmation.  
  空输入在确认后清空持仓。  
- Display errors inline when the input is invalid.  
  无效输入时在输入框旁展示错误提示。

## 6. Data Model / 数据模型

### 6.1 New Entity / 新增实体

Add a `FundPosition` record keyed by `fund_code` with `holding_amount`, `shares`, and `unit_price`.  
新增 `FundPosition` 记录，以 `fund_code` 为键，包含 `holding_amount`、`shares`、`unit_price`。

Example (JSON):  
示例（JSON）：

```json
{
  "fund_code": "000001",
  "holding_amount": 12000.50,
  "shares": 1000.00,
  "unit_price": 1.20,
  "updated_at": 1737427200
}
```

### 6.2 Storage Format Update / 存储格式更新

Extend `StorageFormat` with a `positions` map of `FundPosition` keyed by fund code.  
为 `StorageFormat` 增加以基金代码为键的 `positions` map。

Backward compatibility: if `positions` is missing, treat as empty.  
向后兼容：缺少 `positions` 时视为空。

Migration: on load, if `positions` missing, initialize to empty and update `last_modified` only upon first write.  
迁移策略：加载时若无 `positions`，初始化为空，并在首次写入时更新 `last_modified`。

## 7. Tauri Commands / Tauri 命令

Add commands for reading and writing positions (amount is derived).  
新增读取与写入持仓信息的命令（金额为派生值）。

Proposed API:  
建议 API：

```ts
get_fund_position(fundCode: string) -> FundPosition | null
set_fund_position(fundCode: string, shares: number | null, unitPrice: number | null) -> FundPosition | null
```

If `shares` is `null`, delete the record.  
当 `shares` 为 `null` 时删除记录。

## 8. UI State & Rendering / UI 状态与渲染

When a fund is selected, load its position and compute the holding amount using latest net value.  
当基金被选中时加载持仓信息，并用最新净值计算持仓金额。

If a fund has no position, show the placeholder and an edit affordance.  
若无持仓信息，显示占位文本并提供可编辑入口。

Local optimistic update is allowed, but the UI must reconcile with persisted state on success/failure.  
允许本地乐观更新，但保存成功/失败后需与持久化结果一致。

## 9. Edge Cases / 边界情况

- Fund is deleted from all lists: keep the position record or cleanup on demand (decision needed).  
  基金从所有列表删除：保留持仓记录或按需清理（需要决策）。  
- Invalid or NaN inputs must never be persisted.  
  无效或 NaN 输入不得持久化。  
- Concurrent updates from multiple windows (if supported) should use last-write-wins.  
  多窗口并发更新（如支持）采用最后写入覆盖。

## 10. Open Questions / 待确认问题

- What is the exact interaction in the GitHub leek fund plugin for edit/save/cancel?  
  GitHub leek fund 插件的编辑/保存/取消交互到底是什么？  
- Should positions be per list or global per fund code?  
  持仓信息是按列表维度还是按基金代码全局维度？  
- Do we need derived metrics (e.g., estimated P/L) in the right panel?  
  右侧是否需要衍生指标（如预估盈亏）？

## 11. Success Criteria / 成功标准

- Users can set, update, and clear a holding amount within the fund detail panel.  
  用户可在基金详情面板内设置、更新与清空持仓金额。  
- Value persists across app restarts and is loaded on fund selection.  
  数值可跨重启保存，并在选中基金时加载。  
- Invalid inputs are rejected with clear feedback.  
  无效输入被拒绝并给出清晰提示。
