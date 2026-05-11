---
description: Propose a new change — create a topic with spec and tasks in one step
---

# /opsx:propose

> **Scope note.** This is an OpenSpec-style "propose a change" prompt mapped onto UniSpec's MCP tools. There is **no** `openspec` CLI in this repository — anywhere this prompt mentions `openspec status`, `openspec instructions`, or `.openspec.yaml`, use the corresponding UniSpec MCP tools listed below instead.

In one step, create a topic and a spec + task list in `Staging`. When ready to implement, run `/opsx:apply` (or `/build`).

## Input

The argument after `/opsx:propose` is either:
- A kebab-case topic name, e.g. `add-user-auth`.
- A description of the change ("add user authentication"). Derive a kebab-case name from it (`add-user-auth`).

If no argument is provided, ask the user:
> "What change do you want to work on? Describe what you want to build or fix."

Do not proceed without a clear answer. A vague answer earns one follow-up question.

## Tools

| Tool | Required args | Purpose |
|------|---------------|---------|
| `topics_list` | — | Confirm the chosen name isn't taken. |
| `read_asset` | `topic: "templates", asset_type: …` | Read templates for the topic/spec/task body. |
| `topics_add` | `topic, area, short, content` | Create `spec/Staging/<name>/topic.md`. |
| `spec_add` | `topic, area, short, spec_content, task_content` | Create `<name>_spec.md` + `<name>_task.md`. |
| `queue_add` | `topic, area` | Add to `spec/Staging/queue.md`. |

## Steps

### 1. Verify the name is free
```
topics_list { area: "Staging" }
```
If `<name>` already exists, ask whether to update it (`/spec`) or pick a different name.

### 2. Read the templates
```
read_asset { topic: "templates", asset_type: "topic" }
read_asset { topic: "templates", asset_type: "spec" }
read_asset { topic: "templates", asset_type: "task" }
```

### 3. Create the topic
```
topics_add {
  topic: "<name>",
  area: "Staging",
  short: "<one-line description>",
  content: "<topic body mirroring templates/topic.md, real content>"
}
```
`content` must be ≥ 10 chars. Don't include `---`; the server writes frontmatter.

### 4. Create the spec + tasks
```
spec_add {
  topic: "<name>",
  area: "Staging",
  short: "<one-line description>",
  spec_content: "<body mirroring templates/spec.md, every section filled>",
  task_content: "<body mirroring templates/task.md, implementation tasks only>"
}
```

Both `spec_content` and `task_content` must be ≥ 10 chars. Server strips your `---` and writes its own.

### 5. Register in the queue
```
queue_add { topic: "<name>", area: "Staging" }
```

### 6. Verify
```
topics_show { topic: "<name>", area: "Staging" }
queue_check { topic: "<name>", area: "Staging" }
```

## Content rules

- **Spec = WHAT, not HOW.** Use SHALL/SHOULD; acceptance criteria are checkable.
- **Tasks = implementation only.** No test tasks here.
- **No placeholders.** `[Requirement statement]` etc. exist in templates to be replaced.
- **Nested topics use `/`.** Parent must already exist.

## Definition of done

- `topic.md`, `<name>_spec.md`, `<name>_task.md` all exist in `spec/Staging/<name>/`.
- The spec contains at least one `REQ-*` row and at least one example.
- The task file has only implementation tasks.
- `queue_check` returns `ready: true`.

## Output

```
## Proposal Created: <name>

Files:
- spec/Staging/<name>/topic.md
- spec/Staging/<name>/<name>_spec.md
- spec/Staging/<name>/<name>_task.md

Queue: Staging/queue.md updated.
Next: /opsx:apply <name>  (or /build <name>)
```

## Guardrails

- Create ALL three files (topic, spec, task) in this single workflow — never partial.
- If `topics_add` or `spec_add` fails, stop and report the exact error; don't try to repair file state manually with Write tools.
- If a topic with this name already exists, ask the user whether to continue with `/spec` (update) or pick a new name.
- Don't fabricate requirements. If you don't know enough, ask the user.
