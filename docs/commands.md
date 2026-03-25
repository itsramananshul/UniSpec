# Commands Reference

Complete reference for all UniSpec CLI commands.

## Init

Initialize a new UniSpec project.

```bash
unispec init [OPTIONS]
```

**Options:**
- `--root <path>` - Project root (default: current directory)
- `--cursor` - Add Cursor integration
- `--cline` - Add Cline integration  
- `--windsurf` - Add Windsurf integration
- `--claude-code` - Add Claude Code integration
- `--continue` - Add Continue integration
- `--antigravity` - Add Antigravity integration
- `--all` - Add all editor integrations

**Examples:**

```bash
# Basic init
unispec init

# Init with editor integrations
unispec init --cursor --windsurf
unispec init --all
```

## Area

Manage areas (Staging, Working, Ship).

```bash
# List areas
unispec area list

# Add area
unispec area add <name>

# Remove area
unispec area remove <name>

# Rename area
unispec area rename <old> <new>

# Show area health/stats
unispec area health

# Set default area
unispec area default <name>
```

## Topic

Manage topics (features, specs, tasks).

```bash
# Create topic
unispec topic add <name> [OPTIONS]

# List topics
unispec topic list [OPTIONS]

# Show topic details
unispec topic show <name>

# Move topic to area
unispec topic push <name> <area>

# Pull topic from area
unispec topic pull <name> <area>

# Remove topic
unispec topic remove <name>

# Show progress
unispec topic progress [OPTIONS]
```

**Topic Options:**
- `-a, --area <area>` - Specify area
- `-d, --description <desc>` - Topic description

**Examples:**

```bash
# Create topic in Staging
unispec topic add "User Login" -a Staging

# List topics in Working area
unispec topic list -a Working

# Move topic to Ship
unispec topic push "User Login" Ship

# Show all progress
unispec topic progress
```

## Mode

Manage modes (workflow configurations).

```bash
# List modes
unispec mode list

# Show mode info
unispec mode info <name>

# Switch active mode
unispec mode activate <name>

# Add custom mode
unispec mode add <path>

# Remove mode
unispec mode remove <name>

# Show current mode
unispec mode current
```

## Index

Manage links between topics and files.

```bash
# Add link
unispec index add --topic <name> --path <file>

# Remove link
unispec index remove --topic <name> --path <file>

# List links
unispec index list [OPTIONS]

# Find links
unispec index find <query> --by topic|path
```

**Examples:**

```bash
# Link a file to a topic
unispec index add --topic "user-login" --path src/auth/login.rs

# Find all links for a topic
unispec index find "user-login" --by topic

# List all links
unispec index list
```

## Connector

Manage custom commands (connectors) that become MCP tools.

```bash
# Create connector
unispec connector new <name> <description> <command> [args...]

# List connectors
unispec connector list

# Run connector
unispec connector run <name>

# Edit connector
unispec connector edit <name> <description>

# Delete connector
unispec connector delete <name>

# Generate MCP config
unispec connector mcp
```

**Examples:**

```bash
# Add a test runner connector
unispec connector new test "Run test suite" "pytest" "tests/" "-v"

# Add a linter
unispec connector new lint "Run linter" "ruff" "check" "."

# Run a connector
unispec connector run test

# Generate MCP config for Claude Desktop
unispec connector mcp > claude_desktop_config.json
```

## MCP

Launch the MCP server for agent integration.

```bash
# Start MCP server
unispec mcp
```

## Set

Set the current area or topic.

```bash
# Set current area
unispec set area <name>

# Set current topic
unispec set topic <name>
```

## Global Options

```bash
--help          # Show help
--version       # Show version
--root <path>   # Specify project root
```

## TUI Mode

Run `unispec` without any arguments to launch the interactive TUI.

**Navigation:**
| Key | Action |
|-----|--------|
| `↑/↓` | Move between topics |
| `→/←` | Navigate into/out of topics |
| `Enter` | Open topic or file |
| `n` | Create new topic |
| `r` | Remove topic |
| `p` | Push topic to area |
| `f` | Find files linked to topic |
| `g` | Go to area |
| `/` | Search topics |
| `\` | Toggle platypus |
| `q` | Quit |

---

*Questions? Check modes.md for custom workflows or mcp.md for AI integration.*
