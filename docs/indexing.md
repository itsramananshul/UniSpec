# Indexing

Indexing links your specs to your code. When you index a file to a topic, AI agents can find the relevant code when working on that spec.

## Why Index?

Without indexing:
- AI doesn't know which code implements which feature
- You have to explain context every time
- Code-to-spec connections are lost

With indexing:
- AI sees "this file is for the user-login feature"
- Automatic context when reading specs
- Traceability from code to requirements

## Basic Usage

### Add a Link

```bash
# Link a file to a topic
unispec index add --topic "user-login" --path src/auth/login.rs

# Link a directory
unispec index add --topic "user-login" --path src/auth/
```

### List Links

```bash
# All links
unispec index list

# Links for a topic
unispec index list --topic "user-login"

# Links for a file
unispec index list --path src/auth/login.rs
```

### Find Links

```bash
# Find by topic name
unispec index find "user-login" --by topic

# Find by file path
unispec index find "login.rs" --by path
```

### Remove a Link

```bash
unispec index remove --topic "user-login" --path src/auth/login.rs
```

## How Indexing Works

The index stores:
- Topic name
- File path
- Link type (file, directory)
- Added date

```
spec/
└── Staging/
    └── user-login/
        └── .index/           # Auto-generated
            └── links.json   # Stored links
```

## Index in Action

### With AI Editors

When you open a spec in Cursor/Claude:

```
User Login spec says:
- Email/password login
- Password reset

Linked files:
- src/auth/login.rs
- src/auth/password.rs
- tests/test_login.py
```

The AI automatically sees the linked files.

### With MCP

Query via MCP:

```
What files are linked to the user-login topic?
→ src/auth/login.rs
→ src/auth/password.rs  
→ tests/test_login.py
→ docs/auth-flow.md
```

## Best Practices

### Index Early

Add links when creating a topic:

```bash
unispec topic add "API" -a Staging
unispec index add --topic "api" --path api/
```

### Index Incrementally

As you create files, link them:

```bash
# New file created
touch src/models/user.rs
unispec index add --topic "user-login" --path src/models/user.rs
```

### Link Tests Too

Tests prove specs are met. Link them:

```bash
unispec index add --topic "user-login" --path tests/
```

## Index Patterns

### By Feature

```
topic: "user-login"
  - src/auth/login.rs
  - src/auth/password.rs
  - src/auth/session.rs
  - tests/login_test.py
  - docs/login-flow.md
```

### By Component

```
topic: "api"
  - api/routes/
  - api/middleware/
  - api/models/
```

### By File Type

```
topic: "frontend"
  - src/views/
  - src/components/
  - src/stores/
```

## Automation


### Git Hooks

Add to `.git/hooks/pre-commit`:

```bash
#!/bin/bash
# Auto-index new files matching patterns
find . -name "*.rs" -newer .git/index | while read f; do
  echo "Indexing $f"
done
```

## Troubleshooting

### "Topic not found"

Make sure the topic exists:
```bash
unispec topic list
```

### "File not found"

Use the full path or path relative to project root:
```bash
unispec index add --topic "api" --path ./src/api/main.rs
```

### Index is slow

Large projects can have many links. Use filtering:
```bash
unispec index list --topic "api" | head -20
```

## Index and Modes

Different modes can have different indexing schemes:

### Simple Mode
- Link code to features
- Link tests to features

### Sprint Mode  
- Link code to sprint items
- Link PRs to features

### Docs Mode
- Link docs to topics
- Link examples to APIs

## Tips

1. **Be consistent** - Use the same topic names
2. **Link tests** - They're part of the spec too
3. **Link docs** - API docs, architecture decisions
4. **Review periodically** - Clean up stale links

## Example Workflow

```bash
# 1. Create topic
unispec topic add "Payment API" -a Staging

# 2. Write spec
# ... edit spec/Staging/payment-api/spec.md

# 3. Link existing code
unispec index add --topic "payment-api" --path src/payment/
unispec index add --topic "payment-api" --path tests/payment/

# 4. As you create files, keep linking
touch src/payment/stripe.rs
unispec index add --topic "payment-api" --path src/payment/stripe.rs

# 5. AI can now find all payment-related code
```

---

## See Also

- [Commands Reference](commands.md) - CLI command documentation
- [Configuration Reference](configuration.md) - Config files, environment variables
- [MCP Documentation](mcp.md) - How AI uses indexing

*See commands.md for all CLI commands, or mcp.md for how AI uses indexing.*
