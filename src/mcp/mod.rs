pub mod server;

use serde_json::json;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Tool {
    pub name: String,
    pub description: String,
    pub input_schema: serde_json::Value,
}

pub fn get_tools() -> Vec<Tool> {
    vec![
        // === Area Management ===
        Tool {
            name: "areas_list".to_string(),
            description: "List all areas".to_string(),
            input_schema: json!({
                "type": "object",
                "properties": {}
            }),
        },
        // === Topic Management ===
        Tool {
            name: "topics_list".to_string(),
            description: "List all topics in an area".to_string(),
            input_schema: json!({
                "type": "object",
                "properties": {
                    "area": { "type": "string", "description": "Area name (default: Working)" }
                }
            }),
        },
        Tool {
            name: "topics_add".to_string(),
            description: "Create a new topic in an area".to_string(),
            input_schema: json!({
                "type": "object",
                "properties": {
                    "topic": { "type": "string", "description": "Topic name" },
                    "area": { "type": "string", "description": "Area name (default: Working)" }
                },
                "required": ["topic"]
            }),
        },
        Tool {
            name: "topics_delete".to_string(),
            description: "Delete a topic from an area".to_string(),
            input_schema: json!({
                "type": "object",
                "properties": {
                    "topic": { "type": "string", "description": "Topic name" },
                    "area": { "type": "string", "description": "Area name (default: Working)" },
                    "force": { "type": "boolean", "description": "Skip confirmation" }
                },
                "required": ["topic"]
            }),
        },
        Tool {
            name: "topics_show".to_string(),
            description: "Show details of a topic".to_string(),
            input_schema: json!({
                "type": "object",
                "properties": {
                    "topic": { "type": "string", "description": "Topic name" },
                    "area": { "type": "string", "description": "Area name (default: Working)" }
                },
                "required": ["topic"]
            }),
        },
        Tool {
            name: "topics_push".to_string(),
            description: "Push/move a topic to another area".to_string(),
            input_schema: json!({
                "type": "object",
                "properties": {
                    "topic": { "type": "string", "description": "Topic name" },
                    "area": { "type": "string", "description": "Target area" },
                    "source_area": { "type": "string", "description": "Source area (auto-detected if not provided)" }
                },
                "required": ["topic", "area"]
            }),
        },
        Tool {
            name: "topics_pull".to_string(),
            description: "Pull a topic from another area into Working".to_string(),
            input_schema: json!({
                "type": "object",
                "properties": {
                    "topic": { "type": "string", "description": "Topic name" },
                    "source_area": { "type": "string", "description": "Source area to pull from" }
                },
                "required": ["topic", "source_area"]
            }),
        },
        Tool {
            name: "topics_progress".to_string(),
            description: "Show progress across topics in an area".to_string(),
            input_schema: json!({
                "type": "object",
                "properties": {
                    "area": { "type": "string", "description": "Area name (default: Working)" }
                }
            }),
        },
        // === Read Specs ===
        Tool {
            name: "unispec_read_spec".to_string(),
            description: "Read spec.md and task.md for a topic".to_string(),
            input_schema: json!({
                "type": "object",
                "properties": {
                    "topic": { "type": "string", "description": "Topic name" },
                    "area": { "type": "string", "description": "Area name" }
                },
                "required": ["topic"]
            }),
        },
        // === Tasks ===
        Tool {
            name: "tasks_list".to_string(),
            description: "List all tasks for a topic with their status".to_string(),
            input_schema: json!({
                "type": "object",
                "properties": {
                    "topic": { "type": "string", "description": "Topic name" },
                    "area": { "type": "string", "description": "Area name (default: Working)" }
                },
                "required": ["topic"]
            }),
        },
        Tool {
            name: "tasks_complete".to_string(),
            description: "Mark a task as complete".to_string(),
            input_schema: json!({
                "type": "object",
                "properties": {
                    "topic": { "type": "string", "description": "Topic name" },
                    "task_index": { "type": "integer", "description": "Task index (0-based)" },
                    "note": { "type": "string", "description": "Optional note" },
                    "area": { "type": "string", "description": "Area name (default: Working)" }
                },
                "required": ["topic", "task_index"]
            }),
        },
        Tool {
            name: "tasks_incomplete".to_string(),
            description: "Mark a task as incomplete".to_string(),
            input_schema: json!({
                "type": "object",
                "properties": {
                    "topic": { "type": "string", "description": "Topic name" },
                    "task_index": { "type": "integer", "description": "Task index (0-based)" },
                    "note": { "type": "string", "description": "Optional note" },
                    "area": { "type": "string", "description": "Area name (default: Working)" }
                },
                "required": ["topic", "task_index"]
            }),
        },
        // === Notes ===
        Tool {
            name: "notes_read".to_string(),
            description: "Read notes section from task.md".to_string(),
            input_schema: json!({
                "type": "object",
                "properties": {
                    "topic": { "type": "string", "description": "Topic name" },
                    "area": { "type": "string", "description": "Area name (default: Working)" }
                },
                "required": ["topic"]
            }),
        },
        Tool {
            name: "notes_add".to_string(),
            description: "Add a note to the notes section of task.md".to_string(),
            input_schema: json!({
                "type": "object",
                "properties": {
                    "topic": { "type": "string", "description": "Topic name" },
                    "note": { "type": "string", "description": "Note content to add" },
                    "area": { "type": "string", "description": "Area name (default: Working)" }
                },
                "required": ["topic", "note"]
            }),
        },
        // === Spec Add (creates from templates) ===
        Tool {
            name: "spec_add".to_string(),
            description: "Create spec.md and task.md files from templates for a topic".to_string(),
            input_schema: json!({
                "type": "object",
                "properties": {
                    "topic": { "type": "string", "description": "Topic name" },
                    "area": { "type": "string", "description": "Area name (default: Working)" }
                },
                "required": ["topic"]
            }),
        },
        // === Spec Writing ===
        Tool {
            name: "spec_write".to_string(),
            description: "Write spec.md content for a topic".to_string(),
            input_schema: json!({
                "type": "object",
                "properties": {
                    "topic": { "type": "string", "description": "Topic name" },
                    "area": { "type": "string", "description": "Area name (default: Working)" },
                    "content": { "type": "string", "description": "Full spec.md content" }
                },
                "required": ["topic", "content"]
            }),
        },
        Tool {
            name: "spec_read".to_string(),
            description: "Read spec.md content for a topic".to_string(),
            input_schema: json!({
                "type": "object",
                "properties": {
                    "topic": { "type": "string", "description": "Topic name" },
                    "area": { "type": "string", "description": "Area name (default: Working)" }
                },
                "required": ["topic"]
            }),
        },
        // === Task Writing ===
        Tool {
            name: "task_write".to_string(),
            description: "Write task.md content for a topic".to_string(),
            input_schema: json!({
                "type": "object",
                "properties": {
                    "topic": { "type": "string", "description": "Topic name" },
                    "area": { "type": "string", "description": "Area name (default: Working)" },
                    "content": { "type": "string", "description": "Full task.md content" }
                },
                "required": ["topic", "content"]
            }),
        },
        Tool {
            name: "task_read".to_string(),
            description: "Read task.md content for a topic".to_string(),
            input_schema: json!({
                "type": "object",
                "properties": {
                    "topic": { "type": "string", "description": "Topic name" },
                    "area": { "type": "string", "description": "Area name (default: Working)" }
                },
                "required": ["topic"]
            }),
        },
        // === Task Queue (Master Task) ===
        Tool {
            name: "queue_list".to_string(),
            description: "List the task queue (ordered list of topics to work on)".to_string(),
            input_schema: json!({
                "type": "object",
                "properties": {
                    "area": { "type": "string", "description": "Area name (default: Working)" }
                }
            }),
        },
        Tool {
            name: "queue_add".to_string(),
            description: "Add a topic to the task queue".to_string(),
            input_schema: json!({
                "type": "object",
                "properties": {
                    "topic": { "type": "string", "description": "Topic name to add" },
                    "position": { "type": "integer", "description": "Position in queue (0=first, -1=last)" },
                    "area": { "type": "string", "description": "Area name (default: Working)" }
                },
                "required": ["topic"]
            }),
        },
        Tool {
            name: "queue_remove".to_string(),
            description: "Remove a topic from the task queue".to_string(),
            input_schema: json!({
                "type": "object",
                "properties": {
                    "topic": { "type": "string", "description": "Topic name to remove" },
                    "area": { "type": "string", "description": "Area name (default: Working)" }
                },
                "required": ["topic"]
            }),
        },
        Tool {
            name: "queue_reorder".to_string(),
            description: "Reorder topics in the task queue".to_string(),
            input_schema: json!({
                "type": "object",
                "properties": {
                    "topic": { "type": "string", "description": "Topic to move" },
                    "new_position": { "type": "integer", "description": "New position (0-based)" },
                    "area": { "type": "string", "description": "Area name (default: Working)" }
                },
                "required": ["topic", "new_position"]
            }),
        },
        // === Index Actions (Bind) ===
        Tool {
            name: "index_add".to_string(),
            description: "Add a link between a topic and a file path".to_string(),
            input_schema: json!({
                "type": "object",
                "properties": {
                    "topic": { "type": "string", "description": "Topic name" },
                    "path": { "type": "string", "description": "File path to link" },
                    "area": { "type": "string", "description": "Area (default: Working)" },
                    "link_type": { "type": "string", "description": "Link type (implementation, test, config, docs)" },
                    "tags": { "type": "string", "description": "Comma-separated tags" },
                    "annotation": { "type": "string", "description": "Brief annotation" }
                },
                "required": ["topic", "path"]
            }),
        },
        Tool {
            name: "unispec_bind_spec".to_string(),
            description: "Bind a code file to a spec".to_string(),
            input_schema: json!({
                "type": "object",
                "properties": {
                    "spec_path": { "type": "string", "description": "Spec file path" },
                    "file_path": { "type": "string", "description": "Code file path to bind" },
                    "topic": { "type": "string", "description": "Topic name" },
                    "area": { "type": "string", "description": "Area name (default: Working)" }
                },
                "required": ["spec_path", "file_path", "topic"]
            }),
        },
        // === Index Queries ===
        Tool {
            name: "index_find".to_string(),
            description: "Find links by topic, path, or tag".to_string(),
            input_schema: json!({
                "type": "object",
                "properties": {
                    "query": { "type": "string", "description": "Search query" },
                    "by": { "type": "string", "enum": ["topic", "path", "tag"], "description": "Search by (default: topic)" }
                },
                "required": ["query"]
            }),
        },
        Tool {
            name: "index_lookup".to_string(),
            description: "Find export by full ID (e.g., user-login:login_user)".to_string(),
            input_schema: json!({
                "type": "object",
                "properties": {
                    "id": { "type": "string", "description": "Export ID (topic:name)" }
                },
                "required": ["id"]
            }),
        },
        // === Index Listing ===
        Tool {
            name: "index_list".to_string(),
            description: "List all index links with optional filters".to_string(),
            input_schema: json!({
                "type": "object",
                "properties": {
                    "topic": { "type": "string", "description": "Filter by topic" },
                    "path": { "type": "string", "description": "Filter by path" },
                    "tag": { "type": "string", "description": "Filter by tag" }
                }
            }),
        },
        Tool {
            name: "index_graph".to_string(),
            description: "Export index as graph JSON for visualization".to_string(),
            input_schema: json!({
                "type": "object",
                "properties": {}
            }),
        },
        Tool {
            name: "index_backlinks".to_string(),
            description: "Generate backlinks for a topic".to_string(),
            input_schema: json!({
                "type": "object",
                "properties": {
                    "topic": { "type": "string", "description": "Topic name" }
                },
                "required": ["topic"]
            }),
        },
    ]
}
