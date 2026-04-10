# Feature Specification: ETH Price Tab & Minute History

**Feature Branch**: `001-eth-price-tab`  
**Created**: 2026-02-06  
**Status**: Draft  
**Input**: User description: "增加 ETH tab，可以查看 ETH 的价格，刷新频率还是全局统一控制的， 请求地址 https://www.okx.com/api/v5/market/ticker?instId=ETH-USDT 响应内容 { code: 0, msg: , data: [ { instType: SPOT, instId: ETH-USDT, last: 1901.65, lastSz: 0.00313, askPx: 1901.61, askSz: 5.679606, bidPx: 1901.6, bidSz: 0.56528, open24h: 2135.74, high24h: 2149.99, low24h: 1744, volCcy24h: 1723564983.91155042, vol24h: 884897.083247, ts: 1770346613516, sodUtc0: 1826.7, sodUtc8: 1961.01 } ] } 不要影响现在的功能，新增 tab，后端也是新增接口，新增数据库，记录每分钟的金额"

## User Scenarios & Testing *(mandatory)*

### User Story 1 - View Current ETH Price (Priority: P1)

A user opens the new ETH tab to see the current ETH price and the last update time without affecting existing features.  
用户打开新的 ETH 标签页，查看当前 ETH 价格与最后更新时间，同时不影响已有功能。

**Why this priority**: This is the core user value requested and must be available immediately.  
**为什么是这个优先级**：这是用户明确需求的核心价值，必须优先提供。

**Independent Test**: Can be fully tested by opening the ETH tab and verifying a current price and timestamp are visible.  
**独立测试**：打开 ETH 标签页，确认显示当前价格与更新时间即可完成测试。

**Acceptance Scenarios**:

1. **Given** the app is running, **When** the user opens the ETH tab, **Then** the latest ETH price and last update time are displayed.  
   **给定** 应用正在运行，**当** 用户打开 ETH 标签页，**那么** 显示最新 ETH 价格与最后更新时间。
2. **Given** other tabs are available, **When** the user switches between tabs, **Then** existing tab behavior remains unchanged.  
   **给定** 其他标签页可用，**当** 用户在标签页间切换，**那么** 现有标签页行为不受影响。

---

### User Story 2 - Global Refresh Consistency (Priority: P2)

A user expects ETH price updates to follow the same global refresh setting used elsewhere in the app.  
用户期望 ETH 价格更新频率与应用其他部分使用的全局刷新设置保持一致。

**Why this priority**: Consistent refresh behavior prevents confusion and matches the existing app control model.  
**为什么是这个优先级**：一致的刷新行为可避免困惑，并符合现有应用的控制方式。

**Independent Test**: Can be tested by changing the global refresh setting and observing ETH updates follow that interval.  
**独立测试**：修改全局刷新设置并观察 ETH 更新间隔与之匹配即可测试。

**Acceptance Scenarios**:

1. **Given** the global refresh interval is set, **When** the interval changes, **Then** ETH price updates follow the new interval.  
   **给定** 全局刷新间隔已设置，**当** 间隔被修改，**那么** ETH 价格更新按新间隔执行。

---

### User Story 3 - Minute-Level Price Recording (Priority: P3)

A user keeps the app running and expects ETH prices to be recorded every minute for later review.  
用户保持应用运行，期望 ETH 价格按分钟记录，供后续查看或分析。

**Why this priority**: Minute-level recording is explicitly requested and supports future analytics.  
**为什么是这个优先级**：按分钟记录是明确要求，并支撑后续分析用途。

**Independent Test**: Can be tested by running the app for several minutes and verifying minute-level records are created and persisted.  
**独立测试**：运行应用数分钟并确认每分钟记录生成且可持久保存。

**Acceptance Scenarios**:

1. **Given** the app is running for 5 minutes, **When** the user inspects stored ETH records, **Then** there is one record per minute with timestamps.  
   **给定** 应用运行 5 分钟，**当** 用户查看已保存的 ETH 记录，**那么** 每分钟有一条带时间戳的记录。
2. **Given** the app restarts, **When** the user returns later, **Then** previously recorded ETH data remains available.  
   **给定** 应用重启，**当** 用户稍后返回，**那么** 之前记录的 ETH 数据仍可用。

---

### Edge Cases

- What happens when the market data source is temporarily unavailable?  
  当行情数据源短暂不可用时会如何处理？
