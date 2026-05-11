# Workflow: /spec

Create or refine a topic, spec, and task list in `Staging`. No code is written here.

This workflow uses the UniSpec MCP tools. Tool names are literal; arguments are JSON objects.

---

## Tools

| Tool | Required args | Purpose |
|------|---------------|---------|
| `read_asset` | `topic, asset_type` | Read templates (`topic: "templates"`) or existing topic/spec/task. `asset_type ∈ {"topic","spec","task"}`. |
| `topics_add` | `topic, area, short, content` | Create `topic.md`. `content` ≥ 10 chars. Server writes frontmatter — don't include `---`. |
| `spec_add` | `topic, area, short, spec_content, task_content` | Create `<topic>_spec.md` and `<topic>_task.md`. Both content fields ≥ 10 chars. Server strips your frontmatter and writes its own. |
| `queue_add` | `topic, area` | Add the topic to `spec/<area>/queue.md` so BUILD can later push it. |
| `topics_show` | `topic, area` | Verify files exist after creation. |

---

## Steps

### 1. Read templates first
```
read_asset { topic: "templates", asset_type: "topic" }
read_asset { topic: "templates", asset_type: "spec" }
read_asset { topic: "templates", asset_type: "task" }
```
Mirror the section headings exactly. Fill the body with project-specific content — never commit `[placeholder]` text.

### 2. Create the topic
```
topics_add {
  topic: "<topic-name>",
  area: "Staging",
  short: "<one-line description>",
  content: "<body matching templates/topic.md, with real content in each section>"
}
```

Constraints: `short` non-empty, `content` ≥ 10 chars, no leading `---` block (server writes its own frontmatter).

### 3. Create the spec and task files
```
spec_add {
  topic: "<topic-name>",
  area: "Staging",
  short: "<one-line description>",
  spec_content: "<body matching templates/spec.md, sections filled>",
  task_content: "<body matching templates/task.md, implementation tasks only>"
}
```

Filenames produced (slashes/spaces in `topic` become `-`):
- `<topic>_spec.md`
- `<topic>_task.md`

### 4. Register in the queue
```
queue_add { topic: "<topic-name>", area: "Staging" }
```

### 5. Verify
```
topics_show { topic: "<topic-name>", area: "Staging" }
```
Must list `topic.md`, `<topic>_spec.md`, `<topic>_task.md`.

---

## Content rules

- **Spec = WHAT, not HOW.** Use SHALL/SHOULD; acceptance criteria are checkable.
- **Tasks = implementation steps only.** No test tasks here — those are added during BUILD.
- **One topic = one bounded scope.** If you list five unrelated capabilities, split into sub-topics (`feature/sub-a`, `feature/sub-b`). The parent topic must already exist before adding a sub-topic.
- **No placeholder strings in the body.** `[Requirement statement]`, `[Foundation task]`, etc. exist in templates to be replaced.

---

## Definition of done

- `spec/Staging/<topic>/topic.md` exists with a real Overview.
- `spec/Staging/<topic>/<topic>_spec.md` exists with every template section filled with real content.
- `spec/Staging/<topic>/<topic>_task.md` exists with at least one concrete implementation task and zero test tasks.
- `queue_check { topic, area: "Staging" }` returns `ready: true`.

If any condition fails, the topic is not ready for `/build`.

---

## Failure modes

- **`topics_add` errors with "content too short"** — `content` is under 10 chars or empty. Write a real body.
- **`spec_add` errors with "spec_content required"** — you passed `content` instead. The parameter name is `spec_content` (not `content`) for `spec_add`.
- **`spec_add` errors on nested topic** — the parent topic doesn't exist. Run `topics_add` for the parent first (e.g. `topics_add { topic: "auth", ... }` before `spec_add { topic: "auth/login", ... }`).

---

## Asking the user

If you don't know any of these, ask before writing anything:

- One-line description (`short`)
- What problem does this solve, and for whom?
- What are the must-have requirements (REQ-001, REQ-002, …)?
- What is explicitly out of scope?
- What's a checkable acceptance criterion?

A vague answer means you ask again. Do not guess and commit.
