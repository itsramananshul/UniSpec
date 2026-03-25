# Simple Mode - The Default Agent Mode

Simple Mode is OpenSDD's default mode for spec-driven development. It's designed to be straightforward: write specs, build code, ship software.

## Overview

Simple Mode follows a three-stage workflow:

```
Staging (Write) → Working (Build) → Build (Ship)
```

Each stage has a clear purpose and rules.

## Area Definitions

### Staging
**Purpose**: This is where specs are written and worked on before any code is written.

**Philosophy**: "Think before you type." Staging is for exploration, discussion, and specification. Nothing moves to Working until the spec is solid.

**What belongs here**:
- New feature ideas
- Problem statements
- User research
- Spec drafts
- Requirements gathering

**What doesn't belong here**:
- Code (yet)
- "Good enough" solutions
- Features without specs

### Working
**Purpose**: This is where specs are being built. This may be tested as well.

**Philosophy**: "Ship it when it's ready." Working is where the magic happens. Code gets written, tests get run, and features take shape.

**What belongs here**:
- Active implementation
- Code under development
- Running tests
- Bug fixes
- Integration work

**What doesn't belong here**:
- Unreviewed specs
- Speculative features
- Code that doesn't compile

### Build
**Purpose**: When spec is turned into code and is in a shippable state, this is where it goes.

**Philosophy**: "Ready to go." Build contains completed, tested, shippable code.

**What belongs here**:
- Completed features
- Shippable code
- Release-ready implementations
- Deployed artifacts

**What doesn't belong here**:
- Work in progress
- Known bugs
- Untested features

## Workflows

### osdd:spec - Creating a Spec

1. **Create the topic**
   ```bash
   osdd topic add "User Authentication" -a Staging
   ```

2. **Write the spec**
   - Fill out `spec.md` with:
     - Problem statement
     - User stories
     - Requirements
     - Acceptance criteria
   
3. **Create initial tasks**
   - Fill out `tasks.md` with:
     - Implementation steps
     - Testing requirements
     - Documentation needs

4. **Link relevant files**
   ```bash
   osdd index add --topic user-auth --path docs/architecture.md
   ```

5. **Review and refine**
   - Read the spec aloud (really)
   - Check: Can you explain it to someone in 2 minutes?
   - Check: Are acceptance criteria testable?

### osdd:build - Building a Feature

1. **Push to Working**
   ```bash
   osdd topic push "User Authentication" Working
   ```

2. **Implement**
   - Follow the spec
   - Check off tasks as you complete them
   - Update `tasks.md` with progress

3. **Test**
   - Run your connectors (tests, linters, etc.)
   - Verify against acceptance criteria

4. **Push to Build**
   ```bash
   osdd topic push "User Authentication" Build
   ```

### osdd:verify - Releasing

1. **Review in Build**
   ```bash
   osdd topic show user-authentication
   ```

2. **Check completion**
   - All acceptance criteria met?
   - All tasks checked?
   - Tests passing?

3. **Deploy**
   - Ship it!
   - Update stakeholders
   - Celebrate (Patty does)

## Mode Configuration

Simple Mode is defined in `.agent/modes/simple/`:

```
simple/
├── mode.toml          # Mode metadata
├── skill.md          # Agent persona
├── workflows/        # Workflow definitions
│   ├── osdd:spec.md
│   ├── osdd:build.md
│   └── osdd:verify.md
├── areas/            # Area templates
│   ├── staging/
│   ├── working/
│   └── build/
└── templates/         # Topic templates
    ├── specs.md
    └── tasks.md
```

### mode.toml

```toml
[mode]
name = "simple"
display_name = "Simple Mode"
version = "1.0.0"

[author]
name = "OpenSDD Team"

[mode.description]
short = "Default mode with Spec-Driven Development workflows"
long = """
Simple Mode provides a straightforward three-stage workflow:
1. Write specs in Staging
2. Build in Working
3. Ship from Build

Perfect for teams new to spec-driven development or
those who want a minimal, effective workflow.
"""

[areas]
default = ["Staging", "Working", "Build"]

[capabilities]
spec_writing = true
building = true
verification = true
connectors = true
custom_workflows = true
```

## Agent Persona (skill.md)

Simple Mode includes a default agent persona:

```markdown
# Simple Mode Agent Persona

You are a spec-first developer who believes:
- Good code starts with good specs
- Clarity before implementation
- Tests prove specs are met

Your approach:
1. Always read the spec first
2. Update tasks as you work
3. Link code to specs
4. Verify before shipping
```

## Customizing Simple Mode

Want to make Simple Mode your own? Here's how:

