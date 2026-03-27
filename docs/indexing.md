# Indexing

Indexing links your specs to your code. When you index a file to a topic, AI agents can find the relevant code when working on that spec. The index now supports tags, annotations, exports (capability registry), and provides graph visualization and backlinks.

## Why Index?

Without indexing:
- AI doesn't know which code implements which feature
- You have to explain context every time
- Code-to-spec connections are lost
- Agent A reading Agent B's code must read entire files to find what it needs

With indexing:
- AI sees "this file is for the user-login feature"
- Automatic context when reading specs
- Traceability from code to requirements
- Searchable by tags and annotations
- **Queryable exports** - find just what functions/types are available without reading all the code

## The Problem Without Exports

```
Agent A on "checkout-flow" needs auth from Agent B's "user-login"
- Must read ALL of login.rs to find usable functions (500+ lines)
- Wasted context - just needs 3 functions
```

## The Solution: Exports / Capability Registry

Now index declares what's available from each file:

```
user-login → src/auth/login.rs exports:
  - user-login:login_user (function) - "Authenticate and create session"
  - user-login:validate_token (function) - "Verify JWT token"  
  - user-login:logout (function) - "Clear user session"
```

Agent A can now:
1. Query: "what does user-login export?"
2. Get: Just the 3 functions with descriptions
3. Use: `from auth import login_user  # ref:index:user-login:login_user`
4. Backlink auto-created: checkout-flow depends on user-login

## Basic Usage

### Add a Link

```bash
# Link a file to a topic (basic)
unispec index add --topic "user-login" --path src/auth/login.rs

# Link a directory
unispec index add --topic "user-login" --path src/auth/

# Link with tags
unispec index add --topic "user-login" --path src/auth/login.rs --tags "auth,backend,security"

# Link with an annotation
unispec index add --topic "user-login" --path src/auth/login.rs --annotation "Core login logic, handles password verification"
```

### List Links

```bash
# All links
unispec index list

# Links for a topic
unispec index list --topic "user-login"

# Links for a file
unispec index list --path src/auth/login.rs

# Links with a specific tag
unispec index list --tag backend
```

### Find Links

```bash
# Find by topic name
unispec index find "user-login" --by topic

# Find by file path
unispec index find "login.rs" --by path

# Find by tag
unispec index find "security" --by tag

# Find by annotation text
unispec index find "password" --by annotation
```

### Remove a Link

```bash
unispec index remove --topic "user-login" --path src/auth/login.rs
```

## Enhanced Index Format

The index now stores rich metadata:

```toml
[[links]]
topic = "user-login"
area = "Working"
path = "src/auth/login.rs"
type = "file"
added = "2026-03-26T10:00:00Z"
tags = ["auth", "backend", "security"]
annotation = "Core login logic"
```

### Fields

| Field | Description |
|-------|-------------|
| `topic` | Topic name |
| `area` | Area containing the topic |
| `path` | File or directory path |
| `type` | "file" or "directory" |
| `added` | Timestamp when linked |
| `tags` | List of tags for categorization |
| `annotation` | Optional note about why linked |
| `exports` | List of available functions/classes/etc |

## Exports (Capability Registry)

Exports declare what's available from a linked file. This is the key feature for parallel agents - Agent A can find exactly what Agent B's topic provides without reading all the code.

### Adding Exports

```bash
# Add link with exports (comma-separated names, types, descriptions)
unispec index add \
  --topic "user-login" \
  --path src/auth/login.rs \
  --exports "login_user,logout,validate_token" \
  --descriptions "Authenticate user,Clear session,Verify token" \
  --export-types "function,function,function"

# With function signatures
unispec index add \
  --topic "user-login" \
  --path src/auth/login.rs \
  --exports "login_user" \
  --descriptions "Authenticate and create session" \
  --export-types "function" \
  --signatures "fn login_user(email: String, pass: String) -> Result<User>"
```

### Export Types

Available types:
- `function` - Regular functions
- `class` - Classes or structs
- `endpoint` - API endpoints
- `model` - Data models/types
- `service` - Service definitions
- `config` - Configuration items

### Listing Exports

