// src/fs/spec.rs
use anyhow::Result;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SpecMetadata {
    pub title: Option<String>,
    pub impact: Option<String>,
    pub change_type: Option<String>,
    pub status: Option<String>,
    pub created: Option<String>,
    pub modified: Option<String>,
    pub completed: Option<String>,
    pub checked_out: Option<String>,
    pub checked_out_at: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoadmapItem {
    pub topic: String,
    pub path: String,
    pub metadata: SpecMetadata,
}

pub fn parse_spec_metadata(content: &str) -> Option<SpecMetadata> {
    if !content.contains("---") {
        return None;
    }

    let lines: Vec<&str> = content.lines().collect();
    if lines.len() < 3 {
        return None;
    }

    let mut metadata = SpecMetadata::default();
    let mut in_frontmatter = false;
    let mut frontmatter_lines = Vec::new();

    for line in lines {
        let trimmed = line.trim();
        if trimmed == "---" {
            if !in_frontmatter {
                in_frontmatter = true;
                continue;
            } else {
                break;
            }
        }
        if in_frontmatter {
            frontmatter_lines.push(trimmed);
        }
    }

    for line in frontmatter_lines {
        if let Some((key, value)) = line.split_once(':') {
            let key = key.trim().to_lowercase();
            let value = value.trim().to_string();
            match key.as_str() {
                "title" => metadata.title = Some(value),
                "impact" => metadata.impact = Some(value.to_lowercase()),
                "change_type" | "changetype" | "change-type" => {
                    metadata.change_type = Some(value.to_lowercase())
                }
                "status" => metadata.status = Some(value.to_lowercase()),
                "created" => metadata.created = Some(value),
                "modified" => metadata.modified = Some(value),
                "completed" => metadata.completed = Some(value),
                "checked_out" => metadata.checked_out = Some(value),
                "checked_out_at" => metadata.checked_out_at = Some(value),
                _ => {}
            }
        }
    }

    // Return Some if any metadata exists (title, impact, change_type, or status)
    if metadata.title.is_none()
        && metadata.impact.is_none()
        && metadata.change_type.is_none()
        && metadata.status.is_none()
    {
        return None;
    }

    Some(metadata)
}

pub fn parse_roadmap_items(dir_path: &std::path::Path, spec_file: &str) -> Vec<RoadmapItem> {
    let mut items = Vec::new();

    if !dir_path.exists() {
        return items;
    }

    for entry in walkdir::WalkDir::new(dir_path)
        .max_depth(1)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let path = entry.path();
        if path.is_dir() {
            let spec_path = path.join(spec_file);
            if spec_path.exists() {
                if let Ok(content) = std::fs::read_to_string(&spec_path) {
                    if let Some(metadata) = parse_spec_metadata(&content) {
                        let topic = path
                            .file_name()
                            .map(|n| n.to_string_lossy().to_string())
                            .unwrap_or_default();
                        items.push(RoadmapItem {
                            topic,
                            path: spec_path.to_string_lossy().to_string(),
                            metadata,
                        });
                    }
                }
            }
        }
    }

    items
}

pub fn extract_title_from_markdown(content: &str) -> Option<String> {
    for line in content.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with("# ") {
            return Some(trimmed[2..].trim().to_string());
        }
    }
    None
}

pub fn get_impact_color(impact: &str) -> &str {
    match impact.to_lowercase().as_str() {
        "critical" => "red",
        "high" => "yellow",
        "medium" => "cyan",
        "low" => "white",
        _ => "white",
    }
}

pub fn format_impact_badge(impact: &str) -> String {
    match impact.to_lowercase().as_str() {
        "critical" => "[CRITICAL]".to_string(),
        "high" => "[HIGH]".to_string(),
        "medium" => "[MEDIUM]".to_string(),
        "low" => "[LOW]".to_string(),
        _ => "[----]".to_string(),
    }
}

