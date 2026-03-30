# Pipeline Mode - Development Workflow

This mode implements a **5-stage development pipeline** for structured, methodical software development.

---

## The Pipeline

```
Roadmap → Staging → Working → Testing → Build
   ↓          ↓         ↓          ↓        ↓
 plans     specs     build      verify   ship!
```

| Stage | Purpose | Agent Can | Agent Cannot |
|-------|---------|-----------|--------------|
| **Roadmap** | High-level proposals and plans | Create plans | Build code |
| **Staging** | Detailed spec writing | Refine specs with nested topics | Build code |
| **Working** | Implementation | Build code | Modify specs |
| **Testing** | Verification | Run tests | Modify specs or build |
| **Build** | Shipped | Nothing (done) | - |

---

## Stage 1: Roadmap (Plans)

**Purpose:** Capture ideas, proposals, and high-level plans.

### What Goes Here
- Feature ideas with brief descriptions
- Problem statements
- High-level impact assessments
- Simple plans that need elaboration

### Creating a Plan

1. Select **Roadmap** area in TUI
2. Press `n` for new topic
3. Enter name (e.g., "user-authentication")
4. This creates `plan.md` only

### Plan File Format

```markdown
---
title: User Authentication
impact: high
change_type: feature
status: proposed
created: 2026-03-28
---

## Problem Statement

Users need secure authentication to access the app.

## High-Level Approach

- Use JWT tokens for session management
- OAuth2 for third-party login
- Password hashing with bcrypt

## Acceptance Criteria (Draft)

- [ ] User can register
- [ ] User can login
- [ ] Session persists across requests
```

### Using /plan Command

The `/plan` command helps you create a comprehensive plan:

```
/plan <topic-name>
```

This command will:
1. Ask clarifying questions about the feature
2. Help define the problem statement
3. Outline high-level approach
4. Draft initial acceptance criteria
5. Suggest impact level

### Moving to Staging

When you have a clear plan ready for detailed specification:

TUI: Select topic → Press `p` → Enter "Staging"

---

## Stage 2: Staging (Detailed Specs)

**Purpose:** Write comprehensive, detailed specifications with nested topics.

### What Goes Here
- Full feature specifications
- Detailed requirements
- Nested sub-specs for complex features
- Implementation notes
- API designs
- Data models

### Creating a Detailed Spec

1. Select **Staging** area in TUI
2. Find your topic (pushed from Roadmap)
3. Edit `spec.md` to add full details

### Spec File Format

```markdown
---
title: User Authentication
impact: high
change_type: feature
status: in_progress
created: 2026-03-28
modified: 2026-03-28
---

## Problem Statement

Users need secure authentication to access the app.

## Requirements

### 2.1 User Registration

#### 2.1.1 Email Validation
- Must be valid email format
- Must be unique in system
- Case-insensitive comparison

#### 2.1.2 Password Requirements
- Minimum 8 characters
- At least one uppercase letter
- At least one number
- At least one special character

### 2.2 User Login

#### 2.2.1 JWT Token Structure
```json
{
  "sub": "user_id",
  "email": "user@example.com",
  "iat": 1234567890,
  "exp": 1234654290
}
```

#### 2.2.2 Token Expiration
- Access token: 15 minutes
- Refresh token: 7 days

## Acceptance Criteria

- [x] User can register with valid email
- [x] User can register with valid password
- [ ] User can login with valid credentials
- [ ] Invalid credentials return 401
- [ ] Expired token returns 401

## Nested Topics

### auth/database-schema
PostgreSQL table for users

### auth/api-endpoints
REST API specification

### auth/token-service
JWT generation and validation
```

### Using /spec Command

The `/spec` command helps you build comprehensive specs:

```
/spec <topic-name>
```

This command will:
1. Review current spec
2. Identify missing sections
3. Suggest nested topics for complex areas
4. Ask detailed questions to fill gaps
5. Help organize nested specs
6. Ensure all acceptance criteria are clear

### Creating Nested Topics

For complex features, break them into nested topics:

```
topics_new auth/database-schema --from Staging
topics_new auth/api-endpoints --from Staging
topics_new auth/token-service --from Staging
```

Each nested topic has its own `spec.md` with detailed implementation specs.

### Moving to Working

When spec is complete (all nested specs done, all details filled):

TUI: Select topic → Press `p` → Enter "Working"

---

## Stage 3: Working (Build)

**Purpose:** Implement code according to the specification.

### What Goes Here
- Actual code implementation
- Task tracking in `tasks.md`
- Index updates for new files

### Creating Tasks

Edit `tasks.md`:

```markdown
## Tasks

### Phase 1: Database
- [ ] Create users table migration
- [ ] Add password hash column
- [ ] Create User model

### Phase 2: Registration
- [ ] Create registration endpoint
- [ ] Add email validation
- [ ] Add password validation
- [ ] Write tests

### Phase 3: Login
- [ ] Create login endpoint
- [ ] Implement JWT generation
- [ ] Add token validation middleware
- [ ] Write tests

### Phase 4: Integration
- [ ] Connect registration to database
- [ ] Connect login to token service
- [ ] End-to-end tests
```

