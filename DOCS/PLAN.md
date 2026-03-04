# Flicker Implementation Plan

Flicker has two independent clients that sync via Supabase. CLI and iOS app can be developed and shipped independently.

---

# CLI

## Phase 1: CLI Core ✓

- [x] Init Cargo project with clap, serde, serde_yaml
- [x] `model.rs` — Flicker struct with frontmatter serde
- [x] `storage.rs` — iCloud path resolution, read/write/list/delete
- [x] Subcommands: `add`, `list`, `show`, `delete`, `search`, `status`

## Phase 2: TUI ✓

- [x] Add ratatui + crossterm dependencies
- [x] `tui/state.rs` — app state (list, detail, search, add modes)
- [x] `tui/ui.rs` — layout: status tabs, flicker list, detail pane
- [x] `tui/mod.rs` — event loop, keybindings
- [x] Keybindings: `/` search, `a` add, `d` delete, `s` cycle status, `q` quit

## Phase 3: Polish

- [ ] Error handling & edge cases
- [ ] Colored output, better formatting
- [ ] README with install instructions
- [x] TUI command bar (`:` trigger, slash commands: add/delete/search)
- [x] TUI command autocomplete (popup candidate list, ↓/Tab navigate, Enter execute)

## Phase 4: Supabase Sync

- [ ] Add `updated_at` to `Frontmatter` (backward-compatible, defaults to `created_at`)
- [ ] Stamp `updated_at = Utc::now()` on every write
- [ ] Add `reqwest` + `serde_json` dependencies
- [ ] `sync.rs` — SyncClient with pull/push via Supabase REST API
- [ ] `sync_state.rs` — persist `last_synced_at` to `~/.config/flicker/sync_state.toml`
- [ ] `flicker sync` subcommand
- [ ] Config: `supabase_url` and `supabase_anon_key` in config.toml
- [ ] TUI: Supabase fields in config popup
- [ ] TUI: sync on launch (if configured)
- [ ] Audio upload/download in sync
- [ ] Remove iCloud path logic (`icloud_path()`, conflict file handling)

---

# iOS App

## Phase 1: iOS Basic ✓

- [x] Xcode project with iCloud entitlement (`iCloud~com.flicker.app`)
- [x] `Flicker.swift` — model with frontmatter parsing
- [x] `StorageService.swift` — iCloud file I/O, conflict detection
- [x] `FlickerListView` — list with status filter
- [x] `FlickerDetailView` — view/edit single flicker
- [x] `NewFlickerView` — text input to create flicker

## Phase 2: Voice Input ✓

- [x] `SpeechService.swift` — Speech framework integration
- [x] AVAudioEngine recording + real-time transcription
- [x] Save audio as `audio/{id}.m4a`, link in frontmatter
- [x] UI: record button in NewFlickerView

## Phase 3: Polish

- [ ] Empty states, loading indicators
- [ ] Error handling & edge cases

## Phase 4: Supabase Sync

- [ ] Add `updated_at` to Flicker model (backward-compatible, defaults to `createdAt`)
- [ ] Stamp `updatedAt = Date()` on every save/updateStatus
- [ ] Add `supabase-swift` (~> 2.0) via SPM
- [ ] `SyncService.swift` — pull/push sync via SupabaseClient
- [ ] `SettingsView.swift` — Supabase URL, anon key, sync button
- [ ] Trigger sync on `onAppear` in list view (if configured)
- [ ] Audio upload/download in sync
- [ ] Remove iCloud storage logic (`forUbiquityContainerIdentifier:`, conflict filtering)
- [ ] Gear icon in FlickerListView → SettingsView
