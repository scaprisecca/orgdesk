# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project

OrgDesk is a Tauri desktop app that brings Emacs Org Mode task/note management to Linux (targeting a Flatpak build). Rust backend (`src-tauri/`) parses and watches `.org` files; React/TypeScript frontend (`src/`) renders the UI. IPC between them is Tauri's JSON command bus.

## Commands

**Frontend (run from repo root):**
```bash
pnpm install       # install JS deps
pnpm dev           # vite dev server only (no Tauri window)
pnpm build         # tsc -b && vite build
pnpm lint          # eslint .
```

**Full app (Tauri shell + React):**
```bash
cargo tauri dev     # hot-reloads Rust + React together
cargo tauri build --target x86_64-unknown-linux-gnu   # production build
```
Note: the README references `yarn tauri dev`, but the lockfile in this repo is `pnpm-lock.yaml` — use pnpm/cargo-tauri, not yarn.

**Backend (Rust, run from `src-tauri/`):**
```bash
cargo test                          # run all Rust tests
cargo test test_name                # run a single test by name
cargo test --lib parser::           # run tests in a module (e.g. the parser module)
```
There is no JS test runner configured yet (no Jest/Vitest in `package.json`), despite `build-plan/tasks/tasks-frontend.md` referencing `pnpm exec jest`.

## Architecture

### Backend (`src-tauri/src/`)
- `main.rs` — **the real desktop entry point.** It builds the `TaskStore` and `FileWatcher`, calls `.manage(AppState { store, parser })`, and registers the full command set (`parse_org_file`, `create_task`, `update_task`, `delete_task`, `list_tasks`, `get_agenda_range`).
- `lib.rs` — exposes `run()`, gated behind `#[cfg_attr(mobile, tauri::mobile_entry_point)]`. It builds a *separate*, smaller Tauri app (only `hello_from_rust`, `parse_org_content`, `parse_org_file`, no `AppState`) and is **not** the code path used on desktop. Don't assume changes to `lib.rs::run()` affect the running desktop app — wire new commands into `main.rs` instead.
- `parser/org_parser.rs` — wraps the `orgize` crate. `OrgParser::parse_content`/`parse_file` walk headline events into `OrgHeadline` structs (title, level, todo state, tags, priority, scheduled/deadline, properties, and the original `TextRange` in the source file). `OrgHeadline::to_org_string()` + `OrgParser::update_headline()` write edits back into the `.org` file in place using that stored range — this round-trip is how in-place task edits are meant to persist to disk.
- `models/task.rs` — `Task` is the IPC-facing struct; it's built `From<&OrgHeadline>` (note: this always regenerates a fresh `Uuid`, so parsing the same file twice currently produces different task IDs — there's no stable identity yet).
- `store/task_store.rs` — `TaskStore` is a `HashMap<file_path, Vec<Task>>`, i.e. tasks are cached per source file. Any watcher event reparses a whole file and replaces its task list wholesale.
- `watcher/file_watcher.rs` — debounced (`notify-debouncer-full`, 1s) recursive watch; on create/modify it reparses and calls `store.add_tasks_from_file`, on remove it calls `store.remove_tasks_by_file`. Currently watches `.` (cwd) from `main.rs`, not a user-configured folder list.
- `commands.rs` — Tauri command handlers. Several are stubs/partial: `create_task` builds a `Task` but never actually appends it to an org file or the store; `update_task`/`delete_task` mutate the in-memory `TaskStore` but don't write the change back through `OrgParser::update_headline`; `get_agenda_range` ignores its date args and returns `[]`. Treat these as scaffolding, not finished behavior.

### Frontend (`src/`)
- State is Zustand, split into slices under `src/stores/` (`tasksSlice`, `agendaSlice`, `uiSlice`, `settingsSlice`) and re-exported from `src/stores/index.ts`. Components import hooks like `useTasksSlice`/`useUiSlice` from `./stores`.
- `src/lib/api.ts` is the sole IPC boundary — wraps `@tauri-apps/api/core`'s `invoke()`. `getTasks()` silently falls back to hardcoded mock data on invoke failure, which can mask a broken/unwired backend command during development.
- `App.tsx` composes `MainLayout` (three resizable panes via `react-resizable-panels`: Toolbar, TaskListPane, AgendaPane) plus modals/dialogs (`QuickCaptureModal`, `RefileDialog`, `AgendaBuilderDialog`, `SettingsDialog`) driven by `uiSlice.activeModal`.
- Styling is Tailwind CSS v4 (via `@tailwindcss/postcss`); UI primitives come from Radix (`@radix-ui/react-checkbox`, `-dialog`, `-dropdown-menu`) plus `lucide-react` icons.
- `RefileDialog` is meant to use `Fuse.js` for fuzzy search against refile targets.

### Data flow
UI action → Zustand slice does an optimistic update → `api.ts` invokes a Tauri command → Rust mutates `TaskStore` and (intended, not fully implemented) the underlying `.org` file → file watcher would pick up on-disk changes and reconcile the store. Several links in this chain (org-file persistence on create/update/delete, agenda range filtering, stable task IDs) are not yet implemented — check `commands.rs` before assuming a command is fully functional.

## Planning docs

- `build-plan/product_requirements.md`, `build-plan/backend.md`, `build-plan/frontend.md`, `build-plan/third_party_libraries.md` — the original specs.
- `build-plan/tasks/tasks-backend.md`, `build-plan/tasks/tasks-frontend.md` — checklist task lists generated from those specs; both are currently marked fully complete, but the code-level gaps above (esp. in `commands.rs`) haven't caught up with the checklist.
- `.cursor/rules/generate-tasks.mdc` and `.cursor/rules/process-tasks.mdc` define this repo's PRD → task-list → implementation workflow: one sub-task at a time, mark `[x]` on completion, pause for explicit user go-ahead before starting the next sub-task, keep each task file's "Relevant Files" section current.
