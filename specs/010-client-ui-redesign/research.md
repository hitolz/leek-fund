# Research: Client UI Redesign From Demo / 研究：参照演示的客户端界面重设计

## Mapping Notes / 布局映射说明

- Demo left panel → Client list panel (list selection, create/rename/delete).  
  Demo 左侧面板 → 客户端列表面板（列表选择、创建/重命名/删除）。
- Demo middle panel → Client fund list view (add fund, sort controls, fund rows).  
  Demo 中间面板 → 客户端基金列表视图（添加基金、排序控件、基金行）。
- Demo right panel → Client fund detail panel (metrics, charts, holdings).  
  Demo 右侧面板 → 客户端基金详情面板（指标、走势图、持仓）。

## Style Tokens / 样式要点

- Background: dark gradient with layered panels and subtle borders.  
  背景：深色渐变，卡片式面板与细边框。
- Panels: rounded corners, soft shadows, translucent surfaces.  
  面板：圆角、柔和阴影、半透明质感。
- Typography: bold titles, muted subtitles, compact meta text.  
  文案：标题加粗，副标题柔和，元信息紧凑。
- States: active/selected uses green-blue glow; up/down color accents.  
  状态：选中态使用青绿高亮，涨跌色彩区分。

## Decision 1: Demo-to-Client Mapping / 决策 1：演示布局到客户端映射

- Decision: Use the demo page’s three-column structure (list panel, fund list, detail panel) as the primary layout scaffold in the client.
- Rationale: The demo’s layout expresses the desired information hierarchy and aligns with the core workflow defined in the spec.
- Alternatives considered: Keep the existing client layout and only apply visual styling. Rejected because it would not satisfy the “align with demo layout and style” requirement.

## Decision 2: Style Alignment Strategy / 决策 2：样式对齐策略

- Decision: Extract reusable style tokens (colors, spacing, typography, state badges) from the demo page and apply them to the client UI consistently.
- Rationale: Ensures recognizable visual alignment while respecting the current component structure.
- Alternatives considered: One-off styling overrides per component. Rejected because it risks inconsistency and higher maintenance.

## Decision 3: Behavior Parity Guardrails / 决策 3：行为一致性护栏

- Decision: Treat all data access and business rules as unchanged; UI redesign only updates presentation and interaction layout.
- Rationale: Preserves constitution alignment and reduces regression risk.
- Alternatives considered: Adjust UI-side calculations to match demo formulas. Rejected because calculations must remain Rust-owned.

## Decision 4: Sorting Feedback / 决策 4：排序反馈方式

- Decision: Keep tri-state sorting and default-order restoration behavior identical to existing rules, only updating the visual controls and indicators to match the demo style.
- Rationale: Sorting semantics are user-critical and must not change during visual redesign.
- Alternatives considered: Simplify sorting to two states. Rejected because it conflicts with existing requirements.
