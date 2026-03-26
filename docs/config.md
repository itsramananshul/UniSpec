# Configuration Reference

Complete reference for UniSpec configuration files, environment variables, and settings.

---

## Configuration Hierarchy

UniSpec uses a three-level configuration hierarchy:

```
System (install-wide)
    ↓
Global (user-wide)  
    ↓
Local (project-specific)
```

### Level 1: System (Install-Wide)

Shared across all users on the system. Located at:

| Platform | Path |
|----------|------|
| Linux | `/usr/share/unispec/` |
| macOS | `/usr/local/share/unispec/` |
| Windows | `%ProgramData%\unispec\` |

Contains:
- Default modes (e.g., `simple`)
- Default area templates
- System-wide workflows

### Level 2: Global (User-Wide)

Per-user configuration. Located at:

| Platform | Path |
|----------|------|
| Linux | `~/.config/unispec/` |
| macOS | `~/.config/unispec/` |
| Windows | `%APPDATA%\unispec\` |

Contains:
- Custom modes (`.agent/modes/`)
- Custom area templates (`.agent/areas/`)
- User preferences

### Level 3: Local (Project-Specific)

Per-project configuration. Located in your project directory:

```
project/
├── .agent/           ← Project-level config
│   ├── config.toml
│   ├── modes/
│   └── workflows/
└── spec/             ← Topic specs
    ├── Staging/
    ├── Working/
    └── Build/
```

Contains:
- Project-specific config (`.agent/config.toml`)
- Active mode (local copy)
- Project topics

### Priority

When loading configurations, UniSpec searches in this order:
1. Local (project) → 2. Global (user) → 3. System (install)

This means:
- Local configs override global configs
- Global configs override system defaults
- You can always customize at the project level

---

## Configuration Files

### `.agent/config.toml` (Local)

Main configuration file for the UniSpec agent system. Created in your project root under `.agent/config.toml`.

```toml
# UniSpec Agent Configuration

# Current active mode (default: "simple")
current_mode = "simple"

# Default area for topic operations
default_area = "Working"

# Protected areas that cannot be deleted
protected_areas = ["Staging", "Working", "Build"]

# Connectors - Custom commands that become MCP tools
[[connector]]
name = "test"
description = "Run test suite"
command = "pytest"
args = ["tests/", "-v"]
env = { RUST_BACKTRACE = "1" }
working_dir = "/project/root"
timeout = 120
```

### Configuration Options

| Option | Type | Description | Default |
|--------|------|-------------|---------|
| `current_mode` | string | Active agent mode | `"simple"` |
| `default_area` | string | Default area for operations | `"Working"` |
| `protected_areas` | array | Areas that cannot be deleted | `["Staging", "Working", "Build"]` |

### Connector Configuration

Each connector in the config file:

| Option | Type | Description |
|--------|------|-------------|
| `name` | string | Connector identifier (lowercase, underscores only) |
| `description` | string | Human-readable description |
| `command` | string | Shell command to execute |
| `args` | array | Command arguments |
| `env` | table | Environment variables |
| `working_dir` | string | Working directory for command |
| `timeout` | integer | Timeout in seconds |

---

## Global Modes

Modes can be installed at the global level to be available across all projects.

### Global Mode Directory

```
~/.config/unispec/
└── .agent/
    └── modes/
        ├── simple/
        ├── sprint/
        └── custom-mode/
```

### Installing Modes Globally

Use `--global` flag when adding modes:

```bash
# Add a mode to global (user-wide) storage
unispec mode add /path/to/mymode --global

# Remove a global mode
unispec mode remove mymode --global
```

### Mode Search Order

When you run `unispec mode list` or `unispec mode activate`, modes are searched:

1. Local: `./.agent/modes/`
2. Global: `~/.config/unispec/.agent/modes/`
3. System: `/usr/share/unispec/.agent/modes/`

The first match wins.

---

## Area Files

Each area has an `area.md` file in `spec/<area>/area.md`.

### Local Areas

```
spec/
├── Staging/
│   └── area.md
├── Working/
│   └── area.md
└── Build/
    └── area.md
```

### Global Area Templates

```
~/.config/unispec/
└── .agent/
    └── areas/
        ├── staging/
        │   └── area.md
        ├── working/
        │   └── area.md
        └── custom/
            └── area.md
```

When you create a new area with `unispec area add`, UniSpec checks:
1. Local `.agent/areas/<name>/area.md`
2. Global `.agent/areas/<name>/area.md`
3. System `/usr/share/unispec/.agent/areas/<name>/area.md`

### Format

```markdown
# Area: <Area Name>

Description of this area's purpose.

## Topics

- Topic A
- Topic B

## Notes

Additional notes about this area.
```

---

## MCP Configuration

Generated automatically from connectors to `.agent/mcp.json`:

```json
{
  "mcpServers": {
    "unispec_test": {
      "command": "unispec",
      "args": ["connector", "run", "test"],
      "env": null
    }
  }
}
```

---

## Environment Variables

| Variable | Description | Default |
|----------|-------------|---------|
| `UNISPEC_ROOT` | Override project root directory | Auto-detected |
| `UNISPEC_CONFIG` | Custom config file path | `.agent/config.toml` |
| `UNISPEC_MCP_PORT` | MCP server port | `3000` |

---

## Exit Codes

| Code | Description |
|------|-------------|
| 0 | Success |
| 1 | General error |
| 2 | Invalid arguments |
| 3 | Project not initialized |
| 4 | Topic/area not found |

---

## See Also

- [Commands Reference](commands.md) - CLI command documentation
- [Modes Documentation](modes.md) - Custom workflow configurations
- [MCP Documentation](mcp.md) - AI agent integration
- [Getting Started](getting-started.md) - Quick start guide