// src/commands/topic.rs
use crate::agent::load_agent_config;
use crate::fs::{config::load_config, ensure_dir, topic_path};
use anyhow::Result;
use std::fs;
use std::iter::repeat;

pub fn run_new(topic: &str, area_str: &str) -> Result<String> {
    run_new_with_metadata(topic, area_str, None, None)
}

pub fn run_new_with_metadata(
    topic: &str,
    area_str: &str,
    impact: Option<&str>,
    change_type: Option<&str>,
) -> Result<String> {
    let topic_path = topic_path(topic, area_str);

    if topic_path.exists() {
        return Err(anyhow::anyhow!(
            "❌ Topic '{}' already exists in {}.",
            topic,
            area_str
        ));
    }

    ensure_dir(&topic_path)?;

    let spec_filename = crate::agent::mode::get_spec_filename_for_area(area_str);
    let task_filename = crate::agent::mode::get_task_filename_for_area(area_str);

    let today = chrono::Local::now().format("%Y-%m-%d").to_string();

    let spec_content = if impact.is_some() || change_type.is_some() {
        let impact_line = impact.map(|i| format!("impact: {}", i)).unwrap_or_default();
        let change_type_line = change_type
            .map(|ct| format!("change_type: {}", ct))
            .unwrap_or_default();
        let status_line = "status: proposed".to_string();

        format!(
            "---\ntitle: {}\n{}\n{}\n{}\ncreated: {}\n---\n\n# {}\n\n## Problem Statement\n\nWhat problem does this solve?\n\n## Requirements\n\n### Must Have\n- [ ] Requirement 1\n- [ ] Requirement 2\n\n## Acceptance Criteria\n\n- [ ] Criterion 1\n- [ ] Criterion 2\n",
            topic,
            impact_line,
            change_type_line,
            status_line,
            today,
            topic
        )
    } else {
        let specs_template = crate::fs::read_area_template(area_str, "specs.md")
            .unwrap_or_else(|| read_default_specs_template());
        specs_template
    };

    let tasks_template = crate::fs::read_area_template(area_str, "tasks.md")
        .unwrap_or_else(|| read_default_tasks_template());

    fs::write(topic_path.join(&spec_filename), &spec_content)?;
    fs::write(topic_path.join(&task_filename), &tasks_template)?;

    let type_info = if let (Some(i), Some(ct)) = (impact, change_type) {
        format!(" ({}: {})", i, ct)
    } else {
        String::new()
    };

    Ok(format!(
        "✅ Topic '{}' created in {}/{}",
        topic, area_str, type_info
    ))
}

fn read_default_specs_template() -> String {
    "# [Topic Name]\n\n## Problem Statement\n\nWhat problem does this solve?\n\n## Requirements\n\n### Must Have\n- [ ] Requirement 1\n- [ ] Requirement 2\n\n## Acceptance Criteria\n\n- [ ] Criterion 1\n- [ ] Criterion 2\n".to_string()
}

fn read_default_tasks_template() -> String {
    "# Tasks - [Topic Name]\n\n## Tasks\n\n- [ ] Task 1\n- [ ] Task 2\n- [ ] Task 3\n\n## Notes\n\n- \n".to_string()
}

fn read_default_area_template() -> String {
    "# Area: [Area Name]\n\n## Purpose\n\nDescribe the purpose of this area in the workflow.\n\n## Entry Criteria\n\nWhat must be completed before entering this area?\n\n## Exit Criteria\n\nWhat must be completed to leave this area?\n\n## Notes\n\n- \n".to_string()
}

