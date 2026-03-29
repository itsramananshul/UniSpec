// src/commands/area.rs
use crate::fs::{area_exists, list_areas, rename_area};
use anyhow::Result;
use std::collections::HashSet;
use std::fs;

fn get_current_area() -> String {
    crate::agent::load_agent_config()
        .ok()
        .and_then(|c| c.default_area)
        .unwrap_or_else(|| "Working".to_string())
}

pub fn run_add(name: &str) -> Result<String> {
    let spec_area = crate::fs::spec_dir().join(name);

    if spec_area.exists() {
        return Err(anyhow::anyhow!("Area '{}' already exists.", name));
    }

    crate::fs::ensure_dir(&spec_area)?;
    fs::write(spec_area.join("area.md"), create_area_md(name))?;

    Ok(format!("Area '{}' added!", name))
}

pub fn create_area_md(area_name: &str) -> String {
    let areas_dir = crate::fs::global_config_dir().join("areas");
    let template_path = areas_dir.join(area_name.to_lowercase()).join("area.md");
    if template_path.exists() {
        if let Ok(content) = std::fs::read_to_string(&template_path) {
            return content;
        }
    }
    format!(
        r#"# {}

## Purpose

[Description of this area's role]

## Guidelines

- [Guideline 1]
- [Guideline 2]
"#,
        area_name
    )
}

pub fn run_remove(name: &str) -> Result<String> {
    if name.is_empty() {
        return Err(anyhow::anyhow!("No area specified."));
    }

    let spec_dir = crate::fs::spec_dir().join(name);

    if !spec_dir.exists() {
        return Err(anyhow::anyhow!("Area '{}' does not exist.", name));
    }

    // Check if area is protected
    if let Ok(protected) = get_protected_areas() {
        if protected.contains(&name.to_string()) {
            return Err(anyhow::anyhow!(
                "Area '{}' is protected and cannot be removed.",
                name
            ));
        }
    }

    // Check if area has topics
    let has_topics =
        fs::read_dir(&spec_dir)?.any(|e| e.map(|e| e.path().is_dir()).unwrap_or(false));
    if has_topics {
        return Err(anyhow::anyhow!(
            "Area '{}' has topics. Cannot remove.",
            name
        ));
    }

    fs::remove_dir_all(&spec_dir)?;

    // Clear default_area in config if it references this area
    if let Ok(mut config) = crate::agent::load_agent_config() {
        if config.default_area.as_deref() == Some(name) {
            config.default_area = Some("Working".to_string());
            let _ = crate::agent::save_agent_config(&config);
        }
    }

    Ok(format!("Area '{}' removed.", name))
}

fn get_protected_areas() -> Result<Vec<String>> {
    crate::agent::get_protected_areas()
}

pub fn run_list() -> Result<()> {
    let ordered_areas = crate::agent::mode::get_area_order();
    let spec_dir = crate::fs::spec_dir();
    let mut existing_areas = std::collections::HashSet::new();

    if spec_dir.exists() {
        for entry in std::fs::read_dir(spec_dir)? {
            let entry = entry?;
            if entry.path().is_dir() {
                let area_filename = crate::agent::mode::get_area_filename();
                if entry.path().join(&area_filename).exists() {
                    existing_areas.insert(entry.file_name().to_string_lossy().to_string());
                }
            }
        }
    }

    let areas: Vec<String> = if !ordered_areas.is_empty() {
        let mut ordered: Vec<String> = ordered_areas
            .into_iter()
            .filter(|a| existing_areas.contains(a))
            .collect();
        let ordered_count = ordered.len();
        for area in &existing_areas {
            if !ordered.contains(area) {
                ordered.push(area.clone());
            }
        }
        // Sort only the extra areas (after the ordered ones)
        if ordered.len() > ordered_count {
            let extra = ordered.split_off(ordered_count);
            let mut sorted_extra = extra;
            sorted_extra.sort_by(|a, b| a.to_lowercase().cmp(&b.to_lowercase()));
            ordered.extend(sorted_extra);
        }
        ordered
    } else {
        let mut sorted: Vec<String> = existing_areas.into_iter().collect();
        sorted.sort_by(|a, b| a.to_lowercase().cmp(&b.to_lowercase()));
        sorted
    };

    let current = get_current_area();

    println!("Areas:");
    for area in areas {
        if area == current {
            println!("{} (current)", area);
        } else {
            println!("{}", area);
        }
    }
    Ok(())
}

pub fn run_rename(old: &str, new: &str) -> Result<()> {
    if !area_exists(old) {
        return Err(anyhow::anyhow!("Area '{}' does not exist.", old));
    }
    if area_exists(new) {
        return Err(anyhow::anyhow!("Area '{}' already exists.", new));
    }

    rename_area(old, new)?;
    println!("Area '{}' renamed to '{}'.", old, new);
    Ok(())
}

pub fn run_default(name: &str) -> Result<()> {
    if !area_exists(name) {
        return Err(anyhow::anyhow!("Area '{}' does not exist.", name));
    }

    let mut config = crate::agent::load_agent_config()?;
    config.default_area = Some(name.to_string());
    crate::agent::save_agent_config(&config)?;
    println!("Default area set to '{}'.", name);
    Ok(())
}

pub fn run_health() -> Result<()> {
    let areas = crate::fs::list_areas()?;

    for area in areas {
        let area_path = crate::fs::spec_dir().join(&area);
        if !area_path.exists() {
            continue;
        }

        let mut complete = 0;
        let mut in_progress = 0;
        let mut draft = 0;

        for entry in fs::read_dir(&area_path)? {
            let entry: std::fs::DirEntry = entry?;
            if entry.path().is_dir() {
                let tasks_path = entry.path().join("tasks.md");

                if !tasks_path.exists() {
                    draft += 1;
                    continue;
                }

                let content = std::fs::read_to_string(&tasks_path)?;

                let mut unchecked = 0;
                let mut checked = 0;
                let mut ip = 0;

                for line in content.lines() {
                    let trimmed = line.trim();
                    if trimmed.starts_with("- [ ]") {
                        unchecked += 1;
                    } else if trimmed.starts_with("- [x]") || trimmed.starts_with("- [X]") {
                        checked += 1;
                    } else if trimmed.starts_with("- [-]") {
                        ip += 1;
                    }
                }

                let total_tasks = unchecked + checked + ip;

                if total_tasks == 0 {
                    draft += 1;
                } else if checked > 0 && unchecked == 0 && ip == 0 {
                    complete += 1;
                } else if ip > 0 || (checked > 0 && unchecked > 0) {
                    in_progress += 1;
                } else {
                    draft += 1;
                }
            }
        }

        let current = if area == get_current_area() {
            " (current)"
        } else {
            ""
        };

        println!("{}:{}:", area, current);
        println!("  Complete: {}", complete);
        println!("  In Progress: {}", in_progress);
        println!("  Draft: {}", draft);
        println!();
    }

    Ok(())
}

pub fn run_area_order_show() -> Result<String> {
    let order = crate::agent::mode::get_area_order();

    if order.is_empty() {
        return Ok(
            "No custom area order defined. Areas will be sorted alphabetically.".to_string(),
        );
    }

    let content: String = order
        .iter()
        .enumerate()
        .map(|(i, a)| format!("{}. {}", i + 1, a))
        .collect::<Vec<_>>()
        .join("\n");

    Ok(content)
}

pub fn run_area_order_add(areas: Vec<String>, position: Option<usize>) -> Result<String> {
    crate::agent::mode::add_to_area_order(areas.clone(), position)?;

    let order = crate::agent::mode::get_area_order();
    let order_content: String = order
        .iter()
        .enumerate()
        .map(|(i, a)| format!("{}. {}", i + 1, a))
        .collect::<Vec<_>>()
        .join("\n");

    let pos_info = if let Some(p) = position {
        format!(" at position {}", p + 1)
    } else {
        String::new()
    };

    Ok(format!(
        "✅ Added {} area(s) to order{}.\n\nCurrent order:\n{}",
        areas.len(),
        pos_info,
        order_content
    ))
}

pub fn run_area_order_remove(areas: Vec<String>) -> Result<String> {
    crate::agent::mode::remove_from_area_order(areas.clone())?;

    let order = crate::agent::mode::get_area_order();
    let order_content: String = if order.is_empty() {
        "(alphabetical)".to_string()
    } else {
        order
            .iter()
            .enumerate()
            .map(|(i, a)| format!("{}. {}", i + 1, a))
            .collect::<Vec<_>>()
            .join("\n")
    };

    Ok(format!(
        "✅ Removed {} area(s) from order.\n\nCurrent order:\n{}",
        areas.len(),
        order_content
    ))
}

pub fn run_area_order_reset() -> Result<String> {
    crate::agent::mode::reset_area_order()?;
    Ok("✅ Reset area order to alphabetical.".to_string())
}
