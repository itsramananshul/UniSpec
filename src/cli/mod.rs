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
  unispec pkg list                 List available packages
  unispec pkg install <pkg>       Install a package
  unispec pkg remove <pkg>        Remove a package

INDEX:
  unispec index add --topic X --path Y [--exports a,b]  Add link with exports
  unispec index exports --topic X   List exports for topic
  unispec index query <q> --by name   Query exports by name/type/description
  unispec index depends --topic X    Show what depends on topic
  unispec index lookup --id X         Lookup export by ID
  unispec index backlinks --topic X  Show backlinks
 "
)]
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
    /// Manage packages (list, install, remove)
    #[command(subcommand)]
    Pkg(PkgCommands),
    /// Ingest a codebase and create specs from it
    #[command(subcommand)]
    Ingest(IngestCommands),
    /// Parse a single file with tree-sitter (auto-detects language)
    #[command(subcommand)]
    Parse(ParseCommands),
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
        /// Source area to push from
        #[arg(long)]
        from: Option<String>,
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
        /// Show all files in the topic (not just current area's files)
        #[arg(short, long)]
        all: bool,
        /// Show files from a specific area (useful when topic has files from multiple areas)
        #[arg(short, long)]
        from: Option<String>,
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
        /// Tags (comma-separated)
        #[arg(short, long)]
        tags: Option<String>,
        /// Annotation/note about this link
        #[arg(short, long)]
        annotation: Option<String>,
        /// Exports (comma-separated names)
        #[arg(long)]
        exports: Option<String>,
        /// Export descriptions (comma-separated, matching exports order)
        #[arg(long)]
        descriptions: Option<String>,
        /// Export types (comma-separated: function,class,endpoint,model,service,config)
        #[arg(long)]
        export_types: Option<String>,
        /// Function signatures (semicolon-separated)
        #[arg(long)]
        signatures: Option<String>,
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
        /// Filter by tag
        #[arg(long)]
        tag: Option<String>,
    },
    /// Find links by topic, path, or tag
    Find {
        /// Query (topic name, path, or tag)
        query: String,
        /// Search by: 'topic', 'path', or 'tag'
        #[arg(short, long, default_value = "topic")]
        by: String,
    },
    /// Show index stats
    Full,
    /// Start watcher
    Watch,
    /// Clean up orphaned links (topics/paths that no longer exist)
    Cleanup,
    /// List all unique tags in the index
    Tags,
    /// Generate graph.json for visualization
    Graph,
    /// Generate backlinks file for a topic
    Backlinks {
        /// Topic name
        #[arg(short, long)]
        topic: String,
    },
    /// List exports for a topic
    Exports {
        /// Topic name
        #[arg(short, long)]
        topic: Option<String>,
    },
    /// Query exports by name, type, description, or ID
    Query {
        /// Search query
        query: String,
        /// Search by: name, type, description, or id
        #[arg(short, long, default_value = "name")]
        by: String,
    },
    /// Find what depends on a topic (reverse lookups)
    Depends {
        /// Topic name
        #[arg(short, long)]
        topic: String,
    },
    /// Find export by full ID (e.g., user-login:login_user)
    Lookup {
        /// Full export ID
        #[arg(short, long)]
        id: String,
    },
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

#[derive(Subcommand)]
pub enum PkgCommands {
    /// List available packages from the repository
    List {
        /// Repository URL (default: official UniSpec repo)
        #[arg(short, long)]
        repo: Option<String>,
    },
    /// Search for packages
    Search {
        /// Search query
        query: String,
        /// Repository URL (default: official UniSpec repo)
        #[arg(short, long)]
        repo: Option<String>,
    },
    /// Install a package (mode, connectors, workflows)
    Install {
        /// Package name or GitHub URL
        package: String,
        /// Install globally (user-wide)
        #[arg(short, long)]
        global: bool,
        /// Repository URL
        #[arg(short, long)]
        repo: Option<String>,
    },
    /// Remove an installed package
    Remove {
        /// Package name
        package: String,
        /// Remove from global installation
        #[arg(short, long)]
        global: bool,
    },
    /// List installed packages
    Installed {
        /// Show globally installed packages
        #[arg(short, long)]
        global: bool,
    },
}

#[derive(Subcommand)]
pub enum IngestCommands {
    /// Ingest a codebase directory and create specs from it
    Run {
        /// Path to the codebase to ingest
        path: String,
        /// Target area to create specs in
        #[arg(short, long, default_value = "Ingested")]
        area: String,
        /// Topic name for the ingested code
        #[arg(short, long)]
        topic: Option<String>,
        /// Languages to parse (default: all supported)
        #[arg(short, long)]
        languages: Option<String>,
        /// Watch for file changes and re-ingest automatically
        #[arg(short, long)]
        watch: bool,
    },
    /// Start live file watching for auto-indexing
    Watch {
        /// Path to watch (default: current directory)
        #[arg(short, long)]
        path: Option<String>,
        /// Topic to link code to
        #[arg(short, long)]
        topic: Option<String>,
    },
    /// Stop watching
    Stop,
}

#[derive(Subcommand)]
pub enum ParseCommands {
    /// Parse a single file and extract code elements
    File {
        /// Path to the file to parse
        path: String,
        /// Language to use (auto-detected if not specified)
        #[arg(short, long)]
        language: Option<String>,
        /// What to extract: functions, structs, enums, imports, all (default: all)
        #[arg(short, long, default_value = "all")]
        item_type: String,
        /// Filter by name pattern
        #[arg(short, long)]
        pattern: Option<String>,
        /// Output as JSON (for agent consumption)
        #[arg(long)]
        json: bool,
    },
}
