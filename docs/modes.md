# Creating Modes

Modes define how you work with UniSpec. The default "Simple Mode" uses Staging → Building → Ship. But you can create custom modes for different workflows.

## What is a Mode?

A mode is a complete workflow configuration containing:

- **Workflows** - Step-by-step guides for your process
- **Areas** - Where topics live at each stage
- **Skill** - How AI agents should behave
- **Templates** - Starting points for specs and tasks
- **Connectors** - Commands to run

## Mode Directory Structure

```
.agent/modes/<mode-name>/
├── mode.toml          # Mode metadata
├── skill.md          # Agent persona
├── workflows/        # Workflow definitions
│   ├── feature:spec.md
│   ├── feature:build.md
│   └── feature:verify.md
├── areas/            # Area templates
│   ├── staging/
│   │   └── area.md
│   ├── building/
│   │   └── area.md
│   └── ship/
│       └── area.md
└── templates/         # Topic templates
    ├── spec.md
    └── tasks.md
```

## Creating a Mode

### 1. Create the directory

```bash
mkdir -p .agent/modes/sprint
```

### 2. Create mode.toml

```toml
[mode]
name = "sprint"
display_name = "Sprint Mode"
version = "1.0.0"

[author]
name = "Your Name"
email = "you@example.com"

[mode.description]
short = "Two-week sprint workflow"
long = """
Sprint Mode follows a two-week sprint cycle:
1. Sprint planning creates specs
2. Daily standups update progress
3. Sprint review moves to done
"""

[areas]
default = ["Backlog", "In Sprint", "Review", "Done"]
protected = ["In Sprint", "Review"]

[capabilities]
spec_writing = true
building = true
verification = true
connectors = true
custom_workflows = true
```

### 3. Create skill.md

```markdown
# Sprint Mode Agent Persona

You are a sprint-focused developer who values:
- Incremental progress
- Clear sprint goals
- Daily momentum

Your approach:
1. Check sprint backlog first
2. Update task status daily
3. Flag blockers immediately
4. Focus on completing, not starting
```

### 4. Create workflows

Create `.agent/modes/sprint/workflows/sprint:plan.md`:

```markdown
# Sprint Planning

1. Review backlog items
2. Estimate effort (t-shirt sizes or story points)
3. Select items for sprint
4. Break into tasks
5. Assign to team members
6. Set sprint goal
```

Create `.agent/modes/sprint/workflows/sprint:standup.md`:

```markdown
# Daily Standup

1. What did I complete yesterday?
2. What will I do today?
3. Any blockers?
```

### 5. Create area templates

Create `.agent/modes/sprint/areas/backlog/area.md`:

```markdown
# Backlog

Items ready for prioritization but not yet in a sprint.

## Criteria for Backlog
- Has a spec
- Has acceptance criteria
- Is prioritized by product owner
```

Create `.agent/modes/sprint/areas/in-sprint/area.md`:

```markdown
# In Sprint

Currently being worked on in this sprint.

## Rules
- Each task has an owner
- Blockers must be flagged
- Daily updates required
```

### 6. Create templates

Create `.agent/modes/sprint/templates/spec.md`:

```markdown
# [Feature Name]

## Sprint Goal
[One sentence about what we're achieving]

## From Backlog
- Original spec: [link]
- Priority: [high/medium/low]
- Estimate: [XS/S/M/L/XL]

## This Sprint
- What we're building: [specific scope]
- What we're NOT building: [out of scope]

## Tasks
- [ ] ...
```

## Activating a Mode

```bash
# List available modes
unispec mode list

# Activate your mode
unispec mode activate sprint

# Check current mode
unispec mode current
```

## Installing Modes

### Local Modes

Modes in your project's `.agent/modes/` directory are available only to that project.

```bash
# Add a local mode (project-specific)
unispec mode add ./modes/my-custom-mode
```

### Global Modes

Modes in your user config directory are available across all projects.

```bash
# Add a mode globally (available to all projects)
unispec mode add /path/to/mymode --global

# Remove a global mode
unispec mode remove mymode --global
```

Global modes are stored in `~/.config/unispec/.agent/modes/` and searched when:
- Listing modes (`unispec mode list`)
- Activating a mode (`unispec mode activate`)
- Running the MCP server

### Mode Search Order

Modes are searched in this order:
1. Local: `./.agent/modes/`
2. Global: `~/.config/unispec/.agent/modes/`
3. System: `/usr/share/unispec/.agent/modes/`

The first match wins.

## Installing Modes from Repository

You can install pre-built modes from the community package repository:

```bash
# List available packages
unispec pkg list

# Search for modes
unispec pkg search sprint

# Install a mode package
unispec pkg install sprint-mode

# Install globally (available to all projects)
unispec pkg install sprint-mode --global

# List installed packages
unispec pkg installed
```

Packages can include modes, connectors, and workflows. See [Commands Reference](commands.md#pkg) for full details.

## Mode Types

### Simple Mode (Default)
- Staging → Building → Ship
- Good for: Solo devs, small teams

### Sprint Mode
- Backlog → In Sprint → Review → Done
- Good for: Agile teams, two-week cycles

### Kanban Mode
- To Do → In Progress → Code Review → Done
- Good for: Continuous delivery, Ops teams

### Docs Mode
- Draft → Review → Published
- Good for: Documentation projects

### RFC Mode
- Proposed → Discussion → Approved/Rejected
- Good for: Architecture decisions

## Sharing Modes

Modes are just directories. Share them by:

1. **Git**: Commit the mode folder
2. **Template**: Create a template repo
3. **Distribution**: Package as a tarball

Example sharing:
```bash
# Export mode
tar -cvzf my-mode.tar.gz .agent/modes/my-mode/

# Import mode
tar -xvzf my-mode.tar.gz -C /path/to/project/.agent/modes/
```

## Mode Configuration Options

### mode.toml Reference

```toml
[mode]
name = "simple"
display_name = "Simple Mode"
description = "Default mode with Spec-Driven Development workflows. Staging: specs being written. Working: specs being built. Build: shippable code."
version = "1.0.0"

[author]
name = "OpenSDD Team"
contact = "https://github.com/osdd"

[requirements]
min_osdd_version = "0.9.0"

[areas]
default = ["Staging", "Working", "Build"]
protected = ["Staging", "Working", "Build"]
default_area = "Working"

[capabilities]
spec_writing = true
building = true
verification = true
connectors = true
custom_workflows = false

[dependencies]
extends = []

[scripts]
pre_activate = ""
post_activate = ""
```

## Best Practices

1. **Start simple** - Don't over-engineer your first mode
2. **Document the why** - Your skill.md explains agent behavior
3. **Share templates** - Help teammates get started
4. **Iterate** - Modes evolve with your team

---

## See Also

- [Commands Reference](commands.md) - CLI command documentation
- [Configuration Reference](configuration.md) - Config files, environment variables
- [MCP Documentation](mcp.md) - AI agent integration
- [Getting Started](getting-started.md) - Quick start guide

*Need help? Check commands.md for CLI usage or mcp.md to connect AI agents to your mode.*
