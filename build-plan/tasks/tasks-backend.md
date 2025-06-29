# Backend Implementation Tasks

Based on the backend.md specification, here are the tasks required to implement the OrgDesk backend.

## Relevant Files

- `src-tauri/Cargo.toml` - Rust dependencies including tauri, orgize, notify, serde crates. ✅ Created & Configured
- `src-tauri/src/main.rs` - Main Tauri application entry point and window configuration. ✅ Created & Updated
- `src-tauri/src/lib.rs` - Library module exports and core setup. ✅ Created & Updated
- `src-tauri/src/commands.rs` - Tauri command handlers for IPC communication with frontend. ✅ Created
- `src-tauri/src/models/task.rs` - Task data structure and related models.
- `src-tauri/src/models/mod.rs` - Module declarations for data models. ✅ Created
- `src-tauri/src/parser/org_parser.rs` - Org file parsing logic using orgize crate.
- `src-tauri/src/parser/mod.rs` - Parser module declarations. ✅ Created
- `src-tauri/src/store/task_store.rs` - In-memory task cache and store management.
- `src-tauri/src/store/mod.rs` - Store module declarations. ✅ Created
- `src-tauri/src/watcher/file_watcher.rs` - File system watching using notify crate.
- `src-tauri/src/watcher/mod.rs` - Watcher module declarations. ✅ Created
- `src-tauri/tauri.conf.json` - Tauri configuration file for app metadata and permissions. ✅ Created
- `src-tauri/src/tests/` - Directory for Rust unit tests.
- `src/App.tsx` - React frontend with Tauri communication test interface. ✅ Updated

### Notes

- Tauri uses a `src-tauri/` directory for all Rust backend code.
- Run `cargo test` from the `src-tauri/` directory to execute Rust tests.
- Use `cargo tauri dev` to run the app in development mode.
- Use `cargo tauri build` to create production builds.

## Tasks

- [x] 1.0 Set up Tauri Infrastructure
  - [x] 1.1 Install Tauri CLI and add Tauri to the project
  - [x] 1.2 Initialize Tauri configuration and directory structure
  - [x] 1.3 Configure Cargo.toml with required dependencies (serde, serde_json)
  - [x] 1.4 Set up basic main.rs and lib.rs files
  - [x] 1.5 Test basic Tauri app launch and React-Rust communication
  - [x] 1.6 Configure tauri.conf.json for desktop app settings

- [ ] 2.0 Implement Org File Parser with orgize
  - [x] 2.1 Add orgize crate dependency to Cargo.toml
  - [x] 2.2 Create parser module structure (src/parser/mod.rs)
  - [ ] 2.3 Implement basic org file reading functionality
  - [ ] 2.4 Create task extraction from org headlines
  - [ ] 2.5 Parse TODO states, tags, priority, and dates
  - [ ] 2.6 Implement org file writing/modification capabilities
  - [ ] 2.7 Add error handling for malformed org files

- [ ] 3.0 Create Task Store and Data Models  
  - [ ] 3.1 Define Task struct with all required fields (id, title, tags, etc.)
  - [ ] 3.2 Create supporting data models (TodoState enum, Priority enum)
  - [ ] 3.3 Implement task store with HashMap for file-based caching
  - [ ] 3.4 Add task CRUD operations (create, read, update, delete)
  - [ ] 3.5 Implement task filtering and search functionality
  - [ ] 3.6 Add data validation and serialization support

- [ ] 4.0 Build File Watcher System
  - [ ] 4.1 Add notify crate dependency for file watching
  - [ ] 4.2 Create file watcher module structure
  - [ ] 4.3 Implement directory watching for .org files
  - [ ] 4.4 Handle file change events (create, modify, delete)
  - [ ] 4.5 Integrate watcher with task store updates
  - [ ] 4.6 Add debouncing to prevent excessive parsing on rapid changes

- [ ] 5.0 Implement API Endpoints and Command Bus
  - [ ] 5.1 Create command handlers module for Tauri IPC
  - [ ] 5.2 Implement createTask command with JSON request/response
  - [ ] 5.3 Implement updateTask and deleteTask commands
  - [ ] 5.4 Implement listTasks command with filtering support
  - [ ] 5.5 Implement getAgendaRange command for date-based queries
  - [ ] 5.6 Add proper error handling and JSON error responses
  - [ ] 5.7 Wire all commands to Tauri app in main.rs 