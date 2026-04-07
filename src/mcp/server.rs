// src/mcp/server.rs - MCP Server implementation
use anyhow::Result;
use serde_json::{json, Value};
use std::io::{Read, Write};

fn call_tool(name: &str, args: &Value) -> Result<Value> {
    match name {
        "topics_list" => {
            let area = args
                .get("area")
                .and_then(|v| v.as_str())
                .unwrap_or("Working");
            let topics = crate::fs::spec_dir().join(area);
            if !topics.exists() {
                return Ok(
                    json!({ "success": true, "topics": [], "message": format!("Area '{}' does not exist", area) }),
                );
            }
            let spec_filename = crate::agent::mode::get_spec_filename_for_area(area);
            let task_filename = crate::agent::mode::get_task_filename_for_area(area);
            let mut topic_list = vec![];
            for entry in std::fs::read_dir(&topics)? {
                let entry = entry?;
                if entry.path().is_dir() {
                    let name = entry.file_name().to_string_lossy().to_string();
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
                    topic_list.push(json!({ "name": name, "status": status }));
                }
            }
            Ok(json!({ "success": true, "area": area, "topics": topic_list }))
        }
        "topics_add" => {
            let topic = args.get("topic").and_then(|v| v.as_str()).unwrap();
            let area = args
                .get("area")
                .and_then(|v| v.as_str())
                .unwrap_or("Working");
            let result = crate::commands::topic::run_new(topic, area)?;
            Ok(json!({ "success": true, "message": result, "topic": topic, "area": area }))
        }
        "topics_show" => {
            let topic = args.get("topic").and_then(|v| v.as_str()).unwrap();
            let show_all = args
                .get("show_all")
                .and_then(|v| v.as_bool())
                .unwrap_or(false);
            let from_area = args.get("from").and_then(|v| v.as_str());

            // Return JSON data instead of printing
            if show_all {
                // Show all areas
                let spec_dir = crate::fs::spec_dir();
                let mut area_topics = vec![];
                for area_entry in std::fs::read_dir(&spec_dir)? {
                    let area_entry = area_entry?;
                    let area_path = area_entry.path();
                    if !area_path.is_dir() {
                        continue;
                    }
                    let area_name = area_entry.file_name().to_string_lossy().to_string();
                    let topic_path = area_path.join(topic);
                    if topic_path.exists() {
                        let spec_filename =
                            crate::agent::mode::get_spec_filename_for_area(&area_name);
                        let task_filename =
                            crate::agent::mode::get_task_filename_for_area(&area_name);
                        let mut files = vec![];
                        for entry in std::fs::read_dir(&topic_path)? {
                            let entry = entry?;
                            if entry.path().is_file() {
                                let name = entry.file_name().to_string_lossy().to_string();
                                let is_spec = name == spec_filename;
                                let is_task = name == task_filename;
                                files.push(json!({ "name": name, "type": if is_spec { "spec" } else if is_task { "task" } else { "other" } }));
                            }
                        }
                        area_topics.push(json!({ "area": area_name, "files": files }));
                    }
                }
                Ok(json!({ "success": true, "topic": topic, "areas": area_topics }))
            } else {
                let area = from_area.unwrap_or("Working");
                let topic_path = crate::fs::spec_dir().join(area).join(topic);
                if !topic_path.exists() {
                    return Err(anyhow::anyhow!(
                        "Topic '{}' not found in area '{}'",
                        topic,
                        area
                    ));
                }
                let spec_filename = crate::agent::mode::get_spec_filename_for_area(area);
                let task_filename = crate::agent::mode::get_task_filename_for_area(area);

                // Debug mode info
                eprintln!("DEBUG: area='{}'", area);
                if let Ok(mode_name) = crate::agent::current_mode() {
                    eprintln!("DEBUG: mode_name='{}'", mode_name);
                    if let Ok(config) = crate::agent::mode::get_mode_info(&mode_name) {
                        eprintln!(
                            "DEBUG: templates.spec_file='{}'",
                            config.templates.spec_file
                        );
                    }
                }
                let mut files = vec![];
                for entry in std::fs::read_dir(&topic_path)? {
                    let entry = entry?;
                    if entry.path().is_file() {
                        let name = entry.file_name().to_string_lossy().to_string();
                        let is_spec = name == spec_filename;
                        let is_task = name == task_filename;
                        files.push(json!({ "name": name, "type": if is_spec { "spec" } else if is_task { "task" } else { "other" } }));
                    }
                }
                Ok(json!({ "success": true, "topic": topic, "area": area, "files": files }))
            }
        }
        "topics_delete" => {
            let topic = args.get("topic").and_then(|v| v.as_str()).unwrap();
            let area = args
                .get("area")
                .and_then(|v| v.as_str())
                .unwrap_or("Working");
            let force = args.get("force").and_then(|v| v.as_bool()).unwrap_or(true);
            let result = crate::commands::topic::run_delete(topic, area, force)?;
            Ok(json!({ "success": true, "message": result }))
        }
        "topics_push" => {
            let topic = args.get("topic").and_then(|v| v.as_str()).unwrap();
            let target_area = args.get("area").and_then(|v| v.as_str()).unwrap();
            let source_area = args.get("source_area").and_then(|v| v.as_str());
            let result = crate::commands::topic::run_push(topic, target_area, source_area)?;
            Ok(
                json!({ "success": true, "message": result, "topic": topic, "from": source_area.unwrap_or("Working"), "to": target_area }),
            )
        }
        "topics_pull" => {
            let topic = args.get("topic").and_then(|v| v.as_str()).unwrap();
            let source_area = args.get("source_area").and_then(|v| v.as_str()).unwrap();
            let result = crate::commands::topic::run_pull(topic, source_area)?;
            Ok(
                json!({ "success": true, "message": result, "topic": topic, "from": source_area, "to": "Working" }),
            )
        }
        "topics_progress" => {
            let area = args
                .get("area")
                .and_then(|v| v.as_str())
                .unwrap_or("Working");
            let spec_dir = crate::fs::spec_dir().join(&area);
            if !spec_dir.exists() {
                return Ok(
                    json!({ "success": true, "area": area, "topics": [], "message": "Area does not exist" }),
                );
            }
            let spec_filename = crate::agent::mode::get_spec_filename_for_area(&area);
            let task_filename = crate::agent::mode::get_task_filename_for_area(&area);
            let mut topic_progress = vec![];
            for entry in std::fs::read_dir(&spec_dir)? {
                let entry = entry?;
                if entry.path().is_dir() {
                    let name = entry.file_name().to_string_lossy().to_string();
                    let spec_path = entry.path().join(&spec_filename);
                    let task_path = entry.path().join(&task_filename);
                    let (status, total_tasks, completed_tasks) = if spec_path.exists() {
                        let mut completed = 0;
                        let mut total = 0;
                        if task_path.exists() {
                            if let Ok(content) = std::fs::read_to_string(&task_path) {
                                for line in content.lines() {
                                    if line.trim().starts_with("- [") {
                                        total += 1;
                                        if line.contains("[x]") {
                                            completed += 1;
                                        }
                                    }
                                }
                            }
                        }
                        if total == 0 {
                            ("complete", 0, 0)
                        } else if completed == total {
                            ("complete", total, completed)
                        } else {
                            ("in-progress", total, completed)
                        }
                    } else {
                        ("draft", 0, 0)
                    };
                    topic_progress.push(json!({ "topic": name, "status": status, "total_tasks": total_tasks, "completed_tasks": completed_tasks }));
                }
            }
            Ok(json!({ "success": true, "area": area, "topics": topic_progress }))
        }
        "areas_list" => {
            let areas = crate::fs::list_areas()?;
            Ok(json!({ "success": true, "areas": areas }))
        }
        "areas_add" => {
            let name = args.get("name").and_then(|v| v.as_str()).unwrap();
            let result = crate::commands::area::run_add(name)?;
            Ok(json!({ "success": true, "message": result, "name": name }))
        }
        "areas_remove" => {
            let name = args.get("name").and_then(|v| v.as_str()).unwrap();
            let result = crate::commands::area::run_remove(name)?;
            Ok(json!({ "success": true, "message": result, "name": name }))
        }
        "areas_rename" => {
            let old = args.get("old").and_then(|v| v.as_str()).unwrap();
            let new_name = args.get("new").and_then(|v| v.as_str()).unwrap();
            crate::commands::area::run_rename(old, new_name)?;
            Ok(json!({ "success": true, "old": old, "new": new_name }))
        }
        "areas_default" => {
            let name = args.get("name").and_then(|v| v.as_str()).unwrap();
            crate::commands::area::run_default(name)?;
            Ok(json!({ "success": true, "name": name }))
        }
        "areas_health" => {
            // Get health stats for all areas
            let areas = crate::fs::list_areas()?;
            let mut health_data = vec![];
            for area in areas {
                let area_path = crate::fs::spec_dir().join(&area);
                if !area_path.is_dir() {
                    continue;
                }
                let spec_filename = crate::agent::mode::get_spec_filename_for_area(&area);
                let task_filename = crate::agent::mode::get_task_filename_for_area(&area);
                let mut total_topics = 0;
                let mut completed = 0;
                let mut in_progress = 0;
                let mut draft = 0;
                if let Ok(entries) = std::fs::read_dir(&area_path) {
                    for entry in entries.flatten() {
                        if entry.path().is_dir() {
                            total_topics += 1;
                            let spec_path = entry.path().join(&spec_filename);
                            let task_path = entry.path().join(&task_filename);
                            if spec_path.exists() {
                                let mut all_done = true;
                                if task_path.exists() {
                                    if let Ok(content) = std::fs::read_to_string(&task_path) {
                                        for line in content.lines() {
                                            if line.trim().starts_with("- [ ]") {
                                                all_done = false;
                                                break;
                                            }
                                        }
                                    }
                                }
                                if all_done {
                                    completed += 1;
                                } else {
                                    in_progress += 1;
                                }
                            } else {
                                draft += 1;
                            }
                        }
                    }
                }
                health_data.push(json!({
                    "area": area,
                    "total_topics": total_topics,
                    "completed": completed,
                    "in_progress": in_progress,
                    "draft": draft
                }));
            }
            Ok(json!({ "success": true, "areas": health_data }))
        }
        "index_list" => {
            let topic_filter = args.get("topic").and_then(|v| v.as_str());
            let path_filter = args.get("path").and_then(|v| v.as_str());
            let tag_filter = args.get("tag").and_then(|v| v.as_str());

            let all_links = crate::fs::index::list_all()?;
            let filtered: Vec<serde_json::Value> = all_links
                .iter()
                .filter(|l| {
                    if let Some(t) = topic_filter {
                        if !l.topic.to_lowercase().contains(&t.to_lowercase()) {
                            return false;
                        }
                    }
                    if let Some(p) = path_filter {
                        if !l.path.to_lowercase().contains(&p.to_lowercase()) {
                            return false;
                        }
                    }
                    if let Some(tg) = tag_filter {
                        if !l
                            .tags
                            .iter()
                            .any(|tag| tag.to_lowercase().contains(&tg.to_lowercase()))
                        {
                            return false;
                        }
                    }
                    true
                })
                .map(|l| {
                    json!({
                        "topic": l.topic,
                        "area": l.area,
                        "path": l.path,
                        "type": l.link_type,
                        "tags": l.tags,
                        "annotation": l.annotation
                    })
                })
                .collect();
            Ok(json!({ "success": true, "links": filtered, "count": filtered.len() }))
        }
        "index_add" => {
            let topic = args.get("topic").and_then(|v| v.as_str()).unwrap();
            let path = args.get("path").and_then(|v| v.as_str()).unwrap();
            let area = args
                .get("area")
                .and_then(|v| v.as_str())
                .unwrap_or("Working");
            let link_type = args
                .get("link_type")
                .and_then(|v| v.as_str())
                .map(|s| s.to_string())
                .unwrap_or_else(|| crate::commands::index::detect_type(path));
            let tags = args.get("tags").and_then(|v| v.as_str());
            let annotation = args.get("annotation").and_then(|v| v.as_str());
            let exports = args.get("exports").and_then(|v| v.as_str());
            let descriptions = args.get("descriptions").and_then(|v| v.as_str());
            let export_types = args.get("export_types").and_then(|v| v.as_str());
            let signatures = args.get("signatures").and_then(|v| v.as_str());
            crate::commands::index::run_add(
                topic,
                path,
                area,
                &link_type,
                tags,
                annotation,
                exports,
                descriptions,
                export_types,
                signatures,
            )?;
            Ok(
                json!({ "success": true, "message": "Link added successfully", "topic": topic, "path": path, "area": area }),
            )
        }
        "index_remove" => {
            let topic = args.get("topic").and_then(|v| v.as_str()).unwrap();
            let path = args.get("path").and_then(|v| v.as_str()).unwrap();
            crate::commands::index::run_remove(topic, path)?;
            Ok(json!({ "success": true, "message": "Link removed", "topic": topic, "path": path }))
        }
        "unispec_nav" => {
            let area = args.get("area").and_then(|v| v.as_str());
            let topics = crate::fs::list_areas()?;
            Ok(json!({ "success": true, "areas": topics }))
        }
        "unispec_read_spec" => {
            let topic = args.get("topic").and_then(|v| v.as_str()).unwrap();
            let area = args
                .get("area")
                .and_then(|v| v.as_str())
                .unwrap_or("Working");
            let spec_filename = crate::agent::mode::get_spec_filename_for_area(area);
            let task_filename = crate::agent::mode::get_task_filename_for_area(area);
            let spec_path = crate::fs::spec_dir()
                .join(area)
                .join(topic)
                .join(&spec_filename);
            let task_path = crate::fs::spec_dir()
                .join(area)
                .join(topic)
                .join(&task_filename);
            let spec = std::fs::read_to_string(spec_path).unwrap_or_default();
            let tasks = std::fs::read_to_string(task_path).unwrap_or_default();
            Ok(
                json!({ "success": true, "spec": spec, "tasks": tasks, "spec_file": spec_filename, "task_file": task_filename }),
            )
        }
        "unispec_update_task" => {
            let topic = args.get("topic").and_then(|v| v.as_str()).unwrap();
            let task_index = args.get("task_index").and_then(|v| v.as_u64()).unwrap() as usize;
            let status = args.get("status").and_then(|v| v.as_str()).unwrap();
            let note = args.get("note").and_then(|v| v.as_str()).unwrap_or("");
            let area = args
                .get("area")
                .and_then(|v| v.as_str())
                .unwrap_or("Working");
            let task_filename = crate::agent::mode::get_task_filename_for_area(area);
            let path = crate::fs::spec_dir()
                .join(area)
                .join(topic)
                .join(&task_filename);
            if !path.exists() {
                return Err(anyhow::anyhow!("Task file not found: {:?}", path));
            }
            let mut lines: Vec<String> = std::fs::read_to_string(&path)?
                .lines()
                .map(|s| s.to_string())
                .collect();
            let mut updated = false;
            if let Some(line) = lines.get_mut(task_index) {
                // Check if it's a task line
                if line.trim().starts_with("- [") {
                    let task_content = line.split("Task: ").nth(1).unwrap_or("").trim();
                    *line = format!("- [{}] Task: {} - Note: {}", status, task_content, note);
                    updated = true;
                }
            }
            if !updated {
                return Err(anyhow::anyhow!("Task at index {} not found", task_index));
            }
            std::fs::write(&path, lines.join("\n"))?;
            Ok(
                json!({ "success": true, "message": "Task updated", "topic": topic, "task_index": task_index, "status": status }),
            )
        }
        "unispec_query_relations" => {
            let symbol = args.get("symbol").and_then(|v| v.as_str()).unwrap();
            let callers = crate::fs::index::find_callers(symbol)?;
            Ok(json!({ "success": true, "callers": callers }))
        }
        "unispec_write_code" => {
            let topic = args.get("topic").and_then(|v| v.as_str()).unwrap();
            let area = args
                .get("area")
                .and_then(|v| v.as_str())
                .unwrap_or("Working");
            let path = args.get("path").and_then(|v| v.as_str()).unwrap();
            let content = args.get("content").and_then(|v| v.as_str()).unwrap();

            let spec_path = crate::fs::spec_dir().join(area).join(topic).join("spec.md");
            let spec_content = std::fs::read_to_string(&spec_path)?;
            let metadata = crate::fs::spec::parse_spec_metadata(&spec_content)
                .ok_or_else(|| anyhow::anyhow!("Spec metadata not found"))?;

            // Gatekeeper: Verify binding
            let binding = spec_content
                .lines()
                .find(|l| l.trim().starts_with("binding:"))
                .map(|l| l.replace("binding:", "").trim().to_string())
                .ok_or_else(|| anyhow::anyhow!("No binding found in spec"))?;

            if !path.ends_with(&binding) {
                return Err(anyhow::anyhow!(
                    "File path {} is not bound to spec binding {}",
                    path,
                    binding
                ));
            }

            std::fs::write(path, content)?;
            Ok(json!({ "success": true }))
        }
        "unispec_auto_build" => {
            let topic = args.get("topic").and_then(|v| v.as_str()).unwrap();
            let area = args
                .get("area")
                .and_then(|v| v.as_str())
                .unwrap_or("Working");
            let result = crate::agent::auto::build::run_auto_build(topic, Some(area), None)?;
            Ok(json!({ "success": true, "result": format!("{:?}", result) }))
        }
        "unispec_auto_verify" => {
            let topic = args.get("topic").and_then(|v| v.as_str()).unwrap();
            let fix = args.get("fix").and_then(|v| v.as_bool()).unwrap_or(false);
            let result = crate::agent::auto::verify::run_auto_verify(topic, Some("Working"))?;
            Ok(json!({ "success": true, "result": format!("{:?}", result) }))
        }
        "unispec_bind_spec" => {
            let spec_path = args.get("spec_path").and_then(|v| v.as_str()).unwrap();
            let file_path = args.get("file_path").and_then(|v| v.as_str()).unwrap();
            let topic = args.get("topic").and_then(|v| v.as_str()).unwrap();
            let area = args
                .get("area")
                .and_then(|v| v.as_str())
                .unwrap_or("Working");
            crate::fs::spec::bind_spec_to_file(
                std::path::Path::new(spec_path),
                file_path,
                topic,
                area,
            )?;
            Ok(json!({ "success": true }))
        }
        "index_find" => {
            let query = args.get("query").and_then(|v| v.as_str()).unwrap();
            let by = args.get("by").and_then(|v| v.as_str()).unwrap_or("topic");

            let links = match by {
                "topic" => crate::fs::index::find_by_topic(query)?,
                "path" => crate::fs::index::find_by_path(query)?,
                "tag" => crate::fs::index::find_by_tag(query)?,
                "annotation" => crate::fs::index::find_by_annotation(query)?,
                _ => return Err(anyhow::anyhow!("Unknown search type: {}", by)),
            };

            let links_json: Vec<serde_json::Value> = links
                .iter()
                .map(|l| {
                    serde_json::json!({
                        "topic": l.topic,
                        "area": l.area,
                        "path": l.path,
                        "type": l.link_type,
                        "tags": l.tags,
                        "annotation": l.annotation
                    })
                })
                .collect();

            Ok(json!({ "success": true, "links": links_json }))
        }
        "index_cleanup" => {
            crate::commands::index::run_cleanup()?;
            Ok(json!({ "success": true, "message": "Cleanup completed" }))
        }
        "index_tags" => {
            let tags = crate::fs::index::list_all_tags()?;
            Ok(json!({ "success": true, "tags": tags, "count": tags.len() }))
        }
        "index_graph" => {
            let graph = crate::fs::index::export_graph()?;
            Ok(json!({ "success": true, "graph": graph }))
        }
        "index_backlinks" => {
            let topic = args.get("topic").and_then(|v| v.as_str()).unwrap();
            let md = crate::fs::index::generate_backlinks_file(topic, "Working")?;
            Ok(json!({ "success": true, "topic": topic, "backlinks": md }))
        }
        "index_exports" => {
            let topic = args.get("topic").and_then(|v| v.as_str());
            if let Some(t) = topic {
                let exports = crate::fs::index::get_exports_for_topic(t)?;
                let exports_json: Vec<serde_json::Value> = exports
                    .iter()
                    .map(|e| {
                        serde_json::json!({
                            "id": e.id,
                            "name": e.name,
                            "type": e.export_type,
                            "description": e.description,
                            "signature": e.signature
                        })
                    })
                    .collect();
                Ok(json!({ "success": true, "exports": exports_json }))
            } else {
                Ok(json!({ "success": true, "exports": [] }))
            }
        }
        "index_query" => {
            let query = args.get("query").and_then(|v| v.as_str()).unwrap();
            let by = args.get("by").and_then(|v| v.as_str()).unwrap_or("name");
            let results = crate::fs::index::find_exports(query, by)?;
            let results_json: Vec<serde_json::Value> = results
                .iter()
                .map(|r| {
                    serde_json::json!({
                        "id": r.id,
                        "topic": r.topic,
                        "path": r.path,
                        "name": r.name,
                        "type": r.export_type,
                        "description": r.description,
                        "signature": r.signature
                    })
                })
                .collect();
            Ok(json!({ "success": true, "results": results_json }))
        }
        "index_depends" => {
            let topic = args.get("topic").and_then(|v| v.as_str()).unwrap();
            let dependents = crate::fs::index::get_dependents(topic)?;
            let deps_json: Vec<serde_json::Value> = dependents
                .iter()
                .map(|d| {
                    serde_json::json!({
                        "topic": d.topic,
                        "id": d.id,
                        "name": d.name,
                        "type": d.export_type
                    })
                })
                .collect();
            Ok(json!({ "success": true, "dependents": deps_json }))
        }
        "index_lookup" => {
            let id = args.get("id").and_then(|v| v.as_str()).unwrap();
            let result = crate::fs::index::find_export_by_id(id)?;
            match result {
                Some(exp) => Ok(json!({
                    "success": true,
                    "export": {
                        "id": exp.id,
                        "topic": exp.topic,
                        "path": exp.path,
                        "name": exp.name,
                        "type": exp.export_type,
                        "description": exp.description,
                        "signature": exp.signature
                    }
                })),
                None => Ok(json!({ "success": false, "error": "Export not found" })),
            }
        }
        "config_get" => {
            let config = crate::fs::config::load_config()?;
            Ok(json!({ "success": true, "area": config.area }))
        }
        "config_set" => {
            let area = args.get("area").and_then(|v| v.as_str()).unwrap();
            crate::commands::set::run_set(area)?;
            Ok(json!({ "success": true, "message": "Default area set", "area": area }))
        }
        "mode_list" => {
            let modes = crate::agent::mode::list_modes()?;
            Ok(json!({ "success": true, "modes": modes, "count": modes.len() }))
        }
        "mode_info" => {
            let name = args.get("name").and_then(|v| v.as_str()).unwrap();
            let config = crate::agent::mode::get_mode_info(name)?;
            Ok(json!({ "success": true, "mode": config }))
        }
        "mode_activate" => {
            let name = args.get("name").and_then(|v| v.as_str()).unwrap();
            let result = crate::agent::mode::run_activate(name)?;
            Ok(json!({ "success": true, "message": result, "mode": name }))
        }
        "mode_current" => {
            let current = crate::agent::current_mode()?;
            Ok(json!({ "success": true, "mode": current }))
        }
        "connector_list" => {
            let config = crate::agent::load_agent_config().unwrap_or_default();
            let connectors: Vec<serde_json::Value> = config.connectors.iter().map(|c| {
                json!({ "name": c.name, "description": c.description, "command": c.command, "args": c.args })
            }).collect();
            Ok(json!({ "success": true, "connectors": connectors, "count": connectors.len() }))
        }
        "connector_run" => {
            let name = args.get("name").and_then(|v| v.as_str()).unwrap();
            let extra_args: Vec<String> = args
                .get("args")
                .and_then(|v| v.as_array())
                .map(|arr| {
                    arr.iter()
                        .filter_map(|v| v.as_str().map(String::from))
                        .collect()
                })
                .unwrap_or_default();
            let output = crate::agent::connector::run_run(name, &extra_args)?;
            Ok(json!({ "success": true, "output": output }))
        }
        // === Spec Writing ===
        "spec_write" => {
            let topic = args.get("topic").and_then(|v| v.as_str()).unwrap();
            let area = args
                .get("area")
                .and_then(|v| v.as_str())
                .unwrap_or("Working");
            let content = args.get("content").and_then(|v| v.as_str()).unwrap();

            let spec_filename = crate::agent::mode::get_spec_filename_for_area(area);

            // Try lowercase first, then uppercase
            let spec_dir_candidate = crate::fs::spec_dir().join(area.to_lowercase()).join(topic);
            let spec_dir = if spec_dir_candidate.exists() {
                spec_dir_candidate
            } else {
                let upper = crate::fs::spec_dir().join(area).join(topic);
                if upper.exists() {
                    upper
                } else {
                    std::fs::create_dir_all(&spec_dir_candidate)?;
                    spec_dir_candidate
                }
            };

            let spec_path = spec_dir.join(&spec_filename);
            std::fs::write(&spec_path, content)?;
            Ok(
                json!({ "success": true, "message": "Spec written", "topic": topic, "area": area, "file": spec_filename }),
            )
        }
        "spec_read" => {
            let topic = args.get("topic").and_then(|v| v.as_str()).unwrap();
            let area = args
                .get("area")
                .and_then(|v| v.as_str())
                .unwrap_or("Working");

            let spec_filename = crate::agent::mode::get_spec_filename_for_area(area);

            // Try both lowercase and uppercase
            let spec_path_lower = crate::fs::spec_dir()
                .join(area.to_lowercase())
                .join(topic)
                .join(&spec_filename);
            let spec_path_upper = crate::fs::spec_dir()
                .join(area)
                .join(topic)
                .join(&spec_filename);

            let spec_path = if spec_path_upper.exists() {
                spec_path_upper
            } else if spec_path_lower.exists() {
                spec_path_lower
            } else {
                return Err(anyhow::anyhow!(
                    "Spec file not found for topic '{}' in area '{}'",
                    topic,
                    area
                ));
            };

            let content = std::fs::read_to_string(&spec_path)?;
            Ok(json!({ "success": true, "topic": topic, "area": area, "content": content }))
        }
        // === Task Writing ===
        "task_write" => {
            let topic = args.get("topic").and_then(|v| v.as_str()).unwrap();
            let area = args
                .get("area")
                .and_then(|v| v.as_str())
                .unwrap_or("Working");
            let content = args.get("content").and_then(|v| v.as_str()).unwrap();

            let task_filename = crate::agent::mode::get_task_filename_for_area(area);

            // Try lowercase first, then uppercase
            let spec_dir_candidate = crate::fs::spec_dir().join(area.to_lowercase()).join(topic);
            let spec_dir = if spec_dir_candidate.exists() {
                spec_dir_candidate
            } else {
                let upper = crate::fs::spec_dir().join(area).join(topic);
                if upper.exists() {
                    upper
                } else {
                    std::fs::create_dir_all(&spec_dir_candidate)?;
                    spec_dir_candidate
                }
            };

            let task_path = spec_dir.join(&task_filename);
            std::fs::write(&task_path, content)?;
            Ok(
                json!({ "success": true, "message": "Task file written", "topic": topic, "area": area, "file": task_filename }),
            )
        }
        "task_read" => {
            let topic = args.get("topic").and_then(|v| v.as_str()).unwrap();
            let area = args
                .get("area")
                .and_then(|v| v.as_str())
                .unwrap_or("Working");

            let task_filename = crate::agent::mode::get_task_filename_for_area(area);

            // Try both lowercase and uppercase
            let task_path_lower = crate::fs::spec_dir()
                .join(area.to_lowercase())
                .join(topic)
                .join(&task_filename);
            let task_path_upper = crate::fs::spec_dir()
                .join(area)
                .join(topic)
                .join(&task_filename);

            let task_path = if task_path_upper.exists() {
                task_path_upper
            } else if task_path_lower.exists() {
                task_path_lower
            } else {
                return Err(anyhow::anyhow!(
                    "Task file not found for topic '{}' in area '{}'",
                    topic,
                    area
                ));
            };

            let content = std::fs::read_to_string(&task_path)?;
            Ok(json!({ "success": true, "topic": topic, "area": area, "content": content }))
        }
        // === Spec Add (creates <topic>_spec.md and <topic>_task.md from templates) ===
        "spec_add" => {
            let topic = args.get("topic").and_then(|v| v.as_str()).unwrap();
            let area = args
                .get("area")
                .and_then(|v| v.as_str())
                .unwrap_or("Working");

            // Create safe filename from topic (replace / with _)
            let topic_safe = topic.replace("/", "-").replace(" ", "-");

            // Filename is <topic>_spec.md and <topic>_task.md
            let spec_filename = format!("{}_spec.md", topic_safe);
            let task_filename = format!("{}_task.md", topic_safe);

            // Find or create the topic directory (supports nested paths like "user-auth/user-login")
            let spec_dir = {
                let upper = crate::fs::spec_dir().join(area).join(topic);
                if upper.exists() {
                    upper
                } else {
                    let lower = crate::fs::spec_dir().join(area.to_lowercase()).join(topic);
                    if lower.exists() {
                        lower
                    } else {
                        std::fs::create_dir_all(&upper)?;
                        upper
                    }
                }
            };

            // Read templates for content
            let base_spec_template = crate::fs::read_area_template(area, "spec.md")
                .or_else(|| crate::fs::read_template("spec.md"))
                .unwrap_or_else(|| {
                    "# {topic}\n\n## Overview\n\n## Requirements\n\n## Acceptance Criteria\n"
                        .to_string()
                });
            let base_task_template = crate::fs::read_area_template(area, "task.md")
                .or_else(|| crate::fs::read_template("task.md"))
                .unwrap_or_else(|| {
                    "# Tasks - {topic}\n\n## Tasks\n- [ ] Task 1\n\n## Notes\n".to_string()
                });

            // Replace {topic} placeholder
            let spec_template = base_spec_template.replace("{topic}", topic);
            let task_template = base_task_template.replace("{topic}", topic);

            // Write files
            let spec_path = spec_dir.join(&spec_filename);
            let task_path = spec_dir.join(&task_filename);

            std::fs::write(&spec_path, spec_template)?;
            std::fs::write(&task_path, task_template)?;

            Ok(json!({
                "success": true,
                "message": "Spec and task files created from templates",
                "topic": topic,
                "area": area,
                "spec_file": spec_filename,
                "task_file": task_filename
            }))
        }
        name => {
            // Check if it's a dynamic connector tool
            if name.starts_with("unispec_") {
                let connector_name = &name[8..]; // Remove "unispec_" prefix (8 chars)
                let output = crate::agent::connector::run_run(connector_name, &[])?;
                Ok(json!({ "success": true, "output": output }))
            } else {
                Err(anyhow::anyhow!("Unknown tool: {}", name))
            }
        }
    }
}

