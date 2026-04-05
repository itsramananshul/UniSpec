pub mod server;

use serde_json::json;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Tool {
    pub name: String,
    pub description: String,
    pub input_schema: serde_json::Value,
}

pub fn get_tools() -> Vec<Tool> {
    let mut tools = vec![
        Tool {
            name: "unispec_nav".to_string(),
            description: "List topics and areas to navigate the project tree.".to_string(),
            input_schema: json!({
                "type": "object",
                "properties": {
                    "area": { "type": "string", "description": "Optional area to filter by" }
                }
            }),
        },
        Tool {
            name: "unispec_read_spec".to_string(),
            description: "Read spec.md and task.md for a topic.".to_string(),
            input_schema: json!({
                "type": "object",
                "properties": {
                    "topic": { "type": "string", "description": "Topic name" },
                    "area": { "type": "string", "description": "Area name" }
                },
                "required": ["topic"]
            }),
        },
        Tool {
            name: "unispec_update_task".to_string(),
            description: "Update task status in task.md.".to_string(),
            input_schema: json!({
                "type": "object",
                "properties": {
                    "topic": { "type": "string" },
                    "task_index": { "type": "integer" },
                    "status": { "type": "string", "enum": [" ", "x", "-", "!"] },
                    "note": { "type": "string" }
                },
                "required": ["topic", "task_index", "status"]
            }),
        },
        Tool {
            name: "unispec_bind_spec".to_string(),
            description: "Bind a code file to a spec.".to_string(),
            input_schema: json!({
                "type": "object",
                "properties": {
                    "spec_path": { "type": "string" },
                    "file_path": { "type": "string" },
                    "topic": { "type": "string" },
                    "area": { "type": "string" }
                },
                "required": ["spec_path", "file_path", "topic"]
            }),
        },
        Tool {
            name: "unispec_query_relations".to_string(),
            description: "Find callers/callees for a symbol.".to_string(),
            input_schema: json!({
                "type": "object",
                "properties": {
                    "symbol": { "type": "string" }
                },
                "required": ["symbol"]
            }),
        },
        Tool {
            name: "unispec_auto_build".to_string(),
            description: "Orchestrate a build for a topic.".to_string(),
            input_schema: json!({
                "type": "object",
                "properties": {
                    "topic": { "type": "string" },
                    "area": { "type": "string" }
                },
                "required": ["topic"]
            }),
        },
        Tool {
            name: "unispec_auto_verify".to_string(),
            description: "Verify spec alignment, optionally fixing issues.".to_string(),
            input_schema: json!({
                "type": "object",
                "properties": {
                    "topic": { "type": "string" },
                    "fix": { "type": "boolean" }
                },
                "required": ["topic"]
            }),
        },
    ];

    tools
}
