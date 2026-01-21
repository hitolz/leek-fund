# Quickstart Guide: Fund List Management

## Overview

This guide helps developers get started with implementing the Fund List Management feature. It provides step-by-step instructions for setting up the development environment and understanding the architecture.

---

## Prerequisites

### System Requirements

- **Operating System**: macOS, Windows, or Linux
- **Rust**: Version 1.70 or higher
- **Node.js**: Version 18 or higher
- **Package Manager**: npm or pnpm

### Install Tauri CLI

```bash
cargo install tauri-cli
# or
npm install -g @tauri-apps/cli
```

### Verify Installation

```bash
cargo --version
# Should output: cargo 1.70.x or higher

node --version
# Should output: v18.x or higher

cargo tauri --version
# Should output: @tauri-apps/cli 1.5.x
```

---

## Project Structure

After initialization, the project structure will be:

```
leek-fund/
├── src/                      # Frontend source (React + TypeScript)
│   ├── components/           # React components
│   │   ├── SearchBar.tsx
│   │   ├── FundInfoCard.tsx
│   │   ├── ListsPanel.tsx
│   │   └── ListDetailView.tsx
│   ├── hooks/                # Custom React hooks
│   │   └── useTauriCommands.ts
│   ├── types/                # TypeScript type definitions
│   │   └── index.ts
│   ├── App.tsx               # Root component
│   └── main.tsx              # Entry point
├── src-tauri/                # Rust backend source
│   ├── src/
│   │   ├── modules/          # Core modules
│   │   │   ├── fund_api.rs   # Fund API integration
│   │   │   ├── storage.rs    # Data persistence
│   │   │   ├── list_manager.rs  # List operations
│   │   │   └── mod.rs
│   │   ├── commands.rs       # Tauri command handlers
│   │   ├── models.rs         # Data structures
│   │   └── main.rs           # Rust entry point
│   ├── Cargo.toml            # Rust dependencies
│   └── tauri.conf.json       # Tauri configuration
├── specs/                    # Design documentation
│   └── 001-fund-list-management/
│       ├── spec.md           # Feature specification
│       ├── plan.md           # Implementation plan
│       ├── research.md       # Technical decisions
│       ├── data-model.md     # Data structures
│       └── contracts/        # API contracts
└── package.json              # Frontend dependencies
```

---

## Quick Start: 5-Minute Setup

### Step 1: Initialize Tauri Project

```bash
cd /Users/hitol/code/ai/leek-fund
npm create tauri-app
# Select:
# - Project name: leek-fund
# - Choose frontend: React
# - Add TypeScript: Yes
# - Package manager: npm
```

### Step 2: Install Dependencies

```bash
# Frontend dependencies
npm install

# Additional frontend packages
npm install @tauri-apps/api

# Rust dependencies (add to src-tauri/Cargo.toml, then run)
cd src-tauri
cargo build
```

### Step 3: Configure Tauri

Edit `src-tauri/tauri.conf.json`:

```json
{
  "tauri": {
    "allowlist": {
      "http": {
        "all": false,
        "request": true,
        "scope": ["http://fundgz.1234567.com.cn/**"]
      }
    }
  }
}
```

### Step 4: Run Development Server

```bash
# From project root
npm run tauri dev
```

Your app should open automatically!

---

## Architecture Overview

### Data Flow Diagram

```
┌─────────────────────────────────────────────────────────┐
│                     Frontend (React)                     │
│  ┌────────────┐  ┌──────────────┐  ┌────────────────┐  │
│  │ SearchBar  │  │ ListsPanel   │  │ ListDetailView │  │
│  └─────┬──────┘  └──────┬───────┘  └────────┬───────┘  │
│        │                 │                    │          │
│        └─────────────────┴────────────────────┘          │
│                          │                               │
│                ┌─────────▼─────────┐                     │
│                │  Tauri Commands   │                     │
│                │  (invoke calls)   │                     │
│                └─────────┬─────────┘                     │
└──────────────────────────┼───────────────────────────────┘
                           │ IPC
┌──────────────────────────▼───────────────────────────────┐
│                    Rust Backend                          │
│  ┌──────────────┐  ┌──────────────┐  ┌───────────────┐  │
│  │  fund_api    │  │list_manager  │  │   storage     │  │
│  │  - HTTP      │  │  - CRUD ops  │  │  - JSON I/O   │  │
│  │  - Parse     │  │  - Dedup     │  │  - Validate   │  │
│  └──────┬───────┘  └──────┬───────┘  └───────┬───────┘  │
│         │                  │                  │          │
│         └──────────────────┴──────────────────┘          │
└──────────────────────────────────────────────────────────┘
         │                                      │
         ▼                                      ▼
  External API                           Local Storage
  fundgz.1234567.com.cn            ~/.local/share/leek-fund/
```