### Add Custom Areas

1. Create area directory:
   ```bash
   mkdir -p .agent/modes/simple/areas/review
   ```

2. Add area.md:
   ```markdown
   # Review
   
   ## Purpose
   
   Code review and QA before staging.
   
   ## Guidelines
   
   - All PRs must be reviewed
   - Security review required
   - Performance benchmarks met
   ```

3. Update mode.toml:
   ```toml
   [areas]
   default = ["Review", "Staging", "Working", "Build"]
   protected = ["Staging", "Working", "Build"]
   ```

### Create Custom Workflows

Add to `.agent/modes/simple/workflows/`:

```markdown
# osdd:security-review

1. Run security scanner
2. Review dependencies
3. Check for OWASP Top 10
4. Document security considerations
```

### Customize Templates

Edit `templates/specs.md` to match your team's style:

```markdown
# [Feature Name]

## Context
[Why are we doing this?]

## Requirements
### Functional
- [ ] ...

### Non-Functional
- Performance: < 100ms response
- Availability: 99.9%
- Security: No SQL injection

## API Design
[If applicable]

## Metrics
- Success: X% conversion
- Failure: < 1% error rate
```

## Connectors for Simple Mode

Recommended connectors:

```bash
# Testing
osdd connector new test "Run test suite" "pytest" "tests/" "-v"
osdd connector new test-coverage "Check coverage" "coverage" "report" "-m"

# Quality
osdd connector new lint "Run linter" "ruff" "check" "."
osdd connector new typecheck "Type check" "mypy" "src/"

# Build
osdd connector new build "Build project" "cargo" "build"
osdd connector new format "Format code" "cargo" "fmt"
```

## Best Practices

### Writing Good Specs

1. **Start with why**
   ```markdown
   ## Problem Statement
   
   Users cannot reset their passwords without contacting support.
   This causes 40 support tickets per week...
   ```

2. **Be specific about success**
   ```markdown
   ## Acceptance Criteria
   
   - [ ] User receives email within 30 seconds
   - [ ] Reset link expires after 1 hour
   - [ ] Link can only be used once
   ```

3. **Include examples**
   ```markdown
   ## User Flow
   
   1. User clicks "Forgot Password"
   2. Enter: user@example.com
   3. See: "Check your email"
   4. Email contains link to /reset-password?token=xxx
   ```

### Task Management

1. **Break it down**
   ```markdown
   - [ ] Create database migration
   - [ ] Add User model fields
   - [ ] Write password reset function
   - [ ] Add email template
   - [ ] Create email service
   - [ ] Wire up endpoints
   - [ ] Write tests
   ```

2. **Use consistent language**
   - Implementation: "Implement", "Add", "Create"
   - Testing: "Test", "Verify", "Check"
   - Documentation: "Document", "Update", "Add docs"

### Progress Tracking

```bash
# Check progress across areas
osdd topic progress -a Staging
osdd topic progress -a Working
osdd topic progress -a Build

# Full health check
osdd area health
```

## Example: Building User Authentication

### 1. Create Spec (Staging)

```bash
osdd topic add "User Authentication" -a Staging
```

**spec.md**:
```markdown
# User Authentication

## Problem Statement

Users need secure ways to log into our application.

## Requirements

### Must Have
- [ ] Email/password login
- [ ] Password reset via email
- [ ] Session management
- [ ] Logout functionality

### Should Have
- [ ] Remember me option
- [ ] Account lockout after failed attempts

## Acceptance Criteria

- [ ] User can register with email
- [ ] User can login with email/password
- [ ] User can reset password via email
- [ ] Sessions expire after 24 hours
- [ ] Logout invalidates session
```

### 2. Build It (Working)

```bash
osdd topic push "User Authentication" Working
```

**tasks.md**:
```markdown
# Tasks

## Backend
- [x] Create User model
- [x] Add password hashing
- [x] Implement login endpoint
- [ ] Add password reset endpoint
- [ ] Implement session management

## Frontend
- [ ] Login form
- [ ] Registration form
- [ ] Password reset form

## Testing
- [ ] Unit tests for auth functions
- [ ] Integration tests for endpoints

## Documentation
- [ ] API documentation
- [ ] User guide
```

### 3. Ship It (Build)

```bash
osdd topic push "User Authentication" Build
```

Review completion, deploy, and celebrate!

## Summary

Simple Mode is... simple:

| Stage | Question | Action |
|-------|----------|--------|
| Staging | "What are we building?" | Write spec |
| Working | "How do we build it?" | Implement |
| Build | "Is it done?" | Verify & ship |

Questions? Create an issue or ping Patty.

🦫
