# SPEC Workflow

Use this workflow to create or refine a topic, its spec, and its task list — all in the `Staging` area. No code is written here.

---

## Preconditions

- You know the rough scope of the feature.
- You can produce a one-line description (`short`) and a body for the topic, the spec, and the task list.

If you don't have those yet, stop and ask the user. Do not invent requirements.

---

## Tools used

All literal MCP tool names:

- `read_asset { topic: "templates", asset_type: "topic" | "spec" | "task" }` — read the template body to mirror.
- `topics_add { topic, area, short, content }` — creates `spec/<area>/<topic>/topic.md`.
- `spec_add { topic, area, short, spec_content, task_content }` — creates `<topic>_spec.md` and `<topic>_task.md` in the same directory.
- `queue_add { topic, area }` — register the topic in `spec/Staging/queue.md` so it can later be pushed.

The server prepends frontmatter to every file. Pass body text only — no leading `---` block.

---

## Steps

### 1. Read the templates

```
read_asset { topic: "templates", asset_type: "topic" }
read_asset { topic: "templates", asset_type: "spec" }
read_asset { topic: "templates", asset_type: "task" }
```

These return the canonical structure each file should have. Mirror the section headings exactly; fill the body with real, project-specific content.

### 2. Create the topic

```
topics_add {
  topic: "<topic-name>",
  area: "Staging",
  short: "<one-line description>",
  content: "<body matching templates/topic.md — Overview, Specs, Sub-topics, Notes>"
}
```

Constraints enforced by the server:
- `content` must be ≥ 10 characters of real text.
- `short` is required and non-empty.
- Do **not** include `---` frontmatter in `content`. The server writes `title`, `short`, `created`, `author`.

Nested topics use `/`, e.g. `auth/login`. The directory becomes `spec/Staging/auth/login/` and the parent (`auth`) must already exist as a topic.

### 3. Create the spec and task files

```
spec_add {
  topic: "<topic-name>",
  area: "Staging",
  short: "<one-line description>",
  spec_content: "<body matching templates/spec.md — Overview, Purpose, In-Depth Details, Requirements, Examples, Data Model, Out of Scope>",
  task_content: "<body matching templates/task.md — Implementation phases only, no test tasks>"
}
```

Both `spec_content` and `task_content` are required and must each be ≥ 10 characters. The server strips any `---` block you include and prepends correct frontmatter.

Filenames produced (slashes and spaces in `topic` are converted to `-`):
- `<topic>_spec.md`
- `<topic>_task.md`

### 4. Register in the readiness queue

```
queue_add { topic: "<topic-name>", area: "Staging" }
```

This is required for the BUILD workflow to later `topics_push` the topic to Working.

---

## Content rules

- **Specs describe WHAT, not HOW.** Requirements use SHALL/SHOULD; acceptance criteria are checkable.
- **Tasks describe implementation steps only.** No "write tests", "QA the feature", or similar. Test tasks are added during BUILD, after implementation.
- **One topic = one bounded scope.** If you find yourself listing five unrelated capabilities, split into sub-topics (`feature/sub-a`, `feature/sub-b`).
- **No placeholder text in the final body.** Strings like `[Requirement statement]` or `[Foundation task]` exist in the template to be replaced, not committed.

---

## Definition of done

A topic is done with SPEC when **all** of these hold:

- `spec/Staging/<topic>/topic.md` exists and has a real `short` and Overview.
- `spec/Staging/<topic>/<topic>_spec.md` exists with every template section filled with non-placeholder content.
- `spec/Staging/<topic>/<topic>_task.md` exists with at least one concrete implementation task and no test tasks.
- The topic appears in `spec/Staging/queue.md` (verify with `queue_check`).
- `topics_show { topic, area: "Staging" }` lists `topic.md`, `<topic>_spec.md`, and `<topic>_task.md`.

If any condition fails, the topic is not ready for BUILD.

---

## Example: full sequence

```
read_asset  { topic: "templates", asset_type: "spec" }
read_asset  { topic: "templates", asset_type: "task" }

topics_add {
  topic: "user-login",
  area: "Staging",
  short: "Email/password login with JWT session",
  content: "# user-login\n\n## Overview\nLogin flow for the customer portal — email + password, JWT issued on success.\n\n## Specs\n- `user-login_spec.md`: login flow design and data model.\n\n## Sub-topics\n- (none yet)\n\n## Notes\n- Password reset is out of scope; tracked under `account-recovery`."
}

spec_add {
  topic: "user-login",
  area: "Staging",
  short: "Email/password login with JWT session",
  spec_content: "# Design: user-login\n\n## Overview\nUsers submit email + password; server returns a signed JWT.\n\n## Purpose\nAuthenticate returning customers without third-party identity providers.\n\n## In-Depth Details\n- Passwords stored with argon2id.\n- JWT signed with HS256, 30-min expiry, refresh via /token endpoint.\n\n## Requirements\n| ID | Requirement | Priority |\n|----|-------------|----------|\n| REQ-001 | The service SHALL accept POST /login with {email, password}. | Must |\n| REQ-002 | The service SHALL return 401 on bad credentials within 200ms. | Must |\n| REQ-003 | The service SHOULD lock an account after 5 failed attempts in 5 min. | Should |\n\n## Examples\n### Example 1: Successful login\n- Input: {email: \"a@b.com\", password: \"...\"}\n- Output: 200 with {jwt: \"...\"}\n\n## Data Model\n### Entities\n| Entity | Fields | Description |\n|--------|--------|-------------|\n| User | id, email, password_hash, locked_until | One row per customer. |\n\n## Out of Scope\n- Password reset (see account-recovery).\n- SSO.",
  task_content: "# Tasks: user-login\n\n## Implementation\n\n### Phase 1: Foundation\n- [ ] **1.1** Add User table + migration.\n\n### Phase 2: Core Features\n- [ ] **2.1** Implement POST /login handler.\n- [ ] **2.2** Implement JWT signing helper.\n\n### Phase 3: Integration\n- [ ] **3.1** Wire handler into router with rate-limit middleware.\n\n### Phase 4: Polish\n- [ ] **4.1** Add structured logging on failure.\n\n## Notes\n"
}

queue_add { topic: "user-login", area: "Staging" }
```

After this, the BUILD workflow can pick up `user-login` from the queue and push it to Working.
