use crate::fs::index::Export;
use anyhow::Result;
use std::path::Path;

pub fn run_add(
    topic: &str,
    path: &str,
    area: &str,
    link_type: &str,
    tags: Option<&str>,
    annotation: Option<&str>,
    exports: Option<&str>,
    descriptions: Option<&str>,
    export_types: Option<&str>,
    signatures: Option<&str>,
) -> Result<()> {
    let tags_vec: Vec<String> = tags
        .map(|t| t.split(',').map(|s| s.trim().to_string()).collect())
        .unwrap_or_default();

    let exports_vec: Vec<Export> = if let Some(exp) = exports {
        let names: Vec<&str> = exp.split(',').map(|s| s.trim()).collect();
        let descs: Vec<&str> = descriptions
            .map(|d| d.split(',').map(|s| s.trim()).collect())
            .unwrap_or_default();
        let types: Vec<&str> = export_types
            .map(|t| t.split(',').map(|s| s.trim()).collect())
            .unwrap_or_default();
        let sigs: Vec<&str> = signatures
            .map(|s| s.split(';').map(|s| s.trim()).collect())
            .unwrap_or_default();

        names
            .iter()
            .enumerate()
            .map(|(i, name)| {
                let id = format!("{}:{}", topic, name);
                Export {
                    id: id.clone(),
                    name: name.to_string(),
                    export_type: types.get(i).unwrap_or(&"function").to_string(),
                    description: descs.get(i).unwrap_or(&"").to_string(),
                    signature: sigs.get(i).map(|s| s.to_string()),
                }
            })
            .collect()
    } else {
        vec![]
    };

    if !exports_vec.is_empty() || tags.is_some() || annotation.is_some() {
        crate::fs::index::add_link_with_exports(
            topic,
            area,
            path,
            link_type,
            &tags_vec,
            annotation,
            &exports_vec,
        )?;
    } else {
        crate::fs::index::add_link(topic, area, path, link_type)?;
    }

    println!("Added link: {} -> {} ({})", topic, path, link_type);
    if !tags_vec.is_empty() {
        println!("  Tags: {}", tags_vec.join(", "));
    }
    if let Some(ann) = annotation {
        println!("  Note: {}", ann);
    }
    if !exports_vec.is_empty() {
        println!("  Exports:");
        for exp in &exports_vec {
            println!(
                "    - {} ({}) - {}",
                exp.name, exp.export_type, exp.description
            );
        }
    }
    Ok(())
}

pub fn run_remove(topic: &str, path: &str) -> Result<()> {
    crate::fs::index::remove_link(topic, path)?;
    println!("Removed link: {} -> {}", topic, path);
    Ok(())
}

pub fn run_list(topic: Option<&str>, path: Option<&str>, tag: Option<&str>) -> Result<()> {
    let links = if let Some(t) = topic {
        crate::fs::index::find_by_topic(t)?
    } else if let Some(p) = path {
        crate::fs::index::find_by_path(p)?
    } else if let Some(t) = tag {
        crate::fs::index::find_by_tag(t)?
    } else {
        crate::fs::index::list_all()?
    };

    if links.is_empty() {
        println!("No links found.");
    } else {
        println!("Links:");
        for link in links {
            let tags_str = if link.tags.is_empty() {
                String::new()
            } else {
                format!(" [{}]", link.tags.join(", "))
            };
            let ann_str = link
                .annotation
                .as_ref()
                .map(|a| format!(" - {}", a))
                .unwrap_or_default();
            let exports_str = if link.exports.is_empty() {
                String::new()
            } else {
                format!(" {{{} items}}", link.exports.len())
            };
            println!(
                "  {} | {} | {}{}{}{}",
                link.topic, link.area, link.path, tags_str, ann_str, exports_str
            );
        }
    }
    Ok(())
}

pub fn run_find(query: &str, by: &str) -> Result<()> {
    let links = match by {
        "topic" => crate::fs::index::find_by_topic(query)?,
        "path" => crate::fs::index::find_by_path(query)?,
        "tag" => crate::fs::index::find_by_tag(query)?,
        "annotation" => crate::fs::index::find_by_annotation(query)?,
        _ => {
            return Err(anyhow::anyhow!(
                "Invalid search type. Use --by topic, path, tag, or annotation"
            ));
        }
    };

    if links.is_empty() {
        match by {
            "topic" => println!("No paths found for topic '{}'.", query),
            "path" => println!("No topics found for path '{}'.", query),
            "tag" => println!("No links found with tag '{}'.", query),
            "annotation" => println!("No links found with annotation containing '{}'.", query),
            _ => {}
        }
    } else {
        match by {
            "topic" => println!("Paths for topic '{}':", query),
            "path" => println!("Topics for path '{}':", query),
            "tag" => println!("Links with tag '{}':", query),
            "annotation" => println!("Links with annotation matching '{}':", query),
            _ => {}
        }
        for link in links {
            let tags_str = if link.tags.is_empty() {
                String::new()
            } else {
                format!(" [{}]", link.tags.join(", "))
            };
            match by {
                "topic" => println!("  {}{}", link.path, tags_str),
                "path" => println!("  {} in {}{}", link.topic, link.area, tags_str),
                "tag" | "annotation" => println!(
                    "  {} -> {} in {}{}",
                    link.topic, link.path, link.area, tags_str
                ),
                _ => {}
            }
        }
    }
    Ok(())
}

pub fn detect_type(path: &str) -> String {
    if Path::new(path).is_dir() {
        "directory".to_string()
    } else {
        "file".to_string()
    }
}

