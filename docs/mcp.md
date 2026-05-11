# MCP Integration

UniSpec ships an MCP (Model Context Protocol) server so editors like Claude, Cursor, Windsurf, Cline, and any MCP-aware client can read your specs and drive your workflow.

## What MCP lets you do

- Inspect every area, topic, spec, and task.
- Create and update topics, specs, and tasks through structured tool calls.
- Move topics through the pipeline and manage the readiness queue.
- Link source files to specs and query the index.

## Available MCP tools (ground truth)

The MCP server publishes exactly these 31 tools (plus one dynamic tool per connector you've defined). This list is generated from `src/mcp/mod.rs::get_tools()`; if you don't see a tool here, it's not exposed via MCP.

### Areas

| Tool | Required args | Description |
|------|---------------|-------------|
| `areas_list` | — | List all areas. |

### Topics

| Tool | Required args | Description |
|------|---------------|-------------|
| `topics_list` | — (`area?` defaults to `Staging`) | List topics in an area. |
| `topics_add` | `topic, area, short, content` | Create topic dir + `topic.md`. `content` must be ≥ 10 chars. Server writes frontmatter; don't include `---` in your `content`. |
| `topics_show` | `topic` (`area?`, `show_all?`, `from?`) | Show files in a topic. |
| `topics_delete` | `topic` (`area?`, `force?`) | Delete a topic. |
| `topics_push` | `topic, area` (`source_area?`) | Move topic to another area. Source areas listed in `[readiness]` (default: `Staging`, `Fixing`) require the topic to be in `<source>/queue.md` first. |
| `topics_pull` | `topic, source_area` | Pull a topic into `Working`. |
| `topics_progress` | — (`area?`) | Per-topic task counts. |

### Asset reading

| Tool | Required args | Description |
|------|---------------|-------------|
| `read_asset` | `topic, asset_type` (`area?`) | `asset_type ∈ {"topic", "spec", "task"}`. Use `topic: "templates"` to read templates from `.agent/modes/default/templates/`. |
| `unispec_read_spec` | `topic` (`area?`) | Returns spec + task content together. |

### Specs & tasks

| Tool | Required args | Description |
|------|---------------|-------------|
| `spec_add` | `topic, area, short, spec_content, task_content` | Creates `<topic>_spec.md` and `<topic>_task.md` (slashes/spaces in `topic` become `-`). Both content fields must be ≥ 10 chars. |
| `spec_write` | `topic, content` (`area?`) | Overwrite an existing spec. |
| `task_write` | `topic, content` (`area?`) | Overwrite an existing task file. **Fails** if the spec doesn't exist. |
| `task_status` | `topic, status` (`area?`) | Update frontmatter `status:`. Allowed: `pending`, `working`, `complete`. Does not touch `- [ ]` checkboxes. |
| `tasks_list` | `topic` (`area?`) | List task lines + checkbox state. |
| `tasks_complete` | `topic, task_index` (`note?`, `area?`) | Flip the 0-indexed task to `[x]`. |
| `tasks_incomplete` | `topic, task_index` (`note?`, `area?`) | Flip the 0-indexed task back to `[ ]`. |

### Notes

| Tool | Required args | Description |
|------|---------------|-------------|
| `notes_read` | `topic` (`area?`) | Read the notes section of `<topic>_task.md`. |
| `notes_add` | `topic, note` (`area?`) | Append a note. |

### Readiness queue

| Tool | Required args | Description |
|------|---------------|-------------|
| `queue_list` | — (`area?`) | Show `spec/<area>/queue.md`. |
| `queue_add` | `topic` (`position?`, `area?`) | `position: 0` = first; default last. |
| `queue_remove` | `topic` (`area?`) | Remove topic from queue. |
| `queue_check` | `topic` (`area?`) | Returns `ready: true|false`. |
| `queue_reorder` | `topic, new_position` (`area?`) | Move a topic in the queue. |

### Index (file ↔ spec linking)

| Tool | Required args | Description |
|------|---------------|-------------|
| `index_add` | `topic, path` (`area?`, `link_type?`, `tags?`, `annotation?`) | Link a file or directory to a topic. |
| `unispec_bind_spec` | `spec_path, file_path, topic` (`area?`) | Bind a code file to a spec record. |
| `index_find` | `query` (`by?`) | `by ∈ {"topic", "path", "tag"}`, default `"topic"`. |
| `index_lookup` | `id` | `id` is `topic:name`. |
| `index_list` | — (`topic?`, `path?`, `tag?`) | List all links with optional filters. |
| `index_graph` | — | Export the link graph as JSON. |
| `index_backlinks` | `topic` | Backlinks markdown block for a topic. |

## Tools NOT exposed via MCP

These commands exist in the CLI but are intentionally not in the MCP surface. Calling them as MCP tool names will fail.

- Area mutation: `areas_add`, `areas_remove`, `areas_rename`, `areas_default`, `areas_health` (use `unispec area …` from the shell).
- Index mutation/inspection: `index_remove`, `index_cleanup`, `index_tags`, `index_exports`, `index_query`, `index_depends`, `index_callers`, `index_full`, `index_watch`.
- Config: `config_get`, `config_set`.
- Mode: `mode_list`, `mode_info`, `mode_activate`, `mode_current`.
- Connector management: `connector_list`, `connector_run` (each named connector is, however, exposed dynamically as `unispec_<name>` — see below).
- Ingest/parse: `code_analysis`, `code_parse` (use `unispec ingest run` / `unispec parse file`).
- Setup: `unispec init`, `unispec pkg …`, `unispec mode add|remove`, `unispec connector new|delete|edit`, `unispec patty …`.

If you need to drive these from an agent, shell out to the CLI.

## Dynamic connector tools

Every connector defined in `.agent/config.toml` becomes an MCP tool named `unispec_<connector_name>`:

```toml
[[connector]]
name = "test"
description = "Run test suite"
command = "pytest"
args = ["tests/", "-v"]
timeout = 60
```

Exposes:

```json
{
  "name": "unispec_test",
  "description": "Run test suite",
  "inputSchema": {
    "type": "object",
    "properties": {
      "args": {
        "type": "array",
        "items": {"type": "string"},
        "description": "Extra arguments appended to the connector command"
      }
    }
  }
}
```

The implementation just shells out to `unispec connector run <name> -- <extra-args>` and returns combined stdout/stderr.

## Starting the MCP server

```bash
# Default: current directory as project root
unispec mcp

# Explicit root
unispec mcp /path/to/project
```

It speaks JSON-RPC over stdio. Each line is one complete JSON message.

### Claude Desktop

`~/.config/claude/settings.json`:

```json
{
  "mcpServers": {
    "unispec": {
      "command": "unispec",
      "args": ["mcp"],
      "env": { "UNISPEC_ROOT": "/abs/path/to/project" }
    }
  }
}
```

### Cursor / Windsurf / Continue / VS Code (MCP)

Use the same shape:

```json
{
  "mcpServers": {
    "unispec": {
      "command": "unispec",
      "args": ["mcp"],
      "cwd": "/abs/path/to/project"
    }
  }
}
```

For `unispec init --cursor` etc., UniSpec drops workflow files into the editor's commands directory. These are prose prompts, not MCP — they instruct the agent on which MCP tools to call.

## Calling tools

Once connected, your agent can call tools by name. Example (Claude):

```
"Read the spec for user-login in Working"
→ unispec_read_spec { "topic": "user-login", "area": "Working" }

"Mark task 2 of payment-api done"
→ tasks_complete { "topic": "payment-api", "task_index": 1 }   # 0-based

"Link src/auth/login.rs to user-login as an implementation"
→ index_add { "topic": "user-login", "path": "src/auth/login.rs", "link_type": "implementation" }
```

## Argument quirks worth remembering

- `topics_add` and `spec_add` **strip any frontmatter** you include in `content` / `spec_content` / `task_content` and prepend their own. Just provide the body.
- `task_write` **fails** if the spec file doesn't already exist for that topic; call `spec_add` first.
- `task_index` is **0-based** for `tasks_complete` and `tasks_incomplete`.
- `topics_push` from `Staging` or `Fixing` **requires** the topic to appear in `<area>/queue.md`. Use `queue_check` before `topics_push`.
- The default `area` for every tool that accepts one is `Staging`.

## Environment variables

| Variable | Purpose |
|----------|---------|
| `UNISPEC_ROOT` | Override project root (default: current dir). |
| `UNISPEC_MODE` | Override the active mode for this MCP session. |

---

## See also

- [Commands Reference](commands.md)
- [Configuration Reference](configuration.md)
- [Modes](modes.md)
- [Getting Started](getting-started.md)
