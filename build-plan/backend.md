# Backend Specification

## 1. Service Diagram (text)

```
┌──────────────────────────────────────────────────────────┐
│                       Tauri Shell                       │
│ (Cross‑platform desktop runtime; bridges Rust ↔ JS IPC) │
│                                                          │
│  ┌────────────┐        JSON IPC        ┌──────────────┐  │
│  │  React UI  │◄──────────────────────►│  Command Bus │  │
│  └────────────┘                        └──────────────┘  │
│                                          ▲               │
│                                          │ calls         │
│  ┌───────────────┐      parses           │               │
│  │   Org Parser  │──orgize crate───────► │               │
│  └───────────────┘                       │               │
│          │ writes                        │               │
│          ▼                               │               │
│  ┌───────────────┐  file events  ┌───────────────┐       │
│  │  File Watcher │──────────────►│  Task Store   │       │
│  └───────────────┘  (notify)     └───────────────┘       │
└──────────────────────────────────────────────────────────┘
```

- **Command Bus** — maps incoming IPC requests to Rust service functions, serialises JSON responses.
- **Org Parser** — uses `orgize` to transform `.org` files ↔ Rust structs.
- **Task Store** — in‑memory cache of parsed tasks, keyed by file path for quick look‑ups.
- **File Watcher** — watches user‑defined folders; on change, reparses affected files and emits delta over IPC.

---

## 2. API Endpoints Table

| Endpoint          | Verb   | Request Body / Params                                                                       | Response            | Notes                                                   |
| ----------------- | ------ | ------------------------------------------------------------------------------------------- | ------------------- | ------------------------------------------------------- |
| `/createTask`     | POST   | `{ title, tags?, priority?, todoState?, scheduled?, deadline?, destFile, destHeadingPath }` | `{ taskId }`        | Writes headline to chosen file/heading; returns new ID. |
| `/updateTask/:id` | PUT    | Partial task fields to update                                                               | `{ success: true }` | Moves or rewrites headline in file.                     |
| `/deleteTask/:id` | DELETE | —                                                                                           | `{ success: true }` | Removes (or archives) headline.                         |
| `/listTasks`      | GET    | `?files?&tags?&states?`                                                                     | `[ Task ]`          | Optional filters; returns collection.                   |
| `/getAgendaRange` | GET    | `?start=YYYY-MM-DD&end=YYYY-MM-DD`                                                          | `[ Task ]`          | Returns tasks whose scheduled/deadline fall in range.   |

All responses JSON‑serialised. Errors return `{ error: { code, message } }`.

---

## 3. Entity‑Relationship Overview

```
Folder (path) 1 ─┐
                 │  contains
Folder (path) n ─┘
    │
    │
    ▼
File (.org) *────── contains multiple ───► Task (headline)*
                                         │
                                         ├─ tags [string]
                                         ├─ todoState (enum)
                                         ├─ priority (char)
                                         ├─ scheduled (date?)
                                         ├─ deadline (date?)
                                         ├─ properties { key: string }
                                         └─ headingPath (array of indices)
```

- Only **Task** entities are persisted by backend; other user prefs live in frontend JSON.

---

## 4. Deployment Outline

1. **Local build**
   - `cargo tauri build --target x86_64-unknown-linux-gnu` produces native binary.
   - Bundle via `flatpak-builder` with manifest (app ID `io.specgpt.OrgDesk`).
2. **CI/CD (GitHub Actions)**
   - Trigger on `main` branch tag.
   - Jobs:
     1. Set up Rust tool‑chain + Node.
     2. Install `flatpak` & Tauri CLI.
     3. Run unit tests.
     4. Build Flatpak, generate checksum.
     5. Upload artifact & create GitHub Release.
3. **Distribution**
   - Users add Flathub repo or download `.flatpakref` via project site.
4. **Update flow**
   - App pings GitHub Releases JSON; if newer `flatpak` found, prompts user to install via `flatpak update`.

---

*This backend spec now meets Definition‑of‑Done: service diagram, API endpoints, entity‑relationship, deployment outline.*