pub fn run_full() -> Result<()> {
    println!(
        "Index stored at: {}",
        crate::fs::index::index_path().display()
    );
    let links = crate::fs::index::list_all()?;
    println!("Total links: {}", links.len());

    let tags = crate::fs::index::list_all_tags()?;
    println!("Unique tags: {}", tags.len());
    if !tags.is_empty() {
        println!("Tags: {}", tags.join(", "));
    }

    let export_count: usize = links.iter().map(|l| l.exports.len()).sum();
    println!("Total exports: {}", export_count);

    Ok(())
}

pub fn run_cleanup() -> Result<()> {
    let (removed_topics, removed_paths) = crate::fs::index::cleanup()?;

    let total_removed = removed_topics.len() + removed_paths.len();

    if total_removed == 0 {
        println!("No orphaned links found. Index is clean.");
    } else {
        if !removed_topics.is_empty() {
            println!("Removed links to non-existent topics:");
            for topic in &removed_topics {
                println!("  - {}", topic);
            }
        }
        if !removed_paths.is_empty() {
            println!("Removed links to non-existent paths:");
            for path in &removed_paths {
                println!("  - {}", path);
            }
        }
        println!("\nTotal: {} link(s) removed.", total_removed);
    }
    Ok(())
}

pub fn run_watch() -> Result<()> {
    println!("Auto-index watcher coming soon!");
    Ok(())
}

pub fn run_tags() -> Result<()> {
    let tags = crate::fs::index::list_all_tags()?;

    if tags.is_empty() {
        println!("No tags in index.");
    } else {
        println!("Tags in index:");
        for tag in tags {
            let links = crate::fs::index::find_by_tag(&tag)?;
            println!("  {} ({} links)", tag, links.len());
        }
    }
    Ok(())
}

pub fn run_graph() -> Result<()> {
    let graph = crate::fs::index::export_graph()?;
    let json = serde_json::to_string_pretty(&graph)?;
    println!("{}", json);
    Ok(())
}

pub fn run_backlinks(topic: &str) -> Result<()> {
    let area = "Working";
    let md = crate::fs::index::generate_backlinks_file(topic, area)?;
    println!("{}", md);
    Ok(())
}

pub fn run_callers(symbol: &str) -> Result<()> {
    let callers = crate::fs::index::find_callers(symbol)?;
    if callers.is_empty() {
        println!("No callers found for symbol '{}'.", symbol);
    } else {
        println!("Callers for '{}':", symbol);
        for caller in callers {
            println!("  - {}", caller);
        }
    }
    Ok(())
}

pub fn run_exports(topic: Option<&str>) -> Result<()> {
    if let Some(t) = topic {
        let exports = crate::fs::index::get_exports_for_topic(t)?;
        if exports.is_empty() {
            println!("No exports for topic '{}'.", t);
        } else {
            println!("Exports for '{}':\n", t);
            for exp in &exports {
                let sig = exp
                    .signature
                    .as_ref()
                    .map(|s| format!("\n    Signature: {}", s))
                    .unwrap_or_default();
                println!("  {} ({}){}", exp.name, exp.export_type, sig);
                if !exp.description.is_empty() {
                    println!("    Description: {}", exp.description);
                }
                println!("    ID: {}", exp.id);
                println!();
            }
        }
    } else {
        let links = crate::fs::index::list_all()?;
        let mut has_exports = false;
        for link in links {
            if !link.exports.is_empty() {
                has_exports = true;
                println!("Topic: {} - {}", link.topic, link.path);
                for exp in &link.exports {
                    println!(
                        "  - {} ({}) - {}",
                        exp.name, exp.export_type, exp.description
                    );
                }
                println!();
            }
        }
        if !has_exports {
            println!("No exports in index.");
        }
    }
    Ok(())
}

pub fn run_query(query: &str, by: &str) -> Result<()> {
    let results = crate::fs::index::find_exports(query, by)?;
    if results.is_empty() {
        println!("No exports found matching '{}' by {}.", query, by);
    } else {
        println!("Exports matching '{}' (by {}):\n", query, by);
        for r in &results {
            println!("  {} ({})", r.name, r.export_type);
            println!("    Topic: {}", r.topic);
            println!("    Path: {}", r.path);
            println!("    Description: {}", r.description);
            if let Some(ref sig) = r.signature {
                println!("    Signature: {}", sig);
            }
            println!("    ID: {}", r.id);
            println!();
        }
    }
    Ok(())
}

pub fn run_depends(topic: &str) -> Result<()> {
    let dependents = crate::fs::index::get_dependents(topic)?;
    if dependents.is_empty() {
        println!("No topics depend on '{}'.", topic);
    } else {
        println!("Topics depending on '{}':\n", topic);
        let mut by_topic: std::collections::HashMap<
            String,
            Vec<&crate::fs::index::ExportQueryResult>,
        > = std::collections::HashMap::new();
        for d in &dependents {
            by_topic.entry(d.topic.clone()).or_default().push(d);
        }
        for (topic_name, exports) in by_topic {
            println!("  {}", topic_name);
            for exp in exports {
                println!("    - {} ({})", exp.name, exp.id);
            }
            println!();
        }
    }
    Ok(())
}

pub fn run_lookup(id: &str) -> Result<()> {
    let result = crate::fs::index::find_export_by_id(id)?;
    match result {
        Some(exp) => {
            println!("Found: {}", exp.id);
            println!("  Name: {}", exp.name);
            println!("  Type: {}", exp.export_type);
            println!("  Topic: {}", exp.topic);
            println!("  Path: {}", exp.path);
            println!("  Description: {}", exp.description);
            if let Some(ref sig) = exp.signature {
                println!("  Signature: {}", sig);
            }
        }
        None => {
            println!("Export not found: {}", id);
        }
    }
    Ok(())
}
