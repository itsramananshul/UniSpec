# Connectors

A **connector** is a shell command you define in `.agent/config.toml`. UniSpec exposes each one as a dynamic MCP tool named `unispec_<connector-name>`. This is how you let an AI agent run your tests, your linter, your build, or any other project-specific tooling without giving it raw shell access.

## Connector schema

Connectors are TOML array-of-tables under `[[connector]]` in `.agent/config.toml`:

```toml
[[connector]]
name = "test"                            # required: identifier (lowercase, underscores)
description = "Run the test suite"       # required: shown in `connector_list`
command = "pytest"                        # required: binary to invoke
args = ["tests/", "-v"]                  # optional: default args (array of strings)
env = { RUST_BACKTRACE = "1" }           # optional: env vars passed to the child process
working_dir = "/abs/path/to/project"     # optional: cwd for the child (default: project root)
timeout = 120                            # optional: kill after N seconds (default: 60)
```

Field types match `src/agent/connector.rs` exactly — if the binary fails to load your config, double-check field names and types.

## Adding a connector

Three ways:

### CLI

```bash
unispec connector new test "Run test suite" "pytest" \
  --args "tests/ -v" \
  --timeout 120
```

| Flag | Maps to |
|------|---------|
| `--args "<space-separated>"` | `args` (split on spaces) |
| `--env "KEY=value,KEY2=value2"` | `env` (parsed into a table) |
| `--working-dir <path>` | `working_dir` |
| `--timeout <secs>` | `timeout` |

### Edit `.agent/config.toml` by hand

Append a `[[connector]]` block. The CLI re-reads the file on every invocation, so no restart is needed.

### `unispec connector mcp`

```bash
unispec connector mcp > claude_mcp.json
```

Prints a ready-to-paste `mcpServers` block where each connector is its own MCP server entry. Useful if you want each connector to appear under its own name in the editor's tool list rather than buried inside `unispec_<name>`.

## How a connector becomes an MCP tool

When the MCP server's `tools/call` dispatcher receives a tool name it doesn't recognise, it checks: does the name start with `unispec_`? If yes, the suffix (`name[8..]`) is treated as a connector name. The server then calls `crate::agent::connector::run_run(connector_name, &[])`, which:

1. Reads `.agent/config.toml`.
2. Finds the matching `[[connector]]` block by `name`.
3. Spawns `command` with `args`, `env`, `working_dir`, `timeout` as configured.
4. Captures combined stdout/stderr.
5. Returns `{ "success": true, "output": "<captured-output>" }` to the caller.

If the connector isn't found, you get `Err("Unknown tool: unispec_<name>")`.

## Listing, running, deleting

```bash
unispec connector list                 # show all configured connectors
unispec connector run test             # run by name (uses configured args + env)
unispec connector run test -- -k auth  # append extra args after --
unispec connector delete test
unispec connector edit test --description "Run the unit + integration test suite"
```

The MCP equivalents are `connector_list` (returns all configured connectors) and `connector_run` (which takes a `name` and an optional `args` array). For most cases, calling `unispec_<name>` directly from the agent is cleaner.

## Worked examples

### `pytest` for a Python project

```toml
[[connector]]
name = "test"
description = "Run pytest with verbose output"
command = "pytest"
args = ["tests/", "-v", "--tb=short"]
timeout = 300
```

Agent usage:

```json
{ "name": "unispec_test", "arguments": {} }
```

Returns the full pytest report.

### `cargo test` for a Rust project

```toml
[[connector]]
name = "test"
description = "Run cargo test on the workspace"
command = "cargo"
args = ["test", "--all-features", "--quiet"]
env = { RUST_BACKTRACE = "1" }
timeout = 600
```

Agent usage:

```json
{ "name": "unispec_test", "arguments": {} }
```

### Multiple stages

You can configure several connectors and let the agent choose. A typical Rust project might have:

