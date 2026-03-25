use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndexLink {
    pub topic: String,
    pub area: String,
    pub path: String,
    #[serde(rename = "type")]
    pub link_type: String,
    pub added: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Index {
    #[serde(default)]
    pub links: Vec<IndexLink>,
}

impl Default for Index {
    fn default() -> Self {
        Index { links: vec![] }
    }
}

pub fn index_path() -> PathBuf {
    crate::fs::spec_dir().join("index.toml")
}

pub fn load_index() -> Result<Index> {
    let path = index_path();
    if !path.exists() {
        return Ok(Index::default());
    }
    let content = fs::read_to_string(&path)?;
    let index: Index = toml::from_str(&content)?;
    Ok(index)
}

pub fn save_index(index: &Index) -> Result<()> {
    let path = index_path();
    let content = toml::to_string_pretty(index)?;
    fs::write(&path, content)?;
    Ok(())
}

pub fn add_link(topic: &str, area: &str, path: &str, link_type: &str) -> Result<()> {
    let mut index = load_index()?;

    if index
        .links
        .iter()
        .any(|l| l.topic == topic && l.path == path)
    {
        return Err(anyhow::anyhow!(
            "Link already exists: {} -> {}",
            topic,
            path
        ));
    }

    let link = IndexLink {
        topic: topic.to_string(),
        area: area.to_string(),
        path: path.to_string(),
        link_type: link_type.to_string(),
        added: chrono_now(),
    };

    index.links.push(link);
    save_index(&index)?;
    Ok(())
}

pub fn remove_link(topic: &str, path: &str) -> Result<()> {
    let mut index = load_index()?;
    let original_len = index.links.len();
    index
        .links
        .retain(|l| !(l.topic == topic && l.path == path));

    if index.links.len() == original_len {
        return Err(anyhow::anyhow!("Link not found: {} -> {}", topic, path));
    }

    save_index(&index)?;
    Ok(())
}

pub fn find_by_topic(topic: &str) -> Result<Vec<IndexLink>> {
    let index = load_index()?;
    let links: Vec<IndexLink> = index
        .links
        .into_iter()
        .filter(|l| l.topic.to_lowercase() == topic.to_lowercase())
        .collect();
    Ok(links)
}

pub fn find_by_path(search_path: &str) -> Result<Vec<IndexLink>> {
    let index = load_index()?;
    let search_lower = search_path.to_lowercase();
    let links: Vec<IndexLink> = index
        .links
        .into_iter()
        .filter(|l| l.path.to_lowercase().contains(&search_lower))
        .collect();
    Ok(links)
}

pub fn list_all() -> Result<Vec<IndexLink>> {
    let index = load_index()?;
    Ok(index.links)
}

fn chrono_now() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let duration = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    let secs = duration.as_secs();
    let hours = (secs / 3600) % 24;
    let minutes = (secs / 60) % 60;
    let day = secs / 86400;
    format!("{}T{:02}:{:02}:00Z", day, hours, minutes)
}

pub fn cleanup() -> Result<(Vec<String>, Vec<String>)> {
    let mut index = load_index()?;
    let spec_dir = crate::fs::spec_dir();

    let mut removed_topics: Vec<String> = Vec::new();
    let mut removed_paths: Vec<String> = Vec::new();

    let original_links = index.links.clone();
    index.links.clear();

    for link in original_links {
        let topic_path = spec_dir.join(&link.area).join(&link.topic);
        let path_exists = std::path::Path::new(&link.path).exists();

        if topic_path.exists() && path_exists {
            index.links.push(link);
        } else {
            if !topic_path.exists() {
                removed_topics.push(format!("{}/{}", link.area, link.topic));
            }
            if !path_exists {
                removed_paths.push(link.path.clone());
            }
        }
    }

    save_index(&index)?;
    Ok((removed_topics, removed_paths))
}
