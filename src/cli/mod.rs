// src/cli/mod.rs
pub mod model;

use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "unispec")]
#[command(
    about = "unispec",
    long_about = "UniSpec - Spec-Driven Development

A TUI-based tool for managing specs and topics across different areas.

Run 'unispec' without arguments to launch the interactive TUI.

QUICK START:
  unispec init              Launch the TUI
  unispec init              Initialize a new project (creates spec/ and .agent/ folders)                  Launch the TUI
  unispec topic add <name>  Add a new topic to the current area
  unispec area list        List all areas

TOPICS:
  unispec topic add <topic> [-a <area>]      Create a new topic
  unispec topic list [-a <area>]             List topics in an area
  unispec topic push <topic> <area>          Push topic to another area
  unispec topic pull <topic> <area>          Pull topic from another area
  unispec topic remove <topic>                Remove a topic
  unispec topic show <topic>                 Show topic details
  unispec topic progress [-a <area>]         Show progress across topics

AREAS:
  unispec area add <name>                    Add a new area
  unispec area remove <name>                 Remove an area
  unispec area list                          List all areas
  unispec area rename <old> <new>            Rename an area
  unispec area default <name>                Set default area
  unispec area health                        Show area health stats

INDEX:
  unispec index add --topic <name> --path <path>    Link topic to path
  unispec index remove --topic <name> --path <path> Remove link
  unispec index list [--topic <name>] [--path <path>]  List links
  unispec index find <query> --by topic|path        Find links

AGENT MODES:
  unispec mode list               List available agent modes
  unispec mode activate <name>    Activate an agent mode
  unispec mode info <name>        Show mode details
  unispec mode add <path>         Add a mode from a directory
  unispec mode remove <name>      Remove a mode
  unispec mode current            Show current mode

AGENT CONNECTORS:
  unispec connector new <name> <description> <command>  Create a connector
  unispec connector list          List all connectors
  unispec connector run <name>    Run a connector
  unispec connector delete <name> Delete a connector
  unispec connector mcp           Generate MCP config for connectors

OTHER:
  unispec set <area>               Set default area
  unispec init [-r <path>]         Initialize project
  unispec mcp                      Launch MCP server for agent integration
"
)]
#[command(version = crate::version::VERSION)]
#[command(args_conflicts_with_subcommands = true)]
pub struct Cli {
    #[arg(short, long)]
    pub quiet: bool,
    #[arg(short, long)]
    pub verbose: bool,
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Initialize a new UniSpec project
    Init {
        /// Root directory to initialize (default: current directory)
        #[arg(short, long)]
        root: Option<PathBuf>,
        /// Add Amazon Q Developer integration (.amazonq/prompts)
        #[arg(long)]
        amazon_q: bool,
        /// Add Antigravity integration (.agent/workflows)
        #[arg(long)]
        antigravity: bool,
        /// Add Augment CLI integration (.augment/commands)
        #[arg(long)]
        auggie: bool,
        /// Add Claude Code integration (.claude/commands/unispec)
        #[arg(long)]
        claude_code: bool,
        /// Add Cline integration (.clinerules/workflows)
        #[arg(long)]
        cline: bool,
        /// Add Codex integration (~/.codex/prompts)
        #[arg(long)]
        codex: bool,
        /// Add CodeBuddy integration (.codebuddy/commands/unispec)
        #[arg(long)]
        codebuddy: bool,
        /// Add Continue integration (.continue/prompts)
        #[arg(long)]
        continue_: bool,
        /// Add CoStrict integration (.cospec/unispec/commands)
        #[arg(long)]
        costrict: bool,
        /// Add Crush integration (.crush/commands/unispec)
        #[arg(long)]
        crush: bool,
        /// Add Cursor integration (.cursor/commands)
        #[arg(long)]
        cursor: bool,
        /// Add Factory Droid integration (.factory/commands)
        #[arg(long)]
        factory: bool,
        /// Add Gemini CLI integration (.gemini/commands/unispec)
        #[arg(long)]
        gemini_cli: bool,
        /// Add GitHub integration (.github/prompts)
        #[arg(long)]
        github: bool,
        /// Add iFlow integration (.iflow/commands)
        #[arg(long)]
        iflow: bool,
        /// Add Kilo Code integration (.kilocode/workflows)
        #[arg(long)]
        kilo_code: bool,
        /// Add Kiro integration (.kiro/prompts)
        #[arg(long)]
        kiro: bool,
        /// Add OpenCode integration (.opencode/command)
        #[arg(long)]
        opencode: bool,
        /// Add Pi integration (.pi/prompts)
        #[arg(long)]
        pi: bool,
        /// Add Qoder integration (.qoder/commands/unispec)
        #[arg(long)]
        qoder: bool,
        /// Add Qwen Code integration (.qwen/commands)
        #[arg(long)]
        qwen_code: bool,
        /// Add RooCode integration (.roo/commands)
        #[arg(long)]
        roo_code: bool,
        /// Add Windsurf integration (.windsurf/workflows)
        #[arg(long)]
        windsurf: bool,
        /// Add TRAE integration (.trae/rule)
        #[arg(long)]
        trae: bool,
    },
    /// Set the default area
    Set { area: String },
    /// Manage areas (add, remove, list, rename, default, health)
    #[command(subcommand)]
    Area(AreaCommands),
    /// Manage topics (add, list, push, pull, remove, show, progress)
    #[command(subcommand)]
    Topic(TopicCommands),
    /// Index commands (full, watch)
    #[command(subcommand)]
    Index(IndexCommands),
    /// Launch MCP server for agent integration
    Mcp {
        /// Project directory to run MCP server for
        path: Option<PathBuf>,
    },
    /// Agent mode commands
    #[command(subcommand)]
    Mode(ModeCommands),
    /// Agent connector commands
    #[command(subcommand)]
    Connector(ConnectorCommands),
    /// Control platypus mascot display
    #[command(subcommand)]
    Patty(PattyCommands),
}