### Using /build Command

The `/build` command starts the build process:

```
/build <topic-name>
```

This command will:
1. Read the spec
2. Parse tasks from `tasks.md`
3. Present current task status
4. Guide implementation step by step
5. Mark tasks `[x]` as completed
6. Update index with new files

### Implementation Rules

1. **DO NOT modify specs** - If you find issues, push back to Staging
2. **Follow specs exactly** - The spec is the contract
3. **Mark progress** - Use `[-]` for in-progress, `[x]` for done
4. **Index files** - `index_add --topic <name> --path <file>` for each new file

### When Issues Arise

If implementation reveals spec problems:

1. **Stop building**
2. **Document the issue** in a comment
3. **Push back to Staging** - TUI: `p` → "Staging"
4. **Note in spec** what needs clarification

### Moving to Testing

When all tasks are `[x]` and you believe the implementation is complete:

TUI: Select topic → Press `p` → Enter "Testing"

---

## Stage 4: Testing (Verify)

**Purpose:** Verify the implementation against specs through automated and manual testing.

### What Happens Here
- Run test suites
- Manual verification
- Acceptance criteria review
- Debugging if issues found

### Using /verify Command

The `/verify` command runs verification:

```
/verify <topic-name>
```

This command will:
1. Check all acceptance criteria
2. Run unit tests
3. Run integration tests
4. Verify against spec requirements
5. Report status

### Verification Output Format

```
## Verification Report: user-auth

### Acceptance Criteria Check
- [x] User can register with valid email
- [x] User can register with valid password  
- [x] User can login with valid credentials
- [x] Invalid credentials return 401
- [x] Expired token returns 401

### Test Results
- Unit tests: 12 passed, 0 failed
- Integration tests: 5 passed, 0 failed

### Status: PASSED ✓
```

### If Tests Fail

**Debugging Process:**

1. **Analysis** - Understand what's failing
2. **Document** - Note the failure in `debug.md`
3. **Investigate** - Find root cause
4. **Fix** - Implement the fix
5. **Re-test** - Run tests again

Debugging happens IN the Testing area. You do NOT push back to Working for fixes.

### Debug File Format

```markdown
## Debug Log: user-auth

### Test Failure 1 (2026-03-28)
**Test:** UserLoginWithExpiredToken
**Error:** Expected 401, got 200

**Root Cause:** Token expiration not checked in middleware

**Fix Applied:** Added expiration check in validate_token()

**Re-test:** Passed ✓

---

### Test Failure 2 (2026-03-28)
**Test:** UserRegistrationPasswordStrength
**Error:** Password "password123!" accepted (should fail)

**Root Cause:** Special character regex too permissive

**Fix Applied:** Updated regex to require specific special chars

**Re-test:** Passed ✓
```

### Moving to Build

When verification passes (all tests green, all criteria met):

TUI: Select topic → Press `p` → Enter "Build"

---

## Stage 5: Build (Ship)

**Purpose:** Final state - code is complete, tested, and ready.

### What Goes Here
- Completed, shippable code
- Topics can be pulled back for fixes (automatic checkout)

### Automatic Checkout Mechanism

**Purpose:** Prevent merge conflicts by ensuring only one agent works on a topic at a time.

**How it works:**
1. When you **pull** from Build → Automatic checkout (moves to Working)
2. When you **push** to Build → Automatic checkin (stays in Build, marks as shipped)
3. If another agent has it checked out → You cannot pull/push
4. **Checkout only works from Build** - you cannot checkout from Working or Roadmap

**TUI Display:**
- Build view shows: `✅ topic │ completed: DATE │ 🔒 checked out by USER`
- Working view shows: `topic ⏳ [====    ] - (0/5) 🔒(by USER)`
- Topics checked out by others are **visually marked** but still visible
- **Push (`p`)** is blocked for checked-out topics with message

**Checkout Metadata:**

```yaml
---
title: user-auth
status: shipped
checked_out: agent-123
checked_out_at: 2026-03-28T14:30:00Z
completed: 2026-03-28
modified: 2026-03-28T16:45:00Z
---
```

### Checkout Workflow

```
Build (shipped) → Pull → Working (automatic checkout, moved from Build)
    ↓
[Make fixes]
    ↓
Testing → Push → Build (automatic checkin, stays in Build)
```

### Why No Merge Conflicts?

This is **not git**. The workflow prevents conflicts:

1. **In Build?** - Done. Don't touch unless you need to fix.
2. **Pull from Build?** - Automatic checkout + topic MOVED to Working. Only you can modify.
3. **Push to Build?** - Automatic checkin. Topic stays in Build. Now it's clean and shippable again.
4. **From Working to Testing?** - Topic MOVED. No longer in Working.
5. **From Testing to Build?** - Topic MOVED. No longer in Testing.

