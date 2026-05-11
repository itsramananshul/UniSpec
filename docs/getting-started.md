# Getting Started with UniSpec

UniSpec is a spec-driven workflow tool. You write a spec for each feature, then move it through a fixed pipeline of areas (default: `Staging → Working → Testing → Fixing → Build`). Code is linked back to specs, so AI agents and humans both know exactly what's being built and why.

This guide walks through the basics.

---

## What is UniSpec?

For each feature, you produce three files under `spec/<Area>/<topic>/`:

| File | Purpose |
|------|---------|
| `topic.md` | What this topic is — short description, sub-topics, notes. |
| `<topic>_spec.md` | **What** you're building: requirements, examples, data model. |
| `<topic>_task.md` | The implementation tasks, each tracked with `- [ ]` / `- [x]`. |

UniSpec's MCP server exposes tools that an AI agent (Claude, Cursor, Windsurf, Cline, etc.) calls directly — so the agent can create topics, write specs, flip checkboxes, and link code without you copy-pasting commands.

---

## Installation

```bash
# Linux / macOS (Cargo)
cargo install unispec

# Arch Linux (AUR)
yay -S unispec
```

Verify:

```bash
unispec --version
```

---

## First project

### 1. Initialize

```bash
mkdir my-first-project
cd my-first-project
unispec init
```

This creates:

```
my-first-project/
├── spec/
│   ├── Staging/area.md
│   ├── Working/area.md
│   ├── Testing/area.md
│   ├── Fixing/area.md
│   └── Build/area.md
└── .agent/
    ├── config.toml
    ├── skill.md
    ├── modes/default/
    └── workflows/
```

The areas come from the default mode (`.agent/modes/default/mode.toml`).

### 2. Launch the TUI

```bash
unispec
```

Areas appear on the left; topics show up inside each area once you create them. The platypus mascot is off by default — toggle with `\`.

### 3. Create a topic

In the TUI:
1. `↓` to highlight `Staging`, `→` to enter.
2. `n` to create a topic.
3. Type a name (kebab-case recommended, e.g., `user-login`).
4. `Enter`.

That creates `spec/Staging/user-login/topic.md`. The spec file (`user-login_spec.md`) and task file (`user-login_task.md`) are written when you (or your agent) run `spec_add` through the MCP server.

---

## File layout the tools create

```
spec/
└── <Area>/
    └── <topic>/
        ├── topic.md
        ├── <topic>_spec.md      # slashes/spaces in <topic> become '-'
        └── <topic>_task.md
```

For a nested topic `auth/login`, the files are `auth-login_spec.md` and `auth-login_task.md` inside `spec/<Area>/auth/login/`.

---

## The pipeline

| Area | Stage |
|------|-------|
| Staging | Writing the spec. |
| Working | Implementing code in `src/`. |
| Testing | Running build/test pipelines. |
| Fixing | Repairing issues found in Testing. |
| Build | Done. Treated as immutable. |

To move a topic between areas:

```bash
unispec topic push <topic> <target-area>
```

Or in the TUI, select the topic and press `p`.

`Staging` and `Fixing` require the topic to be listed in `spec/<area>/queue.md` before push (the "readiness queue"). The CLI doesn't have a direct command for that yet — your agent or the MCP `queue_add` tool handles it.

---

## TUI cheatsheet

| Key | Action |
|-----|--------|
| `↑` / `↓` | Move between items |
| `→` | Enter an area or topic |
| `←` | Go back |
| `Enter` | Open the highlighted file in your default editor |
| `n` | Create new topic |
| `r` | Remove topic (confirm) |
| `p` | Push topic to another area |
| `f` | Find files linked to this topic |
| `/` | Search/filter |
| `\` | Toggle the platypus mascot |
| `q` | Quit |

Your specs are plain Markdown — open them in any editor. `unispec topic list` works from the command line too.

---

## Quick CLI reference

```bash
unispec                      # launch TUI
unispec init                 # initialize project
unispec topic add "Feature"  # create a topic in the default area (Staging)
unispec topic list           # list topics in the default area
unispec topic progress       # task progress per area
unispec topic push <name> <area>
unispec area list
unispec area health
unispec index add --topic <name> --path <path>
unispec mode list
unispec mode activate <mode>
unispec mcp                  # start the MCP server (for agents)
```

For the complete CLI surface, see [commands.md](commands.md).

---

## Connecting an AI agent

UniSpec speaks MCP. To let an agent drive the pipeline:

```bash
unispec init --cursor --cline --windsurf --claude_code
```

Drops workflow files into the right editor folders. Or wire the MCP server directly:

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

See [mcp.md](mcp.md) for the full MCP tool surface and per-editor configs.

---

## Indexing code to specs

When an agent (or you) writes a code file, link it to a topic so the spec ↔ code relationship is preserved:

```bash
unispec index add --topic user-login --path src/auth/login.rs \
  --link_type implementation \
  --tags "auth,backend" \
  --annotation "Core login handler"
```

The link is stored in `spec/index.toml`. Agents can query it with `index_find`, `index_list`, `index_backlinks`, and `index_graph`.

See [indexing.md](indexing.md) for the full feature set.

---

## What's next?

- [Commands Reference](commands.md) — all CLI commands and flags
- [MCP Integration](mcp.md) — every MCP tool the server exposes
- [Modes](modes.md) — customize the pipeline (areas, templates, workflows)
- [Configuration](configuration.md) — `.agent/config.toml`, env vars, exit codes
- [Indexing](indexing.md) — link code to specs

---

*Write the spec first. Code second.*
