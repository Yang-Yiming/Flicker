# Flicker Progress

## Current Phase: Phase 1 (CLI Core) ✓

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

### Next Up

- Phase 2: TUI — ratatui interactive interface
