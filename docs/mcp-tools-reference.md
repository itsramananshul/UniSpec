# MCP Tools Reference

Every MCP tool the UniSpec server publishes via `tools/list`, with required args, optional args, an example JSON-RPC `tools/call` request, and a representative response.

This list is generated against `src/mcp/mod.rs::get_tools()` on the `everything` branch. There are **31 built-in tools** plus one dynamic `unispec_<name>` tool per `[[connector]]` entry in `.agent/config.toml`.

> Filename convention: `spec_add` writes `<topic-safe>_spec.md` and `<topic-safe>_task.md`, where `<topic-safe>` is the topic name with `/` and ` ` replaced by `-`. Every read tool (`unispec_read_spec`, `read_asset`, `topics_show`) uses the same names.

> Default area: every tool that takes an optional `area` defaults to `"Staging"` if omitted. (The server reads `Staging` directly, not the project config, so set the area explicitly when in doubt.)

## Areas

### `areas_list`

List every area present in `spec/`.

**Args:** none.

**Request:**
```json
{ "jsonrpc": "2.0", "id": 1, "method": "tools/call",
  "params": { "name": "areas_list", "arguments": {} } }
```

**Response:**
```json
{ "success": true, "areas": ["Build","Fixing","Staging","Testing","Working"] }
```

## Topics

### `topics_list`

List topics in an area, with their roll-up status.

**Args:** `area?` (default `"Staging"`).

**Request:**
```json
{ "name": "topics_list", "arguments": { "area": "Staging" } }
```

**Response:**
```json
{ "success": true, "area": "Staging",
  "topics": [
    { "name": "user-login", "status": "draft" },
    { "name": "payment-flow", "status": "in-progress" }
  ] }
```

### `topics_add`

Create a new topic. Writes `spec/<area>/<topic>/topic.md` with server-managed frontmatter (`title`, `short`, `created`, `author`).

**Required:** `topic`, `area`, `short`, `content`.

**Constraints:** `content` is trimmed and must be > 10 chars after trim. Any leading `---` frontmatter block in the supplied `content` is stripped.

**Request:**
```json
{ "name": "topics_add", "arguments": {
    "topic": "user-login",
    "area": "Staging",
    "short": "Email/password login with JWT",
    "content": "# user-login\n\n## Overview\nAuthentication system with JWT and refresh tokens."
} }
```

**Response:**
```json
{ "success": true, "message": "Topic 'user-login' created in Staging/",
  "topic": "user-login", "area": "Staging" }
```

### `topics_show`

Inspect what files exist in a topic directory.

**Required:** `topic`. **Optional:** `area`, `show_all`, `from`.

```json
{ "name": "topics_show", "arguments": { "topic": "user-login", "area": "Staging" } }
```

### `topics_delete`

Delete a topic directory.

**Required:** `topic`. **Optional:** `area`, `force` (default `true`).

```json
{ "name": "topics_delete", "arguments": { "topic": "user-login", "force": true } }
```

### `topics_push`

Move a topic to another area (real move — source is removed).

**Required:** `topic`, `area` (target). **Optional:** `source_area` (defaults to `"Staging"`).

**Gated by readiness** if `source_area` is in `[readiness].areas` (default `Staging` and `Fixing`) — the topic must appear in `spec/<source_area>/queue.md` first.

```json
{ "name": "topics_push", "arguments": {
    "topic": "user-login", "area": "Working", "source_area": "Staging"
} }
```

### `topics_pull`

Pull a topic back from another area into `Working`.

**Required:** `topic`, `source_area`.

```json
{ "name": "topics_pull", "arguments": { "topic": "user-login", "source_area": "Build" } }
```

### `topics_progress`

Per-topic task counts in an area.

**Optional:** `area` (default `"Staging"`).

```json
{ "name": "topics_progress", "arguments": { "area": "Working" } }
```

Response shape:
```json
{ "success": true, "area": "Working",
  "topics": [
    { "topic": "user-login", "status": "in-progress", "total_tasks": 5, "completed_tasks": 2 }
  ] }
```

## Asset reading

### `read_asset`

Read one of `topic.md`, `<topic>_spec.md`, or `<topic>_task.md` for a topic. Special case: passing `topic: "templates"` reads from `.agent/modes/default/templates/`.

**Required:** `topic`, `asset_type` ∈ `{"topic", "spec", "task"}`. **Optional:** `area`.

```json
{ "name": "read_asset", "arguments": {
    "topic": "user-login", "asset_type": "spec", "area": "Staging"
} }
```

```json
{ "name": "read_asset", "arguments": {
    "topic": "templates", "asset_type": "spec"
} }
```

### `unispec_read_spec`

Read both the spec and task content for a topic in a single call.

**Required:** `topic`. **Optional:** `area`.

```json
{ "name": "unispec_read_spec", "arguments": { "topic": "user-login", "area": "Working" } }
```

**Response:**
```json
{ "success": true,
  "spec": "---\ntitle: user-login\n...---\n\nPOST /login takes ...",
  "tasks": "---\nspec: user-login\n...---\n\n- [ ] Implement POST /login\n- [ ] ...",
  "spec_file": "user-login_spec.md",
  "task_file": "user-login_task.md" }
```

## Spec & task writing

### `spec_add`

Create `<topic-safe>_spec.md` and `<topic-safe>_task.md` for a topic. The topic itself (the directory and `topic.md`) must already exist (via `topics_add`).

**Required:** `topic`, `area`, `short`, `spec_content`, `task_content`.

**Constraints:** both content fields are trimmed and must be > 10 chars. Any leading `---` block in either content field is stripped before the server prepends its own frontmatter.

