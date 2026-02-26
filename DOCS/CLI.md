# Flicker CLI Reference

`flicker` with no arguments launches the interactive TUI. All subcommands are for scripting and automation.

---

## flicker add

Create a new flicker.

```
flicker add "<text>"
```

**Example:**
```
flicker add "look into diffusion transformers"
```

Creates a `.md` file in the flickers directory with a generated 8-char hex ID, `status: inbox`, and `source: cli`.

---

## flicker list

List flickers, sorted by creation date (newest first). Deleted flickers are hidden by default.

```
flicker list [--status <status>]
```

**Statuses:** `inbox` | `kept` | `archived` | `deleted`

**Examples:**
```
flicker list
flicker list --status kept
flicker list --status deleted
```

---

## flicker show

Show full details of a single flicker.

```
flicker show <id>
```

**Example:**
```
flicker show a1b2c3d4
```

Prints frontmatter fields and body text.

---

## flicker delete

Soft-delete a flicker (sets `status: deleted`). Does not remove the file.

```
flicker delete <id>
```

**Example:**
```
flicker delete a1b2c3d4
```

---

## flicker search

Case-insensitive full-text search across all flicker bodies.

```
flicker search <query>
```

**Example:**
```
flicker search "transformer"
```

---

## flicker status

Show a summary of flicker counts by status, and report any iCloud conflict files.

```
flicker status
```

**Example output:**
```
inbox:    5
kept:     12
archived: 3
deleted:  1

No conflicts.
```

---

## flicker rename

Replace the body text of a flicker.

```
flicker rename <id> "<new body>"
```

**Example:**
```
flicker rename a1b2c3d4 "look into diffusion transformers (DiT)"
```

---

## flicker bash

Run an arbitrary shell command via `sh -c`. Useful for scripting.

```
flicker bash "<shell command>"
```

**Example:**
```
flicker bash "flicker list --status inbox | wc -l"
```

---

## flicker config

Manage CLI configuration. Config is stored at `~/.config/flicker/config.toml`.

**Keys:**
| Key | Default | Description |
|-----|---------|-------------|
| `editor` | `nvim` or `vim` | Editor used by TUI `v` shortcut |
| `shell` | value of `$SHELL` | Shell used for `bash` command |

### config list

Print all config keys and their current values.

```
flicker config list
```

**Example output:**
```
editor = nvim
shell = zsh
```

### config get

Print the value of a single key.

```
flicker config get <key>
```

**Example:**
```
flicker config get editor
# → nvim
```

### config set

Set the value of a single key.

```
flicker config set <key> <value>
```

**Examples:**
```
flicker config set editor hx
flicker config set shell bash
```

---

## TUI Keybindings

Launched with `flicker` (no args).

| Key | Action |
|-----|--------|
| `j` / `k` | Navigate list |
| `Enter` | Open detail view |
| `Esc` | Back / cancel |
| `a` | Add new flicker |
| `d` | Delete selected |
| `s` | Cycle status |
| `v` | Open in editor (`$editor` from config) |
| `/` | Search |
| `Tab` | Next status tab |
| `:` | Command bar (supports `add`, `delete`, `search`) |
| `!` | Bash input bar |
| `q` | Quit |

---

## Environment Variables

| Variable | Description |
|----------|-------------|
| `FLICKER_DIR` | Override iCloud directory path (useful for local dev/testing) |
