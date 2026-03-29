# UniSpec Skill

UniSpec is a structured development system that enforces quality through a deliberate pipeline. No cutting corners.

---

## The Vision

Software development fails when we skip steps. We write code before we understand the problem. We skip tests because we're "done". We ship bugs because we didn't think it through.

**UniSpec fixes this by making the steps unavoidable.**

Every feature flows through a pipeline:
```
Roadmap → Spec → Working → Testing → Debugging → Build
```

Each area has rules. You can't skip areas. You can't skip tests. The system enforces quality.

---

## Philosophy

### 1. Think Before You Build

Before writing code, you must understand:
- What problem are you solving?
- Who has this problem?
- How do you know you solved it?

If you can't answer these, go back to Spec.

### 2. Spec is the Contract

When you're in Working, the spec is law. Don't modify it. If you find problems, push back to Spec and discuss.

The spec protects both you and the project:
- You know exactly what to build
- The project knows what to expect

### 3. Tests Are Requirements

A feature isn't done until tests pass. Not "mostly working". Not "works on my machine". Tests pass.

If tests fail, you're not done. Simple as that.

### 4. No Shortcuts

You can go faster by skipping steps. You'll regret it later. UniSpec makes skipping hard.

The pipeline exists for a reason. Trust it.

### 5. Build is Sacred

Build contains shipped work. Treat it with respect. You can pull from Build if needed, but the default is forward motion.

---

## The Areas

### Roadmap
Ideas. Big picture. What to build.

**Think about:** What problem? Who benefits? Why does it matter?

**Don't do:** Write code. That's Working's job.

### Spec
Define what you're building. Make it testable.

**Include:**
- Problem statement
- User stories
- Acceptance criteria (testable!)
- Examples

**Output:** A spec that you could hand to someone and they could build it correctly.

### Working
Build the thing. Follow the spec.

**Rules:**
- Don't modify the spec
- If you find issues, document them and push back to Spec
- Mark progress in tasks.md

**Output:** Working code that implements the spec.

### Testing
Run tests. Verify everything works.

**Rules:**
- All tests pass → Push to Build
- Tests fail → Go to Debugging

### Debugging
Tests failed. Find out why. Fix it.

