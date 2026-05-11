---
description: Trace every requirement in the spec to code in the repo; record evidence
---

# /osdd:verify

Verify that the implementation matches the spec. Trace each `REQ-*` to code, with file:line evidence.

The `osdd:` prefix is historical. This workflow runs against the UniSpec MCP server; the binary is `unispec`.

## Usage
```
/osdd:verify <TopicName> [Working|Testing|Build]
```
Default area: `Testing`.

## Rules

1. **Be thorough.** Check every `REQ-*` in the spec.
2. **Show evidence.** Cite `<file>:<line>` for every status.
3. **Don't assume.** If the code isn't found, mark `✗ missing`.
4. **Distinguish partial.** "Similar but not exact" is `⚠ partial`, not `✓`.

## Status meanings

| Status | Meaning |
|--------|---------|
| `✓ implemented` | Code exists and matches the requirement. |
| `⚠ partial` | Partially implemented or behaviorally different. |
| `✗ missing` | Not found anywhere in the repo. |

## Tools

MCP:
- `unispec_read_spec { topic, area }`
- `read_asset { topic, asset_type: "spec", area }`
- `index_list { topic }`
- `index_find { query, by: "topic" | "path" | "tag" }`
- `index_backlinks { topic }`
- `notes_add { topic, note }`
- `topics_push { topic, area }`
- `task_status { topic, area, status }`

CLI:
- `unispec auto verify <topic>` — runs the configured verifier (if present).
- `unispec index callers <symbol>` — find references; CLI only, not MCP.

## Steps

### 1. Load the spec
```
unispec_read_spec { topic: "<TopicName>", area: "<Area>" }
index_list        { topic: "<TopicName>" }
```

### 2. Extract requirements
From the spec's Requirements table, list every `REQ-*` ID.

### 3. Trace each requirement
For each `REQ-NNN`:
- From `index_list`, identify candidate files (filter `link_type: "implementation"`).
- Read those files; locate the code that implements (or should implement) the requirement.
- Record state and evidence: `<file>:<line>`.

### 4. Run the verifier (if configured)
```bash
unispec auto verify <TopicName>
```
Combine its output with your manual trace.

### 5. Report
```
notes_add {
  topic: "<TopicName>",
  note: "Verification YYYY-MM-DD\n\n## Coverage\n| REQ | Status | Evidence |\n|-----|--------|----------|\n| REQ-001 | ✓ | src/auth/login.rs:42 |\n| REQ-002 | ✗ | not found |\n| REQ-003 | ⚠ | src/auth/login.rs:88 — locks after 10 failures, spec says 5 |\n\n## Summary\nN/M requirements verified."
}
```

### 6. Route (only if user added `--fix`)
- If any `✗` or `⚠`:
  ```
  topics_push { topic: "<TopicName>", area: "Fixing" }
  task_status { topic: "<TopicName>", area: "Fixing", status: "working" }
  ```
  Then close the gaps in `src/`, flip checkboxes, and:
  ```
  topics_push { topic: "<TopicName>", area: "Testing" }
  task_status { topic: "<TopicName>", area: "Testing", status: "complete" }
  ```

## Definition of done

- Every `REQ-*` in the spec has a recorded state with `<file>:<line>` evidence.
- The verification block is appended via `notes_add`.
- With `--fix`: all gaps closed, topic returned to `Testing`, `task_status: complete`.
- Without `--fix`: gaps are reported by `REQ-*` ID; the topic remains in its current area.

## Output formats

### All pass
```
✓ Verification Complete: <TopicName>
Coverage: 5/5 (100%)
All requirements verified.
Ready for /osdd:test or push to Build.
```

### With gaps
```
✓ Verification Complete: <TopicName>
Coverage: 4/5 (80%)

✗ REQ-003: Feature not implemented
  Evidence: not found
  Recommendation: implement user preference storage

⚠ REQ-004: Partial implementation
  Evidence: src/auth/login.rs:88
  Found: 10-failure lockout; spec says 5.

Options:
1. /osdd:build <TopicName>     # add work to close the gaps
2. Update the spec             # if 10 is correct, change REQ-004
3. /osdd:verify --fix          # repair now, return to Testing
```
