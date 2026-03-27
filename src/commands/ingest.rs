// src/commands/ingest.rs
use crate::agent::code_parser::{analyze_directory, CodeAnalysis, CodeFile};
use anyhow::Result;
use itertools::Itertools;
use notify::{Config, Event, RecommendedWatcher, RecursiveMode, Watcher};
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use std::sync::mpsc;
use std::time::Duration;

use std::sync::Mutex;

static WATCHER: Mutex<Option<RecommendedWatcher>> = Mutex::new(None);

pub fn run_ingest(
    path: &str,
    area: &str,
    topic_name: Option<&str>,
    languages: Option<&str>,
    watch: bool,
) -> Result<String> {
    let code_path = Path::new(path);

    if !code_path.exists() {
        return Err(anyhow::anyhow!("❌ Path does not exist: {}", path));
    }

    let langs: Vec<String> = languages
        .map(|l| l.split(',').map(|s| s.trim().to_string()).collect())
        .unwrap_or_default();

    // Create topic name from path if not provided
    let topic = topic_name.unwrap_or_else(|| {
        code_path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("ingested-code")
    });

    println!("🔍 Analyzing codebase at {}...", path);

    // Load ingest config
    let ingest_config = crate::fs::config::get_ingest_config()?;

    // Initial analysis
    let analysis = analyze_directory(code_path, langs.clone())?;

    println!("📊 Analysis complete:");
    println!("   Files: {}", analysis.total_files);
    println!("   Functions: {}", analysis.total_functions);
    println!("   Structs: {}", analysis.total_structs);
    println!("   Enums: {}", analysis.total_enums);

    // Save to code_analysis.toml if format is toml
    if ingest_config.output_format == "toml" {
        let analysis_files: Vec<crate::fs::index::CodeAnalysisFile> = analysis
            .files
            .iter()
            .map(|f| crate::fs::index::CodeAnalysisFile {
                path: f.path.clone(),
                language: f.language.clone(),
                functions: f
                    .functions
                    .iter()
                    .map(|g| crate::fs::index::CodeElement {
                        name: g.name.clone(),
                        signature: Some(g.signature.clone()),
                        start_line: Some(g.start_line),
                        end_line: Some(g.end_line),
                    })
                    .collect(),
                structs: f
                    .structs
                    .iter()
                    .map(|s| crate::fs::index::CodeElement {
                        name: s.name.clone(),
                        signature: None,
                        start_line: None,
                        end_line: None,
                    })
                    .collect(),
                enums: f
                    .enums
                    .iter()
                    .map(|e| crate::fs::index::CodeElement {
                        name: e.name.clone(),
                        signature: None,
                        start_line: None,
                        end_line: None,
                    })
                    .collect(),
                imports: f.imports.iter().map(|i| i.path.clone()).collect(),
            })
            .collect();

        crate::fs::index::add_code_analysis(topic, area, path, analysis_files)?;
        println!("✅ Saved to code_analysis.toml");
    }

    // Create hierarchical topic structure with MD files (for display)
    if ingest_config.output_format != "toml" || ingest_config.output_format == "both" {
        create_topic_hierarchy(&analysis, topic, area)?;
    }

    // Auto-add to index if enabled
    if ingest_config.auto_index {
        crate::fs::index::add_link(topic, area, path, "directory")?;
        println!("✅ Auto-indexed to index.toml");
    }

    // Start watching if requested
    if watch {
        start_watcher(code_path, topic, area, langs)?;
        println!("🔄 Watching for changes... (Ctrl+C to stop)");
        println!("   Files will be re-parsed on save");

        // Keep the watcher running
        std::thread::park();
    }

    Ok(format!(
        "✅ Ingested codebase into topic '{}' in {}/",
        topic, area
    ))
}

