// src/commands/topic.rs
use crate::fs::{config::load_config, ensure_dir, topic_path};
use anyhow::Result;
use std::fs;

pub fn run_new(topic: &str, area_str: &str) -> Result<String> {
    let topic_path = topic_path(topic, area_str);

    if topic_path.exists() {
        return Err(anyhow::anyhow!(
            "❌ Topic '{}' already exists in {}.",
            topic,
            area_str
        ));
    }

    ensure_dir(&topic_path)?;

    // Get templates from current mode
    let specs_template = crate::fs::read_template("specs.md")
        .unwrap_or_else(|| "# Design: <TopicName>\n\n## Overview\n> [Description]\n".to_string());
    let tasks_template = crate::fs::read_template("tasks.md")
        .unwrap_or_else(|| "# Tasks: <TopicName>\n\n## Tasks\n- [ ] Task 1\n".to_string());

    // Create specs.md and tasks.md from templates
    fs::write(topic_path.join("specs.md"), &specs_template)?;
    fs::write(topic_path.join("tasks.md"), &tasks_template)?;

    Ok(format!("✅ Topic '{}' created in {}/", topic, area_str))
}

pub fn run_list(area_str: &str, _show_status: bool) -> Result<()> {
    let area_path = crate::fs::spec_dir().join(area_str);

    if !area_path.exists() {
        return Err(anyhow::anyhow!("❌ Area '{}' does not exist.", area_str));
    }

    let mut topics = vec![];
    for entry in fs::read_dir(&area_path)? {
        let entry = entry?;
        if entry.path().is_dir() {
            let name = entry.file_name();
            let area_path_file = entry.path().join("specs.md");
            let tasks_path = entry.path().join("tasks.md");

            let status = if area_path_file.exists() {
                let mut all_checked = true;
                if tasks_path.exists() {
                    if let Ok(content) = std::fs::read_to_string(&tasks_path) {
                        for line in content.lines() {
                            if line.trim().starts_with("- [ ]") {
                                all_checked = false;
                                break;
                            }
                        }
                    }
                }
                if all_checked {
                    "complete"
                } else {
                    "in-progress"
                }
            } else {
                "draft"
            };

            topics.push((name.to_string_lossy().to_string(), status.to_string()));
        }
    }

    println!("Topics in {}:", area_str);
    for (topic, status) in topics {
        println!("✅ {} ({})", topic, status);
    }
    Ok(())
}

pub fn run_push(topic: &str, target_area: &str, source_area: Option<&str>) -> Result<String> {
    let source_area = source_area.map(String::from).unwrap_or_else(|| {
        load_config()
            .map(|c| c.area)
            .unwrap_or_else(|_| "Working".to_string())
    });

    if source_area == target_area {
        return Err(anyhow::anyhow!(
            "❌ Source and target areas are the same: {}",
            source_area
        ));
    }

    let src = topic_path(topic, &source_area);
    let dst = topic_path(topic, target_area);

    if !src.exists() {
        return Err(anyhow::anyhow!(
            "❌ Topic '{}' not found in {}.",
            topic,
            source_area
        ));
    }

    if dst.exists() {
        fs::remove_dir_all(&dst)?;
    }

    // Copy directory recursively
    copy_dir_recursive(&src, &dst)?;

    Ok(format!(
        "✅ Pushed topic '{}' from {} to {}",
        topic, source_area, target_area
    ))
}

fn copy_dir_recursive(src: &std::path::Path, dst: &std::path::Path) -> Result<()> {
    ensure_dir(dst)?;
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let path = entry.path();
        let dest_path = dst.join(entry.file_name());
        if path.is_file() {
            fs::copy(&path, &dest_path)?;
        } else if path.is_dir() {
            copy_dir_recursive(&path, &dest_path)?;
        }
    }
    Ok(())
}

