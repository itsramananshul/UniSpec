// src/commands/queue.rs
//
// Shared logic for the readiness queue (`spec/<area>/queue.md`). Used by both
// the CLI (`unispec queue add`) and the MCP server (`queue_add` tool).
//
// Behaviour preserved verbatim from the original MCP handler:
// - The queue file name comes from the mode config (default: `queue.md`).
// - If the queue file doesn't exist, a fresh one is created with a standard
//   header.
// - `position < 0` or `position >= items.len()` appends to the end.
//   Anything else inserts at the 0-based slot.
// - Existing entries are preserved in their original order.

use anyhow::Result;

pub struct QueueAddOutput {
    pub topic: String,
    pub area: String,
    pub position: i32,
    pub queue_file: String,
}

pub fn run_queue_add(topic: &str, area: &str, position: i32) -> Result<QueueAddOutput> {
    let queue_file = crate::agent::mode::get_readiness_queue_file();
    let queue_path = crate::fs::spec_dir().join(area).join(&queue_file);

    let existing = if queue_path.exists() {
        std::fs::read_to_string(&queue_path)?
    } else {
        String::new()
    };

    let mut items: Vec<String> = existing
        .lines()
        .filter(|l| l.trim().starts_with("- "))
        .map(|l| l.trim().trim_start_matches("- ").to_string())
        .collect();

    if position < 0 || position as usize >= items.len() {
        items.push(topic.to_string());
    } else {
        items.insert(position as usize, topic.to_string());
    }

    let header = "# Task Queue\n\nOrdered list of topics to work on:\n";
    let mut content = String::from(header);
    for item in &items {
        content.push_str(&format!("- {}\n", item));
    }

    // Ensure the parent area directory exists so the queue file write
    // doesn't fail on a fresh project where the area was just created.
    if let Some(parent) = queue_path.parent() {
        crate::fs::ensure_dir(parent)?;
    }
    std::fs::write(&queue_path, content)?;

    Ok(QueueAddOutput {
        topic: topic.to_string(),
        area: area.to_string(),
        position,
        queue_file,
    })
}
