# Workflow: unispec:spec (default mode)

Create a topic, spec, and task list in `Staging` using the MCP tools. This is the spec-authoring command for the default mode.

---

## Preconditions

- You have a one-line description of the feature (`short`).
- You have enough information to write real content for the topic, spec, and task list — no `[placeholder]` strings.

If either is missing, ask the user before doing anything.

---

## Tools

| Tool | Required args | Notes |
|------|---------------|-------|
| `read_asset` | `topic, asset_type` | Read templates with `topic: "templates"`, `asset_type ∈ {"topic","spec","task"}`. |
| `topics_add` | `topic, area, short, content` | `content` ≥ 10 chars. Server prepends frontmatter — don't include `---`. |
| `spec_add` | `topic, area, short, spec_content, task_content` | Both content fields ≥ 10 chars. Creates `<topic>_spec.md` and `<topic>_task.md`. |
| `queue_add` | `topic, area` | Add to `spec/Staging/queue.md`. |
| `topics_show` | `topic, area` | Verify the files exist. |

---

## Steps

### 1. Read the templates
```
read_asset { topic: "templates", asset_type: "topic" }
read_asset { topic: "templates", asset_type: "spec" }
read_asset { topic: "templates", asset_type: "task" }
```

### 2. Create the topic
```
topics_add {
  topic: "<topic-name>",
  area: "Staging",
  short: "<one-line description>",
  content: "<body mirroring templates/topic.md, with real content>"
}
```

### 3. Create the spec and tasks
```
spec_add {
  topic: "<topic-name>",
  area: "Staging",
  short: "<one-line description>",
  spec_content: "<body mirroring templates/spec.md, every section filled>",
  task_content: "<body mirroring templates/task.md, implementation tasks only>"
}
```

Filenames produced: `<topic>_spec.md`, `<topic>_task.md` (slashes/spaces in `topic` become `-`).

### 4. Add to the readiness queue
```
queue_add { topic: "<topic-name>", area: "Staging" }
```

### 5. Verify
```
topics_show { topic: "<topic-name>", area: "Staging" }
```
Expect to see `topic.md`, `<topic>_spec.md`, `<topic>_task.md`.

---

## Content rules

- **Spec = WHAT, not HOW.** Use SHALL/SHOULD; acceptance criteria are checkable.
- **Tasks = implementation steps only.** No test tasks here.
- **No placeholder text in the body.** `[Requirement statement]`, `[Foundation task]`, etc. exist in the template to be replaced.
- **Nested topics use `/`** (e.g., `auth/login`). The parent topic must already exist.

---

## Definition of done

- `spec/Staging/<topic>/topic.md` exists with real content.
- `spec/Staging/<topic>/<topic>_spec.md` exists with every template section filled.
- `spec/Staging/<topic>/<topic>_task.md` exists with concrete implementation tasks and zero test tasks.
- `queue_check { topic, area: "Staging" }` returns `ready: true`.

---

## Failure modes

- **`topics_add` errors with "content required"** — `content` was empty or under 10 chars. Write a real body.
- **`spec_add` errors with "spec_content required"** — you passed `content` instead. Use `spec_content`.
- **`spec_add` errors with "parent topic does not exist"** — for a nested topic like `auth/login`, run `topics_add { topic: "auth", ... }` first.
- **Conflict on existing topic** — `topics_add` refuses to overwrite. If you genuinely want to redo it, `topics_delete { topic, force: true }` first.