- How does the system handle receiving an invalid or empty price?  
  当收到无效或空的价格数据时系统如何处理？
- What happens if the app is paused or suspended for more than one minute?  
  当应用暂停或挂起超过一分钟时会发生什么？

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: The system MUST provide a new ETH tab accessible alongside existing tabs.  
  **系统必须** 提供一个新的 ETH 标签页，可与现有标签页并列访问。
- **FR-002**: The ETH tab MUST display the latest ETH price and the last update time.  
  **系统必须** 在 ETH 标签页显示最新 ETH 价格与最后更新时间。
- **FR-003**: ETH price updates MUST follow the global refresh setting used by the app.  
  **系统必须** 让 ETH 价格更新遵循应用的全局刷新设置。
- **FR-004**: The system MUST record one ETH price snapshot per minute while the app is running.  
  **系统必须** 在应用运行期间按分钟记录 ETH 价格快照。
- **FR-005**: The system MUST avoid creating duplicate minute records for the same minute.  
  **系统必须** 避免同一分钟产生重复记录。
- **FR-006**: If the latest price cannot be retrieved, the ETH tab MUST show the last known price and indicate it may be stale.  
  **系统必须** 在无法获取最新价格时显示上次价格，并提示可能已过期。
- **FR-007**: Existing features and tabs MUST behave the same as before after adding the ETH tab.  
  **系统必须** 在新增 ETH 标签页后保持现有功能与标签页行为不变。
- **FR-008**: The system MUST retain at least 30 days of minute-level ETH price records.  
  **系统必须** 至少保留 30 天的 ETH 分钟级记录。

### Key Entities *(include if feature involves data)*

- **ETH Price Snapshot**: A single minute-level record containing price value and timestamps.  
  **ETH 价格快照**：包含价格数值与时间戳的单分钟记录。
- **Global Refresh Setting**: The user-defined interval that controls update frequency.  
  **全局刷新设置**：控制更新频率的用户设置。

## Constitution Alignment *(mandatory)*

- **Tauri Desktop Architecture**: The feature remains within the desktop app and does not introduce external clients.  
  / **Tauri 桌面架构**：该功能保持在桌面应用内，不引入外部客户端。
- **Rust Owns Data & Network**: Data retrieval and persistence remain owned by Rust commands.  
  / **Rust 管理数据与网络**：数据获取与持久化仍由 Rust 命令负责。
- **UI-Only Frontend**: The UI only presents ETH data and user interactions without business logic.  
  / **仅 UI 前端**：UI 只负责展示与交互，不承载业务逻辑。
- **Local-First Persistence & Recovery**: ETH minute records are stored locally with the same recovery expectations as existing data.  
  / **本地优先持久化与恢复**：ETH 分钟记录本地存储，恢复方式与现有数据一致。
- **Fund List Semantics & Data Integrity**: ETH data does not alter fund list rules or calculations.  
  / **基金列表语义与数据完整性**：ETH 数据不改变基金列表规则或计算。

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: Users can view the current ETH price within 2 seconds of opening the ETH tab.  
  **SC-001**：用户在打开 ETH 标签页后 2 秒内可看到当前价格。
- **SC-002**: During a 24-hour continuous run, at least 95% of minutes have a recorded ETH price snapshot.  
  **SC-002**：应用连续运行 24 小时期间，至少 95% 的分钟产生 ETH 价格记录。
- **SC-003**: 90% of users can verify the ETH price updates match the global refresh setting on first attempt.  
  **SC-003**：90% 的用户在首次尝试时即可确认 ETH 更新符合全局刷新设置。
- **SC-004**: After a restart, users can access previously recorded ETH minute data without errors in 95% of attempts.  
  **SC-004**：应用重启后，95% 的情况下用户可无错误访问历史 ETH 分钟记录。

## Assumptions

- The ETH tab initially focuses on current price and update time; additional analytics or charts are out of scope.  
  初始版本 ETH 标签页仅展示当前价格与更新时间，额外分析或图表不在范围内。
- Data retention is set to a minimum of 30 days unless a future requirement specifies otherwise.  
  数据保留至少 30 天，除非未来需求另行规定。
- The ETH feature follows existing app settings and does not add new user controls beyond the tab.  
  ETH 功能遵循现有设置，不新增除标签页外的用户控制项。
