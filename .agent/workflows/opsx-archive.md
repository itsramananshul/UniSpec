---
description: Archive a completed topic — move it to the Build area
---

# /opsx:archive

> **Scope note.** This is an OpenSpec-style "archive a completed change" prompt mapped onto UniSpec. There is **no** `openspec` CLI — wherever OpenSpec workflows say "archive", in UniSpec this means moving the topic to the `Build` area, which is treated as immutable.

Archive a completed topic by promoting it to `Build`. UniSpec does not delete or rename topics on archive; instead, `Build` is the immutable final stage of the pipeline.

## Input

The argument after `/opsx:archive` is the topic name. If omitted, list candidates from `Testing` (since archive should only follow successful tests) and ask the user to pick:
```
topics_list { area: "Testing" }
```
Never auto-select among multiple candidates.

## Tools

MCP:
- `topics_list { area }`
- `topics_show { topic, area }`
- `unispec_read_spec { topic, area }`
- `tasks_list { topic, area }`
- `notes_add { topic, note }`
- `task_status { topic, area, status }`
- `topics_push { topic, area }`

## Steps

### 1. Confirm the topic
Ask the user (or accept the argument). Announce: "Archiving topic: <name>".

### 2. Check completeness

a. **Spec + tasks present:**
   ```
   topics_show { topic: "<name>", area: "Testing" }
   ```
   Must list `topic.md`, `<name>_spec.md`, `<name>_task.md`.

b. **All tasks complete:**
   ```
   tasks_list { topic: "<name>", area: "Testing" }
   ```
   Count `- [ ]` (incomplete) vs `- [x]` (complete). **If any are incomplete**, show the count and prompt:
   ```
   <N> incomplete tasks remain. Archive anyway?
   ```
   Proceed only on explicit user confirmation.

c. **Test run captured:** Inspect the notes block of `<name>_task.md` for a recent `Test run` or `Verification` entry. If missing, warn and prompt for confirmation.

### 3. Promote to Build
```
task_status { topic: "<name>", area: "Testing", status: "complete" }
topics_push { topic: "<name>", area: "Build" }
```
`Build` is configured as a protected area in `.agent/modes/default/mode.toml`. After this push, you should not modify the topic in `Build`. To make further changes, run `topics_pull { topic, source_area: "Build" }` to bring it back into `Working`.

### 4. Record the archive
```
notes_add {
  topic: "<name>",
  note: "Archived to Build on YYYY-MM-DD.\n  Test run: <link or summary>\n  Verification: <link or summary>"
}
```

### 5. Summarize

```
## Archived: <name>

Pipeline: Staging → Working → Testing → Build
Final files:
- spec/Build/<name>/topic.md
- spec/Build/<name>/<name>_spec.md
- spec/Build/<name>/<name>_task.md

Build is treated as immutable. To resume work, run topics_pull { topic: "<name>", source_area: "Build" }.
```

## Definition of done

- The topic exists under `spec/Build/<name>/` with all three files.
- `task_status` is `complete`.
- A `notes_add` entry records the archive date and references the verification result.
- If incomplete tasks or missing verification were waived, the user explicitly confirmed.

## Output on warnings

```
## Archived (with warnings): <name>

Warnings:
- <N> incomplete tasks at archive time (user confirmed).
- Verification block not found in notes (user confirmed).

Review spec/Build/<name>/<name>_task.md if this wasn't intentional.
```

## Guardrails

- Don't auto-archive. Always confirm with the user, especially with warnings.
- Don't bypass the `topics_push` flow with raw filesystem moves — that breaks the index.
- Once in `Build`, treat the topic as read-only. Pull it back into `Working` if changes are needed.
