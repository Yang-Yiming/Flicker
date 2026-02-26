# Flicker Progress

---

# CLI

## Status: Phase 2 (TUI) ✓

### Completed

- [x] Project skeleton, file format, architecture doc, CLAUDE.md
- [x] Cargo project initialized (clap, serde, serde_yaml, uuid, chrono)
- [x] `model.rs` — Flicker struct, Status enum, frontmatter serde, file parsing
- [x] `storage.rs` — iCloud path resolution, FLICKER_DIR override, read/write/list/delete, conflict detection
- [x] `commands/add.rs` — create flicker with 8-char hex ID
- [x] `commands/list.rs` — sorted by date, filter by `--status`, hides deleted by default
- [x] `commands/show.rs` — display single flicker metadata + body
- [x] `commands/delete.rs` — soft delete (sets status to deleted)
- [x] `commands/search.rs` — case-insensitive full-text search
- [x] `commands/status.rs` — counts by status + conflict file reporting
- [x] `tui/state.rs` — App struct, Mode enum, filter/sort/mutate methods
- [x] `tui/ui.rs` — status tabs, flicker list, detail pane, input overlay
- [x] `tui/mod.rs` — event loop, keybindings (q/a/d/s/Tab/Enter/Esc/j/k//)
- [x] `main.rs` — no-args launches TUI
- [x] TUI command bar — `:` triggers Command mode; `add`/`delete`/`search` dispatch; unknown cmd shows error message
- [x] TUI command autocomplete — `:` shows all candidates; typing filters; ↓/Tab navigate; Enter accepts & executes
- [x] `commands/rename.rs` — update body of a flicker by ID
- [x] `commands/bash.rs` — run arbitrary shell command via `sh -c`
- [x] TUI `!` shortcut — bash input bar; suspends TUI, runs command, waits for Enter, resumes
- [x] TUI `v` shortcut — opens selected flicker in nvim (falls back to vim); reloads on exit
- [x] TUI autocomplete driven by clap subcommand list — no manual registration needed
- [x] `FLICKER_DIR` env var override for local dev (iCloud not required)
- [x] `commands/config.rs` — get/set/list config keys (`editor`, `shell`); persisted to `~/.config/flicker/config.toml`

### Next Up

- Phase 3: Polish — error handling, colored output, README

---

# iOS App

## Status: Phase 2 (Voice Input) ✓

### Completed

- [x] `FlickerApp.swift` — app entry point
- [x] `Models/Flicker.swift` — model with frontmatter parsing
- [x] `Services/StorageService.swift` — iCloud file I/O, conflict detection
- [x] `Views/FlickerListView.swift` — list with status filter chips
- [x] `Views/FlickerDetailView.swift` — view/edit, status change, delete
- [x] `Views/NewFlickerView.swift` — text input to create flicker
- [x] `Services/SpeechService.swift` — Speech framework, AVAudioEngine, real-time transcription
- [x] Audio saved as `audio/{id}.m4a`, linked in frontmatter
- [x] Record button in NewFlickerView

### Next Up

- Phase 3: Polish — empty states, loading indicators, real device iCloud sync test
