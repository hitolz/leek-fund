# Research: AI 对话 Tab

## Decision 1: 流式回复的传输方式

**Decision**: 后端提供本地 SSE 流式接口，前端通过事件流逐段接收并追加展示。
**Rationale**: 与需求的“流式回复”一致，且便于在桌面应用中逐步渲染助手回复。
**Alternatives considered**: WebSocket（更复杂的连接管理）；轮询（延迟更高且不符合流式体验）。

## Decision 2: 会话加载策略

**Decision**: 进入 Tab 默认加载最近会话；若无历史会话则自动创建。
**Rationale**: 交互路径最短，符合轻量对话体验，并与澄清结论一致。
**Alternatives considered**: 强制选择会话列表；每次进入都新建会话。

## Decision 3: 持久化失败与流中断处理

**Decision**: 持久化失败时保留已展示内容并标记未保存；流中断提示可重试并保留已显示内容。
**Rationale**: 保持用户可见内容不丢失，降低误操作成本；提示明确，便于恢复。
**Alternatives considered**: 直接撤回显示内容；自动重连继续但可能引入状态不一致。

## Decision 4: 会话保留策略

**Decision**: 历史会话默认永久保留，暂不提供自动清理。
**Rationale**: 减少本阶段范围与复杂度，符合现有假设。
**Alternatives considered**: 定期自动清理（30/90 天）。
