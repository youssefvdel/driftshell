# driftshell — Rust Wayland Shell for driftwm

## Overview

driftshell is a lightweight, Rust-based Wayland shell built specifically for the [driftwm](https://github.com/malbiruk/driftwm) compositor. It provides:

- **Bar** — wlr-layer-shell top panel with workspace indicators, clock, and module system
- **Launcher** — overlay app launcher with fuzzy search
- **Settings** — XDG toplevel window for driftwm configuration
- **Wallpaper** — background management via config.toml

## Architecture

```
driftshell
├── src/
│   ├── main.rs              # Entry: init logging, CLI, launch iced
│   ├── app.rs               # Root iced Application (model/view/update)
│   ├── bar/                 # Status bar (wlr-layer-shell, top anchor)
│   ├── launcher/            # App launcher (layer-shell overlay)
│   ├── settings/            # Settings window (XDG toplevel)
│   ├── wallpaper/           # Background manager (config.toml writer)
│   ├── driftwm/
│   │   ├── mod.rs           # Public API
│   │   ├── ipc.rs           # Unix socket IPC client (JSON)
│   │   └── config.rs        # TOML config read/write
│   └── shared/              # Theme, widgets, utilities
```

### Rendering Stack

```
[iced widgets] → [iced_layershell] → [sctk (smithay-client-toolkit)] → [wlr-layer-shell] → [driftwm]
```

### Communication with driftwm

| Channel | Type | Path | Usage |
|---------|------|------|-------|
| IPC Socket | Line-delimited JSON over Unix socket | `$XDG_RUNTIME_DIR/driftwm/ipc-<DISPLAY>.sock` | Query camera, zoom, focus; send actions |
| State File | `key=value` text file | `$XDG_RUNTIME_DIR/driftwm/state` | Poll at ~10 Hz for window list, layout |
| Config File | TOML | `~/.config/driftwm/config.toml` | Read/write settings, then `reload-config` |

## Development Rules

### 1. Document Everything in study/ by Date/Time

Every implementation step MUST be preceded by a study note in `src/study/YYYY-MM-DD-topic.md`.

Each study file contains:
```
# YYYY-MM-DD HH:MM — Topic

## Context
What prompted this work.

## Research
What was investigated before implementation.

## Plan
Step-by-step implementation plan.

## Notes
Observations, gotchas, decisions made during implementation.
```

### 2. Commit Frequently

Small, atomic commits with descriptive messages. One logical change per commit.

### 3. Test Before Merge

Each component gets a basic test before integration. Run `cargo check` after every significant change.

### 4. Keep it Lightweight

- Minimize dependencies
- No C library bindings if a pure Rust alternative exists
- Binary size is a concern — monitor it

## Toolchain

- **Rust edition**: 2024
- **Key crates**: iced 0.14, iced_layershell 0.18, sctk 0.20, serde, toml
- **Async**: tokio (for IPC and timers)
- **D-Bus**: zbus (for status notifier / tray)
