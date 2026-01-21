# Research & Technical Decisions: Fund List Management

## Metadata
- **Created**: 2025-10-20
- **Phase**: Phase 0 - Research
- **Status**: Complete

## Purpose

This document consolidates research findings and technical decisions for implementing the Fund List Management feature, resolving all technical unknowns identified during planning.

---

## Decision 1: Storage Backend Selection

### Context
Need to choose between JSON file storage and SQLite for persisting user lists locally.

### Decision
**Use JSON file storage for MVP**

### Rationale
1. **Simplicity**: JSON with `serde_json` requires zero schema management
2. **Transparency**: Users can inspect/backup their data file easily
3. **Performance**: For expected data size (<1000 funds across <50 lists), JSON is sufficient
4. **Dependencies**: One less dependency (no `rusqlite`)
5. **Atomic writes**: Simple file operations with temp file + rename pattern

### Implementation Details
- Storage location: `{app_data_dir}/leek-fund/lists.json`
- Structure: Single JSON object with lists array and metadata
- Atomic write pattern: Write to `lists.json.tmp` → rename to `lists.json`
- Backup on corruption: Rename corrupted file to `lists.json.backup.{timestamp}`

### Alternatives Considered
- **SQLite**: More scalable, better for complex queries, but overkill for MVP
- **TOML**: Less suitable for nested structures than JSON
- **bincode**: Binary format faster but not human-readable

### Future Migration Path
If data grows beyond 5MB or >10,000 funds, migrate to SQLite with migration script.

---

## Decision 2: Fund API Response Parsing

### Context
Fund API at `fundgz.1234567.com.cn/js/{code}.js` returns JSONP (JavaScript) format, not pure JSON.

### Decision
**Parse JSONP by extracting JSON from JavaScript wrapper**

### Rationale
Response format: `jsonpgz({"fundcode":"001632","name":"...","jzrq":"..."})`

Strategy:
1. Fetch response as text
2. Regex extract content between `jsonpgz(` and `)`
3. Parse extracted JSON with `serde_json`
4. Validate required fields exist

### Implementation Details

```rust
// Pseudo-code for parsing
fn parse_fund_response(text: &str) -> Result<FundInfo> {
    // Extract JSON from JSONP wrapper
    let json_str = text
        .strip_prefix("jsonpgz(")
        .and_then(|s| s.strip_suffix(")"))
        .ok_or("Invalid JSONP format")?;
    
    // Parse JSON
    let data: serde_json::Value = serde_json::from_str(json_str)?;
    
    // Extract fields
    Ok(FundInfo {
        code: data["fundcode"].as_str().ok_or("Missing fundcode")?.to_string(),
        name: data["name"].as_str().ok_or("Missing name")?.to_string(),
        net_value: data["gsz"].as_str().and_then(|s| s.parse().ok()),
        update_time: data["gztime"].as_str().map(String::from),
    })
}
```

### Alternatives Considered
- **JavaScript execution**: Use JS engine (unsafe, complex)
- **JSONP library**: No maintained crates, overkill for simple format
- **String replacement**: Simple regex is sufficient and safe

### Error Handling
- Invalid format: Return `Err("数据格式异常")`
- Missing fields: Return `Err("基金代码不存在")`
- Network timeout: Return `Err("网络请求超时，请重试")`

---

## Decision 3: Duplicate Detection Implementation

### Context
Must prevent duplicate fund codes within same list (Principle 5) while allowing cross-list duplication.

### Decision
**Use in-memory HashSet for O(1) duplicate checking**

### Rationale
1. **Performance**: HashSet lookup is O(1) vs O(n) for Vec contains
2. **Simplicity**: Standard library, no external dependencies
3. **Memory**: Minimal overhead for <200 fund codes per list

### Implementation Details

