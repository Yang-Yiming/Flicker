# Flicker Progress

## Current Phase: Phase 3 (iOS Basic) ✓

### Completed

- [x] Project skeleton created (`cli/`, `ios-app/`, `shared/`, `DOCS/`)
- [x] File format defined (`shared/file-format.md`)
- [x] Architecture doc written (`DOCS/ARCHITECTURE.md`)
- [x] Implementation plan written (`DOCS/PLAN.md`)
- [x] CLAUDE.md configured
- [x] Cargo project initialized (clap, serde, serde_yaml, uuid, chrono)
- [x] `model.rs` — Flicker struct, Status enum, frontmatter serde, file parsing
- [x] `storage.rs` — iCloud path resolution, FLICKER_DIR override, read/write/list/delete, conflict detection
- [x] `commands/add.rs` — create flicker with 8-char hex ID, prints ID
- [x] `commands/list.rs` — list flickers sorted by date, filter by `--status`, hides deleted by default
- [x] `commands/show.rs` — display single flicker metadata + body
- [x] `commands/delete.rs` — soft delete (sets status to deleted)
- [x] `commands/search.rs` — case-insensitive full-text search
- [x] `commands/status.rs` — counts by status + conflict file reporting
- [x] ratatui + crossterm added to Cargo.toml
- [x] `tui/state.rs` — App struct, Mode enum (List/Detail/Search/Add), filter/sort/mutate methods
- [x] `tui/ui.rs` — status tabs, flicker list, detail pane, input overlay
- [x] `tui/mod.rs` — event loop, keybindings (q/a/d/s/Tab/Enter/Esc/j/k//)
- [x] `main.rs` — no-args launches TUI

- [x] `FlickerApp.swift` — app entry point
- [x] `Models/Flicker.swift` — model with frontmatter parsing
- [x] `Services/StorageService.swift` — iCloud file I/O, conflict detection
- [x] `Views/FlickerListView.swift` — list with status filter chips
- [x] `Views/FlickerDetailView.swift` — view/edit single flicker, status change, delete
- [x] `Views/NewFlickerView.swift` — text input to create flicker

### Next Up

- Phase 4: iOS Voice
- Create Xcode project: File → New → App, bundle ID `com.flicker.app`, add iCloud entitlement (`iCloud~com.flicker.app`), add all Swift files
