# Feature 001: Fund List Management

> **Status**: 📋 Planning Complete - Ready for Implementation  
> **Branch**: `001-fund-list-management`  
> **Estimated Timeline**: 5-6 days for experienced developer

---

## Quick Links

- 📖 **[Feature Specification](./spec.md)** - What we're building and why
- 🗺️ **[Implementation Plan](./plan.md)** - How we'll build it
- 📊 **[Data Model](./data-model.md)** - Data structures and relationships
- 🔌 **[API Contracts](./contracts/tauri-commands.md)** - Frontend-Backend interface
- 🚀 **[Quickstart Guide](./quickstart.md)** - Get started in 5 minutes
- ✅ **[Task List](./tasks.md)** - 28 tasks broken down by phase
- 🔬 **[Research & Decisions](./research.md)** - Technical choices explained

---

## Overview

This feature enables users to:
- 🔍 Search for Chinese mutual funds by 6-digit code
- 📝 Create multiple custom fund lists (portfolios)
- ➕ Add funds to lists with automatic duplicate prevention
- 💾 Persist all data locally with 100% reliability
- 🔄 Manage lists: create, rename, delete, reorder

**Key Principle**: Multi-list management with uniqueness within lists, allowing cross-list duplication.

---

## Architecture Summary

```
┌─────────────────────────────────────────┐
│         Frontend (React + TS)           │
│  SearchBar | ListsPanel | ListDetail    │
│            ↓ Tauri IPC ↓                │
├─────────────────────────────────────────┤
│         Rust Backend (Tauri)            │
│  fund_api | list_manager | storage      │
│            ↓           ↓                │
│      External API   SQLite (local)      │
└─────────────────────────────────────────┘
```

### Technology Stack

**Backend**:
- Rust 1.70+
- Tauri 1.5 (desktop framework)
- reqwest (HTTP client)
- serde/serde_json (serialization)

**Frontend**:
- React 18
- TypeScript 5
- Vite (build tool)

**Data**:
- Local SQLite storage (with JSON migration) / 本地 SQLite 存储（含 JSON 迁移）
- External fund API: `fundgz.1234567.com.cn` / 外部基金 API：`fundgz.1234567.com.cn`

---

## Constitution Compliance

This feature strictly adheres to all 5 constitutional principles:

| Principle | Implementation |
|-----------|----------------|
| **1. Tauri Desktop Architecture / Tauri 桌面架构** | ✅ Cross-platform desktop via Tauri |
| **2. Rust Owns Data & Network / Rust 管理数据与网络** | ✅ All HTTP/data ops in Rust modules |
| **3. UI-Only Frontend / 仅 UI 前端** | ✅ UI only, no business logic |
| **4. Local-First Persistence & Recovery / 本地优先持久化与恢复** | ✅ SQLite storage, JSON migration, no cloud dependency |
| **5. Fund List Semantics & Data Integrity / 基金列表语义与数据完整性** | ✅ Uniqueness within list, deterministic validation |

---

## Feature Scope

### Included ✅

- Fund code search with real-time results
- Multiple user-created lists
- Add/remove funds to/from lists
- Duplicate prevention within each list
- List management (create, rename, delete, reorder)
- Local data persistence
- Cross-platform support (macOS, Windows, Linux)

### Explicitly Excluded ❌

- Historical fund performance charts
- Real-time price updates (manual search only)
- Fund comparison tools
- Portfolio value calculations
- Buying/selling funds
- User accounts or cloud sync
- Import/export functionality
- Price alerts or notifications

---

## Documentation Structure

### Planning Documents (You Are Here)

```
specs/001-fund-list-management/
├── README.md                    # This file - overview
├── spec.md                      # Feature specification (user-focused)
├── plan.md                      # Implementation plan (developer-focused)
├── research.md                  # Technical decisions with rationale
├── data-model.md                # Data structures and validation
├── tasks.md                     # 28 tasks across 8 phases
├── quickstart.md                # Developer onboarding
├── contracts/
│   └── tauri-commands.md        # API contract (9 commands)
└── checklists/
    ├── requirements.md          # Spec quality validation (✅ passed)
    └── plan-checklist.md        # Plan quality validation (✅ passed)
```