fn send_response(stdout: &mut impl Write, id: Option<Value>, result: Value) -> Result<()> {
    let response = json!({
        "jsonrpc": "2.0",
        "id": id,
        "result": result
    });

    let response_str = serde_json::to_string(&response)?;

    // Send with newline - Zed expects simple JSON lines
    stdout.write_all(response_str.as_bytes())?;
    stdout.write_all(b"\n")?;
    stdout.flush()?;

    Ok(())
}

fn send_error(stdout: &mut impl Write, id: Option<Value>, code: i32, message: &str) -> Result<()> {
    let response = json!({
        "jsonrpc": "2.0",
        "id": id,
        "error": {
            "code": code,
            "message": message
        }
    });

    let response_str = serde_json::to_string(&response)?;

    // Send with newline - Zed expects simple JSON lines
    stdout.write_all(response_str.as_bytes())?;
    stdout.write_all(b"\n")?;
    stdout.flush()?;

    Ok(())
}

fn handle_request(request: &Value, stdout: &mut impl Write) -> Result<()> {
    let id = request.get("id").cloned();
    let method = request.get("method").and_then(|v| v.as_str()).unwrap_or("");

    match method {
        "initialize" => {
            let result = json!({
                "protocolVersion": "2024-11-05",
                "capabilities": {
                    "tools": {}
                },
                "serverInfo": {
                    "name": "unispec",
                    "version": crate::version::VERSION
                }
            });
            send_response(stdout, id, result)?;
        }
        "tools/list" => {
            let tools: Vec<Value> = crate::mcp::get_tools()
                .iter()
                .map(|t| {
                    json!({
                        "name": t.name,
                        "description": t.description,
                        "inputSchema": t.input_schema
                    })
                })
                .collect();
            let result = json!({ "tools": tools });
            send_response(stdout, id, result)?;
        }
        "tools/call" => {
            let name = request
                .get("params")
                .and_then(|p| p.get("name"))
                .and_then(|v| v.as_str())
                .unwrap_or("");
            let arguments = request
                .get("params")
                .and_then(|p| p.get("arguments"))
                .cloned()
                .unwrap_or(json!({}));

            match call_tool(name, &arguments) {
                Ok(result) => {
                    let response_result = json!({
                        "content": [{
                            "type": "text",
                            "text": serde_json::to_string_pretty(&result).unwrap_or_default()
                        }],
                        "isError": false
                    });
                    send_response(stdout, id, response_result)?;
                }
                Err(e) => {
                    let response_result = json!({
                        "content": [{
                            "type": "text",
                            "text": format!("Error: {}", e)
                        }],
                        "isError": true
                    });
                    send_response(stdout, id, response_result)?;
                }
            }
        }
        "notifications/initialized" => {}
        "notifications/loggingMessage" => {}
        "logging/setLevel" => {}
        _ => {
            if id.is_some() {
                send_error(stdout, id, -32601, &format!("Method not found: {}", method))?;
            }
        }
    }
    Ok(())
}