### Key Components

1. **Frontend (React)**:
   - UI components for user interaction
   - Calls Tauri commands via `invoke()`
   - Manages local UI state only

2. **Tauri Commands Layer**:
   - Bridge between frontend and backend
   - Async command handlers
   - Type-safe serialization

3. **Rust Backend**:
   - **fund_api**: HTTP client for external fund data
   - **list_manager**: Business logic for list operations
   - **storage**: JSON file persistence

4. **External Dependencies**:
   - **Fund API**: Real-time fund information
   - **Local Storage**: User's lists and preferences

---

## Development Workflow

### 1. Backend Development (Rust)

#### Add Dependencies to Cargo.toml

```toml
[dependencies]
tauri = { version = "1.5", features = ["shell-open"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
reqwest = { version = "0.11", features = ["json"] }
tokio = { version = "1.35", features = ["full"] }
uuid = { version = "1.6", features = ["v4", "serde"] }
chrono = { version = "0.4", features = ["serde"] }
```

#### Create Module Structure

```bash
cd src-tauri/src
mkdir modules
touch modules/mod.rs
touch modules/fund_api.rs
touch modules/storage.rs
touch modules/list_manager.rs
touch models.rs
touch commands.rs
```

#### Implement Data Models (models.rs)

```rust
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FundInfo {
    pub code: String,
    pub name: String,
    pub net_value: Option<f64>,
    pub update_time: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FundList {
    pub id: String,
    pub name: String,
    pub fund_codes: Vec<String>,
    pub created_at: i64,
    pub position: usize,
}
```

#### Register Tauri Commands (main.rs)

```rust
fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            commands::search_fund,
            commands::get_all_lists,
            commands::create_list,
            commands::rename_list,
            commands::delete_list,
            commands::add_fund_to_list,
            commands::remove_fund_from_list,
            commands::get_list_funds,
            commands::reorder_lists,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```

### 2. Frontend Development (React)

#### Define TypeScript Types (src/types/index.ts)

```typescript
export interface FundInfo {
  code: string;
  name: string;
  net_value: number | null;
  update_time: string | null;
}

export interface FundList {
  id: string;
  name: string;
  fund_codes: string[];
  created_at: number;
  position: number;
}
```

#### Create Tauri Hooks (src/hooks/useTauriCommands.ts)

```typescript
import { invoke } from '@tauri-apps/api/tauri';
import { FundInfo, FundList } from '../types';

export function useTauriCommands() {
  const searchFund = async (code: string): Promise<FundInfo> => {
    return await invoke('search_fund', { code });
  };

  const getAllLists = async (): Promise<FundList[]> => {
    return await invoke('get_all_lists');
  };

  // ... other commands

  return {
    searchFund,
    getAllLists,
    // ... export others
  };
}
```

#### Create Components

Example SearchBar component:

```typescript
import { useState } from 'react';
import { useTauriCommands } from '../hooks/useTauriCommands';

export function SearchBar() {
  const [code, setCode] = useState('');
  const [loading, setLoading] = useState(false);
  const [fund, setFund] = useState(null);
  const { searchFund } = useTauriCommands();

  const handleSearch = async () => {
    if (code.length !== 6) return;
    
    setLoading(true);
    try {
      const result = await searchFund(code);
      setFund(result);
    } catch (error) {
      alert(error);
    } finally {
      setLoading(false);
    }
  };

  return (
    <div>
      <input
        type="text"
        value={code}
        onChange={(e) => setCode(e.target.value)}
        maxLength={6}
        placeholder="输入6位基金代码"
      />
      <button onClick={handleSearch} disabled={loading}>
        {loading ? '查询中...' : '查询'}
      </button>
      {fund && <div>{fund.name}</div>}
    </div>
  );
}
```

---

## Testing Strategy