```rust
impl FundList {
    fn contains_fund(&self, fund_code: &str) -> bool {
        self.fund_codes.iter().any(|code| code == fund_code)
    }
    
    fn add_fund(&mut self, fund_code: String) -> Result<(), String> {
        if self.contains_fund(&fund_code) {
            return Err(format!("基金 {} 已在列表中", fund_code));
        }
        self.fund_codes.push(fund_code);
        Ok(())
    }
}
```

For larger lists (>1000 funds), convert to HashSet internally during operations.

### Alternatives Considered
- **Database unique constraint**: Requires SQLite (ruled out for MVP)
- **Bloom filter**: Overkill for small datasets, false positives unacceptable
- **Sorted Vec + binary search**: O(log n) but requires maintaining sort order

---

## Decision 4: Frontend Framework Selection

### Context
Need to choose frontend framework compatible with Tauri.

### Decision
**React 18 with TypeScript and Vite**

### Rationale
1. **Tauri Support**: First-class support in Tauri templates
2. **Ecosystem**: Largest component library ecosystem
3. **TypeScript**: Type safety for Tauri API interactions
4. **Vite**: Fast HMR, excellent DX, built-in in Tauri templates
5. **Team Familiarity**: Most widely known framework (assumption)

### Implementation Details
- Initialize with: `npm create tauri-app` → select React-TS template
- State management: React Context for list state, local state for UI
- Styling: CSS Modules or Tailwind CSS (to be decided in UI implementation)
- UI library: Consider Ant Design or Material-UI for Chinese localization

### Alternatives Considered
- **Vue 3**: Simpler for small apps, but smaller ecosystem
- **Svelte**: Fastest, smallest bundle, but less mature Tauri tooling
- **Vanilla JS**: Too complex for state management needs

---

## Decision 5: Async Runtime and HTTP Client

### Context
Rust backend needs async support for HTTP requests and Tauri commands.

### Decision
**Tokio runtime with reqwest HTTP client**

### Rationale
1. **Tokio**: Standard async runtime for Rust, required by Tauri
2. **reqwest**: Most popular HTTP client, well-maintained, good error handling
3. **Integration**: reqwest built on tokio, seamless integration

### Implementation Details

```toml
# Cargo.toml
tokio = { version = "1.35", features = ["full"] }
reqwest = { version = "0.11", features = ["json"] }
```

Configuration:
- Timeout: 10 seconds (per spec requirement)
- User-Agent: "LeekFund/1.0.0"
- Connection pooling: Default (handled by reqwest)

### Alternatives Considered
- **hyper**: Lower-level, more complex API
- **ureq**: Synchronous, simpler but blocks threads
- **surf**: Less mature, smaller ecosystem

---

## Decision 6: Error Handling Strategy

### Context
Need consistent error handling across Rust backend and user-facing error messages.

### Decision
**Custom error types with user-friendly Chinese messages**

### Rationale
1. **User Experience**: Technical errors translated to actionable messages
2. **Type Safety**: Rust Result types with specific error variants
3. **Debuggability**: Log technical details, show simple messages to users

### Implementation Details

```rust
#[derive(Debug)]
enum FundError {
    NetworkError(String),
    ParseError(String),
    NotFound(String),
    DuplicateFund(String),
    StorageError(String),
}

impl FundError {
    fn user_message(&self) -> String {
        match self {
            Self::NetworkError(_) => "网络连接失败，请检查网络后重试".to_string(),
            Self::ParseError(_) => "数据解析失败，请稍后重试".to_string(),
            Self::NotFound(code) => format!("基金代码 {} 不存在", code),
            Self::DuplicateFund(code) => format!("基金 {} 已在列表中", code),
            Self::StorageError(_) => "数据保存失败，请重试".to_string(),
        }
    }
}
```

Tauri commands return `Result<T, String>` with user_message() as error string.

### Alternatives Considered
- **anyhow**: Too generic, loses type information
- **thiserror**: Good for libraries, overkill here
- **String errors**: Chosen for simplicity in Tauri integration

---

## Decision 7: Data Model Versioning

### Context
Need to handle data format changes in future versions without losing user data.

