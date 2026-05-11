# Workflow: /verify

Confirm that the implementation in `src/` matches every requirement in the spec for a topic.

`/verify` is read-mostly. If `--fix` is requested, the failing topic is pulled into `Fixing`, the issue is addressed, and the topic returns to `Testing`.

---

## Tools

MCP tools used:
- `unispec_read_spec { topic, area }` — load spec + task.
- `read_asset { topic, asset_type: "spec", area }` — re-read just the spec when needed.
- `index_list { topic }` — list every file linked to the topic.
- `index_find { query, by }` — `by ∈ {"topic","path","tag"}`.
- `index_backlinks { topic }` — find which other topics depend on this one (helpful for regression scoping).
- `notes_add { topic, note }` — record verification findings.
- `topics_push { topic, area }` — move into `Fixing` (only with `--fix`).
- `task_status { topic, area, status }` — update `status:` to `pending` if you're sending it back to Fixing.

CLI tools used (shell out — not MCP):
- `unispec auto verify <topic>` — runs the configured verification script for the topic.

---

## Steps

### 1. Load context
```
unispec_read_spec { topic: "<topic>", area: "<Testing or Working>" }
index_list        { topic: "<topic>" }
```
The spec gives you the list of `REQ-*` items. `index_list` tells you which files claim to implement them.

### 2. Trace each requirement to code

For each `REQ-NNN` in the spec:
- Identify the linked file(s) most likely to implement it (from `index_list`, filter by `link_type: "implementation"`).
- Read those files with the host editor's Read tool.
- Record one of three states for each requirement:
  - `✓ implemented` — code matches the requirement.
  - `⚠ partial` — partial or different from spec.
  - `✗ missing` — not found.

Cite evidence with `<file>:<line>` so the user can verify.

### 3. Run the configured verifier (optional)

If a verifier is configured for this project:
```bash
unispec auto verify <topic>
```
Parse its output. Combine with your manual trace.

### 4. Check regressions on dependents
```
index_backlinks { topic: "<topic>" }
```
If other topics depend on this one, re-check their critical paths still work.

### 5. Produce the report

Append a verification block to the topic's notes:
```
notes_add {
  topic: "<topic>",
  note: "Verification YYYY-MM-DD: <N>/<M> requirements implemented.\n- REQ-001 ✓ src/auth/login.rs:42\n- REQ-002 ✗ not found\n- REQ-003 ⚠ partial — src/auth/login.rs:88 (locks after 10 failures, spec says 5)"
}
```

### 6. If `--fix` was requested and there are gaps
```
topics_push  { topic: "<topic>", area: "Fixing" }
task_status  { topic: "<topic>", area: "Fixing", status: "working" }
```
Address each gap in `src/`, update `tasks_complete` / `notes_add`, then push back to Testing:
```
topics_push  { topic: "<topic>", area: "Testing" }
task_status  { topic: "<topic>", area: "Testing", status: "complete" }
```

---

## Definition of done

`/verify` is done when:
- Every `REQ-*` in the spec has a recorded state (`✓` / `⚠` / `✗`) with file:line evidence.
- The verification block is recorded via `notes_add`.
- If `--fix` was requested, gaps are addressed and the topic is back in `Testing` with `task_status: complete`.
- If `--fix` was not requested and gaps exist, the user has been told exactly which `REQ-*` IDs are unimplemented, and the topic has **not** been advanced.

---

## Failure modes

- **`unispec_read_spec` returns empty content** — the topic isn't in this area, or the spec file was renamed manually. Run `topics_show { topic, show_all: true }` to locate it.
- **`index_list` returns no entries** — code was written but never linked. Either link the files via `index_add` (catch-up indexing) or report this as a structural failure for the BUILD step.
- **Verifier exits non-zero with no `REQ-` mapping** — the verifier failed before producing a report; don't push to Fixing automatically, ask the user.
