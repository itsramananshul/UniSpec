# UniSpec Agent Skill

This is the core UniSpec knowledge base. It covers how the framework works, MCP tools, and universal best practices. This file should not change - it applies to all UniSpec projects.

---

## Part 1: How UniSpec Works

### The Core Principle

**The spec is the contract.** It is the single source of truth that defines what problem is solved, what requirements must be met, and what "done" means.

Before writing code: read the spec. Before changing code: update the spec.

### The Three Areas

Areas are containers for work. They represent stages of completion.

| Area | Meaning | Shows |
|------|---------|--------|
| **Roadmap** | What might be needed | Proposal with type and severity |
| **Working** | Currently being built | Task progress bars |
| **Build** | Complete and shippable | Completion dates |

### The Work Flow

```
Roadmap ──→ Working ──→ Build
   │            │            │
   ▼            ▼            ▼
 Propose    Implement    Shipped
```

Work moves **forward** through areas. When a topic is done in Working, push to Build.

### Topics

A topic is a unit of work. It contains:
- `spec.md` or `specs.md` - The specification
- `tasks.md` - Implementation checklist

---

## Part 2: MCP Tools

These tools let you query and modify specs, index, and code analysis. **Use these tools - don't guess.**

### Topic Management

| Tool | Arguments | Description |
|------|-----------|-------------|
| `topics_list` | `area` | List all topics in an area |
| `topics_show` | `topic`, `area`, `from`, `all` | Show topic content and files |
| `topics_add` | `topic`, `area` | Create a new topic |
| `topics_delete` | `topic`, `area`, `force` | Delete a topic |
| `topics_push` | `topic`, `target`, `from` | Move topic to another area |
| `topics_pull` | `topic`, `source` | Pull topic from another area |
| `topics_progress` | `area` | Show completion statistics |

### Area Management

| Tool | Arguments | Description |
|------|-----------|-------------|
| `areas_list` | - | List all areas |
| `areas_add` | `name` | Create a new area |
| `areas_remove` | `name` | Remove an area |
| `areas_rename` | `old`, `new` | Rename an area |
| `areas_default` | `name` | Set default area |
| `areas_health` | - | Show area health statistics |

### Index (Code ↔ Spec Links)

The index connects specs to code files. Use to find what code implements what.

| Tool | Arguments | Description |
|------|-----------|-------------|
| `index_list` | `topic`, `area`, `path`, `tag` | List files linked to topic |
| `index_add` | `topic`, `area`, `path`, `link_type`, `tags`, `annotation` | Link file to topic |
| `index_remove` | `topic`, `path` | Remove a link |
| `index_find` | `query`, `by` | Find links by topic/path/tag/annotation |
| `index_cleanup` | - | Remove orphaned links |
| `index_tags` | - | List all unique tags |
| `index_graph` | - | Export index as graph JSON |
| `index_backlinks` | `topic` | Generate backlinks markdown |
| `index_exports` | `topic` | List exports (functions, classes) for topic |
| `index_query` | `query`, `by` | Query exports by name/type/description/ID |
| `index_depends` | `topic` | Find what topics depend on this topic |
| `index_lookup` | `id` | Find export by full ID (e.g., `user-login:login_user`) |

### Code Analysis

| Tool | Arguments | Description |
|------|-----------|-------------|
| `code_analysis` | `topic`, `area`, `module`, `item_type`, `pattern` | Query already-ingested code |
| `code_parse` | `path`, `language`, `item_type`, `pattern` | Parse file on-demand with tree-sitter |

### Mode Management

| Tool | Arguments | Description |
|------|-----------|-------------|
| `mode_list` | - | List all available modes |
| `mode_info` | `name` | Get detailed info about a mode |
| `mode_activate` | `name` | Activate an agent mode |
| `mode_current` | - | Get current active mode |

### Configuration

| Tool | Arguments | Description |
|------|-----------|-------------|
| `config_get` | - | Get current configuration |
| `config_set` | `area` | Set default area |

