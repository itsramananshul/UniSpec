use anyhow::Result;
use std::path::Path;

pub fn run_add(
    topic: &str,
    path: &str,
    area: &str,
    link_type: &str,
    tags: Option<&str>,
    annotation: Option<&str>,
) -> Result<()> {
    let tags_vec: Vec<String> = tags
        .map(|t| t.split(',').map(|s| s.trim().to_string()).collect())
        .unwrap_or_default();

    if tags.is_some() || annotation.is_some() {
        crate::fs::index::add_link_with_metadata(
            topic, area, path, link_type, &tags_vec, annotation,
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
            println!(
                "  {} | {} | {}{}{}",
                link.topic, link.area, link.path, tags_str, ann_str
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
