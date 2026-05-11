# Contributing to UniSpec

Thanks for your interest in contributing. This document covers the practical ways to help.

## Ways to contribute

### 1. Create a mode

Modes are the heart of UniSpec — they define the area pipeline, workflow prompts, and templates an agent uses. Layout:

```
.agent/modes/<mode-name>/
├── mode.toml          # Required: metadata, areas, readiness rules, templates config
├── skill.md           # Required: agent persona
├── workflows/         # Optional: workflow prompts (spec.md, build.md, …)
├── areas/             # Optional: per-area templates
│   └── <area>/area.md
└── templates/         # Optional: global fallback templates
    ├── topic.md
    ├── spec.md
    ├── task.md
    └── area.md
```

See `.agent/modes/default/` for a complete working example and [docs/modes.md](docs/modes.md) for the full reference.

### 2. Improve documentation

- Fix typos and broken links.
- Add examples that map MCP tool calls to outcomes.
- Clarify ambiguous prompts in `.agent/workflows/` or `.agent/modes/<mode>/`.

When you touch a prompt that an agent will read, treat it as code: tool names must match `src/mcp/mod.rs::get_tools()`, and every workflow needs a clear definition of done.

### 3. Report bugs

When reporting bugs, include:
- OS and version.
- Steps to reproduce.
- Expected vs actual behavior.
- `unispec --version` output.

### 4. Feature requests

Describe:
- The problem you're solving.
- How you envision it working.
- Any existing workarounds.

### 5. Code contributions

1. Fork the repo.
2. Create a branch: `git checkout -b feat/<short-description>`.
3. If your change affects user-facing behavior, write or update a spec first (use `/spec` or the MCP tools directly).
4. Implement with tests where applicable.
5. Open a PR.

## Development setup

```bash
git clone https://github.com/uwzis/UniSpec.git
cd UniSpec

cargo build
cargo test
cargo run -- --help
```

## Code style

- Run `cargo fmt` before committing.
- Run `cargo clippy` — fix or justify every warning.
- Add comments only when the *why* isn't obvious from the code.
- Update specs and docs when behavior changes.

## Commit messages

Format:

```
type: short description

(optional) longer explanation

Fixes #123
```

Types: `feat:`, `fix:`, `docs:`, `refactor:`, `test:`, `chore:`.

## Pull request checklist

- [ ] Tests pass: `cargo test`.
- [ ] No clippy warnings: `cargo clippy --all-targets --all-features -- -D warnings`.
- [ ] Public API or CLI changes are reflected in `docs/`.
- [ ] If you added or renamed an MCP tool, `docs/mcp.md` and the agent prompts in `.agent/` are updated.
- [ ] If you added a connector, sample example is in `docs/configuration.md`.

## Code of conduct

Be respectful. Be helpful. Be kind to Paddy.

## Questions?

- Open an issue: <https://github.com/uwzis/UniSpec/issues>
- Browse discussions on the repo
- Email the maintainers
