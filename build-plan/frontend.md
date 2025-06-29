# Frontend Specification

## 1. Architecture Diagram (text)

```
┌────────────────────────────┐
│          Tauri             │
│   (Electron‑like shell)    │
│                            │
│  ┌──────────────────────┐  │
│  │   React Renderer     │  │  ⇦— Zustand global store
│  │  • Toolbar           │  │
│  │  • TaskListPane      │  │
│  │  • AgendaPane        │  │
│  │  • Modals & Dialogs  │  │
│  └──────────────────────┘  │
│          ▲  ▲              │
│          │  │ IPC (JSON)   │
└──────────┼──┼──────────────┘
           │  │
           │  └───► Rust Core Service (orgize parser, file‑watcher)
           └──────► Flatpak update checker / system APIs
```

## 2. UI Component List

| Component | Purpose |
| --------- | ------- |
|           |         |

| **Toolbar** (`File / Edit / Settings`) | App‑level commands, Settings toggle for Vim mode.                                    |
| -------------------------------------- | ------------------------------------------------------------------------------------ |
| **TaskListPane**                       | Hierarchical outline of headlines; supports inline edit, quick filters, drag‑scroll. |
| **AgendaPane**                         | 7‑day agenda preview; can mark TODO → DONE; dropdown to switch presets.              |
| **QuickCaptureModal**                  | Global shortcut ⇒ modal to capture new headline, choose destination, tags, priority. |
| **RefileDialog**                       | Keyboard‑triggered fuzzy search to move headline; powered by `Fuse.js`.              |
| **AgendaBuilderDialog**                | Create / edit saved agenda presets.                                                  |
| **SettingsDialog**                     | Toggle Vim keybindings, manage watched folders, default agenda scope, update‑check.  |
| **UpdatePrompt**                       | Non‑modal toast/dialog when new Flatpak release detected.                            |

## 3. State‑Management Plan

- **Library:** Zustand (tiny, atomic slices, React‑server friendly)
- **Slices:**
  - `tasksSlice` – loaded tasks, CRUD actions, optimistic cache.
  - `agendaSlice` – selected date range, preset list, derived agenda items.
  - `uiSlice` – pane sizes, active modal, Vim‑mode flag.
  - `settingsSlice` – watched folders, TODO state config, toolbar prefs.
- **Data Flow:**
  1. UI dispatches action (e.g., `createTask`).
  2. Zustand updates optimistic state; IPC message sent to Rust.
  3. Rust persists change to `.org` file via **orgize** and responds with updated JSON.
  4. Store reconciles any deltas; affected components re‑render.

## 4. Accessibility Targets (v1)

- Semantic HTML elements for lists, headings, buttons.
- All interactive elements reachable via keyboard (Tab + arrows).
- Minimum colour‑contrast AA via Tailwind default palette.
- Screen‑reader labels for modals and dialog titles.

---

**Definition‑of‑Done satisfied:** architecture diagram, UI component list, state‑management plan, accessibility targets.

