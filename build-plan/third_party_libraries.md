# Third‑Party Libraries

| Name | Link | Purpose | License | Why Chosen |
|------|------|---------|---------|------------|
| **Tauri** | <https://tauri.app> | Cross‑platform desktop shell that bridges Rust core with a WebView front‑end. | MIT | Lightweight, secure alternative to Electron; first‑class Rust support and small bundle size. |
| **React** | <https://reactjs.org> | Declarative JavaScript library for building user interfaces. | MIT | De‑facto standard for component‑based UIs; vast ecosystem and tooling. |
| **Zustand** | <https://github.com/pmndrs/zustand> | Minimal global state‑management for React. | MIT | Tiny footprint, no boilerplate; perfect fit for a desktop MVP. |
| **Tailwind CSS** | <https://tailwindcss.com> | Utility‑first CSS framework. | MIT | Enables consistent styling with minimal CSS files; easy theming and responsive design. |
| **Radix UI Primitives** | <https://www.radix-ui.com> | Accessible, unstyled React primitives (dialogs, menus). | MIT | Provides a11y‑correct building blocks that pair well with Tailwind for custom design. |
| **shadcn/ui** | <https://ui.shadcn.com> | Component library that stitches Radix primitives with Tailwind. | MIT | Accelerates development with pre‑wired examples while staying design‑agnostic. |
| **orgize** (Rust crate) | <https://github.com/PoiScript/orgize> | Parses and modifies Emacs Org files. | MIT / Apache‑2.0 | Mature, actively maintained Org‑mode parser in Rust; exactly matches our file‑compat requirement. |
| **notify** (Rust crate) | <https://github.com/notify-rs/notify> | Cross‑platform filesystem watch. | MIT / Apache‑2.0 | Simple API for detecting file changes across Linux, Windows, macOS. |
| **serde & serde_json** | <https://serde.rs> | Serialize/deserialize Rust structs to JSON. | MIT / Apache‑2.0 | De‑facto standard for Rust JSON handling; zero‑cost abstractions and broad ecosystem use. |
| **Fuse.js** | <https://fusejs.io> | Lightweight JavaScript fuzzy‑search library. | Apache‑2.0 | Fast client‑side fuzzy matching for the re‑file dialog with minimal bundle impact. |
| **Flatpak Builder Tools** | <https://flatpak.org> | Packaging & sandboxing for Linux desktop apps. | LGPL‑2.1+ | Provides unified distribution on Mint and other distros; integrates cleanly with CI. |

