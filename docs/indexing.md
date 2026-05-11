# Indexing

The index links each topic to one or more code files (or directories). Once linked, AI agents can find which code implements which spec without re-reading whole files.

Indexing supports tags, annotations, exports (capability registry), graph visualization, and backlinks.

## Why index?

Without indexing:
- An agent must read every file to figure out which spec it implements.
- Code-to-spec connections are lost when files move or split.
- Agents on different topics can't tell what each other's topic exposes.

With indexing:
- `index_find { query: "user-login", by: "topic" }` returns every linked file in one call.
- `index_backlinks { topic: "user-login" }` lists every topic that depends on `user-login`.
- `index_exports` (CLI) lists the public surface a topic provides — agents query exports instead of reading entire files.

## MCP vs CLI

Indexing has both an MCP surface and a CLI surface. The MCP server publishes a subset.

### MCP tools (callable by agents)

| Tool | Required args | Description |
|------|---------------|-------------|
| `index_add` | `topic, path` (`area?`, `link_type?`, `tags?`, `annotation?`) | Add a link. |
| `index_find` | `query` (`by?`) | Search by `topic` (default), `path`, or `tag`. |
| `index_lookup` | `id` | Find an export by full `topic:name` ID. |
| `index_list` | — (`topic?`, `path?`, `tag?`) | List all links with optional filters. |
| `index_graph` | — | Export the link graph as JSON for visualization. |
| `index_backlinks` | `topic` | Markdown block of files linked to the topic. |
| `unispec_bind_spec` | `spec_path, file_path, topic` (`area?`) | Bind a code file to a spec record. |

### CLI-only (not in MCP)

These exist as `unispec index <sub>` but are not exposed as MCP tools. Use the host shell.

| CLI | Description |
|-----|-------------|
| `unispec index remove` | Remove a single link. |
| `unispec index cleanup` | Remove links to non-existent topics or paths. |
| `unispec index tags` | List all unique tags. |
| `unispec index exports` | List exports (functions, classes, …) for a topic. |
| `unispec index query` | Query exports by name/type/description/id. |
| `unispec index depends` | Find topics that depend on a given topic. |
| `unispec index callers` | Find references to a symbol. |
| `unispec index full` | Show overall index statistics. |
| `unispec index watch` | File-system watcher; updates links on file changes. |

If you want any of those from an agent, shell out to the CLI.

## Basic usage

### Add a link

```bash
# Link a file
unispec index add --topic user-login --path src/auth/login.rs

# Link a directory
unispec index add --topic user-login --path src/auth/

# Explicit link type (clap flag is kebab-case; the field is `link_type` in
# source and TOML, but the CLI accepts `--link-type`)
unispec index add --topic user-login --path src/auth/login.rs \
  --link-type implementation

# With tags
unispec index add --topic user-login --path src/auth/login.rs --tags auth,backend,security

# With an annotation
unispec index add --topic user-login --path src/auth/login.rs \
  --annotation "Core login logic — password verification"

# Everything together
unispec index add --topic user-login --path src/auth/login.rs \
  --link-type implementation \
  --tags auth,backend,security \
  --annotation "Core login logic — password verification"
```

Valid `--link-type` values: `implementation` (most common), `test`, `doc`, `config`, `directory`. If omitted, the type is auto-detected as `file` or `directory` based on what `--path` points at.

From MCP:
```json
{
  "name": "index_add",
  "arguments": {
    "topic": "user-login",
    "path": "src/auth/login.rs",
    "link_type": "implementation",
    "tags": "auth,backend,security",
    "annotation": "Core login logic — password verification"
  }
}
```

### List links

```bash
unispec index list                                # all
unispec index list --topic user-login
unispec index list --path src/auth/login.rs
unispec index list --tag backend
```

From MCP:
```json
{ "name": "index_list", "arguments": { "topic": "user-login" } }
```

### Find by topic / path / tag

```bash
unispec index find user-login --by topic
unispec index find login.rs --by path
unispec index find security --by tag
unispec index find password --by annotation     # CLI supports annotation too
```

From MCP — `by` is `"topic"` (default), `"path"`, or `"tag"`. Annotation search is CLI-only.

### Remove (CLI only)

```bash
unispec index remove --topic user-login --path src/auth/login.rs
```

## Link record format

Each link is stored in `spec/index.toml`:

```toml
[[links]]
topic = "user-login"
area = "Working"
path = "src/auth/login.rs"
type = "file"             # or "directory"
added = "2026-03-26T10:00:00Z"
tags = ["auth", "backend", "security"]
annotation = "Core login logic"
```

| Field | Description |
|-------|-------------|
| `topic` | The topic this file implements. |
| `area` | The area containing the topic at link time. |
| `path` | File or directory path. |
| `type` | `"file"` or `"directory"` (auto-detected). |
| `added` | Timestamp. |
| `tags` | Categorization. |
| `annotation` | Free-form note about why this file is linked. |
| `exports` | Optional: declared public surface of the file (see below). |