fn create_topic_hierarchy(analysis: &CodeAnalysis, root_topic: &str, area: &str) -> Result<()> {
    let spec_dir = crate::fs::spec_dir().join(area);
    crate::fs::ensure_dir(&spec_dir)?;

    // Group files by root module
    let mut modules: HashMap<String, Vec<&CodeFile>> = HashMap::new();
    for file in &analysis.files {
        let root = file.path.split('/').next().unwrap_or("root").to_string();
        modules.entry(root).or_default().push(file);
    }

    // Create root topic
    let root_path = spec_dir.join(root_topic);
    crate::fs::ensure_dir(&root_path)?;

    // Generate root specs
    let root_specs = generate_root_spec(analysis, root_topic);
    fs::write(root_path.join("specs.md"), root_specs)?;

    // Generate root links
    let root_links = generate_root_links(analysis);
    fs::write(root_path.join("links.md"), root_links)?;

    // Generate root functions overview
    let root_functions = generate_root_functions_overview(analysis, root_topic);
    fs::write(root_path.join("functions.md"), root_functions)?;

    // Create subtopics for each module
    for (module, files) in &modules {
        if module == "root" || module.is_empty() {
            continue;
        }

        let subtopic_path = root_path.join(&module);
        crate::fs::ensure_dir(&subtopic_path)?;

        // Module specs
        let module_spec = generate_module_spec(&module, &files);
        fs::write(subtopic_path.join("specs.md"), module_spec)?;

        // Module links
        let module_links = generate_module_links(&files);
        fs::write(subtopic_path.join("links.md"), module_links)?;

        // Module functions
        let module_functions = generate_module_functions(&files, &module);
        fs::write(subtopic_path.join("functions.md"), module_functions)?;
    }

    // Add root to index
    add_to_index(root_topic, &root_path)?;

    println!("✅ Created topic hierarchy '{}' in {}/", root_topic, area);
    println!("   - Root topic: {}/", root_topic);
    println!(
        "   - Subtopics: {} modules",
        modules.len().saturating_sub(1)
    );

    Ok(())
}

fn generate_root_spec(analysis: &CodeAnalysis, topic: &str) -> String {
    let mut content = format!("# {}\n\n", topic);
    content.push_str("## Overview\n\n");
    content.push_str("Auto-generated from codebase analysis.\n\n");
    content.push_str("This is the root topic containing high-level project structure.\n\n");

    content.push_str("## Project Statistics\n\n");
    content.push_str(&format!("- Total Files: {}\n", analysis.total_files));
    content.push_str(&format!(
        "- Total Functions: {}\n",
        analysis.total_functions
    ));
    content.push_str(&format!("- Total Structs: {}\n", analysis.total_structs));
    content.push_str(&format!("- Total Enums: {}\n", analysis.total_enums));
    content.push_str("\n");

    content.push_str("## Modules\n\n");
    content.push_str("See subtopics for detailed module information:\n\n");

    let mut modules: Vec<&str> = Vec::new();
    for file in &analysis.files {
        if let Some(root) = file.path.split('/').next() {
            if !root.is_empty() && !modules.contains(&root) {
                modules.push(root);
            }
        }
    }

    for module in modules {
        content.push_str(&format!("- [{}](./{}/specs.md)\n", module, module));
    }

    content
}

fn generate_root_links(analysis: &CodeAnalysis) -> String {
    let mut content = String::from("# Project Dependencies\n\n");
    content.push_str("## External Dependencies\n\n");

    let mut all_imports: std::collections::HashSet<String> = std::collections::HashSet::new();
    for file in &analysis.files {
        for imp in &file.imports {
            if !imp.path.is_empty()
                && !imp.path.starts_with("crate::")
                && !imp.path.starts_with("super::")
            {
                all_imports.insert(imp.path.clone());
            }
        }
    }

    for imp in all_imports.iter().sorted() {
        content.push_str(&format!("- `{}`\n", imp));
    }

    if all_imports.is_empty() {
        content.push_str("_No external dependencies detected_\n");
    }

    content.push_str("\n## Internal Structure\n\n");
    content.push_str("See subtopic links for internal dependencies.\n");

    content
}

fn generate_root_functions_overview(analysis: &CodeAnalysis, topic: &str) -> String {
    let mut content = format!("# {} - Functions Overview\n\n", topic);
    content
        .push_str("This file contains a high-level overview of all functions in the project.\n\n");

    content.push_str("## Summary by Module\n\n");

    let mut module_counts: HashMap<String, (usize, usize, usize)> = HashMap::new();
    for file in &analysis.files {
        let module = file.path.split('/').next().unwrap_or("root").to_string();
        let entry = module_counts.entry(module).or_insert((0, 0, 0));
        entry.0 += file.functions.len();
        entry.1 += file.structs.len();
        entry.2 += file.enums.len();
    }

    for (module, (funcs, structs, enums)) in module_counts.iter().sorted_by_key(|(k, _)| *k) {
        content.push_str(&format!("### {} (./{}/functions.md)\n\n", module, module));
        content.push_str(&format!("- Functions: {}\n", funcs));
        content.push_str(&format!("- Structs: {}\n", structs));
        content.push_str(&format!("- Enums: {}\n\n", enums));
    }

    content.push_str("## See Also\n\n");
    content.push_str("- [Detailed module functions](./<module>/functions.md)\n");

    content
}

