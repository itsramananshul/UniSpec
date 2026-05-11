# Workflow: /test (default mode)

Run the project's build/test pipeline for a topic in `Testing`. Promote on green; route to `Fixing` on red. This workflow does not modify code.

Mirrors `.agent/workflows/test.md`; per-mode copy.

---

## Tools

MCP:
- `topics_list { area: "Testing" }`
- `unispec_read_spec { topic, area: "Testing" }`
- `notes_add { topic, note }`
- `topics_push { topic, area }`
- `task_status { topic, area, status }`
- `unispec_<connector>` — one MCP tool per connector defined in `.agent/config.toml` (e.g. `unispec_test`, `unispec_build`).

CLI (shell out):
- `unispec auto test [-t <topic>] [--pre-script …] [--post-script …]`
- `unispec connector run <name>` (equivalent to the dynamic MCP tool)

There is no `unispec_auto_test` MCP tool; use the configured connector or shell out.

---

## Steps

1. **Identify the topic.**
   ```
   topics_list       { area: "Testing" }
   unispec_read_spec { topic: "<topic>", area: "Testing" }
   ```

2. **Run the pipeline.** Prefer configured connectors via MCP:
   ```
   unispec_build { args: [] }
   unispec_test  { args: [] }
   ```
   Or shell out:
   ```bash
   unispec auto test -t <topic>
   ```
   Capture exit code and combined output.

3. **Report.**
   ```
   notes_add {
     topic: "<topic>",
     note: "Test run YYYY-MM-DD: build=<pass|fail> test=<pass|fail>\n<first 20 lines of failing output, if any>"
   }
   ```

4. **Route.**
   - **All green** — promote to Build. Testing → Build is **not** queue-gated by the default mode, so no `queue_add` is needed.
     ```
     task_status { topic: "<topic>", area: "Testing", status: "complete" }
     topics_push { topic: "<topic>", area: "Build", source_area: "Testing" }
     ```
     CLI: `unispec topic push <topic> --area Build --from Testing`.
   - **Any red** — route to Fixing. Testing → Fixing is also not queue-gated.
     ```
     topics_push { topic: "<topic>", area: "Fixing", source_area: "Testing" }
     task_status { topic: "<topic>", area: "Fixing", status: "working" }
     ```
     CLI: `unispec topic push <topic> --area Fixing --from Testing`.

     Do not attempt to fix code in `/test`. Hand off to `/verify --fix`. (Note: when `/verify --fix` later pushes back to Testing, that push **is** queue-gated — Fixing is in `[readiness].areas` — so it will run `queue_add { topic, area: "Fixing" }` before the push.)

---

## Definition of done

- Every configured step (build, test, optionally lint/typecheck) ran for the topic.
- Each step's result was recorded via `notes_add`.
- The topic was pushed to `Build` (all green) or `Fixing` (any red).
- `task_status` matches the new area: `complete` for Build, `working` for Fixing.

---

## Failure modes

- **Connector not found** — verify with `unispec connector list`. If missing, fall back to shelling out — but ask the user first.
- **Connector timed out** — the exit code reflects the timeout, not a true failure. Mention this in the note and pause for user input before routing.
- **No connectors at all** — read the spec for a manual test plan, execute it, and report via `notes_add`.