## Exports (capability registry, CLI only)

Exports declare what a file makes available to other topics. This is the central feature for cross-topic agent collaboration: an agent on `checkout-flow` can query what `user-login` exports without reading `login.rs`.

### Add a link with exports

```bash
unispec index add \
  --topic user-login \
  --path src/auth/login.rs \
  --exports login_user,logout,validate_token \
  --descriptions "Authenticate user,Clear session,Verify token" \
  --export-types function,function,function \
  --signatures "fn login_user(email: String, pass: String) -> Result<User>"
```

`exports`, `descriptions`, `export-types`, and `signatures` are positional comma-separated lists; the Nth entry of each refers to the same export.

Export types: `function`, `class`, `endpoint`, `model`, `service`, `config`.

### List / query exports (CLI only)

```bash
unispec index exports --topic user-login
unispec index exports                              # all topics

unispec index query login --by name
unispec index query function --by type
unispec index query authenticate --by description
unispec index query user-login --by id
```

### Lookup by full ID (MCP)

```json
{ "name": "index_lookup", "arguments": { "id": "user-login:login_user" } }
```

### Reference comments

When one topic uses another's export, add a reference comment so dependencies are tracked:

```python
from auth import login_user  # ref:index:user-login:login_user
```
```rust
use auth::login_user; // ref:index:user-login:login_user
```
```typescript
import { loginUser } from './auth'; // ref:index:user-login:login_user
```

`unispec index depends --topic user-login` (CLI) then lists every topic that uses one of its exports.

## Graph export (MCP)

```bash
unispec index graph
```

```json
{
  "nodes": [
    {"id": "topic-user-login", "topic": "user-login", "area": "Working", "path": "", "tags": []},
    {"id": "path-src-auth-login-rs", "topic": "user-login", "area": "Working", "path": "src/auth/login.rs", "tags": ["auth"]}
  ],
  "edges": [
    {"source": "topic-user-login", "target": "path-src-auth-login-rs", "type": "links_to"}
  ]
}
```

Feeds visualizers like Obsidian, Gephi, D3.js, or Graphviz.

## Backlinks (MCP)

```bash
unispec index backlinks --topic user-login
```

```markdown
# Backlinks: user-login

Area: Working

## Linked Files
- [src/auth/login.rs](src/auth/login.rs) — Core login logic
- [src/auth/password.rs](src/auth/password.rs) — password handling
- [tests/login_test.py](tests/login_test.py) — test coverage

## Tags
- auth
- backend
- security
```

## Code-analysis store (TOML)

`unispec ingest run <path>` parses every supported source file and writes the result to `spec/code_analysis.toml` (or per-file markdown, per `[ingest].output_format` in `.agent/config.toml`).

### File structure

```toml
[topics.myproject]
area = "Ingested"
source_path = "./src"
analyzed = "2026-03-26T10:00:00Z"

[[topics.myproject.files]]
path = "src/main.rs"
language = "rust"
functions = [
  { name = "main", signature = "fn main()", start_line = 1, end_line = 10 }
]
structs = [{ name = "Cli" }]
enums = []
imports = ["use std::..."]
```

### Configuration

`.agent/config.toml`:

```toml
[ingest]
auto_index = true
capture_functions = true
capture_structs = true
capture_enums = true
capture_imports = true
output_format = "toml"           # "toml" | "md" | "both"
languages = []                   # empty = all supported
```

There is no MCP tool named `code_analysis` or `code_parse` — these queries are CLI-only via `unispec parse file`. If you need the data from an agent, shell out.

## Best practices

### Tags

Establish conventions so filtering is reliable:

| Prefix | Meaning |
|--------|---------|
| `backend` | Server-side code |
| `frontend` | Client-side code |
| `tests` | Test files |
| `docs` | Documentation |
| `config` | Configuration |
| `deprecated` | Legacy code |

### Annotations

Explain why the file is linked, not what it is:

```bash
unispec index add --topic payment --path src/payment/stripe.rs \
  --annotation "Main integration point — all payment flows go through here"
```

### Workflow example

```bash
# 1. Create a topic (or use MCP topics_add)
unispec topic add "Payment API" -a Staging

# 2. Write the spec (use /spec or MCP spec_add)

# 3. Link implementation files with tags + annotations
unispec index add --topic payment-api --path src/payment/stripe.rs \
  --tags payments,stripe,backend --annotation "Main Stripe integration"
unispec index add --topic payment-api --path src/payment/webhook.rs \
  --tags payments,webhook,backend --annotation "Stripe webhook handler"

# 4. Link tests
unispec index add --topic payment-api --path tests/payment/ --tags payments,tests

# 5. Query
unispec index find stripe --by tag
unispec index backlinks --topic payment-api
unispec index graph > graph.json
```

---

## See also

- [Commands Reference](commands.md)
- [MCP Integration](mcp.md)
- [Configuration](configuration.md)
