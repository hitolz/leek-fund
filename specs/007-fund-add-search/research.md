# Research: Fund Add Search Filter

**Date**: 2026-01-22  
**Feature**: `/Users/hitol/code/ai/leek-fund/specs/007-fund-add-search/spec.md`

## Decision 1: Identify existing funds by unique identifier

**Decision**: Treat the unique fund identifier (id/code) as the source of truth for duplicate checks and filtering. If the identifier is missing, the add action behaves like a new fund (no filter).  
**决策**：以基金唯一标识（id/代码）作为重复判断与过滤的唯一依据；若缺失标识，则按新增基金处理（不触发过滤）。

**Rationale**: Names can collide across different funds; using the identifier prevents false matches and aligns with the current assumption in the spec.  
**理由**：基金名称可能重复，使用唯一标识可避免误匹配，并与规格中的假设一致。

**Alternatives considered**:
- Name-only match: rejected due to high false-positive risk.  
  仅名称匹配：误判风险高，否决。
- Name + provider match: rejected because provider metadata is not guaranteed in all lists.  
  名称 + 供应商匹配：并非所有列表都具备供应商信息，否决。

## Decision 2: Filter UX and recovery behavior

**Decision**: When an existing fund is detected, the lower list switches to a single-item view, shows a non-blocking “already exists” hint, and exposes a one-step clear action to restore the full list.  
**决策**：检测到已存在基金时，下方列表切换为单项视图，显示“已存在”提示，并提供一步清除以恢复全量列表。

**Rationale**: Users need fast confirmation without blocking the flow, and a clear path to continue browsing.  
**理由**：用户需要快速确认且不被阻断，同时需要清晰入口继续浏览。

**Alternatives considered**:
- Modal warning dialog: rejected for interrupting the flow.  
  弹窗警告：打断流程，否决。
- Auto-clear after a timeout: rejected because it can confuse users who are still reading the result.  
  自动延时清除：可能干扰用户阅读结果，否决。

## Decision 3: Empty state when filtered fund is unavailable

**Decision**: If the lower list cannot show the matching fund (due to current filters or no data), display an empty-state message indicating no results found.  
**决策**：若下方列表无法显示匹配基金（如被其他筛选影响或无数据），显示“无结果”的空状态提示。

**Rationale**: A clear empty state prevents users from assuming the action failed silently.  
**理由**：明确的空状态可避免用户误以为操作无效。

**Alternatives considered**:
- Keep the full list visible: rejected because it obscures the fact that the target fund is missing.  
  保持全量列表：会掩盖目标基金缺失的事实，否决。

## Decision 4: Performance expectations for filtering

**Decision**: The filtered result should appear within 1 second for typical list sizes (<= 1,000 items).  
**决策**：典型列表规模（<= 1,000 项）下，过滤结果在 1 秒内可见。

**Rationale**: This aligns with the success criteria and preserves a responsive add experience.  
**理由**：与成功标准一致，并保持添加操作的响应性。

**Alternatives considered**:
- No explicit target: rejected because it makes success criteria hard to verify.  
  不设目标：难以验证成功标准，否决。
