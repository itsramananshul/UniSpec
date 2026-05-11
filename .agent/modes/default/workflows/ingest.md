# Workflow: /ingest-all (default mode)

Reverse-engineer an existing codebase into a spec tree under the `Ingested` area. See `.agent/workflows/ingest.md` for the full workflow — this file mirrors it for the default mode.

---

## Tools

CLI:
- `unispec ingest run <path> [-a <area>] [-t <topic>] [-l <langs>]`
- `unispec parse file <path> [--item-type functions|structs|enums|imports|all] [--json]`
- `unispec index add --topic <name> --path <path>`
- `unispec index callers <symbol>` (CLI only — not an MCP tool)

MCP:
- `topics_add { topic, area, short, content }`
- `spec_add { topic, area, short, spec_content, task_content }`
- `index_add { topic, path, link_type, tags?, annotation? }`
- `notes_add { topic, note }`

---

## Steps

1. **Bulk ingest:**
   ```bash
   unispec ingest run <path> -a Ingested -t <topic-prefix>
   ```
   Writes parsed code analysis to `spec/code_analysis.toml` (or markdown, per `[ingest].output_format`).

2. **Create a topic per module** with real Overview / Purpose / Sub-topics in `content`. Set `short` to a real one-liner.

3. **Write the spec from the parsed surface.** Every public function/type appears as a `REQ-*` row referencing `<file>:<line>`. The task file is all `- [x]` because the code already exists.

4. **Link every source file** with `index_add` (`link_type: "implementation"`, tag with the language and `ingested`).

5. **Capture cross-module dependencies** via `unispec index callers <symbol>` and record them with `notes_add` on the dependent topic.

6. **Recurse** through sub-directories. Skip directories with no supported source or that are already represented in `topics_list { area: "Ingested" }`.

---

## Definition of done

- Every supported source directory has a topic with topic/spec/task files.
- Every source file appears in `index_list` linked to its topic.
- Cross-module dependencies are captured via `notes_add` on dependents.
- No source code was modified.

---

## Agent rules

- Do not modify source code.
- Every `REQ-*` must cite real existing behavior at `<file>:<line>`. No invented requirements.
- For unclear functions, write "TODO: clarify purpose" and open the question via `notes_add` — don't guess.
