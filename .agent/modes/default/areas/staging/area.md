---
area: Staging
short: Write and refine specs before any code is written.
---

# Staging

## Purpose

Staging is where new ideas become formal specs. Topics here have a `topic.md`, a `<topic>_spec.md`, and a `<topic>_task.md` (created via `topics_add` and `spec_add` MCP tools). No source code is written for a topic while it's in Staging.

A topic in Staging is considered ready for `Working` when:
- Its spec has at least one `REQ-*` row and one example.
- Its task file contains concrete implementation tasks (no test tasks).
- It appears in `spec/Staging/queue.md` (use `queue_add` or `queue_check`).

## Guidelines

- Specs describe **WHAT**, not HOW. Use SHALL/SHOULD; acceptance criteria must be testable.
- Each topic = one bounded scope. Split into nested sub-topics (`feature/sub-a`) when the scope grows.
- Use `topics_add`, `spec_add`, `spec_write`, `task_write` — never write to the spec files directly from the host editor.
- `topics_push { area: "Working" }` is gated on `queue.md`; add the topic with `queue_add` first.
