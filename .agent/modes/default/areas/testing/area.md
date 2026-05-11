---
area: Testing
short: Run build/test pipelines; route green to Build, red to Fixing.
---

# Testing

## Purpose

Testing runs the configured build, test, and lint pipelines for a topic. The mode strips `queue.md` on entry — readiness gating doesn't apply here.

A topic in Testing is considered done when:
- Every configured pipeline step (build, test, optionally lint) has run.
- Each step's pass/fail and a short output snippet are captured via `notes_add`.
- The topic has been pushed to `Build` (all green) or `Fixing` (any red).
- `task_status` reflects the destination area: `complete` for Build, `working` for Fixing.

## Guidelines

- Run pipelines via configured connectors (`unispec_<connector>` MCP tools) or `unispec auto test`. Don't write code in this area.
- Record every test run with `notes_add` so the audit trail survives across pushes.
- If any step fails, push to `Fixing` and hand off to `/verify --fix`. Do not try to repair code while still in Testing.
- If you must touch a config file (e.g., correcting a test runner setup), make that the only change and document it in `notes_add`.
