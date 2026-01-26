# Research / 调研

## Decision 1: Scope holding by group + fund / 决策 1：按分组 + 基金范围持仓

**Decision**: Store holding info keyed by group (list) and fund code so the same fund can have different holdings in different groups.  
**决策**：持仓信息按分组（列表）+ 基金代码作为键，以便同一基金在不同分组可配置不同持仓。

**Rationale**: The feature explicitly requires linking holdings to a specific group and fund, and avoids ambiguity when a fund appears in multiple lists.  
**理由**：需求明确要求关联到具体分组与基金，可避免基金在多个列表中出现时的歧义。

**Alternatives considered**: Global holding per fund code only (simpler but cannot differentiate by group).  
**备选方案**：仅按基金代码全局持仓（更简单但无法区分分组）。

## Decision 2: Input model uses shares + cost price / 决策 2：输入采用份额 + 成本价

**Decision**: Users enter shares and cost price; holding amount is computed from shares and latest net value.  
**决策**：用户输入持仓份额与成本价；持仓金额由份额与最新净值计算得出。

**Rationale**: Aligns with the leek-fund reference behavior and avoids stale raw amounts when net value changes.  
**理由**：对齐 leek-fund 参考行为，避免净值变化导致金额失真。

**Alternatives considered**: Enter raw holding amount only (simpler UI but less accurate over time).  
**备选方案**：仅输入持仓金额（UI 更简单但长期准确性不足）。

## Decision 3: Daily change amount calculation / 决策 3：当日涨跌金额计算

**Decision**: Compute daily change amount as `holding_amount * daily_change_percent / 100` using the latest available daily change percent.  
**决策**：当日涨跌金额按 `持仓金额 × 当日涨跌百分比 / 100` 计算，使用最新可用涨跌百分比。

**Rationale**: This reuses existing daily change percent data and provides an immediate, consistent estimate of today's impact.  
**理由**：复用现有涨跌百分比数据，提供直观一致的今日影响估算。

**Alternatives considered**: Compute by `shares * (today_nav - yesterday_nav)` (more precise but depends on consistent nav fields).  
**备选方案**：按 `份额 × (今日净值 - 昨日净值)` 计算（更精确但依赖净值字段一致性）。

## Decision 4: Local persistence format / 决策 4：本地持久化格式

**Decision**: Extend local storage to include a positions collection for group-fund holdings with migration on load.  
**决策**：本地存储扩展 `positions` 集合保存分组-基金持仓，并在加载时进行迁移。

**Rationale**: Keeps data local per constitution and ensures backward compatibility.  
**理由**：符合宪法要求的本地存储，并保证向后兼容。

**Alternatives considered**: Separate file for holdings (more isolated but increases operational complexity).  
**备选方案**：单独文件存储持仓（隔离性更好但操作复杂）。
