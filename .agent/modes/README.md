# UniSpec Modes

A **mode** is a complete workflow configuration: areas, templates, workflows, skill prompt, and any per-area template overrides. The default mode lives in `.agent/modes/default/`.

## Mode directory layout

```
.agent/modes/<mode-name>/
├── mode.toml             # Required: metadata, areas, readiness rules, templates config
├── skill.md              # Required: agent persona for this mode
├── workflows/            # Optional: workflow prompts (spec.md, build.md, …)
├── areas/                # Optional: per-area templates
│   └── <area>/area.md
├── templates/            # Optional: global fallback templates
│   ├── topic.md
│   ├── spec.md
│   ├── task.md
│   └── area.md
└── system_prompts/       # Optional: additional prompt fragments loaded by the agent
```

## Creating a mode

1. Create the directory: `mkdir -p .agent/modes/<mode-name>/`.
2. Write `mode.toml`. Mandatory tables: `[mode]`, `[areas]`, `[templates]`. See `default/mode.toml` for a working example.
3. Write `skill.md` describing the agent persona.
4. Add any custom workflows under `workflows/`.
5. Add per-area templates under `areas/<area>/` and global fallback templates under `templates/`.

## Activating a mode

Modes are activated through the CLI:
```bash
unispec mode list
unispec mode activate <mode-name>
unispec mode current
```

`unispec mode activate` is **not** exposed as an MCP tool — agents cannot switch modes for the user. The user must run it.

## Global modes

Modes installed under `~/.config/unispec/.agent/modes/` are available to every project. Use `unispec mode add <path> --global` to install one globally; `unispec mode remove <name> --global` to remove it.

Mode search order (first match wins):
1. Local: `./.agent/modes/`
2. Global: `~/.config/unispec/.agent/modes/`
3. System: `/usr/share/unispec/.agent/modes/`

## Area templates

When `unispec init` creates a new area, it copies the template at `.agent/modes/<mode>/areas/<area>/area.md` (or the global fallback `.agent/modes/<mode>/templates/area.md`) into `spec/<area>/area.md`.

Areas defined in `mode.toml`:
```toml
[areas]
default = ["Staging", "Working", "Testing", "Fixing", "Build"]
protected = ["Build"]
default_area = "Staging"
```

`protected` areas refuse `areas_remove` and `topics_delete` until you remove the protection.

## Readiness queue

Areas listed under `[readiness].areas` require a topic to appear in `spec/<area>/queue.md` before `topics_push` will move it out. Configure per mode in `mode.toml`:

```toml
[readiness]
queue_file = "queue.md"
required_for_push = true
areas = ["Staging", "Fixing"]
```
