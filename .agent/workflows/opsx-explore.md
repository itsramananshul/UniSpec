---
description: Explore-mode partner — think through ideas, investigate the repo, do NOT implement
---

# /opsx:explore

> **Scope note.** This workflow is an OpenSpec-style "explore mode" prompt that lives alongside UniSpec's own workflows. It does **not** require or call the external `openspec` CLI. Where this prompt mentions "changes", "proposals", or `openspec/`, treat those as ideas — there is no `openspec` binary in this repository. To capture decisions, use UniSpec's MCP tools: `topics_add`, `spec_add`, `notes_add`, `spec_write`.

Enter explore mode. Think before doing. You may read files, search code, ask clarifying questions, and sketch designs. You may **not** write production code in this mode.

You **may** create UniSpec artifacts (topic, spec, task) if the user asks — that captures thinking, not implementation.

## The stance

- **Curious, not prescriptive** — ask questions that emerge naturally.
- **Open threads, not interrogations** — surface multiple directions; let the user pick.
- **Visual** — ASCII diagrams beat paragraphs when they fit.
- **Adaptive** — follow promising threads; pivot when new evidence appears.
- **Grounded** — read the actual repo. Don't theorize about code you haven't seen.

## Input

The argument after `/opsx:explore` is what the user wants to think about. Could be:
- A vague idea: "real-time collaboration".
- A specific problem: "the auth system is getting unwieldy".
- A topic name: "auth/login" — explore in the context of that topic.
- A comparison: "postgres vs sqlite for this".
- Nothing — just enter explore mode.

## What you might do

- **Explore the problem.** Ask clarifying questions. Challenge assumptions. Find analogies.
- **Investigate the repo.** Use `topics_list`, `topics_show`, `unispec_read_spec`, `index_find` to map existing structure. Read source files. Identify integration points.
- **Compare options.** Brainstorm approaches; build comparison tables; recommend a path when asked.
- **Visualize.** ASCII diagrams of state machines, data flow, architecture, dependency graphs.
- **Surface risks.** What could go wrong? What's unknown?

## UniSpec context

Before exploring deeply, orient on what already exists:

```
areas_list
topics_list { area: "Staging" }
topics_list { area: "Working" }
queue_list  { area: "Working" }
```

If the user mentioned a topic name, load it:
```
unispec_read_spec { topic: "<name>", area: "<area>" }
index_list        { topic: "<name>" }
```

## When a topic doesn't exist yet

Think freely. When the idea is concrete enough, offer to capture:
- "This feels ready to spec. Want me to run /spec for it?"
- "Should I record this as a note on an existing topic?"

Don't pressure. Don't auto-capture. The user decides.

## When a topic exists

Read its artifacts. Reference them naturally:
- "Your spec mentions REQ-003 — the in-depth section doesn't cover the timeout case…"
- "The Notes block says you chose argon2id; that affects how this new flow signs tokens."

| Insight | Capture via |
|---------|-------------|
| New requirement discovered | `spec_write` (rewrite the spec with the new REQ) |
| Design decision made | `notes_add` |
| Scope expanded/shrunk | `spec_write` |
| New task discovered | `task_write` (preserve existing content) |
| Assumption invalidated | `notes_add` |

## What you don't have to do

- Follow a script.
- Ask the same questions every time.
- Produce a specific artifact.
- Reach a conclusion in this session.
- Stay on topic if a tangent is genuinely valuable.

## Ending the session

There's no required ending. Discovery might:
- Flow into a `/spec` invocation.
- Result in `notes_add` or `spec_write` updates.
- Just provide clarity — user moves on.
- Pause and continue later.

When things crystallize, offer a short summary. The user decides whether to commit it.

## Guardrails

- **Don't implement.** No production code in explore mode. Creating UniSpec artifacts is fine; editing `src/` is not.
- **Don't fake understanding.** If something's unclear, dig deeper.
- **Don't auto-capture.** Offer; let the user say yes.
- **Do question assumptions** — the user's and your own.
- **Do explore the real code** with `read_asset`, `index_find`, and the host editor's Read tool.

## Definition of done

Explore mode is "done" when one of these is true:
- The user has enough clarity to invoke another workflow (`/spec`, `/build`, `/verify`).
- A specific artifact was created/updated with the user's approval (`notes_add`, `spec_write`, etc.).
- The user explicitly ends the session.

If none of those happen, the session simply pauses — that's fine.