pub fn run_mcp_server(project_path: Option<&str>) -> Result<()> {
    // Change to project directory if specified
    if let Some(path) = project_path {
        std::env::set_current_dir(path)?;
    }

    let mut stdin = std::io::stdin();
    let stdout = std::io::stdout();

    let mut stdout = stdout;
    let mut input = String::new();
    let mut depth = 0;
    let mut in_string = false;
    let mut escaped = false;

    loop {
        // Read all available input
        let mut buf = [0u8; 1024];
        let n = stdin.read(&mut buf)?;

        if n == 0 {
            return Ok(());
        }

        // Process character by character to find complete JSON objects
        for &byte in &buf[..n] {
            let ch = byte as char;

            // Handle escape sequences in strings
            if escaped {
                escaped = false;
                input.push(ch);
                continue;
            }

            if ch == '\\' && in_string {
                escaped = true;
                input.push(ch);
                continue;
            }

            // Track string boundaries
            if ch == '"' {
                in_string = !in_string;
            }

            // Track braces only outside strings
            if !in_string {
                if ch == '{' {
                    depth += 1;
                } else if ch == '}' {
                    depth -= 1;
                }
            }

            input.push(ch);

            // If we've closed the top-level object, try to parse it
            if depth == 0 && !input.trim().is_empty() {
                if let Ok(request) = serde_json::from_str::<Value>(input.trim()) {
                    let _ = handle_request(&request, &mut stdout);
                }
                input.clear();
            }
        }
    }
}
