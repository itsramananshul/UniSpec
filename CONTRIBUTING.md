# Contributing to UniSpec

Thanks for your interest in contributing. This document covers the practical ways to help.

## Active branch: `everything`

The current stable branch with all recent fixes is **`everything`**, not `main`. It bundles seven topic branches (PR1–PR7) that close the engine-side bugs, init gaps, MCP gaps, and CLI UX gaps surfaced by extensive end-to-end testing. See `CHANGELOG.md` for the per-version breakdown. New work should branch from `everything` until it's merged upstream into `main`.

Specifically, on `everything` (vs. `main`):
- Init creates all five areas (`Staging`, `Working`, `Testing`, `Fixing`, `Build`) and ships the default mode embedded in the binary — no system install required.
- `topics_push` is a real move (source removed) rather than a copy.
- `unispec_read_spec` and `read_asset` look up `<topic>_spec.md` / `<topic>_task.md` (the names `spec_add` actually writes).
- All six previously-unimplemented MCP handlers exist: `tasks_list`, `tasks_complete`, `tasks_incomplete`, `notes_read`, `notes_add`, `queue_reorder`.
- CLI gained `spec add` and `queue add` subcommands.
- `topic push` accepts `--area` and `--from` named flags (positional form no longer works).
- All `topic` subcommands default `--area` from `.agent/config.toml`, falling back to `Staging`.
- `auto_checkin` uses the same `<topic>_spec.md` convention as everything else.
- `topic push` auto-creates a missing target area on first push.
- TUI `Enter` opens topics in `$EDITOR` / `nano` / `vi` (no more `xdg-open` failure on WSL); `q` queues the highlighted topic when inside an area.

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
2. Branch from `everything` (or whichever branch carries the latest fixes): `git checkout everything && git checkout -b feat/<short-description>`.
3. If your change affects user-facing behavior, write or update a spec first via `unispec spec add` or the MCP `spec_add` tool.
4. Implement with tests where applicable. Cargo workspace lives at the repo root.
5. Run `cargo build`, then sanity-test in `/tmp/<scratch>` with `unispec init`.
6. Open a PR against `everything`.

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

- [ ] `cargo build` is clean.
- [ ] `cargo test` is green if your area has tests.
- [ ] CLI signature changes are reflected in `docs/cli-reference.md`, `docs/commands.md`, and `docs/quickstart.md` if relevant.
- [ ] New or renamed MCP tools land in `src/mcp/mod.rs::get_tools()` AND a matching `match` arm in `src/mcp/server.rs::call_tool`, and are documented in `docs/mcp-tools-reference.md`.
- [ ] New connectors get a worked example in `docs/connectors.md`.
- [ ] CHANGELOG.md updated under the unreleased / next-version section.

## Common end-to-end test pattern

```bash
cargo build
rm -rf /tmp/unispec-scratch && mkdir /tmp/unispec-scratch
cd /tmp/unispec-scratch
/path/to/unispec init
/path/to/unispec topic add demo --short "smoke" --content "$(printf 'A small smoke test topic.\nWill walk through the pipeline once.\n')"
/path/to/unispec spec add --topic demo --short demo --spec-content "Spec body." --task-content "- [ ] Do the thing"
/path/to/unispec queue add demo
/path/to/unispec topic push demo --area Working --from Staging
```

If any of those fail on a clean `cargo build`, your change likely regressed something documented in `CHANGELOG.md` — check there before debugging from scratch.

## Code of conduct

Be respectful. Be helpful. Be kind to Paddy.

## Questions?

- Open an issue: <https://github.com/uwzis/UniSpec/issues>
- Browse discussions on the repo
- Email the maintainers