**Process:**
1. Read the error (it tells you what's wrong)
2. Find the root cause
3. Fix it
4. Back to Testing

### Build
Done. Shipped. Complete.

**Rules:**
- Only push here if tests pass
- You can pull back if you find bugs
- Push to other areas is locked for checked-out topics

---

## Commands

### Topic Management

```bash
# List topics in an area
unispec topic list --area Working

# Create new topic
unispec topic add my-feature --area Roadmap

# Push topic to another area (moves it)
unispec topic push my-feature --to Spec --from Roadmap

# Pull topic from another area
unispec topic pull my-feature --from Build

# Show topic details
unispec topic show my-feature

# Delete topic
unispec topic remove my-feature
```

### Areas

```bash
# List areas
unispec area list

# Create area
unispec area add InReview

# Show area health
unispec area health
```

### Index (Code Navigation)

```bash
# Add file to topic index
unispec index add --topic my-feature --path src/auth.rs

# List files in topic
unispec index list --topic my-feature

# Find file by pattern
unispec index find "auth"

# Show topic exports (functions, structs)
unispec index exports my-feature
```

### Code Analysis

```bash
# Parse code structure
unispec parse file src/auth.rs --pattern functions

# Get code analysis for topic
unispec code-analysis my-feature
```

### Ingest

```bash
# Ingest codebase
unispec ingest run --topic my-feature

# Watch mode
unispec ingest run --watch
```

---

## Workflows

### Feature Development

1. **Roadmap** - Create topic, add plan
2. **Spec** - Write detailed spec, make it testable
3. **Working** - Implement, follow the spec
4. **Testing** - Run tests, fix failures
5. **Debugging** - If tests fail, debug
6. **Build** - When tests pass, ship it

### Bug Fixes

1. **Roadmap** - Document the bug
2. **Spec** - Define what "fixed" looks like
3. **Working** - Fix the code
4. **Testing** - Verify tests pass
5. **Build** - Ship the fix

### Refactoring

1. **Roadmap** - Explain why refactoring is needed
2. **Spec** - Define the target structure
3. **Working** - Transform the code
4. **Testing** - Ensure behavior unchanged
5. **Build** - Ship the refactor

---

## Nested Topics

Topics can be nested with `/`:

```
Roadmap/
├── user-auth/
│   └── login/
│       └── oauth/
```

When you pull a nested topic, ancestors are pulled too:

```
Pull: user-auth/login/oauth
Result:
  - Pulls: user-auth/login/oauth
  - Also pulls: user-auth/login (if exists)
  - Also pulls: user-auth (if exists)
```

This gives you context from parent specs.

---

## Checkout System

Topics in Build can be checked out for fixes:

- **Pull from Build** → Topic moves to Working, marked as checked out by you
- **Push to Build** → Topic moves back, checkout cleared
- **Checked out by others** → You can't push/pull until they check in

The checkout prevents conflicts. Only one person works on a topic at a time.

---

## Writing Good Specs

A spec should answer:

1. **What problem does this solve?**
2. **Who has this problem?**
3. **What does success look like?**
4. **What are the edge cases?**
5. **How do we verify it works?**

### Template

```markdown
---
title: Feature Name
impact: high
change_type: feature
---

## Problem

What problem does this solve?

## Users

Who has this problem?

## Solution

What are we building?

## Requirements

### Must Have
- [ ] Requirement 1
- [ ] Requirement 2

### Should Have
- [ ] Nice to have

### Won't Have
- Out of scope

## Acceptance Criteria

- [ ] Criterion 1
- [ ] Criterion 2

## Examples

**Input:** X
**Output:** Y

**Input:** A
**Output:** B

## Edge Cases

- What happens if X?
- What happens if Y?
```

---

## Writing Tests

Tests verify your code works. Write them alongside code.

### Test Structure

```markdown
## Tests

### Should do X
- Given: A and B
- When: I do C
- Then: X happens

### Should handle Y error
- Given: invalid input
- When: I do C
- Then: Error Z is returned
```

### What to Test

- Happy path (normal operation)
- Edge cases (empty, null, boundary values)
- Error cases (invalid input, network failure)
- Edge cases specific to your domain

### What NOT to Test

- Implementation details (test behavior, not how)
- Third-party code
- Trivial code (getters/setters)

---

## Debugging

When tests fail:

1. **Read the error** - It's trying to help you
2. **Locate the failure** - Which test? Which line?
3. **Find the root cause** - Ask "why" five times
4. **Fix it** - Make minimum change
5. **Verify** - Run tests again

### Common Mistakes

- Changing multiple things at once
- Not running tests after changes
- Ignoring edge cases
- Hardcoding values
- Not checking null

---

## Tips

1. **Small commits** - Each topic should do one thing
2. **Clear names** - Topic names describe what it does
3. **Test early** - Write tests before or alongside code
4. **Ask questions** - If you're unsure, ask in Spec
5. **Trust the process** - The pipeline exists for a reason

---

## Mental Model

Think of UniSpec like a factory assembly line:

- Each station (area) has a specific job
- Products (topics) move through stations
- Each station inspects and improves
- Defective products go back for rework
- Only perfect products ship

You can't skip stations. You can't skip inspections. The result is quality.

---

## When Stuck

1. **Unclear spec?** → Push to Spec, write it better
2. **Tests failing?** → Go to Debugging, figure out why
3. **Scope creep?** → Push back to Spec, define scope
4. **Blocked?** → Document blockers, push to Roadmap

---

## Remember

> Programs must be written for people to read, and only incidentally for machines to execute. — Harold Abelson

UniSpec helps you write code worth reading by forcing you to think before you type.
