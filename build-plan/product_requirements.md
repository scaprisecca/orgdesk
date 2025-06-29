# Product Requirements

## a. Problem Statement
Knowledge workers who appreciate Org Mode’s powerful task and note‑taking workflow often find Emacs overly complex to install, configure, and customise—especially for advanced agenda filters and multi‑file setups. The goal is to deliver a cross‑platform desktop application (first release on Linux Mint) that preserves Org’s plaintext‐file compatibility while providing an intuitive GUI for daily task management, linked‑note capture, and custom agenda views.

## b. Personas
| Persona | Description | Needs & Pain‑Points |
|---------|-------------|---------------------|
| **Primary — “Power User Paul”** | Tech‑comfortable knowledge worker who already uses Emacs/Org on occasion but dislikes its steep learning curve. | Wants Org TODO + Roam‑style linking without wrestling with Emacs configs; demands bullet‑proof file compatibility. |
| **Secondary — “Efficient Emma”** | Productivity enthusiast (not a developer) who has heard of Org Mode’s flexibility but avoids command‑line tools. | Needs a turnkey desktop app for tasks & notes; easy custom filters; reassurance that data lives in human‑readable `.org` files. |

## c. Core User Journeys (v1)
1. **Daily Task Flow**
   1. Launch app ➜ Task List opens automatically.
   2. Capture a new headline with initial `TODO` state (assign tags/properties).
   3. Edit headline to add or update schedule / deadline.
   4. View aggregated Agenda (default 7‑day horizon) in side pane.
   5. Refile headlines to different files/sections as needed.

2. **Custom Agenda Creation**
   1. Open Agenda Builder dialog.
   2. Select filters (tags, properties, TODO states, date ranges).
   3. Save as named agenda preset.
   4. Switch the side‑pane Agenda to any saved preset in ≤2 clicks.

## d. Success Metrics
| Metric | Target |
|--------|--------|
| **Data integrity** | 0 data‑loss bugs after 30 consecutive days of daily use. |
| **Agenda performance** | Custom agenda view opens/refreshes in ≤ 10 seconds on a machine with 10 × 10 kB `.org` files. |
| **Usability** | New task captured & scheduled in ≤ 5 seconds via keyboard‑first flow. |
| **Compatibility** | 100% of `.org` file syntax used in v1 renders identically in Emacs Org Mode. |