### How to Use These Documents

**If you are...**

- 🎯 **Product Manager**: Read `spec.md` for requirements
- 👨‍💻 **Implementing**: Start with `quickstart.md`, then `tasks.md`
- 🏗️ **Architecting**: Review `plan.md` and `data-model.md`
- 🤝 **Integrating Frontend**: Check `contracts/tauri-commands.md`
- 🔍 **Understanding Decisions**: Read `research.md`
- ✅ **Reviewing**: Use `checklists/` to validate completeness

---

## Implementation Phases

### Phase 0: Project Setup (1.25 hours)
- Initialize Tauri project
- Configure dependencies
- Set up directory structure

### Phase 1: Backend - Data & Storage (5.5 hours)
- Implement data models (FundInfo, FundList, StorageFormat)
- Build JSON storage with atomic writes
- Create list manager with duplicate prevention

### Phase 2: Backend - Fund API (2.75 hours)
- Define error types
- Build HTTP client for fund API
- Implement JSONP parsing

### Phase 3: Backend - Tauri Commands (4 hours)
- Implement 9 command handlers
- Initialize app state management
- Wire up modules

### Phase 4: Frontend - Infrastructure (2.75 hours)
- Define TypeScript types
- Create Tauri command hooks
- Build validation utilities

### Phase 5: Frontend - UI Components (9 hours)
- Layout and navigation
- SearchBar with debounced search
- FundInfoCard with add action
- ListsPanel with CRUD operations
- ListDetailView with fund display

### Phase 6: Integration Testing (5.5 hours)
- Backend integration tests
- Frontend integration tests
- End-to-end user flows

### Phase 7: Polish & Optimization (5.5 hours)
- UI/UX improvements
- Error handling refinement
- Performance optimization
- Documentation

### Phase 8: Release Preparation (3 hours)
- Cross-platform builds
- Final QA testing

**Total Estimated Time**: 39.25 hours (~5-6 days)

---

## Key Technical Decisions

| Decision | Choice | Rationale |
|----------|--------|-----------|
| Storage Backend | SQLite (with JSON migration) | Reliable local store with migration path |
| API Parsing | JSONP Extraction | Fund API returns `jsonpgz(...)` wrapper |
| Duplicate Detection | HashSet | O(1) lookup, in-memory efficiency |
| Frontend Framework | React + TypeScript | Mature ecosystem, Tauri support |
| Async Runtime | Tokio | Standard for Rust, required by Tauri |
| HTTP Client | reqwest | Most popular, well-maintained |
| Error Strategy | Custom Enums | User-friendly Chinese messages |
| List Ordering | Position Field | Explicit order, survives restarts |

All decisions documented with alternatives considered in [`research.md`](./research.md).

---

## API Surface

### Tauri Commands (Rust → Frontend)

```typescript
// Fund Operations
search_fund(code: string) → FundInfo

// List Operations
get_all_lists() → FundList[]
create_list(name: string) → FundList
rename_list(id: string, new_name: string) → void
delete_list(id: string) → void
reorder_lists(list_ids: string[]) → void

// Fund-List Membership
add_fund_to_list(list_id: string, fund_code: string) → void
remove_fund_from_list(list_id: string, fund_code: string) → void
get_list_funds(list_id: string) → FundInfo[]
```

Full contract with error cases: [`contracts/tauri-commands.md`](./contracts/tauri-commands.md)

---

## Data Model Summary

### FundInfo
```typescript
{
  code: string;          // 6-digit fund code
  name: string;          // Fund name (Chinese)
  net_value: number?;    // Current NAV
  update_time: string?;  // Last update timestamp
}
```

### FundList
```typescript
{
  id: string;            // UUID v4
  name: string;          // User-defined (1-30 chars, unique)
  fund_codes: string[];  // Ordered, unique within list
  created_at: number;    // Unix timestamp
  position: number;      // Display order
}
```

