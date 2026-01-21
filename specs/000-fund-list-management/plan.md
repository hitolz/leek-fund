# Implementation Plan: Fund List Management

## Metadata
- **Plan ID**: PLAN-001
- **Created**: 2025-10-20
- **Status**: Draft
- **Related Constitution Principles**: Principles 1, 2, 3, 4, 5

## Objective

Implement a cross-platform desktop application using Tauri and Rust that enables users to query Chinese mutual fund information, organize funds into multiple custom lists, and persist data locally. The implementation must strictly adhere to all five constitutional principles, with clear separation between Rust backend (data operations) and frontend (UI/UX).

## Constitution Check

Before proceeding, verify alignment with project constitution:
- [x] **Principle 1 (Tauri Architecture)**: Uses Tauri for cross-platform desktop framework with native webview
- [x] **Principle 2 (Rust Backend)**: All HTTP requests to fund APIs and data processing implemented in Rust
- [x] **Principle 3 (Frontend Separation)**: Frontend only handles UI rendering, state management, and user interactions via Tauri commands
- [x] **Principle 4 (Local Storage)**: User lists and preferences persisted in local file system (JSON or SQLite)
- [x] **Principle 5 (Multi-List Management)**: List manager module enforces uniqueness constraint within lists, allows cross-list duplication

**Constitution Compliance Status**: ✅ All principles satisfied

## Technical Approach

### Backend (Rust)

#### Core Modules

1. **fund_api Module**
   - HTTP client using `reqwest` crate
   - Async fund data fetching from `fundgz.1234567.com.cn`
   - Response parsing (JavaScript format → structured data)
   - Error handling with retry logic
   - Timeout configuration (10 seconds)
   - Data models: `FundInfo` struct with code, name, net_value, update_time

2. **storage Module**
   - Persistence layer using `serde` for serialization
   - Storage backend options:
     - **Option A**: JSON file in app data directory (simpler, recommended for MVP)
     - **Option B**: SQLite with `rusqlite` (more scalable, future consideration)
   - CRUD operations for lists and fund memberships
   - Atomic write operations to prevent corruption
   - Data validation on load (handle corrupted files gracefully)

3. **list_manager Module**
   - Business logic for list operations (create, rename, delete, reorder)
   - Duplicate detection: HashSet-based uniqueness check within lists
   - Fund membership management (add, remove)
   - Cross-list operations support
   - Data models: `FundList` struct with id, name, fund_codes, created_at

4. **tauri_commands Module**
   - Command handlers bridging frontend and backend
   - Commands expose async functions to frontend
   - Error propagation to frontend with user-friendly messages
   - State management using Tauri's managed state

#### Key Data Structures

```rust
// Core data models (serializable)
struct FundInfo {
    code: String,          // 6-digit fund code
    name: String,          // Fund name in Chinese
    net_value: Option<f64>, // Current net value
    update_time: Option<String>, // Last update timestamp
}

struct FundList {
    id: String,            // UUID for list
    name: String,          // User-defined list name
    fund_codes: Vec<String>, // Ordered collection of fund codes
    created_at: i64,       // Unix timestamp
    position: usize,       // For list ordering
}

struct AppState {
    lists: Vec<FundList>,
    storage_path: PathBuf,
}
```

#### Tauri Commands (Rust → Frontend Interface)

- `search_fund(code: String) -> Result<FundInfo, String>`
- `get_all_lists() -> Result<Vec<FundList>, String>`
- `create_list(name: String) -> Result<FundList, String>`
- `rename_list(id: String, new_name: String) -> Result<(), String>`
- `delete_list(id: String) -> Result<(), String>`
- `add_fund_to_list(list_id: String, fund_code: String) -> Result<(), String>`
- `remove_fund_from_list(list_id: String, fund_code: String) -> Result<(), String>`
- `get_list_funds(list_id: String) -> Result<Vec<FundInfo>, String>`
- `reorder_lists(list_ids: Vec<String>) -> Result<(), String>`

### Frontend

#### Technology Selection

**Recommended**: React + TypeScript + Vite (well-supported by Tauri, modern tooling)
**Alternatives**: Vue 3 + TypeScript, Svelte + TypeScript

#### Component Structure

1. **SearchBar Component**
   - Input field for 6-digit fund code
   - Real-time validation (only numbers, max 6 digits)
   - Debounced search trigger (300ms after typing stops)
   - Loading state during API call
   - Error display for invalid/not found codes

2. **FundInfoCard Component**
   - Displays search result (fund name, net value, update time)
   - "Add to List" button with dropdown for list selection
   - Success/error feedback after add operation

3. **ListsPanel Component**
   - Sidebar or main panel showing all user lists
   - List items with name and fund count
   - Create new list button
   - List operations menu (rename, delete)
   - Drag-and-drop for list reordering

4. **ListDetailView Component**
   - Shows all funds within selected list
   - Fund cards with code, name, and remove button
   - Empty state message for new lists
   - Batch operations (future: select multiple, remove all)

5. **App Layout Component**
   - Top navigation/header
   - Main content area with routing
   - Toast/notification system for user feedback