**Key insight:** Topics are MOVED (not copied) as they flow through the pipeline. Only Build items stay put (they're shipped).

---

## Nested Topics & Ancestor Pull

**Purpose:** Hierarchical topic names like `UI/Dashboard/Auth/LDAP` automatically pull parent topics.

### How It Works

When you pull a nested topic like `UI/Dashboard/Auth/LDAP`:

```
Pull: UI/Dashboard/Auth/LDAP
Result:
  - Pulls: UI/Dashboard/Auth/LDAP
  - Also pulls: UI/Dashboard/Auth (if exists)
  - Also pulls: UI/Dashboard (if exists)
  - Also pulls: UI (if exists)
```

### Topic Hierarchy

```
UI/
├── Dashboard/
│   └── Auth/
│       └── LDAP/
│           └── spec.md
```

### Why Ancestor Pull?

If you need to work on `Auth/LDAP`, you likely need context from `Auth` and `Dashboard`. Ancestor pull ensures:

1. **Context** - You have the parent specs
2. **Consistency** - Related work stays together
3. **Automation** - No need to manually pull each level

### What Gets Pulled

| Topic | Status |
|-------|--------|
| `UI` | ✅ Pulled if exists |
| `UI/Dashboard` | ✅ Pulled if exists |
| `UI/Dashboard/Auth` | ✅ Pulled if exists |
| `UI/Dashboard/Auth/LDAP` | ✅ Always pulled |

Each ancestor is checked:
- If it **exists** → Pulled
- If it **doesn't exist** → Skipped
- If it **checked out by others** → Skipped (with warning)

---

## Available Commands

### Topic Commands

| Command | Description |
|---------|-------------|
| `topics_list [area]` | List topics in area |
| `topics_new <name>` | Create new topic in current area |
| `topics_show <name>` | Show topic details |
| `topics_push <name> --to <area>` | Move topic to another area (removes from current) |
| `topics_pull <name> --from <area>` | Move topic from another area (removes from source) |

**Note:** Topics are MOVED, not copied. Build topics stay in Build (they're shipped).

### Workflow Commands (Slash Commands)

| Command | Description |
|---------|-------------|
| `/plan <topic>` | Create/refine a roadmap plan |
| `/spec <topic>` | Build comprehensive spec in Staging |
| `/build <topic>` | Start building in Working area |
| `/verify <topic>` | Verify implementation in Testing area |

### Index Commands

| Command | Description |
|---------|-------------|
| `index_add --topic <name> --path <file>` | Add file to topic's index |
| `index_list --topic <name>` | List indexed files |
| `index_find <pattern>` | Find code by pattern |
| `index_exports <topic>` | List topic's exports |

### Code Commands

| Command | Description |
|---------|-------------|
| `code_parse <file> --pattern <name>` | Parse code structure |
| `code_analysis <topic>` | Get code analysis for topic |

---

## Area-Specific Spec Files

| Area | Spec File | Purpose |
|------|-----------|---------|
| Roadmap | `plan.md` | High-level plans |
| Staging | `spec.md` | Detailed specifications |
| Working | `spec.md` + `tasks.md` | Spec + implementation tasks |
| Testing | `spec.md` + `debug.md` | Spec + debug log |
| Build | `spec.md` | Final spec |

---

## Example Workflow: User Authentication

### Step 1: Create Plan (Roadmap)

```
> /plan user-auth
Creating plan for user-auth...

Questions:
1. What problem does this solve?
2. Who are the users?
3. What auth method? (password, oauth, etc.)

[Agent answers questions, creates plan.md]
```

### Step 2: Detail Spec (Staging)

```
> /spec user-auth
Reviewing spec for user-auth...

Missing sections identified:
- [ ] Token refresh mechanism
- [ ] Password reset flow
- [ ] Nested spec: database schema

[Agent fills in details, creates nested topics]
```

### Step 3: Build (Working)

```
> /build user-auth
Starting build for user-auth...

Tasks from tasks.md:
- [ ] Create users table migration
- [-] Create registration endpoint
...

[Agent implements, marks tasks done]
```

### Step 4: Verify (Testing)

```
> /verify user-auth
Verifying user-auth...

Test Results:
- Unit: 12/12 passed
- Integration: 5/5 passed
- Manual: All criteria met

Status: PASSED ✓
```

### Step 5: Ship (Build)

```
> topics_push user-auth --to Build
Topic user-auth moved from Staging to Build.
Status: shipped
Completed: 2026-03-28
```

### Later: Need a Fix

```
> topics_pull user-auth --from Build
Topic user-auth moved from Build to Working.
Auto-checked out by this agent.

[Agent makes fix]

> topics_push user-auth --to Testing
Topic user-auth moved from Working to Testing.

> topics_push user-auth --to Build
Topic user-auth moved from Testing to Build.
Status: shipped
```

---

## Tips

1. **Start small in Roadmap** - Don't over-plan
2. **Detail in Staging** - Nested topics help organize complex specs
3. **Trust the spec in Working** - Don't modify, push back instead
4. **Debug in Testing** - Stay in Testing until passing
5. **Checkout sparingly** - Build is meant to be final

---

## Remember

**The pipeline ensures quality:**
- Plans are reviewed before specs
- Specs are detailed before building
- Code is built against specs
- Code is verified before shipping
- Checkouts prevent conflicts

**Flow forward, not backward.** Push issues back to Staging, don't modify in place.