pub fn run_list(area_str: &str, _show_status: bool) -> Result<()> {
    let area_path = crate::fs::spec_dir().join(area_str);

    if !area_path.exists() {
        return Err(anyhow::anyhow!("❌ Area '{}' does not exist.", area_str));
    }

    // Get template file names from mode config for this area
    let spec_filename = crate::agent::mode::get_spec_filename_for_area(area_str);
    let task_filename = crate::agent::mode::get_task_filename_for_area(area_str);

    let mut topics = vec![];
    for entry in fs::read_dir(&area_path)? {
        let entry = entry?;
        if entry.path().is_dir() {
            let name = entry.file_name();
            let spec_path = entry.path().join(&spec_filename);
            let task_path = entry.path().join(&task_filename);

            let status = if spec_path.exists() {
                let mut all_checked = true;
                if task_path.exists() {
                    if let Ok(content) = std::fs::read_to_string(&task_path) {
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
    let current_config = load_agent_config()?;
    let source_area = source_area.map(String::from).unwrap_or_else(|| {
        current_config
            .default_area
            .clone()
            .unwrap_or_else(|| "Working".to_string())
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

    ensure_dir(&dst)?;

    // Get source and target file names from mode config
    let src_spec = crate::agent::mode::get_spec_filename_for_area(&source_area);
    let src_task = crate::agent::mode::get_task_filename_for_area(&source_area);
    let src_area_file = crate::agent::mode::get_area_filename_for_area(&source_area);

    let dst_spec = crate::agent::mode::get_spec_filename_for_area(target_area);
    let dst_task = crate::agent::mode::get_task_filename_for_area(target_area);
    let dst_area_file = crate::agent::mode::get_area_filename_for_area(target_area);

    // Copy files - keep source files AND create target area files from templates
    // Only copy specs and tasks, NOT area.md (area files stay in area root)
    for entry in fs::read_dir(&src)? {
        let entry = entry?;
        let path = entry.path();
        let filename = entry.file_name().to_string_lossy().to_string();

        // Skip area.md - it belongs to the area, not the topic
        if filename == "area.md" || filename.ends_with("_area.md") {
            continue;
        }

        // First, copy with original filename (keep source area files)
        let orig_dest = dst.join(&filename);
        if path.is_file() {
            fs::copy(&path, &orig_dest)?;
        } else if path.is_dir() {
            copy_dir_recursive(&path, &orig_dest)?;
        }

        // Second, if this is a spec or task file, create target version from template
        let is_spec_file = filename == src_spec
            || filename == dst_spec
            || filename.ends_with("_spec.md")
            || filename.ends_with("_specs.md")
            || filename == "spec.md"
            || filename == "specs.md";
        let is_task_file = filename == src_task
            || filename == dst_task
            || filename.ends_with("_tasks.md")
            || filename == "tasks.md";

        if !is_spec_file && !is_task_file {
            continue;
        }

        let target_filename = if is_spec_file {
            dst_spec.clone()
        } else {
            dst_task.clone()
        };

        // Only create target file if it has a different name
        if target_filename != filename || is_spec_file {
            let target_dest = dst.join(&target_filename);

            let mut content: String;

            // Check if pushing to Build and it's a spec file
            let is_build = target_area.to_lowercase() == "build";

            if is_build && is_spec_file {
                // Read source file and add completion metadata
                content = fs::read_to_string(&path)?;
                let today = chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
                content = add_completion_metadata(&content, &today);
                fs::write(&target_dest, &content)?;
            } else if path.is_file() {
                fs::copy(&path, &target_dest)?;
            }
        }
    }

    let completed_info = if target_area.to_lowercase() == "build" {
        let today = chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
        format!(" (completed: {})", today)
    } else {
        String::new()
    };

    Ok(format!(
        "✅ Pushed '{}' from {} → {}{}",
        topic, source_area, target_area, completed_info
    ))
}

fn add_completion_metadata(content: &str, completed_date: &str) -> String {
    let today = chrono::Local::now().format("%Y-%m-%d").to_string();

    // Find if there's frontmatter
    if !content.starts_with("---") {
        return content.to_string();
    }

    // Find the end of frontmatter
    if let Some(end_idx) = content.find("\n---\n").or_else(|| content.find("\n--- ")) {
        let frontmatter = &content[3..end_idx];
        let rest = &content[end_idx + 4..];

        // Parse existing frontmatter
        let mut lines: Vec<String> = frontmatter.lines().map(|l| l.to_string()).collect();

        // Update or add fields
        let mut has_status = false;
        let mut has_modified = false;
        let mut has_completed = false;
        let mut has_created = false;

        for line in &lines {
            let trimmed = line.trim();
            if trimmed.starts_with("status:") {
                has_status = true;
            } else if trimmed.starts_with("modified:") {
                has_modified = true;
            } else if trimmed.starts_with("completed:") {
                has_completed = true;
            } else if trimmed.starts_with("created:") {
                has_created = true;
            }
        }

        // Update lines in place or add new ones
        for line in &mut lines {
            let trimmed = line.trim();
            if trimmed.starts_with("status:") {
                *line = format!("status: completed");
                has_status = true;
            } else if trimmed.starts_with("modified:") {
                *line = format!("modified: {}", today);
                has_modified = true;
            } else if trimmed.starts_with("completed:") {
                *line = format!("completed: {}", completed_date);
                has_completed = true;
            }
        }

        // Add missing fields
        if !has_status {
            lines.push("status: completed".to_string());
        }
        if !has_modified {
            lines.push(format!("modified: {}", today));
        }
        if !has_completed {
            lines.push(format!("completed: {}", completed_date));
        }
        if !has_created {
            lines.push(format!("created: {}", today));
        }

        // Reconstruct
        format!("---\n{}\n---\n{}", lines.join("\n"), rest)
    } else {
        // No proper frontmatter, add at the top
        format!(
            "---\ntitle: Topic\nstatus: completed\ncreated: {}\nmodified: {}\ncompleted: {}\n---\n\n{}",
            today, today, completed_date, content
        )
    }
}

pub fn run_pull(topic: &str, source_area: &str) -> Result<String> {
    let current_config = load_agent_config()?;
    let target_area = current_config
        .default_area
        .unwrap_or_else(|| "Working".to_string());

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
        fs::remove_dir_all(&dst)?;
    }

    ensure_dir(&dst)?;

    // Get source and target file names from mode config
    let src_spec = crate::agent::mode::get_spec_filename_for_area(source_area);
    let src_task = crate::agent::mode::get_task_filename_for_area(source_area);
    let src_area_file = crate::agent::mode::get_area_filename_for_area(source_area);

    let dst_spec = crate::agent::mode::get_spec_filename_for_area(&target_area);
    let dst_task = crate::agent::mode::get_task_filename_for_area(&target_area);
    let dst_area_file = crate::agent::mode::get_area_filename_for_area(&target_area);

    // Copy files - keep source files and create target area files from templates
    // Only copy specs and tasks, NOT area.md
    for entry in fs::read_dir(&src)? {
        let entry = entry?;
        let path = entry.path();
        let filename = entry.file_name().to_string_lossy().to_string();

        // Skip area.md - it belongs to the area, not the topic
        if filename == "area.md" || filename.ends_with("_area.md") {
            continue;
        }

        // First, copy with original filename (keep source area files)
        let orig_dest = dst.join(&filename);
        if path.is_file() {
            fs::copy(&path, &orig_dest)?;
        } else if path.is_dir() {
            copy_dir_recursive(&path, &orig_dest)?;
        }

        // Second, if this is a spec or task file, create target version from template
        let target_filename = if filename == src_spec {
            dst_spec.clone()
        } else if filename == src_task {
            dst_task.clone()
        } else {
            continue;
        };

        // Only create target file if it has a different name
        if target_filename != filename {
            let target_dest = dst.join(&target_filename);
            // Read template for target area
            let template_content = if filename == src_spec {
                crate::fs::read_area_template(&target_area, "specs.md")
            } else if filename == src_task {
                crate::fs::read_area_template(&target_area, "tasks.md")
            } else {
                None
            };

            if let Some(content) = template_content {
                fs::write(&target_dest, &content)?;
            } else if path.is_file() {
                fs::copy(&path, &target_dest)?;
            }
        }
    }

    Ok(format!(
        "✅ Pulled topic '{}' from {} to {}",
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
    let current_config = load_agent_config()?;
    let area =
        area_str.unwrap_or_else(|| current_config.default_area.as_deref().unwrap_or("Working"));
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

pub fn run_show(topic: &str, show_all: bool, from_area: Option<&str>) -> Result<()> {
    let current_config = load_agent_config()?;
    let current_area = current_config
        .default_area
        .unwrap_or_else(|| "Working".to_string());

    if show_all {
        show_all_topic_files(topic)
    } else if let Some(ref area) = from_area {
        // Show files from a specific area (useful when topic has files from multiple areas)
        show_topic_from_area(topic, area)
    } else {
        // Show current area's files - but also show files from other areas that exist here
        let spec_filename = crate::agent::mode::get_spec_filename_for_area(&current_area);
        let task_filename = crate::agent::mode::get_task_filename_for_area(&current_area);
        show_topic_current_area(topic, &current_area, &spec_filename, &task_filename)
    }
}

fn show_topic_from_area(topic: &str, area: &str) -> Result<()> {
    let topic_path = crate::fs::topic_path(topic, area);

    if !topic_path.exists() {
        return Err(anyhow::anyhow!(
            "❌ Topic '{}' not found in {}.",
            topic,
            area
        ));
    }

    println!("📁 Files for topic '{}' in {}:\n", topic, area);

    // Get the file names for this area
    let spec_filename = crate::agent::mode::get_spec_filename_for_area(area);
    let task_filename = crate::agent::mode::get_task_filename_for_area(area);

    // List all files in the topic directory for this area
    for entry in fs::read_dir(&topic_path)? {
        let entry = entry?;
        let path = entry.path();
        if !path.is_file() {
            continue;
        }

        let filename = entry.file_name().to_string_lossy().to_string();

        if filename == spec_filename {
            println!("📝 {} (specs)", filename);
            if let Ok(content) = fs::read_to_string(&path) {
                for line in content.lines() {
                    println!("    > {}", line);
                }
            }
            println!();
        } else if filename == task_filename {
            println!("📋 {} (tasks)", filename);
            if let Ok(content) = fs::read_to_string(&path) {
                for line in content.lines() {
                    println!("    > {}", line);
                }
            }
            println!();
        } else {
            println!("📄 {}", filename);
        }
    }
    Ok(())
}

fn show_all_topic_files(topic: &str) -> Result<()> {
    let spec_dir = crate::fs::spec_dir();

    println!("📁 All files for topic: {}\n", topic);

    for area_entry in fs::read_dir(&spec_dir)? {
        let area_entry = area_entry?;
        let area_path = area_entry.path();
        if !area_path.is_dir() {
            continue;
        }

        let area_name = area_entry.file_name().to_string_lossy().to_string();
        let topic_path = area_path.join(topic);

        if !topic_path.exists() {
            continue;
        }

        println!("═══ {} ═══", area_name);

        // Get area-specific file names
        let spec_filename = crate::agent::mode::get_spec_filename_for_area(&area_name);
        let task_filename = crate::agent::mode::get_task_filename_for_area(&area_name);

        // List all files in the topic directory for this area
        for file_entry in fs::read_dir(&topic_path)? {
            let file_entry = file_entry?;
            let filename = file_entry.file_name().to_string_lossy().to_string();

            if filename == spec_filename {
                println!("  📝 {} (specs)", filename);
                if let Ok(content) = fs::read_to_string(file_entry.path()) {
                    for line in content.lines() {
                        println!("    > {}", line);
                    }
                }
            } else if filename == task_filename {
                println!("  📋 {} (tasks)", filename);
                if let Ok(content) = fs::read_to_string(file_entry.path()) {
                    for line in content.lines() {
                        println!("    > {}", line);
                    }
                }
            } else {
                println!("  📄 {}", filename);
            }
        }
        println!();
    }

    Ok(())
}

fn show_topic_current_area(
    topic: &str,
    area: &str,
    spec_filename: &str,
    task_filename: &str,
) -> Result<()> {
    let topic_path = topic_path(topic, area);
    let spec_path = topic_path.join(spec_filename);
    let task_path = topic_path.join(task_filename);

    if !spec_path.exists() {
        return Err(anyhow::anyhow!(
            "❌ Topic '{}' not found in {}.",
            topic,
            area
        ));
    }

    // Show specs
    println!("📝 {}", spec_filename);
    println!("{}", "─".repeat(40));
    if let Ok(content) = std::fs::read_to_string(&spec_path) {
        let lines: Vec<&str> = content.lines().take(30).collect();
        for line in lines {
            println!("  {}", line);
        }
        if content.lines().count() > 30 {
            println!("  ... (showing first 30 lines)");
        }
    }

    // Show tasks
    println!("\n📋 {}", task_filename);
    println!("{}", "─".repeat(40));
    let mut all_checked = true;
    if task_path.exists() {
        if let Ok(content) = std::fs::read_to_string(&task_path) {
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
        println!("  ℹ️  No {} file", task_filename);
    }

    let status = if all_checked {
        "complete"
    } else {
        "in-progress"
    };

    println!("\nStatus: {}", status);
    Ok(())
}
