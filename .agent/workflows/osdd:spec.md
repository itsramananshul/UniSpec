---
description: Create a spec in Staging via the UniSpec MCP server
---

# /osdd:spec

Create a new specification in the **Staging** area using UniSpec's MCP tools.

The `osdd:` prefix is historical (the project's predecessor was OpenSDD). The actual binary is `unispec`, and the file-management tool calls are MCP tools — `osdd` is not a CLI you can shell out to.

## Usage
```
/osdd:spec <TopicName>
```

## Rules

1. **Staging only.** Never create specs directly in Working or Build.
2. **WHAT, not HOW.** Spec out requirements; implementation choices belong in the BUILD phase.
3. **Unique names.** Run `topics_list { area: "Staging" }` first to confirm `<TopicName>` is not already taken.
4. **Nested topics use `/`** (e.g., `auth/login`). The parent topic must already exist via `topics_add`.

## Steps

### 1. Verify the topic name is free
```
topics_list { area: "Staging" }
```
If `<TopicName>` already appears, either pick a new name or use `/spec` to update the existing one.

### 2. Create the topic
```
topics_add {
  topic: "<TopicName>",
  area: "Staging",
  short: "<one-line description>",
  content: "# <TopicName>\n\n## Overview\n<2-3 sentences>\n\n## Specs\n- <TopicName>_spec.md: <what it covers>\n\n## Sub-topics\n- (none yet)\n\n## Notes\n- Created <YYYY-MM-DD>."
}
```

### 3. Create the spec + task files
```
spec_add {
  topic: "<TopicName>",
  area: "Staging",
  short: "<one-line description>",
  spec_content: "# Design: <TopicName>\n\n## Overview\n<what does this feature do, and why is it needed>\n\n## Purpose\n<problem solved, who benefits>\n\n## In-Depth Details\n<technical explanation>\n\n## Requirements\n| ID | Requirement | Priority |\n|----|-------------|----------|\n| REQ-001 | The service SHALL <…>. | Must |\n\n## Examples\n### Example 1: <name>\n- Input: <…>\n- Output: <…>\n\n## Data Model\n<entities + fields>\n\n## Out of Scope\n- <what this spec does NOT cover>",
  task_content: "# Tasks: <TopicName>\n\n## Implementation\n\n### Phase 1: Foundation\n- [ ] **1.1** <concrete task>\n\n### Phase 2: Core Features\n- [ ] **2.1** <concrete task>\n\n## Notes\n"
}
```

### 4. Register in the readiness queue
```
queue_add { topic: "<TopicName>", area: "Staging" }
```

### 5. Verify
```
topics_show { topic: "<TopicName>", area: "Staging" }
queue_check { topic: "<TopicName>", area: "Staging" }
```

## Files produced

```
spec/Staging/<TopicName>/
├── topic.md
├── <TopicName>_spec.md
└── <TopicName>_task.md
```

For nested topics like `auth/login`, slashes are replaced with `-` in filenames: `auth-login_spec.md`, `auth-login_task.md`.

## Definition of done

- `topics_show` lists all three files (`topic.md`, `<TopicName>_spec.md`, `<TopicName>_task.md`).
- The spec contains at least one `REQ-*` row and at least one example.
- The task file has only implementation tasks, no test tasks.
- `queue_check` returns `ready: true`.

## Output

```
✓ Topic created at spec/Staging/<TopicName>/
✓ Queue updated (Staging/queue.md)
Next: /osdd:build <TopicName>
```
