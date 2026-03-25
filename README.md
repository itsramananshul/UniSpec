# UniSpec - Spec-Driven Development That Doesn't Suck

Write specs. Build code. Ship software. Structured clarity for humans and clankers alike! No more cognitive debt. All in our favorite RustLang 🦀

## The Problem

You write code. Then you write more code. Then someone asks "wait, what are we building again?" and nobody remembers. Your AI models constantly hallucinate and go on a bender while your team has no idea how things work. But it just works... for now.

For us, we're creating a frontier infrastructure project and found that spec-driven development gave us some efficiency at waterfalling, but became a nightmare when debugging large and complex codebases. Using OpenSpec, BMAD, and SpecKit was starting to destroy our work.

## The Fix

UniSpec is a fully open source spec-driven development orchestrator that allows you to build your own spec-driven workflows that can work inside production environments. This allows you to create specs in a tree-like format so your code is fully referenced and documented.

This splits up the development process into 3 concepts:

- **Modes** – Custom built workflows for your IDE agents
- **Areas** – Specification workspaces designed for your objectives
- **Topics** – Defined subjects that can nest into trees of specifications



## Quick Start

```bash
# Install from source
git clone https://github.com/uwzis/unispec.git
cd unispec
cargo install unispec

# Or from Arch Linux AUR
yay -S unispec

# Initialize
mkdir project && cd project
unispec init

# Launch TUI
unispec
```

## Core Concepts

### Areas (Simple Mode)

| Area | Purpose |
|------|---------|
| **Staging** | Writing specs |
| **Building** | Writing code |
| **Ship** | Done. Ready to deploy. |

### Indexing (The Secret Sauce)

This is what makes UniSpec actually useful:

```bash
# Link code to a spec
unispec index add --topic "user-login" --path src/auth/login.rs
unispec index add --topic "user-login" --path tests/login_test.py

# Now AI knows which code implements which feature
```

When your agents query a spec, they see the actual code too. Not just words.

### Modes


Out of the box, UniSpec comes with Simple Mode. Here's how it works:

```
Spec → Build → Verify
```

That's it. Three areas. Clear boundaries. Magic optional.

You can make UniSpec yours by creating your own modes. Check the docs for features including:

- MCP tooling commands
- Area scripting
- New agent abilities

Custom workflows for different teams:

- `.agent/modes/simple/` - Default (spec → build → ship)
- `.agent/modes/custom/` - Create your own workflow!

## Commands at a Glance

```bash
unispec init                          # Set up project
unispec                               # Launch TUI
unispec topic add "Feature"           # Create spec
unispec topic push "Feature" Ship    # Move to deploy
unispec index add --topic "feature" --path src/main.rs  # Link code
unispec topic progress                 # See status
```

## Editor Integrations

UniSpec plays nice with 24 AI editors. When you run `unispec init`, it can set up your editor:

```bash
unispec init --cursor --cline --windsurf
```

Supported editors:

| Editor | CLI Flag | Editor | CLI Flag |
|--------|----------|--------|----------|
| Amazon Q | `--amazon-q` | Kilo Code | `--kilo-code` |
| Antigravity | `--antigravity` | Kiro | `--kiro` |
| Augment | `--auggie` | OpenCode | `--opencode` |
| Claude Code | `--claude-code` | Pi | `--pi` |
| Cline | `--cline` | Qoder | `--qoder` |
| Codex | `--codex` | Qwen Code | `--qwen-code` |
| CodeBuddy | `--codebuddy` | RooCode | `--roo-code` |
| Continue | `--continue` | Windsurf | `--windsurf` |
| CoStrict | `--costrict` | TRAE | `--trae` |
| Crush | `--crush` | Cursor | `--cursor` |
| Factory | `--factory` | Gemini CLI | `--gemini-cli` |
| GitHub | `--github` | iFlow | `--iflow` |

Or use `--all` to set up all of them.

**Bonus:** I use Zed - just copy the commands into the Rules. Highly recommend Zed!

## What Goes In A Spec?

1. What problem are we solving?
2. Who is this for?
3. What are we building? (be specific)
4. How do we know it's done? (acceptance criteria)
5. What's NOT included?

Then `tasks.md` breaks it into actionable chunks with notes about implementations and challenges.

## Meeting Paddy the Platypus

There's a platypus named **Paddy** in the TUI. He's here to be like your personal cheerleader for all you ADHD GenZ Tik-Tok glued addicts like myself. He is just a reminder that you can do it! Toggle him with `\` in the TUI.

He believes in you.

## What's Next?

- [Getting Started](docs/getting-started.md) - Full walkthrough
- [Commands Reference](docs/commands.md) - All CLI commands
- [Creating Modes](docs/modes.md) - Build custom workflows
- [MCP Integration](docs/mcp.md) - Connect AI agents
- [Indexing](docs/indexing.md) - Link code to specs

---

**Remember**: Code is what computers run. Specs are what humans understand. Write the spec first, work based off understanding.

— Paddy the Platypus 🦫
