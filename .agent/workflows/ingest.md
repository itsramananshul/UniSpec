# Workflow: /ingest-all

Reverse-engineer an existing codebase into a spec tree. For each module, extract its public surface and create a topic + spec + task set that documents what the code already does.

This workflow uses the CLI (`unispec parse`, `unispec ingest run`, `unispec index add`) and the MCP tools (`topics_add`, `spec_add`, `index_add`).

---

## Tools

CLI tools used:
- `unispec ingest run <path> [-a <area>] [-t <topic>] [-l <langs>]` — bulk ingest a directory; writes to `spec/code_analysis.toml` by default.
- `unispec parse file <path> [--item-type functions|structs|enums|imports|all] [--json]` — parse a single file.
- `unispec index add --topic <name> --path <path>` — link a file to a topic (CLI form of `index_add`).
- `unispec index callers <symbol>` — find references to a symbol (CLI only; not an MCP tool).

MCP tools used:
- `topics_add { topic, area, short, content }` — create a topic for the module.
- `spec_add { topic, area, short, spec_content, task_content }` — record the spec and a task list that's already marked complete.
- `index_add { topic, path, link_type: "implementation" }` — link the source file.

Supported languages for parsing: rust, javascript, typescript, python, go, bash.

---

## Steps

### 1. Run the bulk ingest

```bash
unispec ingest run <path> -a Ingested -t <topic-prefix>
```

This produces `spec/code_analysis.toml` (or markdown files, per `[ingest]` config in `.agent/config.toml`) summarizing functions, structs, enums, and imports per file.

### 2. For each module, create a topic

Pick a meaningful boundary (a directory, a package, a Rust module). For each:

```
topics_add {
  topic: "<module>",
  area: "Ingested",
  short: "<one-line description of what the module does>",
  content: "# <module>\n\n## Overview\n<2-4 sentences on responsibility>\n\n## Specs\n- <module>_spec.md: existing behaviour\n\n## Sub-topics\n- (one per sub-module)\n\n## Notes\n- Ingested from <path> on YYYY-MM-DD."
}
```

### 3. Write the spec from the parsed surface

Use `unispec parse file <path> --json` to extract the file's functions/structs/enums, then:

```
spec_add {
  topic: "<module>",
  area: "Ingested",
  short: "<one-line description>",
  spec_content: "# Design: <module>\n\n## Overview\n<what this module does>\n\n## Purpose\n<why it exists>\n\n## In-Depth Details\n<list of public functions/types with signatures>\n\n## Requirements\n| ID | Requirement | Priority |\n|----|-------------|----------|\n| REQ-001 | Module SHALL expose <fn>(<sig>) — currently in <file>:<line>. | Must |\n\n## Data Model\n<structs/enums>\n\n## Out of Scope\n- (anything in adjacent modules)",
  task_content: "# Tasks: <module>\n\n## Implementation\n\n### Phase 1: Foundation\n- [x] **1.1** Public surface as captured (ingested).\n\n## Notes\n- Ingested from <path> on YYYY-MM-DD."
}
```

All implementation tasks are pre-checked (`- [x]`) because the code already exists. New tasks may be added later when refactoring.

### 4. Link the source files

For every file the module covers:
```
index_add {
  topic: "<module>",
  path: "<path>",
  link_type: "implementation",
  tags: "<language>,ingested",
  annotation: "Source file for module <module>."
}
```

For cross-module dependencies, use the CLI to find callers:
```bash
unispec index callers <symbol>
```
Each caller indicates a topic that depends on yours; capture this via `notes_add` on the dependent topic.

### 5. Recurse

Move to sub-directories. Stop when:
- A directory has no files of supported languages, or
- The directory is already represented by a topic (check `topics_list { area: "Ingested" }` first).

---

## Definition of done

For each ingested module:
- `topics_add` succeeded and `topic.md` shows a real Overview.
- `spec_add` succeeded and the spec documents every public function/type with file:line references.
- Every source file in the module is linked via `index_add`.
- The task file is all `- [x]` (since the code already exists), with a Notes block mentioning the ingest date and source path.
- Cross-module dependencies are noted on the dependent topic.

For the project overall:
- `topics_list { area: "Ingested" }` covers every directory of supported source.
- `unispec index list` returns ≥ one entry for every ingested file.

---

## Failure modes

- **`unispec parse file` returns nothing** — the language isn't supported, or the file has a syntax error. Skip the file and add it to a "skipped" note on the parent topic.
- **`spec_add` fails on a nested topic** — the parent topic must already exist. Create parents top-down.
- **`topic.md` already exists when re-ingesting** — `topics_add` errors out. Either delete the existing topic via `topics_delete` (only if you're sure) or update via `spec_write` / `task_write` instead.

---

## Agent rules

- Do not modify any source code during ingest. This workflow only documents existing code.
- If you can't determine a function's purpose from name + signature + neighbors, write the body description as "TODO: clarify purpose" and add a `notes_add` entry asking the user.
- Do not invent requirements that the code doesn't actually fulfill. Every `REQ-*` must correspond to a real existing behavior, cited by file:line.
