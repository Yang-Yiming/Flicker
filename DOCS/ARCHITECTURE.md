# Flicker Architecture

## Overview

```
┌─────────────┐      ┌──────────────────┐      ┌──────────────┐
│   iOS App   │ ←──→ │    Supabase      │ ←──→ │  Rust CLI/TUI│
│  (SwiftUI)  │      │  (PostgreSQL +   │      │  (ratatui)   │
│             │      │   Storage)       │      │              │
└─────────────┘      └──────────────────┘      └──────────────┘
```

Both clients store flickers locally as Markdown files. Supabase acts as the sync backend — each client pulls remote changes and pushes local changes on demand. Local-first: the app works fully offline; sync catches up when connectivity returns.

## Sync Backend — Supabase

- **Database:** PostgreSQL table `flickers` stores metadata + body
- **Storage:** `flicker-audio` bucket stores audio files as `{id}.m4a`
- **Auth:** No RLS, no user auth — single-user personal app. The anon key acts as the access credential.
- **Protocol:** REST API via PostgREST (CLI uses reqwest, iOS uses supabase-swift)

### Sync Protocol

Strategy: local-first, pull-then-push, last-write-wins on `updated_at`.

```
sync():
  last_synced = load_last_synced_at()

  // Phase 1: Pull
  remote_changes = GET /rest/v1/flickers?updated_at=gt.{last_synced}
  for each remote in remote_changes:
    local = read_local(remote.id)
    if local == nil OR remote.updated_at > local.updated_at:
      write_local(remote)
      if remote.audio_file AND !local_audio_exists(remote.id):
        download_audio(remote.id)

  // Phase 2: Push
  local_changes = all local flickers where updated_at > last_synced
  for each local in local_changes:
    UPSERT to /rest/v1/flickers
    if local.audio_file AND audio_exists_locally(local.id):
      upload_audio(local.id)

  // Phase 3: Update timestamp
  save_last_synced_at(now())
```

### Supabase Schema

```sql
CREATE TABLE flickers (
    id          TEXT PRIMARY KEY,
    created_at  TIMESTAMPTZ NOT NULL,
    updated_at  TIMESTAMPTZ NOT NULL,
    source      TEXT NOT NULL DEFAULT 'cli',
    audio_file  TEXT,
    status      TEXT NOT NULL DEFAULT 'inbox',
    body        TEXT NOT NULL DEFAULT ''
);

CREATE INDEX idx_flickers_updated_at ON flickers (updated_at);
```

Storage bucket: `flicker-audio` (path pattern: `{flicker_id}.m4a`)

## Local Directory Structure

Each client stores files locally:

- **CLI default:** `~/Documents/flicker/` (configurable via `storage_path` in config)
- **iOS:** app's Documents directory

```
{storage_root}/
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
updated_at: 2026-03-05T09:15:00Z  # for sync conflict resolution
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

## Conflict Resolution

Last-write-wins based on `updated_at` timestamp. During sync:
- If remote `updated_at` > local `updated_at`, remote wins
- Otherwise local version is preserved and pushed

Offline changes are pushed on next sync. No manual conflict resolution needed.

## CLI Design — Dual Mode

### Subcommand Mode (for scripts / AI)

```
flicker add "idea text"
flicker list [--status inbox]
flicker show <id>
flicker delete <id>
flicker search <query>
flicker status
flicker rename <id> <new body>
flicker bash <shell cmd>
flicker config list
flicker config get <key>
flicker config set <key> <value>
flicker sync
```

### TUI Mode (interactive)

`flicker` with no arguments launches ratatui TUI:
- List view with status filter tabs
- `/` to search, `Enter` to view detail
- `a` to add, `d` to delete, `s` to cycle status
- `?` for config (includes Supabase settings)
- `q` to quit

## CLI Module Structure

```
cli/
├── Cargo.toml
└── src/
    ├── main.rs          # arg parsing (clap), dispatch
    ├── model.rs         # Flicker struct, frontmatter serde
    ├── storage.rs       # file I/O, local path resolution
    ├── config.rs        # Config struct, load/save (~/.config/flicker/config.toml)
    ├── sync.rs          # SyncClient — Supabase REST API (reqwest)
    ├── sync_state.rs    # last_synced_at persistence
    ├── commands/
    │   ├── mod.rs
    │   ├── add.rs
    │   ├── list.rs
    │   ├── show.rs
    │   ├── delete.rs
    │   ├── search.rs
    │   ├── status.rs
    │   ├── rename.rs
    │   ├── bash.rs
    │   └── config.rs
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
│   │   ├── StorageService.swift  # local file read/write
│   │   ├── SpeechService.swift   # Speech framework + AVAudioEngine
│   │   └── SyncService.swift     # Supabase sync (supabase-swift)
│   └── Views/
│       ├── FlickerListView.swift
│       ├── FlickerDetailView.swift
│       ├── NewFlickerView.swift
│       └── SettingsView.swift    # Supabase config + manual sync
```

## Tech Stack

| Component | Technology |
|-----------|-----------|
| CLI | Rust, clap, serde_yaml |
| TUI | ratatui, crossterm |
| CLI sync | reqwest (blocking), serde_json |
| iOS | SwiftUI, Speech framework, AVAudioEngine |
| iOS sync | supabase-swift (~> 2.0) |
| Sync backend | Supabase (PostgreSQL + Storage) |
| Data format | Markdown + YAML frontmatter |
| ID generation | UUID v4, truncated to 8 hex chars |