#[derive(Subcommand)]
pub enum ModeCommands {
    /// List available modes
    List,
    /// Show detailed info about a mode
    Info {
        /// Name of the mode
        name: String,
    },
    /// Activate a mode
    Activate {
        /// Name of the mode to activate
        name: String,
    },
    /// Add a mode from a directory (copies to local or global modes)
    Add {
        /// Path to the mode directory
        path: String,
        /// Add to global modes instead of local
        #[arg(short, long)]
        global: bool,
    },
    /// Remove a mode
    Remove {
        /// Name of the mode to remove
        name: String,
        /// Remove from global modes instead of local
        #[arg(short, long)]
        global: bool,
    },
    /// Show current mode
    Current,
}

#[derive(Subcommand)]
pub enum ConnectorCommands {
    /// Create a new connector
    New {
        /// Name of the connector
        name: String,
        /// Description of what it does
        description: String,
        /// Command to run
        command: String,
        /// Arguments (space-separated)
        #[arg(short, long)]
        args: Option<String>,
        /// Environment variables (KEY=value format, comma-separated)
        #[arg(short, long)]
        env: Option<String>,
        /// Working directory
        #[arg(short, long)]
        working_dir: Option<String>,
        /// Timeout in seconds
        #[arg(short, long)]
        timeout: Option<u64>,
    },
    /// List all connectors
    List,
    /// Delete a connector
    Delete {
        /// Name of the connector to delete
        name: String,
    },
    /// Run a connector
    Run {
        /// Name of the connector to run
        name: String,
        /// Additional arguments
        #[arg(last = true)]
        args: Vec<String>,
    },
    /// Edit a connector
    Edit {
        /// Name of the connector
        name: String,
        /// New description
        #[arg(short, long)]
        description: Option<String>,
    },
    /// Generate MCP server configuration for connectors
    Mcp,
}

#[derive(Subcommand)]
pub enum AreaCommands {
    /// Add a new area
    Add {
        /// Name of the area to add
        name: String,
    },
    /// Remove an area
    Remove {
        /// Name of the area to remove
        name: String,
    },
    /// List all areas
    List,
    /// Rename an area
    Rename {
        /// Current name of the area
        old: String,
        /// New name for the area
        new: String,
    },
    /// Set the default area
    Default {
        /// Name of the area to set as default
        name: String,
    },
    /// Show area health (topic counts by status)
    Health,
}

#[derive(Subcommand)]
pub enum TopicCommands {
    /// Add a new topic
    Add {
        /// Name of the topic to create
        topic: String,
        /// Area to create the topic in (default: Working)
        #[arg(short, long, default_value = "Working")]
        area: String,
    },
    /// List topics in an area
    List {
        /// Area to list topics from (default: Working)
        #[arg(short, long, default_value = "Working")]
        area: String,
        /// Show hierarchical view
        #[arg(short, long)]
        hierarchy: bool,
    },
    /// Push a topic to another area
    Push {
        /// Name of the topic to push
        topic: String,
        /// Target area to push to
        area: String,
    },
    /// Pull a topic from another area
    Pull {
        /// Name of the topic to pull
        topic: String,
        /// Source area to pull from
        area: String,
    },
    /// Remove a topic
    Remove {
        /// Name of the topic to remove
        topic: String,
        /// Force removal without confirmation
        #[arg(short, long)]
        force: bool,
    },
    /// Show topic details
    Show {
        /// Name of the topic to show
        topic: String,
    },
    /// Show progress across all topics
    Progress {
        /// Area to show progress for (default: current area)
        #[arg(short, long)]
        area: Option<String>,
    },
}

#[derive(Subcommand)]
pub enum IndexCommands {
    /// Add a link between a topic and a path
    Add {
        /// Topic name
        #[arg(short, long)]
        topic: String,
        /// Path to link (file or directory)
        #[arg(short, long)]
        path: String,
        /// Area name (auto-detected from topic if not specified)
        #[arg(short, long)]
        area: Option<String>,
        /// Type: 'file' or 'directory' (auto-detected if not specified)
        #[arg(short, long)]
        link_type: Option<String>,
    },
    /// Remove a link between a topic and a path
    Remove {
        /// Topic name
        #[arg(short, long)]
        topic: String,
        /// Path to unlink
        #[arg(short, long)]
        path: String,
    },
    /// List all links (optionally filtered by topic or path)
    List {
        /// Filter by topic name
        #[arg(short, long)]
        topic: Option<String>,
        /// Filter by path
        #[arg(short, long)]
        path: Option<String>,
    },
    /// Find links by topic or path
    Find {
        /// Query (topic name or path)
        query: String,
        /// Search by: 'topic' or 'path'
        #[arg(short, long, default_value = "topic")]
        by: String,
    },
    /// Show index stats
    Full,
    /// Start watcher
    Watch,
    /// Clean up orphaned links (topics/paths that no longer exist)
    Cleanup,
}

#[derive(Subcommand)]
pub enum PattyCommands {
    /// Enable platypus mascot
    Enable,
    /// Disable platypus mascot
    Disable,
    /// Show current status
    Status,
}
