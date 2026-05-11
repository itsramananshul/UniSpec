<!--
Template for <topic>_task.md, read via:
  read_asset { topic: "templates", asset_type: "task" }

Replace every <…> marker. Do not commit literal `<TopicName>` or `<task text>`. The MCP server prepends frontmatter (spec, short, status, date) automatically — omit the `---` block when passing this body as `task_content` to `spec_add`.

Rule: implementation tasks only. Test tasks are appended later in BUILD via task_write, never here in SPEC.
-->

# Tasks: <TopicName>

## Implementation

### Phase 1: Foundation

- [ ] **1.1** <concrete foundation task — set up a module, add a migration, scaffold a route. Specific enough that a developer can start.>

### Phase 2: Core Features

- [ ] **2.1** <concrete core task>

### Phase 3: Integration

- [ ] **3.1** <concrete integration task: wire components together, expose to callers>

### Phase 4: Polish

- [ ] **4.1** <concrete polish task: logging, error messages, edge cases>

## Notes

<!--
Use this block during /build to record decisions that aren't in the spec.

Format:
  **YYYY-MM-DD**: <decision>
    Reason: <why>
    Alternative: <what was rejected>
-->
