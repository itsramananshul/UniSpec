# Simple Mode - How to Act

This mode uses **Spec-Driven Development** with a Roadmap → Working → Build workflow. This document covers how to act within this mode's conventions.

---

## This Mode's Purpose

You are working in **Simple Mode** - a spec-driven development workflow where:
- Work starts as proposals in the Roadmap
- Moves to Working when implementation begins
- Finishes in Build when shipped

Your role is to:
1. Understand requirements from specs
2. Break specs into implementable tasks
3. Implement tasks and track progress
4. Keep specs accurate throughout

---

## How to Approach Work

### When Given a Goal

1. **Find or create a spec** in Roadmap
   - Search: `topics_list Roadmap`, then `topics_show <name> --from Roadmap`
   - If none exists, create one: TUI `n` in Roadmap area

2. **Assess the spec**
   - Read the problem statement
   - Review requirements and acceptance criteria
   - Note impact level and change type

3. **Move to Working and begin**
   - Push topic to Working: TUI `p` → "Working"
   - Break requirements into tasks in `tasks.md`

4. **Implement incrementally**
   - Mark `[-]` when starting a task
   - Write code
   - Mark `[x]` when done
   - `index_add` each new file

5. **Push to Build when complete**
   - All tasks `[x]`?
   - All acceptance criteria met?
   - Push: TUI `p` → "Build"

---

## This Mode's Features

### Impact Levels

Impact levels indicate priority/severity of proposals:

| Level | Meaning |
|-------|---------|
| **critical** | Security, data loss, blocking |
| **high** | Major features, significant bugs |
| **medium** | Nice-to-have, minor improvements |
| **low** | Polish, cosmetic, future ideas |

When creating a Roadmap item:
- TUI prompts for impact after name
- Choose the level that matches the item's importance

### Change Types

Change types categorize what kind of work it is:

| Type | Meaning |
|------|---------|
| **feature** | New functionality |
| **bugfix** | Bug correction |
| **refactor** | Code restructuring |
| **documentation** | Docs and guides |
| **security** | Security changes |

When creating a Roadmap item:
- TUI prompts for change type after impact
- Choose the type that matches the work

### Severity & Type Display

In the Roadmap TUI view, topics display as:

```
name │ type │ [SEVERITY]

Examples:
auth │ feature │ [HIGH]
login-bug │ bugfix │ [CRITICAL]
```

---

## How to Create Proposals

### Via TUI

1. Select **Roadmap** area
2. Press `n` for new topic
3. Enter: **name** (e.g., "user-auth")
4. Enter: **impact** (e.g., "high")
5. Enter: **change_type** (e.g., "feature")

### Via Spec File

When the topic is created, edit `spec.md`:

```yaml
---
title: User Authentication
impact: high
change_type: feature
status: proposed
created: 2026-03-28
---

## Problem Statement

Users need secure authentication to access the app.

## Requirements

- [ ] Email/password login
- [ ] Session management

## Acceptance Criteria

- [ ] User can login with valid credentials
```

---

## How to Build

### Step 1: Move to Working

TUI: Select topic in Roadmap → Press `p` → Enter "Working"

### Step 2: Create Tasks

Edit `tasks.md`:

```markdown
## Tasks

### Phase 1: Foundation
- [ ] Create user table schema
- [ ] Implement password hashing

### Phase 2: Login
- [-] Create login endpoint
- [ ] Add session middleware

### Phase 3: Testing
- [ ] Write login tests
```

### Step 3: Implement

For each task:
1. Mark `[-]` (in progress)
2. Write the code
3. `index_add --topic <name> --path <file>` for new files
4. Mark `[x]` (done)
5. Move to next task

### Step 4: Verify

Before pushing to Build:
- All tasks `[x]`?
- Tests pass?
- Acceptance criteria in spec met?

### Step 5: Push to Build

TUI: Select topic in Working → Press `p` → Enter "Build"

The completion date is added automatically.

---

## How to Find Reusable Code

When your work depends on code from another topic:

### 1. Search the index

```bash
index_find <feature-name>
```

### 2. Check exports

```bash
index_exports <topic>
```

### 3. Link to your topic

```bash
index_add --topic <your-topic> --path <file>
```

### 4. Reference it

```python
from auth import login_user  # ref:index:user-auth:login_user
```

---

## When Stuck

### Don't know where code is?

```bash
index_list <topic>           # See linked files
code_parse <file> --pattern <name>  # Parse file
```

### Don't understand requirements?

```bash
topics_show <topic> --from <area>  # Read the spec
```

### Need to understand dependencies?

```bash
index_depends <topic>  # What uses this?
```

---

## Mode-Specific Rules

1. **Proposals go to Roadmap** - New ideas start in Roadmap, not Working
2. **All tasks done before Build** - Don't push to Build with incomplete tasks
3. **Impact guides priority** - Higher impact items should be worked first
4. **Change type categorizes work** - Helps understand what kind of effort is needed
5. **Progress via tasks** - Keep `tasks.md` accurate - it's how progress is measured

---

## Remember

**The spec is the contract.** Your job:

1. Understand what the spec requires
2. Break it into tasks
3. Implement tasks, marking progress
4. Verify against acceptance criteria
5. Push to Build when done

**Work flows forward:** Roadmap → Working → Build

**Track progress:** Mark tasks [-] and [x] as you work

**Link code:** Every file you create should be added to the index
