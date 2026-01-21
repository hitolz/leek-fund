<!--
Sync Impact Report
- Version change: 1.0.0 → 1.1.0
- Modified principles: 一、数据源与准确性 (expanded historical data source)
- Added sections: None
- Removed sections: None
- Templates requiring updates:
  - ✅ .specify/templates/plan-template.md
  - ✅ README.md
-->
# Leek Fund 项目宪法

## Core Principles

### 一、数据源与准确性
- 基金实时数据 MUST 仅来自 `fundgz.1234567.com.cn` JSONP 接口。
- 基金历史走势数据 MUST 仅来自 `fund.eastmoney.com/pingzhongdata/{code}.js`。
- 除上述数据源外不得使用其他外部基金数据服务。
- 基金代码 MUST 为 6 位数字，解析失败 MUST 明确提示并记录错误。
- 数据展示 MUST 标注抓取时间或更新时间以避免误导。
理由：统一数据来源与时效标识，保证一致性与可追溯性。

### 二、本地持久化与数据完整性
- 用户列表与基金集合 MUST 仅存储在本地，不得上传或同步到外部服务。
- 持久化格式变更 MUST 提供迁移逻辑，避免静默丢失历史数据。
- 读取失败或数据损坏 MUST 提供恢复指引并保留原文件。
理由：本地数据是核心资产，需要可恢复与可验证的完整性策略。

### 三、跨平台一致性与性能
- 功能在 macOS、Windows、Linux 上 MUST 保持一致行为与关键交互。
- 网络请求与磁盘 I/O MUST 为异步或非阻塞，避免卡顿主界面。
- UI 渲染 MUST 保持流畅（目标 60fps），避免长时间主线程占用。
理由：桌面应用必须可靠且流畅，避免平台差异造成用户困扰。

### 四、隐私与最小外部依赖
- 应用 MUST 不收集或上传用户数据与行为轨迹。
- 运行时网络访问 MUST 限定为基金数据接口，不得引入未审批的外部服务。
- 任何凭据、密钥或个人信息 MUST 不得进入仓库。
理由：降低合规风险与用户隐私暴露面。

### 五、可维护性与测试
- `fund_api`、`storage`、`list_manager` 变更 MUST 配套单元测试。
- Tauri 命令变更 MUST 同步更新 `specs/.../contracts/tauri-commands.md`。
- 错误类型与边界条件 MUST 明确建模，避免静默失败。
理由：核心逻辑可测试、可追踪，便于长期维护与演进。

## 技术约束与平台要求

- 技术栈 MUST 保持为 Tauri + Rust + React + TypeScript。
- 基金数据源 MUST 遵循 README 中的实时与历史接口规范与字段语义。
- 本地数据 MUST 位于平台标准路径（Tauri 默认应用数据目录）。
- 构建与分发 MUST 通过 `npm run tauri:build` 的标准流程完成。

## 开发流程与质量门禁

- 需求变更 MUST 先更新 `specs/` 中的规格文档，再实施代码修改。
- 影响用户数据结构的改动 MUST 更新数据模型文档并记录迁移说明。
- 任何跨层变更 MUST 同步更新对应的 contracts 与 tasks 文档。
- 合并前 MUST 完成核心模块测试或在 `spec.md` 说明不可测试理由。

## Governance

- 宪法优先于其他约定；如有冲突，以宪法为准。
- 修订流程：提出变更 → 更新宪法与相关模板 → 记录迁移影响 →
  通过评审后合并。
- 版本规则：重大约束调整为 MAJOR，新原则或章节为 MINOR，
  文字澄清为 PATCH。
- 每次评审 MUST 进行宪法合规检查并记录在计划或评审说明中。

**Version**: 1.1.0 | **Ratified**: 2026-01-21 | **Last Amended**: 2026-01-21
