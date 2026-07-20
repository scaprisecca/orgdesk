# OrgDesk Code Review — Findings & Fix Plan

Reviewed: 2026-07-16 (branch `main`, commit `16f7c31`).
Scope: full Rust backend (`src-tauri/`), React frontend (`src/`), build configuration.

Each finding lists **what's wrong**, **why it matters**, and **how to fix it**, ordered by severity. A suggested order of attack is at the bottom.

---

## Critical — the app cannot currently build or run correctly

### C1. `pnpm build` fails — 10 TypeScript errors — DONE

`tsc -b` fails with `TS6133 'React' is declared but its value is never read` in 10 files
(`src/App.tsx:1`, `src/components/AgendaPane.tsx:1`, `src/components/Toolbar.tsx:1`, all dialogs/modals, `src/components/ui/UpdatePrompt.tsx:1`), plus unused `useTasksSlice`/`Task` imports in `src/components/dialogs/RefileDialog.tsx:3-4`.

**Why:** `tsconfig.app.json` uses the modern `"jsx": "react-jsx"` transform (no `React` import needed) together with `"noUnusedLocals": true`. Any `import React from 'react'` that isn't otherwise used is a hard build error, so no production build is possible.

**Fix:**
1. In every component, delete the bare `import React from 'react';` line.
2. Where React types/hooks are actually used, import only those:
   - `App.tsx`: `import { useState, useEffect } from 'react';` (already partially correct — just drop `React,`).
   - `MainLayout.tsx` uses `React.ReactNode` — change to `import type { ReactNode } from 'react';` and use `ReactNode`.
   - `TaskListPane.tsx` uses `React.KeyboardEvent` — `import type { KeyboardEvent } from 'react';`.
3. In `RefileDialog.tsx` remove the unused `useTasksSlice` and `Task` imports (lines 3–4).
4. Re-run `pnpm build` until clean.

### C2. Tailwind v3 directives with Tailwind v4 — styling pipeline is broken — DONE

`src/index.css:1-3` starts with:
```css
@tailwind base;
@tailwind components;
@tailwind utilities;
```
but the project uses **Tailwind CSS v4** via `@tailwindcss/postcss` (`package.json`, `postcss.config.js`). In v4 the `@tailwind` directives were removed and cause a build error (or, at best, produce no utility classes) — so all the `className="..."` styling across the UI doesn't render.

**Fix:**
1. Replace the three `@tailwind` lines in `src/index.css` with:
   ```css
   @import "tailwindcss";
   ```
2. Delete `tailwind.config.js` (v4 auto-detects content; the v3-style config is ignored unless you opt in with `@config`). If you later need customization, use the v4 CSS-first `@theme { ... }` syntax inside `index.css`.
3. Remove `autoprefixer` from `postcss.config.js` and `package.json` — Tailwind v4 handles vendor prefixing itself:
   ```js
   export default { plugins: { '@tailwindcss/postcss': {} } };
   ```
4. While you're in `index.css`: the rest of the file is the default Vite starter theme (`:root { color: rgba(255,255,255,.87); background-color: #242424; ... }`, plus button/body rules). These fight with the Tailwind `bg-white dark:bg-gray-800` classes the components use. Strip the starter rules down to just fonts/global resets you actually want.

### C3. Runtime crash: Zustand v5 object selector without `useShallow` — DONE

`src/components/TaskListPane.tsx:8-13`:
```ts
const { updateTaskTitle, toggleTaskState } = useTasksSlice(
  (state) => ({ updateTaskTitle: state.updateTaskTitle, toggleTaskState: state.toggleTaskState }),
);
```
The selector returns a **new object on every call**. With Zustand v5 (which uses React's `useSyncExternalStore`), this triggers "The result of getSnapshot should be cached to avoid an infinite loop" and infinite re-renders as soon as a `TaskItem` mounts.

**Fix (pick one):**
- Two separate primitive selectors (simplest, matches the rest of the codebase):
  ```ts
  const updateTaskTitle = useTasksSlice((s) => s.updateTaskTitle);
  const toggleTaskState = useTasksSlice((s) => s.toggleTaskState);
  ```
- Or wrap with `useShallow` from `zustand/react/shallow`:
  ```ts
  import { useShallow } from 'zustand/react/shallow';
  const { updateTaskTitle, toggleTaskState } = useTasksSlice(
    useShallow((s) => ({ updateTaskTitle: s.updateTaskTitle, toggleTaskState: s.toggleTaskState })),
  );
  ```

### C4. Data loss: `OrgParser::update_headline` deletes body text and child headlines — DONE

`src-tauri/src/parser/org_parser.rs:200-216` replaces `old_headline.range` with `new_headline.to_org_string()`.

I verified in the orgize 0.10.0-alpha.10 source that a `Headline`'s `text_range()` covers the **entire subtree node** — the headline line *plus* its section (body text under the headline) *plus* all nested sub-headlines (sub-headlines are `AstChildren` of the `HEADLINE` node). `to_org_string()` (`org_parser.rs:68-97`) only regenerates the headline line, SCHEDULED/DEADLINE, and the properties drawer.

**Consequence:** editing a task title on a headline that has notes underneath it, or sub-tasks, silently deletes all of that content from the user's `.org` file. For a note-taking app this is the single most dangerous bug in the codebase. (The existing test `test_update_headline` doesn't catch it because its test headline has no body or children.)

