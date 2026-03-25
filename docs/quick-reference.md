# OpenSDD Quick Reference

## Essential Commands

| Command | Description |
|---------|-------------|
| `osdd` | Launch TUI |
| `osdd init` | Initialize project |
| `osdd area list` | List areas |
| `osdd topic list` | List topics |
| `osdd topic progress` | Show progress |

## TUI Keys

| Key | Action |
|-----|--------|
| `↑/↓` | Move |
| `→/←` | Navigate |
| `Enter` | Open |
| `n` | New topic |
| `r` | Remove |
| `p` | Push |
| `f` | Find links |
| `q` | Quit |
| `\` | Toggle Patty |

## Area Operations

```bash
osdd area add <name>          # Add area
osdd area remove <name>       # Remove area
osdd area rename <old> <new>  # Rename
osdd area default <name>      # Set default
osdd area health              # Stats
```

## Topic Operations

```bash
osdd topic add <name> [-a Area]     # Create
osdd topic list [-a Area]          # List
osdd topic push <name> <area>     # Move
osdd topic pull <name> <area>     # Pull
osdd topic show <name>             # Details
osdd topic remove <name>           # Delete
```

## Index Operations

```bash
osdd index add --topic <t> --path <p>   # Link
osdd index remove --topic <t> --path <p> # Unlink
osdd index list [--topic <t>] [--path <p>] # List
osdd index find <query> --by topic|path
```

## Mode Operations

```bash
osdd mode list              # List modes
osdd mode info <name>       # Mode details
osdd mode activate <name>   # Switch mode
osdd mode add <path>       # Add mode
osdd mode remove <name>    # Remove mode
osdd mode current           # Show active
```

## Connector Operations

```bash
osdd connector new <name> <desc> <cmd> [args...]  # Create
osdd connector list                                  # List
osdd connector run <name>                            # Run
osdd connector edit <name> <desc>                    # Edit
osdd connector delete <name>                         # Delete
osdd connector mcp                                   # MCP config
```

## Patty Commands

```bash
osdd patty enable     # Show platypus
osdd patty disable    # Hide platypus
osdd patty status     # Check status
```

## MCP Server

```bash
osdd mcp              # Start MCP server
```

---

**Remember**: Write the spec first! 🦫
