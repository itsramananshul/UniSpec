// src/commands/area.rs
use crate::fs::{area_exists, list_areas, rename_area};
use anyhow::Result;
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
    let areas = list_areas()?;
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