#### State Management

- **Local component state**: UI-only state (input values, modal visibility)
- **Tauri invoke calls**: Direct async calls to Rust backend for data operations
- **React Context or Zustand**: Shared state for lists, current selection
- **React Query (optional)**: Caching and background refetching for fund searches

#### Error Handling

- Network errors: Display retry button with clear message
- Duplicate fund: Show informative toast "基金已在列表中"
- List name conflict: Inline validation error on input
- Data load failure: Full-screen error with option to restart with clean state

### Data Flow

#### Search Fund Flow
1. User types fund code in SearchBar
2. Frontend debounces input, validates format
3. Frontend calls `invoke('search_fund', { code })`
4. Rust backend: HTTP request to `fundgz.1234567.com.cn/js/{code}.js`
5. Rust backend: Parse JavaScript response, extract fund info
6. Rust backend: Return `FundInfo` or error
7. Frontend: Display result in FundInfoCard or show error

#### Add Fund to List Flow
1. User clicks "Add to List" on FundInfoCard, selects target list
2. Frontend calls `invoke('add_fund_to_list', { list_id, fund_code })`
3. Rust backend: Load current list from storage
4. Rust backend: Check for duplicate (HashSet contains)
5. Rust backend: If unique, add to list and save to storage
6. Rust backend: Return success or duplicate error
7. Frontend: Show toast confirmation or error message

#### Load Lists on Startup
1. App launches, frontend mounts
2. Frontend calls `invoke('get_all_lists')`
3. Rust backend: Read from storage file (JSON or SQLite)
4. Rust backend: Deserialize to Vec<FundList>
5. Rust backend: Return lists
6. Frontend: Update state, render ListsPanel

#### Data Persistence
- All list modifications (create, rename, delete, add fund, remove fund) trigger immediate save
- Storage operations atomic: write to temp file, then rename (prevents corruption)
- On startup: load from file, validate structure, fallback to empty state if corrupted

## Implementation Steps

### Phase 1: Project Setup & Foundation
1. Initialize Tauri project with React frontend (`npm create tauri-app`)
2. Configure Rust dependencies in `Cargo.toml`:
   - `reqwest` with async and JSON features
   - `serde` and `serde_json` for serialization
   - `tokio` for async runtime
   - `uuid` for list IDs
   - `chrono` for timestamps
3. Set up frontend tooling (TypeScript, ESLint, Prettier)
4. Create base directory structure:
   - `src-tauri/src/modules/` (fund_api, storage, list_manager)
   - `src/components/` (React components)
   - `src/hooks/` (custom React hooks for Tauri commands)
5. Configure Tauri permissions and CSP for external API access

### Phase 2: Rust Backend Implementation
1. **storage module**:
   - Define `FundList` and `AppState` structs with `serde` derives
   - Implement JSON file storage functions (read, write, atomic update)
   - Implement data validation and error recovery
   - Write unit tests for storage operations

2. **fund_api module**:
   - Define `FundInfo` struct
   - Implement `search_fund_info(code: &str) -> Result<FundInfo>`
   - Handle timeout, network errors, parsing errors
   - Write unit tests with mock HTTP responses

3. **list_manager module**:
   - Implement list CRUD operations
   - Implement duplicate checking with HashSet
   - Implement fund add/remove with uniqueness enforcement
   - Write unit tests for all operations, especially duplicate scenarios

4. **tauri_commands module**:
   - Implement all Tauri command handlers
   - Wire up to modules (fund_api, list_manager, storage)
   - Initialize managed state in `main.rs`
   - Write integration tests for command invocations

### Phase 3: Frontend Implementation
1. **Create base components**:
   - App layout with navigation
   - Toast notification system
   - Loading spinner component

2. **Implement SearchBar and FundInfoCard**:
   - Input with validation
   - Debounced search with `useDebouncedValue` hook
   - Display fund info with add button
   - Handle loading and error states

3. **Implement ListsPanel**:
   - Display all lists with fund counts
   - Create new list dialog
   - List operations menu (rename, delete with confirmation)
   - Drag-and-drop reordering (using react-dnd or native)

4. **Implement ListDetailView**:
   - Fetch and display funds for selected list
   - Fund cards with remove functionality
   - Empty state for new lists
   - Confirmation dialog for fund removal

5. **Create custom hooks for Tauri commands**:
   - `useFundSearch(code: string)`
   - `useLists()`
   - `useAddFundToList()`
   - `useListOperations()`

### Phase 4: Integration & Testing
1. **End-to-end user flows**:
   - Search fund → Add to list → Verify persistence
   - Create multiple lists → Add same fund to different lists
   - Attempt duplicate add → Verify error handling
   - Close and reopen app → Verify data loads

2. **Error scenario testing**:
   - Network timeout during fund search
   - Invalid fund code
   - Corrupted storage file
   - List name conflicts

3. **Performance testing**:
   - Large lists (200 funds) load time
   - Search response time
   - UI responsiveness during operations

