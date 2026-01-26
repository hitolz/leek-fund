# Feature Specification / 功能规格: Holding Amount in Fund Detail Panel

**Feature Branch**: `004-holding-amount`  
**功能分支**：`004-holding-amount`  
**Created**: 2026-01-21  
**创建日期**：2026-01-21  
**Status**: Draft  
**状态**：草稿  
**Input**: User description: "更新， 我想要在最右侧基金详情页里设置 持仓金额，需要关联到具体哪个组 哪个基金，设置了多少金额，并且可以计算出当天涨跌的金额"  
**输入**：用户描述：“更新， 我想要在最右侧基金详情页里设置 持仓金额，需要关联到具体哪个组 哪个基金，设置了多少金额，并且可以计算出当天涨跌的金额”

## User Scenarios & Testing *(mandatory)* / 用户场景与测试（必填）

### User Story 1 - Set Holding Amount Per Group (Priority: P1) / 用户故事 1 - 按分组设置持仓金额（优先级：P1）

As a user, I want to set my holding amount for the selected fund in the right detail panel, and it must be tied to the current group and fund.  
作为用户，我希望在右侧详情面板为当前基金设置持仓金额，并且必须与当前分组和基金绑定。

**Why this priority**: This is the core new capability and delivers immediate value even without additional analytics.  
**优先级理由**：这是新增能力的核心，即使没有额外分析也能直接产生价值。

**Independent Test**: Select a group and fund, set holding info, save, and verify the displayed holding amount updates for that group-fund pair.  
**独立测试**：选择分组与基金，设置持仓信息并保存，验证该分组-基金组合的持仓金额更新。

**Acceptance Scenarios** / **验收场景**：

1. **Given** a group and fund are selected, **When** I enter holding info and save, **Then** the right panel shows the updated holding amount for that group-fund pair.  
   **假设** 已选中分组与基金，**当** 我输入持仓信息并保存，**则** 右侧显示该分组-基金的持仓金额。
2. **Given** holding info exists for a group-fund pair, **When** I edit and save new values, **Then** the displayed holding amount reflects the new values.  
   **假设** 某分组-基金已存在持仓信息，**当** 我编辑并保存新数值，**则** 持仓金额显示为新数值。

---

### User Story 2 - Clear Holding Amount (Priority: P2) / 用户故事 2 - 清空持仓金额（优先级：P2）

As a user, I want to clear my holding info so the fund shows as not held.  
作为用户，我希望清空持仓信息，使该基金显示为未持仓。

**Why this priority**: Clearing is essential for accuracy when a position is closed.  
**优先级理由**：当清仓时，清空功能是保持准确性的关键。

**Independent Test**: Set holding info for a group-fund pair, clear it, and verify the placeholder is shown.  
**独立测试**：为某分组-基金设置持仓信息，再清空，并验证显示占位文本。

**Acceptance Scenarios** / **验收场景**：

1. **Given** holding info exists for a group-fund pair, **When** I clear it and confirm, **Then** the panel shows the "Not set" placeholder.  
   **假设** 某分组-基金已存在持仓信息，**当** 我清空并确认，**则** 面板显示“未设置”占位文本。

---

### User Story 3 - See Daily Change Amount (Priority: P3) / 用户故事 3 - 查看当日涨跌金额（优先级：P3）

As a user, I want to see the daily change amount for my holding so I can understand today's impact.  
作为用户，我希望看到我的持仓当日涨跌金额，以便了解今日影响。

**Why this priority**: The daily change amount is the most actionable insight after setting holdings.  
**优先级理由**：当日涨跌金额是在设置持仓后最可用的直接洞察。

**Independent Test**: Set holding info and verify the daily change amount appears using the latest available daily change.  
**独立测试**：设置持仓信息并验证基于最新日涨跌的金额显示。

**Acceptance Scenarios** / **验收场景**：

1. **Given** holding info exists and daily change data is available, **When** I view the fund detail panel, **Then** the daily change amount is shown.  
   **假设** 已存在持仓信息且有当日涨跌数据，**当** 我查看基金详情，**则** 显示当日涨跌金额。
2. **Given** daily change data is unavailable, **When** I view the fund detail panel, **Then** the daily change amount shows an unavailable state.  
   **假设** 当日涨跌数据不可用，**当** 我查看基金详情，**则** 当日涨跌金额显示不可用状态。

