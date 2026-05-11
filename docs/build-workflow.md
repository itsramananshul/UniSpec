# BUILD Workflow

Use this workflow to implement a topic — convert a spec in Staging into working code in `src/`, and hand it off to Testing.

This file assumes the MCP tools listed in `docs/mcp.md` are available. Tool names are literal; arguments are objects.

---

## Preconditions

- The topic already has both files: `<topic>_spec.md` and `<topic>_task.md` (created via `spec_add`).
- The topic is listed in `spec/Staging/queue.md` (or you'll add it below — `topics_push` from Staging requires this).
- `src/` exists at the project root. If not, create it once at the start.

If any precondition isn't met, fix it before running the steps below.

---

## Steps

### 1. Orient

```
topics_list   { area: "Staging" }
queue_list    { area: "Staging" }
```

If the topic isn't in the queue:

```
queue_add    { topic: "<topic>", area: "Staging" }
queue_check  { topic: "<topic>", area: "Staging" }
```

`queue_check` must return `ready: true` before you can push.

### 2. Push to Working

```
topics_push { topic: "<topic>", area: "Working" }
```

This copies the topic into `spec/Working/<topic>/` and leaves the original in Staging.

### 3. Load the spec and tasks

```
unispec_read_spec { topic: "<topic>", area: "Working" }
tasks_list        { topic: "<topic>", area: "Working" }
task_status       { topic: "<topic>", area: "Working", status: "working" }
```

Read both before writing code. The full task list comes back ordered, with each item's 0-based index, text, and checkbox state.

### 4. Implement, one task at a time

For each open task at `task_index = N`:

1. Write code into `src/` at the project root. **Code never goes inside `spec/`.**
2. After saving the file, link it:
   ```
   index_add {
     topic: "<topic>",
     path: "src/<file>",
     link_type: "implementation",
     tags: "<comma-separated>",
     annotation: "<one line: what this file contributes>"
   }
   ```
3. Mark the task complete:
   ```
   tasks_complete { topic: "<topic>", task_index: N }
   ```
4. If a decision wasn't in the spec, capture it:
   ```
   notes_add { topic: "<topic>", note: "<decision + reason>" }
   ```

Do these four steps for every task before moving on. Do not batch them across tasks — `tasks_complete` is cheap, and stale checkboxes lose information.

### 5. Add test tasks (BUILD phase only)

Implementation tasks belong in the spec. **Test tasks belong here, after implementation is finished.** Append them to the task file:

```
task_write {
  topic: "<topic>",
  area: "Working",
  content: "<full task file content including the original Implementation phases plus a new ### Phase 5: Testing section with each test as `- [ ]`>"
}
```

`task_write` overwrites the whole file, so include the existing implementation phases in your `content`. (Use `read_asset { topic: "<topic>", asset_type: "task", area: "Working" }` first to get the current content.)

### 6. Mark the topic complete and push to Testing

```
task_status  { topic: "<topic>", area: "Working", status: "complete" }
topics_push  { topic: "<topic>", area: "Testing" }
```

The mode config deletes `queue.md` when entering Testing — that's intentional.

---

## File placement

```
<project-root>/
├── src/                    # all source code lives here
└── spec/
    ├── Staging/
    │   ├── queue.md
    │   └── <topic>/
    │       ├── topic.md
    │       ├── <topic>_spec.md
    │       └── <topic>_task.md
    └── Working/
        └── <topic>/
            ├── topic.md
            ├── <topic>_spec.md
            └── <topic>_task.md
```

For nested topics (`auth/login`), the spec/task filenames become `auth-login_spec.md` and `auth-login_task.md` inside `spec/<Area>/auth/login/`.

---

## Definition of done

A topic is done with BUILD when **all** of these hold:

- Every implementation task in `<topic>_task.md` is `- [x]`.
- Every source file you wrote or substantively changed has a matching `index_add` link with `link_type: "implementation"`.
- Decisions not in the spec are captured via `notes_add`.
- Test tasks have been appended to the task file.
- `task_status` is `complete`.
- `topics_push { area: "Testing" }` succeeded.

If any item above is missing, the topic is not ready for Testing.

---

## Failure modes

- **`topics_push` rejects you in Staging** — topic isn't in `Staging/queue.md`. Run `queue_add`, then `queue_check`.
- **`task_write` rejects you** — the spec file doesn't exist yet. The topic isn't fully created; run `spec_add` first.
- **`tasks_complete` says "task at index N not found"** — the index is out of range. Run `tasks_list` again to see the current indices; they're recalculated each call.
- **You wrote a file outside `src/`** — that's a smell. Either move it into `src/`, or skip `index_add` and add a project-level note via `notes_add` explaining why.
