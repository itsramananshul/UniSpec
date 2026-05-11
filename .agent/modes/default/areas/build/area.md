---
area: Build
short: Verified, production-ready specs and code. Treated as immutable.
---

# Build

## Purpose

Build is the final destination for topics that have passed Testing. The mode marks `Build` as a protected area in `mode.toml`, so topics here are not edited in place.

A topic in Build represents:
- A spec whose every `REQ-*` was verified against code at the time of push.
- A task file with every implementation and test task marked `- [x]`.
- A `notes_add` entry summarizing the final test/verification run.

## Guidelines

- Treat Build as read-only. Do not edit `topic.md`, `<topic>_spec.md`, `<topic>_task.md`, or any linked source file as part of "fixing the build".
- To revise a Build topic, pull it back into Working: `topics_pull { topic, source_area: "Build" }`. Make changes there, then walk it through Testing again.
- Do not run `topics_delete` against Build topics — they are the project's shipped history. If you absolutely must remove one, do so via the CLI after disabling the area's protection in `mode.toml`.
