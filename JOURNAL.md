# driftshell Journal

## 2026-06-20 — Project Init

- **First commit**: `bdc87ee` — initial scaffold pushed to GitHub
- **Compiles**: cargo check + clippy clean, zero warnings
- **Modules**: app, bar, driftwm (ipc + config), shared, launcher, settings, wallpaper (stubs)
- **Dead code**: driftwm/ipc + config have `#![allow(dead_code)]` for upcoming workspace polling + settings features

## 2026-06-20 — Bar Workspace Indicators + Launcher + Settings

- **`b46e6b5`**: Bar workspace polling via driftwm IPC. Workspace buttons render in bar with accent color for active window. 500ms poll interval.
- **`a6c2f2f`**: Launcher overlay. App list from `.desktop` files (substring search). Text input + scrollable results. Toggle via `driftwm msg action OpenLauncher`. Daemon mode with `NewLayerShell`/`RemoveWindow`.
- **`<pending>`**: Settings window. Layer-shell overlay with cursor theme/size + background type/path form. Save writes config.toml + triggers `reload-config` IPC. Toggle via `driftwm msg action OpenSettings`.
- **Next**: Wallpaper config integration

- **Repo created**: `youssefvdel/driftshell` (public)
- **Stack decided**: Iced 0.14 + iced_layershell 0.18 for Wayland layer-shell UI
- **Dependencies**: iced, iced_layershell, sctk, wayland-client, serde, toml, clap, chrono, zbus, tokio, calloop
- **Goal**: Lightweight Rust shell for driftwm (bar + launcher + settings + wallpaper)
- **Architecture documented**: AGENTS.md
- **First milestone**: Minimal bar that displays on driftwm

### Research Outputs

1. **iced_layershell** crate allows Iced widgets on wlr-layer-shell surfaces without forking Iced
2. **driftwm IPC** is line-delimited JSON over Unix socket at `$XDG_RUNTIME_DIR/driftwm/ipc-<DISPLAY>.sock`
3. **driftwm config** is TOML at `~/.config/driftwm/config.toml`, hot-reloadable via `reload-config` action
4. **Eight+ Rust Wayland shell/bar projects** exist (Icebar, IceLauncher, lala-bar, bar-rs, etc.)
5. **Waybar's module system** is the reference architecture (per-output bars, module factory pattern)
