# Getting Started with OpenSDD

Welcome! This guide will help you go from zero to spec-driven development hero in 10 minutes.

## What You'll Build

By the end of this guide, you'll have:
- A working OpenSDD project
- Your first spec
- A clear understanding of the workflow

## Step 1: Installation

### From Source

```bash
git clone https://github.com/yourname/osdd.git
cd osdd
cargo install --path .
```

### From Binary (when published)

```bash
# macOS
brew install osdd

# Linux
curl -fsSL https://install.osdd.dev | bash

# Windows (PowerShell)
iwr https://install.osdd.dev | iex
```

Verify installation:

```bash
osdd --version
```

## Step 2: Initialize Your Project

```bash
mkdir my-awesome-project
cd my-awesome-project
osdd init
```

You'll see:

```
   ██████╗ ██████╗ ███████╗███╗   ██╗
  ██╔═══██╗██╔══██╗██╔════╝████╗  ██║
  ██║   ██║██████╔╝█████╗  ██╔██╗ ██║
  ██║   ██║██╔═══╝ ██╔══╝  ██║╚██╗██║
  ╚██████╔╝██║     ███████╗██║ ╚████║
   ╚═════╝ ╚═╝     ╚══════╝╚═╝  ╚═══╝
                                             
OpenSDD initialized with Simple Mode!
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
    └── skill.md
```

## Step 3: Launch the TUI

```bash
osdd
```

You should see:

```
┌─────────────────────────────────────────────┐
│ Open SDD v0.5.0  | Area: Working | Topics: 0 │
├─────────────────────────────────────────────┤
│                                             │
│     ⠀⠀⠀⠀⠀⣴⠶⠶⣒⣛⣛⣛...             │
│     ⠀⠀🦫                                │
│                                             │
├─────────────────────────────────────────────┤
│ (No topics yet)                             │
├─────────────────────────────────────────────┤
│ 🡙 Move | 🡘 Navigate | ↵ Open | n: New | q │
└─────────────────────────────────────────────┘
```

Congratulations! You're running OpenSDD!

## Step 4: Create Your First Spec

Press `n` to create a new topic. Enter:

```
User Login
```

Select the Staging area when prompted.

Now you have:
```
spec/
└── Staging/
    └── user-login/
        ├── spec.md      ← Your spec goes here
        └── tasks.md     ← Your tasks go here
```

## Step 5: Write the Spec

Open `spec/Staging/user-login/spec.md`:

```markdown
# User Login

## Problem Statement

Users need a secure way to log into our application. Currently, 
there's no authentication, so anyone can access anything.

## User Stories

- As a **new user**, I want to **register with email and password** 
  so that I can have a personal account.

- As a **returning user**, I want to **log in with email and password** 
  so that I can access my account.

- As a **forgetful user**, I want to **reset my password via email** 
  so that I can regain access if I forget it.

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

1. User can register with valid email and password (8+ chars)
2. User receives confirmation email on registration
3. User can login with registered email/password
4. Invalid login shows error, not what's wrong
5. User can request password reset
6. Password reset link expires after 1 hour
7. Session expires after 24 hours of inactivity
8. Logout invalidates session immediately

## Out of Scope

- Social login (Google, GitHub, etc.)
- Two-factor authentication
- SSO integration
- Password strength requirements (beyond length)

## Technical Notes

- Use bcrypt for password hashing
- JWT tokens for session management
- Email via existing email service (SendGrid, etc.)
```

## Step 6: Create Tasks

Open `spec/Staging/user-login/tasks.md`:

```markdown
# Tasks - User Login

## Backend
- [ ] Create User model with email, password_hash
- [ ] Add registration endpoint POST /api/auth/register
- [ ] Add login endpoint POST /api/auth/login
- [ ] Add logout endpoint POST /api/auth/logout
- [ ] Add password reset request POST /api/auth/reset-request
- [ ] Add password reset confirm POST /api/auth/reset-confirm
- [ ] Implement JWT token generation/validation
- [ ] Add middleware for protected routes

## Frontend
- [ ] Create registration form
- [ ] Create login form
- [ ] Create password reset request form
- [ ] Create password reset confirm form
- [ ] Add "Remember me" checkbox
- [ ] Add loading states
- [ ] Add error handling

## Database
- [ ] Create users table
- [ ] Create sessions table
- [ ] Create password_resets table

## Testing
- [ ] Unit tests for auth functions
- [ ] Integration tests for endpoints
- [ ] E2E tests for user flows

## Security
- [ ] Rate limiting on login
- [ ] Input validation
- [ ] SQL injection prevention
- [ ] XSS prevention
```

## Step 7: Build It

When your spec is ready, push to Working:

```bash
osdd topic push "User Login" Working
```

Implement your tasks, checking them off as you go.

## Step 8: Ship It

When complete:

```bash
osdd topic push "User Login" Build
```

Review, deploy, and celebrate! 🎉

## What's Next?

- Explore [CLI commands](../README.md#commands)
- Create [custom modes](../docs/simple-mode/)
- Set up [AI integration](../README.md#integrating-with-ai-agents)
- Configure [connectors](../README.md#connectors)

## Tips

### Start Small
Don't try to spec everything at once. Start with one feature.

### Be Specific
"User can log in" is not a spec. "User enters email/password and sees dashboard" is.

### Update Frequently
As you learn, update specs. They're not carved in stone.

### Link Everything
Connect your code to specs. It helps everyone understand the why.

## Troubleshooting

### "No spec folder found"
Run `osdd init` first.

### "Topic already exists"
Choose a different name or remove the existing one.

### TUI looks weird
Make sure your terminal supports Unicode and has a decent font.

## Need Help?

- Check the [FAQ](./FAQ.md)
- Open an issue on GitHub
- Ask in the community

---

Now you know the basics! Go forth and spec. 🦫

*"The best code is code that doesn't need to be written because the spec was so good."* - Patty
