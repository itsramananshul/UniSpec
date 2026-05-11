<!--
Template for <topic>_spec.md, read via:
  read_asset { topic: "templates", asset_type: "spec" }

Replace every <…> placeholder. Do not commit literal `<TopicName>`, `<requirement statement>`, etc. — those exist only to be replaced. The MCP server prepends frontmatter (title, short, created, author, status) automatically, so omit the `---` block when you pass this body as `spec_content` to `spec_add`.

Required sections: Overview, Purpose, Requirements (≥ 1 row), Examples (≥ 1), Out of Scope. Drop Data Model only if the feature has no persistent state.
-->

# Design: <TopicName>

## Overview

<2 sentences. What is this feature, plainly?>

## Purpose

<Why does it exist? What problem does it solve? Who benefits? Concrete, not marketing.>

## In-Depth Details

<Technical explanation. How does it work? What components participate? What are the key design decisions, and what was rejected?>

## Requirements

<Numbered REQ-* rows. Use SHALL for must-haves, SHOULD for desirable. Each row must be testable — if you can't write an acceptance check for it, rewrite the requirement.>

| ID | Requirement | Priority |
|----|-------------|----------|
| REQ-001 | <The service SHALL …> | Must |
| REQ-002 | <The service SHOULD …> | Should |

## Examples

### Example 1: <descriptive name>

- **Input**: <concrete data, not a placeholder>
- **Output**: <concrete data>
- **Flow**:
  1. <step>
  2. <step>

## Data Model

### Entities

<Table per entity if the feature stores or transmits structured data. Omit if not applicable and say so explicitly.>

| Entity | Fields | Description |
|--------|--------|-------------|
| <Name> | <field>: <type> | <description> |

### Relationships

<ASCII or text. Example: User --1:N--> Session. Omit if there's only one entity.>

## Out of Scope

<List what this spec deliberately does NOT cover. This is where you put adjacent features that belong in a different spec.>

- <out-of-scope item 1>
- <out-of-scope item 2>
