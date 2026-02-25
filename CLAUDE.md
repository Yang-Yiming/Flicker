Flicker — lightweight app to record flickering ideas. Syncs via iCloud Drive between iOS (SwiftUI) and macOS (Rust CLI/TUI).

## Docs

- `DOCS/ARCHITECTURE.md` — architecture and tech details
- `DOCS/PLAN.md` — implementation plan (CLI + iOS App)
- `DOCS/PROGRESS.md` — current progress (CLI + iOS App)

READ these files when you need context. UPDATE them when you make changes.

## Key Facts

- Data format: Markdown + YAML frontmatter (`shared/file-format.md`)
- CLI: Rust + clap + ratatui. No args → TUI, subcommands for scripting
- iOS: SwiftUI + Speech framework
- iCloud container: `iCloud~com.flicker.app`
- File IDs: 8-char hex (truncated UUID v4)

---

# Reminders

Write down important things / errors here to help future sessions.
