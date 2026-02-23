# Flicker Implementation Plan

## Phase 1: CLI Core

Goal: Basic file operations via subcommands.

- [x] Init Cargo project with clap, serde, serde_yaml
- [x] `model.rs` — Flicker struct with frontmatter serde
- [x] `storage.rs` — iCloud path resolution, read/write/list/delete
- [x] Subcommands:
  - [x] `add` — create new flicker with generated ID
  - [x] `list` — list flickers, filter by `--status`
  - [x] `show` — display single flicker by ID
  - [x] `delete` — set status to deleted
  - [x] `search` — full-text search across flickers
  - [x] `status` — show counts by status, report conflicts

## Phase 2: TUI

Goal: Interactive terminal UI with ratatui.

- [x] Add ratatui + crossterm dependencies
- [x] `tui/state.rs` — app state (list, detail, search, add modes)
- [x] `tui/ui.rs` — layout: status tabs, flicker list, detail pane
- [x] `tui/mod.rs` — event loop, keybindings
- [x] Keybindings: `/` search, `a` add, `d` delete, `s` cycle status, `q` quit

## Phase 3: iOS Basic

Goal: SwiftUI app with iCloud read/write.

- [ ] Xcode project with iCloud entitlement (`iCloud~com.flicker.app`)
- [ ] `Flicker.swift` — model with frontmatter parsing
- [ ] `StorageService.swift` — iCloud file I/O, conflict detection
- [ ] `FlickerListView` — list with status filter
- [ ] `FlickerDetailView` — view/edit single flicker
- [ ] `NewFlickerView` — text input to create flicker

## Phase 4: iOS Voice

Goal: Voice-to-text input on iOS.

- [ ] `SpeechService.swift` — Speech framework integration
- [ ] AVAudioEngine recording + real-time transcription
- [ ] Save audio as `audio/{id}.m4a`, link in frontmatter
- [ ] UI: record button in NewFlickerView

## Phase 5: Polish

- [ ] Error handling & edge cases
- [ ] CLI: colored output, better formatting
- [ ] iOS: empty states, loading indicators
- [ ] Test on real iCloud sync between devices
- [ ] README with install instructions