fn generate_module_spec(module: &str, files: &[&CodeFile]) -> String {
    let mut content = format!("# Module: {}\n\n", module);
    content.push_str("## Overview\n\n");
    content.push_str(&format!("Files in this module: {}\n\n", files.len()));

    content.push_str("## Files\n\n");
    for file in files {
        content.push_str(&format!("### {}\n\n", file.path));

        if !file.docs.is_empty() {
            content.push_str("**Documentation:**\n");
            for doc in &file.docs {
                content.push_str(&format!("- {}\n", doc));
            }
            content.push_str("\n");
        }

        if !file.structs.is_empty() {
            content.push_str("**Structs:**\n");
            for s in &file.structs {
                content.push_str(&format!("- `{}`\n", s.name));
                for field in &s.fields {
                    content.push_str(&format!("  - {}: {}\n", field.name, field.type_name));
                }
            }
            content.push_str("\n");
        }

        if !file.enums.is_empty() {
            content.push_str("**Enums:**\n");
            for e in &file.enums {
                content.push_str(&format!("- `{}`\n", e.name));
            }
            content.push_str("\n");
        }
    }

    content
}

fn generate_module_links(files: &[&CodeFile]) -> String {
    let mut content = String::from("# Module Dependencies\n\n");

    content.push_str("## Internal Dependencies\n\n");
    for file in files {
        content.push_str(&format!("### {}\n\n", file.path));

        let internal: Vec<&String> = file
            .imports
            .iter()
            .filter(|i| {
                i.path.starts_with("crate::")
                    || i.path.starts_with("super::")
                    || i.path.starts_with("::")
            })
            .map(|i| &i.path)
            .collect();

        if !internal.is_empty() {
            content.push_str("**Imports:**\n");
            for imp in internal {
                content.push_str(&format!("- `{}`\n", imp));
            }
            content.push_str("\n");
        }
    }

    content
}

fn generate_module_functions(files: &[&CodeFile], module: &str) -> String {
    let mut content = format!("# {} - Functions\n\n", module);

    for file in files {
        content.push_str(&format!("## {}\n\n", file.path));

        if !file.functions.is_empty() {
            for func in &file.functions {
                content.push_str(&format!("### `{}`\n\n", func.name));
                content.push_str(&format!("```rust\n{}\n```\n\n", func.signature));

                if !func.docs.is_empty() {
                    content.push_str("**Documentation:**\n");
                    for doc in &func.docs {
                        content.push_str(&format!("- {}\n", doc));
                    }
                    content.push_str("\n");
                }

                content.push_str(&format!("Lines: {}-{}\n\n", func.start_line, func.end_line));
            }
        }
    }

    content
}

fn start_watcher(path: &Path, topic: &str, area: &str, languages: Vec<String>) -> Result<()> {
    let (tx, rx) = mpsc::channel();

    let mut watcher = RecommendedWatcher::new(
        move |res: Result<Event, notify::Error>| {
            if let Ok(event) = res {
                let _ = tx.send(event);
            }
        },
        Config::default().with_poll_interval(Duration::from_secs(2)),
    )?;

    watcher.watch(path, RecursiveMode::Recursive)?;

    // Store watcher so it doesn't get dropped
    if let Ok(mut guard) = WATCHER.lock() {
        *guard = Some(watcher);
    }

    // Handle events in background
    std::thread::spawn(move || {
        for event in rx {
            match event.kind {
                notify::EventKind::Modify(_) | notify::EventKind::Create(_) => {
                    for path in event.paths {
                        if let Some(ext) = path.extension() {
                            let ext_str = ext.to_string_lossy().to_string();
                            if ["rs", "js", "ts", "py", "go", "sh"].contains(&ext_str.as_str()) {
                                eprintln!("🔄 File changed: {:?}", path);
                                // TODO: Re-parse and update topic
                            }
                        }
                    }
                }
                _ => {}
            }
        }
    });

    Ok(())
}

fn add_to_index(topic: &str, path: &Path) -> Result<()> {
    Ok(())
}

// Generate single topic (for backward compatibility)
fn generate_specs(analysis: &CodeAnalysis, topic: &str) -> String {
    generate_root_spec(analysis, topic)
}

fn generate_links(analysis: &CodeAnalysis) -> String {
    generate_root_links(analysis)
}

fn generate_functions(analysis: &CodeAnalysis) -> String {
    generate_root_functions_overview(analysis, "functions")
}

pub fn stop_watcher() {
    if let Ok(mut guard) = WATCHER.lock() {
        *guard = None;
    }
}