4. **Cross-platform testing**:
   - Build and test on macOS, Windows, Linux
   - Verify storage paths work on all platforms
   - Check UI rendering consistency

### Phase 5: Polish & Optimization
1. Optimize bundle size (tree-shaking, code splitting)
2. Add loading skeletons for better perceived performance
3. Implement keyboard shortcuts (Ctrl+N for new list, etc.)
4. Add fund search history (optional)
5. Accessibility improvements (ARIA labels, keyboard navigation)
6. Chinese language support verification for all UI text

## Testing Strategy

- [x] **Unit tests for Rust functions**
  - Storage: read, write, atomic operations, corruption handling
  - List manager: CRUD, duplicate detection, cross-list scenarios
  - Fund API: parsing, error handling, timeout

- [x] **Integration tests for Tauri commands**
  - Each command invocation with success and error cases
  - State management across multiple commands
  - Concurrent command handling

- [x] **Frontend component tests**
  - Component rendering with various props
  - User interaction simulation (clicks, input)
  - Error state display

- [x] **End-to-end user workflow tests**
  - Complete user scenarios from spec
  - Data persistence verification
  - Cross-platform functionality

**Test Coverage Goals**:
- Rust backend: >80% code coverage
- Critical paths: 100% (duplicate detection, data persistence)
- Frontend: >70% component coverage

## Risks & Mitigations

| Risk | Impact | Mitigation |
|------|--------|------------|
| Fund API response format changes | High | Implement robust parsing with fallbacks; log parse errors; version detection |
| Data corruption from concurrent writes | High | Use atomic file operations (write to temp + rename); file locking if needed |
| Network unreliability in China | Medium | Implement retry logic with exponential backoff; cache last successful result |
| Large list performance degradation | Medium | Virtualized list rendering for >100 items; pagination if needed |
| Storage file size growth | Low | Monitor size; implement archiving or cleanup for very old data |
| Cross-platform storage path issues | Medium | Use Tauri's app_data_dir() API; test on all platforms early |
| Fund code format changes | Low | Validate input format; make code length configurable |
| Tauri security vulnerabilities | Medium | Keep Tauri updated; follow security best practices; regular audits |

## Dependencies

### External Libraries/Crates

**Rust (Cargo.toml)**:
```toml
[dependencies]
tauri = "1.5"
reqwest = { version = "0.11", features = ["json"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
tokio = { version = "1.35", features = ["full"] }
uuid = { version = "1.6", features = ["v4", "serde"] }
chrono = { version = "0.4", features = ["serde"] }

[dev-dependencies]
mockito = "1.2"  # For HTTP mocking in tests
```

**Frontend (package.json)**:
```json
{
  "dependencies": {
    "@tauri-apps/api": "^1.5.0",
    "react": "^18.2.0",
    "react-dom": "^18.2.0",
    "react-router-dom": "^6.20.0"
  },
  "devDependencies": {
    "@types/react": "^18.2.0",
    "@vitejs/plugin-react": "^4.2.0",
    "typescript": "^5.3.0",
    "vite": "^5.0.0"
  }
}
```

### API Endpoints

- `http://fundgz.1234567.com.cn/js/{fund_code}.js` - Real-time fund information
- Response format: JavaScript variable assignment (parse as JSONP)

### Configuration Requirements

1. **Tauri Configuration** (`tauri.conf.json`):
   - Allow HTTP access to `fundgz.1234567.com.cn` in CSP
   - Configure app data directory for storage
   - Set window dimensions and constraints

2. **Development Environment**:
   - Rust 1.70+ (for Tauri compatibility)
   - Node.js 18+ and npm/pnpm
   - Tauri CLI installed globally

3. **Build Configuration**:
   - Cross-compilation setup for target platforms
   - Code signing certificates for distribution (future)

## Success Criteria

- [x] All 8 functional requirements from spec implemented and tested
- [x] Fund search returns results in <2 seconds for valid codes
- [x] Duplicate prevention works 100% reliably (zero failures in testing)
- [x] Data persists correctly across app restarts (100 consecutive tests pass)
- [x] Application builds and runs on macOS, Windows, Linux
- [x] All constitution principles upheld (verified in code review)
- [x] No implementation shortcuts that violate architectural principles
- [x] User can complete "search → add to list" flow in <10 seconds
- [x] All edge cases from spec handled gracefully
- [x] Zero crashes or data loss in testing

## Next Steps

After plan approval:
1. Review and approve this implementation plan
2. Execute Phase 1: Project setup
3. Proceed through phases sequentially with testing at each stage
4. Regular constitution compliance checks during development
5. User acceptance testing before final release

## Notes

- **Constitution Compliance**: This plan strictly adheres to all five principles. Any deviations during implementation must be justified and documented.
- **Technology Choices**: React chosen for frontend due to mature ecosystem and Tauri support. SQLite deferred to post-MVP for simplicity.
- **API Parsing**: Fund API returns JavaScript (not JSON). Parser must handle `jsonpgz(...)` wrapper format.
- **Storage Location**: Use Tauri's `app_data_dir()` to ensure cross-platform compatibility for storage file location.
