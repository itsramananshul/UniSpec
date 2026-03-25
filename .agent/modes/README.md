# UniSpec Modes

Modes define different agent configurations. Each mode has its own workflows and capabilities.

## Creating a Mode

1. Create directory: `.agent/modes/<mode_name>/`
2. Add `mode.toml` with metadata
3. Add `skill.md` with agent persona
4. Add `workflows/*.md` files
5. Add `areas/` with staging/, working/, build/ directories containing area.md

## Global Modes

Modes in `~/.config/unispec/modes/` are available to all projects.

## Area Templates

Place area templates in `.agent/areas/<area_name>/area.md`. The init command will copy these to spec/<area_name>/area.md.
