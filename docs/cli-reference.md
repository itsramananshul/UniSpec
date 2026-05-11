# CLI Reference

Complete reference for all unispec CLI commands.

## topic

### topic add
Create a new topic in an area.

    unispec topic add <name> --short <description> --content <body> [--area <area>]

- area defaults to the value in .agent/config.toml, or Staging if not set
- short is a one-line description shown in the TUI
- content must be at least 20 characters

### topic list
List all topics in an area.

    unispec topic list [--area <area>]

- area defaults to config, or Staging

### topic push
Move a topic from one area to another.

    unispec topic push <name> --area <target> --from <source>

- both --area and --from default to config area if not provided
- topics in Staging and Fixing must be queued before pushing (see queue add)

### topic pull
Pull a topic back from another area.

    unispec topic pull <name> [--area <area>]

### topic remove
Delete a topic from an area.

    unispec topic remove <name> [--area <area>] [--force]

## spec

### spec add
Create a spec and task file for a topic.

    unispec spec add --topic <name> --short <description> --spec-content <body> --task-content <tasks> [--area <area>]

- area defaults to config area
- task-content supports markdown bullet lists (- [ ] task)
- spec-content and task-content must be at least 11 characters

### spec show
Show the master spec file.

    unispec spec show

## queue

### queue add
Add a topic to an area's readiness queue. Required before pushing from Staging or Fixing.

    unispec queue add <topic> [--area <area>] [--position <n>]

- area defaults to config area
- position defaults to -1 (append to end)

## index

### index add
Link a file to a topic.

    unispec index add --topic <name> --path <file> [--link-type <type>]

### index list
List all indexed files for a topic.

    unispec index list --topic <name>

## mcp

Start the MCP server for AI editor integration.

    unispec mcp [--path <project-dir>]

The server speaks JSON-RPC over stdio. Configure your editor to run this command as an MCP server pointed at your project directory.

## TUI

Launch the interactive terminal UI.

    unispec

### Keybindings

    Arrow keys   Navigate
    →            Enter an area or topic
    ←            Go back
    Enter        Open topic file in editor ($EDITOR, nano, or vi)
    n            Create new topic
    r            Remove topic
    p            Push topic to next area
    f            Find topic
    q            Add topic to queue (when inside an area)
    q            Quit (when at the area selection screen)
    \            Toggle Paddy the platypus

## Pipeline

The full spec-driven workflow:

    Staging → Working → Testing → Fixing → Build

Topics must be queued before leaving Staging or Fixing:

    unispec queue add <topic>
    unispec topic push <topic> --area Working --from Staging

Working, Testing, and Fixing can be pushed freely without queuing.
