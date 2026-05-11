# MCP Integration — Editor Configs

`unispec mcp` runs a JSON-RPC over stdio MCP server that exposes 31 built-in tools (plus dynamic `unispec_<name>` tools for each connector). This document shows how to wire it into five popular MCP-aware editors.

> **Looking for the tool list?** See [mcp-tools-reference.md](mcp-tools-reference.md). For mcp.md's design-level overview, see [mcp.md](mcp.md).

## Common shape

All editors below boot the same command:

```bash
unispec mcp
```

Optional first positional arg: the project path to operate against. If omitted, the server uses its current working directory.

```bash
unispec mcp /path/to/project
```

Each editor configures the command, args, and (optionally) the working directory.

## Claude Code (Desktop)

Edit `~/.config/Claude/claude_desktop_config.json` on Linux, `~/Library/Application Support/Claude/claude_desktop_config.json` on macOS:

```json
{
  "mcpServers": {
    "unispec": {
      "command": "unispec",
      "args": ["mcp"],
      "env": {
        "UNISPEC_ROOT": "/abs/path/to/your/project"
      }
    }
  }
}
```

Restart Claude Code. The 31 tools should appear in the tools dropdown.

If the `unispec` binary isn't on Claude Code's `PATH`, use an absolute path:

```json
{
  "mcpServers": {
    "unispec": {
      "command": "/home/you/.cargo/bin/unispec",
      "args": ["mcp", "/home/you/your-project"]
    }
  }
}
```

## Cursor

Open the Cursor settings → MCP → "Add new MCP server". Or edit `~/.cursor/mcp.json`:

```json
{
  "mcpServers": {
    "unispec": {
      "command": "unispec",
      "args": ["mcp"],
      "cwd": "/abs/path/to/your/project"
    }
  }
}
```

Cursor honours `cwd` instead of `env.UNISPEC_ROOT`, so passing the project via `cwd` works on every recent Cursor build.

## Windsurf

`~/.codeium/windsurf/mcp_config.json`:

```json
{
  "mcpServers": {
    "unispec": {
      "command": "unispec",
      "args": ["mcp"],
      "cwd": "/abs/path/to/your/project"
    }
  }
}
```

Restart Windsurf for it to pick up the new server.

## Cline (VS Code extension)

`~/.config/Code/User/globalStorage/saoudrizwan.claude-dev/settings/cline_mcp_settings.json`:

```json
{
  "mcpServers": {
    "unispec": {
      "command": "unispec",
      "args": ["mcp"],
      "env": {
        "UNISPEC_ROOT": "/abs/path/to/your/project"
      },
      "disabled": false,
      "autoApprove": []
    }
  }
}
```

`autoApprove` is Cline-specific: if you want to skip the per-call approval prompt for safe read-only tools, add them by name, e.g. `["topics_list", "tasks_list", "queue_check"]`.

## Zed

Zed reads MCP servers from its `settings.json` (Cmd/Ctrl+, then "Open Settings JSON"):

```json
{
  "context_servers": {
    "unispec": {
      "command": {
        "path": "unispec",
        "args": ["mcp"],
        "env": {
          "UNISPEC_ROOT": "/abs/path/to/your/project"
        }
      }
    }
  }
}
```

(Field name `context_servers` is Zed's, not the generic `mcpServers`.)

## Verifying the connection

Once an editor is configured, the simplest verification is to ask the agent to list tools. The set should include `topics_list`, `topics_add`, `spec_add`, `queue_add`, `tasks_list`, `tasks_complete`, `notes_add`, `index_add`, `unispec_read_spec`. If any of those are missing, the binary on `PATH` is an older revision — see [troubleshooting.md](troubleshooting.md).

A bare smoke test from any shell (no editor needed) — pipe a single JSON-RPC line:

```bash
printf '%s\n' '{"jsonrpc":"2.0","id":1,"method":"initialize","params":{}}' \
  '{"jsonrpc":"2.0","id":2,"method":"tools/list","params":{}}' \
  | unispec mcp /path/to/project 2>/dev/null \
  | grep -c '"name":"'
```

Should print `31` (plus one per configured connector).

## Project-root resolution

Three ways to point the server at a project, in order of precedence:

1. `unispec mcp /abs/path/to/project` — positional arg on the server command line.
2. `cwd` in the editor MCP config.
3. `UNISPEC_ROOT` environment variable.

If none are set, the server uses its OS-level current directory (likely your home directory). MCP tool calls then look for `spec/` in the wrong place. Always set at least one of the three.

## Dynamic connector tools

Any `[[connector]]` entry in `.agent/config.toml` becomes a dynamic MCP tool named `unispec_<connector-name>`. From the editor's perspective, these look like any other tool. See [connectors.md](connectors.md) for the configuration format and worked examples.

## See also

- [MCP Tools Reference](mcp-tools-reference.md) — every tool with required args and JSON-RPC examples.
- [Connectors](connectors.md) — how to expose shell commands as MCP tools.
- [Troubleshooting](troubleshooting.md) — "MCP server not connecting" and similar issues.