### Edge Cases / 边界情况

- What happens when the fund has no latest net value available? / 当基金无最新净值可用时会怎样？
- How does the system handle invalid or negative input? / 系统如何处理无效或负数输入？
- What happens when the selected fund is deleted from all lists? / 当所选基金从所有列表删除时会怎样？
- What happens when the same fund exists in multiple groups? / 当同一基金存在于多个分组时会怎样？

## Requirements *(mandatory)* / 需求（必填）

### Functional Requirements / 功能性需求

- **FR-001**: The system MUST provide a holding section in the right fund detail panel.  
  **FR-001**：系统必须在右侧基金详情面板提供持仓区域。
- **FR-002**: Users MUST be able to enter holding info and save it for the selected group and fund.  
  **FR-002**：用户必须能够为当前分组与基金输入并保存持仓信息。
- **FR-003**: The system MUST display the computed holding amount based on the saved holding info for that group-fund pair.  
  **FR-003**：系统必须基于该分组-基金的持仓信息计算并展示持仓金额。
- **FR-004**: The system MUST allow clearing holding info with a confirmation step.  
  **FR-004**：系统必须允许清空持仓信息并提供确认步骤。
- **FR-005**: The system MUST persist holding info locally and restore it on subsequent launches for each group-fund pair.  
  **FR-005**：系统必须将各分组-基金的持仓信息本地持久化并在下次启动时恢复。
- **FR-006**: The system MUST validate holding inputs and show inline feedback on invalid values.  
  **FR-006**：系统必须校验持仓输入并在无效时提供内联提示。
- **FR-007**: When no holding info exists, the system MUST display a clear placeholder state.  
  **FR-007**：当无持仓信息时，系统必须显示清晰的占位状态。
- **FR-008**: The system MUST calculate and display the daily change amount based on holding info and the latest daily change data.  
  **FR-008**：系统必须基于持仓信息与最新当日涨跌数据计算并展示当日涨跌金额。

### Constitution Constraints *(mandatory)* / 宪法约束（必填）

- Data source, privacy, cross-platform consistency, and testing requirements MUST align with `.specify/memory/constitution.md`.  
  数据源、隐私、跨平台一致性、测试要求等 MUST 与 `.specify/memory/constitution.md` 保持一致。
- If any constraint cannot be met, an alternative with explicit rationale MUST be provided.  
  若某条约束无法满足，必须给出替代方案与明确理由。

### Key Entities *(include if feature involves data)* / 关键实体（如涉及数据）

- **Group Fund Position**: A user-defined holding record scoped to a specific group and fund, including holding info used to compute holding amount.  
  **分组基金持仓**：绑定到具体分组与基金的持仓记录，包含用于计算持仓金额的信息。
- **Holding Amount**: The computed value shown in the detail panel for a group-fund position.  
  **持仓金额**：在基金详情面板展示的分组-基金持仓计算值。
- **Daily Change Amount**: The computed daily change in currency based on holding info and daily change data.  
  **当日涨跌金额**：基于持仓信息与当日涨跌数据计算的金额变化。

## Assumptions / 假设

- The holding record is scoped by group + fund code (same fund can have different holdings in different groups).  
  持仓记录以分组 + 基金代码为范围（同一基金在不同分组可有不同持仓）。  
- Daily change data is available from the latest daily change shown in the app; if missing, the daily change amount shows as unavailable.  
  当日涨跌数据来自应用中显示的最新当日涨跌；缺失时显示不可用。

## Success Criteria *(mandatory)* / 成功标准（必填）

### Measurable Outcomes / 可衡量结果

- **SC-001**: Users can set or update holding info for a group-fund pair in under 1 minute.  
  **SC-001**：用户可在 1 分钟内完成分组-基金的持仓信息设置或更新。
- **SC-002**: 95% of users can successfully save holding info on the first attempt.  
  **SC-002**：95% 的用户可在首次尝试时成功保存持仓信息。
- **SC-003**: Holding info remains available after app restart in 99% of sessions.  
  **SC-003**：在 99% 的会话中，重启后仍可加载持仓信息。
- **SC-004**: Daily change amount displays successfully for at least 95% of funds with available daily change data.  
  **SC-004**：对有当日涨跌数据的基金，至少 95% 能成功显示当日涨跌金额。
