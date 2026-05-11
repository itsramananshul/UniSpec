// src/commands/topic.rs
use crate::agent::load_agent_config;
use crate::fs::{ensure_dir, topic_path};
use anyhow::Result;
use std::fs;

pub fn get_agent_id() -> String {
    std::process::Command::new("git")
        .args(["config", "--global", "user.name"])
        .output()
        .ok()
        .and_then(|o| String::from_utf8(o.stdout).ok())
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .unwrap_or_else(|| {
            std::env::var("USER")
                .or_else(|_| std::env::var("USERNAME"))
                .unwrap_or_else(|_| "unknown".to_string())
        })
}

pub fn run_new(
    topic: &str,
    area_str: &str,
    short: Option<&str>,
    content: Option<&str>,
) -> Result<String> {
    let topic_path = crate::fs::spec_dir().join(area_str).join(topic);
    let topic_name = topic_path
        .file_name()
        .map(|n| n.to_string_lossy().to_string())
        .unwrap_or_else(|| topic.to_string());

    if topic_path.exists() {
        return Err(anyhow::anyhow!(
            "❌ Topic '{}' already exists in {}.",
            topic_name,
            area_str
        ));
    }

    // Require short description
    let short = short.ok_or_else(|| {
        anyhow::anyhow!("❌ 'short' parameter is required!\n\nUsage: unispec topic add <topic> --short \"One line description\" [--content \"Markdown content\"]")
    })?;

    // Require content (use provided or prompt user)
    let content = content.ok_or_else(|| {
        anyhow::anyhow!("❌ 'content' parameter is required!\n\nUsage: unispec topic add <topic> --short \"One line description\" --content \"# Topic Name\\n\\n## Overview\\n...\"")
    })?;

    if content.len() < 20 {
        return Err(anyhow::anyhow!(
            "❌ Content must be at least 20 characters. You provided {} characters.",
            content.len()
        ));
    }

    std::fs::create_dir_all(&topic_path)?;

    let now = chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
    let author = get_agent_id();

    // Prepend frontmatter with metadata
    let frontmatter = format!(
        "---\ntitle: {}\nshort: {}\ncreated: {}\nauthor: {}\n---\n\n",
        topic_name, short, now, author
    );

    let full_content = format!("{}{}", frontmatter, content);

    fs::write(topic_path.join("topic.md"), full_content)?;

    Ok(format!(
        "✅ Topic '{}' created in {}/",
        topic_name, area_str
    ))
}

pub fn run_new_spec(spec_name: &str, topic_path: &str, area_str: &str) -> Result<String> {
    let sanitized_name = spec_name.replace(" ", "-");
    let topic_dir = crate::fs::spec_dir().join(area_str).join(topic_path);

    if !topic_dir.exists() {
        return Err(anyhow::anyhow!(
            "❌ Topic directory '{}' does not exist.",
            topic_path
        ));
    }

    let spec_filename = format!("{}_spec.md", sanitized_name);
    let task_filename = format!("{}_task.md", sanitized_name);

    let spec_content = format!(
        "---\ntitle: {}\nstatus: proposed\ncreated: {}\n---\n\n# {}\n\n## Problem Statement\n\n## Requirements\n\n## Acceptance Criteria\n",
        sanitized_name,
        chrono::Local::now().format("%Y-%m-%d"),
        sanitized_name
    );

    let tasks_template = read_default_tasks_template();

    fs::write(topic_dir.join(&spec_filename), &spec_content)?;
    fs::write(topic_dir.join(&task_filename), &tasks_template)?;

    Ok(format!(
        "✅ Spec '{}' created in {}",
        sanitized_name, topic_path
    ))
}

