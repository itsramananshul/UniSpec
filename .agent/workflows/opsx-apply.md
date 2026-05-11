---
description: Implement the tasks from a Staging topic — push to Working, write code, flip checkboxes
---

# /opsx:apply

> **Scope note.** This is an OpenSpec-style "apply the change" prompt mapped onto UniSpec's MCP tools. There is **no** `openspec` CLI in this repo — wherever the OpenSpec prompts mention `openspec status`, `openspec instructions`, etc., translate to the UniSpec MCP tools listed below.

Implement the tasks of a topic created by `/opsx:propose` (or `/spec`). This is the UniSpec BUILD loop with one extra step at the start to select the topic.

## Input

The argument after `/opsx:apply` is the topic name. If omitted:
- Infer it from conversation context if the user mentioned one.
- Otherwise, run `topics_list { area: "Staging" }` and ask the user to pick.

Never auto-select among multiple candidates. Always confirm: "Using topic: <name>".

## Tools

MCP:
- `topics_list { area }`
- `queue_check { topic, area }`
- `queue_add { topic, area }`
- `topics_push { topic, area, source_area? }`
- `unispec_read_spec { topic, area }`
- `tasks_list { topic, area }`
- `task_status { topic, area, status }`
- `tasks_complete { topic, task_index, note? }`
- `index_add { topic, path, link_type, tags?, annotation? }`
- `notes_add { topic, note }`
- `read_asset { topic, asset_type, area }`
- `task_write { topic, area, content }`

## Steps

### 1. Select the topic
- If a name was provided, use it.
- Otherwise list candidates from Staging:
  ```
  topics_list { area: "Staging" }
  ```
  Then ask the user which to apply. Don't auto-select.

Announce: "Using topic: <name>".

### 2. Verify readiness and push to Working
```
queue_check { topic: "<name>", area: "Staging" }
```
If `ready: false`:
```
queue_add   { topic: "<name>", area: "Staging" }
queue_check { topic: "<name>", area: "Staging" }   # must now be ready: true
```
Then push:
```
topics_push { topic: "<name>", area: "Working" }
```

### 3. Load the spec
```
unispec_read_spec { topic: "<name>", area: "Working" }
tasks_list        { topic: "<name>", area: "Working" }
task_status       { topic: "<name>", area: "Working", status: "working" }
```

Show progress so the user can track:
```
## Implementing <name>
Progress: 0/<M> tasks complete
Remaining tasks:
- task 0: <text>
- task 1: <text>
...
```

### 4. Implement each task in order

For each open task at index `N`:

1. Announce: `"Working on task N: <text>"`.
2. Make minimal, focused changes in `src/`.
3. `index_add { topic: "<name>", path: "src/<file>", link_type: "implementation", … }`.
4. `tasks_complete { topic: "<name>", task_index: N }`.
5. If you made a decision the spec didn't pin, `notes_add { topic: "<name>", note: "<decision + reason>" }`.

After each task, show updated progress.

**Pause if:**
- A task is unclear → ask the user.
- The spec is wrong/incomplete → propose updating it via `spec_write` (or `notes_add`).
- A real error blocks the work → report and wait.
- The user interrupts.

### 5. Append test tasks
```
read_asset { topic: "<name>", asset_type: "task", area: "Working" }
task_write {
  topic: "<name>",
  area: "Working",
  content: "<original body + ### Phase 5: Testing block of `- [ ]` items>"
}
```

### 6. Promote
```
task_status { topic: "<name>", area: "Working", status: "complete" }
topics_push { topic: "<name>", area: "Testing" }
```

## Output during implementation

```
## Implementing: <name>
Working on task 3/7: <task text>
[…changes…]
✓ Task complete (task_index 2)
```

## Output on completion

```
## Implementation Complete

Topic: <name>
Tasks: 7/7 (100%)

Next: /opsx:verify <name>   (or /test, /verify)
```

## Output on pause

```
## Implementation Paused

Topic: <name>
Progress: 4/7 tasks complete

### Issue
<short description of what blocked progress>

### Options
1. <option 1>
2. <option 2>
3. Other approach

What would you like to do?
```

## Definition of done

- Every original implementation task in `<name>_task.md` is `- [x]`.
- `task_status` is `complete`.
- Every file written in `src/` has an `index_add` link.
- A `### Phase 5: Testing` block has been appended via `task_write`.
- The topic is in `Testing`.

## Guardrails

- Keep changes minimal per task — don't rewrite unrelated code.
- Always read the spec before starting.
- Flip checkboxes immediately, not at the end.
- Pause on real ambiguity; don't guess at requirements.
- Don't write production code while paused; use `notes_add` to capture investigation results instead.
