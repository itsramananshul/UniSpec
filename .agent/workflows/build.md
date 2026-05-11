# Workflow: /build

Implement a topic from Staging — write code into `src/`, link every file, flip task checkboxes, and push to Testing.

This workflow uses the UniSpec MCP tools. Tool names are literal.

---

## Preconditions

- The topic has both files: `<topic>_spec.md` and `<topic>_task.md` in `spec/Staging/<topic>/` (verify with `topics_show`).
- The topic is in `spec/Staging/queue.md` (verify with `queue_check`).
- `src/` exists at project root (create it once if not).

If any precondition fails, run `/spec` for the topic first, then `queue_add`.

---

## Tools

| Tool | Purpose |
|------|---------|
| `topics_list`, `queue_list`, `queue_check` | Orient. |
| `queue_add` | Register the topic if it's missing from `Staging/queue.md`. |
| `topics_push` | Move the topic between areas. |
| `unispec_read_spec` | Load spec + task content in one call. |
| `read_asset` | Re-read the task file when you need its current state. |
| `tasks_list` | Get task indices and checkbox state. |
| `task_status` | Update frontmatter `status:` to `working` / `complete`. |
| `tasks_complete` / `tasks_incomplete` | Flip `- [ ]` / `- [x]` at a 0-based index. |
| `index_add` | Link each new file to the topic. |
| `notes_add` | Capture decisions that aren't in the spec. |
| `task_write` | Used once to append test tasks before pushing to Testing. |

---

## Steps

### 1. Orient
```
topics_list   { area: "Staging" }
queue_list    { area: "Staging" }
queue_check   { topic: "<topic>", area: "Staging" }
```
If `ready: false`, add to queue:
```
queue_add     { topic: "<topic>", area: "Staging" }
queue_check   { topic: "<topic>", area: "Staging" }   # must now return ready: true
```

### 2. Push to Working
```
topics_push   { topic: "<topic>", area: "Working" }
```

### 3. Load the spec
```
unispec_read_spec { topic: "<topic>", area: "Working" }
tasks_list        { topic: "<topic>", area: "Working" }
task_status       { topic: "<topic>", area: "Working", status: "working" }
```

### 4. Implement, one task at a time

For each open task at index `N`:

1. Write code into `src/<file>` (host editor's Write/Edit tool — code files are not managed by MCP).
2. Link it:
   ```
   index_add {
     topic: "<topic>",
     path: "src/<file>",
     link_type: "implementation",
     tags: "<comma-separated>",
     annotation: "<one line: what this contributes>"
   }
   ```
3. Flip the checkbox:
   ```
   tasks_complete { topic: "<topic>", task_index: N }
   ```
4. If the choice you made isn't in the spec:
   ```
   notes_add { topic: "<topic>", note: "<decision + reason>" }
   ```

Do all four for the task before moving to the next index.

### 5. Append test tasks (BUILD phase only)

Test tasks are added now, after implementation, not during SPEC.

```
read_asset { topic: "<topic>", asset_type: "task", area: "Working" }
```
Then call `task_write` with the original content plus a `### Phase 5: Testing` section:
```
task_write {
  topic: "<topic>",
  area: "Working",
  content: "<existing task body + new Phase 5 Testing block with `- [ ]` items>"
}
```
`task_write` overwrites the whole file — include the existing implementation phases verbatim.

### 6. Mark complete and push to Testing
```
task_status { topic: "<topic>", area: "Working", status: "complete" }
topics_push { topic: "<topic>", area: "Testing" }
```

Pushing into Testing deletes `queue.md` in Testing per the mode config — that is expected.

---

## File layout

```
<project-root>/
├── src/                      # all code
└── spec/
    ├── Staging/
    │   ├── queue.md
    │   └── <topic>/{topic.md, <topic>_spec.md, <topic>_task.md}
    └── Working/
        └── <topic>/{topic.md, <topic>_spec.md, <topic>_task.md}
```

Nested topic `auth/login` → directory `spec/<Area>/auth/login/` with `auth-login_spec.md` and `auth-login_task.md`.

---

## Definition of done

A topic is done with BUILD when **all** of these hold:

- Every implementation task in `<topic>_task.md` is `- [x]`.
- `task_status` is `complete`.
- Every code file you wrote or substantively changed has a matching `index_add` link with `link_type: "implementation"`.
- Decisions not in the spec are captured via `notes_add`.
- A test-tasks section was appended via `task_write`.
- `topics_push { area: "Testing" }` succeeded.

If any item is missing, the topic is not ready for Testing.

---

## Failure modes

- **`topics_push` rejects the push from Staging** — topic not in `Staging/queue.md`. Run `queue_add`, then `queue_check`.
- **`task_write` errors with "spec doesn't exist"** — the topic has no `<topic>_spec.md`. Run `/spec` first.
- **`tasks_complete` errors with "task at index N not found"** — `tasks_list` indices change after any edit; re-list before each `tasks_complete`.
- **You wrote a file outside `src/`** — fix by moving the file into `src/` or, if intentional, document via `notes_add`.