### Rust Unit Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fund_list_add_duplicate() {
        let mut list = FundList {
            id: "test".to_string(),
            name: "Test".to_string(),
            fund_codes: vec!["001632".to_string()],
            created_at: 0,
            position: 0,
        };

        let result = list.add_fund("001632".to_string());
        assert!(result.is_err());
    }
}
```

Run tests:
```bash
cd src-tauri
cargo test
```

### Frontend Tests

```typescript
import { render, screen } from '@testing-library/react';
import { SearchBar } from './SearchBar';

test('renders search input', () => {
  render(<SearchBar />);
  const input = screen.getByPlaceholderText('输入6位基金代码');
  expect(input).toBeInTheDocument();
});
```

Run tests:
```bash
npm test
```

---

## Common Tasks

### Task 1: Add a New Tauri Command

1. Define function in `src-tauri/src/commands.rs`:
```rust
#[tauri::command]
pub async fn my_command(param: String) -> Result<String, String> {
    Ok(format!("Received: {}", param))
}
```

2. Register in `main.rs`:
```rust
.invoke_handler(tauri::generate_handler![
    commands::my_command,
    // ... other commands
])
```

3. Call from frontend:
```typescript
const result = await invoke<string>('my_command', { param: 'test' });
```

### Task 2: Debug Rust Backend

Add debug logging:
```rust
println!("Debug: {:?}", variable);
```

View logs in terminal where `npm run tauri dev` is running.

### Task 3: Build for Production

```bash
npm run tauri build
```

Outputs:
- macOS: `.app` in `src-tauri/target/release/bundle/macos/`
- Windows: `.exe` in `src-tauri/target/release/bundle/msi/`
- Linux: AppImage in `src-tauri/target/release/bundle/appimage/`

---

## Troubleshooting

### Issue: "Command not found"

**Cause**: Command not registered in `main.rs`

**Fix**: Add command to `generate_handler![]` macro

### Issue: Network timeout

**Cause**: External API slow or blocked

**Fix**: Increase timeout in reqwest client configuration:
```rust
let client = reqwest::Client::builder()
    .timeout(Duration::from_secs(10))
    .build()?;
```

### Issue: Storage file corrupted

**Cause**: Interrupted write operation

**Fix**: Implemented atomic write (already in plan). Manual fix:
```bash
rm ~/.local/share/leek-fund/lists.json
# App will create new file on next start
```

### Issue: CORS error in dev mode

**Cause**: CSP not configured

**Fix**: Update `tauri.conf.json` with API domain in scope

---

## Resources

### Documentation
- [Tauri Docs](https://tauri.app/v1/guides/)
- [React Docs](https://react.dev/)
- [Rust Book](https://doc.rust-lang.org/book/)

### Key Files Reference
- `specs/001-fund-list-management/spec.md` - Feature requirements
- `specs/001-fund-list-management/plan.md` - Implementation plan
- `specs/001-fund-list-management/data-model.md` - Data structures
- `specs/001-fund-list-management/contracts/tauri-commands.md` - API contract

### External APIs
- Fund data: `http://fundgz.1234567.com.cn/js/{code}.js`
- Format: JSONP (extract JSON from `jsonpgz(...)` wrapper)

---

## Next Steps

1. ✅ Review all design documents in `specs/001-fund-list-management/`
2. ✅ Set up development environment (this guide)
3. ⏭️ Implement Rust backend modules (start with `storage.rs`)
4. ⏭️ Implement Tauri commands (in `commands.rs`)
5. ⏭️ Build frontend components (start with `SearchBar.tsx`)
6. ⏭️ Integration testing
7. ⏭️ Production build and distribution

---

## Quick Reference Commands

```bash
# Development
npm run tauri dev          # Start dev server with hot reload

# Testing
cargo test                 # Run Rust tests
npm test                   # Run frontend tests

# Building
npm run tauri build        # Production build (all platforms)

# Debugging
cargo check                # Check Rust code without building
npm run lint               # Lint frontend code
```

---

## Support

For questions or issues during implementation:
1. Review constitution principles in `.specify/memory/constitution.md`
2. Check feature spec for requirements clarification
3. Consult API contracts for command signatures
4. Review research.md for technical decisions rationale

---

**Status**: Ready for implementation. All design documents complete.

**Estimated Implementation Time**: 3-5 days for experienced developer

**Priority Order**: Backend → Commands → Frontend → Testing → Polish

