// src/mcp/mod.rs - Tool definitions for MCP server
mod server;

use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tool {
    pub name: String,
    pub description: String,
    pub input_schema: Value,
}

pub fn get_tools() -> Vec<Tool> {
    let mut tools = vec![
        Tool {
            name: "topics_list".to_string(),
            description: "List all topics in an area".to_string(),
            input_schema: serde_json::json!({
                "type": "object",
                "properties": {
                    "area": {
                        "type": "string",
                        "description": "Area to list topics from (default: current area)",
                        "default": "Working"
                    }
                }
            }),
        },
        Tool {
            name: "topics_add".to_string(),
            description: "Create a new topic".to_string(),
            input_schema: serde_json::json!({
                "type": "object",
                "properties": {
                    "topic": {
                        "type": "string",
                        "description": "Name of the topic to create"
                    },
                    "area": {
                        "type": "string",
                        "description": "Area to create the topic in",
                        "default": "Working"
                    }
                },
                "required": ["topic"]
            }),
        },
        Tool {
            name: "topics_show".to_string(),
            description: "Show details of a topic".to_string(),
            input_schema: serde_json::json!({
                "type": "object",
                "properties": {
                    "topic": {
                        "type": "string",
                        "description": "Name of the topic to show"
                    },
                    "show_all": {
                        "type": "boolean",
                        "description": "Show files from all areas"
                    },
                    "from": {
                        "type": "string",
                        "description": "Show files from a specific area"
                    }
                },
                "required": ["topic"]
            }),
        },
        Tool {
            name: "topics_delete".to_string(),
            description: "Delete a topic".to_string(),
            input_schema: serde_json::json!({
                "type": "object",
                "properties": {
                    "topic": {
                        "type": "string",
                        "description": "Name of the topic to delete"
                    },
                    "area": {
                        "type": "string",
                        "description": "Area containing the topic (default: current area)",
                        "default": "Working"
                    },
                    "force": {
                        "type": "boolean",
                        "description": "Force deletion without confirmation",
                        "default": true
                    }
                },
                "required": ["topic"]
            }),
        },
        Tool {
            name: "topics_push".to_string(),
            description: "Push a topic to another area".to_string(),
            input_schema: serde_json::json!({
                "type": "object",
                "properties": {
                    "topic": {
                        "type": "string",
                        "description": "Name of the topic to push"
                    },
                    "area": {
                        "type": "string",
                        "description": "Target area to push to"
                    },
                    "source_area": {
                        "type": "string",
                        "description": "Source area (default: current area)"
                    }
                },
                "required": ["topic", "area"]
            }),
        },
        Tool {
            name: "topics_pull".to_string(),
            description: "Pull a topic from another area".to_string(),
            input_schema: serde_json::json!({
                "type": "object",
                "properties": {
                    "topic": {
                        "type": "string",
                        "description": "Name of the topic to pull"
                    },
                    "source_area": {
                        "type": "string",
                        "description": "Source area to pull from"
                    }
                },
                "required": ["topic", "source_area"]
            }),
        },
        Tool {
            name: "topics_progress".to_string(),
            description: "Show progress across topics in an area".to_string(),
            input_schema: serde_json::json!({
                "type": "object",
                "properties": {
                    "area": {
                        "type": "string",
                        "description": "Area to show progress for (default: current area)"
                    }
                }
            }),
        },
        Tool {
            name: "areas_list".to_string(),
            description: "List all areas".to_string(),
            input_schema: serde_json::json!({
                "type": "object",
                "properties": {}
            }),
        },
        Tool {
            name: "areas_add".to_string(),
            description: "Add a new area".to_string(),
            input_schema: serde_json::json!({
                "type": "object",
                "properties": {
                    "name": {
                        "type": "string",
                        "description": "Name of the area to add"
                    }
                },
                "required": ["name"]
            }),
        },
        Tool {
            name: "areas_remove".to_string(),
            description: "Remove an area".to_string(),
            input_schema: serde_json::json!({
                "type": "object",
                "properties": {
                    "name": {
                        "type": "string",
                        "description": "Name of the area to remove"
                    }
                },
                "required": ["name"]
            }),
        },
        Tool {
            name: "areas_rename".to_string(),
            description: "Rename an area".to_string(),
            input_schema: serde_json::json!({
                "type": "object",
                "properties": {
                    "old": {
                        "type": "string",
                        "description": "Current name of the area"
                    },
                    "new": {
                        "type": "string",
                        "description": "New name for the area"
                    }
                },
                "required": ["old", "new"]
            }),
        },
        Tool {
            name: "areas_default".to_string(),
            description: "Set the default area".to_string(),
            input_schema: serde_json::json!({
                "type": "object",
                "properties": {
                    "name": {
                        "type": "string",
                        "description": "Name of the area to set as default"
                    }
                },
                "required": ["name"]
            }),
        },
        Tool {
            name: "areas_health".to_string(),
            description: "Show area health (topic counts by status)".to_string(),
            input_schema: serde_json::json!({
                "type": "object",
                "properties": {}
            }),
        },
        Tool {
            name: "index_list".to_string(),
            description: "List all index links, optionally filtered".to_string(),
            input_schema: serde_json::json!({
                "type": "object",
                "properties": {
                    "topic": {
                        "type": "string",
                        "description": "Filter by topic name"
                    },
                    "path": {
                        "type": "string",
                        "description": "Filter by path"
                    }
                }
            }),
        },
        Tool {
            name: "index_add".to_string(),
            description: "Add a link between a topic and a path".to_string(),
            input_schema: serde_json::json!({
                "type": "object",
                "properties": {
                    "topic": {
                        "type": "string",
                        "description": "Topic name"
                    },
                    "path": {
                        "type": "string",
                        "description": "Path to link (file or directory)"
                    },
                    "area": {
                        "type": "string",
                        "description": "Area name (auto-detected if not specified)",
                        "default": "Working"
                    },
                    "link_type": {
                        "type": "string",
                        "description": "Type: 'file' or 'directory' (auto-detected if not specified)"
                    }
                },
                "required": ["topic", "path"]
            }),
        },
        Tool {
            name: "index_remove".to_string(),
            description: "Remove a link between a topic and a path".to_string(),
            input_schema: serde_json::json!({
                "type": "object",
                "properties": {
                    "topic": {
                        "type": "string",
                        "description": "Topic name"
                    },
                    "path": {
                        "type": "string",
                        "description": "Path to unlink"
                    }
                },
                "required": ["topic", "path"]
            }),
        },
        Tool {
            name: "index_find".to_string(),
            description: "Find links by topic, path, tag, or annotation".to_string(),
            input_schema: serde_json::json!({
                "type": "object",
                "properties": {
                    "query": {
                        "type": "string",
                        "description": "Query (topic name, path, tag, or annotation text)"
                    },
                    "by": {
                        "type": "string",
                        "description": "Search by: 'topic', 'path', 'tag', or 'annotation'",
                        "enum": ["topic", "path", "tag", "annotation"],
                        "default": "topic"
                    }
                },
                "required": ["query", "by"]
            }),
        },
        Tool {
            name: "index_cleanup".to_string(),
            description: "Remove links to topics or paths that no longer exist".to_string(),
            input_schema: serde_json::json!({
                "type": "object",
                "properties": {}
            }),
        },
        Tool {
            name: "index_tags".to_string(),
            description: "List all unique tags in the index".to_string(),
            input_schema: serde_json::json!({
                "type": "object",
                "properties": {}
            }),
        },
        Tool {
            name: "index_graph".to_string(),
            description: "Export index as graph JSON for visualization".to_string(),
            input_schema: serde_json::json!({
                "type": "object",
                "properties": {}
            }),
        },
        Tool {
            name: "index_backlinks".to_string(),
            description: "Generate backlinks markdown for a topic".to_string(),
            input_schema: serde_json::json!({
                "type": "object",
                "properties": {
                    "topic": {
                        "type": "string",
                        "description": "Topic name to get backlinks for"
                    }
                },
                "required": ["topic"]
            }),
        },
        Tool {
            name: "index_exports".to_string(),
            description: "List exports (functions, classes, etc) for a topic".to_string(),
            input_schema: serde_json::json!({
                "type": "object",
                "properties": {
                    "topic": {
                        "type": "string",
                        "description": "Topic name to get exports for"
                    }
                }
            }),
        },
        Tool {
            name: "index_query".to_string(),
            description: "Query exports by name, type, description, or ID".to_string(),
            input_schema: serde_json::json!({
                "type": "object",
                "properties": {
                    "query": {
                        "type": "string",
                        "description": "Search query"
                    },
                    "by": {
                        "type": "string",
                        "description": "Search by: name, type, description, or id",
                        "enum": ["name", "type", "description", "id"],
                        "default": "name"
                    }
                },
                "required": ["query", "by"]
            }),
        },
        Tool {
            name: "index_depends".to_string(),
            description: "Find what topics depend on a given topic".to_string(),
            input_schema: serde_json::json!({
                "type": "object",
                "properties": {
                    "topic": {
                        "type": "string",
                        "description": "Topic name to find dependents for"
                    }
                },
                "required": ["topic"]
            }),
        },
        Tool {
            name: "index_lookup".to_string(),
            description: "Find export by full ID (e.g., user-login:login_user)".to_string(),
            input_schema: serde_json::json!({
                "type": "object",
                "properties": {
                    "id": {
                        "type": "string",
                        "description": "Full export ID (topic:name)"
                    }
                },
                "required": ["id"]
            }),
        },
        Tool {
            name: "config_get".to_string(),
            description: "Get the current configuration".to_string(),
            input_schema: serde_json::json!({
                "type": "object",
                "properties": {}
            }),
        },
        Tool {
            name: "config_set".to_string(),
            description: "Set the default area".to_string(),
            input_schema: serde_json::json!({
                "type": "object",
                "properties": {
                    "area": {
                        "type": "string",
                        "description": "Area to set as default"
                    }
                },
                "required": ["area"]
            }),
        },
        Tool {
            name: "mode_list".to_string(),
            description: "List all available agent modes".to_string(),
            input_schema: serde_json::json!({
                "type": "object",
                "properties": {}
            }),
        },
        Tool {
            name: "mode_info".to_string(),
            description: "Get detailed info about a mode".to_string(),
            input_schema: serde_json::json!({
                "type": "object",
                "properties": {
                    "name": {
                        "type": "string",
                        "description": "Name of the mode"
                    }
                },
                "required": ["name"]
            }),
        },
        Tool {
            name: "mode_activate".to_string(),
            description: "Activate an agent mode".to_string(),
            input_schema: serde_json::json!({
                "type": "object",
                "properties": {
                    "name": {
                        "type": "string",
                        "description": "Name of the mode to activate"
                    }
                },
                "required": ["name"]
            }),
        },
        Tool {
            name: "mode_current".to_string(),
            description: "Get the current active mode".to_string(),
            input_schema: serde_json::json!({
                "type": "object",
                "properties": {}
            }),
        },
        Tool {
            name: "connector_list".to_string(),
            description: "List all available connectors".to_string(),
            input_schema: serde_json::json!({
                "type": "object",
                "properties": {}
            }),
        },
        Tool {
            name: "connector_run".to_string(),
            description: "Run a connector command".to_string(),
            input_schema: serde_json::json!({
                "type": "object",
                "properties": {
                    "name": {
                        "type": "string",
                        "description": "Name of the connector to run"
                    },
                    "args": {
                        "type": "array",
                        "items": {"type": "string"},
                        "description": "Additional arguments to pass to the command"
                    }
                },
                "required": ["name"]
            }),
        },
        Tool {
            name: "code_analysis".to_string(),
            description: "Query code analysis data from ingested topics - get functions, structs, enums by topic/module name".to_string(),
            input_schema: serde_json::json!({
                "type": "object",
                "properties": {
                    "topic": {
                        "type": "string",
                        "description": "Topic name to query (root topic from ingest)"
                    },
                    "area": {
                        "type": "string",
                        "description": "Area containing the topic",
                        "default": "Ingested"
                    },
                    "module": {
                        "type": "string",
                        "description": "Optional module/subtopic to filter to"
                    },
                    "item_type": {
                        "type": "string",
                        "description": "Filter by type: functions, structs, enums (default: all)",
                        "default": "all"
                    },
                    "pattern": {
                        "type": "string",
                        "description": "Optional search pattern to filter results"
                    }
                },
                "required": ["topic"]
            }),
        },
        Tool {
            name: "code_parse".to_string(),
            description: "Parse a single file on-demand using tree-sitter - useful for debugging or finding code elements while working".to_string(),
            input_schema: serde_json::json!({
                "type": "object",
                "properties": {
                    "path": {
                        "type": "string",
                        "description": "Path to the file to parse"
                    },
                    "language": {
                        "type": "string",
                        "description": "Language (auto-detected if not specified): rust, javascript, typescript, python, go, bash"
                    },
                    "item_type": {
                        "type": "string",
                        "description": "What to extract: functions, structs, enums, imports, all (default: all)",
                        "default": "all"
                    },
                    "pattern": {
                        "type": "string",
                        "description": "Filter by name pattern"
                    }
                },
                "required": ["path"]
            }),
        },
        Tool {
            name: "master_spec".to_string(),
            description: "Get the master spec - a high-level overview of the project for context".to_string(),
            input_schema: serde_json::json!({
                "type": "object",
                "properties": {}
            }),
        },
        Tool {
            name: "auto_build".to_string(),
            description: "Build topic from spec - spawns agents, tracks commits, merges via PR".to_string(),
            input_schema: serde_json::json!({
                "type": "object",
                "properties": {
                    "topic": { "type": "string", "description": "Topic to build" },
                    "area": { "type": "string", "description": "Area (default: current)" },
                    "spec_file": { "type": "string", "description": "Optional specific spec file" }
                },
                "required": ["topic"]
            }),
        },
        Tool {
            name: "auto_ingest".to_string(),
            description: "Ingest codebase into specs - requires master.md".to_string(),
            input_schema: serde_json::json!({
                "type": "object",
                "properties": {
                    "code_path": { "type": "string", "description": "Path to codebase" },
                    "master_spec": { "type": "string", "description": "Path to master.md (required)" },
                    "topic": { "type": "string", "description": "Optional topic name" },
                    "area": { "type": "string", "description": "Area (default: Working)" }
                },
                "required": ["code_path", "master_spec"]
            }),
        },
        Tool {
            name: "auto_verify".to_string(),
            description: "Verify topic alignment - code → specs → topic".to_string(),
            input_schema: serde_json::json!({
                "type": "object",
                "properties": {
                    "topic": { "type": "string", "description": "Topic to verify" },
                    "area": { "type": "string", "description": "Area (default: current)" }
                },
                "required": ["topic"]
            }),
        },
        Tool {
            name: "auto_locks_list".to_string(),
            description: "List all active locks".to_string(),
            input_schema: serde_json::json!({
                "type": "object",
                "properties": {}
            }),
        },
        Tool {
            name: "auto_locks_clear".to_string(),
            description: "Clear a lock by session_id".to_string(),
            input_schema: serde_json::json!({
                "type": "object",
                "properties": {
                    "session_id": { "type": "string", "description": "Session ID to clear" }
                },
                "required": ["session_id"]
            }),
        },
        Tool {
            name: "auto_commits".to_string(),
            description: "Get commit history for a topic".to_string(),
            input_schema: serde_json::json!({
                "type": "object",
                "properties": {
                    "topic": { "type": "string", "description": "Topic name" }
                },
                "required": ["topic"]
            }),
        },
        Tool {
            name: "auto_topic_tree".to_string(),
            description: "Get nested topic structure".to_string(),
            input_schema: serde_json::json!({
                "type": "object",
                "properties": {
                    "topic": { "type": "string", "description": "Topic name" }
                },
                "required": ["topic"]
            }),
        },
        Tool {
            name: "auto_agent".to_string(),
            description: "Run agent process manually for debugging".to_string(),
            input_schema: serde_json::json!({
                "type": "object",
                "properties": {
                    "topic": { "type": "string", "description": "Topic to work on" },
                    "session_id": { "type": "string", "description": "Optional session ID" },
                    "parent_topic": { "type": "string", "description": "Optional parent topic" },
                    "area": { "type": "string", "description": "Area (default: Working)" },
                    "workflow": { "type": "string", "description": "Workflow template (default: build)" }
                },
                "required": ["topic"]
            }),
        },
    ];

    // Add dynamic connector tools
    if let Ok(config) = crate::agent::load_agent_config() {
        for connector in config.connectors {
            tools.push(Tool {
                name: format!("unispec_{}", connector.name),
                description: connector.description,
                input_schema: serde_json::json!({
                    "type": "object",
                    "properties": {
                        "args": {
                            "type": "array",
                            "items": {"type": "string"},
                            "description": "Additional arguments to pass to the command"
                        }
                    }
                }),
            });
        }
    }

    tools
}

pub use server::run_mcp_server;
