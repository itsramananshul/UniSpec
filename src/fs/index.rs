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
    #[serde(default)]
    pub tags: Vec<String>,
    #[serde(default)]
    pub annotation: Option<String>,
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
    add_link_with_metadata(topic, area, path, link_type, &[], None)
}

pub fn add_link_with_metadata(
    topic: &str,
    area: &str,
    path: &str,
    link_type: &str,
    tags: &[String],
    annotation: Option<&str>,
) -> Result<()> {
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
        tags: tags.to_vec(),
        annotation: annotation.map(String::from),
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

pub fn find_by_tag(tag: &str) -> Result<Vec<IndexLink>> {
    let index = load_index()?;
    let tag_lower = tag.to_lowercase();
    let links: Vec<IndexLink> = index
        .links
        .into_iter()
        .filter(|l| l.tags.iter().any(|t| t.to_lowercase() == tag_lower))
        .collect();
    Ok(links)
}

pub fn find_by_annotation(query: &str) -> Result<Vec<IndexLink>> {
    let index = load_index()?;
    let query_lower = query.to_lowercase();
    let links: Vec<IndexLink> = index
        .links
        .into_iter()
        .filter(|l| {
            l.annotation
                .as_ref()
                .map(|a| a.to_lowercase().contains(&query_lower))
                .unwrap_or(false)
        })
        .collect();
    Ok(links)
}

pub fn find_by_tags_any(tags: &[String]) -> Result<Vec<IndexLink>> {
    let index = load_index()?;
    let tags_lower: Vec<String> = tags.iter().map(|t| t.to_lowercase()).collect();
    let links: Vec<IndexLink> = index
        .links
        .into_iter()
        .filter(|l| {
            l.tags
                .iter()
                .any(|t| tags_lower.contains(&t.to_lowercase()))
        })
        .collect();
    Ok(links)
}

pub fn list_all_tags() -> Result<Vec<String>> {
    let index = load_index()?;
    let mut tags_set: std::collections::HashSet<String> = std::collections::HashSet::new();
    for link in &index.links {
        for tag in &link.tags {
            tags_set.insert(tag.clone());
        }
    }
    let mut tags: Vec<String> = tags_set.into_iter().collect();
    tags.sort();
    Ok(tags)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphNode {
    pub id: String,
    pub topic: String,
    pub area: String,
    pub path: String,
    #[serde(default)]
    pub tags: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphEdge {
    pub source: String,
    pub target: String,
    #[serde(rename = "type")]
    pub edge_type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphData {
    pub nodes: Vec<GraphNode>,
    pub edges: Vec<GraphEdge>,
}

pub fn export_graph() -> Result<GraphData> {
    let index = load_index()?;

    let mut nodes: Vec<GraphNode> = Vec::new();
    let mut edges: Vec<GraphEdge> = Vec::new();
    let mut seen_paths: std::collections::HashSet<String> = std::collections::HashSet::new();

    for link in &index.links {
        if !seen_paths.contains(&link.path) {
            seen_paths.insert(link.path.clone());
            nodes.push(GraphNode {
                id: format!("path-{}", link.path.replace('/', "-").replace('.', "-")),
                topic: link.topic.clone(),
                area: link.area.clone(),
                path: link.path.clone(),
                tags: link.tags.clone(),
            });
        }

        edges.push(GraphEdge {
            source: format!("topic-{}", link.topic.replace('/', "-")),
            target: format!("path-{}", link.path.replace('/', "-").replace('.', "-")),
            edge_type: "links_to".to_string(),
        });
    }

    let topics: std::collections::HashSet<String> =
        index.links.iter().map(|l| l.topic.clone()).collect();
    for topic in topics {
        nodes.push(GraphNode {
            id: format!("topic-{}", topic.replace('/', "-")),
            topic: topic.clone(),
            area: String::new(),
            path: String::new(),
            tags: vec![],
        });
    }

    Ok(GraphData { nodes, edges })
}

pub fn generate_backlinks() -> Result<std::collections::HashMap<String, Vec<IndexLink>>> {
    let index = load_index()?;
    let mut backlinks: std::collections::HashMap<String, Vec<IndexLink>> =
        std::collections::HashMap::new();

    for link in &index.links {
        backlinks
            .entry(link.topic.clone())
            .or_insert_with(Vec::new)
            .push(link.clone());
    }

    Ok(backlinks)
}

pub fn generate_backlinks_file(topic: &str, area: &str) -> Result<String> {
    let links = find_by_topic(topic)?;

    let mut md = String::new();
    md.push_str(&format!("# Backlinks: {}\n\n", topic));
    md.push_str(&format!("Area: {}\n\n", area));
    md.push_str("## Linked Files\n\n");

    if links.is_empty() {
        md.push_str("_No files linked to this topic._\n");
    } else {
        for link in &links {
            md.push_str(&format!(
                "- [{}]({}) - {}\n",
                link.path,
                link.path,
                link.annotation.as_deref().unwrap_or(&link.link_type)
            ));
        }
    }

    md.push_str("\n## Tags\n\n");
    let all_tags: std::collections::HashSet<String> =
        links.iter().flat_map(|l| l.tags.clone()).collect();
    if all_tags.is_empty() {
        md.push_str("_No tags._\n");
    } else {
        for tag in all_tags {
            md.push_str(&format!("- {}\n", tag));
        }
    }

    Ok(md)
}
