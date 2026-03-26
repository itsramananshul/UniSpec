# Getting Started with UniSpec

This guide walks you from "what is this?" to shipping your first spec-driven feature.

## Prerequisites

- Rust (for building from source) or a pre-built binary
- A terminal (the TUI needs it)

## Installation

### From Source

```bash
git clone https://github.com/uwzis/unispec.git
cd unispec
cargo install --path .
```

### From AUR (Arch Linux)

```bash
paru -S unispec
```

Verify:

```bash
unispec --version
```

## Step 1: Initialize Your Project

```bash
mkdir my-project
cd my-project
unispec init
```

You'll see:

```
 ██╗   ██╗███╗   ██╗██╗    ███████╗██████╗ ███████╗ ██████╗
 ██║   ██║████╗  ██║██║    ██╔════╝██╔══██╗██╔════╝██╔════╝
 ██║   ██║██╔██╗ ██║██║    ███████╗██████╔╝█████╗  ██║
 ██║   ██║██║╚██╗██║██║    ╚════██║██╔═══╝ ██╔══╝  ██║
 ╚██████╔╝██║ ╚████║██║    ███████║██║     ███████╗╚██████╗
  ╚═════╝ ╚═╝  ╚═══╝╚═╝    ╚══════╝╚═╝     ╚══════╝ ╚═════╝

UniSpec initialized with Simple Mode! -- Meet Paddy the Pladdy
```

This creates:

```
my-project/
├── spec/
│   ├── Staging/
│   │   └── area.md
│   ├── Working/
│   │   └── area.md
│   └── Build/
│       └── area.md
└── .agent/
    ├── config.toml
    ├── modes/
    ├── workflows/
    └── skill.md
```

## Step 2: Launch the TUI

```bash
unispec
```

You'll see the TUI with Paddy the platypus (toggle him with `\`).

## Step 3: Create Your First Topic

Press `n` to create a new topic. Enter:

```
User Login
```

Select "Staging" when prompted.

Now you have:

```
spec/
└── Staging/
    └── user-login/
        ├── spec.md      ← Your spec goes here
        └── tasks.md     ← Your tasks go here
```

## Step 4: Write the Spec

Open `spec/Staging/user-login/spec.md`:

```markdown
# User Login

## Problem Statement

Users need a secure way to log into our application.

## User Stories

- As a **new user**, I want to **register with email and password** so I can have a personal account.
- As a **returning user**, I want to **log in with email and password** so I can access my account.
- As a **forgetful user**, I want to **reset my password via email** so I can regain access.

## Requirements

### Must Have
- [ ] Email/password registration
- [ ] Email/password login  
- [ ] Password reset via email
- [ ] Session management (24hr expiry)
- [ ] Logout functionality

### Should Have
- [ ] "Remember me" checkbox
- [ ] Account lockout after 5 failed attempts

## Acceptance Criteria

1. User can register with valid email/password (8+ chars)
2. User receives confirmation email on registration
3. User can login with registered credentials
4. Invalid login shows generic error
5. Password reset link expires after 1 hour
6. Session expires after 24 hours of inactivity

## Out of Scope

- Social login (Google, GitHub)
- Two-factor authentication
- SSO integration
```

## Step 5: Create Tasks

Open `spec/Staging/user-login/tasks.md`:

```markdown
# Tasks - User Login

## Backend
- [ ] Create User model
- [ ] Add registration endpoint
- [ ] Add login endpoint
- [ ] Add logout endpoint
- [ ] Add password reset flow
- [ ] Implement JWT tokens

## Frontend
- [ ] Registration form
- [ ] Login form
- [ ] Password reset forms

## Testing
- [ ] Unit tests for auth
- [ ] Integration tests
```

## Step 6: Move to Working

When your spec is solid:

```bash
unispec topic push "User Login" Working
```

Now implement. Check off tasks as you go.

## Step 7: Move to Build

When it's done and tested:

```bash
unispec topic push "User Login" Build
```

Deploy. Celebrate. You're done.

## Quick Reference

| Command | What it does |
|---------|-------------|
| `unispec` | Launch TUI |
| `unispec init` | Initialize project |
| `unispec topic add "Name"` | Create topic |
| `unispec topic push "Name" Area` | Move topic |
| `unispec topic list` | Show all topics |
| `unispec topic progress` | Show progress |

## TUI Keys

| Key | Action |
|-----|--------|
| `↑/↓` | Move between topics |
| `Enter` | Open topic |
| `n` | New topic |
| `p` | Push to area |
| `r` | Remove topic |
| `f` | Find links |
| `\` | Toggle platypus |
| `q` | Quit |

## Editor Integration

Want your AI editor to see your specs? Run:

```bash
unispec init --cursor --cline --windsurf
```

This creates workflow files in your editor's config folder. Now Claude/Cursor/Windsurf see your specs automatically.

## What's Next?

- [Commands Reference](commands.md) - Full CLI docs
- [Configuration Reference](config.md) - Config files, environment variables
- [Creating Modes](modes.md) - Build custom workflows
- [MCP Integration](mcp.md) - Connect AI agents
- [Indexing](indexing.md) - Link code to specs

---

*Write the spec first. Code second. Paddy believes in you.* 🦫
