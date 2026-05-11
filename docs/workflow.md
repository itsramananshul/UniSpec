# Workflow

UniSpec's default mode defines a five-area pipeline. Topics enter at one end and leave at the other; each area has a clear purpose, and two of the five are gated by a readiness queue.

```
Staging  →  Working  →  Testing  →  Fixing  →  Build
   ▲                                  │
   │                                  │
   └──── (pull back if needed) ◄──────┘
```

## The five areas

| Area | When a topic should be here | Gated by readiness queue? |
|------|-----------------------------|---------------------------|
| **Staging** | Spec is being written. No code in `src/` yet. | **Yes** — must be in `spec/Staging/queue.md` to push out |
| **Working** | Spec frozen, code is being implemented. | No |
| **Testing** | Build and test scripts are running against the implementation. | No |
| **Fixing** | A defect was found in Testing; the topic is being repaired before being re-tested. | **Yes** — must be in `spec/Fixing/queue.md` to push out |
| **Build** | Topic is shipped. Treated as immutable; do not edit in place. | n/a (no further push) |

Each area is a directory under `spec/`. Each contains a single per-area `area.md` (describing what that area means in your project) and zero or more topic subdirectories.

The set of areas and which ones are queue-gated come from `.agent/modes/default/mode.toml`:

```toml
[areas]
default = ["Staging", "Working", "Testing", "Fixing", "Build"]
protected = ["Build"]
default_area = "Staging"

[readiness]
queue_file = "queue.md"
required_for_push = true
areas = ["Staging", "Fixing"]
```

If you change the mode, those rules change with it. See [modes.md](modes.md) for custom mode authoring.

## What pushing does

`unispec topic push <topic> --area <target> --from <source>`:

1. Verifies the source area exists.
2. **Creates the target area on the fly** if it doesn't exist yet (writes a minimal `area.md` stub). This is a backstop in case `init` ran with fewer areas than the current mode declares.
3. Confirms the topic exists in the source area.
4. If the source area is queue-gated (`Staging` or `Fixing` by default), checks that the topic appears in `spec/<source>/queue.md`. Errors out with a clear message if not.
5. Copies every file from `spec/<source>/<topic>/` to `spec/<target>/<topic>/`. (No legacy `specs.md`/`tasks.md` duplicates are synthesised.)
6. Removes the source directory. **Push is a move, not a copy.**
7. If pushing into `Build` and the topic carries ownership metadata (someone has it "checked out" in `<topic>_spec.md` frontmatter), the auto-checkin step clears the metadata so Build artefacts are clean.

## The readiness queue

Each gated area carries a `queue.md` at the area root:

```
spec/Staging/queue.md     ← lists topics ready to push out of Staging
spec/Fixing/queue.md      ← lists topics ready to push out of Fixing
```

The file is plain markdown:

```markdown
# Task Queue

Ordered list of topics to work on:
- user-login
- payment-flow
```

Add to it three ways:

| Surface | Command |
|---------|---------|
| CLI | `unispec queue add <topic> [--area <area>] [--position <n>]` |
| MCP | `queue_add { "topic": "user-login" }` |
| TUI | Highlight the topic in a TopicList view and press `q` |

The queue is intentionally append-only by default; remove with `queue_remove` (MCP) or by editing `queue.md` directly. Reorder via the MCP `queue_reorder` tool.

### Why a queue gate?

Two reasons:

1. **Explicit readiness signal.** Anyone who has worked spec-first knows the failure mode: someone writes half a spec and immediately starts coding. The queue gate forces an explicit "I'm done with the spec, this is ready to implement" before push.
2. **AI-agent guardrail.** Without the gate, an MCP-driven agent could ping-pong a topic through every area in seconds. With the gate, an actor (human or agent) has to deliberately enqueue the topic before the next stage will accept it.

`Working`, `Testing`, and `Build` are NOT gated by default — once a topic is past the spec, transitions between implementation stages don't need an extra approval beat. You can change which areas are gated by editing `[readiness].areas` in your mode's `mode.toml`.

## Filenames in a topic directory

```
spec/<Area>/<topic>/
├── topic.md                  ← written by `topic add`
├── <topic-safe>_spec.md      ← written by `spec add`
└── <topic-safe>_task.md      ← written by `spec add`
```

`<topic-safe>` is `<topic>` with `/` and ` ` replaced by `-`. So `auth/login` becomes `auth-login_spec.md` inside `spec/<Area>/auth/login/`.

Nested topics are real directories. Pushing the parent does **not** push the children — each child is its own pipeline traversal.

## Backflow: `pull`

If a topic in `Build` is found to have a bug, pull it back:

```bash
unispec topic pull <topic> --area Build
```

The topic returns to the current default area (typically `Staging`). From there it can be re-spec'd, re-implemented, and re-pushed.

## End-to-end shape

```bash
# Spec
unispec topic add  <name> --short "..." --content "..."
unispec spec add   --topic <name> --short "..." \
                   --spec-content "..." --task-content "..."

# Implement
unispec queue add  <name>
unispec topic push <name> --area Working --from Staging
# (code in src/)

# Test
unispec topic push <name> --area Testing --from Working
# (run tests)

# If broken
unispec topic push <name> --area Fixing  --from Testing
# (fix in src/)
unispec queue add  <name> --area Fixing
unispec topic push <name> --area Build   --from Fixing
# else
unispec topic push <name> --area Build   --from Testing   # if no Fixing pass needed
```

The whole pipeline can also be driven via MCP — the tools mirror the CLI surface 1:1. See [mcp-tools-reference.md](mcp-tools-reference.md).

## See also

- [Areas](areas.md) — per-area conventions (what code belongs in Working vs. Fixing, etc.).
- [Quickstart](quickstart.md) — five-minute walkthrough with copy-pasteable commands.
- [Modes](modes.md) — how to customise the pipeline (different areas, different gates).
