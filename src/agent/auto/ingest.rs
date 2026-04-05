// src/agent/auto/ingest.rs
// Auto ingest - generates specs from existing codebase

use anyhow::Result;
use itertools::Itertools;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::Path;

use crate::agent::code_parser::{analyze_directory, CodeAnalysis, CodeFile};
use crate::fs::spec_dir;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IngestResult {
    pub topic: String,
    pub files_analyzed: u32,
    pub modules_created: Vec<String>,
    pub specs_path: String,
    pub locked: bool,
    pub lock_message: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModuleSpec {
    pub name: String,
    pub files: Vec<String>,
    pub functions: Vec<String>,
    pub structs: Vec<String>,
    pub enums: Vec<String>,
}

fn read_master_spec(master_path: &Path) -> Result<String> {
    if !master_path.exists() {
        return Err(anyhow::anyhow!(
            "Master spec not found at: {}. Please create spec/master.md first.",
            master_path.display()
        ));
    }
    let content = fs::read_to_string(master_path)?;
    Ok(content)
}

fn parse_master_spec(content: &str) -> (String, HashMap<String, String>) {
    let mut project_name = String::new();
    let mut structure = HashMap::new();
    let mut current_section = String::new();

    for line in content.lines() {
        let trimmed = line.trim();

        if trimmed.starts_with("# ") && project_name.is_empty() {
            project_name = trimmed.trim_start_matches("# ").to_string();
        } else if trimmed.starts_with("Structure:") || trimmed.starts_with("- src/") {
            current_section = "structure".to_string();
        } else if trimmed.starts_with('-') && current_section == "structure" {
            let path = trimmed
                .trim_start_matches("- ")
                .trim_end_matches("/")
                .to_string();
            structure.insert(path.clone(), path);
        }
    }

    if project_name.is_empty() {
        project_name = "Imported Project".to_string();
    }

    (project_name, structure)
}

fn generate_topic_description(module: &str, files: &[&CodeFile]) -> String {
    let mut description = format!("Auto-generated module for {}", module);

    if let Some(first_file) = files.first() {
        if !first_file.docs.is_empty() {
            description = first_file.docs.first().unwrap().clone();
        }
    }

    description
}

pub fn run_auto_ingest(
    code_path: &str,
    master_spec: &str,
    topic: Option<&str>,
    area: Option<&str>,
) -> Result<IngestResult> {
    let area = area.unwrap_or("Working");
    let code = Path::new(code_path);

    if !code.exists() {
        return Err(anyhow::anyhow!("Code path does not exist: {}", code_path));
    }

    let master_path = Path::new(master_spec);
    let master_content = read_master_spec(master_path)?;
    let (project_name, structure) = parse_master_spec(&master_content);

    let topic_name = topic.unwrap_or(&project_name).to_string();

    let langs: Vec<String> = vec![];
    let analysis = analyze_directory(code, langs)?;

    let spec_area_dir = spec_dir().join(area);
    let root_topic_dir = spec_area_dir.join(&topic_name);
    fs::create_dir_all(&root_topic_dir)?;

    let root_spec_content = generate_root_spec(&topic_name, &analysis, &structure, &master_content);
    fs::write(root_topic_dir.join("specs.md"), root_spec_content)?;

    let root_topic_md = format!(
        "# {}\n\nShort: {}\n\nParent: \nChildren: \n\nStatus: ingested\n",
        topic_name, project_name
    );
    fs::write(root_topic_dir.join("topic.md"), root_topic_md)?;

    let root_links = generate_root_links(&analysis);
    fs::write(root_topic_dir.join("links.md"), root_links)?;

    let root_functions = generate_root_functions(&topic_name, &analysis);
    fs::write(root_topic_dir.join("functions.md"), root_functions)?;

    let mut modules_created = Vec::new();

    let mut modules: HashMap<String, Vec<&CodeFile>> = HashMap::new();
    for file in &analysis.files {
        let root = file.path.split('/').next().unwrap_or("root").to_string();
        modules.entry(root).or_default().push(file);
    }

    for (module, files) in &modules {
        if module == "root" || module.is_empty() {
            continue;
        }

        let subtopic_path = root_topic_dir.join(module);
        fs::create_dir_all(&subtopic_path)?;

        let description = generate_topic_description(module, files);
        let module_topic_md = format!(
            "# {}\n\nShort: {}\n\nParent: {}\nChildren: \n\nStatus: ingested\n",
            module, description, topic_name
        );
        fs::write(subtopic_path.join("topic.md"), module_topic_md)?;

        let module_spec = generate_module_spec(module, files);
        fs::write(subtopic_path.join("specs.md"), module_spec)?;

        let module_links = generate_module_links(files);
        fs::write(subtopic_path.join("links.md"), module_links)?;

        let module_functions = generate_module_functions(module, files);
        fs::write(subtopic_path.join("functions.md"), module_functions)?;

        modules_created.push(module.to_string());
    }

    let specs_path = root_topic_dir
        .join("specs.md")
        .to_string_lossy()
        .to_string();

    println!(
        "Ingested {} files, created {} modules for topic '{}'",
        analysis.total_files,
        modules_created.len(),
        topic_name
    );

    Ok(IngestResult {
        topic: topic_name,
        files_analyzed: analysis.total_files as u32,
        modules_created,
        specs_path,
        locked: false,
        lock_message: None,
    })
}

fn generate_root_spec(
    topic: &str,
    analysis: &CodeAnalysis,
    structure: &HashMap<String, String>,
    master_content: &str,
) -> String {
    let mut content = format!("# {}\n\n", topic);
    content.push_str("## Overview\n\n");
    content.push_str("Auto-generated from codebase analysis.\n\n");
    content.push_str(&format!(
        "This spec was generated from master spec:\n\n```\n{}\n```\n\n",
        master_content
            .lines()
            .take(10)
            .collect::<Vec<_>>()
            .join("\n")
    ));

    content.push_str("## Project Statistics\n\n");
    content.push_str(&format!("- Total Files: {}\n", analysis.total_files));
    content.push_str(&format!(
        "- Total Functions: {}\n",
        analysis.total_functions
    ));
    content.push_str(&format!("- Total Structs: {}\n", analysis.total_structs));
    content.push_str(&format!("- Total Enums: {}\n", analysis.total_enums));
    content.push_str("\n");

    if !structure.is_empty() {
        content.push_str("## Project Structure\n\n");
        for (path, _) in structure {
            content.push_str(&format!("- `{}`\n", path));
        }
        content.push_str("\n");
    }

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

fn generate_root_functions(topic: &str, analysis: &CodeAnalysis) -> String {
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

fn generate_module_functions(module: &str, files: &[&CodeFile]) -> String {
    let mut content = format!("# {} - Functions\n\n", module);

    for file in files {
        content.push_str(&format!("## {}\n\n", file.path));

        if !file.functions.is_empty() {
            for func in &file.functions {
                content.push_str(&format!("### `{}`\n\n", func.name));
                content.push_str(&format!("```\n{}\n```\n\n", func.signature));

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