```bash
# List all exports for a topic
unispec index exports --topic user-login

# Output:
# Exports for 'user-login':
#   login_user (function)
#     Description: Authenticate and create session
#     ID: user-login:login_user
#     Signature: fn login_user(email: String, pass: String) -> Result<User>
#
#   logout (function)
#     Description: Clear user session
#     ID: user-login:logout
#
#   validate_token (function)
#     Description: Verify JWT token
#     ID: user-login:validate_token

# List all exports across all topics
unispec index exports
```

### Querying Exports

```bash
# Search by name
unispec index query "login" --by name

# Search by type
unispec index query "function" --by type

# Search by description
unispec index query "authenticate" --by description

# Search by ID
unispec index query "user-login" --by id
```

### Finding Dependencies (What Uses What)

```bash
# Find what topics depend on a given topic
unispec index depends --topic user-login

# Output:
# Topics depending on 'user-login':
#   checkout-flow
#     - login_user (user-login:login_user)
#     - validate_token (user-login:validate_token)
```

### Lookup by ID

```bash
# Find export by full ID
unispec index lookup --id user-login:login_user

# Output:
# Found: user-login:login_user
#   Name: login_user
#   Type: function
#   Topic: user-login
#   Path: src/auth/login.rs
#   Description: Authenticate and create session
```

### Reference Comments (Auto-Backlinks)

When using another topic's exports in your code, include the reference:

```python
# Python
from auth import login_user  # ref:index:user-login:login_user

# Rust
use auth::login_user; // ref:index:user-login:login_user

# TypeScript
import { loginUser } from './auth'; // ref:index:user-login:login_user
```

This creates automatic backlinks - when you query `index depends --topic user-login`, it will show which other topics are using its exports.

Tags allow you to categorize and filter links.

### Adding Tags

```bash
# Add tags when creating a link
unispec index add --topic "payment" --path src/payment/stripe.rs --tags "payments,stripe,backend"

# Multiple tags (comma-separated)
unispec index add --topic "user-auth" --path src/auth/ --tags "auth,security,api"
```

### Searching by Tag

```bash
# Find all links with a tag
unispec index find "backend" --by tag

# List links filtered by tag
unispec index list --tag security
```

### Listing All Tags

```bash
# See all tags in the index
unispec index tags
```

Output:
```
Tags in index:
  backend (15 links)
  security (8 links)
  auth (12 links)
  frontend (6 links)
```

## Annotations

Annotations add context to links - why is this file linked?

```bash
# Add annotation when linking
unispec index add \
  --topic "user-login" \
  --path src/auth/validate.rs \
  --annotation "Contains password hashing and validation logic"

# Search by annotation content
unispec index find "password" --by annotation
```

## Graph Export

Export the index as a graph JSON for visualization tools:

```bash
unispec index graph
```

Output:
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

This can be visualized in tools like:
- Obsidian
- Gephi
- D3.js
- Graphviz

## Backlinks

Generate a backlinks file for any topic:

```bash
unispec index backlinks --topic "user-login"
```

Output:
```
# Backlinks: user-login

Area: Working

## Linked Files

- [src/auth/login.rs](src/auth/login.rs) - Core login logic
- [src/auth/password.rs](src/auth/password.rs) - password handling
- [tests/login_test.py](tests/login_test.py) - test coverage

## Tags

- auth
- backend
- security
```

## MCP Tools

The enhanced index is available via MCP:

| Tool | Description |
|------|-------------|
| `index_list` | List links, optionally filtered by topic, path, or tag |
| `index_add` | Add link with tags, annotation, and exports |
| `index_find` | Find by topic, path, tag, or annotation |
| `index_tags` | List all unique tags |
| `index_graph` | Export graph JSON |
| `index_backlinks` | Generate backlinks markdown |
| `index_exports` | List exports for a topic |
| `index_query` | Query exports by name, type, description, or ID |
| `index_depends` | Find what topics depend on a given topic |
| `index_lookup` | Find export by full ID |

### MCP Examples

```python
# Get exports for a topic
{"name": "index_exports", "arguments": {"topic": "user-login"}}

# Query exports by name
{"name": "index_query", "arguments": {"query": "login", "by": "name"}}

# Query exports by type
{"name": "index_query", "arguments": {"query": "function", "by": "type"}}

# Query exports by description
{"name": "index_query", "arguments": {"query": "authenticate", "by": "description"}}

# Find what depends on a topic
{"name": "index_depends", "arguments": {"topic": "user-login"}}

# Lookup by full ID
{"name": "index_lookup", "arguments": {"id": "user-login:login_user"}}

# Find links by tag
{"name": "index_find", "arguments": {"query": "backend", "by": "tag"}}

# Get graph for visualization
{"name": "index_graph", "arguments": {}}

# Get backlinks for a topic
{"name": "index_backlinks", "arguments": {"topic": "user-login"}}
```

