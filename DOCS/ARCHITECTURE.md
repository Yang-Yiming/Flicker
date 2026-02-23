# Flicker Architecture

## Overview

```
┌─────────────┐      ┌──────────────────┐      ┌──────────────┐
│   iOS App   │ ←──→ │   iCloud Drive   │ ←──→ │  Rust CLI/TUI│
│  (SwiftUI)  │      │  (shared folder) │      │  (ratatui)   │
└─────────────┘      └──────────────────┘      └──────────────┘
```

Both clients read/write the same Markdown files in iCloud Drive. No custom sync logic — iCloud handles replication automatically.

## iCloud Directory Structure

Container: `iCloud~com.flicker.app`

macOS path: `~/Library/Mobile Documents/iCloud~com~flicker~app/Documents/`

```
Documents/
├── flickers/
│   ├── a1b2c3d4.md
│   ├── e5f6a7b8.md
│   └── ...
└── audio/
    ├── a1b2c3d4.m4a
    └── ...
```

## File Format

Each flicker is a Markdown file with YAML frontmatter (defined in `shared/file-format.md`):

```yaml
---
id: f81d4fae          # 8-char hex short UUID
created_at: 2026-02-23T10:31:22Z
source: ios            # ios | cli
audio_file: audio/f81d4fae.m4a  # optional
status: inbox          # inbox | kept | archived | deleted
---

Free-form text content here.
```

### File Naming

- Filename = `{id}.md` (8-char hex, e.g. `a1b2c3d4.md`)
- ID generated from first 8 hex chars of UUID v4
- Audio files share the same ID: `audio/{id}.m4a`

## Conflict Handling

iCloud may create conflict copies named `{name} 2.md`. Strategy:

1. On startup / refresh, scan for conflict files
2. Keep the file with the later `created_at`
3. Discard the duplicate (move to deleted status)
4. CLI `status` command reports unresolved conflicts

## CLI Design — Dual Mode

### Subcommand Mode (for scripts / AI)

```
flicker add "idea text"
flicker list [--status inbox]
flicker show <id>
flicker delete <id>
flicker search <query>
flicker status
```

### TUI Mode (interactive)

`flicker` with no arguments launches ratatui TUI:
- List view with status filter tabs
- `/` to search, `Enter` to view detail
- `a` to add, `d` to delete, `s` to cycle status
- `q` to quit

## CLI Module Structure

```
cli/
├── Cargo.toml
└── src/
    ├── main.rs          # arg parsing (clap), dispatch
    ├── model.rs         # Flicker struct, frontmatter serde
    ├── storage.rs       # file I/O, iCloud path resolution
    ├── commands/
    │   ├── mod.rs
    │   ├── add.rs
    │   ├── list.rs
    │   ├── show.rs
    │   ├── delete.rs
    │   ├── search.rs
    │   └── status.rs
    └── tui/
        ├── mod.rs       # app loop, event handling
        ├── ui.rs        # layout & rendering
        └── state.rs     # TUI state machine
```

## iOS Module Structure

```
ios-app/
├── Flicker.xcodeproj
├── Flicker/
│   ├── FlickerApp.swift
│   ├── Models/
│   │   └── Flicker.swift        # data model, frontmatter parsing
│   ├── Services/
│   │   ├── StorageService.swift  # iCloud file read/write
│   │   └── SpeechService.swift   # Speech framework + AVAudioEngine
│   └── Views/
│       ├── FlickerListView.swift
│       ├── FlickerDetailView.swift
│       └── NewFlickerView.swift
```

## Tech Stack

| Component | Technology |
|-----------|-----------|
| CLI | Rust, clap, serde_yaml, pulldown-cmark |
| TUI | ratatui, crossterm |
| iOS | SwiftUI, Speech framework, AVAudioEngine |
| Data format | Markdown + YAML frontmatter |
| Sync | iCloud Drive (native) |
| ID generation | UUID v4, truncated to 8 hex chars |
