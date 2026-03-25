use anyhow::Result;
use std::path::Path;

pub fn run_add(topic: &str, path: &str, area: &str, link_type: &str) -> Result<()> {
    crate::fs::index::add_link(topic, area, path, link_type)?;
    println!("Added link: {} -> {} ({})", topic, path, link_type);
    Ok(())
}

pub fn run_remove(topic: &str, path: &str) -> Result<()> {
    crate::fs::index::remove_link(topic, path)?;
    println!("Removed link: {} -> {}", topic, path);
    Ok(())
}

pub fn run_list(topic: Option<&str>, path: Option<&str>) -> Result<()> {
    let links = if let Some(t) = topic {
        crate::fs::index::find_by_topic(t)?
    } else if let Some(p) = path {
        crate::fs::index::find_by_path(p)?
    } else {
        crate::fs::index::list_all()?
    };

    if links.is_empty() {
        println!("No links found.");
    } else {
        println!("Links:");
        for link in links {
            println!(
                "  {} | {} | {} ({}) - added: {}",
                link.topic, link.area, link.path, link.link_type, link.added
            );
        }
    }
    Ok(())
}

pub fn run_find(query: &str, by: &str) -> Result<()> {
    let links = match by {
        "topic" => crate::fs::index::find_by_topic(query)?,
        "path" => crate::fs::index::find_by_path(query)?,
        _ => {
            return Err(anyhow::anyhow!(
                "Invalid search type. Use --by topic or --by path"
            ));
        }
    };

    if links.is_empty() {
        match by {
            "topic" => println!("No paths found for topic '{}'.", query),
            "path" => println!("No topics found for path '{}'.", query),
            _ => {}
        }
    } else {
        match by {
            "topic" => println!("Paths for topic '{}':", query),
            "path" => println!("Topics for path '{}':", query),
            _ => {}
        }
        for link in links {
            match by {
                "topic" => println!("  {} ({}) - {}", link.path, link.link_type, link.added),
                "path" => println!(
                    "  {} in {} ({}) - {}",
                    link.topic, link.area, link.link_type, link.added
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
