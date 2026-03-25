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
            description: "Find links by topic or path".to_string(),
            input_schema: serde_json::json!({
                "type": "object",
                "properties": {
                    "query": {
                        "type": "string",
                        "description": "Query (topic name or path)"
                    },
                    "by": {
                        "type": "string",
                        "description": "Search by: 'topic' or 'path'",
                        "enum": ["topic", "path"],
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
