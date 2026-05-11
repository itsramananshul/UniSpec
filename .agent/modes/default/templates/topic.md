<!--
Template for topic.md, read via:
  read_asset { topic: "templates", asset_type: "topic" }

Replace every <…> marker with real content. Do not commit `<TopicName>` etc. — those exist only to be replaced. The MCP server prepends frontmatter automatically, so omit the leading `---` block when you pass this body as the `content` argument to `topics_add`.

Required: Overview is the only section that must be non-empty. Sub-topics and Notes may be empty.
-->

# <TopicName>

## Overview

<2-4 sentences describing what this topic covers and why it exists. Keep it concrete; avoid marketing copy.>

## Specs

<List the spec files belonging to this topic. Each spec is created via spec_add and lives next to this file.>

- `<TopicName>_spec.md`: <one-line summary of what this spec covers>

## Sub-topics

<List nested topics. Each appears as `<TopicName>/<sub>` and has its own topic.md/spec/task. Remove this section if there are none.>

- <sub-topic name>

## Notes

<Anything that doesn't fit above: open questions, links to external context, decisions logged later. Free-form. Date entries when relevant.>