### Decision
**Include schema version in storage file with migration support**

### Rationale
1. **Forward Compatibility**: Detect old formats and migrate
2. **Graceful Degradation**: Handle unknown versions safely
3. **User Trust**: Never lose user data due to updates

### Implementation Details

```rust
#[derive(Serialize, Deserialize)]
struct StorageFormat {
    version: u32,  // Current: 1
    lists: Vec<FundList>,
    created_at: i64,
    last_modified: i64,
}

fn load_with_migration(path: &Path) -> Result<StorageFormat> {
    let data: StorageFormat = load_json(path)?;
    match data.version {
        1 => Ok(data),
        v if v < 1 => migrate_to_v1(data),
        _ => Err("未知的数据格式版本".into()),
    }
}
```

### Alternatives Considered
- **No versioning**: Risky, breaks on any format change
- **Separate migration files**: Complex for simple use case
- **Database migrations**: Requires SQLite (not MVP)

---

## Decision 8: List Ordering Mechanism

### Context
Users need to reorder lists (Spec FR5: "User can reorder lists").

### Decision
**Position field with reordering operation**

### Rationale
1. **Explicit Order**: Position field stores user's intended order
2. **Persistence**: Order survives restart
3. **Flexibility**: Easy to implement drag-and-drop in UI

### Implementation Details

```rust
struct FundList {
    id: String,
    name: String,
    fund_codes: Vec<String>,
    position: usize,  // Lower number = higher in list
    created_at: i64,
}

fn reorder_lists(list_ids: Vec<String>) -> Result<()> {
    // list_ids is new order (index 0 = first position)
    for (new_position, list_id) in list_ids.iter().enumerate() {
        // Update position field for each list
        update_list_position(list_id, new_position)?;
    }
    save_lists()?;
    Ok(())
}
```

Lists always displayed sorted by position field.

### Alternatives Considered
- **Array order only**: Fragile, requires complex splice operations
- **Linked list**: Overkill, complex persistence
- **Timestamp-based**: Can't support arbitrary reordering

---

## Best Practices Summary

### Rust Backend
1. **Error Handling**: Always use `Result` types, never `unwrap()` in production code
2. **Testing**: Unit test each module, integration test all Tauri commands
3. **Logging**: Use `log` crate with appropriate levels (error, warn, info, debug)
4. **Documentation**: Doc comments for all public functions and structs

### Frontend
1. **Type Safety**: Define TypeScript interfaces for all Tauri command responses
2. **Error Boundaries**: React error boundaries for graceful failure handling
3. **Loading States**: Show skeleton loaders during async operations
4. **Accessibility**: ARIA labels, keyboard navigation, focus management

### Testing
1. **Unit Tests**: Rust modules with >80% coverage
2. **Integration Tests**: All Tauri commands with success/error paths
3. **E2E Tests**: User workflows from spec (Tauri's test framework or Playwright)
4. **Performance Tests**: Large list handling, concurrent operations

### Security
1. **Input Validation**: Validate fund codes (6 digits only) before API calls
2. **CSP Configuration**: Restrict to required domains only
3. **Data Sanitization**: Escape user input in list names
4. **HTTPS Only**: All external API calls use HTTPS

---

## Research Completion Checklist

- [x] Storage backend chosen and justified (JSON file)
- [x] API response parsing strategy defined (JSONP extraction)
- [x] Duplicate detection approach decided (HashSet)
- [x] Frontend framework selected (React + TS + Vite)
- [x] Async runtime and HTTP client chosen (Tokio + reqwest)
- [x] Error handling strategy defined (Custom error types)
- [x] Data versioning approach planned (Schema version field)
- [x] List ordering mechanism designed (Position field)
- [x] Best practices documented for all areas
- [x] All technical unknowns from plan resolved

## Status: ✅ Research Phase Complete

All technical decisions made with clear rationale. Ready to proceed to Phase 1 (Design & Contracts).

