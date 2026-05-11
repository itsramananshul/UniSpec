---
area: Fixing
short: Repair issues found in Testing, then return to Testing.
---

# Fixing

## Purpose

Fixing is the debug workspace for topics that failed Testing. The mode requires the topic to be re-added to `spec/Fixing/queue.md` before it can be pushed back to Testing.

A topic in Fixing is considered done when:
- The failure documented in the topic's notes block has been reproduced and addressed in `src/`.
- Any task added or modified during repair is reflected in `<topic>_task.md` (use `tasks_complete` / `task_write` as needed).
- A fix summary is captured via `notes_add` (root cause, fix, follow-up).
- The topic appears in `spec/Fixing/queue.md` (via `queue_add`), and `topics_push { area: "Testing" }` has succeeded.

## Guidelines

- Read failure logs end-to-end before touching code.
- Use `index_backlinks { topic }` and `index_list { topic }` to scope regressions: a fix here might affect dependents.
- For symbol-level callers, shell out to `unispec index callers <symbol>` — it's CLI-only.
- Once the fix is in place, re-link any new files with `index_add` and capture the root-cause + remediation via `notes_add`.
- After verifying the fix locally, `queue_add { topic, area: "Fixing" }`, then `topics_push { topic, area: "Testing" }`.