pub fn run_pull(topic: &str, source_area: &str) -> Result<String> {
    let current_config = load_config()?;
    let target_area = current_config.area;

    let src = topic_path(topic, source_area);
    let dst = topic_path(topic, &target_area);

    if !src.exists() {
        return Err(anyhow::anyhow!(
            "❌ Topic '{}' not found in {}.",
            topic,
            source_area
        ));
    }

    if dst.exists() {
        return Err(anyhow::anyhow!(
            "❌ Topic '{}' already exists in {}.",
            topic,
            target_area
        ));
    }

    // Copy directory recursively
    copy_dir_recursive(&src, &dst)?;

    Ok(format!(
        "✅ Pulled topic '{}' from {} to {}",
        topic, source_area, target_area
    ))
}

pub fn run_delete(topic: &str, area: &str, _force: bool) -> Result<String> {
    let path = topic_path(topic, area);
    if !path.exists() {
        return Err(anyhow::anyhow!(
            "❌ Topic '{}' not found in {}.",
            topic,
            area
        ));
    }

    // Remove topic dir
    fs::remove_dir_all(&path)?;

    Ok(format!("✅ Deleted topic '{}' from {}/", topic, area))
}

pub fn run_progress(area_str: Option<&str>) -> Result<()> {
    let current_config = load_config()?;
    let area = area_str.unwrap_or(&current_config.area);
    let area_path = crate::fs::spec_dir().join(area);

    if !area_path.exists() {
        return Err(anyhow::anyhow!("❌ Area '{}' does not exist.", area));
    }

    let mut complete = 0;
    let mut in_progress = 0;
    let mut draft = 0;

    for entry in fs::read_dir(&area_path)? {
        let entry = entry?;
        if entry.path().is_dir() {
            let area_path_file = entry.path().join("specs.md");
            let tasks_path = entry.path().join("tasks.md");

            if !area_path_file.exists() {
                draft += 1;
                println!(
                    "⚠️  {} (draft — no area.md)",
                    entry.file_name().to_string_lossy()
                );
                continue;
            }

            let content = std::fs::read_to_string(&tasks_path)?;
            let mut all_checked = true;
            for line in content.lines() {
                if line.starts_with("- [ ]") {
                    all_checked = false;
                    break;
                }
            }

            if all_checked {
                complete += 1;
                println!("✅ {} (complete)", entry.file_name().to_string_lossy());
            } else {
                in_progress += 1;
                println!("⏳ {} (in-progress)", entry.file_name().to_string_lossy());
            }
        }
    }

    println!("\n📊 Summary:");
    println!("  🟢 Complete: {}", complete);
    println!("  🟡 In Progress: {}", in_progress);
    println!("  🟠 Draft: {}", draft);

    Ok(())
}

pub fn run_show(topic: &str) -> Result<()> {
    let current_config = load_config()?;
    let area = current_config.area;

    let topic_path = topic_path(topic, area.as_str());
    let area_path = topic_path.join("specs.md");
    let tasks_path = topic_path.join("tasks.md");

    if !area_path.exists() {
        return Err(anyhow::anyhow!(
            "❌ Topic '{}' not found in {}.",
            topic,
            area
        ));
    }

    println!("📝 Area: {}", area_path.display());

    // Read first few lines of area.md
    if let Ok(content) = std::fs::read_to_string(&area_path) {
        for line in content.lines().take(5) {
            println!("  {}", line);
        }
    }

    println!("\n📋 Tasks:");
    let mut all_checked = true;
    if tasks_path.exists() {
        if let Ok(content) = std::fs::read_to_string(&tasks_path) {
            for line in content.lines() {
                if line.starts_with("- [ ]") {
                    all_checked = false;
                    println!("  ❌ [ ] {}", &line[6..]);
                } else if line.starts_with("- [x]") || line.starts_with("- [X]") {
                    println!("  ✅ [x] {}", &line[6..]);
                }
            }
        }
    } else {
        println!("  ℹ️  No tasks.md file");
    }

    let status = if all_checked {
        "complete"
    } else {
        "in-progress"
    };
    println!("\n📊 Status: {}", status);

    Ok(())
}