```json
{ "name": "spec_add", "arguments": {
    "topic": "user-login", "area": "Staging", "short": "Auth design",
    "spec_content": "POST /login takes {email, password} and returns {jwt} on success.",
    "task_content": "- [ ] Implement POST /login\n- [ ] Add JWT signing\n- [ ] Write tests"
} }
```

### `spec_write`

Overwrite an existing `<topic-safe>_spec.md`. Strips any caller-supplied frontmatter.

**Required:** `topic`, `content`. **Optional:** `area`.

### `task_write`

Overwrite an existing `<topic-safe>_task.md`. **Errors** if the spec file doesn't exist for that topic — use `spec_add` first.

**Required:** `topic`, `content`. **Optional:** `area`.

### `task_status`

Update the `status:` line in `<topic-safe>_task.md`'s frontmatter. Accepted values: `"pending"`, `"working"`, `"complete"`. Does **not** touch the `- [ ]` checkboxes in the body — for that, use `tasks_complete` / `tasks_incomplete`.

**Required:** `topic`, `status`. **Optional:** `area`.

```json
{ "name": "task_status", "arguments": { "topic": "user-login", "status": "working" } }
```

### `tasks_list`

List every `- [ ]` / `- [x]` line in the task file with index, line number, status, and text.

**Required:** `topic`. **Optional:** `area`.

```json
{ "name": "tasks_list", "arguments": { "topic": "user-login", "area": "Working" } }
```

**Response:**
```json
{ "success": true, "topic": "user-login", "area": "Working", "count": 3,
  "tasks": [
    { "index": 0, "line": 7, "status": "pending",   "text": "Implement POST /login" },
    { "index": 1, "line": 8, "status": "complete",  "text": "Add JWT signing helper" },
    { "index": 2, "line": 9, "status": "pending",   "text": "Write tests" }
  ] }
```

### `tasks_complete`

Flip the 0-indexed task in the task file to `[x]`. Index is among task lines, not raw file lines.

**Required:** `topic`, `task_index`. **Optional:** `note`, `area`.

```json
{ "name": "tasks_complete", "arguments": { "topic": "user-login", "task_index": 0 } }
```

### `tasks_incomplete`

Inverse of `tasks_complete` — flip `[x]` back to `[ ]`.

**Required:** `topic`, `task_index`. **Optional:** `note`, `area`.

## Notes

### `notes_read`

Read the `## Notes` section of `<topic-safe>_task.md`.

**Required:** `topic`. **Optional:** `area`.

### `notes_add`

Append a dated note (`- **YYYY-MM-DD**: <text>`) to the `## Notes` section. Creates the section if it doesn't exist.

**Required:** `topic`, `note`. **Optional:** `area`.

```json
{ "name": "notes_add", "arguments": {
    "topic": "user-login", "note": "Chose argon2id over bcrypt — see issue #42"
} }
```

## Readiness queue

All queue tools operate on `spec/<area>/queue.md`.

### `queue_list`

Show the raw text of the queue file.

**Optional:** `area`.

### `queue_add`

Add a topic to the queue (appended by default, inserted at `position` if provided).

**Required:** `topic`. **Optional:** `position` (default `-1` = append), `area`.

```json
{ "name": "queue_add", "arguments": { "topic": "user-login", "area": "Staging" } }
```

### `queue_remove`

Remove a topic from the queue.

**Required:** `topic`. **Optional:** `area`.

### `queue_check`

Returns `{ "ready": true|false }` depending on whether the topic appears in the queue.

**Required:** `topic`. **Optional:** `area`.

```json
{ "name": "queue_check", "arguments": { "topic": "user-login", "area": "Staging" } }
```

### `queue_reorder`

Move a topic to a new position within its current queue.

**Required:** `topic`, `new_position`. **Optional:** `area`.

```json
{ "name": "queue_reorder", "arguments": {
    "topic": "user-login", "new_position": 0, "area": "Staging"
} }
```

## Index (file ↔ spec linking)

### `index_add`

Add a topic↔path link.

**Required:** `topic`, `path`. **Optional:** `area`, `link_type`, `tags` (comma-separated), `annotation`, `exports`, `descriptions`, `export_types`, `signatures`.

```json
{ "name": "index_add", "arguments": {
    "topic": "user-login", "path": "src/auth/login.rs",
    "link_type": "implementation", "tags": "auth,backend",
    "annotation": "Core POST /login handler"
} }
```

### `unispec_bind_spec`

Bind a code file to a spec file path. Used when you want a stronger "this code implements this spec file" linkage.

**Required:** `spec_path`, `file_path`, `topic`. **Optional:** `area`.

### `index_find`

Find links by topic, path, or tag.

**Required:** `query`. **Optional:** `by` ∈ `{"topic" (default), "path", "tag"}`.

```json
{ "name": "index_find", "arguments": { "query": "user-login", "by": "topic" } }
```

### `index_lookup`

Find an export by full ID, e.g., `user-login:login_user`.

**Required:** `id`.

### `index_list`

List every link, with optional filters.

**Optional:** `topic`, `path`, `tag`.

### `index_graph`

Export the entire link graph as JSON (nodes + edges) for visualization.

**Args:** none.

### `index_backlinks`

Generate a markdown backlinks block for a topic.

**Required:** `topic`.

## Dynamic connector tools

Each `[[connector]]` in `.agent/config.toml` is published as a tool named `unispec_<connector-name>`. Calling it shells out to the configured command and returns combined stdout. See [connectors.md](connectors.md).

## Discovering the live surface

You can verify the exact list of tools a running server publishes:

```bash
printf '%s\n' \
  '{"jsonrpc":"2.0","id":1,"method":"initialize","params":{}}' \
  '{"jsonrpc":"2.0","id":2,"method":"tools/list","params":{}}' \
  | unispec mcp 2>/dev/null
```

This is the canonical answer when the documentation and the binary diverge — the binary wins.
