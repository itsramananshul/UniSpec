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
                _ => {}
            }
        }
    }

    if metadata.title.is_none() && metadata.impact.is_none() && metadata.change_type.is_none() {
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