### Connectors

| Tool | Arguments | Description |
|------|-----------|-------------|
| `connector_list` | - | List all available connectors |
| `connector_run` | `name`, `args` | Run a connector command |

---

## Part 3: How to Query

### Before implementing, query existing data:

1. **Find specs**: `topics_list(area="Roadmap")`
2. **Read spec**: `topics_show(topic, area="Roadmap")`
3. **Find code**: `index_find(query)` or `code_parse(path, pattern)`
4. **Check exports**: `index_exports(topic)` - find functions available

### When linking code:

```
index_add(topic, path, tags, annotation)
```

### When referencing other topics' code:

```python
# Include reference comment for tracking
from auth import login_user  # ref:index:user-login:login_user
```

---

## Part 4: Frontmatter

Specs support YAML frontmatter for metadata:

```yaml
---
title: Feature Name
impact: high           # critical, high, medium, low
change_type: feature   # feature, bugfix, refactor, docs, security
status: proposed      # proposed, in_progress, completed
created: 2026-03-28
completed: 2026-03-28  # Added when pushed to Build
---
```

---

## Part 5: Task Syntax

Tasks track implementation progress. Use checkboxes:

```markdown
## Tasks

### Phase 1
- [ ] Not started task
- [-] Task in progress
- [x] Completed task
```

| Marker | Meaning |
|--------|---------|
| `[ ]` | Not started |
| `[-]` | In progress |
| `[x]` | Done |

Mark tasks as you work. This keeps progress visible.

---

## Part 6: Spec Structure

A spec answers three questions:

1. **Problem** - What problem does this solve?
2. **Requirements** - What must be true?
3. **Acceptance** - How do we know it's done?

```markdown
## Problem Statement

Users cannot authenticate, exposing data to unauthorized access.

## Requirements

- [ ] Email/password login
- [ ] Session management

## Acceptance Criteria

- [ ] User can login with valid credentials
- [ ] Invalid credentials show error
```

---

## Part 7: Ingest & Parse

### Ingest a codebase

```bash
unispec ingest run ./src -a Ingested -t project-name
```

This:
- Parses all code with tree-sitter
- Extracts functions, structs, enums, imports
- Saves to `spec/code_analysis.toml`
- Optionally auto-indexes files

### Query ingested code

```
code_analysis(topic, area, item_type="functions", pattern="handle")
code_parse(path, item_type="functions", pattern="login")
```

---

## Part 8: Best Practices

### DO

- Read the spec before writing code
- Update the spec before changing behavior
- Link every file you create: `index_add(topic, path)`
- Use `index_exports` before implementing shared functionality
- Mark tasks `[-]` when starting, `[x]` when done
- Reference other topics in code: `# ref:index:topic:name`
- Push work forward: Roadmap → Working → Build

### DON'T

- Write code without reading the spec
- Skip `index_add` - unlinked code is invisible
- Duplicate functionality - check exports first
- Leave tasks unmarked
- Push to Build with incomplete work

### The Golden Rule

> **Query before implementing. Link when created. Complete tasks as you work.**

---

## Part 9: CLI Commands

### Topic Commands

```bash
unispec topic add <name> -a <area>
unispec topic list -a <area>
unispec topic show <name> --from <area>
unispec topic push <name> <target> --from <source>
```

### Index Commands

```bash
unispec index add --topic <name> --path <file>
unispec index list --topic <name>
unispec index exports --topic <name>
unispec index find <query>
```

### Ingest Commands

```bash
unispec ingest run <path> -a <area> -t <topic>
unispec parse file <path> --item-type functions
```

### TUI Keys

| Key | Action |
|-----|--------|
| `n` | New topic |
| `r` | Remove topic |
| `p` | Push to another area |
| `f` | Find links |
| `←` `→` | Navigate |
| `q` | Quit |

---

## Remember

**The spec is the contract.**

Your job:
1. Understand the spec
2. Implement the spec
3. Keep the spec updated
4. Track progress via tasks
5. Link all code to specs
