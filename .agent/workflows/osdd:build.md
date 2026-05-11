---
description: Implement a Staging spec — push to Working, write code in src/, link files, flip task checkboxes, push to Testing
---

# /osdd:build

Build a topic from Staging using UniSpec's MCP tools. The `osdd:` prefix is historical; the binary is `unispec` and the file operations are MCP calls.

## Usage
```
/osdd:build <TopicName>
```

## Rules

1. **Read the spec first.** Never guess requirements; always call `unispec_read_spec` before writing code.
2. **One task at a time, in order.** Don't skip ahead.
3. **Flip the checkbox immediately.** Call `tasks_complete` after each finished task — do not batch.
4. **Document decisions.** Use `notes_add` whenever you choose something that isn't pinned by the spec.
5. **Stay faithful to the spec.** If the spec says X, implement X. If you must diverge, surface it to the user before merging.

## Preconditions

- `<TopicName>` exists in Staging with `<TopicName>_spec.md` and `<TopicName>_task.md`.
- `<TopicName>` appears in `spec/Staging/queue.md` (`queue_check` returns `ready: true`).
- `src/` exists at the project root.

If any precondition fails, run `/osdd:spec <TopicName>` and `queue_add` first.

## Steps

### 1. Verify readiness and push to Working
```
queue_check  { topic: "<TopicName>", area: "Staging" }
topics_push  { topic: "<TopicName>", area: "Working" }
```

### 2. Load the spec
```
unispec_read_spec { topic: "<TopicName>", area: "Working" }
tasks_list        { topic: "<TopicName>", area: "Working" }
task_status       { topic: "<TopicName>", area: "Working", status: "working" }
```

### 3. For each open task at index `N`
1. Announce: "Working on task N: <text from tasks_list>".
2. Write code into `src/`.
3. Link it:
   ```
   index_add {
     topic: "<TopicName>",
     path: "src/<file>",
     link_type: "implementation",
     tags: "<…>",
     annotation: "<one-line role of this file>"
   }
   ```
4. Flip the checkbox:
   ```
   tasks_complete { topic: "<TopicName>", task_index: N }
   ```
5. If you made a decision the spec didn't pin:
   ```
   notes_add { topic: "<TopicName>", note: "Date YYYY-MM-DD: <decision>\n  Reason: <why>\n  Alternative: <what was rejected>" }
   ```

After each task, report progress:
```
Progress: X/Y tasks (Z%)
```

### 4. Append test tasks
Read the current task body, then overwrite with the original content + a new `### Phase 5: Testing` block of `- [ ]` items:
```
read_asset { topic: "<TopicName>", asset_type: "task", area: "Working" }
task_write {
  topic: "<TopicName>",
  area: "Working",
  content: "<original body + Phase 5: Testing block>"
}
```

### 5. Promote to Testing
```
task_status { topic: "<TopicName>", area: "Working", status: "complete" }
topics_push { topic: "<TopicName>", area: "Testing" }
```

## Files produced/touched

```
src/                        # code written here
spec/Working/<TopicName>/
├── topic.md
├── <TopicName>_spec.md     # unchanged unless you also call spec_write
└── <TopicName>_task.md     # checkboxes flipped, status updated, notes appended
```

## Definition of done

- Every implementation task is `- [x]` in `<TopicName>_task.md`.
- `task_status` is `complete`.
- Every file written to `src/` has a corresponding `index_add` link.
- Decisions not in the spec are captured in the task file's notes section.
- A `### Phase 5: Testing` block has been appended via `task_write`.
- `topics_push { area: "Testing" }` succeeded.

## Output on completion

```
## Build Complete: <TopicName>

Tasks: N/N (100%)
Topic now in Testing.
Next: /osdd:verify <TopicName>
```

## Guardrails

- Don't skip tasks. If a task is impossible to do, stop and ask the user.
- Don't leave `TODO:` markers in code; they don't show up in `tasks_list`.
- If the spec is genuinely unclear, ask the user — don't fabricate an answer.
- If your implementation diverges from the spec, capture it via `notes_add` and tell the user.
