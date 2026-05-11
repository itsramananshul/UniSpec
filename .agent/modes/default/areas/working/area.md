---
area: Working
short: Compile specs into code in src/.
---

# Working

## Purpose

Working is the primary development area. Code for each topic is written into `src/` at the project root (never inside `spec/`), and every file is linked back to its topic via `index_add`. Task checkboxes are flipped with `tasks_complete` as work progresses.

A topic in Working is considered done when:
- Every implementation task in `<topic>_task.md` is `- [x]`.
- `task_status` is `complete`.
- Every code file written or substantively changed has an `index_add` link with `link_type: "implementation"`.
- A `### Phase 5: Testing` block has been appended (via `task_write`) listing the test cases the next stage will run.

## Guidelines

- Follow the spec literally. If you must diverge, capture the decision via `notes_add` and surface it to the user.
- Update task state via MCP tools, not by editing the task file directly:
  - `task_status { … status: "working" }` when you begin work.
  - `tasks_complete { … task_index: N }` after each finished task.
  - `task_status { … status: "complete" }` when every implementation task is done.
- Link every new file: `index_add { topic, path, link_type: "implementation", tags, annotation }`.
- If you hit a real blocker, stop and report — do not paper over the issue with `notes_add` and continue.