pub fn format_change_type_badge(change_type: &str) -> String {
    match change_type.to_lowercase().as_str() {
        "feature" => "feature".to_string(),
        "bugfix" | "bug" => "bugfix".to_string(),
        "refactor" | "refactoring" => "refactor".to_string(),
        "documentation" | "docs" => "docs".to_string(),
        "security" => "security".to_string(),
        _ => "unknown".to_string(),
    }
}

pub fn is_checked_out(metadata: &SpecMetadata, agent_id: &str) -> bool {
    if let Some(ref checked_out_by) = metadata.checked_out {
        return checked_out_by != agent_id;
    }
    false
}

pub fn format_checkout_status(metadata: &SpecMetadata) -> String {
    if let Some(ref agent) = metadata.checked_out {
        if let Some(ref time) = metadata.checked_out_at {
            return format!("[checked out by {} at {}]", agent, time);
        }
        return format!("[checked out by {}]", agent);
    }
    String::new()
}

pub fn update_spec_with_checkout(content: &str, agent_id: &str, checked_out: bool) -> String {
    let today = chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string();

    if !content.starts_with("---") {
        return content.to_string();
    }

    if let Some(end_idx) = content.find("\n---\n").or_else(|| content.find("\n--- ")) {
        let frontmatter = &content[3..end_idx];
        let rest = &content[end_idx + 4..];

        let mut lines: Vec<String> = frontmatter.lines().map(|l| l.to_string()).collect();

        let mut has_checked_out = false;
        let mut has_checked_out_at = false;
        let mut has_modified = false;

        for line in &lines {
            let trimmed = line.trim();
            if trimmed.starts_with("checked_out:") {
                has_checked_out = true;
            } else if trimmed.starts_with("checked_out_at:") {
                has_checked_out_at = true;
            } else if trimmed.starts_with("modified:") {
                has_modified = true;
            }
        }

        for line in &mut lines {
            let trimmed = line.trim();
            if trimmed.starts_with("checked_out:") {
                if checked_out {
                    *line = format!("checked_out: {}", agent_id);
                } else {
                    *line = "checked_out:".to_string();
                }
                has_checked_out = true;
            } else if trimmed.starts_with("checked_out_at:") {
                if checked_out {
                    *line = format!("checked_out_at: {}", today);
                } else {
                    *line = "checked_out_at:".to_string();
                }
                has_checked_out_at = true;
            } else if trimmed.starts_with("modified:") {
                *line = format!("modified: {}", today);
                has_modified = true;
            }
        }

        if !has_checked_out {
            if checked_out {
                lines.push(format!("checked_out: {}", agent_id));
            }
        }
        if !has_checked_out_at {
            if checked_out {
                lines.push(format!("checked_out_at: {}", today));
            }
        }
        if !has_modified {
            lines.push(format!("modified: {}", today));
        }

        format!("---\n{}\n---\n{}", lines.join("\n"), rest)
    } else {
        content.to_string()
    }
}

pub fn can_push_pull(metadata: &SpecMetadata, agent_id: &str) -> Result<bool, String> {
    if let Some(ref checked_out_by) = metadata.checked_out {
        if checked_out_by != agent_id {
            return Err(format!(
                "Topic is checked out by {}. Cannot push/pull until checked back in.",
                checked_out_by
            ));
        }
    }
    Ok(true)
}

pub fn strip_checkout_metadata(content: &str) -> String {
    if !content.starts_with("---") {
        return content.to_string();
    }

    if let Some(end_idx) = content.find("\n---\n").or_else(|| content.find("\n--- ")) {
        let frontmatter = &content[3..end_idx];
        let rest = &content[end_idx + 4..];

        let lines: Vec<String> = frontmatter
            .lines()
            .filter(|line| {
                let trimmed = line.trim();
                !trimmed.starts_with("checked_out:") && !trimmed.starts_with("checked_out_at:")
            })
            .map(|l| l.to_string())
            .collect();

        format!("---\n{}\n---\n{}", lines.join("\n"), rest)
    } else {
        content.to_string()
    }
}