## Best Practices

### Use Tags for Categorization

```bash
# By layer
unispec index add --topic "api" --path api/ --tags "backend,api,rest"

# By feature
unispec index add --topic "api" --path api/v2/ --tags "backend,api,v2"

# By status
unispec index add --topic "api" --path api/legacy/ --tags "backend,api,deprecated"
```

### Use Annotations for Context

```bash
# Explain why this file is linked
unispec index add \
  --topic "payment" \
  --path src/payment/stripe.rs \
  --annotation "Main integration point - all payment flows go through here"

# Note important details
unispec index add \
  --topic "auth" \
  --path src/auth/jwt.rs \
  --annotation "Token generation - handles refresh tokens differently on mobile vs web"
```

### Tag Conventions

Establish conventions for consistency:

| Prefix | Meaning |
|--------|---------|
| `backend` | Server-side code |
| `frontend` | Client-side code |
| `tests` | Test files |
| `docs` | Documentation |
| `config` | Configuration |
| `deprecated` | Legacy code |

## Index in Action

### With AI Editors

When you open a spec in Cursor/Claude:

```
User Login spec says:
- Email/password login
- Password reset

Linked files (tagged: auth, backend):
- src/auth/login.rs (Core login logic)
- src/auth/password.rs
- tests/login_test.py

Files tagged 'security':
- src/auth/rate_limit.rs
- src/auth/mfa.rs
```

### With MCP

```
Find all files tagged 'security' for user-login
→ src/auth/rate_limit.rs (rate limiting)
→ src/auth/mfa.rs (two-factor)
→ src/auth/session.rs (session security)
```

## Example Workflow

```bash
# 1. Create topic
unispec topic add "Payment API" -a Staging

# 2. Write spec
# ... edit spec/Staging/payment-api/spec.md

# 3. Link code with tags and annotations
unispec index add \
  --topic "payment-api" \
  --path src/payment/stripe.rs \
  --tags "payments,stripe,backend" \
  --annotation "Main Stripe integration"

unispec index add \
  --topic "payment-api" \
  --path src/payment/webhook.rs \
  --tags "payments,webhook,backend" \
  --annotation "Handles Stripe webhook events"

# 4. Link tests
unispec index add \
  --topic "payment-api" \
  --path tests/payment/ \
  --tags "payments,tests"

# 5. Query by tag
unispec index find "stripe" --by tag

# 6. Get graph for visualization
unispec index graph > graph.json

# 7. Generate backlinks
unispec index backlinks --topic "payment-api"
```

---

## Code Analysis Store (TOML)

UniSpec stores code analysis data in `spec/code_analysis.toml`. This provides a central repository for all code extracted from ingested topics.

### File Structure

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
structs = [
  { name = "Cli" }
]
enums = []
imports = ["use std::..."]
```

### How It Works

1. **During Ingest**: When you run `unispec ingest run`, tree-sitter parses all files
2. **Configuration**: Controlled by `.agent/config.toml` `[ingest]` settings
3. **Storage**: Saved to `spec/code_analysis.toml` (or MD files, or both)

### Configuration

In `.agent/config.toml`:

```toml
[ingest]
auto_index = true           # Auto-add to index.toml when ingesting
capture_functions = true    # Extract functions
capture_structs = true      # Extract structs
capture_enums = true       # Extract enums
capture_imports = true     # Extract imports
output_format = "toml"      # "toml", "md", or "both"
languages = []              # Specific languages (empty = all)
```

### MCP Access

```python
# Query code analysis from topics
{"name": "code_analysis", "arguments": {"topic": "myproject", "item_type": "functions"}}

# Parse any file on-demand
{"name": "code_parse", "arguments": {"path": "src/main.rs", "item_type": "functions"}}
```

---

## See Also

- [Commands Reference](commands.md) - CLI command documentation
- [Configuration Reference](configuration.md) - Config files, environment variables
- [MCP Documentation](mcp.md) - How AI uses indexing

*See commands.md for all CLI commands, or mcp.md for how AI uses indexing.*