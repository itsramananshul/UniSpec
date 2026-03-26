# MCP Integration

UniSpec speaks MCP (Model Context Protocol). This means you can connect AI agents like Claude, Cursor, Windsurf, and they understand your specs, workflows, and progress.

## What is MCP?

MCP is a protocol that lets AI tools interact with your project. Instead of just chatting with AI, it can:

- Read your specs
- See your workflows
- Run your connectors
- Query your progress
- Understand your project structure

## Supported Editors

UniSpec integrates with 24 editors:

| Editor | CLI Flag | Config Location |
|--------|----------|-----------------|
| Amazon Q Developer | `--amazon-q` | `.amazonq/prompts` |
| Antigravity | `--antigravity` | `.agent/workflows` |
| Augment CLI | `--auggie` | `.augment/commands` |
| Claude Code | `--claude-code` | `.claude/commands/unispec` |
| Cline | `--cline` | `.clinerules/workflows` |
| Codex | `--codex` | `~/.codex/prompts` |
| CodeBuddy | `--codebuddy` | `.codebuddy/commands/unispec` |
| Continue | `--continue` | `.continue/prompts` |
| CoStrict | `--costrict` | `.cospec/unispec/commands` |
| Crush | `--crush` | `.crush/commands/unispec` |
| Cursor | `--cursor` | `.cursor/commands` |
| Factory Droid | `--factory` | `.factory/commands` |
| Gemini CLI | `--gemini-cli` | `.gemini/commands/unispec` |
| GitHub | `--github` | `.github/prompts` |
| iFlow | `--iflow` | `.iflow/commands` |
| Kilo Code | `--kilo-code` | `.kilocode/workflows` |
| Kiro | `--kiro` | `.kiro/prompts` |
| OpenCode | `--opencode` | `.opencode/command` |
| Pi | `--pi` | `.pi/prompts` |
| Qoder | `--qoder` | `.qoder/commands/unispec` |
| Qwen Code | `--qwen-code` | `.qwen/commands` |
| RooCode | `--roo-code` | `.roo/commands` |
| Windsurf | `--windsurf` | `.windsurf/workflows` |
| TRAE | `--trae` | `.trae/rule` |

## Quick Setup

### Init with Editors

```bash
# Initialize with specific editors
unispec init --cursor --cline

# Or all of them
unispec init --all
```

This creates the necessary workflow files in each editor's config folder.

### Manual Setup

```bash
# For each editor you want to use
unispec init --cursor
unispec init --windsurf
unispec init --claude-code
```

## Using MCP Tools

Once configured, your AI editor has access to these tools:

### Read Specs

```
Read the spec for "User Login"
What's the current status of the API project?
Show me the acceptance criteria for the payment feature
```

### Manage Topics

```
Create a new topic called "API Redesign" in Staging
Push "User Login" to Building
Show all topics in the Ship area
```

### Run Connectors

```
Run the tests for the current topic
Run lint on the codebase
Run the build connector
```

### Query Index

```
What files are linked to the authentication topic?
Find all references to the user profile spec
Show me the code related to payment processing
```

## Connector MCP Tools

Create connectors that become AI-runnable commands:

```bash
# Add a test connector
unispec connector new test "Run test suite" "pytest" "tests/" "-v"

# Add a build connector  
unispec connector new build "Build project" "cargo" "build"

# Add a lint connector
unispec connector new lint "Run linter" "ruff" "check" "."
```

Now your AI can run:
- "Run the test connector"
- "Execute build"
- "Run lint and fix errors"

### Connector Structure

Connectors are defined in `.agent/config.toml`:

```toml
[[connector]]
name = "test"
description = "Run test suite"
command = "pytest"
args = ["tests/", "-v"]
timeout = 60
```

### Running Connectors

```bash
# Run a connector
unispec connector run test

# Run with arguments
unispec connector run test -- -k "test_user"

# List all connectors
unispec connector list

# Generate MCP config for connectors
unispec connector mcp
```

### Using Connectors in IDE

When you run `unispec init --cursor --cline --windsurf`, connectors are included in the workflow files. Your AI can then run them directly from chat:

- "Run the test connector" → Executes `pytest tests/ -v`
- "Run lint and fix errors" → Executes `ruff check .` then applies fixes
- "Build the project" → Executes `cargo build`

### Per-Project Connectors

Create connectors specific to your project:

