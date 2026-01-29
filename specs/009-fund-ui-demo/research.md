# Research: Fund Demo UI Redesign / 研究：基金演示页面重设计

## Decision 1: Demo Scope and Architecture Boundary / 决策 1：演示范围与架构边界

- Decision: Implement the demo as a non-shipping, root-level `demo/index.html` artifact with embedded sample data and no live network or storage access.
- Rationale: This preserves the constitution's Rust ownership of data and network while still allowing a fast UI redesign prototype.
- Alternatives considered: Prototype inside the production React app. Rejected because it would either require Rust wiring for demo-only flows or risk UI-side business logic drift.

## Decision 2: Tri-State Sorting Behavior / 决策 2：三态排序行为

- Decision: Provide per-field tri-state sorting (descending, ascending, none). When set to none, restore the list's preserved default order.
- Rationale: This directly satisfies FR-013 and FR-014 and keeps sorting behavior predictable and testable.
- Alternatives considered: Global sort direction shared across fields. Rejected because it makes the UI less explicit and complicates acceptance testing.

## Decision 3: Default Order Preservation / 决策 3：默认顺序保留

- Decision: Capture and retain each fund's default position within its list. Sorting operates on a derived view and never mutates default positions.
- Rationale: Ensures that "no sorting" has a clear, deterministic meaning and supports stable tie handling.
- Alternatives considered: Recompute default order after every add or delete. Rejected because it can cause surprising jumps and makes validation harder.

## Decision 4: Holding Calculations and Display Rules / 决策 4：持仓计算与展示规则

- Decision: Use explicit demo formulas: cost per share equals holding amount divided by holding shares when shares are greater than zero; daily change amount equals holding amount multiplied by daily change percentage.
- Rationale: Matches FR-010 and FR-011 and keeps calculations simple, auditable, and consistent with the assumptions section.
- Alternatives considered: Derive daily change amount from price deltas. Rejected because it requires additional data not present in the demo scope.

## Decision 5: Trend Chart Representation / 决策 5：走势图呈现方式

- Decision: Render the trend chart as inline SVG based on sample trend points embedded with each fund.
- Rationale: Inline SVG avoids dependencies, keeps the demo single-file friendly, and is sufficient for a visual walkthrough.
- Alternatives considered: External chart libraries. Rejected because it would add dependencies and conflict with the "single HTML demo" constraint.

## Decision 6: Sort Cycling and Stability / 决策 6：排序循环与稳定性

- Decision: Sorting is field-specific and cycles through descending → ascending → none on repeated clicks of the same field. “None” restores the preserved default order.
- Rationale: This makes the tri-state behavior explicit, keeps “no sorting” deterministic, and aligns with the default-order preservation decision.
- Alternatives considered: Sharing a global sort direction across fields. Rejected because it obscures the current state and complicates reviewer validation.

- Decision: Sorting is stable for ties by using the fund’s default index as a tie-breaker and by sorting a derived view instead of mutating the stored default order.
- Rationale: Stable sorting prevents confusing row jumps and guarantees that returning to “none” yields the original order.
- Alternatives considered: Mutating the default order to match the last applied sort. Rejected because it breaks the meaning of “no sorting.”
