# Workflow: /test

Run the project's build and test pipeline for a topic in the `Testing` area. Promote on success; route to `Fixing` on failure.

`/test` does not modify code. It only runs scripts and reports results.

---

## Tools

MCP tools used:
- `topics_list { area: "Testing" }` — list topics ready to test.
- `unispec_read_spec { topic, area: "Testing" }` — context, especially for which connector to run.
- `notes_add { topic, note }` — record the test run result.
- `topics_push { topic, area }` — promote to `Build` on success or `Fixing` on failure.
- `task_status { topic, area, status }` — mark `complete` on promotion, `working` on rollback.
- Any `unispec_<connector>` MCP tool defined in `.agent/config.toml` (e.g. `unispec_test`, `unispec_build`, `unispec_lint`).

CLI tools used (shell out — not MCP):
- `unispec auto test [-t <topic>]` — runs the configured pre/test/post scripts.

---

## Steps

### 1. Identify the topic
```
topics_list       { area: "Testing" }
unispec_read_spec { topic: "<topic>", area: "Testing" }
```
Note which connectors are referenced in the spec (build, test, lint, etc.).

### 2. Run the pipeline

Preferred — invoke configured connectors via MCP:
```
unispec_build  { args: [] }
unispec_test   { args: [] }
unispec_lint   { args: [] }   # only if defined
```

Or shell out:
```bash
unispec auto test -t <topic>
```

Capture the full stdout/stderr and the exit code.

### 3. Report

```
notes_add {
  topic: "<topic>",
  note: "Test run YYYY-MM-DD:\n- build: <pass|fail>\n- test: <pass|fail>\n- lint: <pass|fail>\n<first 20 lines of failing output, if any>"
}
```

### 4. Route

**On all green:**
```
task_status { topic: "<topic>", area: "Testing", status: "complete" }
topics_push { topic: "<topic>", area: "Build" }
```

**On any failure:**
```
topics_push { topic: "<topic>", area: "Fixing" }
task_status { topic: "<topic>", area: "Fixing", status: "working" }
```
Do **not** attempt to fix the code yourself in this workflow. `/verify --fix` owns the repair loop. Tell the user which step failed and hand off.

---

## Definition of done

`/test` is done when:
- Every configured pipeline step (build, test, optionally lint/typecheck) has run for the topic.
- The result of each step is captured via `notes_add`.
- The topic has been pushed into either `Build` (all green) or `Fixing` (any red).
- `task_status` matches the new area (`complete` for Build, `working` for Fixing).

If you ran a partial pipeline because a step blocked the next, say so explicitly in the `notes_add` body and leave the topic in `Testing`.

---

## Failure modes

- **`unispec_<connector>` not found** — the connector isn't defined; check `.agent/config.toml`. Fall back to shelling out to the underlying command if the user authorizes it.
- **Connector timeout** — exit code reflects the timeout, not a real failure. Mention this in the note and ask the user before routing to Fixing.
- **No connectors configured at all** — there's nothing to run automatically. Read the spec to find a manual test plan, run it via host shell, and report results via `notes_add`.
