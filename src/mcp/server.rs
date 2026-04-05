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
            crate::commands::topic::run_list(area, false)?;
            Ok(json!({ "success": true }))
        }
        "topics_add" => {
            let topic = args.get("topic").and_then(|v| v.as_str()).unwrap();
            let area = args
                .get("area")
                .and_then(|v| v.as_str())
                .unwrap_or("Working");
            let result = crate::commands::topic::run_new(topic, area)?;
            Ok(json!({ "success": true, "message": result }))
        }
        "topics_show" => {
            let topic = args.get("topic").and_then(|v| v.as_str()).unwrap();
            let show_all = args
                .get("show_all")
                .and_then(|v| v.as_bool())
                .unwrap_or(false);
            let from_area = args.get("from").and_then(|v| v.as_str());
            crate::commands::topic::run_show(topic, show_all, from_area)?;
            Ok(json!({ "success": true }))
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
            let area = args.get("area").and_then(|v| v.as_str()).unwrap();
            let source_area = args.get("source_area").and_then(|v| v.as_str());
            let result = crate::commands::topic::run_push(topic, area, source_area)?;
            Ok(json!({ "success": true, "message": result }))
        }
        "topics_pull" => {
            let topic = args.get("topic").and_then(|v| v.as_str()).unwrap();
            let source_area = args.get("source_area").and_then(|v| v.as_str()).unwrap();
            let result = crate::commands::topic::run_pull(topic, source_area)?;
            Ok(json!({ "success": true, "message": result }))
        }
        "topics_progress" => {
            let area = args.get("area").and_then(|v| v.as_str());
            crate::commands::topic::run_progress(area)?;
            Ok(json!({ "success": true }))
        }
        "areas_list" => {
            crate::commands::area::run_list()?;
            Ok(json!({ "success": true }))
        }
        "areas_add" => {
            let name = args.get("name").and_then(|v| v.as_str()).unwrap();
            let result = crate::commands::area::run_add(name)?;
            Ok(json!({ "success": true, "message": result }))
        }
        "areas_remove" => {
            let name = args.get("name").and_then(|v| v.as_str()).unwrap();
            let result = crate::commands::area::run_remove(name)?;
            Ok(json!({ "success": true, "message": result }))
        }
        "areas_rename" => {
            let old = args.get("old").and_then(|v| v.as_str()).unwrap();
            let new = args.get("new").and_then(|v| v.as_str()).unwrap();
            crate::commands::area::run_rename(old, new)?;
            Ok(json!({ "success": true }))
        }
        "areas_default" => {
            let name = args.get("name").and_then(|v| v.as_str()).unwrap();
            crate::commands::area::run_default(name)?;
            Ok(json!({ "success": true }))
        }
        "areas_health" => {
            crate::commands::area::run_health()?;
            Ok(json!({ "success": true }))
        }
        "index_list" => {
            let topic = args.get("topic").and_then(|v| v.as_str());
            let path = args.get("path").and_then(|v| v.as_str());
            let tag = args.get("tag").and_then(|v| v.as_str());
            crate::commands::index::run_list(topic, path, tag)?;
            Ok(json!({ "success": true }))
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
            Ok(json!({ "success": true }))
        }
        "index_remove" => {
            let topic = args.get("topic").and_then(|v| v.as_str()).unwrap();
            let path = args.get("path").and_then(|v| v.as_str()).unwrap();
            crate::commands::index::run_remove(topic, path)?;
            Ok(json!({ "success": true }))
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
            Ok(json!({ "success": true }))
        }
        "index_tags" => {
            let tags = crate::fs::index::list_all_tags()?;
            Ok(json!({ "success": true, "tags": tags }))
        }
        "index_graph" => {
            let graph = crate::fs::index::export_graph()?;
            Ok(json!({ "success": true, "graph": graph }))
        }
        "index_backlinks" => {
            let topic = args.get("topic").and_then(|v| v.as_str()).unwrap();
            let md = crate::fs::index::generate_backlinks_file(topic, "Working")?;
            Ok(json!({ "success": true, "backlinks": md }))
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
            Ok(json!({ "success": true }))
        }
        "mode_list" => {
            let modes = crate::agent::mode::list_modes()?;
            Ok(json!({ "success": true, "modes": modes }))
        }
        "mode_info" => {
            let name = args.get("name").and_then(|v| v.as_str()).unwrap();
            let config = crate::agent::mode::get_mode_info(name)?;
            Ok(json!({ "success": true, "mode": config }))
        }
        "mode_activate" => {
            let name = args.get("name").and_then(|v| v.as_str()).unwrap();
            let result = crate::agent::mode::run_activate(name)?;
            Ok(json!({ "success": true, "message": result }))
        }
        "mode_current" => {
            let current = crate::agent::current_mode()?;
            Ok(json!({ "success": true, "mode": current }))
        }
        "connector_list" => {
            crate::agent::connector::run_list()?;
            Ok(json!({ "success": true }))
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
        "code_analysis" => {
            let topic = args.get("topic").and_then(|v| v.as_str()).unwrap();
            let area = args
                .get("area")
                .and_then(|v| v.as_str())
                .unwrap_or("Ingested");
            let module = args.get("module").and_then(|v| v.as_str());
            let item_type = args
                .get("item_type")
                .and_then(|v| v.as_str())
                .unwrap_or("all");
            let pattern = args.get("pattern").and_then(|v| v.as_str());

            let result = crate::agent::code_parser::query_code_analysis(
                topic, area, module, item_type, pattern,
            )?;
            Ok(json!({ "success": true, "data": result }))
        }
        "code_parse" => {
            let path = args.get("path").and_then(|v| v.as_str()).unwrap();
            let language = args.get("language").and_then(|v| v.as_str());
            let item_type = args
                .get("item_type")
                .and_then(|v| v.as_str())
                .unwrap_or("all");
            let pattern = args.get("pattern").and_then(|v| v.as_str());

            let result =
                crate::agent::code_parser::parse_file_to_json(path, language, item_type, pattern)?;
            Ok(
                json!({ "success": true, "data": serde_json::from_str::<serde_json::Value>(&result).unwrap() }),
            )
        }
        "master_spec" => {
            let master_path = crate::fs::spec_dir().join("master.md");
            if master_path.exists() {
                let content = std::fs::read_to_string(&master_path)?;
                Ok(json!({ "success": true, "content": content }))
            } else {
                Ok(
                    json!({ "success": false, "error": "No master spec found. Create spec/master.md" }),
                )
            }
        }
        "auto_build" => {
            let topic = args.get("topic").and_then(|v| v.as_str()).unwrap();
            let area = args.get("area").and_then(|v| v.as_str());
            let spec_file = args.get("spec_file").and_then(|v| v.as_str());
            let result = crate::agent::auto::run_auto_build(topic, area, spec_file)?;
            Ok(json!({ "success": true, "result": result }))
        }
        "auto_ingest" => {
            let code_path = args.get("code_path").and_then(|v| v.as_str()).unwrap();
            let master_spec = args.get("master_spec").and_then(|v| v.as_str()).unwrap();
            let topic = args.get("topic").and_then(|v| v.as_str());
            let area = args.get("area").and_then(|v| v.as_str());
            let result = crate::agent::auto::run_auto_ingest(code_path, master_spec, topic, area)?;
            Ok(json!({ "success": true, "result": result }))
        }
        "auto_verify" => {
            let topic = args.get("topic").and_then(|v| v.as_str()).unwrap();
            let area = args.get("area").and_then(|v| v.as_str());
            let result = crate::agent::auto::run_auto_verify(topic, area)?;
            Ok(json!({ "success": true, "result": result }))
        }
        "auto_locks_list" => {
            let locks = crate::agent::auto::list_locks()?;
            let locks_json: Vec<serde_json::Value> = locks
                .iter()
                .map(|l| {
                    serde_json::json!({
                        "session_id": l.session_id,
                        "topic": l.topic,
                        "parent_topic": l.parent_topic,
                        "last_task": l.last_task,
                        "error_message": l.error_message,
                        "timestamp": l.timestamp,
                        "status": l.status
                    })
                })
                .collect();
            Ok(json!({ "success": true, "locks": locks_json }))
        }
        "auto_locks_clear" => {
            let session_id = args.get("session_id").and_then(|v| v.as_str()).unwrap();
            crate::agent::auto::clear_lock(session_id)?;
            Ok(json!({ "success": true, "message": format!("Lock {} cleared", session_id) }))
        }
        "auto_commits" => {
            let topic = args.get("topic").and_then(|v| v.as_str()).unwrap();
            let commits = crate::agent::auto::get_commits(topic)?;
            let commits_json: Vec<serde_json::Value> = commits
                .iter()
                .map(|c| {
                    serde_json::json!({
                        "id": c.id,
                        "topic": c.topic,
                        "parent_topic": c.parent_topic,
                        "description": c.description,
                        "files": c.files,
                        "timestamp": c.timestamp,
                        "status": "completed"
                    })
                })
                .collect();
            Ok(json!({ "success": true, "commits": commits_json }))
        }
        "auto_topic_tree" => {
            let topic = args.get("topic").and_then(|v| v.as_str()).unwrap();
            let tree = crate::agent::auto::get_topic_tree(topic)?;
            let tree_json = serde_json::to_value(&tree)?;
            Ok(json!({ "success": true, "tree": tree_json }))
        }
        "auto_agent" => {
            let topic = args.get("topic").and_then(|v| v.as_str()).unwrap();
            let session_id = args.get("session_id").and_then(|v| v.as_str());
            let parent_topic = args.get("parent_topic").and_then(|v| v.as_str());
            let area = args.get("area").and_then(|v| v.as_str());
            let workflow = args.get("workflow").and_then(|v| v.as_str());
            let result =
                crate::agent::auto::run_agent(topic, session_id, parent_topic, area, workflow)?;
            Ok(json!({ "success": true, "result": result }))
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