```bash
# Custom build script
unispec connector new deploy-prod "Deploy to production" "./scripts/deploy.sh" "prod"

# Database migrations
unispec connector new migrate "Run migrations" "alembic" "upgrade" "head"

# Type checking
unispec connector new typecheck "Type check project" "mypy" "src/"
```

These are stored in `./.agent/config.toml` and available to your AI editor.

## MCP Server

Start the UniSpec MCP server for direct integration:

```bash
unispec mcp
```

This starts an MCP server that tools like Claude Desktop can connect to.

### Claude Desktop Config

Add to `~/.config/claude/settings.json`:

```json
{
  "mcpServers": {
    "unispec": {
      "command": "unispec",
      "args": ["mcp"]
    }
  }
}
```

### Available MCP Tools

When connected, these tools are available:

| Tool | Description |
|------|-------------|
| `list_topics` | List all topics, optionally by area |
| `get_topic` | Get details of a specific topic |
| `get_spec` | Read a topic's spec.md |
| `get_tasks` | Read a topic's tasks.md |
| `list_areas` | List all areas |
| `run_connector` | Execute a connector |
| `query_index` | Find files linked to topics |
| `get_progress` | Show progress across areas |

## Workflow Files

When you init with an editor, UniSpec creates workflow files in your project:

### Cursor/Windsurf

Files created in `./.cursor/commands/unispec/`:

```markdown
# unispec:spec.md

## When to use
Use this workflow when starting a new feature.

## Steps
1. Check the spec for existing requirements
2. Create a new topic if needed
3. Write clear acceptance criteria
4. Link relevant files
```

### Claude Code

Files created in `./.claude/commands/unispec/`:

```markdown
# unispec-commands

[TOOL_CALL]
{
  tool => 'unispec_list_topics',
  args => {
    --area Staging
  }
}
[/TOOL_CALL]
```

### Cline

Files created in `./.clinerules/workflows/unispec/`:

```markdown
# unispec-spec

Run when user wants to create or review a spec.

1. Check existing specs in ./spec/
2. Create new topic if needed
3. Use specs to guide implementation
```

### Manual IDE MCP Setup

If you want to connect UniSpec MCP directly to your editor:

#### Cursor
1. Go to Settings → MCP
2. Add new MCP server:
   - Command: `unispec`
   - Args: `mcp`
   - Cwd: `./`

#### Windsurf
1. Go to Settings → Extensions → MCP
2. Add server with same config as Cursor

#### Claude Desktop
Add to `~/.config/claude/settings.json`:

```json
{
  "mcpServers": {
    "unispec": {
      "command": "unispec",
      "args": ["mcp"],
      "env": {
        "UNISPEC_ROOT": "./"
      }
    }
  }
}
```

#### VS Code
Use the MCP extension:
1. Install "MCP" extension
2. Add to settings.json:

```json
{
  "mcpServers": {
    "unispec": {
      "command": "unispec",
      "args": ["mcp"],
      "cwd": "./"
    }
  }
}
```

## Custom MCP Integration

### For Custom Tools

If your tool supports MCP, add UniSpec:

```json
{
  "mcpServers": {
    "unispec": {
      "command": "path/to/unispec",
      "args": ["mcp"],
      "env": {
        "UNISPEC_ROOT": "/path/to/project"
      }
    }
  }
}
```

### Environment Variables

| Variable | Description |
|----------|-------------|
| `UNISPEC_ROOT` | Project root (default: current dir) |
| `UNISPEC_MODE` | Mode to use |

## Troubleshooting

### "Editor not found"

Some editors aren't in standard locations. Check:
```bash
# Find editor config
echo $HOME
ls -la $HOME/.cursor/  # or similar
```

### "Workflows not showing up"

Make sure you ran the init for that specific editor:
```bash
unispec init --cursor
```

### "MCP server won't start"

Check for port conflicts:
```bash
lsof -i :3456  # Default MCP port
```

## Tips

1. **Start simple** - Just `unispec init --cursor` to begin
2. **Link everything** - Use `unispec index add` to connect code to specs
3. **Run connectors** - Make AI run your tests and builds
4. **Update specs** - AI works better with accurate specs

---

## See Also

- [Commands Reference](commands.md) - CLI command documentation
- [Configuration Reference](configuration.md) - Config files, environment variables
- [Modes Documentation](modes.md) - Custom workflow configurations
- [Getting Started](getting-started.md) - Quick start guide

*See modes.md to create custom modes with their own MCP workflows, or commands.md for CLI reference.*