**Fix:**
1. Change the strategy from "replace the whole subtree range" to "replace only what you regenerate". Two workable options:
   - **Option A (recommended, minimal):** store two ranges on `OrgHeadline` — the full node range and the range of just the *headline line + planning line + properties drawer* (you can compute the end as the start of the headline's first non-planning/non-drawer section content or first child headline). Replace only that prefix range.
   - **Option B:** keep the parsed `orgize::Org` document around and mutate the AST via orgize's mutation API (`replace_range` on the *token* ranges of title/keyword/priority individually), then `org.to_org()`.
2. Whichever you choose, **add regression tests first**:
   ```rust
   // headline with body text
   "* TODO Task\n  Some notes that must survive.\n"
   // headline with a child
   "* TODO Parent\n** TODO Child\n"
   ```
   Assert after `update_headline` that the notes/child are still present in the file.
3. Also note `to_org_string()` re-emits properties from a `HashMap`, so property **order is randomized** and formatting is normalized — with Option A restrict the replaced range so this only touches lines you intend to rewrite; long term, preserve the original drawer text unless a property actually changed.

### C5. `parse_org_content` is invoked by the frontend but not registered in the desktop app — DONE

`src/lib/api.ts:27` calls `invoke("parse_org_content", ...)`, but `src-tauri/src/main.rs:27-34` (the real desktop entry point) does **not** register `parse_org_content` — only `lib.rs::run()` does, and that builder never runs on desktop. Any call to `parseOrgContent()` fails at runtime with "command not found".

**Fix:** part of C6 below — consolidate to a single command list that includes everything the frontend calls.

### C6. Two divergent Tauri applications (`main.rs` vs `lib.rs`) — DONE

`src-tauri/src/main.rs` builds the real app (AppState + 6 commands, no log plugin). `src-tauri/src/lib.rs:9-19` builds a *different* app (3 commands, log plugin, no AppState) that desktop never executes. This has already caused C5 and will keep causing "I added a command but it doesn't work" confusion.

**Fix (standard Tauri 2 layout):**
1. Move everything from `main.rs::main()` into `lib.rs::run()`: `TaskStore`/`OrgParser` construction, `FileWatcher` setup, `.manage(AppState{...})`, the log plugin, and a **single** `generate_handler![]` listing all commands (`hello_from_rust`, `parse_org_content`, `parse_org_file`, `create_task`, `update_task`, `delete_task`, `list_tasks`, `get_agenda_range`).
2. Reduce `main.rs` to:
   ```rust
   fn main() { orgdesk_lib::run() }
   ```
3. Keep the `FileWatcher` alive by moving it into managed state (see H3) rather than a local variable, so it can be reconfigured at runtime.

---

## High — core features silently don't work end-to-end

### H1. The IPC data contract is mismatched — backend tasks can't be displayed correctly

Backend `Task` (`src-tauri/src/models/task.rs:22-32`) vs frontend `Task` (`src/stores/tasksSlice.ts:4-11`):

| Field | Backend sends | Frontend expects |
|---|---|---|
| `state` | `"Todo"`, `"Done"`, `"InProgress"`, ... (serde default enum names) | `'TODO' \| 'DONE'` |
| `scheduled`/`deadline` | raw org timestamp, e.g. `"<2024-08-01 Thu>"` | `"YYYY-MM-DD"` |
| hierarchy | flat list (no children) | nested `children?: Task[]` |
| extra fields | `tags`, `priority`, `properties`, `file_path` | not modeled |

Consequences: a `DONE` task never renders as done (`'Done' !== 'DONE'`); the agenda (`AgendaPane.tsx:41-43`) compares `task.scheduled === '2026-07-16'` against `"<2026-07-16 Wed>"` and **never matches anything**; the outline hierarchy is lost.

**Fix:**
1. Decide the wire format once, on the Rust side:
   ```rust
   #[derive(Serialize, Deserialize, ...)]
   #[serde(rename_all = "SCREAMING_SNAKE_CASE")]
   pub enum TodoState { Todo, Done, InProgress, Someday, Canceled }
   ```
   (`rename_all` makes serde emit `"TODO"`, `"DONE"`, `"IN_PROGRESS"`, ...)
2. Convert timestamps to ISO dates at the parser boundary. orgize is already built with the `chrono` feature (`Cargo.toml:27`) — use the timestamp's parsed date instead of `ts.raw()` in `org_parser.rs:151-158`, and send `"2024-08-01"` strings (keep the raw string too if you want to preserve repeaters like `+1w` later).
3. Represent hierarchy. Simplest robust approach: keep the backend list flat but add `level: u8` and `parent_id: Option<Uuid>` (or just `level`), and build the tree on the frontend from levels — or send `children` nested from Rust. Either way, both sides must agree.
4. Mirror the full struct in `tasksSlice.ts` (`tags`, `priority`, `filePath`, `level`). Long term, consider generating TS types from the Rust structs with `ts-rs` or `tauri-specta` so this class of bug can't recur.

### H2. Task IDs change on every parse — update/delete by ID is built on sand — DONE

`Task::from(&OrgHeadline)` (`models/task.rs:52`) calls `Uuid::new_v4()` every time. Every file-watcher reparse (`store/task_store.rs:17-29` replaces the whole file's task list) mints all-new IDs. So:
- Any ID the frontend is holding becomes stale the moment the file changes on disk.
- `update_task`/`delete_task` by UUID will return `NotFound` after any reparse.
- Parsing the same file twice yields "different" tasks.

**Fix:**
1. Use the org-standard `:ID:` property as the stable identity: if `headline.properties` contains `ID`, parse/derive the task id from it.
2. When a headline has no `:ID:`, either (a) generate one and write it back into the file's properties drawer (this is exactly what Emacs `org-id` does), or (b) fall back to a deterministic id derived from `file_path + headline path/offset` (stable enough between edits, no file writes).
3. Also store the headline's `range`/line info on `Task` (or keep an `OrgHeadline` alongside it in the store) so `update_task` can find the right headline to rewrite — today `From<&OrgHeadline>` throws the range away, which is why the file write-back in `commands.rs` was never finishable.

### H3. `create_task` / `update_task` / `delete_task` don't persist anything; `get_agenda_range` returns `[]`

`src-tauri/src/commands.rs`:
- `create_task` (77–102) builds a `Task` and returns it; the store write is commented out and nothing is appended to any `.org` file. The task evaporates.
- `update_task` (104–118) mutates only the in-memory store — and returns the **old** task (`store.update_task` returns the pre-replace value via `mem::replace`, `task_store.rs:56`), so a frontend that trusts the response would render stale data.
- `delete_task` (120–133) removes from memory only; the headline stays in the file, and the next watcher reparse resurrects the task.
- `get_agenda_range` (155–163) ignores its arguments.

**Fix (make the .org files the source of truth):**
1. `create_task`: resolve a target file (an "inbox" file from settings — see H5; never a relative `"new_tasks.org"`, which resolves against whatever the process cwd is), append `Task::to_org_string()`-style text (`\n* TODO {title}\n`), fsync, then either reparse that file into the store immediately or let the watcher reconcile. Return the task parsed back from disk so the ID is the persisted one.
2. `update_task`: look up the stored headline (per H2 step 3), build the new `OrgHeadline`, call `OrgParser::update_headline` (after C4 is fixed), then update the store, and return the **new** task.
3. `delete_task`: implement a `delete_headline` in the parser (remove the headline's subtree range — here the full-subtree `text_range()` is actually what you want), write the file, remove from store.
4. `get_agenda_range`: parse `start_date`/`end_date` with `chrono::NaiveDate::parse_from_str(.., "%Y-%m-%d")`, and filter `store.get_all_tasks()` by parsed `scheduled`/`deadline` (after H1 step 2 those are real dates). Return an error for unparseable dates instead of `Ok(vec![])`.
5. Add a store test + command test for each once implemented.

### H4. Frontend mutations never call the backend, and errors are masked by mock data

- `tasksSlice.ts` `addTask`/`updateTaskTitle`/`toggleTaskState` mutate only local state — no `invoke` ever happens, so nothing the user does reaches Rust or disk.
- `src/lib/api.ts:15-21`: `getTasks()` catches any invoke failure and silently returns `[{ id: 'mock1', ... }]`. Combined with the above, the app can *look* like it works while the whole backend is unwired (which is exactly the current state).

**Fix:**
1. Delete the mock fallback. Let `getTasks()` throw, and surface the error in the UI (even a simple error banner). During development a visible failure is a feature.
2. Wire the slice actions through `api.ts`:
   ```ts
   toggleTaskState: async (id) => {
     set(optimisticToggle(id));                 // optimistic
     try { await api.updateTask(...); }         // persist
     catch (e) { set(revertToggle(id)); ... }   // rollback + surface error
   }
   ```
3. Add `createTask`, `updateTask`, `deleteTask`, `getAgendaRange` wrappers in `api.ts` with real types (no `Promise<any>`).
4. Listen for a backend "tasks changed" event (H6) to refresh after watcher reconciliation.

### H5. File watching is wrong-scope and disconnected from settings; no initial scan

- `main.rs:19` watches `"."` — the process cwd. Under `cargo tauri dev` that's `src-tauri/`, so it recursively watches the Cargo `target/` build tree (thousands of files) and none of your actual org folders. In a packaged Flatpak the cwd is undefined/read-only.
- `settingsSlice.ts` has `watchedFolders`, but it never reaches the backend, isn't persisted, and `SettingsDialog.tsx:10-14` adds a fake path instead of opening a folder picker.
- The store starts empty and nothing does an initial scan — `list_tasks` returns `[]` until a file happens to change while the app is running.

**Fix:**
1. Put the `FileWatcher` into managed state: `.manage(Mutex<FileWatcher>)`, and add commands `add_watched_folder(path)` / `remove_watched_folder(path)` / `get_watched_folders()`.
2. On `add_watched_folder` (and on startup for saved folders): first do a **synchronous recursive scan** for `*.org` files, parse each into the store, then start watching the folder.
3. Persist settings on the Rust side (e.g. `tauri-plugin-store`, or a simple JSON file in `app_config_dir()`), and have `settingsSlice` load/save via commands.
4. Use Tauri's dialog plugin (`tauri-plugin-dialog`) for a real native folder picker in `SettingsDialog`.
5. Normalize paths (canonicalize) before using them as `TaskStore` keys — today a file parsed as `./notes.org` and modified as `/abs/path/notes.org` would occupy two different keys in `tasks_by_file`.

### H6. Watcher changes never reach the UI

The debouncer callback (`watcher/file_watcher.rs:26-53`) updates the store but nothing tells the webview. The frontend fetches tasks exactly once on mount (`App.tsx:18-20`), so external edits (Emacs, syncthing, etc.) are invisible until restart.

**Fix:** pass an `AppHandle` into `FileWatcher::new` (obtainable in `.setup(|app| ...)` after C6) and `app_handle.emit("tasks-changed", ())` after each store mutation. In the frontend, `listen('tasks-changed', () => fetchTasks())` from `@tauri-apps/api/event` inside a `useEffect`.

---

## Medium — correctness and design issues to fix soon

### M1. Every non-TODO headline becomes a TODO task

`models/task.rs:36-43`: a headline with **no** todo keyword defaults to `TodoState::Todo`. In an app whose point is notes + tasks in the same files, every plain note headline will show up as an open task.

**Fix:** make `Task.state: Option<TodoState>` (or add a `TodoState::None`), or only materialize `Task`s for headlines where `todo_state.is_some()` in `TaskStore::add_tasks_from_file` — depends on whether the task pane should show note headlines at all. Frontend `state` type must follow (H1).

### M2. TODO keyword configuration is inconsistent in three places

- Parser hardcodes `["TODO", "SOMEDAY"] / ["DONE"]` (`org_parser.rs:119-125`), so `IN_PROGRESS` and `CANCELED` are never recognized as keywords (they end up inside the title text).
- `TodoState` enum has `InProgress`/`Canceled` that can therefore never be produced.
- `settingsSlice.todoStateConfig` has yet another set and is wired to nothing.

**Fix:** define the keyword sets once in Rust (eventually loaded from settings), pass them into `ParseConfig`, and map keyword → `TodoState` from that same table. Add a parser test with `IN_PROGRESS`.

### M3. Timezone bug in agenda date formatting

`AgendaPane.tsx:8-10` uses `date.toISOString().split('T')[0]`, which converts to **UTC**. In any negative-offset timezone (e.g. Phoenix, UTC-7), from 5pm onward "today" formats as tomorrow's date, so the agenda highlights and matches the wrong day.

**Fix:** format from local parts:
```ts
const formatDate = (d: Date) =>
  `${d.getFullYear()}-${String(d.getMonth() + 1).padStart(2, '0')}-${String(d.getDate()).padStart(2, '0')}`;
```

### M4. `update_task`/`delete_task` API inconsistency and error ergonomics

- `update_task` takes a whole `Task`, `delete_task` takes `task_id: String` — pick one convention (id + patch object is typical).
- `CommandError` (`commands.rs:53-58`) has no `Display`/`std::error::Error` impl and serializes as `{"Store": "msg"}` / `{"Parser": "msg"}` — awkward to handle in TS. `thiserror` is already in `Cargo.toml` but unused.

**Fix:** convert `CommandError` and `ParserError` to `thiserror` derives; implement `Serialize` to a flat `{ kind, message }` shape; handle that shape in `api.ts`.

### M5. `Mutex::lock().unwrap()` everywhere

All commands and the watcher callback unwrap the store mutex. One panic while holding the lock (e.g. from C4's parser) poisons it and every subsequent command panics, taking the app down.

**Fix:** `lock().map_err(|_| CommandError::Store("state lock poisoned".into()))?` in commands; in the watcher callback, log and skip. (Or switch to `parking_lot::Mutex`, which doesn't poison.)

### M6. Dead/junk files and dead dependencies — DONE

- `src-tauri/orgtoical_main.rs` is a **saved docs.rs HTML page** (7,000+ lines of HTML with a `.rs` extension) — delete it. **Done** — deleted.
- Unused crates: `regex`, `log` (backend logs via `eprintln!`), `thiserror` (see M4), `tauri-plugin-log` (only registered in the dead `lib.rs` builder — after C6 it becomes live; then replace `eprintln!` with `log::error!`). **Done** — `regex` removed (no usages); `tauri-plugin-log` is now live via C6 and `file_watcher.rs`'s two `eprintln!` calls were switched to `log::error!`. `thiserror` intentionally left in place, still unused, pending the `CommandError`/`ParserError` derive work in M4.
- `commands.rs:14-19` TODO comment claims handlers are unimplemented; stale relative to reality — update or remove alongside H3. **Done** — removed (the handlers already exist below it; H3 tracks making them actually persist).

### M7. Fake UI elements that mislead during development

- `UpdatePrompt` renders "A new version is available!" unconditionally on every launch (`App.tsx:15,45`) with a no-op Update button. Remove it until you integrate `tauri-plugin-updater` (note: auto-update inside Flatpak is handled by Flathub anyway — this component may never be needed for your target).
- `RefileDialog` searches three hardcoded fake targets; `Toolbar`'s dropdown items are `console.log` stubs; `SettingsDialog` adds fake folder paths. Fine as scaffolding, but each should either be wired (RefileDialog → real file list from backend; folders → H5) or visibly marked disabled so testing doesn't chase ghosts.

### M8. Quick-capture IDs can collide and never reach disk

`QuickCaptureModal.tsx:13` uses `new Date().toISOString()` as the task id — two quick adds in the same millisecond collide, and React keys/toggles then misbehave. Use `crypto.randomUUID()` for the optimistic id, and replace it with the backend-assigned id once `create_task` persists (H3/H4).

### M9. `(state: any)` selectors defeat TypeScript

`App.tsx:16` and `QuickCaptureModal.tsx:6` use `useTasksSlice((state: any) => ...)`. The store is fully typed — remove the `any` so renames/refactors stay checked.

### M10. Content Security Policy disabled

`tauri.conf.json:26`: `"csp": null`. Before shipping (especially as a Flatpak), set a restrictive CSP, e.g. `"default-src 'self'; style-src 'self' 'unsafe-inline'"` — Tauri injects the right nonces for its own IPC. Do this early; retrofitting CSP after adding features is painful.

---

## Low — polish / hygiene

- **L1. Hand-rolled modals despite Radix being installed.** `QuickCaptureModal`, `RefileDialog`, `SettingsDialog`, `AgendaBuilderDialog` are plain fixed divs: no focus trap, no Escape-to-close, no scroll lock, no aria roles. `@radix-ui/react-dialog` is already a dependency — wrap each in `Dialog.Root/Portal/Content`. (Also `bg-opacity-50` is deprecated syntax in Tailwind v4 — use `bg-black/50`.)
- **L2. Watcher test is slow and timing-flaky** (`file_watcher.rs:80-106`, two 2-second sleeps). Poll with a timeout loop instead of fixed sleeps, or feature-gate it as an integration test.
- **L3. `parse_org_file`/`parse_org_content` commands construct a fresh `OrgParser`** instead of using `State<AppState>` — harmless now (the parser is stateless), but once ParseConfig comes from settings (M2) they'll silently use the wrong keywords. Route them through `AppState` when doing C6.
- **L4. `list_tasks` filter is case-sensitive substring** (`commands.rs:142-148`) while the frontend filter (`TaskListPane.tsx:79-93`) is case-insensitive and does its own filtering anyway — decide where filtering lives (frontend-only is fine at this scale) and delete the other.
- **L5. `paneSizes` state loop**: `MainLayout` feeds `paneSizes` into `defaultSize` and writes every resize back to the store, but `defaultSize` is only read on mount — the store write is currently useless. Either persist pane sizes (zustand `persist` middleware) so they survive restarts, or drop `setPaneSizes`.
- **L6. Pinned alpha dependency**: `orgize = "0.10.0-alpha.10"` is a pre-release; API churn is expected. Fine for now, but pin exactly (`=0.10.0-alpha.10`) so a `cargo update` can't break the parser silently.
- **L7. No JS test runner** despite `build-plan/tasks/tasks-frontend.md` referencing `pnpm exec jest`. Add Vitest (`pnpm add -D vitest @testing-library/react jsdom`) — it shares the Vite config, and the recursive tree helpers in `tasksSlice.ts`/`TaskListPane.tsx` are perfect first test targets.
- **L8. `pnpm` blocks builds in fresh checkouts**: `esbuild`/`@tailwindcss/oxide` build scripts are unapproved (`pnpm approve-builds`). Commit the approval (pnpm stores it in `package.json` > `pnpm.onlyBuiltDependencies`) so `pnpm install && pnpm build` works from a clean clone.
- **L9. Task checklists are marked complete but the code isn't.** Both `build-plan/tasks/*.md` files show everything `[x]`, yet C4–H6 above are unfinished. Per your own `.cursor/rules/process-tasks.mdc` workflow, un-check the items that aren't actually done (esp. backend 3.x/4.x command persistence and agenda) so the plan reflects reality.

---

## Architecture recommendations (the shape to converge on)

1. **`.org` files are the single source of truth.** Commands should write to files; the in-memory `TaskStore` is only a cache. The reconciliation loop is: command → file write → (watcher reparse → store replace → `tasks-changed` event → frontend refetch). Once that loop is honest, optimistic UI updates become safe because they're always corrected by disk truth.
2. **One Tauri builder** (C6), watcher and settings in managed state, all commands registered in one place.
3. **Stable identity via `:ID:` properties** (H2). This also gives you Emacs interop for free — org-id links keep working.
4. **A single typed IPC contract** (H1), ideally generated (`ts-rs`/`tauri-specta`), so the Rust and TS `Task` types can never drift again — three of the bugs above are pure drift.
5. **Fail loudly during development**: no mock-data fallbacks, no fake UI. Every silent fallback in this codebase directly hid a real broken wire.

## Suggested fix order

| # | Item | Why this order |
|---|------|----------------|
| 1 | C1, C2, C3 | Get `pnpm build` green and the UI rendering without crashes/styles broken. |
| 2 | C6 (+C5, M6 cleanup) | One entry point before touching commands. |
| 3 | C4 (+ its regression tests) | Stop the data-loss risk before wiring any file writes. |
| 4 | H2 (stable IDs) | Everything else keys off identity. |
| 5 | H1 (wire contract) | Frontend can finally display real data correctly. |
| 6 | H5 + H6 (watch folders, initial scan, change events) | Real data flows in. |
| 7 | H3 + H4 (persisting create/update/delete, agenda range) | Full round-trip. |
| 8 | M1–M10, then L-items | Correctness polish, then hygiene. |

---

*Generated by code review on 2026-07-16. Rust tests could not be executed in this environment (missing GTK/`gdk-3.0` system dev packages — on Debian: `sudo apt install libgtk-3-dev libwebkit2gtk-4.1-dev`); findings above are from source inspection, the TypeScript compiler, and verification against the vendored orgize crate source.*
