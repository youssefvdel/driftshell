# 2026-06-20 12:00 — Project Architecture & Initial Scaffolding

## Context

Starting driftshell from scratch. User wants a Rust-based Wayland shell for driftwm compositor with four components: bar, launcher, settings, wallpaper.

## Research (Pre-Implementation)

### Wayland Shell Surface Options

Three approaches exist for Iced + Wayland layer-shell:

1. **iced_layershell** (waycrate/exwlshelleventloop) — Drop-in Wayland event loop replacing winit. Works with upstream Iced 0.14. Six real-world bars use it.
2. **pop-os/iced fork** — System76's fork with built-in Wayland backend. Battle-tested in COSMIC. Requires forking Iced.
3. **Raw sctk + tiny-skia** — Full control, ~5 MB binary, but no widget library.

**Decision**: iced_layershell. Works with upstream Iced, has real bars, SCTK-based (same foundation as driftwm).

### Driftwm IPC Protocol

- Unix socket at `$XDG_RUNTIME_DIR/driftwm/ipc-<WAYLAND_DISPLAY>.sock`
- Line-delimited JSON: one request per line, one reply per line
- Commands: `Camera`, `Zoom`, `Focus`, `Move`, `Layout`, `State`, `Action`, `Screenshot`
- Config actions include: `reload-config`, `close-window`, `zoom-in`, `exec`, `spawn`, etc.
- No event subscriptions — polling via state file at ~10 Hz

### Dependency Analysis

| Crate | Version | Need |
|-------|---------|------|
| iced | 0.14.0 | Widget library, types |
| iced_layershell | 0.18.1 | Wayland layer-shell event loop (no winit needed) |
| sctk | 0.20.0 | Direct Wayland access if needed |
| wayland-client | 0.31.14 | Low-level Wayland |
| serde + toml | latest | Config parsing |
| serde_json | latest | IPC protocol |
| clap | 4.x | CLI args |
| chrono | 0.4.45 | Clock |
| zbus | 5.16.0 | D-Bus (tray, MPRIS future) |
| calloop | 0.14.4 | Event loop utilities |
| tokio | 1.52.3 | Async runtime |

## Plan

### Phase 1: Scaffolding (Today)
1. ✅ Create repo + Cargo project
2. ✅ Add all dependencies
3. ☐ Write driftwm IPC module (socket connect, send request, parse reply)
4. ☐ Write driftwm config module (TOML read/write)
5. ☐ Write bar module with clock + iced_layershell surface
6. ☐ Write main.rs + app.rs that compiles and shows a bar
7. ☐ Verify compilation

### Phase 2: Bar Modules
8. Workspace indicators (via state file polling)
9. Clock module
10. Module system trait

### Phase 3: Launcher
11. Layer-shell overlay surface
12. App list with fuzzy search

### Phase 4: Settings
13. XDG toplevel window
14. Config UI tabs

### Phase 5: Wallpaper
15. Background changer via config.toml

## Notes

- iced_layershell 0.18.1 confirmed on crates.io, works with iced 0.14
- sctk 0.20 resolves alongside iced_layershell's own deps
- Need to add `iced_widget` separately since iced_layershell only re-exports `iced_core`
- Top-level `iced` crate needed for `iced::Renderer` type, `iced::Theme`, etc.
- iced_layershell re-exports: `Anchor`, `Layer`, `KeyboardInteractivity`, `core::*`, `Task`
- Minimal example: anchor top, full width, 40px height, exclusive_zone 40