### StorageFormat
```typescript
{
  version: number;       // Schema version (currently 1)
  lists: FundList[];     // All user's lists (max 50)
  created_at: number;    // First creation
  last_modified: number; // Last save
}
```

Full schema with validation rules: [`data-model.md`](./data-model.md)

---

## Success Criteria (from Spec)

1. ✅ Users complete "search → add to list" in <10 seconds
2. ✅ Zero data loss over 30-day period
3. ✅ 95% of valid fund searches return results successfully
4. ✅ Data persists 100% reliably across restarts
5. ✅ Zero duplicate funds within same list
6. ✅ Application responds within 100ms for local operations

---

## Testing Strategy

### Coverage Targets
- **Rust Backend**: >80% code coverage
- **Critical Paths**: 100% (duplicate detection, persistence)
- **Frontend**: >70% component coverage

### Test Types
1. **Unit Tests**: Each module independently
2. **Integration Tests**: Tauri commands with state
3. **E2E Tests**: Complete user workflows
4. **Performance Tests**: Large lists (200 funds)
5. **Cross-Platform Tests**: macOS, Windows, Linux

---

## Getting Started

### For Developers

1. **Read**: [`quickstart.md`](./quickstart.md) for 5-minute setup
2. **Review**: [`plan.md`](./plan.md) for architecture overview
3. **Start**: Phase 0 tasks from [`tasks.md`](./tasks.md)
4. **Reference**: [`contracts/tauri-commands.md`](./contracts/tauri-commands.md) during implementation

### Prerequisites

```bash
# Check versions
rust --version    # Need 1.70+
node --version    # Need 18+
cargo tauri --version  # Install if missing
```

### Quick Start

```bash
cd /Users/hitol/code/ai/leek-fund
npm create tauri-app
# Select: React, TypeScript, npm

npm install
cd src-tauri
cargo build
cd ..
npm run tauri dev
```

Full instructions: [`quickstart.md`](./quickstart.md)

---

## Quality Assurance

### Specification Quality ✅
- All requirements testable and unambiguous
- Success criteria measurable
- No implementation details in spec
- **Status**: [Passed all checks](./checklists/requirements.md)

### Plan Quality ✅
- All spec requirements addressed
- Constitution compliance verified
- Technical decisions justified
- Comprehensive task breakdown
- **Status**: [Passed all checks](./checklists/plan-checklist.md)

---

## Risk Management

### Identified Risks (8)

1. **Fund API format changes** → Robust parsing + fallbacks
2. **Data corruption** → Atomic writes + backups
3. **Network unreliability** → Retry logic + timeouts
4. **Large list performance** → Virtualized rendering
5. **Storage size growth** → Monitoring + cleanup
6. **Cross-platform paths** → Tauri's app_data_dir()
7. **Fund code format changes** → Configurable validation
8. **Tauri security** → Regular updates + audits

Full risk matrix: [`plan.md#risks--mitigations`](./plan.md#risks--mitigations)

---

## Version History

| Version | Date | Status | Description |
|---------|------|--------|-------------|
| 1.0.0 | 2025-10-20 | ✅ Current | Initial planning complete |

---

## Contributing

Before implementing:
1. Review [constitution principles](../../.specify/memory/constitution.md)
2. Ensure task acceptance criteria understood
3. Write tests alongside code
4. Regular constitution compliance checks

---

## Support & Questions

**During Implementation**:
- Specification unclear? → Review [`spec.md`](./spec.md) user scenarios
- Architecture question? → Check [`plan.md`](./plan.md) technical approach
- API signature? → See [`contracts/tauri-commands.md`](./contracts/tauri-commands.md)
- Why this decision? → Read [`research.md`](./research.md)

**Constitution Questions**:
- Refer to [project constitution](../../.specify/memory/constitution.md)
- All architectural decisions must align with 5 principles

---

## Next Steps

✅ **Planning Phase Complete**

🚀 **Ready to Start Phase 0: Project Setup**

See [`tasks.md`](./tasks.md) for detailed task breakdown and begin with Task 0.1.

---

**Document Version**: 1.0.0  
**Last Updated**: 2025-10-20  
**Maintained By**: Feature specification system
