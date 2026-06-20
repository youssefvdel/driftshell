# 2026-06-20 14:00 — Settings Window Implementation

## Plan

1. **Add `write()` to `driftwm/config.rs`** — serde serialize + write toml to config_path
2. **Write `settings/mod.rs`** — layer-shell overlay with form controls for cursor + background config
3. **Integrate into `app.rs`** — add `settings: settings::Settings`, `settings_window: Option<IcedId>`, `Message::Settings`, `Message::OpenSettings`, toggle logic in update/view

## Settings Window Design

- **Surface**: wlr-layer-shell overlay, centered (all anchors + margin), 600x500, Overlay layer
- **Form sections**:
  - Cursor: theme (text input), size (text input, numeric)
  - Background: type (pick_list: solid/gradient/shader), path (text input)
  - Save button → write toml → reload-config IPC → close window
- **State**: `DriftwmConfig` loaded on open, edited in-place, serialized on save
