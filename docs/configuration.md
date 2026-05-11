# Configuration Reference

Complete reference for UniSpec configuration files, environment variables, and settings.

---

## Configuration hierarchy

UniSpec searches three levels (most specific first):

1. **Local** — `./.agent/` (project-specific). First win.
2. **Global** — `~/.config/unispec/` (user-wide).
3. **System** — `/usr/share/unispec/` on Linux (install-wide).

Local values override global, which override system.

### Level 1: System

| Platform | Path |
|----------|------|
| Linux | `/usr/share/unispec/` |
| macOS | `/usr/local/share/unispec/` |
| Windows | `%ProgramData%\unispec\` |

Contains default modes, area templates, and system-wide workflows.

### Level 2: Global

| Platform | Path |
|----------|------|
| Linux / macOS | `~/.config/unispec/` |
| Windows | `%APPDATA%\unispec\` |

Contains user-installed modes, area templates, and preferences.

### Level 3: Local

```
project/
├── .agent/
│   ├── config.toml
│   ├── skill.md
│   ├── modes/
│   └── workflows/
└── spec/
    ├── Staging/
    ├── Working/
    └── Build/
```

---

## `.agent/config.toml` (Local)

Main project-level configuration. Fields below match what the CLI actually reads (`src/fs/config.rs`).

```toml
# Currently active mode (must match a directory name under .agent/modes/, ~/.config/unispec/.agent/modes/, or the system path).
current_mode = "default"

# Default area for tools that accept an optional `area` argument.
area = "Staging"

# Areas that refuse area_remove / destructive operations.
protected_areas = []

# Show the platypus mascot in the TUI.
paddy_enabled = false

# Ingest configuration — how `unispec ingest run` parses and stores code analysis.
[ingest]
auto_index = false                # Auto-add ingested files to the index
index_on_complete = false         # Re-index when a topic is marked complete
capture_functions = true
capture_structs = true
capture_enums = true
capture_imports = true
output_format = "toml"            # "toml" | "md" | "both"
languages = []                    # Empty = all supported: rust, javascript, typescript, python, go, bash

# Connectors — shell commands exposed as MCP tools (unispec_<name>).
[[connector]]
name = "test"
description = "Run test suite"
command = "pytest"
args = ["tests/", "-v"]
env = { RUST_BACKTRACE = "1" }
working_dir = "/project/root"
timeout = 120
```

### Field reference

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `current_mode` | string | `"default"` | Active mode (directory under `.agent/modes/`). |
| `area` | string | `"Staging"` | Default area used when a tool omits `area`. |
| `protected_areas` | array<string> | `[]` | Areas protected from deletion. |
| `paddy_enabled` | bool | `false` | Show the platypus mascot in the TUI. |

### `[ingest]`

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `auto_index` | bool | `false` | Add ingested files to the link index. |
| `index_on_complete` | bool | `false` | Re-index when a topic completes. |
| `capture_functions` | bool | `true` | Extract function declarations. |
| `capture_structs` | bool | `true` | Extract struct/class declarations. |
| `capture_enums` | bool | `true` | Extract enums. |
| `capture_imports` | bool | `true` | Extract imports/use statements. |
| `output_format` | string | `"toml"` | `"toml"` writes `spec/code_analysis.toml`; `"md"` writes per-file markdown; `"both"` writes both. |
| `languages` | array<string> | `[]` | Restrict parsing to these languages. Empty = all supported. |

### `[[connector]]`

Each connector becomes an MCP tool named `unispec_<name>` automatically.

| Field | Type | Description |
|-------|------|-------------|
| `name` | string | Lowercase, underscores only. |
| `description` | string | Shown in `connector_list` output. |
| `command` | string | Executable to run. |
| `args` | array<string> | Default args, prepended before any extra args. |
| `env` | table | Environment variables. |
| `working_dir` | string | Working directory. |
| `timeout` | integer | Timeout in seconds. |

Supported parser languages: `rust`, `javascript`, `typescript`, `python`, `go`, `bash`.

---

## Modes

See [modes.md](modes.md) for the full mode reference. A mode is a directory under `.agent/modes/<name>/` containing `mode.toml`, `skill.md`, optional `workflows/`, `areas/`, and `templates/`.

Mode search order:
1. Local: `./.agent/modes/`
2. Global: `~/.config/unispec/.agent/modes/`
3. System: `/usr/share/unispec/.agent/modes/`

First match wins.

---

## Area files

Each area has an `area.md` file in `spec/<area>/area.md`. Format:

```markdown
---
area: <AreaName>
short: <one-line description>
---

# <AreaName>

## Purpose
<what this area represents>

## Guidelines
- <what belongs here>
- <what does not>
```

Area templates are stored under `.agent/modes/<mode>/areas/<area>/area.md`. `unispec init` and `unispec area add` copy these into `spec/<area>/area.md` when creating areas.

---

## Connector MCP configuration

`unispec connector mcp` generates a `mcpServers` block you can drop into an editor's settings:

```json
{
  "mcpServers": {
    "unispec_test": {
      "command": "unispec",
      "args": ["connector", "run", "test"]
    }
  }
}
```

Most users instead use a single `unispec mcp` server that exposes both the static UniSpec tools and the dynamic `unispec_<name>` connector tools. See [mcp.md](mcp.md) for the standard setup.

---

## Environment variables

| Variable | Purpose | Default |
|----------|---------|---------|
| `UNISPEC_ROOT` | Override project root directory. | Auto-detected from current dir. |
| `UNISPEC_MODE` | Override active mode for this session. | Value of `current_mode`. |

---

## Exit codes

| Code | Meaning |
|------|---------|
| 0 | Success |
| 1 | General error |
| 2 | Invalid arguments |
| 3 | Project not initialized |
| 4 | Topic or area not found |

---

## See also

- [Commands Reference](commands.md)
- [Modes](modes.md)
- [MCP Integration](mcp.md)
- [Getting Started](getting-started.md)