pub fn run_new_with_metadata(
    topic_path_str: &str,
    area_str: &str,
    short: Option<&str>,
    content: Option<&str>,
) -> Result<String> {
    let topic_path = crate::fs::spec_dir().join(area_str).join(topic_path_str);
    let topic_name = topic_path
        .file_name()
        .map(|n| n.to_string_lossy().to_string())
        .unwrap_or_else(|| topic_path_str.to_string());

    if topic_path.exists() {
        return Err(anyhow::anyhow!(
            "❌ Topic '{}' already exists in {}.",
            topic_name,
            area_str
        ));
    }

    // Require short description
    let short = short.ok_or_else(|| {
        anyhow::anyhow!("❌ Short description is required when creating topics from TUI. Please provide a one-line description.")
    })?;

    // Use content if provided, otherwise use template
    let topic_content = if let Some(content) = content {
        if content.len() < 20 {
            return Err(anyhow::anyhow!(
                "❌ Content must be at least 20 characters."
            ));
        }
        let now = chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
        let author = get_agent_id();
        let frontmatter = format!(
            "---\ntitle: {}\nshort: {}\ncreated: {}\nauthor: {}\n---\n\n",
            topic_name, short, now, author
        );
        format!("{}{}", frontmatter, content)
    } else {
        let today = chrono::Local::now().format("%Y-%m-%d").to_string();
        let now = chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
        let author = get_agent_id();

        let topic_template = crate::fs::read_template("topic.md")
            .or_else(|| crate::fs::read_area_template(area_str, "topic.md"))
            .unwrap_or_else(|| {
                "---\ntitle: <TopicName>\nshort: <Short>\ncreated: <DateTime>\nmodified: <DateTime>\nauthor: <Author>\n---\n# <TopicName>\n\n## Overview\n\n## Specs\n\n## Sub-topics\n".to_string()
            });

        topic_template
            .replace("<TopicName>", &topic_name)
            .replace("{topic}", &topic_name)
            .replace("<Short>", short)
            .replace("{short}", short)
            .replace("<Date>", &today)
            .replace("<DateTime>", &now)
            .replace("{datetime}", &now)
            .replace("<Author>", &author)
            .replace("{author}", &author)
    };

    std::fs::create_dir_all(&topic_path)?;
    fs::write(topic_path.join("topic.md"), topic_content)?;

    Ok(format!(
        "✅ Topic '{}' created in {}/",
        topic_name, area_str
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
        if entry.path().is_dir() && entry.path().join("topic.md").exists() {
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
    let agent_id = get_agent_id();
    let source_area = source_area.map(String::from).unwrap_or_else(|| {
        current_config
            .default_area
            .clone()
            .unwrap_or_else(|| "Working".to_string())
    });

    // Normalize area names (case-insensitive)
    let source_area_normalized = crate::fs::normalize_area_name(&source_area);
    let target_area_normalized = crate::fs::normalize_area_name(target_area);

    // Check if areas exist
    if !crate::fs::area_exists(&source_area_normalized) {
        return Err(anyhow::anyhow!(
            "❌ Source area '{}' not found.",
            source_area
        ));
    }
    if !crate::fs::area_exists(&target_area_normalized) {
        return Err(anyhow::anyhow!(
            "❌ Target area '{}' not found.",
            target_area
        ));
    }

    if source_area_normalized.to_lowercase() == target_area_normalized.to_lowercase() {
        return Err(anyhow::anyhow!(
            "❌ Source and target areas are the same: {}",
            source_area_normalized
        ));
    }

    let src = topic_path(topic, &source_area_normalized);
    let dst = topic_path(topic, &target_area_normalized);

    if !src.exists() {
        return Err(anyhow::anyhow!(
            "❌ Topic '{}' not found in {}.",
            topic,
            source_area_normalized
        ));
    }

    let spec_filename = crate::agent::mode::get_spec_filename_for_area(&source_area_normalized);
    let spec_path = src.join(&spec_filename);

    if spec_path.exists() {
        if let Ok(content) = fs::read_to_string(&spec_path) {
            if let Some(metadata) = crate::fs::spec::parse_spec_metadata(&content) {
                if let Some(ref checked_out_by) = metadata.checked_out {
                    if !checked_out_by.is_empty() && checked_out_by != &agent_id {
                        return Err(anyhow::anyhow!(
                            "❌ Topic '{}' is checked out by {}. Cannot push until checked in.",
                            topic,
                            checked_out_by
                        ));
                    }
                }
            }
        }
    }

    // Check if topic in destination (Build) is checked out by someone else
    if target_area_normalized.to_lowercase() == "build" && dst.exists() {
        let dst_spec_filename =
            crate::agent::mode::get_spec_filename_for_area(&target_area_normalized);
        let dst_spec_path = dst.join(&dst_spec_filename);
        if dst_spec_path.exists() {
            if let Ok(content) = fs::read_to_string(&dst_spec_path) {
                if let Some(metadata) = crate::fs::spec::parse_spec_metadata(&content) {
                    if let Some(ref checked_out_by) = metadata.checked_out {
                        if !checked_out_by.is_empty() && checked_out_by != &agent_id {
                            return Err(anyhow::anyhow!(
                                "❌ Topic '{}' is checked out in Build by {}. Cannot push until checked in.",
                                topic,
                                checked_out_by
                            ));
                        }
                    }
                }
            }
        }
    }

    // Check readiness - only for areas configured in mode.toml
    // Topic must be LISTED in area's queue.md to push from these areas
    if crate::agent::mode::area_requires_readiness(&source_area_normalized) {
        let queue_file = crate::agent::mode::get_readiness_queue_file();

        // Check: is topic listed in area's queue.md? (required for push)
        let area_queue_path = crate::fs::spec_dir()
            .join(&source_area_normalized)
            .join(&queue_file);
        let is_in_queue = if area_queue_path.exists() {
            if let Ok(queue_content) = fs::read_to_string(&area_queue_path) {
                queue_content.lines().any(|line| {
                    let trimmed = line.trim();
                    trimmed.starts_with("- ") && trimmed.contains(topic)
                })
            } else {
                false
            }
        } else {
            false
        };

        if !is_in_queue {
            return Err(anyhow::anyhow!(
                "❌ Topic '{}' is not ready to push. It must be listed in {}/{}.\nAdd it to the area queue first.",
                topic, source_area_normalized, queue_file
            ));
        }
    }

    ensure_dir(&dst)?;

    // If pushing to Working, ensure /src directory exists at project root
    if target_area_normalized.to_lowercase() == "working" {
        let project_root =
            crate::fs::project_root().unwrap_or_else(|_| std::path::PathBuf::from("."));
        let src_dir = project_root.join("src");
        if !src_dir.exists() {
            fs::create_dir_all(&src_dir)?;
            println!("Created /src directory at project root for code files");
        }
    }

    // Get source and target file names from mode config
    let src_spec = crate::agent::mode::get_spec_filename_for_area(&source_area_normalized);
    let src_task = crate::agent::mode::get_task_filename_for_area(&source_area_normalized);
    let src_area_file = crate::agent::mode::get_area_filename_for_area(&source_area_normalized);

    let dst_spec = crate::agent::mode::get_spec_filename_for_area(&target_area_normalized);
    let dst_task = crate::agent::mode::get_task_filename_for_area(&target_area_normalized);
    let dst_area_file = crate::agent::mode::get_area_filename_for_area(&target_area_normalized);

    // Copy every file from the source topic directory into the destination
    // verbatim. We do not synthesise additional files under legacy filenames
    // (the previous behaviour wrote a duplicate `specs.md`/`tasks.md` next to
    // the agent-written `<topic>_spec.md`/`<topic>_task.md`, which produced
    // two divergent specs in the destination).
    for entry in fs::read_dir(&src)? {
        let entry = entry?;
        let path = entry.path();
        let filename = entry.file_name().to_string_lossy().to_string();

        // Skip area.md - it belongs to the area, not the topic
        if filename == "area.md" || filename.ends_with("_area.md") {
            continue;
        }

        let orig_dest = dst.join(&filename);
        if path.is_file() {
            fs::copy(&path, &orig_dest)?;
        } else if path.is_dir() {
            copy_dir_recursive(&path, &orig_dest)?;
        }
    }

    // Apply file filtering based on target area config - delete files that shouldn't pass through
    if let Some(filter) = crate::agent::mode::get_area_file_filter(&target_area_normalized) {
        if !filter.delete_files.is_empty() {
            for entry in fs::read_dir(&dst)? {
                let entry = entry?;
                let path = entry.path();
                if path.is_file() {
                    if let Some(filename) = path.file_name().and_then(|n| n.to_str()) {
                        for pattern in &filter.delete_files {
                            if matches_pattern(filename, pattern) {
                                let _ = fs::remove_file(&path);
                                break;
                            }
                        }
                    }
                }
            }
        }
    }

    let completed_info = if target_area_normalized.to_lowercase() == "build" {
        let today = chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
        format!(" (completed: {})", today)
    } else {
        String::new()
    };

    let checkin_msg = if source_area_normalized.to_lowercase() == "working"
        && target_area_normalized.to_lowercase() == "build"
    {
        auto_checkin(topic, &source_area_normalized)?
    } else {
        String::new()
    };

    // Move: copy to destination, then remove source
    let move_msg = if source_area_normalized.to_lowercase() != "build" {
        // Only move from non-Build areas. Build items stay (they're shipped).
        if source_area_normalized.to_lowercase() != "build" {
            // Copy completed, now remove source
            fs::remove_dir_all(&src)?;
            format!(" (moved from {})", source_area_normalized)
        } else {
            String::new()
        }
    } else {
        String::new()
    };

    Ok(format!(
        "✅ Moved '{}' from {} → {}{}{}{}",
        topic,
        source_area_normalized,
        target_area_normalized,
        completed_info,
        if checkin_msg.is_empty() {
            String::new()
        } else {
            format!("\n{}", checkin_msg)
        },
        if move_msg.is_empty() {
            String::new()
        } else {
            move_msg
        }
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

fn matches_pattern(filename: &str, pattern: &str) -> bool {
    if pattern.starts_with("*.") {
        let ext = &pattern[2..];
        return filename.ends_with(&format!(".{}", ext));
    }
    if pattern.ends_with("*") {
        let prefix = &pattern[..pattern.len() - 1];
        return filename.starts_with(prefix);
    }
    filename == pattern
}

pub fn run_pull(topic: &str, source_area: &str) -> Result<String> {
    let agent_id = get_agent_id();
    let current_config = load_agent_config()?;
    let target_area = current_config
        .default_area
        .unwrap_or_else(|| "Working".to_string());

    // Check if areas exist
    if !crate::fs::area_exists(source_area) {
        return Err(anyhow::anyhow!(
            "❌ Source area '{}' not found.",
            source_area
        ));
    }
    if !crate::fs::area_exists(&target_area) {
        return Err(anyhow::anyhow!(
            "❌ Target area '{}' not found.",
            target_area
        ));
    }

    // Normalize area names (case-insensitive)
    let source_area_normalized = crate::fs::normalize_area_name(source_area);
    let target_area_normalized = crate::fs::normalize_area_name(&target_area);

    let src = topic_path(topic, &source_area_normalized);
    let dst = topic_path(topic, &target_area_normalized);

    if !src.exists() {
        return Err(anyhow::anyhow!(
            "❌ Topic '{}' not found in {}.",
            topic,
            source_area_normalized
        ));
    }

    let spec_filename = crate::agent::mode::get_spec_filename_for_area(&source_area_normalized);
    let spec_path = src.join(&spec_filename);

    if spec_path.exists() {
        if let Ok(content) = fs::read_to_string(&spec_path) {
            if let Some(metadata) = crate::fs::spec::parse_spec_metadata(&content) {
                if let Some(ref checked_out_by) = metadata.checked_out {
                    if !checked_out_by.is_empty() && checked_out_by != &agent_id {
                        return Err(anyhow::anyhow!(
                            "❌ Topic '{}' is checked out by {}. Cannot pull until checked in.",
                            topic,
                            checked_out_by
                        ));
                    }
                }
            }
        }
    }

    if dst.exists() {
        fs::remove_dir_all(&dst)?;
    }

    ensure_dir(&dst)?;

    // Get source and target file names from mode config
    let src_spec = crate::agent::mode::get_spec_filename_for_area(&source_area_normalized);
    let src_task = crate::agent::mode::get_task_filename_for_area(&source_area_normalized);
    let src_area_file = crate::agent::mode::get_area_filename_for_area(&source_area_normalized);

    let dst_spec = crate::agent::mode::get_spec_filename_for_area(&target_area_normalized);
    let dst_task = crate::agent::mode::get_task_filename_for_area(&target_area_normalized);
    let dst_area_file = crate::agent::mode::get_area_filename_for_area(&target_area_normalized);

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
        // Strip checkout metadata from spec files
        let orig_dest = dst.join(&filename);
        if path.is_file() {
            let is_spec_file = filename == src_spec
                || filename.ends_with("_spec.md")
                || filename.ends_with("_specs.md")
                || filename == "spec.md"
                || filename == "specs.md";

            if is_spec_file {
                let content = fs::read_to_string(&path)?;
                let clean_content = crate::fs::spec::strip_checkout_metadata(&content);
                fs::write(&orig_dest, &clean_content)?;
            } else {
                fs::copy(&path, &orig_dest)?;
            }
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
                crate::fs::read_area_template(&target_area_normalized, "specs.md")
            } else if filename == src_task {
                crate::fs::read_area_template(&target_area_normalized, "tasks.md")
            } else {
                None
            };

            if let Some(content) = template_content {
                fs::write(&target_dest, &content)?;
            } else if path.is_file() {
                // Strip checkout metadata from copied spec files
                let content = fs::read_to_string(&path)?;
                let clean_content = crate::fs::spec::strip_checkout_metadata(&content);
                fs::write(&target_dest, &clean_content)?;
            }
        }
    }

    let checkout_msg = if source_area_normalized.to_lowercase() == "build" {
        auto_checkout(topic, &source_area_normalized)?
    } else {
        String::new()
    };

    let ancestor_msg = pull_ancestors(topic, &source_area_normalized, &target_area_normalized)?;

    Ok(format!(
        "✅ Moved topic '{}' from {} → {}{}{}",
        topic,
        source_area_normalized,
        target_area_normalized,
        if checkout_msg.is_empty() {
            String::new()
        } else {
            format!("\n{}", checkout_msg)
        },
        if ancestor_msg.is_empty() {
            String::new()
        } else {
            ancestor_msg
        }
    ))
}

pub fn pull_ancestors(topic: &str, source_area: &str, target_area: &str) -> Result<String> {
    let parts: Vec<&str> = topic.split('/').collect();
    let mut results = vec![];

    let mut current_path = String::new();
    for (i, part) in parts.iter().enumerate() {
        if i > 0 {
            current_path.push('/');
        }
        current_path.push_str(part);

        if current_path == topic {
            continue;
        }

        let src = topic_path(&current_path, source_area);
        if src.exists() {
            let agent_id = get_agent_id();

            let spec_filename = crate::agent::mode::get_spec_filename_for_area(source_area);
            let spec_path = src.join(&spec_filename);

            if spec_path.exists() {
                if let Ok(content) = fs::read_to_string(&spec_path) {
                    if let Some(metadata) = crate::fs::spec::parse_spec_metadata(&content) {
                        if let Some(ref checked_out_by) = metadata.checked_out {
                            if !checked_out_by.is_empty() && checked_out_by != &agent_id {
                                continue;
                            }
                        }
                    }
                }
            }

            match pull_single_topic(&current_path, source_area, target_area, false) {
                Ok(msg) => results.push(format!("  - {}", msg.trim())),
                Err(e) => {
                    results.push(format!("  - {} (skipped: {})", current_path, e));
                }
            }
        }
    }

    if results.is_empty() {
        Ok(String::new())
    } else {
        Ok(format!("\nAncestors also pulled:\n{}", results.join("\n")))
    }
}

fn pull_single_topic(
    topic: &str,
    source_area: &str,
    target_area: &str,
    auto_checkout: bool,
) -> Result<String> {
    let src = topic_path(topic, source_area);
    let dst = topic_path(topic, target_area);

    if !src.exists() {
        return Err(anyhow::anyhow!(
            "Topic '{}' not found in {}",
            topic,
            source_area
        ));
    }

    if dst.exists() {
        fs::remove_dir_all(&dst)?;
    }

    ensure_dir(&dst)?;

    let src_spec = crate::agent::mode::get_spec_filename_for_area(source_area);
    let src_task = crate::agent::mode::get_task_filename_for_area(source_area);

    let dst_spec = crate::agent::mode::get_spec_filename_for_area(target_area);
    let dst_task = crate::agent::mode::get_task_filename_for_area(target_area);

    for entry in fs::read_dir(&src)? {
        let entry = entry?;
        let path = entry.path();
        let filename = entry.file_name().to_string_lossy().to_string();

        if filename == "area.md" || filename.ends_with("_area.md") {
            continue;
        }

        let orig_dest = dst.join(&filename);
        if path.is_file() {
            // Strip checkout metadata from spec files
            let is_spec_file = filename == src_spec
                || filename.ends_with("_spec.md")
                || filename.ends_with("_specs.md")
                || filename == "spec.md"
                || filename == "specs.md";

            if is_spec_file {
                let content = fs::read_to_string(&path)?;
                let clean_content = crate::fs::spec::strip_checkout_metadata(&content);
                fs::write(&orig_dest, &clean_content)?;
            } else {
                fs::copy(&path, &orig_dest)?;
            }
        } else if path.is_dir() {
            copy_dir_recursive(&path, &orig_dest)?;
        }

        let target_filename = if filename == src_spec {
            dst_spec.clone()
        } else if filename == src_task {
            dst_task.clone()
        } else {
            continue;
        };

        if target_filename != filename {
            let target_dest = dst.join(&target_filename);
            let template_content = if filename == src_spec {
                crate::fs::read_area_template(target_area, "specs.md")
            } else if filename == src_task {
                crate::fs::read_area_template(target_area, "tasks.md")
            } else {
                None
            };

            if let Some(content) = template_content {
                fs::write(&target_dest, &content)?;
            } else if path.is_file() {
                // Strip checkout metadata from copied spec files
                let content = fs::read_to_string(&path)?;
                let clean_content = crate::fs::spec::strip_checkout_metadata(&content);
                fs::write(&target_dest, &clean_content)?;
            }
        }
    }

    if auto_checkout && source_area.to_lowercase() == "build" {
        let spec_filename = crate::agent::mode::get_spec_filename_for_area(source_area);
        let spec_path = src.join(&spec_filename);
        if spec_path.exists() {
            let content = fs::read_to_string(&spec_path)?;
            let agent_id = get_agent_id();
            let new_content = crate::fs::spec::update_spec_with_checkout(&content, &agent_id, true);
            fs::write(&spec_path, &new_content)?;
        }
    }

    // Move: copy completed, now remove source (except Build)
    if source_area.to_lowercase() != "build" {
        fs::remove_dir_all(&src)?;
    }

    Ok(format!("Moved '{}'", topic))
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

pub fn run_checkout(topic: &str, area: &str) -> Result<String> {
    // Checkout only allowed from Build area
    if area.to_lowercase() != "build" {
        return Err(anyhow::anyhow!(
            "❌ Checkout is only allowed from Build area. Topic '{}' is in {}.",
            topic,
            area
        ));
    }

    let agent_id = get_agent_id();
    let src_path = topic_path(topic, area);

    if !src_path.exists() {
        return Err(anyhow::anyhow!(
            "❌ Topic '{}' not found in {}.",
            topic,
            area
        ));
    }

    let spec_filename = crate::agent::mode::get_spec_filename_for_area(area);
    let spec_path = src_path.join(&spec_filename);

    if !spec_path.exists() {
        return Err(anyhow::anyhow!(
            "❌ Spec file not found for topic '{}'.",
            topic
        ));
    }

    let content = fs::read_to_string(&spec_path)?;
    let metadata = crate::fs::spec::parse_spec_metadata(&content);

    if let Some(ref m) = metadata {
        if let Some(ref checked_out_by) = m.checked_out {
            if !checked_out_by.is_empty() && checked_out_by != &agent_id {
                return Err(anyhow::anyhow!(
                    "❌ Topic '{}' is already checked out by {}. Cannot checkout until checked in.",
                    topic,
                    checked_out_by
                ));
            }
        }
    }

    let new_content = crate::fs::spec::update_spec_with_checkout(&content, &agent_id, true);
    fs::write(&spec_path, &new_content)?;

    let target_area = "Working";
    let dst = topic_path(topic, target_area);

    if !dst.exists() {
        ensure_dir(&dst)?;
        for entry in fs::read_dir(&src_path)? {
            let entry = entry?;
            let path = entry.path();
            let filename = entry.file_name().to_string_lossy().to_string();

            if filename == "area.md" || filename.ends_with("_area.md") {
                continue;
            }

            if path.is_file() {
                fs::copy(&path, &dst.join(&filename))?;
            } else if path.is_dir() {
                copy_dir_recursive(&path, &dst.join(entry.file_name()))?;
            }
        }
    }

    Ok(format!(
        "✅ Checked out '{}' from {} and moved to Working.\n   Use /build {} to start working.",
        topic, area, topic
    ))
}

pub fn run_checkin(topic: &str, source_area: &str) -> Result<String> {
    let agent_id = get_agent_id();
    let src_path = topic_path(topic, source_area);

    if !src_path.exists() {
        return Err(anyhow::anyhow!(
            "❌ Topic '{}' not found in {}.",
            topic,
            source_area
        ));
    }

    let spec_filename = crate::agent::mode::get_spec_filename_for_area(source_area);
    let spec_path = src_path.join(&spec_filename);

    if !spec_path.exists() {
        return Err(anyhow::anyhow!(
            "❌ Spec file not found for topic '{}'.",
            topic
        ));
    }

    let content = fs::read_to_string(&spec_path)?;
    let metadata = crate::fs::spec::parse_spec_metadata(&content);

    if let Some(ref m) = metadata {
        if let Some(ref checked_out_by) = m.checked_out {
            if !checked_out_by.is_empty() && checked_out_by != &agent_id {
                return Err(anyhow::anyhow!(
                    "❌ Topic '{}' is checked out by {}, not by this agent.",
                    topic,
                    checked_out_by
                ));
            }
        }
    }

    let new_content = crate::fs::spec::update_spec_with_checkout(&content, &agent_id, false);
    fs::write(&spec_path, &new_content)?;

    let dst = topic_path(topic, "Build");
    if !dst.exists() {
        ensure_dir(&dst)?;
        for entry in fs::read_dir(&src_path)? {
            let entry = entry?;
            let path = entry.path();
            let filename = entry.file_name().to_string_lossy().to_string();

            if filename == "area.md" || filename.ends_with("_area.md") {
                continue;
            }

            if path.is_file() {
                fs::copy(&path, &dst.join(&filename))?;
            } else if path.is_dir() {
                copy_dir_recursive(&path, &dst.join(entry.file_name()))?;
            }
        }
    }

    Ok(format!(
        "✅ Checked in '{}' from {} and pushed to Build.",
        topic, source_area
    ))
}

pub fn auto_checkout(topic: &str, area: &str) -> Result<String> {
    // Checkout only allowed from Build area
    if area.to_lowercase() != "build" {
        return Err(anyhow::anyhow!(
            "❌ Checkout is only allowed from Build area. Topic '{}' is in {}.",
            topic,
            area
        ));
    }

    let agent_id = get_agent_id();
    let src_path = topic_path(topic, area);

    if !src_path.exists() {
        return Err(anyhow::anyhow!(
            "❌ Topic '{}' not found in {}.",
            topic,
            area
        ));
    }

    let spec_filename = crate::agent::mode::get_spec_filename_for_area(area);
    let spec_path = src_path.join(&spec_filename);

    if !spec_path.exists() {
        return Err(anyhow::anyhow!(
            "❌ Spec file not found for topic '{}'.",
            topic
        ));
    }

    let content = fs::read_to_string(&spec_path)?;
    let metadata = crate::fs::spec::parse_spec_metadata(&content);

    if let Some(ref m) = metadata {
        if let Some(ref checked_out_by) = m.checked_out {
            if !checked_out_by.is_empty() && checked_out_by != &agent_id {
                return Err(anyhow::anyhow!(
                    "❌ Topic '{}' is checked out by {}. Cannot pull until checked back in.",
                    topic,
                    checked_out_by
                ));
            }
        }
    }

    let new_content = crate::fs::spec::update_spec_with_checkout(&content, &agent_id, true);
    fs::write(&spec_path, &new_content)?;

    Ok(format!(
        "✅ Auto-checked out '{}' from {} (pulled to Working)",
        topic, area
    ))
}

pub fn auto_checkin(topic: &str, area: &str) -> Result<String> {
    let agent_id = get_agent_id();
    let src_path = topic_path(topic, area);

    if !src_path.exists() {
        return Err(anyhow::anyhow!(
            "❌ Topic '{}' not found in {}.",
            topic,
            area
        ));
    }

    // Use the same `<topic>_spec.md` naming convention that `spec_add` writes.
    // The legacy `get_spec_filename_for_area` returns the mode template name
    // ("spec.md" / "specs.md") which doesn't match what's actually on disk.
    let topic_safe = topic.replace('/', "-").replace(' ', "-");
    let spec_filename = format!("{}_spec.md", topic_safe);
    let spec_path = src_path.join(&spec_filename);

    // No spec file yet — nothing to check in. Don't fail the push for this.
    if !spec_path.exists() {
        return Ok(format!(
            "ℹ️  No spec file at {} for '{}'; skipping auto-checkin.",
            spec_path.display(),
            topic
        ));
    }

    let content = fs::read_to_string(&spec_path)?;
    let metadata = crate::fs::spec::parse_spec_metadata(&content);

    if let Some(ref m) = metadata {
        if let Some(ref checked_out_by) = m.checked_out {
            if !checked_out_by.is_empty() && checked_out_by != &agent_id {
                return Err(anyhow::anyhow!(
                    "❌ Topic '{}' is checked out by {}, not by this agent. Cannot push.",
                    topic,
                    checked_out_by
                ));
            }
        }
    }

    let new_content = crate::fs::spec::update_spec_with_checkout(&content, &agent_id, false);
    fs::write(&spec_path, &new_content)?;

    Ok(format!(
        "✅ Auto-checked in '{}' (pushed to {})",
        topic, area
    ))
}

pub fn run_delete(topic: &str, area: &str, _force: bool) -> Result<String> {
    let path = crate::fs::spec_dir().join(area).join(topic);
    if !path.exists() {
        return Err(anyhow::anyhow!(
            "❌ Item '{}' not found in {}.",
            topic,
            area
        ));
    }

    if path.is_dir() {
        fs::remove_dir_all(&path)?;
        Ok(format!("✅ Deleted topic '{}' from {}/", topic, area))
    } else if path.to_string_lossy().ends_with("_spec.md") {
        let spec_path = path.clone();
        let task_path = path.to_string_lossy().replace("_spec.md", "_task.md");
        fs::remove_file(&spec_path)?;
        if std::path::Path::new(&task_path).exists() {
            fs::remove_file(&task_path)?;
        }
        Ok(format!("✅ Deleted spec '{}' from {}/", topic, area))
    } else {
        fs::remove_file(&path)?;
        Ok(format!("✅ Deleted item '{}' from {}/", topic, area))
    }
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

pub fn run_order(area: &str) -> Result<String> {
    let order = crate::agent::mode::get_topic_order(area);

    if order.is_empty() {
        return Ok(format!(
            "No custom order defined for {}. Topics will be sorted alphabetically.",
            area
        ));
    }

    let content: String = order
        .iter()
        .enumerate()
        .map(|(i, t)| format!("{}. {}", i + 1, t))
        .collect::<Vec<_>>()
        .join("\n");

    Ok(content)
}

pub fn run_order_add(area: &str, topics: Vec<String>, position: Option<usize>) -> Result<String> {
    crate::agent::mode::add_to_topic_order(area, topics.clone(), position)?;

    let order = crate::agent::mode::get_topic_order(area);
    let order_content: String = order
        .iter()
        .enumerate()
        .map(|(i, t)| format!("{}. {}", i + 1, t))
        .collect::<Vec<_>>()
        .join("\n");

    let pos_info = if let Some(p) = position {
        format!(" at position {}", p + 1)
    } else {
        String::new()
    };

    Ok(format!(
        "✅ Added {} topic(s) to {} order{}.\n\nCurrent order:\n{}",
        topics.len(),
        area,
        pos_info,
        order_content
    ))
}

pub fn run_order_remove(area: &str, topics: Vec<String>) -> Result<String> {
    crate::agent::mode::remove_from_topic_order(area, topics.clone())?;

    let order = crate::agent::mode::get_topic_order(area);
    let order_content: String = if order.is_empty() {
        "(alphabetical)".to_string()
    } else {
        order
            .iter()
            .enumerate()
            .map(|(i, t)| format!("{}. {}", i + 1, t))
            .collect::<Vec<_>>()
            .join("\n")
    };

    Ok(format!(
        "✅ Removed {} topic(s) from {} order.\n\nCurrent order:\n{}",
        topics.len(),
        area,
        order_content
    ))
}

pub fn run_order_reset(area: &str) -> Result<String> {
    crate::agent::mode::reset_topic_order(area)?;

    Ok(format!("✅ Reset {} order to alphabetical.", area))
}