```toml
[[connector]]
name = "fmt"
description = "Format the codebase"
command = "cargo"
args = ["fmt"]

[[connector]]
name = "clippy"
description = "Lint with clippy, deny all warnings"
command = "cargo"
args = ["clippy", "--all-targets", "--all-features", "--", "-D", "warnings"]

[[connector]]
name = "test"
description = "Run the test suite"
command = "cargo"
args = ["test"]
timeout = 600

[[connector]]
name = "build"
description = "Release build"
command = "cargo"
args = ["build", "--release"]
timeout = 900
```

The agent now has `unispec_fmt`, `unispec_clippy`, `unispec_test`, `unispec_build` as MCP tools. A typical "verify the build before pushing to Testing" workflow then becomes:

```
unispec_fmt    → cargo fmt
unispec_clippy → cargo clippy -- -D warnings
unispec_test   → cargo test
unispec_build  → cargo build --release
```

### Frontend toolchain

```toml
[[connector]]
name = "lint"
description = "Run ESLint on src/"
command = "npx"
args = ["eslint", "src/", "--max-warnings=0"]

[[connector]]
name = "test"
description = "Run Jest"
command = "npx"
args = ["jest", "--passWithNoTests"]
timeout = 600

[[connector]]
name = "typecheck"
description = "Run tsc --noEmit"
command = "npx"
args = ["tsc", "--noEmit"]
```

## Output / error semantics

- The connector's combined stdout + stderr are returned verbatim in the MCP response as a single `output` string.
- The connector's exit code is **not** propagated as a JSON-RPC error. A failing test run is "successful" from the MCP layer's perspective — the agent must read the `output` field to decide whether the run passed.
- If the command itself fails to spawn (binary not on `PATH`, etc.), `run_run` returns an error and the agent gets `Err`.

Agents that need a strict pass/fail signal should either parse the output text or run a wrapper that exits 0 on pass and prints a clear marker on failure.

## Timeouts

`timeout` is in seconds. The connector is killed if it exceeds it. Default is 60s, which is short for many test suites — set higher for `cargo test`, `pytest --cov`, large JS test suites, etc.

A killed connector still returns the partial output captured before the kill, plus an error message in the response. The agent should treat this as "did not complete".

## Working directory

If `working_dir` is set, the child process runs in that directory. Useful when your project root has multiple sub-projects:

```toml
[[connector]]
name = "test-backend"
description = "Run backend tests"
command = "pytest"
args = ["tests/", "-v"]
working_dir = "/abs/path/to/repo/backend"

[[connector]]
name = "test-frontend"
description = "Run frontend tests"
command = "npm"
args = ["test"]
working_dir = "/abs/path/to/repo/frontend"
```

If `working_dir` is omitted, the child runs in the project root (the dir containing `spec/` and `.agent/`).

## Environment variables

`env` is a TOML inline-table mapping `KEY = "value"`:

```toml
[[connector]]
name = "build-prod"
description = "Production build"
command = "cargo"
args = ["build", "--release"]
env = {
  RUST_LOG = "info",
  RUST_BACKTRACE = "1",
  TARGET = "x86_64-unknown-linux-musl"
}
```

These are merged into the child's inherited environment.

## Connector-specific MCP config (per-connector servers)

For editors that prefer a separate MCP server entry per tool (or for security review purposes), you can generate per-connector configs:

```bash
unispec connector mcp
```

Output is a JSON config that runs each connector as its own MCP "server" via `unispec connector run <name>`. The functional difference is minimal — the agent ends up with the same tools — but it allows fine-grained editor-side allowlisting.

## See also

- [MCP Tools Reference](mcp-tools-reference.md) — `connector_list`, `connector_run`, and the `unispec_<name>` dynamic dispatch.
- [Architecture](architecture.md) — where connector spawning lives (`src/agent/connector.rs`).
- `src/cli/mod.rs::ConnectorCommands` — every CLI flag documented above.
